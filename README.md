# e_drone_rpi
Raspberry PI / Rust library for BYROBOT drones.



## Cargo.toml
> e_drone_rpi = "21.*"



## example code
```rust
extern crate e_drone_rpi;

use e_drone::base::system::{*};
use e_drone::protocol::{*};
use e_drone_rpi::{*};


fn main() {
    let mut drone: Drone = Drone::new("COM75");

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
```
