#[cfg(feature = "sled-storage")]
use gluesql::{parse, Glue, Payload::*, Query};

use sqlparser::ast::{OrderByExpr, Statement};

use std::rc::Rc;

use crate::applic_folder::compileargs::*;
use crate::applic_folder::get_storage::get_storage;
use crate::applic_folder::orderby::*;
use crate::applic_folder::show_select::*;
/***************************************************/
pub fn go_exec(how: Rc<HowToOpenStorage>, one_query: String, seen: &Seen) {
    let storage = get_storage(how);
    let mut glue = Glue::new(storage);
    for query in parse(&one_query).unwrap() {
        match &glue.execute(&query).unwrap() {
            Select { labels, rows } => {
                let mut order_by: Vec<OrderByExpr> = Vec::new();
                match query {
                    Query(xxx) => {
                        match &xxx {
                            Statement::Query(yyy) => {
                                order_by = yyy.order_by.clone();
                            }
                            _ => {
                                eprintln!("???{}", xxx);
                            }
                        };
                    }
                }
                let rows = orderby(labels.to_vec(), rows.to_vec(), &order_by);
                show_select(
                    labels.to_vec(),
                    rows.to_vec(),
                    if seen.printsqlstm {
                        Some(&one_query)
                    } else {
                        None
                    },
                )
            }
            Insert(n) => eprintln!("{} rows inserted", n),
            Delete(n) => eprintln!("{} rows deleted", n),
            Update(n) => eprintln!("{} rows update", n),
            Create => eprintln!("Table created"),
            DropTable => eprintln!("Table dropped"),
            _ => eprintln!("Not yet fom glue.execute"),
        }
    }
}
