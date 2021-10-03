extern crate piston_window;

use eliased_chess::{Color, Game, PieceType /* , GameState, Piece */};

use piston_window::*;

// Good inspiration for piston:
// https://gitlab.com/torbmol/pistonpath/blob/master/path.rs

const WHITE: [f32; 4] = [0.94, 0.93, 0.82, 1.0];
const BLACK: [f32; 4] = [0.47, 0.59, 0.34, 1.0];

fn init_textures(window: &mut PistonWindow) -> Vec<((PieceType, Color), G2dTexture)> {
    use Color::*;
    use PieceType::*;
    let assets = std::env::current_dir().unwrap().join("assets");
    let texture_settings = TextureSettings::new()
        .mipmap(Filter::Nearest)
        .filter(Filter::Nearest); // Aliasing filter
    let mut asset = |path: &str| -> G2dTexture {
        let asset_loc = assets.join(path);
        Texture::from_path(
            &mut window.create_texture_context(),
            asset_loc,
            Flip::None,
            &texture_settings,
        )
        .unwrap()
    };
    vec![
        ((King, White), asset("white_king.png")),
        ((Queen, White), asset("white_queen.png")),
        ((Rook, White), asset("white_rook.png")),
        ((Bishop, White), asset("white_bishop.png")),
        ((Knight, White), asset("white_knight.png")),
        ((Pawn, White), asset("white_pawn.png")),
        ((King, Black), asset("black_king.png")),
        ((Queen, Black), asset("black_queen.png")),
        ((Rook, Black), asset("black_rook.png")),
        ((Bishop, Black), asset("black_bishop.png")),
        ((Knight, Black), asset("black_knight.png")),
        ((Pawn, Black), asset("black_pawn.png")),
    ]
}

fn main() {
    let opengl = OpenGL::V3_2;
    // Default size: 640, 480
    let mut window: PistonWindow = WindowSettings::new("Elias Chess Gui", [480, 480])
        .exit_on_esc(true)
        .graphics_api(opengl)
        .build()
        .unwrap();
    window.set_lazy(true);

    let mut _frame = 0_u64;
    let mut mouse: Option<[f64; 2]> = None;
    // let mut last_dragging = false;
    // let mut dragging = false;

    let mut pressed = false;
    let mut picked_up: Option<[f64; 2]> = None;
    let mut dropped: Option<[f64; 2]> = None;

    let textures = init_textures(&mut window);
    let find_texture = |_piecetype: PieceType, _color: Color| -> Option<G2dTexture> {
        if let Some((_, tex)) = textures
            .iter()
            .find(|((_p, _c), _t)| _piecetype == *_p && _color == *_c)
        {
            Some(tex.clone())
        } else {
            None
        }
    };

    let mut game = Game::new();

    while let Some(event) = window.next() {
        let Size { width, height } = window.size();
        let square_size = f64::min(width, height) / 8.0;

        match event {
            Event::Loop(Loop::Render(_render_args)) => {
                _frame += 1;

                if let Some([x, y]) = mouse {
                    if pressed {
                        if let Some([_x, _y]) = picked_up {
                            println!("Dropped piece at ({}, {})", x, y);
                            dropped = Some([x, y]);
                            picked_up = None;
                        } else {
                            println!("Picked up piece at ({}, {})", x, y);
                            dropped = None;
                            picked_up = Some([x, y]);
                        }
                    }
                }

                // if !last_dragging && dragging {
                //     println!("Started dragging at ({}, {})", x, y);
                // }
                // if last_dragging && !dragging {
                //     println!("Stopped dragging at ({}, {})", x, y);
                // }

                window.draw_2d(&event, |context, graphics, _device| {
                    clear([1.0; 4], graphics);
                    for (r, row) in game.board.iter().enumerate() {
                        for (c, piece) in row.iter().enumerate() {
                            let color = if (r + c) % 2 == 0 { WHITE } else { BLACK }; // Even squares
                            let (x, y) = (square_size * r as f64, square_size * c as f64);
                            rectangle(
                                color,
                                [x, y, square_size, square_size],
                                context.transform,
                                graphics,
                            );

                            if let Some([_x, _y]) = picked_up {
                                if _x >= x
                                    && _x < x + square_size
                                    && _y >= y
                                    && _y < y + square_size
                                {
                                    rectangle(
                                        [0.74, 0.23, 0.49, 0.6], // Red overlay
                                        [x, y, square_size, square_size],
                                        context.transform,
                                        graphics,
                                    );
                                }
                            }
                            if let Some([_x, _y]) = dropped {
                                if _x >= x
                                    && _x < x + square_size
                                    && _y >= y
                                    && _y < y + square_size
                                {
                                    rectangle(
                                        [0.0, 1.0, 0.0, 0.7], // Green overlay
                                        [x, y, square_size, square_size],
                                        context.transform,
                                        graphics,
                                    );
                                }
                            }

                            if let Some(_piece) = piece {
                                if let Some(texture) = find_texture(_piece.piecetype, _piece.color)
                                {
                                    let (img_w, img_h) = texture.get_size();
                                    let scale = f64::min(
                                        square_size / img_w as f64,
                                        square_size / img_h as f64,
                                    );
                                    image(
                                        &texture,
                                        context.trans(x, y).scale(scale, scale).transform,
                                        graphics,
                                    );
                                }
                            }
                        }
                    }
                });
                // last_dragging = dragging;
                pressed = false;
            }
            Event::Loop(Loop::Update(UpdateArgs { dt: _dt })) => {
                // Update animation for rotating piece
                // game.update(dt);
            }
            Event::Input(Input::Button(ButtonArgs { state, button, .. }), ..) => {
                match (button, state) {
                    (Button::Keyboard(_key), ButtonState::Press) => {
                        println!("Key pressed: {:?}", _key)
                    }
                    (Button::Mouse(_button), ButtonState::Press) => {
                        // dragging = true;
                        // println!("Mouse pressed: {:?}", _button);
                        pressed = true;
                    }
                    (Button::Mouse(_button), ButtonState::Release) => {
                        // dragging = false;
                        // println!("Mouse released: {:?}", _button);
                    }
                    _ => {}
                }
            }
            Event::Input(Input::Move(Motion::MouseCursor(pos)), ..) => {
                mouse = Some(pos);
            }
            Event::Input(Input::Cursor(false), ..) => {
                // cursor left window, only triggered if a button is pressed.
                // println!("Mouse left screen");
                mouse = None;
            }
            Event::Input(Input::Resize(_res_args), ..) => {
                // let (x, y): (u32, u32) = (_res_args.draw_size[0], _res_args.draw_size[1]);
                // println!("New size: {}, {}", x, y);
            }
            _ => {}
        }
    }
}
