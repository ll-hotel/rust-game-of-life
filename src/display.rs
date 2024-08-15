use sdl2::{event::Event, pixels::Color, rect::Rect, render::WindowCanvas, EventPump};
use std::cell::RefCell;

pub struct Display {
    canvas: RefCell<WindowCanvas>,
    event_pump: RefCell<EventPump>,
    pub width: u32,
    pub height: u32,
}

impl Display {
    pub fn new(width: u32, height: u32) -> Self {
        let sdl_ctx = sdl2::init().expect("Sdl should init");
        let video = sdl_ctx.video().expect("Video subsystem should start");
        let window = video
            .window("Game of life", width, height)
            .build()
            .expect("Window should be created");
        let canvas = RefCell::new(
            window
                .into_canvas()
                .build()
                .expect("Window canvas should be created"),
        );
        let event_pump = RefCell::new(sdl_ctx.event_pump().expect("Event pump should be created"));
        Self {
            canvas,
            event_pump,
            width,
            height,
        }
    }

    pub fn clear(&self) {
        self.canvas.borrow_mut().clear();
    }

    pub fn set_draw_color(&self, r: u8, g: u8, b: u8) {
        self.canvas.borrow_mut().set_draw_color(Color::RGB(r, g, b));
    }

    pub fn draw_line(&self, x0: u32, y0: u32, x1: u32, y1: u32) {
        self.canvas
            .borrow_mut()
            .draw_line((x0 as i32, y0 as i32), (x1 as i32, y1 as i32))
            .expect("Draw call should not fail");
    }

    pub fn fill_rect(&self, x: u32, y: u32, size: u32) {
        self.canvas
            .borrow_mut()
            .fill_rect(Rect::new(x as i32, y as i32, size, size))
            .expect("Draw call should not fail");
    }

    pub fn present(&self) {
        self.canvas.borrow_mut().present();
    }

    pub fn get_events(&self) -> Vec<Event> {
        self.event_pump.borrow_mut().poll_iter().collect()
    }
}
