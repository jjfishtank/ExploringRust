extern crate glutin_window;
use glutin_window::GlutinWindow;
extern crate piston;
use piston::event_loop::{EventLoop, EventSettings, Events};
use piston::input::{Button, ButtonState, Key};
use piston::{ButtonEvent, RenderEvent, WindowSettings};
extern crate graphics;
extern crate opengl_graphics;
use graphics::character::CharacterCache;
use opengl_graphics::{Filter, GlGraphics, GlyphCache, OpenGL, TextureSettings};

type Color = [f32; 4];
const RED: Color = [1.0, 0.0, 0.0, 1.0];
const GREEN: Color = [0.0, 1.0, 0.0, 1.0];
const BLUE: Color = [0.0, 0.0, 1.0, 1.0];
const WHITE: Color = [1.0; 4];
const BLACK: Color = [0.0, 0.0, 0.0, 1.0];

const WINDOW_SIZE: i32 = 512;
const TILE_SIZE: f64 = 32.0;
const WORLD_SIZE: i32 = WINDOW_SIZE / TILE_SIZE as i32;

#[derive(Clone)] // Implements clone trait to Tile
                 // The Tile is the basic unit of the game map
struct Tile {
    color: Color,
}

impl Tile {
    pub fn empty() -> Self {
        Tile { color: WHITE }
    }

    pub fn wall() -> Self {
        Tile { color: BLACK }
    }
}

type Map = Vec<Vec<Tile>>; // 2D Game Map

// Returns a map filled with empty tiles
fn make_map() -> Map {
    let mut map = vec![vec![Tile::empty(); WORLD_SIZE as usize]; WORLD_SIZE as usize];
    map[WORLD_SIZE as usize / 2][WORLD_SIZE as usize / 2] = Tile::wall();
    map
}

#[derive(Clone)]
struct Object {
    x_pos: i32,
    y_pos: i32,
    character: char,
    color: Color,
}

impl Object {
    pub fn new(x_pos: i32, y_pos: i32, character: char, color: Color) -> Self {
        Object {
            x_pos,
            y_pos,
            character,
            color,
        }
    }
}

fn main() {
    // Open Window
    let settings = WindowSettings::new("Game", [992, 992]).exit_on_esc(true);
    let mut window: GlutinWindow = settings.build().expect("Could not create window");

    // OpenGL
    let mut gl = GlGraphics::new(OpenGL::V4_5);

    // Load World
    let map: Map = make_map();
    let mut player: Object = Object::new(0, 0, '@', RED);

    // Load Glyph Texture from font file
    let texture_settings = TextureSettings::new().filter(Filter::Nearest);
    let ref mut glyphs = GlyphCache::new("assets/FiraSans-Regular.ttf", (), texture_settings)
        .expect("Could not load font");

    // Start event loop
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        // Render Game
        if let Some(r) = e.render_args() {
            gl.draw(r.viewport(), |c, g| {
                // Map Render
                for i in 0..WORLD_SIZE as usize {
                    for j in 0..WORLD_SIZE as usize {
                        let pos: [f64; 4] = [
                            TILE_SIZE * i as f64,
                            TILE_SIZE * j as f64,
                            TILE_SIZE * (i + 1) as f64,
                            TILE_SIZE * (j + 1) as f64,
                        ];
                        graphics::Rectangle::new(map[i][j].color).draw(
                            pos,
                            &c.draw_state,
                            c.transform,
                            g,
                        );
                    }
                }

                // Player Render
                use graphics::Transformed;
                let character = glyphs.character(TILE_SIZE as u32, player.character).unwrap();
                graphics::Image::new_color(player.color).draw(
                    character.texture,
                    &c.draw_state,
                    c.transform.trans(player.x_pos as f64, player.y_pos as f64),
                    g,
                );
            });
        }
        
        // Player Movement
        if let Some(k) = e.button_args() {
            if k.state == ButtonState::Press{
                match k.button {
                    Button::Keyboard(Key::Up) => player.y_pos -= TILE_SIZE as i32,
                    Button::Keyboard(Key::Down) => player.y_pos += TILE_SIZE as i32,
                    Button::Keyboard(Key::Left) => player.x_pos -= TILE_SIZE as i32,
                    Button::Keyboard(Key::Right) => player.x_pos += TILE_SIZE as i32,
                    _ => (),
                }
            }
        }
    }
}
