use std::{thread::{self, scope}, time::Duration};

use rusb::{Context, UsbContext};

const VENDOR: u16 = 0x37fa;
const DEVICE: u16 = 0x8201;
const INTERFACE: u8 = 0x0;
const ENDPOINT_OUT: u8 = 0x02;
const ENDPOINT_IN: u8 = 0x82;
const WRITE_TIMEOUT: Duration = Duration::from_secs(1);
const READ_TIMEOUT: Duration = Duration::from_millis(1);
fn main() {
    let context = Context::new().expect("cannot open libusb context");
    let device = context.open_device_with_vid_pid(VENDOR, DEVICE).expect("cannot get device");
    let descriptor = device.device().device_descriptor().expect("Cannot describe device");
    if device.kernel_driver_active(INTERFACE).expect("cannot get kernel driver") {
        device.detach_kernel_driver(INTERFACE).expect("cannot detach kernel driver");
    }
    device.claim_interface(INTERFACE).expect("unable to claim interface");

    let command: [u8; 3] = [0x02, 0x00, 0xc0];
    let color: [u8; 3] = [0x0f, 0xff, 0x0f];
    let body: Vec<u8> = command.into_iter().chain(color.into_iter().cycle().take(192)).collect();
    thread::scope(|s|  {
        s.spawn(|| {
            device.write_interrupt(ENDPOINT_OUT, &body, WRITE_TIMEOUT).expect("unable to write to device");
        });
        s.spawn(|| {
            loop {
                let mut buf = [0_u8; 64];
                match device.read_interrupt(ENDPOINT_IN, &mut buf, READ_TIMEOUT) {
                    Ok(_) => println!("Interrupt: {}", buf[0]),
                    Err(rusb::Error::Timeout) => continue,
                    Err(e) => panic!("{e:?}"),
                }          
            }
        });
    });
    
    println!("{descriptor:#?}");

}
