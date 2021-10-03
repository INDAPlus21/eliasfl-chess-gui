extern crate piston_window;

use std::collections::HashMap;
use std::path::PathBuf;

use piston_window::*;

// Good inspiration for piston:
// https://gitlab.com/torbmol/pistonpath/blob/master/path.rs

const WHITE: [f32; 4] = [0.94, 0.93, 0.82, 1.0];
const BLACK: [f32; 4] = [0.47, 0.59, 0.34, 1.0];

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Color {
    White,
    Black,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Asset {
    Empty,
    King(Color),
    Queen(Color),
    Rook(Color),
    Bishop(Color),
    Knight(Color),
    Pawn(Color),
}
impl Asset {
    pub fn all() -> [Asset; 12] {
        use Asset::*;
        use Color::*;
        [
            // White
            King(White),
            Queen(White),
            Rook(White),
            Bishop(White),
            Knight(White),
            Pawn(White),
            // Black
            King(Black),
            Queen(Black),
            Rook(Black),
            Bishop(Black),
            Knight(Black),
            Pawn(Black),
        ]
    }
    pub fn location(&self) -> PathBuf {
        let assets = std::env::current_dir().unwrap().join("assets");
        use Asset::*;
        use Color::*;
        match self {
            // White
            King(White) => assets.join("white_king.png"),
            Queen(White) => assets.join("white_queen.png"),
            Rook(White) => assets.join("white_rook.png"),
            Bishop(White) => assets.join("white_bishop.png"),
            Knight(White) => assets.join("white_knight.png"),
            Pawn(White) => assets.join("white_pawn.png"),
            // Black
            King(Black) => assets.join("black_king.png"),
            Queen(Black) => assets.join("black_queen.png"),
            Rook(Black) => assets.join("black_rook.png"),
            Bishop(Black) => assets.join("black_bishop.png"),
            Knight(Black) => assets.join("black_knight.png"),
            Pawn(Black) => assets.join("black_pawn.png"),
            Empty => assets.join("not_found.png"),
        }
    }
    pub fn texture(&self, window: &mut PistonWindow) -> G2dTexture {
        let texture_settings = TextureSettings::new()
            .mipmap(Filter::Nearest)
            .filter(Filter::Nearest); // Aliasing filter
        Texture::from_path(
            &mut window.create_texture_context(),
            self.location(),
            Flip::None,
            &texture_settings,
        )
        .unwrap()
    }
}

fn main() {
    use Asset::*;
    use Color::*;
    let board = [
        [
            Rook(Black),
            Knight(Black),
            Bishop(Black),
            Queen(Black),
            King(Black),
            Bishop(Black),
            Knight(Black),
            Rook(Black),
        ],
        [Pawn(Black); 8],
        [Empty; 8],
        [Empty; 8],
        [Empty; 8],
        [Empty; 8],
        [Pawn(White); 8],
        [
            Rook(White),
            Knight(White),
            Bishop(White),
            Queen(White),
            King(White),
            Bishop(White),
            Knight(White),
            Rook(White),
        ],
    ];

    let opengl = OpenGL::V3_2;
    // Default size: 640, 480
    let mut window: PistonWindow = WindowSettings::new("Hello Piston!", [480, 480])
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

    let mut textures = HashMap::new();
    for asset in Asset::all() {
        textures.insert(asset, asset.texture(&mut window));
    }

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
                    for row in 0..8 {
                        for col in 0..8 {
                            let color = if (row + col) % 2 == 0 { WHITE } else { BLACK };
                            let (x, y) = (square_size * row as f64, square_size * col as f64);
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
                                        [0.23, 0.74, 0.49, 0.6], // Green overlay
                                        [x, y, square_size, square_size],
                                        context.transform,
                                        graphics,
                                    );
                                }
                            }

                            if let Some(texture) = textures.get(&board[col][row]) {
                                let (img_w, img_h) = texture.get_size();
                                let scale = f64::min(
                                    square_size / img_w as f64,
                                    square_size / img_h as f64,
                                );
                                image(
                                    texture,
                                    context.trans(x, y).scale(scale, scale).transform,
                                    graphics,
                                );
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
