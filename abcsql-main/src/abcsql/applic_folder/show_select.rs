#[cfg(feature = "sled-storage")]
use gluesql::{Row, Value};

/***************************************************/
pub fn show_select(labels: Vec<String>, rows: Vec<Row>, show_sql: Option<&str>) {
    /*
      +---------+------------+
      | Col1    |    Col2    |
      +---------+------------+
      |      23 | Nisses     |
      | 9299292 | Minnesotaa |
      +---------+------------+
    */

    let all_rows: Vec<Vec<String>> = rows
        .iter()
        .map(|arow| {
            let Row(values) = arow;
            let one_row: Vec<String> = values
                .iter()
                .map(|acolumn| {
                    let x = match acolumn {
                        Value::Str(ref s) => s.to_string(),
                        Value::F64(ref f) => format!("{:?}", f),
                        Value::I64(ref i) => format!("{:?}", i),
                        Value::Empty => "null".to_string(),
                        _ => "notyet".to_string(),
                    };
                    x
                })
                .collect::<Vec<String>>();
            one_row
        })
        .collect::<Vec<Vec<String>>>();
    // calculate label widths
    let label_widths: Vec<usize> = labels.iter().map(|colvalue| colvalue.len()).collect();
    // calculate max width for each column
    let max_col_widths = all_rows.iter().fold(label_widths, |sofar, arow| {
        arow.iter()
            .enumerate()
            .map(|(i, acol)| {
                if acol.len() > sofar[i] {
                    acol.len()
                } else {
                    sofar[i].clone()
                }
            })
            .collect()
    });
    /*------------------------------------------*/
    let build_topline = || -> String {
        format!(
            "+{}+",
            max_col_widths
                .iter()
                .map(|col_width| format!("-{}-", String::from("-".repeat(*col_width))))
                .collect::<Vec<_>>()
                .join("+")
        )
    };
    /*------------------------------------------*/
    let build_labels = || -> String {
        format!(
            "|{}|",
            labels
                .iter()
                .enumerate()
                .map(|(i, lab)| format!(" {: ^1$} ", lab.clone(), max_col_widths[i]))
                .collect::<Vec<_>>()
                .join("|")
        )
    };
    /*------------------------------------------*/
    let build_rows = || -> String {
        all_rows
            .iter()
            .map(|arow| {
                format!(
                    "|{}|",
                    arow.into_iter()
                        .enumerate()
                        .map(|(i, colval)| match colval.parse::<f64>() {
                            Ok(_) => format!(" {: >1$} ", colval.clone(), max_col_widths[i]),
                            Err(_) => format!(" {: <1$} ", colval.clone(), max_col_widths[i]),
                        })
                        .collect::<Vec<String>>()
                        .join("|")
                )
            })
            .collect::<Vec<String>>()
            .join("\n")
    };
    /*------------------------------------------*/
    // FINAL RESULT
    eprintln!(
        "{stm}\n
{topline}
{labels}
{topline}
{rows}
{topline}
",
        stm = show_sql.unwrap_or(""),
        topline = build_topline(),
        labels = build_labels(),
        rows = build_rows()
    );
}
