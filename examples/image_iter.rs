
#![feature(globs)]

extern crate graphics;
extern crate piston;

use graphics::*;
use piston::{
    AssetStore,
    Load,
    GameIterator,
    GameWindow,
    GameWindowSDL2,
    GameWindowSettings,
    Render,
};

#[start]
fn start(argc: int, argv: **u8) -> int {
    // Run gui on the main thread.
    native::start(argc, argv, main)
}

fn main() {
    let mut window: GameWindowSDL2 = GameWindow::new(
        GameWindowSettings {
            title: "Image".to_string(),
            size: [300, 300],
            fullscreen: false,
            exit_on_esc: true,
            background_color: [1.0, 1.0, 1.0, 1.0],
        }
    );

    let mut asset_store = AssetStore::from_folder("assets");

    let mut image: Option<Image> = None;
    let mut game_iter = GameIterator::new(&mut window, &mut asset_store);
    loop {
        match game_iter.next() {
            None => { break },
            Some(e) => match e {
                Load(e) => {
                    let asset_store = e.asset_store;
                    image = Some(asset_store.load_image("rust-logo.png").unwrap());
                },
                Render(e) => {
                    let c = e.context;
                    let mut gl = e.gl;
                    c.image(image.unwrap()).draw(&mut gl);
                },
                _ => {},       
            },
        }
    }
}


