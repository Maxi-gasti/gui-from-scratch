use std::fs;

fn get_total_ram () -> i32 {

    let mut total_ram: i32 = 0;

    if let Ok(meminfo) = fs::read_to_string("/proc/meminfo") {
        for line in meminfo.lines() {
            if line.starts_with("MemTotal") {
                
                // Get the actual Memory Space (KB).
                let mut total: String = String::new();
                for (i,c) in line.chars().enumerate() {
                    if c.is_ascii_digit() {
                        total.push(c);
                    }
                }
                // convert String to float32
                total_ram = total.parse::<i32>().unwrap();

            }
        }
    }
    total_ram
}


fn get_current_ram () -> i32 {

    let mut current_ram: i32 = 0;

    if let Ok(meminfo) = fs::read_to_string("/proc/meminfo") {
        for line in meminfo.lines() {
            if line.starts_with("MemAvailable") {
                
                // Get the actual Memory Space (KB).
                let mut total: String = String::new();
                for (i,c) in line.chars().enumerate() {
                    if c.is_ascii_digit() {
                        total.push(c);
                    }
                }
                // convert String to float32
                current_ram = total.parse::<i32>().unwrap();
                
            }
        }
    }
    current_ram
}

fn get_percentage_ram () -> i32 {
    let total = get_total_ram() / 1000;
    let current = get_current_ram() / 1000;

    (current*100)/total
}

pub fn ram_info (width: u16,height: u16) -> String {
    let mut text: String = String::new();

    // Works like a drop
    if height < 5 && (width - 11) < 0 {
        text = "ERROR-RANGE".to_string();
        return text
    }

    let y_scale: u16 = {

        height / 3
    };
    
    let percentage = get_percentage_ram();

    text = text + &String::from("Used:      ");
    for i in 0..y_scale {
        for i2 in 0..(width - 11 - 3) {
            let width_percentage = 100.0 - ((i2 as f32 / width as f32) * 100.0);
            if (width_percentage - percentage as f32) > 5.0 {
                text = text + &String::from("█");
            } else {
                text +=  &"░".to_string();
            }
            // text = text + &String::from("█");
        }
        text += &'\n'.to_string();
        text += &String::from("           ");
    }

    text += &'\n'.to_string();
    text = text + &String::from("Available: ");
    for i in 0..y_scale {
        for i2 in 0..(width - 11 - 3) {
            text = text + &String::from("█");
        }
        text += &'\n'.to_string();
        text += &String::from("           ");
    }
    text += &'\n'.to_string();
    text = text + &String::from("Free:      ");
    for i in 0..y_scale {
        for i2 in 0..(width - 11 - 3) {
            text = text + &String::from("█");
        }
        if i != (y_scale -1) {
            text += &'\n'.to_string();
            text += &String::from("           ");
        }
    }

    text
}
