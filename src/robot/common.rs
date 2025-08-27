// use cxx::{type_id, ExternType};

// unsafe impl ExternType for ffi::Frame {
//     type Id = type_id!("booster::robot::Frame");
//     type Kind = cxx::kind::Trivial;
// }

// unsafe impl ExternType for ffi::RobotMode {
//     type Id = type_id!("booster::robot::RobotMode");
//     type Kind = cxx::kind::Trivial;
// }

#[cxx::bridge(namespace = "booster::robot")]
pub mod ffi {
    #[repr(i32)]
    pub enum RobotMode {
        /// For error handling
        kUnknown = -1,
        /// All motor enter damping mode, robot will fall down if it is not supported
        kDamping = 0,
        /// Prepare mode, the robot keeps standing on both feet and can switch to walking mode
        kPrepare = 1,
        /// Walking mode, in walking mode, the robot can move, rotate, kick the ball, etc.
        kWalking = 2,
        /// Custom mode, in custom mode, the robot can do some custom actions
        kCustom = 3,
    }

    pub enum Frame {
        /// For error handling
        kUnknown = -1,
        kBody = 0,     
        kHead = 1,
        kLeftHand = 2,
        kRightHand = 3,
        kLeftFoot = 4,
        kRightFoot = 5,
    }

    unsafe extern "C++" {
        include!("booster/robot/common/robot_shared.hpp");

        type RobotMode;
        type Frame;
    }
}
