use rltk::Rltk;
#[macro_use]
extern crate specs_derive;
mod components;
pub use components::*;
mod general_network;
#[cfg(target_arch = "wasm32")]
mod wasm_network;

#[cfg(not(target_arch = "wasm32"))]
mod desktop_network;

mod runstate;

use runstate::Runstate;

use std::sync::{Arc, Mutex};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

extern crate specs;
use specs::prelude::*;
pub mod gui;

mod rltk_main;

mod bundle;
mod game;
mod systems;
use crate::bundle::{GameBundle, NetworkBundle};

#[cfg(target_arch = "wasm32")]
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

use amethyst::{
    core::transform::TransformBundle,
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

mod state;

pub struct Data {
    pub characters: Vec<Point>,
    pub my_uid: String,
    pub map: Vec<(Point, Renderable)>,
    pub info_string: String,
}

pub struct Rect {
    pub width: i32,
    pub height: i32,
    pub x: i32,
    pub y: i32,
}

fn main() {
    //Shared data between the network and the game system
    let data = Data {
        characters: vec![],
        my_uid: "".to_string(),
        map: vec![],
        info_string: "".to_string(),
    };
    let protect_data: Arc<Mutex<Data>> = Arc::new(Mutex::new(data));
    let to_send: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    lauch_network(protect_data.clone(), to_send.clone());
    //rltk_init(protect_data.clone(), to_send.clone());
    amethyst_init(protect_data.clone(), to_send.clone()).expect("Fail in amethyst_init");
}

pub fn rltk_init(protect_data: Arc<Mutex<Data>>, to_send: Arc<Mutex<Vec<String>>>) {
    let context = Rltk::init_simple8x8(180 as u32, 90 as u32, "Ecosystem simulator", "resources");
    let gs = rltk_main::State {
        rectangle: Rect {
            height: 6,
            width: 2,
            x: 5,
            y: 5,
        },
        data: protect_data.clone(),
        to_send: to_send.clone(),
        ecs: World::new(),
        player_info: PlayerInfo {
            inventaire: Vec::new(),
            close_interations: Vec::new(),
            my_info: MyInfo {
                pos: Position { x: 0, y: 0 },
                hp: 0,
                max_hp: 0,
                player_log: vec![],
            },
            possible_builds: Vec::new(),
            equipement: Vec::new(),
            combat_stats: Default::default(),
        },
        runstate: Runstate::Register,
        pseudo: "".to_string(),
    };

    rltk::main_loop(context, gs);
}

pub struct Ball {
    pub velocity: [f32; 2],
    pub radius: f32,
}

impl Component for Ball {
    type Storage = DenseVecStorage<Self>;
}

pub fn amethyst_init(
    protect_data: Arc<Mutex<Data>>,
    to_send: Arc<Mutex<Vec<String>>>,
) -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let resources = app_root.join("resources");
    let display_config = resources.join("display_config.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config)
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderFlat2D::default()),
        )?
        .with_bundle(GameBundle)?
        .with_bundle(NetworkBundle {
            protect_data,
            to_send,
        })?;

    //ici on poura lancer le stat avec la map, penser a faire le buddle aussi, le bundle va initialiser les ressource

    let mut game = Application::new(resources, game::MyGame, game_data)?;
    game.run();

    Ok(())
}

#[cfg(target_arch = "wasm32")]
fn lauch_network(protect_data: Arc<Mutex<Data>>, to_send: Arc<Mutex<Vec<String>>>) {
    wasm_network::start_websocket(protect_data, to_send).expect("Unable to start websocket");
}

#[cfg(not(target_arch = "wasm32"))]
fn lauch_network(protect_data: Arc<Mutex<Data>>, to_send: Arc<Mutex<Vec<String>>>) {
    desktop_network::start_websocket(protect_data, to_send);
}
