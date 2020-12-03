use crate::applic_folder::compileargs::*;
use crate::applic_folder::go_exec::go_exec;
use std::io::{self, Write};
use std::rc::Rc;

/***************************************************/
pub fn go_talk(how: HowToOpenStorage, seen: &Seen) {
    let x_how = Rc::new(how);
    'outmost: loop {
        print!("abcsql   > ");
        io::stdout().flush().unwrap();
        let mut total_input: Vec<String> = Vec::new();
        'collect_statement: loop {
            let mut each_input = String::new();
            match io::stdin().read_line(&mut each_input) {
                Ok(length) => {
                    if length <= 1 {
                        break 'collect_statement;
                    }
                    if each_input.ends_with(";\n") {
                        let last_input = each_input.trim_end_matches(";\n");
                        total_input.push(last_input.to_string());
                        total_input.push(";".to_string());
                        eprintln!("{:?}", total_input.join(" "));
                        go_exec(x_how.clone(), total_input.join(" "), seen);
                        break 'collect_statement;
                    } else {
                        total_input.push(each_input.trim().to_string());
                        print!("         > ");
                        io::stdout().flush().unwrap();
                    }
                }
                Err(_e) => break 'outmost,
            }
            if total_input.len() == 0 {
                break 'outmost;
            }
        }
        if total_input.len() == 0 {
            break;
        }
    }
}
