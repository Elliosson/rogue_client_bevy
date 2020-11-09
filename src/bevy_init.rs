use super::Data;
use bevy::prelude::*;
use std::sync::{Arc, Mutex};

pub fn bevy_init(protect_data: Arc<Mutex<Data>>, to_send: Arc<Mutex<Vec<String>>>) {
    App::build()
        .add_plugins(DefaultPlugins)
        .init_resource::<ButtonMaterials>()
        .add_resource(protect_data)
        .add_startup_system(setup.system())
        .add_system(button_system.system())
        .add_system(player_movement_system.system())
        .run();
}

struct ButtonMaterials {
    normal: Handle<ColorMaterial>,
    hovered: Handle<ColorMaterial>,
    pressed: Handle<ColorMaterial>,
}

impl FromResources for ButtonMaterials {
    fn from_resources(resources: &Resources) -> Self {
        let mut materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();
        ButtonMaterials {
            normal: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
            hovered: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
            pressed: materials.add(Color::rgb(0.35, 0.75, 0.35).into()),
        }
    }
}

pub struct Player {}

fn button_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    button_materials: Res<ButtonMaterials>,
    mut interaction_query: Query<(
        &Button,
        Mutated<Interaction>,
        &mut Handle<ColorMaterial>,
        &Children,
    )>,
    mut text_query: Query<&mut Text>,
) {
    for (_button, interaction, mut material, children) in interaction_query.iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                text.value = "Press".to_string();
                *material = button_materials.pressed.clone();
            }
            Interaction::Hovered => {
                text.value = "Hover".to_string();
                *material = button_materials.hovered.clone();
            }
            Interaction::None => {
                text.value = "Button".to_string();
                *material = button_materials.normal.clone();
            }
        }
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    button_materials: Res<ButtonMaterials>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        // ui camera
        .spawn(Camera2dComponents::default())
        .spawn(UiCameraComponents::default())
        .spawn(SpriteComponents {
            material: materials.add(Color::rgb(0.5, 0.5, 1.0).into()),
            transform: Transform::from_translation(Vec3::new(0.0, 5.0, 0.0)),
            sprite: Sprite::new(Vec2::new(120.0, 30.0)),
            ..Default::default()
        })
        .with(Player {});
}

fn player_movement_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Transform)>,
) {
    for (player, mut transform) in query.iter_mut() {
        let mut direction_x = 0.0;
        let mut direction_y = 0.0;
        if keyboard_input.pressed(KeyCode::Left) {
            direction_x -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::Up) {
            direction_y += 1.0;
        }

        if keyboard_input.pressed(KeyCode::Down) {
            direction_y -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::Right) {
            direction_x += 1.0;
        }
        println!("dir x {} y {}", direction_x, direction_y);

        let translation = &mut transform.translation;
        // move the paddle horizontally
        *translation.x_mut() += time.delta_seconds * direction_x * 500.;
        *translation.y_mut() += time.delta_seconds * direction_y * 500.;
        // bound the paddle within the walls
        *translation.x_mut() = translation.x().min(380.0).max(-380.0);
    }
}
