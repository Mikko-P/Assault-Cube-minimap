mod memory;

use std::io::{self, Write};
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

// offsets
const PLAYER_POS_X_OFFSET: usize = 0x34;
const PLAYER_POS_Y_OFFSET: usize = 0x38;
const PLAYER_POS_Z_OFFSET: usize = 0x3C;
const PLAYER_CAMERA_PAN: usize = 0x40;
const PLAYER_CLASS_ADDRESS: usize = 0x509B74;

#[derive(Default, PartialEq)]
struct Positions {
    x: f32,
    y: f32,
    z: f32,
}

fn send_position_data(mut stream: TcpStream) -> io::Result<()> {
    let memory = memory::Memory::new(String::from("ac_client.exe"));
    let player_class: usize = memory.read_mem::<usize>(PLAYER_CLASS_ADDRESS);
    let mut previous_position = Positions::default();
    let mut previous_pan = Default::default();
    loop {
        let player_positions: Positions = memory.read_mem::<Positions>(player_class + PLAYER_POS_X_OFFSET);
        let player_camera_pan: f32 = memory.read_mem::<f32>(player_class + PLAYER_CAMERA_PAN);
        

        // If playerpos has changed, send it to the server
        if player_positions != previous_position || player_camera_pan != previous_pan {
            let data = format!(r#"{{ "x": {}, "y": {}, "z": {}, "cameraPan": {} }}"#,
                               player_positions.x, player_positions.y, player_positions.z, player_camera_pan);

            stream.write_all(data.as_bytes())?;
            println!("Sent to server: {}", data);

            previous_position = player_positions;
            previous_pan = player_camera_pan;
        }

        thread::sleep(Duration::from_millis(10));
    }
}

fn main() -> io::Result<()> {
    let stream = TcpStream::connect("localhost:3000")?;
    println!("Connected to server!");

    thread::spawn(move || {
        if let Err(err) = send_position_data(stream) {
            eprintln!("Error sending position data: {}", err);
        }
    });

    loop {
        thread::sleep(Duration::from_secs(1));
    }
}
