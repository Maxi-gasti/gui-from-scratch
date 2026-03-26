pub mod models;
pub mod cpu;
pub mod gui;
use crossterm::terminal::size;
use std::io;
// use std::fs;

const BLACK: &str = "\x1b[90m";
const GREEN: &str = "\x1b[32m";
const YELLOW2: &str = "\x1b[4;1;33m";
const YELLOW: &str = "\x1b[1;33m";
const RESET: &str = "\x1b[0m";

fn main() -> io::Result<()> {

    let _ = crossterm::terminal::enable_raw_mode();

    let mut select: i16 = 0;
    let mut menu_location: &str = "menu";
    

    // Esto se cuenta en lineas no en pixeles.
    let (mut terminal_x, mut terminal_y) = size()?;
    
    let mut vec_labels = asign_labels(vec![format!("HARDWARE CHECK"),format!("HOUR"),format!("WEATHER"),format!("CONFIG"),"LEAVE".to_string()],terminal_x as i32,terminal_y as i32);
    let mut select_labels = define_select_labels(&vec_labels);
    
    let window_map: Vec<Vec<String>> = map_window(terminal_x,terminal_y);

    // This is trying to be a buffer for printing GUI, i cant use all time calculations because i
    // cant depend 100% on the cpu, so, i will use this like a buffer, this is my first time using
    // it with something like 1920x1020 lol.
    let mut window_label: Vec<Vec<String>> = window_map.clone();
    window_label = label_window(&window_map,select,&vec_labels,&select_labels,terminal_x,terminal_y);
    gui::print_gui(&window_label,terminal_x,terminal_y);

    // label_window(&window_map,select,&vec_labels,&select_labels,terminal_x,terminal_y);
    
    loop {

        match crossterm::event::read()? {
            crossterm::event::Event::Resize(width,height) => {
                terminal_x = width;
                terminal_y = height;
                let window_map = map_window(width,height);
                vec_labels = reset_labels(vec_labels,width as i32, height as i32);
                select_labels = define_select_labels(&vec_labels);
                // 
                window_label = label_window(&window_map,select,&vec_labels,&select_labels,terminal_x,terminal_y);
                gui::print_gui(&window_label,terminal_x,terminal_y);
            },
            crossterm::event::Event::Key(crossterm::event::KeyEvent {code, ..} ) => { // DESEMPAQUETA EL STRUCT Keyevent y sacas el code que seria un keycode
                match code {
                    crossterm::event::KeyCode::Char('q') => break,
                    crossterm::event::KeyCode::Char('w') | crossterm::event::KeyCode::Up => {
                        if (select-1) >= 0 as i16 {select -= 1}
                        // 
                        window_label = label_window(&window_map,select,&vec_labels,&select_labels,terminal_x,terminal_y);
                        gui::print_gui(&window_label,terminal_x,terminal_y);
                    },
                    crossterm::event::KeyCode::Char('s') | crossterm::event::KeyCode::Down => {
                        if (select+1) <= (select_labels.iter().len() as i16 - 1) { select += 1}
                        // 
                        window_label = label_window(&window_map,select,&vec_labels,&select_labels,terminal_x,terminal_y);
                        gui::print_gui(&window_label,terminal_x,terminal_y);
                    },

                    // aca las views de los labels

                    crossterm::event::KeyCode::Enter => {
                        match menu_location {
                            "menu" => match select {
                                    0 => {

                                        // ACA UN MENU GUI!
                                        
                                        hardware_menu(&window_map,terminal_x,terminal_y);
                                        gui::print_gui(&window_label,terminal_x,terminal_y);

                                    },
                                    1 => {},
                                    2 => {
                                        select = 0;
                                        menu_location = "styles_menu";
                                        vec_labels = vec![
                                            create_label(&String::from("TEXT"), Some(&40), Some(&4), Some(models::LabelType::Line),Some(models::LabelStyle::Text)),
                                            create_label(&String::from("BORDER"), Some(&40), Some(&10), Some(models::LabelType::Line), Some(models::LabelStyle::Border)),
                                            create_label(&String::from("DOBLE_BORDER"), Some(&70), Some(&4), Some(models::LabelType::Line),Some(models::LabelStyle::DobleBorder)),
                                            create_label(&String::from("BOTTOM_BORDER"), Some(&70), Some(&10), Some(models::LabelType::Line),Some(models::LabelStyle::BottomBorder)),
                                            create_label(&String::from("EDGES"), Some(&100), Some(&4), Some(models::LabelType::Line),Some(models::LabelStyle::Edges)),
                                            create_label(&String::from("Return to menu"), Some(&10),Some(&(terminal_y as i32 -10)),Some(models::LabelType::Select),Some(models::LabelStyle::BottomBorder))
                                        ];
                                        select_labels = define_select_labels(&vec_labels);
                                        window_label = label_window(&window_map,select,&vec_labels,&select_labels,terminal_x,terminal_y);
                                        //
                                        gui::print_gui(&window_label,terminal_x,terminal_y);
                                    },
                                    3 => {
                                        select = 0;
                                        menu_location = "config";
                                        // vec_labels = vec![
                                        //     create_label(&String::from(""),),
                                        // ]
                                        vec_labels = asign_labels(vec![format!("CONFIG"),format!("COLOR"),"nose".to_string(),"dosdos".to_string()],terminal_x as i32,terminal_y as i32);
                                        select_labels = define_select_labels(&vec_labels);
                                        // 
                                        window_label = label_window(&window_map,select,&vec_labels,&select_labels,terminal_x,terminal_y);
                                        gui::print_gui(&window_label,terminal_x,terminal_y);
                                    },
                                    4 => { gui::clear_terminal(); break},
                                    _ => {},
                                },


                            "styles_menu" => match select {
                                    0 => {
                                        select = 0;
                                        vec_labels = asign_labels(vec![format!("HARDWARE CHECK"),format!("HOUR"),format!("WEATHER"),format!("CONFIG"),"LEAVE".to_string()],terminal_x as i32,terminal_y as i32);
                                        select_labels = define_select_labels(&vec_labels);
                                        menu_location = "menu";
                                        // 
                                        window_label = label_window(&window_map,select,&vec_labels,&select_labels,terminal_x,terminal_y);
                                        gui::print_gui(&window_label,terminal_x,terminal_y);
                                    },
                                    _ => {},

                            }
                            "hardware_menu" => match select {
                                    0 => {
                                        select = 0;
                                        vec_labels = asign_labels(vec![format!("HARDWARE CHECK"),format!("HOUR"),format!("WEATHER"),format!("CONFIG"),"LEAVE".to_string()],terminal_x as i32,terminal_y as i32);
                                        select_labels = define_select_labels(&vec_labels);
                                        menu_location = "menu";
                                        // 
                                        window_label = label_window(&window_map,select,&vec_labels,&select_labels,terminal_x,terminal_y);
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
                                        vec_labels = asign_labels(vec![format!("HARDWARE CHECK"),format!("HOUR"),format!("WEATHER"),format!("CONFIG"),"LEAVE".to_string()],terminal_x as i32,terminal_y as i32);
                                        select_labels = define_select_labels(&vec_labels);
                                        menu_location = "menu";
                                        // 
                                        window_label = label_window(&window_map,select,&vec_labels,&select_labels,terminal_x,terminal_y);
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
    Ok(())
}

fn hardware_menu(window_map: &Vec<Vec<String>>, terminal_x: u16,terminal_y: u16) -> io::Result<()> {
    
    // Clon of window_map for not touch the main window_label.
    let mut window_label_hardware = window_map.clone();

    let select_hardware = 0;

    let true_x = terminal_x - 2;
    let true_y = terminal_y - 2;

    let vec_label_hardware = vec![
        create_label(
            &String::from("Hardware Check"), 
            Some(&{
                let x = (true_x as f32 / 100.0) as f32 * 3.0;
                x as i32 + 1
            }),
            Some(&{
                let y = (true_y as f32 / 100.0) as f32 * 5.0;
                y as i32 + 1
            }),
            Some(models::LabelType::Line), 
            Some(models::LabelStyle::BottomBorder)),
        create_label(
            &String::from("Leave"), 
            Some(&{
                let x = (true_x as f32 / 100.0) as f32 * 3.0;
                x as i32 + 1
            }),
            Some(&{
                let y = (true_y as f32 / 100.0) as f32 * 94.0;
                y as i32 + 1
            }),
            Some(models::LabelType::Select), 
            Some(models::LabelStyle::Edges))
    ];


    let vec_label_hardware_select = define_select_labels(&vec_label_hardware);

    window_label_hardware = label_window(&window_label_hardware,select_hardware, &vec_label_hardware,&vec_label_hardware_select,terminal_x,terminal_y);


    let mut clock_time: u16 = 1;  

    add_label_to_window(&mut window_label_hardware, create_label(
        &cpu::cpu_info(true_y / 5),
        Some(&{
            let x = (true_x as f32 / 100.0) as f32 * 70.0;
            x as i32 + 1
        }),
        Some(&{
            let y = (true_y as f32 / 100.0) as f32 * 5.0;
            y as i32 + 1
        }),
        Some(models::LabelType::Text),
        Some(models::LabelStyle::Text)
    ));
    add_label_to_window(&mut window_label_hardware, create_label(
        &cpu::clock(clock_time),
        Some(&{
            let x = (true_x as f32 / 100.0) as f32 * 55.0;
            x as i32 + 1
            }),
            Some(&{
                let y = (true_y as f32 / 100.0) as f32 * 5.0;
                y as i32 + 1
            }),
            Some(models::LabelType::Text),
            Some(models::LabelStyle::Text)
    ));
    gui::print_gui(&window_label_hardware,terminal_x,terminal_y);
    if clock_time+1 >= 9 {
        clock_time = 1;
    } else {
        clock_time += 1;
    }

    loop {
        if crossterm::event::poll(std::time::Duration::from_millis(750))? {
            if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                match key.code {
                    crossterm::event::KeyCode::Char('q') => break,
                    crossterm::event::KeyCode::Enter => {
                        break
                    },
                    _ => {},
                }
            }
        }

        add_label_to_window(&mut window_label_hardware, create_label(
               &cpu::cpu_info(true_y / 5),
                Some(&{
                    let x = (true_x as f32 / 100.0) as f32 * 70.0;
                    x as i32 + 1
                }),
                Some(&{
                    let y = (true_y as f32 / 100.0) as f32 * 5.0;
                    y as i32 + 1
                }),
                Some(models::LabelType::Text),
                Some(models::LabelStyle::Text)
        ));
        add_label_to_window(&mut window_label_hardware, create_label(
               &cpu::clock(clock_time),
                Some(&{
                    let x = (true_x as f32 / 100.0) as f32 * 55.0;
                    x as i32 + 1
                }),
                Some(&{
                    let y = (true_y as f32 / 100.0) as f32 * 5.0;
                    y as i32 + 1
                }),
                Some(models::LabelType::Text),
                Some(models::LabelStyle::Text)
        ));
        if clock_time+1 >= 9 {
            clock_time = 1;
        } else {
            clock_time += 1;
        }
        gui::print_gui(&window_label_hardware,terminal_x,terminal_y);
    }
    Ok(())
}


// |--------------- WINDOW ---------------|

fn map_window(terminal_x: u16, terminal_y: u16) -> Vec<Vec<String>> {

    // Just draw the window in a vector[Y][X].

    let mut window_y_temp = Vec::new();

    let mut temp_count = 0;

    while temp_count < terminal_y {
        window_y_temp.push({

            let mut temp_count2 = 0;
            let mut window_x_temp = Vec::new();

            while temp_count2 < terminal_x {

                if temp_count == 0 { 
                    if temp_count2 == 0{window_x_temp.push(format!("{}╔{}",BLACK,RESET))}
                    else if temp_count2 == (terminal_x - 1) { window_x_temp.push(format!("{}╗{}",BLACK,RESET))}
                    else { window_x_temp.push(format!("{}═{}",BLACK,RESET)) }
                } 
                else if temp_count == (terminal_y - 1) { 
                    if temp_count2 == 0 {window_x_temp.push(format!("{}╚{}",BLACK,RESET))}
                    else if temp_count2 == (terminal_x - 1) { window_x_temp.push(format!("{}╝{}",BLACK,RESET))}
                    else { window_x_temp.push(format!("{}═{}",BLACK,RESET)) }
                }
                else if temp_count2 == 0 || temp_count2 == (terminal_x - 1) { window_x_temp.push(format!("{}║{}",BLACK,RESET)) }
                else { window_x_temp.push(format!(" ")) };
                
                temp_count2 += 1;
            }
            window_x_temp
        });
        temp_count += 1;
    }
    window_y_temp
}

fn define_select_labels (vec_label: &Vec<models::Label>) -> Vec<&models::Label> {
    // This returns a vec with the references of who is LabelType::Select, for 
    // menu interface.
    
    let mut labels_temp = Vec::new();
    for label_ in vec_label.iter() {
        if label_.label_type == models::LabelType::Select { labels_temp.push(label_); }
    };
    labels_temp
}

fn add_label_to_window(window_label: &mut Vec<Vec<String>>,label: models::Label) {

    // Esta funcion debe de añadir un label al window_label cuando es llamado,
    // debe respetar las funciones que tiene label_window pero de forma individual
    //
    // A DESTACAR: Es para poner cosas de informacion, ya que no respeta el sistema
    // de vectores que utiliza los labels comunes, asi que no se debe usar con select
    // !!!

    let text_size: u16 = {
        if label.label_type == models::LabelType::Line {
            label.text.len() as u16
        } else {
            let mut count = 0;
            let mut letters: u16 = 0;
            let bytes = label.text.as_bytes();

           for (_i, &item) in bytes.iter().enumerate() {
                count += 1;
                if item != b'\n' {
                    if count > letters {
                        letters = count;
                    }
                } else {
                    count = 0;
                }
            };
           letters
        }
    };
    let impar = if text_size % 2 == 0 { 0 } else { 1 };
    let text_size = if text_size != 1 { if (text_size % 2) == 0 { text_size / 2 } else { (text_size + 1) / 2 } } else { 1 };
    let color = "";
    let color2 = "";
    
    match label.label_type {
        models::LabelType::Text => {
            
            let mut count_y: u16 = 0;
            let mut count_x: i32 = 0;

            for (_i, c) in label.text.chars().enumerate() {
                if c == '\n' {
                    count_y += 1;
                    count_x = -1;
                } else {
                    window_label[(label.pos_y + count_y) as usize][(label.pos_x + count_x as u16) as usize] = c.to_string();
                }
                count_x += 1;
            }

        },
        models::LabelType::Line | models::LabelType::Select => {
            match label.style {
                models::LabelStyle::Border => {
                    for i in 0..(label.text.len() + 6) {
                        if i == 0 {
                            window_label[(label.pos_y + 2) as usize][(label.pos_x - 3) as usize] = format!("{}└{}",color,RESET);
                        } else if i == (label.text.len() + 5) {
                            window_label[(label.pos_y + 2) as usize][(label.pos_x - 3 + i as u16) as usize] = format!("{}┘{}",color,RESET);
                        } else {
                            window_label[(label.pos_y + 2) as usize][(label.pos_x - 3 + i as u16) as usize] = format!("{}─{}",color,RESET);
                        }
                    }
                    for i in 0..(label.text.len() + 6) {
                        if i == 0 {
                            window_label[(label.pos_y - 2) as usize][(label.pos_x - 3 + i as u16) as usize] = format!("{}┌{}",color,RESET);
                        } else if i == (label.text.len() + 5) {
                            window_label[(label.pos_y - 2) as usize][(label.pos_x - 3 + i as u16) as usize] = format!("{}┐{}",color,RESET);
                        } else {
                            window_label[(label.pos_y - 2) as usize][(label.pos_x - 3 + i as u16) as usize] = format!("{}─{}",color,RESET);
                        }
                    }
                    for i in 0..3 {
                        window_label[(label.pos_y - 1 + i as u16) as usize][(label.pos_x - 3) as usize] = format!("{}│{}",color,RESET);
                    }
                    for i in 0..3 {
                        window_label[(label.pos_y - 1 + i as u16) as usize][(label.pos_x + (text_size as u16 * 2) + 2 - impar) as usize] = format!("{}│{}",color,RESET);
                    }
                },
                models::LabelStyle::DobleBorder  => {
                    for i in 0..(label.text.len() + 6) {
                        if i == 0 {
                            window_label[(label.pos_y + 2) as usize][(label.pos_x - 3 + i as u16) as usize] = format!("{}╚{}",color,RESET);
                        } else if i == (label.text.len() + 5) {
                            window_label[(label.pos_y + 2) as usize][(label.pos_x - 3 + i as u16) as usize] = format!("{}╝{}",color,RESET);
                        } else {
                            window_label[(label.pos_y + 2) as usize][(label.pos_x - 3 + i as u16) as usize] = format!("{}═{}",color,RESET);
                        }
                    }
                    for i in 0..(label.text.len() + 6) {
                        if i == 0 {
                            window_label[(label.pos_y - 2) as usize][(label.pos_x - 3 + i as u16) as usize] = format!("{}╔{}",color,RESET);
                        } else if i == (label.text.len() + 5) {
                            window_label[(label.pos_y - 2) as usize][(label.pos_x - 3 + i as u16) as usize] = format!("{}╗{}",color,RESET);
                        } else {
                            window_label[(label.pos_y - 2) as usize][(label.pos_x - 3 + i as u16) as usize] = format!("{}═{}",color,RESET);
                        }
                    }
                    for i in 0..3 {
                        window_label[(label.pos_y - 1 + i as u16) as usize][(label.pos_x - 3) as usize] = format!("{}║{}",color,RESET);
                    }
                    for i in 0..3 {
                        window_label[(label.pos_y - 1 + i as u16) as usize][(label.pos_x + (text_size as u16 * 2) + 2 - impar) as usize] = format!("{}║{}",color,RESET);
                    }
                },
                models::LabelStyle::BottomBorder => {
                    for i in 0..(label.text.len() + 2) {
                        if i == 0 {
                            window_label[(label.pos_y + 1) as usize][(label.pos_x - 1 + i as u16) as usize] = format!("{}←{}",color,RESET);
                        } else if i == (label.text.len() + 1) {
                            window_label[(label.pos_y + 1) as usize][(label.pos_x - 1 + i as u16) as usize] = format!("{}→{}",color,RESET);
                        } else {
                            window_label[(label.pos_y + 1) as usize][(label.pos_x - 1 + i as u16) as usize] = format!("{}─{}",color,RESET);
                        }
                    }
                },
                models::LabelStyle::Edges => {
                    window_label[(label.pos_y - 1) as usize][(label.pos_x - 2) as usize] = format!("{}┌{}",color,RESET);
                    window_label[(label.pos_y - 1) as usize][(label.pos_x + (text_size as u16 * 2)) as usize] = format!("{}┐{}",color,RESET);
                    window_label[(label.pos_y + 1) as usize][(label.pos_x - 2) as usize] = format!("{}└{}",color,RESET);
                    window_label[(label.pos_y + 1) as usize][(label.pos_x + (text_size as u16 * 2)) as usize] = format!("{}┘{}",color,RESET);
                },
                models::LabelStyle::Text => {}
            }
            for (i, c) in label.text.chars().enumerate() {
                window_label[label.pos_y as usize][label.pos_x as usize + i as usize] = c.to_string();
            }
        },
        _ => { print!("ERROR: NINGUN LABELTYPE ENCONTRADO.");}
    }
}

fn label_window(window_map: &Vec<Vec<String>>,select: i16,vec_labels: &Vec<models::Label>,select_labels: &Vec<&models::Label>,terminal_x: u16,terminal_y: u16) -> Vec<Vec<String>> {


    // ESTA FUNCION SOLO SIRVE PARA CUANDO CAMBIAN LOS LABELS, SOLO SE USA CUANDO SE ACTUALIZA LA
    // LISTA DE VEC_LABELS.
    //
    // Es decir, el propocito de esta fn es imprimir el menu, no para proporcionar informacion, ya
    // que no tiene un buffer como el de add_label_to_window() lo que lo hace perfecto para 
    // enseñar nuevos labels, ya que reinicia completamente el mapeo pero el buffer no.

    // ACA SE ASIGNA al window_label el cual se imprime al final, donde las posiciones de los labels se escriben
    // sobre el mapa original.
    //
    // Se debe aclarar que aca tambien se customiza los labels con esquinas si el tipo de label lo requiere.
    
    let mut window_label = window_map.clone();
    
    window_label[(terminal_y-2) as usize][2 as usize] = format!("{}",select);
    window_label[(terminal_y-2) as usize][4 as usize] = String::from("|");
    window_label[(terminal_y-2) as usize][6 as usize] = String::from("L");
    window_label[(terminal_y-2) as usize][7 as usize] = String::from("e");
    window_label[(terminal_y-2) as usize][8 as usize] = String::from("a");
    window_label[(terminal_y-2) as usize][9 as usize] = String::from("v");
    window_label[(terminal_y-2) as usize][10 as usize] = String::from("e");
    window_label[(terminal_y-2) as usize][11 as usize] = String::from(":");
    window_label[(terminal_y-2) as usize][13 as usize] = String::from("Q");

    for (index, label) in vec_labels.iter().enumerate() {
        let text_size = label.text.len();
        let impar = if text_size % 2 == 0 { 0 } else { 1 };
        let text_size = if text_size != 1 { if (text_size % 2) == 0 { text_size / 2 } else { (text_size + 1) / 2 } } else { 1 };
        let mut color = "";
        let mut color2 = "";
        

        if label.label_type == models::LabelType::Select {
            for (i, label_) in select_labels.iter().enumerate() {
                if select == i as i16 && std::ptr::eq(label,*label_) { color = &YELLOW; color2 = &YELLOW2; break }
            }
        }

        // Customize the label.

        match label.style {
            models::LabelStyle::Border => {
                for i in 0..(label.text.len() + 6) {
                    if i == 0 {
                        window_label[(label.pos_y + 2) as usize][(label.pos_x - 3 + i as u16) as usize] = format!("{}└{}",color,RESET);
                    } else if i == (label.text.len() + 5) {
                        window_label[(label.pos_y + 2) as usize][(label.pos_x - 3 + i as u16) as usize] = format!("{}┘{}",color,RESET);
                    } else {
                        window_label[(label.pos_y + 2) as usize][(label.pos_x - 3 + i as u16) as usize] = format!("{}─{}",color,RESET);
                    }
                }
                for i in 0..(label.text.len() + 6) {
                    if i == 0 {
                        window_label[(label.pos_y - 2) as usize][(label.pos_x - 3 + i as u16) as usize] = format!("{}┌{}",color,RESET);
                    } else if i == (label.text.len() + 5) {
                        window_label[(label.pos_y - 2) as usize][(label.pos_x - 3 + i as u16) as usize] = format!("{}┐{}",color,RESET);
                    } else {
                        window_label[(label.pos_y - 2) as usize][(label.pos_x - 3 + i as u16) as usize] = format!("{}─{}",color,RESET);
                    }
                }
                for i in 0..3 {
                    window_label[(label.pos_y - 1 + i as u16) as usize][(label.pos_x - 3) as usize] = format!("{}│{}",color,RESET);
                }
                for i in 0..3 {
                    window_label[(label.pos_y - 1 + i as u16) as usize][(label.pos_x + (text_size as u16 * 2) + 2 - impar) as usize] = format!("{}│{}",color,RESET);
                }
            },
            models::LabelStyle::DobleBorder  => {
                for i in 0..(label.text.len() + 6) {
                    if i == 0 {
                        window_label[(label.pos_y + 2) as usize][(label.pos_x - 3 + i as u16) as usize] = format!("{}╚{}",color,RESET);
                    } else if i == (label.text.len() + 5) {
                        window_label[(label.pos_y + 2) as usize][(label.pos_x - 3 + i as u16) as usize] = format!("{}╝{}",color,RESET);
                    } else {
                        window_label[(label.pos_y + 2) as usize][(label.pos_x - 3 + i as u16) as usize] = format!("{}═{}",color,RESET);
                    }
                }
                for i in 0..(label.text.len() + 6) {
                    if i == 0 {
                        window_label[(label.pos_y - 2) as usize][(label.pos_x - 3 + i as u16) as usize] = format!("{}╔{}",color,RESET);
                    } else if i == (label.text.len() + 5) {
                        window_label[(label.pos_y - 2) as usize][(label.pos_x - 3 + i as u16) as usize] = format!("{}╗{}",color,RESET);
                    } else {
                        window_label[(label.pos_y - 2) as usize][(label.pos_x - 3 + i as u16) as usize] = format!("{}═{}",color,RESET);
                    }
                }
                for i in 0..3 {
                    window_label[(label.pos_y - 1 + i as u16) as usize][(label.pos_x - 3) as usize] = format!("{}║{}",color,RESET);
                }
                for i in 0..3 {
                    window_label[(label.pos_y - 1 + i as u16) as usize][(label.pos_x + (text_size as u16 * 2) + 2 - impar) as usize] = format!("{}║{}",color,RESET);
                }
            },
            models::LabelStyle::BottomBorder => {
                for i in 0..(label.text.len() + 2) {
                    if i == 0 {
                        window_label[(label.pos_y + 1) as usize][(label.pos_x - 1 + i as u16) as usize] = format!("{}←{}",color,RESET);
                    } else if i == (label.text.len() + 1) {
                        window_label[(label.pos_y + 1) as usize][(label.pos_x - 1 + i as u16) as usize] = format!("{}→{}",color,RESET);
                    } else {
                        window_label[(label.pos_y + 1) as usize][(label.pos_x - 1 + i as u16) as usize] = format!("{}─{}",color,RESET);
                    }
                }
            },
            models::LabelStyle::Edges => {
                window_label[(label.pos_y - 1) as usize][(label.pos_x - 2) as usize] = format!("{}┌{}",color,RESET);
                window_label[(label.pos_y - 1) as usize][(label.pos_x + (text_size as u16 * 2)) as usize] = format!("{}┐{}",color,RESET);
                window_label[(label.pos_y + 1) as usize][(label.pos_x - 2) as usize] = format!("{}└{}",color,RESET);
                window_label[(label.pos_y + 1) as usize][(label.pos_x + (text_size as u16 * 2)) as usize] = format!("{}┘{}",color,RESET);

            },
            models::LabelStyle::Text => {}
        }
        
        for (i, c) in label.text.chars().enumerate() {
            if select == index as i16 { window_label[label.pos_y as usize][label.pos_x as usize + i as usize] = format!("{}{}{}",color2,c,RESET); }
            else { window_label[label.pos_y as usize][label.pos_x as usize + i as usize] = c.to_string(); }
        }
    }
    
    window_label
}

// |--------------- LABELS ---------------|

fn create_label(text: &String,posi_x: Option<&i32>,posi_y: Option<&i32>,label_type: Option<models::LabelType>,label_style: Option<models::LabelStyle>) -> models::Label {

    // Crea los labels con los parametros proporcionados, tiene varios option para ser versatil
    // para que automaticamente se asignen algunos valores, como por ejemplo para el assign_labels.
    
    let mut r_type: models::LabelType = models::LabelType::Select;
    let mut r_style: models::LabelStyle = models::LabelStyle::Text;
    let mut r_x: i32 = 0;
    let mut r_y: i32 = 0;

    match label_type {
        Some(label_type) => r_type = label_type,
        None => {}
    }
    match label_style {
        Some(label_style) => r_style = label_style,
        None => {}
    }
    match posi_x {
        Some(x) => r_x = *x,
        None => {}
    }
    match posi_y {
        Some(y) => r_y = *y,
        None => {}
    }
    
    models::Label {
        text: text.to_string(),
        label_type: r_type,
        style: r_style,
        pos_x: r_x as u16,
        pos_y: r_y as u16,
    }
}

fn asign_labels (strings: Vec<String>, ter_x: i32, ter_y: i32) -> Vec<models::Label> {

    // This is for just trow here some labelz and the program read it, and assign the
    // best pos in Y and X (center).

    // Usualmente para asignar labels para el menu tipo select sin informacion entre medio

    let mut vec_label: Vec<models::Label> = Vec::new();
    let vec_size = strings.iter().len() as i32;
    let center_x = ter_x / 2;
    let center_y = (ter_y - (ter_y / 20) - 1) / vec_size;
    let mut count = 0;

    for string in strings.iter() {
        vec_label.push(create_label(string,Some(&(center_x -( string.len() as i32 / 2))),Some(&(center_y * count + (ter_y / 20) + 1)),Some(models::LabelType::Select),Some(models::LabelStyle::Border)));
        count += 1;
    }
    vec_label
}

fn reset_labels (vec_labels: Vec<models::Label>, ter_x: i32, ter_y: i32) -> Vec<models::Label> {

    // This is used regulary when the window width and height changes, the labels for that need
    // to resize his pos_x and pos_y.

    let mut string_vec: Vec<String> = Vec::new();
    for label in vec_labels.iter() {
        string_vec.push(label.text.clone());
    }
    asign_labels(string_vec, ter_x, ter_y)
}
