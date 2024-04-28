use std::{num::NonZeroU32, sync::Arc, time::{Duration, Instant}};

use log::error;
use winit::window::Window;

use crate::{config::{CAMERA_DISTANCE, CAMERA_FOV, MSAA_COUNT}, d2::{aspectuniform::AspectUniform, texture::tex_from_bytes, tovertind2d::ToVertInd2D, vertex::Vertex2D}, d3::{camera::{Camera, CameraUniform}, tovertind3d::ToVertInd3D, vertex::Vertex3D}};

pub struct State<'a> {
    surface: wgpu::Surface<'a>,
    surface_format: wgpu::TextureFormat,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    window: Arc<winit::window::Window>,
    clearcol: wgpu::Color,
    render_pipeline_3d: wgpu::RenderPipeline,
    vertex_buffer_3d: wgpu::Buffer,
    index_buffer_3d: wgpu::Buffer,
    camera: Camera,
    camera_uniform: CameraUniform,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    depth_texture_3d: wgpu::Texture,
    depth_texture_view_3d: wgpu::TextureView,
    msaa_framebuffer: wgpu::Texture,
    render_pipeline_2d: wgpu::RenderPipeline,
    vertex_buffer_2d: wgpu::Buffer,
    index_buffer_2d: wgpu::Buffer,
    pub aspect_uniform: AspectUniform,
    aspect_bind_group: wgpu::BindGroup,
    aspect_buffer: wgpu::Buffer,
    depth_texture_2d: wgpu::Texture,
    depth_texture_view_2d: wgpu::TextureView,
    texture_bind_group: wgpu::BindGroup
}

impl State<'_> {
    pub async fn new(window: Arc<Window>, tex_arr: Vec<&[u8]>, clearcolf32: [f32;4]) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::VULKAN,
            ..Default::default()
        });

        let surface = match instance.create_surface(window.clone()) {
            Ok(o) => {o}
            Err(e) => {
                error!("Failed to create surface due to error: {e}");
                panic!()
            }
        };

        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false
            }
        ).await.expect("Failed to request adapter");

        let (device, queue) = match adapter.request_device(
            &wgpu::DeviceDescriptor {
                required_features: 
                    wgpu::Features::TEXTURE_BINDING_ARRAY
                    | wgpu::Features::SAMPLED_TEXTURE_AND_STORAGE_BUFFER_ARRAY_NON_UNIFORM_INDEXING,
                required_limits: {
                    let mut l = wgpu::Limits::default();
                    l.max_sampled_textures_per_shader_stage = 32;
                    l
                },
                label: None
            },
            None
        ).await {
            Ok(o) => {o}
            Err(e) => {
                error!("Failed to request device due to error: {e}");
                panic!()
            }
        };

        let surface_caps = surface.get_capabilities(&adapter);

        let surface_format = surface_caps.formats.iter()
            .copied()
            .filter(|f| f.is_srgb())
            .next()
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            // present_mode: surface_caps.present_modes[0],
            // present_mode: wgpu::PresentMode::AutoNoVsync,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 1
        };

        surface.configure(&device, &config);

        let mut tex_views: Vec<wgpu::TextureView> = vec![];
        let mut texes: Vec<wgpu::Texture> = vec![];

        for (i,tb) in tex_arr.iter().enumerate() {
            let len = (**tb).len();
            if len == 0 {
                continue;
            }
            let (t, v) = match tex_from_bytes(
                tb,
                &device,
                &queue,
            ) {
                Ok(o) => {o}
                Err(e) => {
                    println!("Failed to import texture due to error: {e}");
                    println!("While trying to import texture index {i}");
                    panic!();
                }
            };

            texes.push(t);
            tex_views.push(v);
        }

        let texture_sampler = device.create_sampler(
            &wgpu::SamplerDescriptor {
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Nearest,
                mipmap_filter: wgpu::FilterMode::Nearest,
                ..Default::default()
            }
        );

        let texture_bind_group_layout = device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float {
                                filterable: true
                            }
                        },
                        count: Some(
                            NonZeroU32::new(texes.len() as u32).expect("Texture array is of length zero")
                        )
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None
                    }
                ],
                label: Some("Texture bind group layout")
            }
        );

        let texture_bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &texture_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureViewArray(
                            &tex_views
								.iter()
								.map(|v| v)
								.collect::<Vec<&wgpu::TextureView>>()
                        )
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&texture_sampler)
                    }
                ],
                label: Some("Gi Hun sphere bind group")
            }
        );

        let shader_3d = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("d3/shader.wgsl").into())
        });

        let shader_2d = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("d2/shader.wgsl").into())
        });

        let vertex_buffer_3d = device.create_buffer(
            &wgpu::BufferDescriptor {
                label: Some("3D Vertex Buffer"),
                size: 2_000_000,
                mapped_at_creation: false,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST
            }
        );

        let vertex_buffer_2d = device.create_buffer(
            &wgpu::BufferDescriptor {
                label: Some("2D Vertex Buffer"),
                size: 2_000_000,
                mapped_at_creation: false,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST
            }
        );

        let index_buffer_3d = device.create_buffer(
            &wgpu::BufferDescriptor {
                label: Some("3D Index Buffer"),
                size: 2_000_000,
                mapped_at_creation: false,
                usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST
            }
        );

        let index_buffer_2d = device.create_buffer(
            &wgpu::BufferDescriptor {
                label: Some("2D Index Buffer"),
                size: 2_000_000,
                mapped_at_creation: false,
                usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST
            }
        );

        let depth_tex_desc_3d = wgpu::TextureDescriptor {
            label: Some("3D Depth Texture"),
            size: wgpu::Extent3d {
                width: config.width,
                height: config.height,
                depth_or_array_layers: 1
            },
            mip_level_count: 1,
            sample_count: MSAA_COUNT,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[]
        };

        let depth_texture_3d = device.create_texture(&depth_tex_desc_3d);

        let depth_texture_view_3d = depth_texture_3d.create_view(&wgpu::TextureViewDescriptor::default());

        let depth_tex_desc_2d = wgpu::TextureDescriptor {
            label: Some("2D Depth Texture"),
            size: wgpu::Extent3d {
                width: config.width,
                height: config.height,
                depth_or_array_layers: 1
            },
            mip_level_count: 1,
            sample_count: MSAA_COUNT,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[]
        };

        let depth_texture_2d = device.create_texture(&depth_tex_desc_2d);

        let depth_texture_view_2d = depth_texture_2d.create_view(&wgpu::TextureViewDescriptor::default());

        let msaa_framebuffer_desc = wgpu::TextureDescriptor {
            label: Some("3D MSAA FrameBuffer"),
            size: wgpu::Extent3d {
                width: config.width,
                height: config.height,
                depth_or_array_layers: 1
            },
            mip_level_count: 1,
            sample_count: MSAA_COUNT,
            dimension: wgpu::TextureDimension::D2,
            format: surface_format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[]
        };

        let msaa_framebuffer = device.create_texture(&msaa_framebuffer_desc);

        let camera = Camera {
            eye: (0.0, CAMERA_DISTANCE, CAMERA_DISTANCE).into(),
            target: (0.0, 0.0, 0.0).into(),
            up: cgmath::Vector3::unit_y(),
            aspect: config.width as f32 / config.height as f32,
            fovy: CAMERA_FOV,
            znear: 0.1,
            zfar: 100.0
        };

        let mut camera_uniform = CameraUniform::new();
        camera_uniform.update_view_proj(&camera);

        let mut aspect_uniform = AspectUniform::new();
        aspect_uniform.update_aspect(config.width as f32 / config.height as f32);

        let camera_buffer = device.create_buffer(
            &wgpu::BufferDescriptor {
                label: Some("Camera Buffer"),
                size: 256,
                mapped_at_creation: false,
                // contents: bytemuck::cast_slice(&[camera_uniform]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST
            }
        );

        let aspect_buffer = device.create_buffer(
            &wgpu::BufferDescriptor {
                label: Some("2D Aspect Buffer"),
                size: 256,
                mapped_at_creation: false,
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST
            }
        );

        let camera_bind_group_layout = device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None
                        },
                        count: None
                    }
                ],
                label: Some("camera_bind_group_layout")
            }
        );

        let aspect_bind_group_layout = device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None
                        },
                        count: None
                    }
                ],
                label: Some("aspect bind group layout")
            }
        );

        let camera_bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &camera_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: camera_buffer.as_entire_binding()
                    }
                ],
                label: Some("camera_bind_group")
            }
        );

        let aspect_bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &aspect_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: aspect_buffer.as_entire_binding()
                    }
                ],
                label: Some("Aspect bind group")
            }
        );

        let render_pipeline_layout_3d = 
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("3D Render Pipeline Layout"),
                bind_group_layouts: &[
                    &camera_bind_group_layout,
                ],
                push_constant_ranges: &[]
            }
        );

        let render_pipeline_3d = device.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor {
                label: Some("3D Render Pipeline"),
                layout: Some(&render_pipeline_layout_3d),
                vertex: wgpu:: VertexState {
                    module: &shader_3d,
                    entry_point: "vs_main",
                    buffers: &[
                        Vertex3D::desc(),
                    ]
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader_3d,
                    entry_point: "fs_main",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: config.format,
                        blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                        write_mask: wgpu::ColorWrites::ALL,
                    })]
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Cw,
                    cull_mode: Some(wgpu::Face::Back),
                    polygon_mode: wgpu::PolygonMode::Fill,
                    unclipped_depth: false,
                    conservative: false
                },
                depth_stencil: Some(wgpu::DepthStencilState {
                    format: wgpu::TextureFormat::Depth32Float,
                    depth_write_enabled: true,
                    depth_compare: wgpu::CompareFunction::Less,
                    stencil: wgpu::StencilState::default(),
                    bias: wgpu::DepthBiasState::default()
                }),
                multisample: wgpu::MultisampleState {
                    count: MSAA_COUNT,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                multiview: None
            }
        );

        let render_pipeline_layout_2d = 
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("2D Render Pipeline Layout"),
                bind_group_layouts: &[
                    &aspect_bind_group_layout,
                    &texture_bind_group_layout
                ],
                push_constant_ranges: &[]
            }
        );

        let render_pipeline_2d = device.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor {
                label: Some("2D Render Pipeline"),
                layout: Some(&render_pipeline_layout_2d),
                vertex: wgpu:: VertexState {
                    module: &shader_2d,
                    entry_point: "vs_main",
                    buffers: &[
                        Vertex2D::desc(),
                    ]
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader_2d,
                    entry_point: "fs_main",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: config.format,
                        blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                        write_mask: wgpu::ColorWrites::ALL,
                    })]
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Cw,
                    // cull_mode: Some(wgpu::Face::Back),
                    cull_mode: None,
                    polygon_mode: wgpu::PolygonMode::Fill,
                    unclipped_depth: false,
                    conservative: false
                },
                depth_stencil: Some(wgpu::DepthStencilState {
                    format: wgpu::TextureFormat::Depth32Float,
                    depth_write_enabled: true,
                    depth_compare: wgpu::CompareFunction::Less,
                    stencil: wgpu::StencilState::default(),
                    bias: wgpu::DepthBiasState::default()
                }),
                multisample: wgpu::MultisampleState {
                    count: MSAA_COUNT,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                multiview: None
            }
        );

        let clearcol = wgpu::Color {
            r: clearcolf32[0] as f64,
            g: clearcolf32[1] as f64,
            b: clearcolf32[2] as f64,
            a: clearcolf32[3] as f64,
        };

        Self {
            window,
            surface,
            surface_format,
            device,
            queue,
            config,
            size,
            clearcol,
            render_pipeline_3d,
            vertex_buffer_3d,
            index_buffer_3d,
            camera,
            camera_uniform,
            camera_buffer,
            camera_bind_group,
            depth_texture_3d,
            depth_texture_view_3d,
            msaa_framebuffer,
            index_buffer_2d,
            vertex_buffer_2d,
            render_pipeline_2d,
            aspect_uniform,
            aspect_bind_group,
            aspect_buffer,
            depth_texture_view_2d,
            depth_texture_2d,
            texture_bind_group
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);

            let aspect = self.config.width as f32 / self.config.height as f32;

            self.camera.aspect = aspect;

            self.aspect_uniform.update_aspect(aspect);

            let depth_tex_desc = wgpu::TextureDescriptor {
                label: Some("3D Depth Texture"),
                size: wgpu::Extent3d {
                    width: self.config.width,
                    height: self.config.height,
                    depth_or_array_layers: 1
                },
                mip_level_count: 1,
                sample_count: MSAA_COUNT,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Depth32Float,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                    | wgpu::TextureUsages::TEXTURE_BINDING,
                view_formats: &[]
            };

            self.depth_texture_3d = self.device.create_texture(&depth_tex_desc);

            self.depth_texture_view_3d = self.depth_texture_3d.create_view(&wgpu::TextureViewDescriptor::default());

            let d2_depth_tex_desc = wgpu::TextureDescriptor {
                label: Some("2D Depth Texture"),
                size: wgpu::Extent3d {
                    width: self.config.width,
                    height: self.config.height,
                    depth_or_array_layers: 1
                },
                mip_level_count: 1,
                sample_count: MSAA_COUNT,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Depth32Float,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                    | wgpu::TextureUsages::TEXTURE_BINDING,
                view_formats: &[]
            };

            self.depth_texture_2d = self.device.create_texture(&d2_depth_tex_desc);

            self.depth_texture_view_2d = self.depth_texture_2d.create_view(&wgpu::TextureViewDescriptor::default());

            let msaa_framebuffer_desc = wgpu::TextureDescriptor {
                label: Some("MSAA FrameBuffer"),
                size: wgpu::Extent3d {
                    width: self.config.width,
                    height: self.config.height,
                    depth_or_array_layers: 1
                },
                mip_level_count: 1,
                sample_count: MSAA_COUNT,
                dimension: wgpu::TextureDimension::D2,
                format: self.surface_format,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                view_formats: &[]
            };

            self.msaa_framebuffer = self.device.create_texture(&msaa_framebuffer_desc);
        }
    }

    // resize to the same as the current size (so basically just reinit)
    pub fn fake_resize(&mut self) {
        self.resize(self.size)
    }

    pub fn update(&mut self) {
        self.camera_uniform.update_view_proj(&self.camera);
        self.queue.write_buffer(&self.camera_buffer, 0, bytemuck::cast_slice(&[self.camera_uniform]))
    }

    pub fn set_bg_col(&mut self, col: [f32;4]) {
        let clearcol = wgpu::Color {
            r: col[0] as f64,
            g: col[1] as f64,
            b: col[2] as f64,
            a: col[3] as f64,
        };
        self.clearcol = clearcol;
    }

    pub fn render<T, U>(&mut self, d3_geom: T, d2_geom: U) -> Result<Duration, wgpu::SurfaceError>
    where
        T: ToVertInd3D,
        U: ToVertInd2D
    {
        let surface_output = self.surface.get_current_texture()?;
        let surface_view = surface_output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let start = Instant::now();

        let output = &self.msaa_framebuffer;
        let view = output.create_view(&wgpu::TextureViewDescriptor::default());

        let (vert3d, ind3d) = d3_geom.to_vert_ind();
        let num_indices3d = ind3d.len() as u32;

        let (vert2d, ind2d) = d2_geom.to_vert_ind();
        let num_indices2d = ind2d.len() as u32;

        self.queue.write_buffer(
            &self.vertex_buffer_3d,
            0,
            bytemuck::cast_slice(&vert3d)
        );

        self.queue.write_buffer(
            &self.index_buffer_3d,
            0,
            bytemuck::cast_slice(&ind3d)
        );

        self.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.camera_uniform])
        );

        self.queue.write_buffer(
            &self.vertex_buffer_2d,
            0,
            bytemuck::cast_slice(&vert2d)
        );

        self.queue.write_buffer(
            &self.index_buffer_2d,
            0,
            bytemuck::cast_slice(&ind2d)
        );

        self.queue.write_buffer(
            &self.aspect_buffer,
            0,
            bytemuck::cast_slice(&[self.aspect_uniform])
        );

        let mut encoder = self.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor {
                label: Some("Render encoder")
            }
        );

        {
            let mut render_pass3d = encoder.begin_render_pass(
                &wgpu::RenderPassDescriptor {
                    label: Some("3D Render pass"),
                    depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                        view: &self.depth_texture_view_3d,
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Clear(1.0),
                            store: wgpu::StoreOp::Store
                        }),
                        stencil_ops: None
                    }),
                    occlusion_query_set: None,
                    timestamp_writes: None,
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: Some(&surface_view),
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(self.clearcol),
                            store: wgpu::StoreOp::Store
                        }
                    })]
                }
            );

            render_pass3d.set_pipeline(&self.render_pipeline_3d);
            render_pass3d.set_bind_group(0, &self.camera_bind_group, &[]);
            render_pass3d.set_vertex_buffer(0, self.vertex_buffer_3d.slice(..));
            render_pass3d.set_index_buffer(self.index_buffer_3d.slice(..), wgpu::IndexFormat::Uint16);
            render_pass3d.draw_indexed(0..num_indices3d, 0, 0..1);
            
        }
        {
            let mut render_pass2d = encoder.begin_render_pass(
                &wgpu::RenderPassDescriptor {
                    label: Some("2D Render pass"),
                    depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                        view: &self.depth_texture_view_2d,
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Clear(1.0),
                            store: wgpu::StoreOp::Store
                        }),
                        stencil_ops: None
                    }),
                    occlusion_query_set: None,
                    timestamp_writes: None,
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: Some(&surface_view),
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Load,
                            store: wgpu::StoreOp::Store
                        }
                    })]
                }
            );

            render_pass2d.set_pipeline(&self.render_pipeline_2d);
            render_pass2d.set_bind_group(0, &self.aspect_bind_group, &[]);
            render_pass2d.set_bind_group(1, &self.texture_bind_group, &[]);
            render_pass2d.set_vertex_buffer(0, self.vertex_buffer_2d.slice(..));
            render_pass2d.set_index_buffer(self.index_buffer_2d.slice(..), wgpu::IndexFormat::Uint16);
            render_pass2d.draw_indexed(0..num_indices2d, 0, 0..1);

        }

        self.queue.submit(std::iter::once(encoder.finish()));

        surface_output.present();

        let t = Instant::now().duration_since(start);

        Ok(t)

    }
}
























