use std::fs;
use std::rc::Rc;

use crate::applic_folder::compileargs::*;
use crate::applic_folder::csvio::{dump_csv, load_csv};
use crate::applic_folder::dump_sql::dump_sql;
use crate::applic_folder::go_exec::go_exec;
use crate::applic_folder::go_talk::go_talk;

#[cfg(feature = "sled-storage")]
use gluesql::SledStorage;

// https://www.mysqltutorial.org/import-csv-file-mysql-table/
// https://www.sqlitetutorial.net/sqlite-import-csv/

#[cfg(feature = "sled-storage")]
pub fn realmain() -> Result<String, String> {
    match compileargs() {
        Ok(what) => match what {
            What::Help(txt) => Ok(txt.to_string()),
            What::NewDB(my_input, _seen) => match SledStorage::new(&my_input.db_name.unwrap()) {
                Ok(_xxx) => Ok("NewDB".to_string()),
                Err(_xxx) => Err(_xxx.to_string()),
            },
            What::InteractiveSQL(my_input, seen) => {
                let how: HowToOpenStorage = HowToOpenStorage::NewStorage(my_input.db_name.unwrap());
                go_talk(how, &seen);
                Ok("".to_string())
            }
            What::InSql(my_input, seen) => {
                let sqls = fs::read_to_string(my_input.sql_filename.unwrap())
                    .expect("Something went wrong reading the file");
                let all_querys = if seen.printsqlstm {
                    let vec = sqls.split(";\n").collect::<Vec<&str>>();
                    vec
                } else {
                    vec![sqls.as_str()]
                };
                let x = my_input.db_name.unwrap();
                for one_query in all_querys {
                    let how = Rc::new(HowToOpenStorage::OldStorage(x.to_string()));
                    go_exec(how, one_query.to_string(), &seen);
                }
                Ok("".to_string())
            }
            What::InCsv(my_input, seen) => match load_csv(&my_input, &seen) {
                Ok(_) => Ok("".to_string()),
                Err(x) => Err(x.to_string()),
            },
            What::OutCsv(my_input, seen) => match dump_csv(&my_input, &seen) {
                Ok(_) => Ok("".to_string()),
                Err(x) => Err(x.to_string()),
            },
            What::OutSql(my_input, seen) => match dump_sql(&my_input, &seen) {
                Ok(_) => Ok("".to_string()),
                Err(x) => Err(x.to_string()),
            },
        },
        Err(txt) => Err(txt.to_string()),
    }
}
