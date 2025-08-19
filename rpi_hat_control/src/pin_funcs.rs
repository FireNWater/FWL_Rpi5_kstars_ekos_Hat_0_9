
// pub mod pin_funcs {

// use rpi_pal::gpio::Gpio;
// //use rpi_pal::system::DeviceInfo;



//     pub fn all_dc_pins_on() {

//         let gpio_pins = [22, 23, 24, 25];   // The LED pins on the Rpi Hat

//         for led_pin in gpio_pins {
//             let mut pin = Gpio::new().get(led_pin).into_output()?;
//             pin.set_high();
//             }
//     }

//     pub fn all_dh_pins_on(){

//         let gpio_pins = [12, 13];   // The dew heater LED pins on the Rpi Hat

//         for led_pin in gpio_pins {
//             let mut pin = Gpio::new().get(led_pin).into_output();
//             pin.set_high();
//             }  
//     }

// }