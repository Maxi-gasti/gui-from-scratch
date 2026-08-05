use std::fs;
use std::process::Command;

fn get_total_ram () -> i32 {
    
    // IN KB, KILOBYTE, 1024 BYTES.

    let mut total_ram: i32 = 0;

    if let Ok(meminfo) = fs::read_to_string("/proc/meminfo") {
        for line in meminfo.lines() {
            if line.starts_with("MemTotal") {
                
                // Get the actual Memory Space.
                let mut total: String = String::new();
                for (_i,c) in line.chars().enumerate() {
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
    
    // IN KB, KILOBYTE, 1024 BYTES.

    let mut current_ram: i32 = 0;

    if let Ok(meminfo) = fs::read_to_string("/proc/meminfo") {
        for line in meminfo.lines() {
            if line.starts_with("MemAvailable") {
                
                // Get the actual Memory Space .
                let mut total: String = String::new();
                for (_i,c) in line.chars().enumerate() {
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
    let total = get_total_ram() / 1024;
    let current = get_current_ram() / 1024; // 1024 because is mebibyte, otherwise 1000 is megabyte.

    (current*100)/total
}

fn get_free_ram () -> i32 {
    // it doesnt give u free ram.
    
    // IN KB, KILOBYTE, 1024 BYTES.
    
    let mut free_ram: i32 = 0;

    if let Ok(meminfo) = fs::read_to_string("/proc/meminfo") {
        for line in meminfo.lines() {
            if line.starts_with("MemFree") {
                
                // Get the actual Memory Space .
                let mut total: String = String::new();
                for (_i,c) in line.chars().enumerate() {
                    if c.is_ascii_digit() {
                        total.push(c);
                    }
                }
                // convert String to float32
                free_ram = total.parse::<i32>().unwrap();
                
            }
        }
    }
    free_ram
}

pub fn ram_info (width: u16,height: u16) -> String {
    
    let mut text: String = String::new();

    // Works like a drop
    if height + 2 < 4 || (width as i32- 13) < 0 {
        text = "ERROR-RANGE".to_string();
        return text
    }
    
    text += &String::from("┤ RAM: \n");

    let y_scale: u16 = {
        // Let make this a 3 because have 3 forms, later i will see it better.
        if ((height - 1) / 3) > 0 {
            (height - 1) / 3
        } else {
            1
        }

    };
    
    let percentage = get_percentage_ram();
    

    // if we have 2 spaces left we add \n to center!
    let x = (height - 1) % 3;
    if x/2 == 1 {
        text += &'\n'.to_string();
    }
    let true_width: u16 = width - 14;


    if y_scale > 1 {
        let true_width: u16 = width -3;
        text += &format!(" [ Used: {}% ]",(100 - get_percentage_ram()));
        text += &'\n'.to_string();
        for _i in 0..(y_scale-1) {
            for i2 in 0..true_width {
                if i2 == 0 {
                    text += &String::from("|");
                } else {
                    // This is the percentage of the NEXT(+1) so we can predict when it gonna change.
                    // And added another because the for starts in 0 (+2).
                    let width_percentage = 100.0 - (((i2 as f32 + 2.0) / true_width as f32) * 100.0);
                    if (width_percentage - percentage as f32) > 0.0 {               
                        text += &String::from("—");
                    } else {
                        let width_percentage = 100.0 - (((i2 as f32 + 1.0) / true_width as f32) * 100.0);
                        if (width_percentage - percentage as f32) > 0.0 {
                            text += &String::from("►");
                        } else {
                            if width_percentage == 0.0 {
                                text += &String::from("|");
                            } else {
                                text += &String::from("–");
                            }
                        }
                    }
                }
            }
            if y_scale != 1 || height >= 3{
                text += &'\n'.to_string();
            }
        }
        text += &'\n'.to_string();
        let percentage = { 100 - get_percentage_ram()};
        
        text += &format!(" [ Available: {} ]",get_percentage_ram());
        text += &'\n'.to_string();
        for _i in 0..(y_scale-1) {
            for i2 in 0..true_width {
                if i2 == 0 {
                    text += &String::from("|");
                } else {
                    let width_percentage = 100.0 - (((i2 as f32 + 2.0) / true_width as f32) * 100.0);
                    if (width_percentage - percentage as f32) > 0.0 {               
                        text += &String::from("—");
                    } else {
                        let width_percentage = 100.0 - (((i2 as f32 + 1.0) / true_width as f32) * 100.0);
                        if (width_percentage - percentage as f32) > 0.0 {
                            text += &String::from("►");
                        } else {
                            if width_percentage == 0.0 {
                                text += &String::from("|");
                            } else {
                                text += &String::from("–");
                            }
                        }
                    }
                }
            }
            if y_scale != 1 || height >= 3 {
                text += &'\n'.to_string();
            }
        }
        
        text += &'\n'.to_string();
        let percentage = {
            100 - (get_free_ram()*100)/ get_total_ram()
        };
        text += &format!(" [ Free: {} ]",(get_free_ram()*100 / get_total_ram()));
        text += &'\n'.to_string();
        
        for i in 0..(y_scale-1) {
            for i2 in 0..true_width {
                if i2 == 0 {
                    text += &String::from("|");
                } else {
                    let width_percentage = 100.0 - (((i2 as f32 + 2.0) / true_width as f32) * 100.0);
                    if (width_percentage - percentage as f32) > 0.0 {               
                        text += &String::from("—");
                    } else {
                        let width_percentage = 100.0 - (((i2 as f32 + 1.0) / true_width as f32) * 100.0);
                        if (width_percentage - percentage as f32) > 0.0 {
                            text += &String::from("►");
                        } else {
                            if width_percentage == 0.0 {
                                text += &String::from("|");
                            } else {
                                text += &String::from("–");
                            }
                        }
                    }
                }
            }
            if i != (y_scale -1) {
                text += &'\n'.to_string();
            }
        }
    } else {
        text = text + &String::from("Used:      ");
        for _i in 0..y_scale {
            for i2 in 0..true_width {
                if i2 == 0 {
                    text = text + &String::from("|");
                } else {
                    // This is the percentage of the NEXT(+1) so we can predict when it gonna change.
                    // And added another because the for starts in 0 (+2).
                    let width_percentage = 100.0 - (((i2 as f32 + 2.0) / true_width as f32) * 100.0);
                    if (width_percentage - percentage as f32) > 0.0 {               
                        text = text + &String::from("—");
                    } else {
                        let width_percentage = 100.0 - (((i2 as f32 + 1.0) / true_width as f32) * 100.0);
                        if (width_percentage - percentage as f32) > 0.0 {
                            text = text + &String::from("►");
                        } else {
                            if width_percentage == 0.0 {
                                text = text + &String::from("|");
                            } else {
                                text = text + &String::from("–");
                            }
                        }
                    }
                }
            }
            if y_scale != 1 || (height-1) >= 3{
                text += &'\n'.to_string();
                text += &String::from("           ");
            }
        }
        
        text += &'\n'.to_string();
        let percentage = { 100 - get_percentage_ram() };
        
        text = text + &String::from("Available: ");
        for _i in 0..y_scale {
            for i2 in 0..true_width {
                if i2 == 0 {
                    text = text + &String::from("|");
                } else {
                    // This is the percentage of the NEXT(+1) so we can predict when it gonna change.
                    // And added another because the for starts in 0.
                    let width_percentage = 100.0 - (((i2 as f32 + 2.0) / true_width as f32) * 100.0);
                    if (width_percentage - percentage as f32) > 0.0 {               
                        text = text + &String::from("—");
                    } else {
                        // WHis is the percentage of the now.
                        let width_percentage = 100.0 - (((i2 as f32 + 1.0) / true_width as f32) * 100.0);
                        if (width_percentage - percentage as f32) > 0.0 {
                            text = text + &String::from("►");
                        } else {
                            if width_percentage == 0.0 {
                                text = text + &String::from("|");
                            } else {
                                text = text + &String::from("–");
                            }
                        }
                    }
                }
            }
            if y_scale != 1 || (height - 1) >= 3 {
                text += &'\n'.to_string();
                text += &String::from("           ");
            }
        }
        text += &'\n'.to_string();
        let percentage = {
            100 - (get_free_ram()*100)/ get_total_ram()
        };
        text = text + &String::from("Free:      ");
        for i in 0..y_scale {
            for i2 in 0..true_width {
                if i2 == 0 {
                    text = text + &String::from("|");
                } else {
                    // This is the percentage of the NEXT(+1) so we can predict when it gonna change.
                    // And added another because the for starts in 0.
                    let width_percentage = 100.0 - (((i2 as f32 + 2.0) / true_width as f32) * 100.0);
                    if (width_percentage - percentage as f32) > 0.0 {               
                        text = text + &String::from("—");
                    } else {
                        // WHis is the percentage of the now.
                        let width_percentage = 100.0 - (((i2 as f32 + 1.0) / true_width as f32) * 100.0);
                        if (width_percentage - percentage as f32) > 0.0 {
                            text = text + &String::from("►");
                        } else {
                            if width_percentage == 0.0 {
                                text = text + &String::from("|");
                            } else {
                                text = text + &String::from("–");
                            }
                        }
                    }
                }
            }
            if i != (y_scale -1) {
                text += &'\n'.to_string();
                text += &String::from("           ");
            }
        }
    }
    text
}


// This is commented because get_disk_used_space uses % and will renplace this.

// fn get_disk_space() -> i32 {
//
//     // IN KB, KILOBYTE, 1024 BYTES.
//
//     let mut disk_space: i32 = 0;
//
//     let disk_name: String = {
//         let mut name = String::new();
//         if let Ok(diskinfo) = fs::read_to_string("/proc/mounts") {
//             for (i,c) in diskinfo.chars().enumerate() {
//                 if c == ' ' { break }
//                 name.push(c);
//             }
//         }
//         let mut name_fixed = String::new();
//         let name_len: i32 = name.len() as i32 - 2;
//         for (i,c) in name.chars().enumerate() {
//             if i > 4 && i < name_len as usize {
//                 name_fixed.push(c);
//             } 
//         }
//         name_fixed
//     };
//
//     if let Ok(diskinfo) = fs::read_to_string("/proc/partitions") {
//         for line in diskinfo.lines() {
//             if line.ends_with(&disk_name) {
//
//                 let mut text = String::new();
//
//                 let mut places: i32 = 3;
//                 // let mut in_place: i32 = false;
//                 let mut space: bool = true;
//
//                 for (i,c) in line.chars().enumerate() {
//                     if space == true {
//                         if c != ' ' {
//                             space = false;
//                             places -= 1;
//                             if places == 0 {
//                                 text.push(c);
//                             }
//                         }
//                     } else {
//                         if c == ' '{
//                             space = true;
//                         } else if places == 0 {
//                             text.push(c);
//                         }
//                     }
//                 }
//
//
//                 disk_space = text.parse::<i32>().unwrap();
//
//                 break
//             }
//         }
//     }
//     print!("{}",disk_space);
//     disk_space
// }

fn get_disk_used_space () -> i32 {

    let mut disk_space: i32 = 0;

    let output = Command::new("df").arg("-h").output().expect("Error df");
    let output = String::from_utf8(output.stdout);

    for line in output.expect("Error utf8").lines() {
        if line.ends_with("/") {
            let mut text = String::new();

            let mut places: i32 = 5;
            // let mut in_place: i32 = false;
            let mut space: bool = true;

            for (_i,c) in line.chars().enumerate() {
                if space == true {
                    if c != ' ' {
                        space = false;
                        places -= 1;
                        if places == 0 {
                            text.push(c);
                        }
                    }
                } else {
                    if c == ' '{
                        space = true;
                    } else if places == 0 {
                        if c.is_ascii_digit() {
                            text.push(c);
                        }
                    }
                }
            }

            disk_space = text.parse::<i32>().unwrap();
                
            break

        }
    }
    disk_space
}

pub fn disk_info (width: u16,height: u16) -> String {

    let mut text: String = String::new();

    if height + 2 < 2 || (width as i32 - 13) < 0 {
        // Actualy i put 2 but can have issues with the normal scale. idk i dont think u will have
        // 90 px resolution haha
        text = "ERROR-RANGE".to_string();
        return text
    }
    
    text += &String::from("┤ ROM: \n");

    let y_scale: u16 = {
        if ((height-1) / 2) > 0 {
            (height-1) / 2
        } else {
            1
        }
    };
    if height != 2 {
        let x = (height - 1) % 2; // This cam return 1.5 so we need the >=
        if x >= 1 {
            text += &'\n'.to_string();
        }
    }

    let percentage = 100 - get_disk_used_space();

    text = text + &String::from("Used:      ");
    for _i in 0..y_scale {
        for i2 in 0..(width - 11 - 3) {
            let width_percentage = 100.0 - ((i2 as f32 / width as f32) * 100.0);
            if (width_percentage - percentage as f32) > 5.0 {
                text = text + &String::from("█");
            } else if (width_percentage - percentage as f32) > 2.5 {
                text +=  &"▒".to_string();
            } else {
                text +=  &"░".to_string();
            }
            // text = text + &String::from("█");
        }
        if y_scale != 1 || (height - 1) >= 2 {
            text += &'\n'.to_string();
            text += &String::from("           ");
        }
    }

    text += &'\n'.to_string();
    let percentage = get_disk_used_space();

    text = text + &String::from("Available: ");
    for i in 0..y_scale {
        for i2 in 0..(width - 11 - 3) {
            let width_percentage = 100.0 - ((i2 as f32 / width as f32) * 100.0);
            if (width_percentage - percentage as f32) > 5.0 {
                text = text + &String::from("█");
            } else if (width_percentage - percentage as f32) > 2.5 {
                text +=  &"▒".to_string();
            } else {
                text +=  &"░".to_string();
            }
        }
        if i != (y_scale -1) {
            text += &'\n'.to_string();
            text += &String::from("           ");
        }
    }

    text 
}
