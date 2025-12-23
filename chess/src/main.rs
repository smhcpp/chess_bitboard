#![allow(dead_code)]
use ggez::conf;
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color};
use ggez::{Context, ContextBuilder, GameResult};
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
    let g = Chess::new(&mut ctx);

    // Run!
    event::run(ctx, event_loop, g);
}

struct Piece {
    bitmask: u64,
    image: graphics::Image,
}

struct Chess {
    bitboards: [u64; 12],
    pieces: [Piece; 12],
    square_size: f32,
    texture_size: f32,
    square_mesh: graphics::Mesh,
}

impl Chess {
    fn draw_board(&mut self, canvas: &mut graphics::Canvas) {
        for i in 0..8 {
            for j in 0..8 {
                let color = if (i + j) % 2 == 0 {
                    Color::from_rgb(233, 233, 233)
                } else {
                    Color::from_rgb(40, 40, 40)
                };
                let x = i as f32 * self.square_size;
                let y = j as f32 * self.square_size;
                let param = graphics::DrawParam::default().dest([x, y]).color(color);
                canvas.draw(&self.square_mesh, param);
            }
        }
    }

    fn draw_pieces(&mut self, canvas: &mut graphics::Canvas) {
        for i in 0..8 {
            for j in 0..8 {
                for piece in &self.pieces {
                    let mask = 1u64 << (j * 8 + i);
                    if piece.bitmask & mask != 0 {
                        let x = i as f32 * self.square_size;
                        let y = j as f32 * self.square_size;
                        let param = graphics::DrawParam::default().dest([x, y]).scale([
                            self.texture_size / self.square_size,
                            self.texture_size / self.square_size,
                        ]);
                        canvas.draw(&piece.image, param);
                    }
                }
            }
        }
    }

    pub fn new(_ctx: &mut Context) -> Chess {
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
            pieces: [
                Piece {
                    bitmask: wpawn_bitmask,
                    image: wpawn_image,
                },
                Piece {
                    bitmask: wknight_bitmask,
                    image: wknight_image,
                },
                Piece {
                    bitmask: wbishop_bitmask,
                    image: wbishop_image,
                },
                Piece {
                    bitmask: wrook_bitmask,
                    image: wrook_image,
                },
                Piece {
                    bitmask: wqueen_bitmask,
                    image: wqueen_image,
                },
                Piece {
                    bitmask: wking_bitmask,
                    image: wking_image,
                },
                Piece {
                    bitmask: bpawn_bitmask,
                    image: bpawn_image,
                },
                Piece {
                    bitmask: bknight_bitmask,
                    image: bknight_image,
                },
                Piece {
                    bitmask: bbishop_bitmask,
                    image: bbishop_image,
                },
                Piece {
                    bitmask: brook_bitmask,
                    image: brook_image,
                },
                Piece {
                    bitmask: bqueen_bitmask,
                    image: bqueen_image,
                },
                Piece {
                    bitmask: bking_bitmask,
                    image: bking_image,
                },
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

impl EventHandler for Chess {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Update code here...
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        self.draw_board(&mut canvas);
        self.draw_pieces(&mut canvas);
        canvas.finish(ctx)
    }
}
