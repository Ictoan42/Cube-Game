use std::{process::exit, sync::Arc, time::Instant};

use rodio::{OutputStream, OutputStreamHandle};
use winit::{dpi::PhysicalSize, event::{ElementState, Event, WindowEvent}, event_loop::EventLoop, keyboard::Key, window::WindowBuilder};

mod gpustate;
mod d3;
mod d2;
mod game;
mod config;
mod mathsutils;
mod soundmanager;

fn main() {
    println!("Deleting System32...");

    pollster::block_on(run());
}

pub async fn run() {

    let ev_loop = EventLoop::new().unwrap();
    let window = Arc::new(
        WindowBuilder::new()
            .with_title("Cube Game")
            .with_inner_size(PhysicalSize::new(config::WINDOW_WIDTH, config::WINDOW_HEIGHT))
            .with_resizable(false)
            .build(&ev_loop)
            .unwrap()
    );

    let textures: Vec<&[u8]> = vec![
        include_bytes!("res/black.png"),
        include_bytes!("res/roundedblackbox.png"),
        include_bytes!("res/gen/0.png"),
        include_bytes!("res/gen/1.png"),
        include_bytes!("res/gen/2.png"),
        include_bytes!("res/gen/3.png"),
        include_bytes!("res/gen/4.png"),
        include_bytes!("res/gen/5.png"),
        include_bytes!("res/gen/6.png"),
        include_bytes!("res/gen/7.png"),
        include_bytes!("res/gen/8.png"),
        include_bytes!("res/gen/9.png"),
    ];

    // (data, volume)
    let sounds: Vec<( &[u8] , f32 )> = vec![
        ( include_bytes!("res/click.wav"),  0.1 ),
        ( include_bytes!("res/click2.wav"), 0.3 ),
        ( include_bytes!("res/dingup.wav"), 0.1 ),
        ( include_bytes!("res/dinglow.wav"), 0.3 ),
    ];

    let (_stream, sh) : (Option<OutputStream>, Option<OutputStreamHandle>) = match rodio::OutputStream::try_default() {
        Ok(o) => {
            (Some(o.0), Some(o.1))
        }
        Err(_) => {
            println!("Failed to get default audio device, there will be no audio");
            (None, None)
        }
    };

    let mut since_last_ftime_readout = 0u128;
    let ftime_readout_delta = 120u128;
    let mut render_ftime = false;
    let mut frametimes: [f64;240] = [0.0;240];

    let mut state = game::GameState::new(
        window.clone(),
        textures,
        sounds,
        sh
    ).await;

    ev_loop.run(move |event, _| match event {
        Event::WindowEvent { window_id, event } if window_id == state.window().id() => {
            match event {
                WindowEvent::CloseRequested => {
                    exit(0);
                }
                WindowEvent::Resized(psize) => {
                    state.resize_window(psize)
                }
                WindowEvent::KeyboardInput {
                    event: kbevent,
                    ..
                } => {
                    if kbevent.logical_key == Key::Named(winit::keyboard::NamedKey::Escape) {
                        exit(0);
                    }
                    if kbevent.logical_key == Key::Named(winit::keyboard::NamedKey::Enter)
                        && kbevent.state == ElementState::Pressed {
                    }
                    match kbevent.logical_key.clone() {
                        Key::Character(char) if kbevent.state == ElementState::Pressed => {
                            match char.as_str() {
                                #[cfg(debug_assertions)]
                                "d" => {
                                    println!("Printing net debug info:\n");
                                    state.print_net_debug();
                                }
                                #[cfg(debug_assertions)]
                                "f" => {
                                    println!("Toggling frametime debug");
                                    render_ftime = !render_ftime;
                                }
                                // #[cfg(debug_assertions)]
                                // "g" => {
                                //     println!("Toggling generation time debug");
                                //     state.debug_print_gen_time = !state.debug_print_gen_time;
                                // }
                                // #[cfg(debug_assertions)]
                                // "t" => {
                                //     println!("Toggling timer override");
                                //     state.toggle_timer_override();
                                // }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
                WindowEvent::CursorMoved { position, .. } => {
                    state.mouse_pos_update(position);
                }
                WindowEvent::MouseInput { state: s, button: b, .. } => {
                    state.mouse_click_update(s, b);
                }
                WindowEvent::RedrawRequested => {
                    let start = Instant::now();
                    if since_last_ftime_readout > ftime_readout_delta {
                        since_last_ftime_readout = 0;
                        let mut max: f64 = 0.0;
                        let mut acc: f64 = 0.0;
                        for i in 0..frametimes.len() {
                            acc += frametimes[i];
                            if frametimes[i] > max {
                                max = frametimes[i]
                            }
                        }
                        let avg = acc / frametimes.len() as f64;
                        if render_ftime {
                            println!(
                                "Average frame time: {:.2}ms Peak: {:.2}ms",
                                avg,
                                max
                            );
                        }
                    } else {
                        since_last_ftime_readout += 1;
                    }
                    state.update();

                    let end = Instant::now();

                    match state.render() {
                        Ok(d) => {
                            frametimes.rotate_left(1);
                            frametimes[239] = {
                                let t = end.duration_since(start) + d;
                                t.as_micros() as f64 / 1000.0
                            };
                            window.request_redraw()
                        },
                        Err(wgpu::SurfaceError::Lost) => state.refresh_window(),
                        Err(wgpu::SurfaceError::OutOfMemory) => exit(0),
                        Err(e) => eprintln!("{:?}", e)
                    }

                }
                _ => {}
            }
        }
        _ => {}
    }).unwrap();
}

