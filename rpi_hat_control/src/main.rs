//pub mod pin_funcs;
pub mod menus;

use std::error::Error;
use std::thread;
use std::time::Duration;
use rpi_pal::gpio::Gpio;
//use rpi_pal::system::DeviceInfo;

pub struct HatItem {
    pub pin_0_status: bool,
    pub pin_1_status: bool,
    pub pin_2_status: bool,
    pub pin_x_status: bool,
    pub dh_1_duty: f64,
    pub dh_2_duty: f64,
    pub gps_coord: String,
    pub temp: f32,
    pub humidity: f32,
    pub exit: bool,
}

fn main()-> Result<(), Box<dyn Error>>  {

// set initial pin statuses
let mut hat_status = HatItem { 
                                pin_0_status: true,
                                pin_1_status: true,
                                pin_2_status: true,
                                pin_x_status: true,
                                dh_1_duty: 50.0,
                                dh_2_duty: 50.0,
                                gps_coord: String::from("Placeholder Coord"),
                                temp: 25.0,
                                humidity: 20.0,
                                exit: false,
                                };

let gpio = Gpio::new()?;
let mut dc_port_1 = gpio.get(22)?.into_output();
let mut dc_port_2 = gpio.get(23)?.into_output();
let mut dc_port_3 = gpio.get(24)?.into_output();
let mut dc_port_x = gpio.get(25)?.into_output();
let mut dh_pin_1 = gpio.get(12)?.into_output();
let mut dh_pin_2 = gpio.get(13)?.into_output();
dh_pin_1.set_pwm_frequency(10.0, hat_status.dh_1_duty)?;
dh_pin_2.set_pwm_frequency(10.0, hat_status.dh_2_duty)?;

loop {

    menus::menus::draw_menu(&mut hat_status);

    if hat_status.pin_0_status == true {
        dc_port_1.set_high();
    } else {
        dc_port_1.set_low();
    }

    if hat_status.pin_1_status == true {
        dc_port_2.set_high();
    } else {
        dc_port_2.set_low();
    }

    if hat_status.pin_2_status == true {
        dc_port_3.set_high();
    } else {
        dc_port_3.set_low();
    }

    if hat_status.pin_x_status == true {
        dc_port_x.set_high();
    } else {
        dc_port_x.set_low();
    }

    dh_pin_1.set_pwm_frequency(10.0, hat_status.dh_1_duty/100.0)?;
    dh_pin_2.set_pwm_frequency(10.0, hat_status.dh_2_duty/100.0)?;

    if hat_status.exit == true {break};

}










thread::sleep(Duration::from_millis(10000));

Ok(())
}



