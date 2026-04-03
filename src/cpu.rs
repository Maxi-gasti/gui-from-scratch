use std::fs;

struct CpuCoreInfo {
    core_number: u16,
    hertz: u16,
}

pub fn cpu_core_num_info () -> u16 {
    let mut core_num: u16 = 0;

    if let Ok(cpuinfo) = fs::read_to_string("/proc/cpuinfo") {
        for line in cpuinfo.lines() {
            if line.starts_with("processor") {
                core_num += 1;
                
            }
        }
    }
    core_num
}

pub fn cpu_get_text_width (width: u16,core_num: u16) -> i32 {
    let mut x_scale: i32 = (width / core_num) as i32;
    x_scale = x_scale * core_num as i32 + core_num as i32;
    x_scale / 2
}

pub fn cpu_info(width: u16,height: u16) -> String {
    // Funcion para obtener la info del cpu que requiere hardware_menu
    
    // Cantidad procesadores
    let mut core_num: u16 = 0;
    let mut text = String::new();
    let mut cores_average: Vec<f32> = Vec::new();

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

                // Finally we need the average mhz, and push it
                cores_average.push((hz_actual * 100.0) / hz_max);
            }
        }
    }

    for _i2 in 0..cores_average.len() {
        text += &format!("{} ",cores_average[_i2] as u16);
    }
    text += &'\n'.to_string();
    let x_scale: i32 = (width / core_num) as i32;
    print!("{},{}",width,x_scale);
    for i in 0..height {

        for _i2 in 0..cores_average.len() {
            let height_percent: f32 = 100.0 - (( i as f32 / height as f32) * 100.0);

            if height_percent - cores_average[_i2] > 3.0 {
                for _i3 in 0..x_scale {
                    text +=  &"░".to_string();
                }
            } else if height_percent - cores_average[_i2] > 1.0 {
                for _i3 in 0..x_scale {
                    text +=  &"▒".to_string();
                }
            } else {
                for _i3 in 0..x_scale {
                    text += &"█".to_string();
                }
            }
            text += &" ".to_string();

        } 
        text += &'\n'.to_string();
    }

    for i in 0..cores_average.len() {
        text += &format!("C{} ",i);
    }

    text
}

pub fn clock(time: u16) -> String {
    let mut clock: String = String::new();
    match time {
        1 => clock = r#"
         .--.
    .-._;.--.;_.-.
   (_.'_..--.._'._)
    /.' . 60 . '.\
   // .   ||   . \\
  |; .    ||    . |;
  ||45    ()    15||
  |; .          . |;
   \\ .        . //
    \'._' 30 '_.'/
     '-._'--'_.-'
         `""`
"#.to_string(),     
        2 => clock = r#"
         .--.
    .-._;.--.;_.-.
   (_.'_..--.._'._)
    /.' . 60 . '.\
   // .      / . \\
  |; .      /   . |;
  ||45    ()    15||
  |; .          . |;
   \\ .        . //
    \'._' 30 '_.'/
     '-._'--'_.-'
         `""`
"#.to_string(),
        3 => clock = r#"
         .--.
    .-._;.--.;_.-.
   (_.'_..--.._'._)
    /.' . 60 . '.\
   // .        . \\
  |; .          . |;
  ||45    ()====15||
  |; .          . |;
   \\ .        . //
    \'._' 30 '_.'/
     '-._'--'_.-'
         `""`
"#.to_string(),
        4 => clock = r#"
         .--.
    .-._;.--.;_.-.
   (_.'_..--.._'._)
    /.' . 60 . '.\
   // .        . \\
  |; .          . |;
  ||45    ()    15||
  |; .      \   . |;
   \\ .      \ . //
    \'._' 30 '_.'/
     '-._'--'_.-'
         `""`
"#.to_string(),
        5 => clock = r#"
         .--.
    .-._;.--.;_.-.
   (_.'_..--.._'._)
    /.' . 60 . '.\
   // .        . \\
  |; .          . |;
  ||45    ()    15||
  |; .    ||    . |;
   \\ .   ||   . //
    \'._' 30 '_.'/
     '-._'--'_.-'
         `""`
"#.to_string(),
        6 => clock = r#"
         .--.
    .-._;.--.;_.-.
   (_.'_..--.._'._)
    /.' . 60 . '.\
   // .        . \\
  |; .          . |;
  ||45    ()    15||
  |; .   /      . |;
   \\ . /      . //
    \'._' 30 '_.'/
     '-._'--'_.-'
         `""`
"#.to_string(),
        7 => clock = r#"
         .--.
    .-._;.--.;_.-.
   (_.'_..--.._'._)
    /.' . 60 . '.\
   // .        . \\
  |; .          . |;
  ||45====()    15||
  |; .          . |;
   \\ .        . //
    \'._' 30 '_.'/
     '-._'--'_.-'
         `""`
"#.to_string(),
        8 => clock = r#"
         .--.
    .-._;.--.;_.-.
   (_.'_..--.._'._)
    /.' . 60 . '.\
   // . \      . \\
  |; .   \      . |;
  ||45    ()    15||
  |; .          . |;
   \\ .        . //
    \'._' 30 '_.'/
     '-._'--'_.-'
         `""`
"#.to_string(),
        _ => {}
    }
    clock
}
