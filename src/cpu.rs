use std::fs;

struct CpuCoreInfo {
    core_number: u16,
    hertz: u16,
}

pub fn cpu_info() -> String {
    // Funcion para obtener la info del cpu que requiere hardware_menu
    
    // Cantidad procesadores
    let mut core_num: u16 = 0;
    let mut text = String::new();

    if let Ok(cpuinfo) = fs::read_to_string("/proc/cpuinfo") {
        for line in cpuinfo.lines() {
            if line.starts_with("processor") {
                core_num += 1;
                
            }
            if line.starts_with("cpu MHz") {

                // Get the actual Cpu MHZ.
                let mut hz_actual: String = String::new();
                for (i,c) in line.chars().enumerate() {
                    if c.is_ascii_digit() || c == '.' {
                        hz_actual.push(c);
                    }
                }
                // convert String to float32
                let hz_actual: f32 = hz_actual.parse::<f32>().unwrap();

                // Now we need the Max mhz !
                let hz_max: f32 = {
                    // Read the max hz of the cpu number (core) and we convert: String -> Float32,
                    // trim() is because the file has \n.
                    let hz = fs::read_to_string({
                        format!("/sys/devices/system/cpu/cpu{}/cpufreq/cpuinfo_max_freq",(core_num-1))
                    }).expect("Error").trim().parse::<i32>().unwrap() as f32;
                    hz / 1000.0

                };

                // Finally we need the average mhz 
                let hz_percentage = (hz_actual * 100.0) / hz_max;
                text = hz_percentage.to_string();


                // text = hz_numbers.to_string();
            }

        }
    }
    //
    // text = hz_max.to_string();

    text
}
