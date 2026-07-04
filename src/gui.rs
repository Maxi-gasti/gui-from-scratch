use crate::models; // to use the models.rs from the crate and not create another module.

const BLACK: &str = "\x1b[90m";
const GREEN: &str = "\x1b[32m";
const YELLOW2: &str = "\x1b[4;1;33m";
const YELLOW: &str = "\x1b[1;33m";
const RESET: &str = "\x1b[0m";

pub fn clear_terminal(){ print!("\x1b[2J\x1b[H"); }

pub fn print_gui(window_label: &Vec<Vec<String>>, terminal_x: u16, terminal_y: u16) {

    let text = {
        let mut temp_count = 0;
        let mut text_temp = String::new();

        while temp_count < terminal_y {
            let mut temp_iter = window_label[temp_count as usize].iter();
            let mut temp_2_count = 0;
            let mut line = String::new();
            while temp_2_count < terminal_x {
                if let Some(value) = temp_iter.next() {  line = line + value; }
                temp_2_count += 1;
            }
            text_temp = text_temp + &line;
            temp_count += 1;
        }
        text_temp
    };
    clear_terminal();
    print!("{}",text)
}

// WINDOWS AND LABELS

pub fn map_window(terminal_x: u16, terminal_y: u16) -> Vec<Vec<String>> {

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

// |--------------- WINDOW ---------------|

pub fn add_label_to_window(window_label: &mut Vec<Vec<String>>,label: models::Label) {

    // This function will add a label to window_label when is called,
    // needs to respect the functions that have label_window but in individual form
    //
    // To be clare: This is for put thing about information because not follow the rules 
    // of vector sistem that use the common labels, so dont use select here.

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
    }
}


pub fn label_window(window_map: &Vec<Vec<String>>,select: i16,vec_labels: &Vec<models::Label>,select_labels: &Vec<&models::Label>,terminal_x: u16,terminal_y: u16) -> Vec<Vec<String>> {


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
    
    // window_label[(terminal_y-2) as usize][2 as usize] = format!("{}",select);
    // window_label[(terminal_y-2) as usize][4 as usize] = String::from("|");
    // window_label[(terminal_y-2) as usize][6 as usize] = String::from("L");
    // window_label[(terminal_y-2) as usize][7 as usize] = String::from("e");
    // window_label[(terminal_y-2) as usize][8 as usize] = String::from("a");
    // window_label[(terminal_y-2) as usize][9 as usize] = String::from("v");
    // window_label[(terminal_y-2) as usize][10 as usize] = String::from("e");
    // window_label[(terminal_y-2) as usize][11 as usize] = String::from(":");
    // window_label[(terminal_y-2) as usize][13 as usize] = String::from("Q");

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

pub fn create_label(text: &String,posi_x: Option<&i32>,posi_y: Option<&i32>,label_type: Option<models::LabelType>,label_style: Option<models::LabelStyle>) -> models::Label {

    // Crea los labels con los parametros proporcionados, tiene varios option para ser versatil
    // para que automaticamente se asignen algunos valores, como por ejemplo para el assign_labels.
    //
    // the r means result.
    
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

pub fn define_select_labels (vec_label: &Vec<models::Label>) -> Vec<&models::Label> {
    // This returns a vec with the references of who is LabelType::Select, for 
    // menu interface.
    
    let mut labels_temp = Vec::new();
    for label_ in vec_label.iter() {
        if label_.label_type == models::LabelType::Select { labels_temp.push(label_); }
    };
    labels_temp
}


pub fn asign_labels (strings: Vec<String>, ter_x: i32, ter_y: i32) -> Vec<models::Label> {

    // This is for just trow here some labels and the program read it, and assign the
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

pub fn reset_labels (vec_labels: Vec<models::Label>, ter_x: i32, ter_y: i32) -> Vec<models::Label> {

    // This is used regulary when the window width and height changes, the labels for that need
    // to resize his pos_x and pos_y.

    let mut string_vec: Vec<String> = Vec::new();
    for label in vec_labels.iter() {
        string_vec.push(label.text.clone());
    }
    asign_labels(string_vec, ter_x, ter_y)
}

// |----- Put lines -----|

pub fn put_hardware_lines_map (window_map: &mut Vec<Vec<String>>, terminal_x: u16, terminal_y: u16) {
    
    // This is not more used, because percentage() remplace it.

    // let mut impar_x = 0;
    // let mut impar_y = 0;
    // if terminal_x % 2 != 0 { impar_x += 1; }
    // if terminal_y % 2 != 0 { impar_y += 1; }
    
    // X

    for i in 0..( terminal_x as i32 - percentage(terminal_x as i32, 40) as i32 - 1) {
        window_map[1][( i + 1 ) as usize] = String::from("─");
    }
    for i in 0..( terminal_x as i32 - percentage(terminal_x as i32, 60) as i32 - 1) {
        window_map[1][(percentage(terminal_x as i32,60) as i32 + i + 1 ) as usize] = String::from("─");
    }
    for i in 0..( terminal_x as i32 - percentage(terminal_x as i32, 40) as i32 - 1) {
        window_map[percentage(terminal_y as i32,20) as usize][( i + 1 ) as usize] = String::from("─");
    }
    for i in 0..( terminal_x as i32 - percentage(terminal_x as i32, 40) as i32 - 1) {
        window_map[(percentage(terminal_y as i32,20) + 1.0) as usize][( i + 1 ) as usize] = String::from("─");
    }
    for i in 0..( terminal_x as i32 - percentage(terminal_x as i32, 40) as i32 - 1) {
        window_map[(percentage(terminal_y as i32,40)) as usize][( i + 1 ) as usize] = String::from("─");
    }
    for i in 0..( terminal_x as i32 - percentage(terminal_x as i32, 40) as i32 - 1) {
        window_map[(percentage(terminal_y as i32,40) + 1.0) as usize][( i + 1 ) as usize] = String::from("─");
    }
    for i in 0..( terminal_x as i32 - percentage(terminal_x as i32, 60) as i32 - 1) {
        window_map[percentage(terminal_y as i32, 60) as usize][(percentage(terminal_x as i32,60) as i32 + i + 1 ) as usize] = String::from("─");
    }
    for i in 0..( terminal_x as i32 - percentage(terminal_x as i32, 60) as i32 - 1) {
        window_map[(percentage(terminal_y as i32, 60) + 1.0) as usize][(percentage(terminal_x as i32,60) as i32 + i + 1 ) as usize] = String::from("─");
    }
    for i in 0..( terminal_x as i32 - percentage(terminal_x as i32, 60) as i32 - 1) {
        window_map[terminal_y as usize][(percentage(terminal_x as i32,60) as i32 + i + 1 ) as usize] = String::from("─");
    }
    for i in 0..( terminal_x as i32 - percentage(terminal_x as i32, 40) as i32 - 1) {
        window_map[terminal_y as usize][( i + 1 ) as usize] = String::from("─");
    }
    
    // Y
    
    for i in 0..(terminal_y - 2) {
        window_map[(2+i as i32) as usize][1 as usize] = String::from("│");
    }
    for i in 0..(terminal_y - 2) {
        window_map[(2+i as i32) as usize][percentage(terminal_x as i32,60) as usize] = String::from("│");
    }
    for i in 0..(terminal_y - 2) {
        window_map[(2+i as i32) as usize][(percentage(terminal_x as i32,60) - 1.0) as usize] = String::from("│");
    }
    for i in 0..(terminal_y - 2) {
        window_map[(2+i as i32) as usize][terminal_x as usize] = String::from("│");
    }
    
    window_map[1][percentage(terminal_x as i32,60) as usize] = String::from("┌");
    window_map[terminal_y as usize][percentage(terminal_x as i32,60) as usize] = String::from("└");
    
    window_map[1][terminal_x as usize] = String::from("┐");
    window_map[terminal_y as usize][terminal_x as usize] = String::from("┘");
    
    window_map[percentage(terminal_y as i32, 60) as usize][percentage(terminal_x as i32,60) as usize] = String::from("└");
    window_map[(percentage(terminal_y as i32, 60) + 1.0) as usize][percentage(terminal_x as i32,60) as usize] = String::from("┌");
    
    window_map[percentage(terminal_y as i32, 60) as usize][terminal_x as usize] = String::from("┘");
    window_map[(percentage(terminal_y as i32, 60,) + 1.0) as usize][terminal_x as usize] = String::from("┐");
    
    window_map[1][1] = String::from("┌");
    window_map[terminal_y as usize][1] = String::from("└");
    
    window_map[1][(percentage(terminal_x as i32,60)-1.0) as usize] = String::from("┐");
    window_map[terminal_y as usize][(percentage(terminal_x as i32,60)-1.0) as usize] = String::from("┘");

    window_map[percentage(terminal_y as i32,40) as usize][(percentage(terminal_x as i32,60) - 1.0) as usize] = String::from("┘");
    window_map[(percentage(terminal_y as i32,40) + 1.0) as usize][(percentage(terminal_x as i32,60) - 1.0) as usize] = String::from("┐");
    
    window_map[percentage(terminal_y as i32,40) as usize][1 as usize] = String::from("└");
    window_map[(percentage(terminal_y as i32,40) + 1.0) as usize][1 as usize] = String::from("┌");

    window_map[percentage(terminal_y as i32,20) as usize][(percentage(terminal_x as i32,60) - 1.0) as usize] = String::from("┘");
    window_map[(percentage(terminal_y as i32,20) + 1.0) as usize][(percentage(terminal_x as i32,60) - 1.0) as usize] = String::from("┐");
    
    window_map[percentage(terminal_y as i32,20) as usize][1 as usize] = String::from("└");
    window_map[(percentage(terminal_y as i32,20) + 1.0) as usize][1 as usize] = String::from("┌");

    if percentage(terminal_y as i32, 20) >= 4.0 {
        window_map[2][1] = String::from("├");
        window_map[2][2] = String::from("─");
        window_map[(2.0 + percentage(terminal_y as i32, 20)) as usize ][1] = String::from("├");
        window_map[(2.0 + percentage(terminal_y as i32, 20)) as usize ][2] = String::from("─");
    } 
    
}

pub fn put_hour_lines_map (window_map: &mut Vec<Vec<String>>, terminal_x: u16, terminal_y: u16) {
    
    // X

    for i in 0..( terminal_x as i32 - percentage(terminal_x as i32, 40) as i32 - 1) {
        window_map[1][( i + 1 ) as usize] = String::from("─");
    }
    for i in 0..( terminal_x as i32 - percentage(terminal_x as i32, 60) as i32 - 1) {
        window_map[1][(percentage(terminal_x as i32,60) as i32 + i + 1 ) as usize] = String::from("─");
    }
    for i in 0..( terminal_x as i32 - percentage(terminal_x as i32, 60) as i32 - 1) {
        window_map[terminal_y as usize][(percentage(terminal_x as i32,60) as i32 + i + 1 ) as usize] = String::from("─");
    }
    for i in 0..( terminal_x as i32 - percentage(terminal_x as i32, 40) as i32 - 1) {
        window_map[terminal_y as usize][( i + 1 ) as usize] = String::from("─");
    }
    
    // Y
    
    for i in 0..(terminal_y - 2) {
        window_map[(2+i as i32) as usize][1 as usize] = String::from("│");
    }
    for i in 0..(terminal_y - 2) {
        window_map[(2+i as i32) as usize][percentage(terminal_x as i32,60) as usize] = String::from("│");
    }
    for i in 0..(terminal_y - 2) {
        window_map[(2+i as i32) as usize][(percentage(terminal_x as i32,60) - 1.0) as usize] = String::from("│");
    }
    for i in 0..(terminal_y - 2) {
        window_map[(2+i as i32) as usize][terminal_x as usize] = String::from("│");
    }
    
    
    window_map[1][terminal_x as usize] = String::from("┐");
    window_map[terminal_y as usize][terminal_x as usize] = String::from("┘");
    
    window_map[1][1] = String::from("┌");
    window_map[terminal_y as usize][1] = String::from("└");
       
    window_map[1][(percentage(terminal_x as i32,60) - 1.0) as usize] = String::from("┐");
    window_map[1][percentage(terminal_x as i32,60) as usize] = String::from("┌");
    window_map[terminal_y as usize][(percentage(terminal_x as i32,60) - 1.0) as usize] = String::from("┘");
    window_map[terminal_y as usize][percentage(terminal_x as i32,60) as usize] = String::from("└");
}

fn percentage (number: i32, percent: i32) -> f32 {
    if  number % 2 == 0 {
        (number as f32 / 100.0) * percent as f32
    } else {
        ((number + 1) as f32 / 100.0) * percent as f32
    }
}
