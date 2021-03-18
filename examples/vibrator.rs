extern crate e_drone_rpi;

use e_drone::system::{*};
use e_drone::communication::{*};
use e_drone::protocol::{*};
use e_drone_rpi::{*};


fn main() {
    let mut drone: Drone = Drone::new();

    if drone.is_connected() == false {
        return;
    }

    drone.send(&transfer::vibrator(500, 500, 2000));

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

