use wgpu::{include_wgsl, Device, RenderPipeline, ShaderModule, SurfaceConfiguration};
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

fn main() {
    let event_loop = EventLoop::new();
    let window = winit::window::Window::new(&event_loop).unwrap();
    #[cfg(not(target_arch = "wasm32"))]
    {
        env_logger::init();
        // Temporarily avoid srgb formats for the swapchain on the web
        pollster::block_on(run(event_loop, window));
    }
    #[cfg(target_arch = "wasm32")]
    {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init().expect("could not initialize logger");
        use winit::dpi::PhysicalSize;
        use winit::platform::web::WindowExtWebSys;
        // On wasm, append the canvas to the document body
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| {
                match doc.get_element_by_id("wasm-example") {
                    Some(dst) => {
                        window.set_inner_size(PhysicalSize::new(450, 400));
                        let _ = dst.append_child(&web_sys::Element::from(window.canvas()));
                    }
                    None => {
                        window.set_inner_size(PhysicalSize::new(800, 800));
                        let canvas = window.canvas();
                        canvas.style().set_css_text(
                            "background-color: black; display: block; margin: 20px auto;",
                        );
                        doc.body().and_then(|body| {
                            Some(body.append_child(&web_sys::Element::from(canvas)))
                        });
                    }
                };
                Some(())
            })
            .expect("couldn't append canvas to document body");
        wasm_bindgen_futures::spawn_local(run(event_loop, window));
    }
}

struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    render_pipeline: wgpu::RenderPipeline,
    render_pipeline2: wgpu::RenderPipeline,
    press_space: bool,
}

impl State {
    // ???????????? wgpu ??????????????????????????????
    async fn new(window: &Window) -> Self {
        let size = window.inner_size();

        // instance ????????? GPU ??????
        // Backends::all ?????? Vulkan???Metal???DX12???WebGL ???????????????????????????
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    // https://docs.rs/wgpu/latest/wgpu/struct.Features.html
                    features: wgpu::Features::empty(),
                    // WebGL ?????????????????? wgpu ??????????????????
                    // ?????????????????? web ????????????????????????????????????????????????
                    limits: if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                    label: None,
                },
                // Some(&std::path::Path::new("trace")), // Trace path
                None,
            )
            .await
            .unwrap();

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_supported_formats(&adapter)[0],
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
        };

        surface.configure(&device, &config);
        let shader = device.create_shader_module(include_wgsl!("shader.wgsl"));
        let render_pipeline = Self::create_render_pipeline(&device, &config, shader);
        let shader2 = device.create_shader_module(include_wgsl!("challenge.wgsl"));
        let render_pipeline2 = Self::create_render_pipeline(&device, &config, shader2);
        Self {
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
            render_pipeline2,
            press_space: false,
        }
    }

    fn create_render_pipeline(
        device: &Device,
        config: &SurfaceConfiguration,
        shader: ShaderModule,
    ) -> RenderPipeline {
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                // ??????????????? Fill ???????????????????????????????????? Feature::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // ???????????? Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // ???????????? Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });
        render_pipeline
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    // ???????????????????????????????????????
    fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state,
                        virtual_keycode: Some(VirtualKeyCode::Space),
                        ..
                    },
                ..
            } => {
                self.press_space = *state == ElementState::Pressed;
                true
            }
            _ => false,
        }
    }

    fn update(&mut self) {}

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,

                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });
            // or drop(render_pass) to release borrow `encoder`
            if self.press_space {
                render_pass.set_pipeline(&self.render_pipeline2);
            } else {
                render_pass.set_pipeline(&self.render_pipeline);
            }
            render_pass.draw(0..3, 0..1);
        }

        // submit ?????????????????????????????? IntoIter trait ?????????
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

pub async fn run(event_loop: EventLoop<()>, window: Window) {
    // State::new uses async code, so we're going to wait for it to finish
    let mut state = State::new(&window).await;

    // vulkan window resize warning https://github.com/rust-windowing/winit/issues/2094
    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => {
            if !state.input(event) {
                match event {
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(physical_size) => {
                        state.resize(*physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        state.resize(**new_inner_size);
                    }
                    _ => {}
                }
            }
        }
        Event::RedrawRequested(window_id) if window_id == window.id() => {
            state.update();
            match state.render() {
                Ok(_) => {}
                // ??????????????????????????????????????????????????????
                Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                // ?????????????????????????????????????????????
                Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                // ???????????????????????????????????????????????????????????????
                Err(e) => eprintln!("{:?}", e),
            }
        }
        Event::MainEventsCleared => {
            // ???????????????????????????RedrawRequested ????????????????????????
            window.request_redraw();
        }
        _ => {}
    });
}
