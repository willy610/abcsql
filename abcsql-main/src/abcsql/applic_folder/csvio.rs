#[cfg(feature = "sled-storage")]
use gluesql::{parse, Glue, Payload::*, Row, Value};

use std::rc::Rc;

use crate::applic_folder::compileargs::*;
use crate::applic_folder::get_storage::get_storage;
use crate::applic_folder::go_exec::go_exec;

extern crate csv;
/*-------------------------------------------------------------*/
pub fn load_csv(my_input: &TheInput, seen: &Seen) -> std::io::Result<()> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b';')
        .from_path(my_input.csv_filename.as_ref().expect("Can't find csv file"))?;
    let headers = rdr.headers().expect("Can't wind headers in csv ");
    /*********************************************************/
    let built_create_table = if seen.append {
        "".to_string()
    } else {
        format!(
            "CREATE TABLE {} ({});",
            my_input
                .tablename
                .as_ref()
                .expect("Missing tablename for create table"),
            headers
                .iter()
                .map(|c| format!("{} text", c))
                .collect::<Vec<String>>()
                .join(",\n")
        )
    };
    /*********************************************************/
    let built_insert_into = format!(
        "INSERT INTO {} VALUES ",
        my_input
            .tablename
            .as_ref()
            .expect("Missing tablename for insert into table")
    );
    /*********************************************************/
    let built_insert_lines = rdr
        .records()
        .map(|arow| {
            format!(
                "({})",
                arow.expect("Some error line from 'select' ")
                    .iter()
                    .map(|acol| format!("\"{}\"", acol))
                    .collect::<Vec<String>>()
                    .join(",")
            )
        })
        .collect::<Vec<String>>()
        .join(",");
    /*********************************************************/
    let db_name = my_input
        .db_name
        .as_ref()
        .expect("Missing dbname")
        .to_string();
    let how = Rc::new(HowToOpenStorage::OldStorage(db_name));

    go_exec(
        how,
        format!(
            "{}{}{};",
            built_create_table, built_insert_into, built_insert_lines,
        ),
        &seen,
    );
    Ok(())
}
/*-------------------------------------------------------------*/
pub fn dump_csv(my_input: &TheInput, _seen: &Seen) -> std::io::Result<()> {
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
    for query in parse(&sql).unwrap() {
        match &glue.execute(&query).unwrap() {
            Select { labels, rows } => {
                let mut wtr = csv::WriterBuilder::new()
                    .has_headers(true)
                    .delimiter(b';')
                    .from_path(
                        my_input
                            .csv_filename
                            .as_ref()
                            .expect("Missing csv filename"),
                    )?;
                //                    .expect("Can't write headers to csv file");
                wtr.write_record(&labels.to_vec()).expect("!");
                for arow in rows {
                    let Row(values) = arow;
                    let one_row: Vec<String> = values
                        .iter()
                        .map(|acolumn| {
                            let a_print_col = match acolumn {
                                Value::Str(ref s) => s.to_string(),
                                Value::F64(ref f) => format!("{:?}", f),
                                Value::I64(ref i) => format!("{:?}", i),
                                Value::Empty => "null".to_string(),
                                _ => "notyet".to_string(),
                            };
                            a_print_col
                        })
                        .collect::<Vec<String>>();
                    wtr.write_record(&one_row.to_vec())?;
                }
                wtr.flush()?; //.expect("Can't flush csv file");
            }
            _ => {}
        }
    }
    Ok(())
}
