use std::env;
use std::fs;
use std::process::Command;
pub fn user_name() -> String { // it cant return str, because have unknown size

    let user = env::var("USER").unwrap_or_else(|_| "Unknown".to_string());
    return user;
}

pub fn cpu_model() -> String {
    
    let mut model_name = String::new();
    
    if let Ok(cpuinfo) = fs::read_to_string("/proc/cpuinfo") {
        for line in cpuinfo.lines() {
            if line.starts_with("model name") {
                let mut start_count: bool = false;
                let mut start_countt: bool = false; // for spaces.
                for (_i,c) in line.chars().enumerate() {
                    if start_count && start_countt {
                        model_name.push(c);
                    } else {
                        if c != ' ' && start_count {
                            start_countt = true;
                            model_name.push(c);
                        }
                        if c == ':' {
                            start_count = true;
                        }
                    }
                }
                break
            }
        }
    }
    if model_name != String::new() {
        return model_name;
    } else {
        return "Unknown".to_string();
    }
}

pub fn ram_model() -> String {
    
    let mut ram_model = String::new();
    
    if let Ok(cpuinfo) = fs::read_to_string("/proc/meminfo") {
        for line in cpuinfo.lines() {
            if line.starts_with("MemTotal") {
                let mut start_count: bool = false;
                let mut start_countt: bool = false; // for spaces.
                for (_i,c) in line.chars().enumerate() {
                    if c.is_ascii_digit() {
                        ram_model.push(c);
                    }
                }
                break
            }
        }
    }
    if ram_model != String::new() {
        let mut ram_model = ram_model.parse::<f32>().unwrap();
        ram_model = (ram_model / 1024.0) / 1024.0;
        let ram_model = format!("{:.2} GB",ram_model);
        return ram_model;
    } else {
        return "Unknown".to_string();
    }
}

pub fn thermal_cpu() -> String {
    let mut thermal_cpu: i32 = 0;

    if let Ok(thermal) = fs::read_to_string("/sys/class/thermal/thermal_zone0/temp") {
        thermal_cpu = thermal.trim().parse::<i32>().unwrap();
        thermal_cpu = thermal_cpu / 1000;
    }
    
    return thermal_cpu.to_string();
}

pub fn disk_model() -> String {

    let mut disk_space: i32 = 0;

    let output = Command::new("df").arg("-h").output().expect("Error df");
    let output = String::from_utf8(output.stdout);

    for line in output.expect("Error utf8").lines() {
        if line.ends_with("/") {
            let mut text = String::new();

            let mut places: i32 = 2;
            // let mut in_place: i32 = false;
            let mut space: bool = true;

            for (_i,c) in line.chars().enumerate() {
                if space == true {
                    if c != ' ' {
                        space = false;
                        places -= 1;
                        if places == 0 && c.is_ascii_digit() {
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

            disk_space = text.trim().parse::<i32>().unwrap();
                
            break

        }
    }

    return disk_space.to_string();
}

// pub fn cpu_core_num_info () -> u16 {
//     let mut core_num: u16 = 0;
//
//     if let Ok(cpuinfo) = fs::read_to_string("/proc/cpuinfo") {
//         for line in cpuinfo.lines() {
//             if line.starts_with("processor") {
//                 core_num += 1;   
//             }
//         }
//     }
//     core_num
// }


// pub fn 
