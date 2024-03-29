extern crate e_drone_rpi;

use e_drone::system::{*};
use e_drone::protocol::{*};
use e_drone::communication::{*};
use e_drone_rpi::{*};


fn main() {
    //let mut drone: Drone = Drone::new();  // UART
    let mut drone: Drone = Drone::new_path("/dev/ttyACM0"); // USB

    if drone.is_connected() == false {
        return;
    }

    drone.request(DeviceType::Controller, DataType::Information);

    println!("#1");
    drone.vibrator(200, 200, 2000);
    drone.sleep(2500);

    println!("#2");
    drone.vibrator(100, 200, 2000);
    drone.sleep(2500);

    println!("#3");
    drone.vibrator(200, 100, 2000);
    drone.sleep(2500);

    println!("#4");
    drone.vibrator(100, 100, 2000);
    drone.sleep(2500);

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

