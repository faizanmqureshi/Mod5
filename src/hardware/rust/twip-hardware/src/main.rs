use gpio::{GpioIn, GpioOut};
use std::{
    io, 
    thread, 
    time, 
    fs, 
    path::Path, 
    sync::mpsc};
use hyper::Client;

#[tokio::main]
async fn send_speed(payload: f32) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // This is where we will setup our HTTP client requests.
//    thread::sleep(time::Duration::from_secs(5));  // testing transmit threading. now main doesn't block
    
    // Still inside `async fn main`...
    let speed: u8 = payload as u8;
    let client = Client::new();
    // Parse an `http::Uri`...
    let uri = format!("http://localhost:8000/hardware/speed/{}", speed).parse()?;
    // Await the response...
    let resp = client.get(uri).await?;
    println!("Response: {}", resp.status());
        Ok(())
}

fn main() {
    print!("helloHall | ");
    let datapin = 20;
    
    let mut gpio0 = gpio::sysfs::SysFsGpioInput::open(datapin).unwrap(); // possible gpio-breakpoint
    let mut power = gpio::sysfs::SysFsGpioOutput::open(16).unwrap();
    power.set_value(true);
    print!("opened GPIO | ");
    let (send, rcv) = mpsc::channel(); // open channel for sensor-polling thread
    print!("opened thread channel | ");
    // set constants 
//    let circumference = 28.0 * 0.0254 * 3.1415;           // wheel perimeter (diameter -> meters * pi)
    let circumference = 0.065 * 3.1415;           
    let passes_threshold = 20; // sensor flutter correction 

    /*
    This thread needs to make sure it only notifies once for the sensor passing. 
    This is why we need to get at least 10* the threshold of "low" before we can go high again
    */
    thread::spawn(move || {
        let mut passes = 0;
        let mut cooldown = 0;
        
        loop {
            let sensor_state = gpio0.read_value().unwrap();  // possible breakpoint

            if sensor_state == gpio::GpioValue::High && cooldown == 0 {
                passes += 1;
            } else if sensor_state == gpio::GpioValue::Low && cooldown > 0 {
                //cooling down.
                cooldown += -1;
            } else {    // sensor value is "unexpected"; reset counter.
                        // thread will also get stuck here if the bike stops with magnet in front of sensor.
                passes = 0;
                thread::yield_now();
            }

            if passes >= passes_threshold {
                send.send(true);
                cooldown = passes * 10;
                passes = 0;
            }
        }
    });
    print!("sensor thread started | ");

    let mut instant_speed = 0.0;
    let mut sensor_state;
    println!("entering main loop");
    loop {
        let pulse = time::Instant::now();
        sensor_state = rcv.recv().unwrap();
        let duration = pulse.elapsed();
        if duration.as_secs_f32() < 4.0 {       //don't send speed if we've waited for longer than 4 secs.
            instant_speed = circumference / duration.as_secs_f32();
            println!("{:?}", instant_speed);

            thread::spawn( move || {
                send_speed(instant_speed); // handle error on transmssion. Probably need a transmit thread. 
            });
        } else {
            thread::yield_now();
        }
    }
}
