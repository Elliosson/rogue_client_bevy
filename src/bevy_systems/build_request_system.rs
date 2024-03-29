use crate::{BuildRequests, Data, PlayerInfo};
use bevy::prelude::*;
use std::sync::{Arc, Mutex};

//send data to the server when the button is pressed
pub fn build_request_system(
    _commands: Commands,
    to_send: ResMut<Arc<Mutex<Vec<String>>>>,
    net_data: ResMut<Arc<Mutex<Data>>>,
    player_info: Res<PlayerInfo>,
    mut requests: ResMut<BuildRequests>,
) {
    let mut to_send_guard = to_send.lock().unwrap();
    let data_guard = net_data.lock().unwrap();

    for request in requests.items.iter() {
        to_send_guard.push(format!(
            "{} {} {} {} {}",
            data_guard.my_uid,
            "build",
            player_info.my_info.pos.x,
            player_info.my_info.pos.y,
            request.name
        ));
    }

    requests.items.clear();
}
