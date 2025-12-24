#![allow(dead_code)]
use ggez::conf;
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color};
use ggez::{Context, ContextBuilder, GameResult};
// use std::collections::HashMap;
fn main() {
    let width: f32 = 640.0;
    let height: f32 = 640.0;
    let assets = std::path::PathBuf::from("./assets");
    let (mut ctx, event_loop) = ContextBuilder::new("Chess", "Leasy")
        .add_resource_path(assets)
        .window_mode(conf::WindowMode::default().dimensions(width, height))
        .window_setup(conf::WindowSetup::default().title("Chess Bitboard"))
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let g = Chess::new(&mut ctx, width, height);

    // Run!
    event::run(ctx, event_loop, g);
}

struct Chess {
    mouse_position: [f32; 2],
    from_square: u64,
    to_square: u64,
    width: f32,
    height: f32,
    bitboards: [u64; 12],
    images: [graphics::Image; 12],
    square_size: f32,
    texture_size: f32,
    square_mesh: graphics::Mesh,
}

impl Chess {
    fn move(&mut self){
        if self.from_square == 0 && self.to_square == 0 {return}
    }
    fn draw_board(&mut self, canvas: &mut graphics::Canvas) {
        for i in 0..8 {
            for j in 0..8 {
                let color = if (i + j) % 2 == 0 {
                    Color::from_rgb(233, 233, 233)
                } else {
                    Color::from_rgb(70, 70, 70)
                };
                let x = i as f32 * self.square_size;
                let y = j as f32 * self.square_size;
                let param = graphics::DrawParam::default().dest([x, y]).color(color);
                canvas.draw(&self.square_mesh, param);
            }
        }
    }

    fn draw_pieces(&mut self, canvas: &mut graphics::Canvas) {
        let original_size = 128.0;
        let scale = 0.6;
        let offset = (self.square_size - original_size * scale) / 2.0;
        for i in 0..8 {
            for j in 0..8 {
                for piece_index in 0..self.images.len() {
                    let mask = 1u64 << (j * 8 + i);
                    if self.bitboards[piece_index] & mask != 0 {
                        let x = self.width - (i + 1) as f32 * self.square_size;
                        let y = self.height - (j + 1) as f32 * self.square_size;
                        let param = graphics::DrawParam::default()
                            .dest([x + offset, y + offset])
                            .scale([scale, scale]);
                        canvas.draw(&self.images[piece_index], param);
                    }
                }
            }
        }
    }

    pub fn new(_ctx: &mut Context, width: f32, height: f32) -> Chess {
        let square_size: f32 = 80.0;
        let rect = graphics::Rect::new(0.0, 0.0, square_size, square_size);
        let bpawn_image =
            graphics::Image::from_path(_ctx, "/bpawn.png").expect("Could not load image");
        let bknight_image =
            graphics::Image::from_path(_ctx, "/bknight.png").expect("Could not load image");
        let bbishop_image =
            graphics::Image::from_path(_ctx, "/bbishop.png").expect("Could not load image");
        let brook_image =
            graphics::Image::from_path(_ctx, "/brook.png").expect("Could not load image");
        let bqueen_image =
            graphics::Image::from_path(_ctx, "/bqueen.png").expect("Could not load image");
        let bking_image =
            graphics::Image::from_path(_ctx, "/bking.png").expect("Could not load image");
        let wpawn_image =
            graphics::Image::from_path(_ctx, "/wpawn.png").expect("Could not load image");
        let wknight_image =
            graphics::Image::from_path(_ctx, "/wknight.png").expect("Could not load image");
        let wbishop_image =
            graphics::Image::from_path(_ctx, "/wbishop.png").expect("Could not load image");
        let wrook_image =
            graphics::Image::from_path(_ctx, "/wrook.png").expect("Could not load image");
        let wqueen_image =
            graphics::Image::from_path(_ctx, "/wqueen.png").expect("Could not load image");
        let wking_image =
            graphics::Image::from_path(_ctx, "/wking.png").expect("Could not load image");
        let wpawn_bitmask = 1u64 << 8
            | 1u64 << 9
            | 1u64 << 10
            | 1u64 << 11
            | 1u64 << 12
            | 1u64 << 13
            | 1u64 << 14
            | 1u64 << 15;
        let wknight_bitmask = 1u64 << 1 | 1u64 << 6;
        let wbishop_bitmask = 1u64 << 2 | 1u64 << 5;
        let wrook_bitmask = 1u64 << 0 | 1u64 << 7;
        let wqueen_bitmask = 1u64 << 4;
        let wking_bitmask = 1u64 << 3;
        let bpawn_bitmask = 1u64 << 48
            | 1u64 << 49
            | 1u64 << 50
            | 1u64 << 51
            | 1u64 << 52
            | 1u64 << 53
            | 1u64 << 54
            | 1u64 << 55;
        let bknight_bitmask = 1u64 << 57 | 1u64 << 62;
        let bbishop_bitmask = 1u64 << 58 | 1u64 << 61;
        let brook_bitmask = 1u64 << 56 | 1u64 << 63;
        let bqueen_bitmask = 1u64 << 60;
        let bking_bitmask = 1u64 << 59;
        Chess {
            mouse_position: [0.0, 0.0],
            from_square: 0,
            to_square: 0,
            width: width,
            height: height,
            images: [
                wpawn_image,
                wknight_image,
                wbishop_image,
                wrook_image,
                wqueen_image,
                wking_image,
                bpawn_image,
                bknight_image,
                bbishop_image,
                brook_image,
                bqueen_image,
                bking_image,
            ],
            bitboards: [
                wpawn_bitmask,
                wknight_bitmask,
                wbishop_bitmask,
                wrook_bitmask,
                wqueen_bitmask,
                wking_bitmask,
                bpawn_bitmask,
                bknight_bitmask,
                bbishop_bitmask,
                brook_bitmask,
                bqueen_bitmask,
                bking_bitmask,
            ],
            texture_size: 64.0,
            square_size,
            square_mesh: graphics::Mesh::new_rectangle(
                _ctx,
                graphics::DrawMode::fill(),
                rect,
                Color::WHITE,
            )
            .expect("Could not make the rectangle"),
        }
    }
}

pub fn get_square_mask(_x: f32, _y: f32, square_size: f32) -> u64 {
    let i = (_x / square_size) as u64;
    let j = (_y / square_size) as u64;
    let j = 7 - j;
    let mask = 7; // = 1u64 | 1u64 << 1 | 1u64 << 2
    let i = i & mask;
    let j = j & mask;
    1u64 << i + j * 8
}

impl EventHandler for Chess {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Update code here...
        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: event::MouseButton,
        _x: f32,
        _y: f32,
    ) -> Result<(), ggez::GameError> {
        if _button == event::MouseButton::Left {
            self.from_square = get_square_mask(_x, _y, self.square_size);
        }
        Ok(())
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        _button: event::MouseButton,
        _x: f32,
        _y: f32,
    ) -> Result<(), ggez::GameError> {
        if _button == event::MouseButton::Left {
            self.to_square = get_square_mask(_x, _y, self.square_size);
            self.move();
        }
        Ok(())
    }

    fn mouse_motion_event(
        &mut self,
        _ctx: &mut Context,
        _x: f32,
        _y: f32,
        _dx: f32,
        _dy: f32,
    ) -> Result<(), ggez::GameError> {
        self.mouse_position = [_x, _y];
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        self.draw_board(&mut canvas);
        self.draw_pieces(&mut canvas);
        canvas.finish(ctx)
    }
}
