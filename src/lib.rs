mod client;
mod common;
mod server;

use {
    std::path::Path,
    client::Client,
    server::Server,
    winit::{
        event::*,
        event_loop::EventLoop,
        window::{WindowBuilder, Window},
    }
};

pub fn run() {
    unsafe { if let Err(e) = common::logger::Logger::init(log::LevelFilter::Warn) {
        println!("Logger has already been set (source code error): '{e}'");
        return
    }};

    let (event_loop, window) = match create_window() {
        Ok(v) => v,
        Err(e) => {log::error!("{}", e); return}
    };

    let mut server: Option<Server> = None;

    let args = std::env::args().collect::<Vec<String>>();
    let server_addr = args.get(1);
    let addr = match server_addr {
        Some(a) => {
            a.parse().unwrap()
        },
        None => {
            let (s, a) = Server::new().unwrap();
            server = Some(s);
            a
        },
    };
    let mut client = match Client::new(window, addr) {
        Ok(v) => v,
        Err(e) => {log::error!("{}", e); return}
    };
    std::thread::spawn(move || {
        let mut last = std::time::Instant::now();
        if let Some(server) = server.as_mut() {
            loop {
                let dt = last.elapsed().as_millis();
                if dt < 100 { continue }
                last = std::time::Instant::now();
                server.update();
            }
        }
    });

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::RedrawRequested(window_id) if client.same_id(window_id) => {
                client.update(control_flow);
            }
            Event::MainEventsCleared => {
                client.req_redraw();
            }
            Event::DeviceEvent { ref event, .. } => {
                client.input(event, control_flow);
            }
            Event::WindowEvent {window_id, ref event} if client.same_id(window_id) => {
                client.window_event(event, control_flow);
            }
            _ => {}
        }
    });
}

fn create_window() -> Result<(EventLoop<()>, Window), String> {
    let icon_path = "assets/sprites/icon.png";
    let (icon_data, icon_info) = match common::files::read_texture(Path::new(icon_path)) {
        Ok(v) => v,
        Err(e) => return Err(format!("Couldn't create icon texture: {e}"))
    };
    let icon = match winit::window::Icon::from_rgba(icon_data, icon_info.width, icon_info.height) {
        Ok(v) => v,
        Err(e) => return Err(format!("Bad Icon: {e} (likely a source code error)"))
    };

    let event_loop = EventLoop::new();
    let window = match WindowBuilder::new()
        .with_title("Voxel Engine test")
        .with_window_icon(Some(icon))
        .build(&event_loop)
    {
        Ok(v) => v,
        Err(e) => return Err(format!("Couldn't build a window: {e}")),
    };

    Ok((event_loop, window))
}