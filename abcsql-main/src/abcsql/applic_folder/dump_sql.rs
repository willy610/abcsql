#[cfg(feature = "sled-storage")]
use gluesql::{parse, Glue, Payload::*, Row, Value};

use std::fs::File;
use std::io::prelude::*;
use std::rc::Rc;

use crate::applic_folder::compileargs::*;
use crate::applic_folder::get_storage::get_storage;

pub fn dump_sql(my_input: &TheInput, seen: &Seen) -> std::io::Result<()> {
    let sql = format!(
        "select * from {};",
        my_input.tablename.as_ref().expect("Missing tablename")
    );
    let db_name = my_input
        .db_name
        .as_ref()
        .expect("Missing dbname")
        .to_string();
    let how = Rc::new(HowToOpenStorage::OldStorage(db_name));
    let storage = get_storage(how);
    let mut glue = Glue::new(storage);
    let the_file_name = my_input.sql_filename.as_ref().unwrap();
    let mut file = File::create(the_file_name)?;
    for query in parse(&sql).unwrap() {
        match &glue.execute(&query).unwrap() {
            Select { labels, rows } => {
                /*********************************************************/
                let build_create_table = || {
                    format!(
                        r#"DROP TABLE IF EXISTS {tablename};
CREATE TABLE S ({allcolls});
"#,
                        tablename = my_input.tablename.as_ref().unwrap(),
                        allcolls = labels
                            .iter()
                            .map(|acol| format!("{} text", acol))
                            .collect::<Vec<String>>()
                            .join(",")
                    )
                };
                /*********************************************************/
                let built_insert_into = format!(
                    "INSERT INTO {} ",
                    my_input.tablename.as_ref().expect("Missing tablename")
                );
                /*********************************************************/
                let build_insert_cols = || {
                    labels
                        .iter()
                        .map(|acol| format!("{}", acol))
                        .collect::<Vec<String>>()
                        .join(",")
                };
                /*********************************************************/
                if seen.droptable {
                    write!(file, "{}", build_create_table())?;
                }
                write!(
                    file,
                    "{}({}) VALUES 
",
                    built_insert_into,
                    build_insert_cols()
                )
                .unwrap();
                let mut first_row_seen: bool = false;
                for arow in rows {
                    let Row(values) = arow;
                    let one_row: Vec<String> = values
                        .iter()
                        .map(|acolumn| {
                            let a_print_col = match acolumn {
                                Value::Str(ref s) => format!("\"{}\"", s.to_string()),
                                Value::F64(ref f) => format!("{:?}", f),
                                Value::I64(ref i) => format!("{:?}", i),
                                Value::Empty => "null".to_string(),
                                _ => "notyet".to_string(),
                            };
                            a_print_col
                        })
                        .collect::<Vec<String>>();

                    if first_row_seen {
                        write!(file, "{}\n", ",")?;
                    } // list separator on prev row
                    first_row_seen = true;
                    write!(file, "({})", one_row.join(","))?;
                }
                write!(file, "{}\n", ";")?; // fina;
                file.flush()?;
            }
            _ => {}
        }
    }
    Ok(())
}
