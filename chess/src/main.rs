#![allow(dead_code)]
use ggez::conf;
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color};
use ggez::winit::window::CursorIcon;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChessMove {
    from_square: u64,
    to_square: u64,
}

struct Chess {
    // possible_moves: Vec<ChessMove>,
    possible_moves: [u64; 12],
    white_occupancy: u64,
    black_occupancy: u64,
    all_occupancy: u64,
    is_white_turn: bool,
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
    pub const NOT_A_FILE: u64 = 0x7f7f7f7f7f7f7f7f;
    pub const NOT_B_FILE: u64 = 0xbfbfbfbfbfbfbfbf;
    pub const NOT_G_FILE: u64 = 0xfdfdfdfdfdfdfdfd;
    pub const NOT_H_FILE: u64 = 0xfefefefefefefefe;
    pub const NOT_AB_FILE: u64 = Self::NOT_A_FILE & Self::NOT_B_FILE;
    pub const NOT_GH_FILE: u64 = Self::NOT_G_FILE & Self::NOT_H_FILE;
    fn make_move(&mut self) {
        if self.from_square == 0 || self.to_square == 0 || self.from_square == self.to_square {
            return;
        }
        if !self.is_move_possible(ChessMove {
            from_square: self.from_square,
            to_square: self.to_square,
        }) {
            return;
        }
        for i in 0..12 {
            if self.bitboards[i] & self.to_square != 0 {
                self.bitboards[i] ^= self.to_square;
            }
        }
        for i in 0..12 {
            if self.bitboards[i] & self.from_square != 0 {
                self.bitboards[i] ^= self.from_square;
                self.bitboards[i] |= self.to_square;
            }
        }
        self.white_occupancy = self.bitboards[0]
            | self.bitboards[1]
            | self.bitboards[2]
            | self.bitboards[3]
            | self.bitboards[4]
            | self.bitboards[5];
        self.black_occupancy = self.bitboards[6]
            | self.bitboards[7]
            | self.bitboards[8]
            | self.bitboards[9]
            | self.bitboards[10]
            | self.bitboards[11];
        self.all_occupancy = self.white_occupancy | self.black_occupancy;

        self.is_white_turn = !self.is_white_turn;
        self.update_possible_moves();
    }

    fn is_move_possible(&self, mv: ChessMove) -> bool {
        for piece_index in 0..self.bitboards.len() {
            if self.bitboards[piece_index] & mv.from_square != 0 {
                match piece_index % 6 {
                    0 => self.is_pawn_move_possible(mv),
                    1 => self.is_knight_move_possible(mv),
                    2 => self.is_bishop_move_possible(mv),
                    3 => self.is_rook_move_possible(mv),
                    4 => self.is_queen_move_possible(mv),
                    5 => self.is_king_move_possible(mv),
                    _ => false,
                };
            }
        }
        false
    }

    fn is_pawn_move_possible(&self, mv: ChessMove) -> bool {
        let single_move = if self.is_white_turn {
            mv.from_square << 8
        } else {
            mv.from_square >> 8
        };
        let double_move = if self.is_white_turn {
            mv.from_square << 16
        } else {
            mv.from_square >> 16
        };
        let take_right = if self.is_white_turn {
            mv.from_square << 7
        } else {
            mv.from_square >> 9
        };
        let take_left = if self.is_white_turn {
            mv.from_square << 9
        } else {
            mv.from_square >> 7
        };
        let possibilities = single_move | double_move | take_right | take_left;
        return possibilities
            & mv.to_square
            & self.possible_moves[if self.is_white_turn { 0 } else { 6 }]
            != 0;
    }
    fn is_knight_move_possible(&self, mv: ChessMove) -> bool {
        let side_index = if self.is_white_turn { 1 } else { 7 };
        let moves: u64 = ((mv.from_square << 6) & Self::NOT_AB_FILE)
            | ((mv.from_square << 10) & Self::NOT_GH_FILE)
            | ((mv.from_square << 15) & Self::NOT_A_FILE)
            | ((mv.from_square << 17) & Self::NOT_H_FILE)
            | ((mv.from_square >> 6) & Self::NOT_GH_FILE)
            | ((mv.from_square >> 10) & Self::NOT_AB_FILE)
            | ((mv.from_square >> 15) & Self::NOT_A_FILE)
            | ((mv.from_square >> 17) & Self::NOT_H_FILE);
        return moves & mv.to_square & self.possible_moves[side_index] != 0;
    }
    fn is_bishop_move_possible(&self, mv: ChessMove) -> bool {
        false
    }
    fn is_rook_move_possible(&self, mv: ChessMove) -> bool {
        false
    }
    fn is_queen_move_possible(&self, mv: ChessMove) -> bool {
        false
    }
    fn is_king_move_possible(&self, mv: ChessMove) -> bool {
        false
    }

    fn update_possible_moves(&mut self) {
        self.gen_pawn_moves(self.is_white_turn);
        self.gen_knight_moves(self.is_white_turn);
        self.gen_bishop_moves(self.is_white_turn);
        self.gen_rook_moves(self.is_white_turn);
        self.gen_queen_moves(self.is_white_turn);
        self.gen_king_moves(self.is_white_turn);
    }

    fn update_possiblee_moves(&mut self) {
        // self.possible_moves.clear();
        for piece_index in 0..self.bitboards.len() {
            let mut temp = self.bitboards[piece_index];
            while temp != 0 {
                let one_index = temp.trailing_zeros();
                match piece_index % 6 {
                    // 0 => self.gen_pawn_moves(piece_index, one_index),
                    // 1 => self.gen_knight_moves(one_index),
                    // 2 => self.gen_bishop_moves(one_index),
                    // 3 => self.gen_rook_moves(one_index),
                    // 4 => self.gen_queen_moves(one_index),
                    // 5 => self.gen_king_moves(one_index),
                    _ => {}
                }
                temp &= temp - 1;
            }
        }
    }

    fn gen_pawn_moves(&mut self, is_white: bool) {
        let mut moves: u64 = 0;
        let side_index: usize = if is_white { 0 } else { 1 };
        let empty_squares = !self.all_occupancy;
        let enemy_squares = if self.is_white_turn {
            self.black_occupancy
        } else {
            self.white_occupancy
        };
        let single_push = if self.is_white_turn {
            self.bitboards[side_index] << 8
        } else {
            self.bitboards[side_index] >> 8
        };
        let double_push = if self.is_white_turn {
            self.bitboards[side_index] << 16
        } else {
            self.bitboards[side_index] >> 16
        };
        let take_right = if self.is_white_turn {
            self.bitboards[side_index] << 9
        } else {
            self.bitboards[side_index] >> 7
        };
        let take_left = if self.is_white_turn {
            self.bitboards[side_index] << 7
        } else {
            self.bitboards[side_index] >> 9
        };
        moves |= empty_squares & single_push;
        moves |= empty_squares & double_push;
        moves |= enemy_squares & take_right & Self::NOT_H_FILE;
        moves |= enemy_squares & take_left & Self::NOT_A_FILE;
        self.possible_moves[side_index] = moves;
    }

    fn gen_pawn_moves2(&mut self, piece_index: usize, one_index: u32) {
        let location_mask: u64 = 1 << one_index;
        let is_white = piece_index < 6;
        let mut moves: u64 = 0;
        let mut is_double_possible =
            (one_index / 8 == 1 && is_white) || (one_index / 8 == 6 && !is_white);
        let mut is_one_possible = true;
        let mut is_taking_possible_r = false;
        let mut is_taking_possible_l = false;
        let to_square = if is_white {
            location_mask << 8
        } else {
            location_mask >> 8
        };
        let to_square2 = if is_white {
            location_mask << 16
        } else {
            location_mask >> 16
        };
        let taking_to_square_r = if is_white {
            location_mask << 7
        } else {
            location_mask >> 9
        };
        let taking_to_square_l = if is_white {
            location_mask << 9
        } else {
            location_mask >> 7
        };
        for p_index in 0..12 {
            let is_enemy = (p_index < 6) != is_white;
            if self.bitboards[p_index] & to_square != 0 {
                is_one_possible = false;
                is_double_possible = false;
            }
            if self.bitboards[p_index] & to_square2 != 0 {
                is_double_possible = false;
            }
            if (self.bitboards[p_index] & taking_to_square_r != 0) && is_enemy {
                is_taking_possible_r = true;
            }
            if (self.bitboards[p_index] & taking_to_square_l != 0) && is_enemy {
                is_taking_possible_l = true;
            }
        }
        if is_one_possible {
            moves |= to_square;
        }
        if is_double_possible {
            moves |= to_square2;
        }
        if is_taking_possible_r {
            moves |= taking_to_square_r;
        }
        if is_taking_possible_l {
            moves |= taking_to_square_l;
        }
        if moves == 0 {
            return;
        }
        // ADD ENPASSANT
        // CHECK If A or H FILES FOR TAKING MOVES AND ENPASSANT
        // self.possible_moves.push(ChessMove {
        //     from_square: location_mask,
        //     moves: moves,
        // });
    }

    fn gen_knight_moves(&mut self, is_white: bool) {
        let side_index = if is_white { 1 } else { 7 };
        self.possible_moves[side_index] = ((self.bitboards[side_index] << 6) & Self::NOT_AB_FILE)
            | ((self.bitboards[side_index] << 10) & Self::NOT_GH_FILE)
            | ((self.bitboards[side_index] << 15) & Self::NOT_A_FILE)
            | ((self.bitboards[side_index] << 17) & Self::NOT_H_FILE)
            | ((self.bitboards[side_index] >> 6) & Self::NOT_GH_FILE)
            | ((self.bitboards[side_index] >> 10) & Self::NOT_AB_FILE)
            | ((self.bitboards[side_index] >> 15) & Self::NOT_A_FILE)
            | ((self.bitboards[side_index] >> 17) & Self::NOT_H_FILE);
    }
    fn gen_bishop_moves(&mut self, is_white: bool) {}
    fn gen_rook_moves(&mut self, is_white: bool) {}
    fn gen_queen_moves(&mut self, is_white: bool) {}
    fn gen_king_moves(&mut self, is_white: bool) {}

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
        let mut moving_piece_index: Option<usize> = None;
        let mut moving_piece_x: f32 = 0.0;
        let mut moving_piece_y: f32 = 0.0;
        for piece_index in 0..self.images.len() {
            let mut temp = self.bitboards[piece_index];
            while temp != 0 {
                let one_index = temp.trailing_zeros();
                let i = one_index % 8;
                let j = one_index / 8;
                let x = self.width - (i + 1) as f32 * self.square_size;
                let y = self.height - (j + 1) as f32 * self.square_size;
                if one_index == self.from_square.trailing_zeros() {
                    moving_piece_index = Some(piece_index);
                    moving_piece_x = self.mouse_position[0] - self.square_size / 2.0;
                    moving_piece_y = self.mouse_position[1] - self.square_size / 2.0;
                    temp &= temp - 1;
                    continue;
                }
                let param = graphics::DrawParam::default()
                    .dest([x + offset, y + offset])
                    .scale([scale, scale]);
                canvas.draw(&self.images[piece_index], param);
                temp &= temp - 1;
            }
        }
        if let Some(piece_index) = moving_piece_index {
            let param = graphics::DrawParam::default()
                .dest([moving_piece_x, moving_piece_y])
                .scale([scale, scale]);
            canvas.draw(&self.images[piece_index], param);
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
        let mut chess = Chess {
            white_occupancy: wpawn_bitmask
                | wknight_bitmask
                | wbishop_bitmask
                | wrook_bitmask
                | wqueen_bitmask
                | wking_bitmask,
            black_occupancy: bpawn_bitmask
                | bknight_bitmask
                | bbishop_bitmask
                | brook_bitmask
                | bqueen_bitmask
                | bking_bitmask,
            all_occupancy: 0,
            possible_moves: [0; 12],
            is_white_turn: true,
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
        };
        chess.all_occupancy = chess.white_occupancy | chess.black_occupancy;
        chess.update_possible_moves();
        return chess;
    }
}

pub fn get_square_mask(_x: f32, _y: f32, square_size: f32) -> u64 {
    let i = (_x / square_size) as u64;
    let j = (_y / square_size) as u64;
    if i > 7 || j > 7 {
        return 0;
    }
    let j = 7 - j;
    let i = 7 - i;
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
            self.make_move();
            _ctx.gfx.window().set_cursor_icon(CursorIcon::Default);
        }
        self.from_square = 0;
        self.to_square = 0;
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
        if self.from_square != 0 {
            _ctx.gfx.window().set_cursor_icon(CursorIcon::Grab);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        self.draw_board(&mut canvas);
        self.draw_pieces(&mut canvas);
        canvas.finish(ctx)
    }
}
