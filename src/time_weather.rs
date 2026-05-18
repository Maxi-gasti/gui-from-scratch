use chrono::Local;

pub fn get_time () -> String {
    let now = Local::now();
    let text: String = now.format("%Y/%m/%d %H:%M:%S").to_string();
    text
}

pub fn get_year () -> String {
    let now = Local::now();
    let text: String = now.format("%Y").to_string();
    text
}

pub fn get_month () -> String {
    let now = Local::now();
    let text: String = now.format("%m").to_string();
    text
}

pub fn get_day () -> String {
    let now = Local::now();
    let text: String = now.format("%d").to_string();
    text
}

pub fn get_hour () -> String {
    let now = Local::now();
    let text: String = now.format("%H").to_string();
    text
}

pub fn get_minute () -> String {
    let now = Local::now();
    let text: String = now.format("%M").to_string();
    text
}

pub fn get_second () -> String {
    let now = Local::now();
    let text: String = now.format("%S").to_string();
    text
}

// type is 1-4 range, because i dont want to do a struct with winter, summer, etc.
// TYPE: 1 = winter, 2 = Otono, 3 = Primavera, 4 = Summer

pub fn get_weather (weather: u16,frame: u16) -> String {

    let mut text: String = String::new();

    match weather {
        // WINTER
        1 => {
            match frame {
                1 => {
                    text = r#"
                *       
    *                   
            *           
                    *   
      *                 
                        
    (\_/)     *         
    ( •_•)              
    / > ❆           *   
 .-------..             
/          --___________
"#.to_string();
                },
                2 => {
                    text = r#"
                        
                 *      
     *                  
             *          
                     *  
       *                
    (\_/)               
    ( •_•)     *        
    / > ❆               
 .-------..          *  
/          --___________
"#.to_string();

                },
                3 => {
                    text = r#"
         *               
                        
                  *     
      *                 
              *         
                      * 
    (\_/)               
    ( •_•)              
    / > ❆       *       
 .-------..               
/          --___________
"#.to_string();

                },
                4 => {
                    text = r#"
   *                    
         *              
                   *     
                        
       *                 
               *          
    (\_/)              *
    ( •_•)              
    / > ❆               
 .-------..       *     
/          --___________
"#.to_string();

                },
                5 => {
                    text = r#"
    *                   
           *            
                  *     
                        
        *                 
                         
    (\_/)      *         
    ( •_•)          *    
    / > ❆               
 .-------..             
/          --___________
"#.to_string();

                },
                _ => {},
            }
        },
        2 => {},
        3 => {},
        4 => {},
        _ => {},
    }
    text
}


pub fn numbers_gui(num: u16,size: u16) -> String {
    let mut text: String = String::new();

    match num {
        1 => {
            for _i in 0..size {
                for _i in 0..size {
                    text += &"░░"
                }
                for _i in 0..size {
                    text += &"██"
                }
                for _i in 0..size {
                    text += &"░░"
                }
                text += &"\n";
            }
            for _i in 0..size {
                for _i in 0..size {
                    text += &"██"
                }
                for _i in 0..size {
                    text += &"██"
                }
                for _i in 0..size {
                    text += &"░░"
                }
                text += &"\n";
            }
            for _i in 0..size {
                for _i in 0..size {
                    text += &"░░"
                }
                for _i in 0..size {
                    text += &"██"
                }
                for _i in 0..size {
                    text += &"░░"
                }
                text += &"\n";
            }
            for _i in 0..size {
                for _i in 0..size {
                    text += &"░░"
                }
                for _i in 0..size {
                    text += &"██"
                }
                for _i in 0..size {
                    text += &"░░"
                }
                text += &"\n";
            }
            for i in 0..size {
                for _i in 0..size {
                    text += &"██"
                }
                for _i in 0..size {
                    text += &"██"
                }
                for _i in 0..size {
                    text += &"██"
                }
                if i != size {
                    text += &"\n";
                }
            }
        },
        2 => {
            for _i in 0..size {
                for _i in 0..size {
                    text += &"██"
                }
                for _i in 0..size {
                    text += &"██"
                }
                for _i in 0..size {
                    text += &"██"
                }
                text += &"\n";
            }
            for _i in 0..size {
                for _i in 0..size {
                    text += &"░░"
                }
                for _i in 0..size {
                    text += &"░░"
                }
                for _i in 0..size {
                    text += &"██"
                }
                text += &"\n";
            }
            for _i in 0..size {
                for _i in 0..size {
                    text += &"██"
                }
                for _i in 0..size {
                    text += &"██"
                }
                for _i in 0..size {
                    text += &"██"
                }
                text += &"\n";
            }
            for _i in 0..size {
                for _i in 0..size {
                    text += &"██"
                }
                for _i in 0..size {
                    text += &"░░"
                }
                for _i in 0..size {
                    text += &"░░"
                }
                text += &"\n";
            }
            for i in 0..size {
                for _i in 0..size {
                    text += &"██"
                }
                for _i in 0..size {
                    text += &"██"
                }
                for _i in 0..size {
                    text += &"██"
                }
                if i != size {
                    text += &"\n";
                }
            }
        },
        3 => {
            for _i in 0..2 {
                for _i in 0..size {
                    for _i in 0..size {
                        text += &"██"
                    }
                    for _i in 0..size {
                        text += &"██"
                    }
                    for _i in 0..size {
                        text += &"██"
                    }
                    text += &"\n";
                }
                for _i in 0..size {
                    for _i in 0..size {
                        text += &"░░"
                    }
                    for _i in 0..size {
                        text += &"░░"
                        }
                    for _i in 0..size {
                        text += &"██"
                    }
                    text += &"\n";
                }
            }
            for i in 0..size {
                for _i in 0..size {
                    text += &"██"
                }
                for _i in 0..size {
                    text += &"██"
                }
                for _i in 0..size {
                    text += &"██"
                }
                if i != size {
                    text += &"\n";
                }
            }
        },
        4 => {
            for _i in 0..2 {
                for _i in 0..size {
                    for _i in 0..size {
                        text += &"██"
                    }
                    for _i in 0..size {
                        text += &"░░"
                    }
                    for _i in 0..size {
                        text += &"██"
                    }
                    text += &"\n";
                }
            }
            for _i in 0..size {
                for _i in 0..size {
                    text += &"██"
                }
                for _i in 0..size {
                    text += &"██"
                }
                for _i in 0..size {
                    text += &"██"
                }
                text += &"\n";
            }
            for _i in 0..size {
                for _i in 0..size {
                    text += &"░░"
                }
                for _i in 0..size {
                    text += &"░░"
                }
                for _i in 0..size {
                    text += &"██"
                }
                text += &"\n";
            }
            for i in 0..size {
                for _i in 0..size {
                    text += &"░░"
                }
                for _i in 0..size {
                    text += &"░░"
                }
                for _i in 0..size {
                    text += &"██"
                }
                if i != size {
                    text += &"\n";
                }
            }
        },
        5 => { 
            for _i in 0..size {
                for _i in 0..size {
                    text += &"██"
                }
                for _i in 0..size {
                    text += &"██"
                }
                for _i in 0..size {
                    text += &"██"
                }
                text += &"\n";
            }
            for _i in 0..size {
                for _i in 0..size {
                    text += &"██"
                }
                for _i in 0..size {
                    text += &"░░"
                }
                for _i in 0..size {
                    text += &"░░"
                }
                text += &"\n";
            }
            for _i in 0..size {
                for _i in 0..size {
                    text += &"██"
                }
                for _i in 0..size {
                    text += &"██"
                }
                for _i in 0..size {
                    text += &"██"
                }
                text += &"\n";
            }
            for _i in 0..size {
                for _i in 0..size {
                    text += &"░░"
                }
                for _i in 0..size {
                    text += &"░░"
                }
                for _i in 0..size {
                    text += &"██"
                }
                text += &"\n";
            }
            for i in 0..size {
                for _i in 0..size {
                    text += &"██"
                }
                for _i in 0..size {
                    text += &"██"
                }
                for _i in 0..size {
                    text += &"██"
                }
                if i != size {
                    text += &"\n";
                }
            }
        },
        6 => {            
            for _i in 0..size {
                for _i in 0..size {
                    text += &"██"
                }
                for _i in 0..size {
                    text += &"██"
                }
                for _i in 0..size {
                    text += &"██"
                }
                text += &"\n";
            }
            for _i in 0..size {
                for _i in 0..size {
                    text += &"██"
                }
                for _i in 0..size {
                    text += &"░░"
                }
                for _i in 0..size {
                    text += &"░░"
                }
                text += &"\n";
            }
            for _i in 0..size {
                for _i in 0..size {
                    text += &"██"
                }
                for _i in 0..size {
                    text += &"██"
                }
                for _i in 0..size {
                    text += &"██"
                }
                text += &"\n";
            }
            for _i in 0..size {
                for _i in 0..size {
                    text += &"██"
                }
                for _i in 0..size {
                    text += &"░░"
                }
                for _i in 0..size {
                    text += &"██"
                }
                text += &"\n";
            }
            for i in 0..size {
                for _i in 0..size {
                    text += &"██"
                }
                for _i in 0..size {
                    text += &"██"
                }
                for _i in 0..size {
                    text += &"██"
                }
                if i != size {
                    text += &"\n";
                }
            }
        },
        7 => {            
            for _i in 0..size {
                for _i in 0..size {
                    text += &"██"
                }
                for _i in 0..size {
                    text += &"██"
                }
                for _i in 0..size {
                    text += &"██"
                }
                text += &"\n";
            }
            for _i in 0..3 {
                for _i in 0..size {
                    for _i in 0..size {
                        text += &"░░"
                    }
                    for _i in 0..size {
                        text += &"░░"
                    }
                    for _i in 0..size {
                        text += &"██"
                    }
                    text += &"\n";
                }
            }
            for i in 0..size {
                for _i in 0..size {
                    text += &"░░"
                }
                for _i in 0..size {
                    text += &"░░"
                }
                for _i in 0..size {
                    text += &"██"
                }
                if i != size {
                    text += &"\n";
                }
            }
        },
        8 => {
            for _i in 0..2 {

                for _i in 0..size {
                    for _i in 0..size {
                        text += &"██"
                    }
                    for _i in 0..size {
                        text += &"██"
                    }
                    for _i in 0..size {
                        text += &"██"
                    }
                    text += &"\n";
                }            
                for _i in 0..size {
                    for _i in 0..size {
                        text += &"██"
                    }
                    for _i in 0..size {
                        text += &"░░"
                    }
                    for _i in 0..size {
                        text += &"██"
                    }
                    text += &"\n";
                }
            }
            for i in 0..size {
                for _i in 0..size {
                    text += &"██"
                }
                for _i in 0..size {
                    text += &"██"
                }
                for _i in 0..size {
                    text += &"██"
                }
                if i != size {
                    text += &"\n";
                }
            }

        },
        9 => {            
            for _i in 0..size {
                for _i in 0..size {
                    text += &"██"
                }
                for _i in 0..size {
                    text += &"██"
                }
                for _i in 0..size {
                    text += &"██"
                }
                text += &"\n";
            }
            for _i in 0..size {
                for _i in 0..size {
                    text += &"██"
                }
                for _i in 0..size {
                    text += &"░░"
                }
                for _i in 0..size {
                    text += &"██"
                }
                text += &"\n";
            }
            for _i in 0..size {
                for _i in 0..size {
                    text += &"██"
                }
                for _i in 0..size {
                    text += &"██"
                }
                for _i in 0..size {
                    text += &"██"
                }
                text += &"\n";
            }
            for _i in 0..size {
                for _i in 0..size {
                    text += &"░░"
                }
                for _i in 0..size {
                    text += &"░░"
                }
                for _i in 0..size {
                    text += &"██"
                }
                text += &"\n";
            }
            for i in 0..size {
                for _i in 0..size {
                    text += &"░░"
                }
                for _i in 0..size {
                    text += &"░░"
                }
                for _i in 0..size {
                    text += &"██"
                }
                if i != size {
                    text += &"\n";
                }
            }

        },
        0 => {            
            for _i in 0..size {
                for _i in 0..size {
                    text += &"██"
                }
                for _i in 0..size {
                    text += &"██"
                }
                for _i in 0..size {
                    text += &"██"
                }
                text += &"\n";
            }
            for _i in 0..3 {
                for _i in 0..size {
                    for _i in 0..size {
                        text += &"██"
                    }
                    for _i in 0..size {
                        text += &"░░"
                    }
                    for _i in 0..size {
                        text += &"██"
                    }
                    text += &"\n";
                }
            }
            for i in 0..size {
                for _i in 0..size {
                    text += &"██"
                }
                for _i in 0..size {
                    text += &"██"
                }
                for _i in 0..size {
                    text += &"██"
                }
                if i != size {
                    text += &"\n";
                }
            }
        },
        _ => {text = "aa".to_string()}
    }

    return text
}

