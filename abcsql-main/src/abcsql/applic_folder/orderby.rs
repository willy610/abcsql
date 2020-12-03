use gluesql::Row;
use sqlparser::ast::{Expr::*, OrderByExpr, Value};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::rc::Rc;
/***********************************************/
#[derive(Debug)]
pub struct OrderByAndCompare {
    pub col_number: usize,
    pub ascending: bool,
}
#[derive(Debug)]
pub struct ARow2Sort {
    pub the_row: Row,
    pub orderby: Rc<Vec<OrderByAndCompare>>,
}
/***********************************************/
// We need an Ord for ARow2Sort
impl Ord for ARow2Sort {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
impl PartialEq for ARow2Sort {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}
impl PartialOrd for ARow2Sort {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        for one_order_by in self.orderby.iter() {
            if self.the_row.get_value(one_order_by.col_number)
                < other.the_row.get_value(one_order_by.col_number)
            {
                return Some(if one_order_by.ascending {
                    Ordering::Less
                } else {
                    Ordering::Greater
                });
            }
            if self.the_row.get_value(one_order_by.col_number)
                > other.the_row.get_value(one_order_by.col_number)
            {
                return Some(if one_order_by.ascending {
                    Ordering::Greater
                } else {
                    Ordering::Less
                });
            }
        }
        Some(Ordering::Equal)
    }
}
impl Eq for ARow2Sort {}
/***********************************************/
//////////////
#[derive(Debug)]
struct ABinaryHeap {
    pub the_heap: BinaryHeap<ARow2Sort>,
}
/*
.pop() will not return sorted!!
must use 'into_sorted_vec()'

impl Iterator for ABinaryHeap {
    type Item = ARow2Sort;
    fn next(&mut self) -> Option<ARow2Sort> {
      let what = self.the_heap.pop();
      match what {
            Some(val) => Some(val),
            None => None,
        }
    }
}
*/
//////////////
/***********************************************/
pub fn orderby(
    labels: Vec<String>,
    rows_in: Vec<Row>,
    order_by: &Vec<OrderByExpr>,
) -> std::vec::Vec<gluesql::Row> {
    let mut mother_the_binary_heap = ABinaryHeap {
        the_heap: BinaryHeap::new(),
    };
    // 1. Compile the order by into a vector with colindex and asc/desc
    //   ( OrderByAndCompare )
    // 2. Insert the rows into the heap. use the OrderByAndCompare in Ord
    // 3. Retreive the rows and map of Row's
    if order_by.len() == 0 {
        return rows_in;
    }
    let mother_order_by_all: Vec<OrderByAndCompare> = order_by
        .iter()
        .map(|one_order_by| {
            let OrderByExpr {
                expr,
                asc,
                nulls_first: _,
            } = one_order_by;
            let colnumber = match expr {
                Identifier(ident) => {
                    let index = labels
                        .iter()
                        .position(|r| &**r == ident.value)
                        .expect("columnname not found");
                    index
                }
                Value(column_number) => match column_number {
                    Value::Number(thevale) => {
                        let my_int: usize = thevale.parse().expect("Order by must be an integer");
                        my_int - 1
                    }
                    _ => panic!("Order by must be a number"),
                },
                _ => panic!("Order by must be a number or an ident"),
            };
            // now look at asc/desc
            let real_asc = match asc {
                Some(what) => *what,
                None => true,
            };
            OrderByAndCompare {
                col_number: colnumber,
                ascending: real_asc,
            }
        })
        .collect::<Vec<OrderByAndCompare>>();
    let the_orderby_once_cloned = Rc::new(mother_order_by_all);
    // 2. Insert into the sorter
    for enrad in rows_in {
        let xxx = ARow2Sort {
            the_row: enrad.clone(),
            orderby: the_orderby_once_cloned.clone(),
        };
        mother_the_binary_heap.the_heap.push(xxx);
    }
    // 3. And retrive the rows
    let arow2_sort_grouped_by = mother_the_binary_heap.the_heap.into_sorted_vec();
    let rows_grouped_by: Vec<Row> = arow2_sort_grouped_by
        .into_iter()
        .map(|arow2_sort| arow2_sort.the_row)
        .collect();
    rows_grouped_by
}
