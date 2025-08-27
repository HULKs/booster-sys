use booster_sys::robot::common::ffi::RobotMode;
use cxx::let_cxx_string;

fn main() {
    let network_interface = std::env::args().nth(2).expect("Expected a network interface as CLI argument");
    let_cxx_string!(network_interface_cxx = network_interface);
    
    booster_sys::robot::ffi::init_channel_factory(&network_interface_cxx);

    let mut client = booster_sys::robot::b1::ffi::b1_loco_client_new();
    client.pin_mut().Init();

    let mut x = 0.0;
    let mut y = 0.0;
    let mut z = 0.0;
    let mut pitch = 0.0;
    let mut yaw = 0.0;
    let mut need_print = false;
    let mut result = 0;
    let mut input_buffer = String::new();

    loop {
        std::io::stdin().read_line(&mut input_buffer).unwrap();
        if !input_buffer.is_empty() {
            match input_buffer.as_str() {
                "mp" => {
                    result = client.pin_mut().ChangeMode(RobotMode::kPrepare)
                }
                "md" => {
                    result = client.pin_mut().ChangeMode(RobotMode::kDamping)
                }
                "mw" => {
                    result = client.pin_mut().ChangeMode(RobotMode::kWalking)
                }
                "mc" => {
                    result = client.pin_mut().ChangeMode(RobotMode::kCustom)
                }
                "w" => {
                    x = 0.2;
                    y = 0.0;
                    z = 0.0;
                    need_print = true;
                    result = client.pin_mut().Move(x, y, z);
                }
                "l" => {
                    x = 0.0;
                    y = 0.0;
                    z = 0.0;
                    need_print = true;
                    result = client.pin_mut().Move(x, y, z);
                }
                "a" => {
                    x = 0.0;
                    y = 0.2;
                    z = 0.0;
                    need_print = true;
                    result = client.pin_mut().Move(x, y, z);
                }
                "s" => {
                    x = -0.2;
                    y = 0.0;
                    z = 0.0;
                    need_print = true;
                    result = client.pin_mut().Move(x, y, z);
                }
                "d" => {
                    x = 0.0;
                    y = -0.2;
                    z = 0.0;
                    need_print = true;
                    result = client.pin_mut().Move(x, y, z);
                }
                "q" => {
                    x = 0.0;
                    y = 0.0;
                    z = 1.0;
                    need_print = true;
                    result = client.pin_mut().Move(x, y, z);
                }
                "e" => {
                    x = 0.0;
                    y = 0.0;
                    z = -1.0;
                    need_print = true;
                    result = client.pin_mut().Move(x, y, z);
                }
                "hd" => {
                    yaw = 0.0;
                    pitch = 1.0;

                    need_print = true;
                    result = client.pin_mut().RotateHead(pitch, yaw);
                }
                "hu" => {
                    yaw = 0.0;
                    pitch = -0.3;

                    need_print = true;
                    result = client.pin_mut().RotateHead(pitch, yaw);
                }
                "hr" => {
                    yaw = -0.785;
                    pitch = 0.0;

                    need_print = true;
                    result = client.pin_mut().RotateHead(pitch, yaw);
                }
                "hl" => {
                    yaw = 0.785;
                    pitch = 0.0;

                    need_print = true;
                    result = client.pin_mut().RotateHead(pitch, yaw);
                }
                "ho" => {
                    yaw = 0.0;
                    pitch = 0.0;

                    need_print = true;
                    result = client.pin_mut().RotateHead(pitch, yaw);
                }
                // } else if (input == "wh") {
                //     res = client.WaveHand(booster::robot::b1::HandAction::kHandOpen);
                // } else if (input == "ch") {
                //     res = client.WaveHand(booster::robot::b1::HandAction::kHandClose);
                // } else if (input == "ld") {
                //     res = client.LieDown();
                // } else if (input == "gu") {
                //     res = client.GetUp();
                // } else if (input == "mhel") {
                //     booster::robot::Posture tar_posture;
                //     tar_posture.position_ = booster::robot::Position(0.35, 0.25, 0.1);
                //     tar_posture.orientation_ = booster::robot::Orientation(-1.57, -1.57, 0.);

                //     res = client.MoveHandEndEffectorV2(
                //         tar_posture, 2000, booster::robot::b1::HandIndex::kLeftHand);

                //     tar_posture.position_ = booster::robot::Position(0.35, -0.2, 0.1);
                //     tar_posture.orientation_ = booster::robot::Orientation(1.57, -1.57, 0.);
                //     res = client.MoveHandEndEffectorV2(
                //         tar_posture, 2000, booster::robot::b1::HandIndex::kRightHand);
                // } else if (input == "gopenl") {
                //     booster::robot::b1::GripperMotionParameter motion_param;
                //     motion_param.position_ = 500;
                //     motion_param.force_ = 100;
                //     motion_param.speed_ = 100;

                //     res = client.ControlGripper(
                //         motion_param, booster::robot::b1::GripperControlMode::kPosition,
                //         booster::robot::b1::HandIndex::kLeftHand);
                // } else if (input == "gft") {
                //     booster::robot::Frame src = booster::robot::Frame::kBody;
                //     booster::robot::Frame dst = booster::robot::Frame::kRightHand;
                //     booster::robot::Transform transform;

                //     res = client.GetFrameTransform(src, dst, transform);
                //     if (res == 0) {
                //         std::cout << "pos:" << transform.position_.x_ << " " << transform.position_.y_
                //                   << " " << transform.position_.z_ << std::endl;
                //         std::cout << "ori:" << transform.orientation_.x_ << " " << transform.orientation_.y_
                //                   << " " << transform.orientation_.z_ << " "
                //                   << transform.orientation_.w_ << std::endl;
                //     }
                // } else if (input == "hcm-start") {
                //     res = client.SwitchHandEndEffectorControlMode(true);
                // } else if (input == "hcm-stop") {
                //     res = client.SwitchHandEndEffectorControlMode(false);
                // } else if (input == "hand-down") {
                //     booster::robot::Posture tar_posture;
                //     tar_posture.position_ = booster::robot::Position(0.28, -0.25, 0.05);
                //     tar_posture.orientation_ = booster::robot::Orientation(0., 0., 0.);

                //     res = client.MoveHandEndEffector(
                //         tar_posture, 1000, booster::robot::b1::HandIndex::kRightHand);
                //     std::this_thread::sleep_for(std::chrono::milliseconds(300));

                //     hand_action_count++;
                //     int random = rand() % 3;
                //     if (random == 0) {
                //         HandRock(client);
                //     } else if (random == 1) {
                //         HandScissor(client);
                //     } else {
                //         HandPaper(client);
                //     }
                // } else if (input == "hand-up") {
                //     booster::robot::Posture tar_posture;
                //     tar_posture.position_ = booster::robot::Position(0.25, -0.3, 0.25);
                //     tar_posture.orientation_ = booster::robot::Orientation(0., -1.0, 0.);

                //     res = client.MoveHandEndEffector(
                //         tar_posture, 1000, booster::robot::b1::HandIndex::kRightHand);

                //     std::this_thread::sleep_for(std::chrono::milliseconds(300));

                //     HandPaper(client);
                // } else if (input == "grasp") {
                //     HandGrasp(client);
                // } else if (input == "ok") {
                //     HandOk(client);
                // } else if (input == "paper") {
                //     HandPaper(client);
                // } else if (input == "scissor") {
                //     HandScissor(client);
                // } else if (input == "rock") {
                //     HandRock(client);
                // }

    
                _ => {}
            }
            if need_print {
                println!("Param: {x} {y} {z}");
                println!("Head param: {pitch} {yaw}");
            }
            if result != 0 {
                println!("Request failed: error = {result}");
            }
        }
    }
}
