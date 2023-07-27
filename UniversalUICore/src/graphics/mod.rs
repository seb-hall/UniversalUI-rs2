
use crate::base::*;
use crate::debug::*;
use crate::window::*;

use crate::wgpu::*;
use crate::winit::*;
use crate::winit::window::*;
use crate::raw_window_handle::*;
use crate::pollster::*;
use std::collections::HashMap;
use std::iter::once;
use std::time::{Duration, Instant};


pub struct uGraphicsWindow {
    raw_window: winit::window::Window,
    surface: wgpu::Surface,
    device: wgpu::Device,
    adapter: wgpu::Adapter,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: uSize,
    render_pipeline: wgpu::RenderPipeline,
}

static mut graphics_windows: Option<HashMap<u64, uGraphicsWindow>> = None;
static mut instance: Option<wgpu::Instance> = None;

pub unsafe fn graphics_init() -> bool {

    internal_debug_info("initialising UniversalUI-Graphics");

    instance = Some(Instance::new(wgpu::InstanceDescriptor {
        backends: Backends::all(),
        dx12_shader_compiler: Default::default(),
    }));

    unsafe {
        graphics_windows = Some(HashMap::new());
    }
    

    return true;
}

pub unsafe fn setup_for_window(handle: uWindowHandle, raw_window: winit::window::Window, size: uSize) -> bool {
    
    let surface: wgpu::Surface;
    let adapter: wgpu::Adapter;
    let device: wgpu::Device;
    let queue: wgpu::Queue;

    unsafe {
        if let Some(ref mut safe_instance) = instance {
            surface = match unsafe { safe_instance.create_surface(&raw_window) } {
                Ok(surface) => {internal_debug_info("surface ok"); surface}
                Err(_) => {
                    internal_debug_error("couldnt create surface");
                    panic!()
                }
            };

            internal_debug_info("got surface");

            adapter = pollster::block_on( 
                async { 
                    safe_instance.request_adapter(
                        &wgpu::RequestAdapterOptions {
                            power_preference: wgpu::PowerPreference::default(),
                            compatible_surface: Some(&surface),
                            force_fallback_adapter: false,
                        },
                    ).await.unwrap() 
                } 
            );

        } else {
            internal_debug_critical("failed to unwrap wgpu instance - has graphics_init() been called?");
            panic!()
        }
    }


    


    let (device, queue): (Device, Queue) = pollster::block_on(
        async { 
            adapter.request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    label: None,
                },
                None, // Trace path
            ).await.unwrap()
        }
    );

    let surface_caps = surface.get_capabilities(&adapter);
    // Shader code in this tutorial assumes an sRGB surface texture. Using a different
    // one will result all the colors coming out darker. If you want to support non
    // sRGB surfaces, you'll need to account for that when drawing to the frame.
    let surface_format = surface_caps.formats.iter()
        .copied()
        .find(|f| f.is_srgb())            
        .unwrap_or(surface_caps.formats[0]);
    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface_format,
        width: size.width as u32,
        height: size.height as u32,
        present_mode: wgpu::PresentMode::AutoNoVsync,//surface_caps.present_modes[0],
        alpha_mode: surface_caps.alpha_modes[0],
        view_formats: vec![],
    };
    surface.configure(&device, &config);

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
    });

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
                blend: Some(wgpu::BlendState {
                    color: wgpu::BlendComponent::REPLACE,
                    alpha: wgpu::BlendComponent::REPLACE,
                }),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: false,
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


    let window_stuff = uGraphicsWindow {
        raw_window,
        surface,
        device,
        adapter,
        queue,
        config,
        size: uSize { width: size.width, height: size.height },
        render_pipeline,
    };

    unsafe {
        if let Some(ref mut safe_graphics_windows) = graphics_windows { 
            safe_graphics_windows.insert(handle, window_stuff);
        } else {
            internal_debug_error("failed to unwrap graphics_windows map");
        }
    }

    render_window(handle, uSize {width: size.width, height: size.height});
    
    return true;
}

//  configure window surface e.g if resized
pub unsafe fn configure_window_surface(handle: uWindowHandle, size: uSize) {


    unsafe {
        if let Some(ref mut safe_graphics_windows) = graphics_windows { 
            let mut window_stuff = safe_graphics_windows.get_mut(&handle).unwrap();
            
            let surface_caps = window_stuff.surface.get_capabilities(&window_stuff.adapter);
            // Shader code in this tutorial assumes an sRGB surface texture. Using a different
            // one will result all the colors coming out darker. If you want to support non
            // sRGB surfaces, you'll need to account for that when drawing to the frame.
            let surface_format = surface_caps.formats.iter()
                .copied()
                .find(|f| f.is_srgb())            
                .unwrap_or(surface_caps.formats[0]);
            let config = wgpu::SurfaceConfiguration {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                format: surface_format,
                width: size.width as u32,
                height: size.height as u32,
                present_mode: wgpu::PresentMode::AutoNoVsync,//surface_caps.present_modes[0],
                alpha_mode: surface_caps.alpha_modes[0],
                view_formats: vec![],
            };

            let now = Instant::now();
            let mut duration: Duration = now.elapsed();
            window_stuff.surface.configure(&window_stuff.device, &config);
            duration = now.elapsed();
            internal_debug_info(&format!("configure took {} us", duration.as_micros()));
            window_stuff.size = uSize {width: size.width, height: size.height};
        } else {
            internal_debug_error("failed to unwrap graphics_windows map");
        }
    }
    
}


//  render window surface
pub unsafe fn render_window(handle: uWindowHandle, size: uSize) {

    let now = Instant::now();
    let mut duration: Duration = now.elapsed();

    unsafe {
        if let Some(ref mut safe_graphics_windows) = graphics_windows { 
            let window_stuff = safe_graphics_windows.get(&handle).unwrap();
            if window_stuff.size.width != size.width || window_stuff.size.height != size.height {
                println!("SIZE DIFFERENT!");
                //configure_window_surface(handle, uSize {width: size.width, height: size.height});
            }
            let output = window_stuff.surface.get_current_texture().unwrap();
            let view = output
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default());
            let mut encoder = window_stuff
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
    
                render_pass.set_pipeline(&window_stuff.render_pipeline);
            }

            
    
            window_stuff.queue.submit(once(encoder.finish()));

            duration = now.elapsed();
            internal_debug_info(&format!("everything else took {} us", duration.as_micros()));

            let now2 = Instant::now();
            let mut duration2: Duration = now2.elapsed();


            output.present();

            duration2 = now2.elapsed();
            internal_debug_info(&format!("present took {} us", duration2.as_micros()));

        } else {
            internal_debug_error("failed to unwrap graphics_windows map");
        }
    }

}