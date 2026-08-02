use std::env;
use std::fs;

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
    
    let mut model_name = String::new();
    
    if let Ok(cpuinfo) = fs::read_to_string("/proc/meminfo") {
        for line in cpuinfo.lines() {
            if line.starts_with("MemTotal") {
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
