//! Minimal wgpu renderer for the scaffold (WASM / browser).
//!
//! Draws:
//! - a paddle (rectangle)
//! - a ball (circle)
//!
//! This renderer is intentionally simple and "tutorial shaped":
//! - fullscreen triangle
//! - fragment shader draws shapes from a small uniform buffer
//!
//! Later phases will update the uniform data from the simulated `World`.

use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::HtmlCanvasElement;

use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct SceneUniforms {
    // Normalized device-ish coordinates in [0..1] space in the fragment shader.
    paddle_x: f32,
    paddle_y: f32,
    paddle_w: f32,
    paddle_h: f32,
    ball_x: f32,
    ball_y: f32,
    ball_r: f32,
    aspect: f32,
    _pad0: f32,
    _pad1: f32,
    _pad2: f32,
}

pub async fn run_canvas(canvas: HtmlCanvasElement) -> anyhow::Result<()> {
    // Note: we keep everything inside this async function so the wasm entrypoint
    // can `spawn_local` it.

    let state = RenderState::new(canvas).await?;
    let state = Rc::new(RefCell::new(state));

    start_animation_loop(state)?;
    Ok(())
}

struct RenderState {
    canvas: HtmlCanvasElement,
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    width: u32,
    height: u32,
    pipeline: wgpu::RenderPipeline,
    uniforms: SceneUniforms,
    uniforms_buffer: wgpu::Buffer,
    uniforms_bind_group: wgpu::BindGroup,
}

impl RenderState {
    async fn new(canvas: HtmlCanvasElement) -> anyhow::Result<Self> {
        // wgpu instance + surface.
        let instance = wgpu::Instance::default();

        // Create surface directly from the canvas to avoid lifetime issues.
        //
        // This is the most beginner-friendly way to run wgpu on the web.
        let surface = instance.create_surface(wgpu::SurfaceTarget::Canvas(canvas.clone()))?;

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .ok_or_else(|| anyhow::anyhow!("No suitable GPU adapter found"))?;

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("device"),
                    required_features: wgpu::Features::empty(),
                    // Match the working pattern used in MagCreate (wgpu 23):
                    // keep limits at defaults and let wgpu negotiate safely.
                    required_limits: wgpu::Limits::default(),
                    memory_hints: Default::default(),
                    ..Default::default()
                },
                None,
            )
            .await
            .map_err(|e| anyhow::anyhow!("{e:?}"))?;

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: canvas.width().max(1),
            height: canvas.height().max(1),
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        // Initial "static" scene values (tutorial will replace with sim state).
        let aspect = (config.width as f32) / (config.height as f32).max(1.0);
        let uniforms = SceneUniforms {
            paddle_x: 0.5,
            paddle_y: 0.12,
            paddle_w: 0.20,
            paddle_h: 0.04,
            ball_x: 0.5,
            ball_y: 0.65,
            ball_r: 0.03,
            aspect,
            _pad0: 0.0,
            _pad1: 0.0,
            _pad2: 0.0,
        };

        let uniforms_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("scene uniforms"),
            contents: bytemuck::bytes_of(&uniforms),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let uniforms_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("uniforms layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let uniforms_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("uniforms bind group"),
            layout: &uniforms_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniforms_buffer.as_entire_binding(),
            }],
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("paddleball shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("pipeline layout"),
            bind_group_layouts: &[&uniforms_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("render pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        let width = config.width;
        let height = config.height;

        Ok(Self {
            canvas,
            surface,
            device,
            queue,
            config,
            width,
            height,
            pipeline,
            uniforms,
            uniforms_buffer,
            uniforms_bind_group,
        })
    }

    fn resize_if_needed(&mut self) {
        // Canvas pixel size is controlled by JS (see `web/src/main.ts`).
        let w = self.canvas.width().max(1);
        let h = self.canvas.height().max(1);
        let needs_resize = crate::wasm_api::take_needs_resize();
        if needs_resize || w != self.width || h != self.height {
            self.width = w;
            self.height = h;
            self.config.width = w;
            self.config.height = h;
            self.surface.configure(&self.device, &self.config);
            self.uniforms.aspect = (w as f32) / (h as f32).max(1.0);
        }
    }

    fn update(&mut self) {
        // Scaffold: static scene (no animation).
        // Keep these values stable; later phases can replace this with simulation.
        self.uniforms.ball_x = 0.5;
        self.uniforms.ball_y = 0.65;

        self.queue
            .write_buffer(&self.uniforms_buffer, 0, bytemuck::bytes_of(&self.uniforms));
    }

    fn render(&mut self) -> anyhow::Result<()> {
        self.resize_if_needed();

        let frame = self.surface.get_current_texture()?;
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("render encoder"),
            });

        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("render pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.06,
                            g: 0.07,
                            b: 0.09,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            rpass.set_pipeline(&self.pipeline);
            rpass.set_bind_group(0, &self.uniforms_bind_group, &[]);
            rpass.draw(0..3, 0..1);
        }

        self.queue.submit(Some(encoder.finish()));
        frame.present();
        Ok(())
    }
}

fn start_animation_loop(state: Rc<RefCell<RenderState>>) -> anyhow::Result<()> {
    let window = web_sys::window().ok_or_else(|| anyhow::anyhow!("no window"))?;
    let window_for_loop = window.clone();

    // We need a self-referential closure pattern to schedule RAF repeatedly.
    let f: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        {
            let mut s = state.borrow_mut();
            s.update();
            if let Err(err) = s.render() {
                log::error!("render error: {err:?}");
            }
        }

        // Schedule next frame.
        let cb = f.borrow();
        let cb = cb.as_ref().expect("closure must exist");
        let _ = window_for_loop.request_animation_frame(cb.as_ref().unchecked_ref());
    }) as Box<dyn FnMut()>));

    // Kick off the loop once.
    let cb = g.borrow();
    let cb = cb.as_ref().expect("closure must exist");
    window
        .request_animation_frame(cb.as_ref().unchecked_ref())
        .map_err(|e| anyhow::anyhow!("requestAnimationFrame failed: {e:?}"))?;

    Ok(())
}


