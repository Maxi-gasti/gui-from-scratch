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

