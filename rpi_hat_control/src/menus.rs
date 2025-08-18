
pub mod menus {
    
use crate::HatItem;

pub fn draw_menu (hat_items: &mut HatItem){

    println!("|-------------------------------------------------|");
    println!("|                 Rpi-Hat Control                 |");
    println!("|-------------------------------------------------|");
    println!("|                                                 |");
    println!("|           1) DC #1 - {}   2) DC #2 - {}         |",hat_items.pin_0_status, hat_items.pin_1_status);
    println!("|                                                 |");
    println!("|           3) DC #3 - {}   4) DC #X - {}         |",hat_items.pin_2_status, hat_items.pin_x_status);
    println!("|                                                 |");
    println!("|           5) DH #1 - {}   6) DH #2 - {}         |",hat_items.dh_1_duty, hat_items.dh_2_duty);
    println!("|                                                 |");
    println!("|                   7) Exit                       |");
    println!("|-------------------------------------------------|");
    println!("|                                                 |");
    println!("|           GPS Coordinates:  {}                  |",hat_items.gps_coord);
    println!("|                                                 |");
    println!("|           Temp/Humidity:  {} / {}               |",hat_items.temp, hat_items.humidity);
    println!("|                                                 |");
    println!("|-------------------------------------------------|");
    println!(" ");
    println!(" ");
    println!("Enter item to change:");
    
    let mut menu_choice = String::new();
    std::io::stdin().read_line(&mut menu_choice).expect("Error reading input");
    let menu_choice: i32 = menu_choice.trim().parse().expect("Please type number 1 - 7.");

    match menu_choice {
                1 => hat_items.pin_0_status = !hat_items.pin_0_status,
                2 => hat_items.pin_1_status = !hat_items.pin_1_status,
                3 => hat_items.pin_2_status = !hat_items.pin_2_status,
                4 => hat_items.pin_x_status = !hat_items.pin_x_status,
                5 => hat_items.dh_1_duty = get_new_duty(),
                6 => hat_items.dh_2_duty = get_new_duty(),
                7 => hat_items.exit = true,
                _ =>  println!("Input error"),   
        }          
    }

    fn get_new_duty() -> f64 {
        println!("Enter new duty cycle: (0-100)");
        let mut new_duty = String::new();
        std::io::stdin().read_line(&mut new_duty).expect("Error reading input");
        let new_duty: f64 = new_duty.trim().parse().expect("Please type number.");
        new_duty
    }

}
