extern crate e_drone_rpi;

use e_drone::system::{*};
use e_drone::protocol::{*};
use e_drone_rpi::{*};


fn main() {
    //let mut drone: Drone = Drone::new();  // UART
    let mut drone: Drone = Drone::new_path("/dev/ttyACM0"); // USB

    if drone.is_connected() == false {
        return;
    }
    
    drone.buzzer_hz(DeviceType::Controller, 1200, 100);
    drone.sleep(10);
    drone.buzzer_hz_reserve(DeviceType::Controller, 1000, 100);
    drone.sleep(500);

    drone.buzzer_scale(DeviceType::Controller, buzzer::Scale::C4, 100);
    drone.sleep(10);
    drone.buzzer_scale_reserve(DeviceType::Controller, buzzer::Scale::D4, 100);
    drone.sleep(10);
    drone.buzzer_scale_reserve(DeviceType::Controller, buzzer::Scale::E4, 100);
    drone.sleep(10);
    drone.buzzer_scale_reserve(DeviceType::Controller, buzzer::Scale::F4, 100);
    drone.sleep(10);
    drone.buzzer_scale_reserve(DeviceType::Controller, buzzer::Scale::G4, 100);
    drone.sleep(10);
    drone.buzzer_scale_reserve(DeviceType::Controller, buzzer::Scale::A4, 100);
    drone.sleep(10);
    drone.buzzer_scale_reserve(DeviceType::Controller, buzzer::Scale::B4, 100);
    drone.sleep(10);
    drone.buzzer_scale_reserve(DeviceType::Controller, buzzer::Scale::C5, 100);
    drone.sleep(2000);
    
    drone.buzzer_hz(DeviceType::Controller, 1000, 100);
    drone.sleep(10);
    drone.buzzer_hz_reserve(DeviceType::Controller, 1200, 100);
    drone.sleep(500);

    drone.buzzer_scale(DeviceType::Controller, buzzer::Scale::C5, 100);
    drone.sleep(10);
    drone.buzzer_scale_reserve(DeviceType::Controller, buzzer::Scale::B4, 100);
    drone.sleep(10);
    drone.buzzer_scale_reserve(DeviceType::Controller, buzzer::Scale::A4, 100);
    drone.sleep(10);
    drone.buzzer_scale_reserve(DeviceType::Controller, buzzer::Scale::G4, 100);
    drone.sleep(10);
    drone.buzzer_scale_reserve(DeviceType::Controller, buzzer::Scale::F4, 100);
    drone.sleep(10);
    drone.buzzer_scale_reserve(DeviceType::Controller, buzzer::Scale::E4, 100);
    drone.sleep(10);
    drone.buzzer_scale_reserve(DeviceType::Controller, buzzer::Scale::D4, 100);
    drone.sleep(10);
    drone.buzzer_scale_reserve(DeviceType::Controller, buzzer::Scale::C4, 100);
    drone.sleep(2000);
}

