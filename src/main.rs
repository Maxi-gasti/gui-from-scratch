pub mod models; // Models, labels, typelabels, and stylelabels.
pub mod cpu; // the fn that returns cpu info like core_nums text_size of the core etc.
pub mod memory; // same like cpu, but with the ram.
pub mod gui; // functions to operate 2 dimentions vector, for gui proposes, like add labels etc.
pub mod time_weather; // time and weather libs.
pub mod pc_info; // computer info libs
use crossterm::terminal::size;
use std::io;
use std::fs;
use std::io::Write;
use std::env;

use log::{info,warn};
// use std::thread;
// use std::fs;

const BLACK: &str = "\x1b[90m";
const GREEN: &str = "\x1b[32m";
const YELLOW2: &str = "\x1b[4;1;33m";
const YELLOW: &str = "\x1b[1;33m";
const RED: &str = "\x1b[1;31m";
const RESET: &str = "\x1b[0m";


// this struct is made for crossterm disable, so if the program panic or Ctrl+C just drop the
// disable function.
struct RawModeGuard;

impl RawModeGuard {
    fn new() -> io::Result<Self> {
        crossterm::terminal::enable_raw_mode()?;
        Ok(Self)
    }
}

impl Drop for RawModeGuard {
    fn drop(&mut self) {
        let _ = crossterm::terminal::disable_raw_mode();
        if let Ok(mut file_log) = fs::OpenOptions::new().create(true).append(true).open("log.txt") {
            let local_time = time_weather::get_time();
            let text = format!("{}: {}DROP:{} El programa dropeo exitosamente",local_time,RED,RESET);
            let _ = writeln!(file_log,"{}",text);
        }

    }
}

fn main() -> io::Result<()> {

    // This let the program to recibe raw touchs, like you not need to put a input fn to insert
    // letters.
    
    // 24-04-26 let put order here, i will use now functions more big for this because for me this
    // is thrash so, lets compare 2 modes, global variables and functions. NOW the window_map ,
    // label etc will be in gui.rs
    //
    // I transfereed  the mostlly of the func, now add the another for suffuly run the program.
    // have a good day :)
    //
    // 04-07 helloooo, i finished moving the func to the gui.rs and anothers, and works, so is more
    // clean here in main.rs and now i need to FINISH the fucking gui.rs problem with scaling, and
    // do exceptions for gui scaling...
    //
    // 06-07 Stable version now! need to change a few things and i can continue with adding things
    
    let _raw_guard = RawModeGuard::new()?;
    
    // Scope para escribir el log.
    {
        let mut file_log = fs::OpenOptions::new().create(true).append(true).open("log.txt")?;
        let local_time = time_weather::get_time();
        let text = format!("{}: {}Program init{}",local_time,GREEN,RESET);
        writeln!(file_log,"{}",text)?;
        let user = pc_info::user_name();
        let local_time = time_weather::get_time();
        let text = format!("{}: {}User:{} {}",local_time,YELLOW,RESET,user);
        writeln!(file_log,"{}",text)?;
        let model = pc_info::cpu_model();
        let local_time = time_weather::get_time();
        let text = format!("{}: {}cpu model:{}{}",local_time,YELLOW,RESET,model);
        writeln!(file_log,"{}",text)?;
        let model = pc_info::ram_model();
        let local_time = time_weather::get_time();
        let text = format!("{}: {}ram model:{}{}",local_time,YELLOW,RESET,model);
        writeln!(file_log,"{}",text)?;
    }

    let mut select: i16 = 0;
    let mut menu_location: &str = "menu";

    // Esto se cuenta en lineas no en pixeles.
    let (mut terminal_x,mut terminal_y) = size()?;
    
    let mut vec_labels = gui::asign_labels(vec![format!("HARDWARE CHECK"),format!("HOUR"),format!("LABELS STYLES"),format!("CONFIG"),"LEAVE".to_string()],terminal_x as i32,terminal_y as i32);
    let mut select_labels = gui::define_select_labels(&vec_labels);
    
    let mut window_map: Vec<Vec<String>> = gui::map_window(terminal_x,terminal_y);

    // This is trying to be a buffer for printing GUI, i cant use all time calculations because i
    // cant depend 100% on the cpu, so, i will use this like a buffer, this is my first time using
    // it with something like 1920x1020 lol.
    let mut window_label: Vec<Vec<String>> = window_map.clone();
    window_label = gui::label_window(&window_map,select,&vec_labels,&select_labels);
    gui::print_gui(&window_label,terminal_x,terminal_y);

    loop {

        match crossterm::event::read()? {
            crossterm::event::Event::Resize(width,height) => {
                terminal_x = width;
                terminal_y = height;
                window_map = gui::map_window(width,height);
                vec_labels = gui::reset_labels(vec_labels,width as i32, height as i32);
                select_labels = gui::define_select_labels(&vec_labels);
                // 
                window_label = gui::label_window(&window_map,select,&vec_labels,&select_labels);
                gui::print_gui(&window_label,terminal_x,terminal_y);
            },
            crossterm::event::Event::Key(crossterm::event::KeyEvent {code, ..} ) => { // DESEMPAQUETA EL STRUCT Keyevent y sacas el code que seria un keycode
                match code {
                    crossterm::event::KeyCode::Char('q') => break,
                    crossterm::event::KeyCode::Char('w') | crossterm::event::KeyCode::Up => {
                        if (select-1) >= 0 as i16 {select -= 1}
                        // 
                        window_label = gui::label_window(&window_map,select,&vec_labels,&select_labels);
                        gui::print_gui(&window_label,terminal_x,terminal_y);
                    },
                    crossterm::event::KeyCode::Char('s') | crossterm::event::KeyCode::Down => {
                        if (select+1) <= (select_labels.iter().len() as i16 - 1) { select += 1}
                        // 
                        window_label = gui::label_window(&window_map,select,&vec_labels,&select_labels);
                        gui::print_gui(&window_label,terminal_x,terminal_y);
                    },

                    // aca las views de los labels

                    crossterm::event::KeyCode::Enter => {
                        match menu_location {
                            "menu" => match select {
                                    0 => {
                                        if let Ok(mut file_log) = fs::OpenOptions::new().create(true).append(true).open("log.txt") {
                                            let local_time = time_weather::get_time();
                                            let text = format!("{}: {}hardware menu init{}",local_time,GREEN,RESET);
                                            writeln!(file_log,"{}",text)?;
                                        }

                                        // ACA UN MENU GUI!
                                        // let text = format!("terminal_x antes de hardware: {}", terminal_x);
                                        hardware_menu(&window_map, &mut terminal_x, &mut terminal_y);
                                        // let text = format!("terminal_x despues de hardware: {}", terminal_x);
                                        
                                        if let Ok(mut file_log) = fs::OpenOptions::new().create(true).append(true).open("log.txt") {
                                            let local_time = time_weather::get_time();
                                            let text = format!("{}: {}hardware menu end{}",local_time,GREEN,RESET);
                                            writeln!(file_log,"{}",text)?;
                                        }

                                        // if terminal_x CHANGES then repeat
                                        window_map = gui::map_window(terminal_x,terminal_y);
                                        vec_labels = gui::reset_labels(vec_labels,terminal_x as i32, terminal_y as i32);
                                        select_labels = gui::define_select_labels(&vec_labels);
                                        
                                        window_label = gui::label_window(&window_map,select,&vec_labels,&select_labels);
                                        gui::print_gui(&window_label,terminal_x,terminal_y);

                                    },
                                    1 => {
                                        hour_weather_menu(&window_map, &mut terminal_x, &mut terminal_y);

                                        window_map = gui::map_window(terminal_x,terminal_y);
                                        vec_labels = gui::reset_labels(vec_labels,terminal_x as i32, terminal_y as i32);
                                        select_labels = gui::define_select_labels(&vec_labels);
                                        
                                        window_label = gui::label_window(&window_map,select,&vec_labels,&select_labels);
                                        gui::print_gui(&window_label,terminal_x,terminal_y);

                                    },
                                    2 => {
                                        select = 0;
                                        menu_location = "styles_menu";
                                        vec_labels = vec![
                                            gui::create_label(&String::from("TEXT"), Some(&40), Some(&4), Some(models::LabelType::Line),Some(models::LabelStyle::Text)),
                                            gui::create_label(&String::from("BORDER"), Some(&40), Some(&10), Some(models::LabelType::Line), Some(models::LabelStyle::Border)),
                                            gui::create_label(&String::from("DOBLE_BORDER"), Some(&70), Some(&4), Some(models::LabelType::Line),Some(models::LabelStyle::DobleBorder)),
                                            gui::create_label(&String::from("BOTTOM_BORDER"), Some(&70), Some(&10), Some(models::LabelType::Line),Some(models::LabelStyle::BottomBorder)),
                                            gui::create_label(&String::from("EDGES"), Some(&100), Some(&4), Some(models::LabelType::Line),Some(models::LabelStyle::Edges)),
                                            gui::create_label(&String::from("Return to menu"), Some(&10),Some(&(terminal_y as i32 -10)),Some(models::LabelType::Select),Some(models::LabelStyle::BottomBorder))
                                        ];
                                        select_labels = gui::define_select_labels(&vec_labels);
                                        window_label = gui::label_window(&window_map,select,&vec_labels,&select_labels);
                                        //
                                        gui::print_gui(&window_label,terminal_x,terminal_y);
                                    },
                                    3 => {
                                        select = 0;
                                        menu_location = "config";
                                        // vec_labels = vec![
                                        //     create_label(&String::from(""),),
                                        // ]
                                        vec_labels = gui::asign_labels(vec![format!("CONFIG"),format!("COLOR"),"nose".to_string(),"dosdos".to_string()],terminal_x as i32,terminal_y as i32);
                                        select_labels = gui::define_select_labels(&vec_labels);
                                        // 
                                        window_label = gui::label_window(&window_map,select,&vec_labels,&select_labels);
                                        gui::print_gui(&window_label,terminal_x,terminal_y);
                                    },
                                    4 => { gui::clear_terminal(); break},
                                    _ => {},
                                },
                            "hour_weather" => match select {
                                    0 => {
                                        select = 0;
                                        vec_labels = gui::asign_labels(vec![format!("HARDWARE CHECK"),format!("HOUR"),format!("LABELS STYLES"),format!("CONFIG"),"LEAVE".to_string()],terminal_x as i32,terminal_y as i32);
                                        select_labels = gui::define_select_labels(&vec_labels);
                                        menu_location = "menu";
                                        // 
                                        window_label = gui::label_window(&window_map,select,&vec_labels,&select_labels);
                                        gui::print_gui(&window_label,terminal_x,terminal_y);
                                    },
                                    _ => {},

                            }
                            "styles_menu" => match select {
                                    0 => {
                                        select = 0;
                                        vec_labels = gui::asign_labels(vec![format!("HARDWARE CHECK"),format!("HOUR"),format!("LABELS STYLES"),format!("CONFIG"),"LEAVE".to_string()],terminal_x as i32,terminal_y as i32);
                                        select_labels = gui::define_select_labels(&vec_labels);
                                        menu_location = "menu";
                                        // 
                                        window_label = gui::label_window(&window_map,select,&vec_labels,&select_labels);
                                        gui::print_gui(&window_label,terminal_x,terminal_y);
                                    },
                                    _ => {},

                            }
                            "hardware_menu" => match select {
                                    0 => {
                                        select = 0;
                                        vec_labels = gui::asign_labels(vec![format!("HARDWARE CHECK"),format!("HOUR"),format!("LABELS STYLES"),format!("CONFIG"),"LEAVE".to_string()],terminal_x as i32,terminal_y as i32);
                                        select_labels = gui::define_select_labels(&vec_labels);
                                        menu_location = "menu";
                                        // 
                                        window_label = gui::label_window(&window_map,select,&vec_labels,&select_labels);
                                        gui::print_gui(&window_label,terminal_x,terminal_y);
                                    },
                                    _ => {},

                            }
                            "config" => match select {
                                    0 => {},
                                    1 => {},
                                    2 => {},
                                    3 => {
                                        select = 0;
                                        vec_labels = gui::asign_labels(vec![format!("HARDWARE CHECK"),format!("HOUR"),format!("LABELS STYLES"),format!("CONFIG"),"LEAVE".to_string()],terminal_x as i32,terminal_y as i32);
                                        select_labels = gui::define_select_labels(&vec_labels);
                                        menu_location = "menu";
                                        // 
                                        window_label = gui::label_window(&window_map,select,&vec_labels,&select_labels);
                                        gui::print_gui(&window_label,terminal_x,terminal_y);
                                    },
                                    _ => {},
                                }
                            _ => {},
                        }
                    },
                    _ => {}
                }
            }
            _ => {}
        }
    }
    // To fix the bad line that leaves.
    let _ = crossterm::terminal::disable_raw_mode();
    gui::clear_terminal();
    if let Ok(mut file_log) = fs::OpenOptions::new().create(true).append(true).open("log.txt") {
        let local_time = time_weather::get_time();
        let text = format!("{}: {}Program end{}",local_time,GREEN,RESET);
        writeln!(file_log,"{}",text)?;
    }
    println!("Good bye!");
    Ok(())
}

fn hardware_menu(window_map: &Vec<Vec<String>>,terminal_x: &mut u16, terminal_y: &mut u16) -> io::Result<()> {
    
    let select_hardware = 0;

    let mut true_x = *terminal_x - 2;
    let mut true_y = *terminal_y - 2;
    
    let mut clock_time: u16 = 1;  
    let mut show_clock = {
        // 20 is the size of the clock
        if cpu::cpu_core_num_info() * 3 + 30 <=  percentage(true_x as i32,60) as u16 {
            true
        } else {
            false
        }
    };
    
    print!("wind_x: {}",terminal_x);
    print!("wind_y: {}",terminal_y);

    // min wind_x = 94 , true_x = 92
    // min wind_y = 21 , true_y = 19

    // if terminal_x < 94 || terminal_y < 21 {
    //
    // } else {
        // Clon of window_map for not touch the main window_label.
        let mut window_label_hardware = window_map.clone();
        gui::put_hardware_lines_map(&mut window_label_hardware,true_x,true_y);

        let mut vec_label_hardware = vec![
            gui::create_label(
                &String::from("Hardware Check"), 
                Some(&{
                    let x = percentage(true_x as i32,60) + 4.0;
                    // this +1 is because x interrup the line because the style of the label can be
                    // upper
                    x as i32 + 1
                }),
                Some(&{
                    let y = percentage(true_y as i32,60) + 2.0;
                    y as i32 + 1
                }),
                Some(models::LabelType::Line), 
                Some(models::LabelStyle::BottomBorder)),
            gui::create_label(
                &String::from("Leave 'Enter'"), 
                Some(&{
                    let x = percentage(true_x as i32,60) + 5.0;
                    x as i32 + 1
                }),
                Some(&{
                    let y = true_y as f32 - 3.0;
                    y as i32 + 1
                }),
                Some(models::LabelType::Line), 
                Some(models::LabelStyle::Edges))
        ];

        let mut vec_label_hardware_select = gui::define_select_labels(&vec_label_hardware);

        window_label_hardware = gui::label_window(&window_label_hardware,select_hardware, &vec_label_hardware,&vec_label_hardware_select);

        gui::add_label_to_window(&mut window_label_hardware, gui::create_label(
            &memory::ram_info((percentage(true_x as i32,60) - 2.0) as u16, (percentage(true_y as i32,20)-4.0) as u16), // the only way to skip the spaces from the diagonal is adding -2
            // the -4 is because (-2) for the 2 lines of put_hardware_lines_map() and another 2 because this are the lines that center the text, in the fn ram_info is added +2 for the real range so is the same.
            Some(&(3 as i32)),
            Some(&(2 as i32)),
            Some(models::LabelType::Text),
            Some(models::LabelStyle::Text)
        ));
        gui::add_label_to_window(&mut window_label_hardware, gui::create_label(
            &memory::disk_info((percentage(true_x as i32,60) - 2.0) as u16, (percentage(true_y as i32,20)-3.0) as u16),
            Some(&(3 as i32)),
            Some(&{
                percentage(true_y as i32, 20) as i32 + 2
            }),
            Some(models::LabelType::Text),
            Some(models::LabelStyle::Text)
        ));

        gui::add_label_to_window(&mut window_label_hardware, gui::create_label(
            &cpu::cpu_info((percentage(true_x as i32,40) - 32.0) as u16,(percentage(true_y as i32,60) - 6.0) as u16),
            Some(&{
                let mut x: f32 = percentage(true_x as i32,60) + 1.0;
                x += 20.0;

                // The problem is HERE, the v is miscalculated. so when is centered collapse the
                // entire gui !
                //
                // for now i will put it in the beginning. 

                let mut v: f32 = 0.0;
                v = cpu::cpu_get_text_width((percentage(true_x as i32,40) - 32.0) as u16, cpu::cpu_core_num_info()) as f32;
                // let size = percentage(true_x as i32,40) - 32.0;
                // if size 
                // if (percentage(true_x as i32, 40) - cpu::cpu_get_text_width((percentage(true_x as i32,40) - 32.0) as u16,cpu::cpu_core_num_info()) as f32 - 20.0) <= 20.0 {
                //     v = 0.0;
                // } else {
                //     v = percentage(true_x as i32, 40) - cpu::cpu_get_text_width((percentage(true_x as i32,40) - 32.0) as u16,cpu::cpu_core_num_info()) as f32 - 20.0;
                //     v = v - 20.0;
                //     v = v / 2.0;
                // }
                x = x + v;
                x as i32 + 2
            }),
            Some(&{
                let y = 2;
                y as i32 + 1
            }),
            Some(models::LabelType::Text),
            Some(models::LabelStyle::Text)
        ));

        if show_clock {
            gui::add_label_to_window(&mut window_label_hardware, gui::create_label(
                &cpu::clock(clock_time),
                Some(&{
                    let x = percentage(true_x as i32,60) + 1.0;
                    x as i32 + 1
                }),
                Some(&{
                    let y = percentage(true_y as i32,5);
                    y as i32 + 1
                }),
                Some(models::LabelType::Text),
                Some(models::LabelStyle::Text)
            ));
        }
        gui::print_gui(&window_label_hardware,*terminal_x,*terminal_y);
        if clock_time+1 >= 9 {
            clock_time = 1;
        } else {
            clock_time += 1;
        }
    loop {
        if crossterm::event::poll(std::time::Duration::from_millis(1000))? {
            match crossterm::event::read()? {
                crossterm::event::Event::Resize(width,height) => {
                    *terminal_x = width;
                    *terminal_y = height;
                    
                    true_x = *terminal_x - 2;
                    true_y = *terminal_y - 2;
                    
                    // THIS WILL ALL HAPPEN AGAIN. because i dont wanna do a function just for
                    // repeat 2 times.
                    
                    // Clon of window_map for not touch the main window_label.
                    window_label_hardware = gui::map_window(*terminal_x,*terminal_y);
                    gui::put_hardware_lines_map(&mut window_label_hardware,true_x,true_y);
                    vec_label_hardware = vec![
                        gui::create_label(
                            &String::from("Hardware Check"), 
                            Some(&{
                                let x = percentage(true_x as i32,60) + 4.0;
                                x as i32 + 1
                            }),
                            Some(&{
                                let y = percentage(true_y as i32,60) + 2.0;
                                y as i32 + 1
                            }),
                            Some(models::LabelType::Line), 
                            Some(models::LabelStyle::BottomBorder)),
                        gui::create_label(
                            &String::from("Leave 'Enter'"), 
                            Some(&{
                                let x = percentage(true_x as i32,60) + 5.0;
                                x as i32 + 1
                            }),
                            Some(&{
                                let y = true_y as f32 - 3.0;
                                y as i32 + 1
                            }),
                            Some(models::LabelType::Line), 
                            Some(models::LabelStyle::Edges)
                        )
                    ];
                    
                    vec_label_hardware_select = gui::define_select_labels(&vec_label_hardware);
                    window_label_hardware = gui::label_window(&window_label_hardware,select_hardware, &vec_label_hardware,&vec_label_hardware_select);

                },
                crossterm::event::Event::Key(key) => {
                    match key.code {
                        crossterm::event::KeyCode::Char('q') => break,
                        crossterm::event::KeyCode::Enter => {
                            break
                        },
                        _ => {},
                    }
                },
                _ => {},
            }
        }

        gui::add_label_to_window(&mut window_label_hardware, gui::create_label(
            &memory::ram_info((percentage(true_x as i32,60) - 2.0) as u16, (percentage(true_y as i32,20)-4.0) as u16), 
            Some(&(3 as i32)),
            Some(&(2 as i32)),
            Some(models::LabelType::Text),
            Some(models::LabelStyle::Text)
        ));
        gui::add_label_to_window(&mut window_label_hardware, gui::create_label(
            &memory::disk_info((percentage(true_x as i32,60) - 2.0) as u16, (percentage(true_y as i32,20)-3.0) as u16), 
            Some(&(3 as i32)),
            Some(&{
                percentage(true_y as i32, 20) as i32 + 2
            }),
            Some(models::LabelType::Text),
            Some(models::LabelStyle::Text)
        ));

        gui::add_label_to_window(&mut window_label_hardware, gui::create_label(
            &cpu::cpu_info((percentage(true_x as i32,40) - 32.0) as u16,(percentage(true_y as i32,60) - 6.0) as u16),
            Some(&{
                let mut x: f32 = percentage(true_x as i32,60) + 1.0;
                x += 20.0;

                // The problem is HERE, the v is miscalculated. so when is centered collapse the
                // entire gui !
                //
                // for now i will put it in the beginning. 

                let mut v: f32 = 0.0;
                v = cpu::cpu_get_text_width((percentage(true_x as i32,40) - 32.0) as u16, cpu::cpu_core_num_info()) as f32;
                // // so v is the center, idk how to get it really. i mean 40% and rest 20 from the 
                // if (percentage(true_x as i32, 40) - cpu::cpu_get_text_width((percentage(true_x as i32,40) - 50.0) as u16,cpu::cpu_core_num_info()) as f32 - 20.0) <= 20.0 {
                //     v = 0.0;
                // } else {
                //     v = percentage(true_x as i32, 40) - cpu::cpu_get_text_width((percentage(true_x as i32,40) - 50.0) as u16,cpu::cpu_core_num_info()) as f32 - 20.0;
                //     v = v - 20.0;
                //     v = v / 2.0;
                // }
                x = x + v;
                x as i32 + 2
            }),
            Some(&{
                let y = 2;
                y as i32 + 1
            }),
            Some(models::LabelType::Text),
            Some(models::LabelStyle::Text)
        ));
        if show_clock {
            gui::add_label_to_window(&mut window_label_hardware, gui::create_label(
                &cpu::clock(clock_time),
                Some(&{
                    let x = percentage(true_x as i32,60) + 1.0;
                    x as i32 + 1
                }),
                Some(&{
                    let y = percentage(true_y as i32,5);
                    y as i32 + 1
                }),
                Some(models::LabelType::Text),
                Some(models::LabelStyle::Text)
            ));
        }
        if clock_time+1 >= 9 {
            clock_time = 1;
        } else {
            clock_time += 1;
        }
        gui::print_gui(&window_label_hardware,*terminal_x,*terminal_y);
    }
    Ok(())
}

fn hour_weather_menu(window_map: &Vec<Vec<String>>,terminal_x: &mut u16, terminal_y: &mut u16) -> io::Result<()> {

    let mut select_hour = 0;
    let mut weather_frame = 1;
    
    let mut true_x = *terminal_x - 2;
    let mut true_y = *terminal_y - 2;

    // Clon of window_map for not touch the main window_label.
    let mut window_label_hour = window_map.clone();
    gui::put_hour_lines_map(&mut window_label_hour,true_x,true_y);
    
    let mut vec_label_hour = vec![
        gui::create_label(
            &String::from("Leave 'Enter'"), 
            Some(&{
                let x = percentage(true_x as i32,60) + 5.0;
                x as i32 + 1
            }),
            Some(&{
                let y = true_y as f32 - 3.0;
                y as i32 + 1
            }),
            Some(models::LabelType::Select), 
            Some(models::LabelStyle::Edges)),
        gui::create_label(
            &format!("Select: {}",select_hour), 
            Some(&{
                let x = percentage(true_x as i32,60) + 5.0;
                x as i32 + 1
            }),
            Some(&{
                let y = true_y as f32 - 50.0;
                y as i32 + 1
            }),
            Some(models::LabelType::Select), 
            Some(models::LabelStyle::Edges)),
        gui::create_label(
            &String::from("Leave 'Enter'"), 
            Some(&{
                let x = percentage(true_x as i32,60) + 5.0;
                x as i32 + 1
            }),
            Some(&{
                let y = true_y as f32 - 30.0;
                y as i32 + 1
            }),
            Some(models::LabelType::Select), 
            Some(models::LabelStyle::Edges))
    ];

    // the letters in size 1 are 6 pixels in width.
    
    let mut center_hour = {
        let x = (percentage(true_x as i32,60) as i32 / 2) - 20;
        x
    };

    let ls = 1;

    // This is a closure, works like a function but you dont need to put arguments in, so i will
    // used it to not repeat the add label thing.
    //
    // let mut closure = || {
    //
    // Fun fact: didnt work, because window_label_hour is borrowed in mut so later i cant use the
    // closure :(

    // HOUR

    for (i,c) in time_weather::get_hour().chars().enumerate() {
        let num: u16 = c.to_string().parse::<u16>().unwrap();
        gui::add_label_to_window(&mut window_label_hour, gui::create_label(
            &time_weather::numbers_gui(num,ls as u16),
            Some(&(center_hour + i as i32*(ls * 7))),
            Some(&5),
            Some(models::LabelType::Text),
            Some(models::LabelStyle::Text)
            )
        );
    }

    for (i,c) in time_weather::get_minute().chars().enumerate() {
        let num: u16 = c.to_string().parse::<u16>().unwrap();
        gui::add_label_to_window(&mut window_label_hour, gui::create_label(
            &time_weather::numbers_gui(num,ls as u16),
            Some(&(center_hour + 5 + i as i32*(ls * 7) + 2 * (7 * ls))),
            Some(&5),
            Some(models::LabelType::Text),
            Some(models::LabelStyle::Text)
            )
        );
    }

    for (i,c) in time_weather::get_second().chars().enumerate() {
        let num: u16 = c.to_string().parse::<u16>().unwrap();
        gui::add_label_to_window(&mut window_label_hour, gui::create_label(
            &time_weather::numbers_gui(num,ls as u16),
            Some(&(center_hour+ 10 + i as i32*(ls * 7)+ 4 * (7 * ls))),
            Some(&5),
            Some(models::LabelType::Text),
            Some(models::LabelStyle::Text)
            )
        );
    }

    //  Wheather

    gui::add_label_to_window(&mut window_label_hour, gui::create_label(
        &time_weather::get_weather(1,weather_frame),
        Some(&5),
        Some(&(percentage(true_y as i32, 50) as i32)),
        Some(models::LabelType::Text),
        Some(models::LabelStyle::Text)
        )
    );

    let vec_label_hour_select = gui::define_select_labels(&vec_label_hour);
    window_label_hour = gui::label_window(&window_label_hour,select_hour, &vec_label_hour,&vec_label_hour_select);
    gui::print_gui(&window_label_hour,*terminal_x,*terminal_y);

    loop {
        if crossterm::event::poll(std::time::Duration::from_millis(1000))? {
            match crossterm::event::read()? {
                crossterm::event::Event::Key(key) => {
                    match key.code {
                        crossterm::event::KeyCode::Char('q') => break,
                        crossterm::event::KeyCode::Char('w') | crossterm::event::KeyCode::Up => {
                            select_hour += 1;
                            window_label_hour = gui::label_window(&window_label_hour,select_hour, &vec_label_hour,&vec_label_hour_select);
                        },
                        crossterm::event::KeyCode::Char('s') | crossterm::event::KeyCode::Down => {
                            select_hour -= 1;
                            window_label_hour = gui::label_window(&window_label_hour,select_hour, &vec_label_hour,&vec_label_hour_select);
                        },
                        crossterm::event::KeyCode::Enter => {
                            break
                        },
                        _ => {},
                    }
                },
                crossterm::event::Event::Resize(width,height) => {
                    *terminal_x = width;
                    *terminal_y = height;
                    
                    true_x = *terminal_x - 2;
                    true_y = *terminal_y - 2;

                    center_hour = {
                        let x = (percentage(true_x as i32,60) as i32 / 2) - 20;
                        x
                    };
                    
                    // THIS WILL ALL HAPPEN AGAIN. because i dont wanna do a function just for
                    // repeat 2 times.
                    
                    // Clon of window_map for not touch the main window_label.
                    window_label_hour = gui::map_window(*terminal_x,*terminal_y);
                    gui::put_hour_lines_map(&mut window_label_hour,true_x,true_y);
                    // here a big new i put let mut, why? because is borrowed and have another size,
                    // so i put this shiet, is made to any case dont explode but it will be horror
                    // for scaling and i dont have much emotive toughts in this project so i will do
                    // some of another fixes and we will see if i continue :)
                    let mut vec_label_hour = vec![
                        gui::create_label(
                            &String::from("Leave 'Enter'"), 
                            Some(&{
                                let x = percentage(true_x as i32,60) + 5.0;
                                x as i32 + 1
                            }),
                            Some(&{
                                let y = true_y as f32 - 3.0;
                                y as i32 + 1
                            }),
                            Some(models::LabelType::Select), 
                            Some(models::LabelStyle::Edges)),
                        gui::create_label(
                            &format!("Select: {}",select_hour), 
                            Some(&{
                                let x = percentage(true_x as i32,60) + 5.0;
                                x as i32 + 1
                            }),
                            Some(&{
                                let y = true_y as f32 - 50.0;
                                y as i32 + 1
                            }),
                            Some(models::LabelType::Select), 
                            Some(models::LabelStyle::Edges)),
                        gui::create_label(
                            &String::from("Leave 'Enter'"), 
                            Some(&{
                                let x = percentage(true_x as i32,60) + 5.0;
                                x as i32 + 1
                            }),
                            Some(&{
                                let y = true_y as f32 - 30.0;
                                y as i32 + 1
                            }),
                            Some(models::LabelType::Select), 
                            Some(models::LabelStyle::Edges))
                    ];

                    let vec_label_hour_select = gui::define_select_labels(&vec_label_hour);
                    window_label_hour = gui::label_window(&window_label_hour,select_hour, &vec_label_hour,&vec_label_hour_select);
                }
                _ => {},
            }
        }

        if weather_frame+1 >= 6 {
            weather_frame = 1;
        } else {
            weather_frame += 1;
        }

        for (i,c) in time_weather::get_hour().chars().enumerate() {
            let num: u16 = c.to_string().parse::<u16>().unwrap();
            gui::add_label_to_window(&mut window_label_hour, gui::create_label(
                &time_weather::numbers_gui(num,ls as u16),
                Some(&(center_hour + i as i32*(ls * 7))),
                Some(&5),
                Some(models::LabelType::Text),
                Some(models::LabelStyle::Text)
            ));
        }
    
        for (i,c) in time_weather::get_minute().chars().enumerate() {
            let num: u16 = c.to_string().parse::<u16>().unwrap();
            gui::add_label_to_window(&mut window_label_hour, gui::create_label(
                &time_weather::numbers_gui(num,ls as u16),
                Some(&(center_hour + 5 + i as i32*(ls * 7) + 2 * (7 * ls))),
                Some(&5),
                Some(models::LabelType::Text),
                Some(models::LabelStyle::Text)
            ));
        }
    
        for (i,c) in time_weather::get_second().chars().enumerate() {
            let num: u16 = c.to_string().parse::<u16>().unwrap();
            gui::add_label_to_window(&mut window_label_hour, gui::create_label(
                &time_weather::numbers_gui(num,ls as u16),
                Some(&(center_hour + 10 + i as i32*(ls * 7)+ 4 * (7 * ls))),
                Some(&5),
                Some(models::LabelType::Text),
                Some(models::LabelStyle::Text)
            ));
        }
        gui::add_label_to_window(&mut window_label_hour, gui::create_label(
            &time_weather::get_weather(1,weather_frame),
            Some(&5),
            Some(&(percentage(true_y as i32, 50) as i32)),
            Some(models::LabelType::Text),
            Some(models::LabelStyle::Text)
            )
        );
        gui::print_gui(&window_label_hour,*terminal_x,*terminal_y);
    }
    
    Ok(())
}

fn percentage (number: i32, percent: i32) -> f32 {
    if  number % 2 == 0 {
        (number as f32 / 100.0) * percent as f32
    } else {
        ((number + 1) as f32 / 100.0) * percent as f32
    }
}
