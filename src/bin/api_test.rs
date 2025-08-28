use cxx::let_cxx_string;

use booster_sys::robot::{
    b1::ffi::b1_loco_client_new, common::ffi::RobotMode, ffi::init_channel_factory,
};

fn main() {
    let_cxx_string!(network_interface = "127.0.0.1");
    init_channel_factory(&network_interface);

    let mut client = b1_loco_client_new();

    client.pin_mut().ChangeMode(RobotMode::kDamping);
}
