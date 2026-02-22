use std::sync::Arc;
use winit::window::Window;

use crate::util::block_on;

pub struct GpuState {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    render_pipeline: wgpu::RenderPipeline,
    bind_group: wgpu::BindGroup,
    texture: wgpu::Texture,
    pixels: Vec<u8>,
    width: u32,
    height: u32,
}

impl GpuState {
    pub fn new(window: Arc<Window>, width: u32, height: u32) -> Self {
        let wgpu_instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());
        let wgpu_surface = wgpu_instance.create_surface(window.clone()).unwrap();
        let adapter = block_on(wgpu_instance.request_adapter(&wgpu::RequestAdapterOptions {
            compatible_surface: Some(&wgpu_surface),
            ..Default::default()
        }))
        .unwrap();
        let (device, queue) =
            block_on(adapter.request_device(&wgpu::DeviceDescriptor::default())).unwrap();

        let window_size = window.inner_size();
        let mut config = wgpu_surface
            .get_default_config(
                &adapter,
                window_size.width.max(1),
                window_size.height.max(1),
            )
            .unwrap();
        config.present_mode = wgpu::PresentMode::AutoVsync;
        wgpu_surface.configure(&device, &config);

        // pixel texture
        let texture_descriptor = wgpu::TextureDescriptor {
            label: Some("pixel_buf"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        };
        let texture = device.create_texture(&texture_descriptor);
        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("blit"),
            layout: None,
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: None,
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });

        let bind_group_layout = render_pipeline.get_bind_group_layout(0);
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("blit_bg"),
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
        });

        let pixels = vec![255u8; (width * height * 4) as usize];

        Self {
            surface: wgpu_surface,
            device,
            queue,
            config,
            render_pipeline,
            bind_group,
            texture,
            pixels,
            width,
            height,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn render(&self) {
        self.queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &self.texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &self.pixels,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(self.width * 4),
                rows_per_image: Some(self.height),
            },
            wgpu::Extent3d {
                width: self.width,
                height: self.height,
                depth_or_array_layers: 1,
            },
        );

        let frame = match self.surface.get_current_texture() {
            Ok(frame) => frame,
            Err(wgpu::SurfaceError::Lost) => {
                self.surface.configure(&self.device, &self.config);
                return;
            }
            Err(_) => return,
        };
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("blit_pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                ..Default::default()
            });
            pass.set_pipeline(&self.render_pipeline);
            pass.set_bind_group(0, &self.bind_group, &[]);
            let (viewport_x, viewport_y, viewport_width, viewport_height) = {
                let window_width = self.config.width as f32;
                let window_height = self.config.height as f32;
                let scale = (window_width / self.width as f32).min(window_height / self.height as f32);
                let scaled_width = self.width as f32 * scale;
                let scaled_height = self.height as f32 * scale;
                ((window_width - scaled_width) / 2.0, (window_height - scaled_height) / 2.0, scaled_width, scaled_height)
            };
            pass.set_viewport(viewport_x, viewport_y, viewport_width, viewport_height, 0.0, 1.0);
            pass.draw(0..3, 0..1);
        }
        self.queue.submit(std::iter::once(encoder.finish()));
        frame.present();
    }
}

impl crate::Renderer for GpuState {
    fn set_pixel(&mut self, x: u32, y: u32, red: u8, green: u8, blue: u8) {
        if x < self.width && y < self.height {
            let index = ((y * self.width + x) * 4) as usize;
            self.pixels[index] = red;
            self.pixels[index + 1] = green;
            self.pixels[index + 2] = blue;
            self.pixels[index + 3] = 255;
        }
    }

    fn clear(&mut self, red: u8, green: u8, blue: u8) {
        for chunk in self.pixels.chunks_exact_mut(4) {
            chunk[0] = red;
            chunk[1] = green;
            chunk[2] = blue;
            chunk[3] = 255;
        }
    }

    fn fill_rect(&mut self, x: i32, y: i32, width: u32, height: u32, red: u8, green: u8, blue: u8) {
        let start_x = x.max(0) as u32;
        let start_y = y.max(0) as u32;
        let end_x = ((x + width as i32) as u32).min(self.width);
        let end_y = ((y + height as i32) as u32).min(self.height);

        for row in start_y..end_y {
            let start = ((row * self.width + start_x) * 4) as usize;
            let end = ((row * self.width + end_x) * 4) as usize;
            let slice = &mut self.pixels[start..end];
            for pixel in slice.chunks_exact_mut(4) {
                pixel[0] = red;
                pixel[1] = green;
                pixel[2] = blue;
                pixel[3] = 255;
            }
        }
    }

    fn draw_sprite(&mut self, x: i32, y: i32, width: u32, height: u32, pixels: &[u8]) {
        let screen_width = self.width as i32;
        let screen_height = self.height as i32;

        // clipping
        let src_x = (-x).max(0) as u32;
        let src_y = (-y).max(0) as u32;
        let dst_x = x.max(0) as u32;
        let dst_y = y.max(0) as u32;
        let draw_width = (width - src_x).min((screen_width - dst_x as i32).max(0) as u32);
        let draw_height = (height - src_y).min((screen_height - dst_y as i32).max(0) as u32);

        if draw_width == 0 || draw_height == 0 {
            return;
        }

        for row in 0..draw_height {
            let src_row = (src_y + row) as usize;
            let dst_row = (dst_y + row) as usize;
            let dst_row_offset = dst_row * self.width as usize;

            for column in 0..draw_width {
                let src_column = (src_x + column) as usize;
                let src_index = (src_row * width as usize + src_column) * 4;
                let alpha = pixels[src_index + 3];
                if alpha == 0 {
                    continue;
                }
                let dst_index = (dst_row_offset + dst_x as usize + column as usize) * 4;
                self.pixels[dst_index] = pixels[src_index];
                self.pixels[dst_index + 1] = pixels[src_index + 1];
                self.pixels[dst_index + 2] = pixels[src_index + 2];
                self.pixels[dst_index + 3] = 255;
            }
        }
    }

    fn width(&self) -> u32 {
        self.width
    }

    fn height(&self) -> u32 {
        self.height
    }
}
