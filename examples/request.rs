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

    drone.request(DeviceType::Controller, DataType::Information);

    loop {
        handler(&drone.check());

        if drone.get_time_passed_from_last_transfer() > 1200 {
            break;
        }
    }
}


fn handler(data: &Data) {
    match data {
        Data::Information(information) => {
            println!("{:?}", information);
        },
        _ => {},
    }
}

