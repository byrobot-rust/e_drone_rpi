# e_drone_rpi
Rust library for BYROBOT drones.

- Tested
  * Raspberry PI Compute Module 4

- Support
  * [E-Drone](http://dev.byrobot.co.kr/documents/kr/products/e_drone/)
  * [Coding Drone](http://dev.byrobot.co.kr/documents/kr/products/coding_drone/)
  * [Battle Drone](http://dev.byrobot.co.kr/documents/kr/products/battle_drone/)


<br>
<br>


## Example

### Cargo.toml

```toml
[dependencies]
e_drone_rpi = "21.*"
e_drone = "21.*"
```


<br>
<br>


### main.rs
```rust
extern crate e_drone_rpi;

use e_drone::system::{*};
use e_drone::protocol::{*};
use e_drone_rpi::{*};


fn main() {
    let mut drone: Drone = Drone::new();

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


<br>
<br>



## Examples in library

<br>

### Source code

https://github.com/byrobot-rust/e_drone_rpi/tree/master/examples

<br>
<br>

### Clone Library
```
git clone https://github.com/byrobot-rust/e_drone_rpi/
```


<br>
<br>

### Run
```
cargo run --example button
```
```
cargo run --example buzzer
```
```
cargo run --example display
```
```
cargo run --example joystick
```
```
cargo run --example light
```
```
cargo run --example request
```
```
cargo run --example vibrator
```


