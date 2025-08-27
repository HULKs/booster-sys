// use cxx::type_id;

pub mod b1;
pub mod common;

// #[cxx::bridge(namespace = "booster::robot")]
// pub mod ffi {
//     unsafe extern "C++" {
//         include!("booster/robot/channel/channel_factory.hpp");
        
//         type ChannelFactory;

//         fn Init(self: &mut ChannelFactory, domain_id: i32, network_interface: &CxxString);

//         #[Self = ChannelFactory]
//         fn Instance() -> *mut ChannelFactory; 
//     }
// }

// unsafe impl cxx::ExternType for ffi::ChannelFactory {
//     type Id = type_id!("booster::robot::ChannelFactory");
//     type Kind = cxx::kind::Trivial;
// }


#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("wrapper.hpp");

        fn init_channel_factory(network_interface: &CxxString);
    }
}
