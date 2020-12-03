// copy from lib-and-bin


//use crate::*;
pub use gluesql::*;
//use gluesql::{sled, Glue, glue, SledStorage};
use gluesql::{parse, sled, Glue, SledStorage};
use std::convert::TryFrom;
use std::fs;
//use gluesql::{sled};
//use gluesql::*;

//use lib_folder::{sled};
//use crate::{sled};
//use gluesql::{sled};

//use gluesql::test;

pub fn main() {
    //    test();

    let config = sled::Config::default()
        .path("examples/recipes/recipe.db")
        .temporary(false);
    let storage = SledStorage::try_from(config).expect("No storage found");
    let mut glue = Glue::new(storage);
    let tables = vec![
        "examples/recipes/ingredient.sql",
        "examples/recipes/category.sql",
        "examples/recipes/categoryandrecipe.sql",
        "examples/recipes/recipe.sql",
        "examples/recipes/recipestep.sql",
        "examples/recipes/recipestepingredient.sql",
    ];
    // Load all tables in the database
    for atable in tables {
        let sqls = fs::read_to_string(atable).expect("Something went wrong when reading the csv file");
        for query in parse(&sqls).unwrap() {
            let _result = glue.execute(&query);
        }
    }
    let statements = vec![
  /*  
    vec![
      "SELECT ingredientid,ingredientid+2000 as PLUS2000 FROM ingredient ",
      "WHERE energy > 500 ",
      "ORDER BY 1;",
  ],
  */
    // subquery 
/*
    vec![
        "SELECT energy",
        " FROM (SELECT energy FROM ingredient) ",
        " WHERE energy > 400 ;"
    ],
    */
    vec![
        "SELECT * FROM ingredient ",
        "WHERE energy > 400 ",
        "ORDER BY 1;",
    ],
    // subquery
    /*
    vec![
        "SELECT energy",
        " FROM (SELECT energy FROM ingredient) as YYY",
        " WHERE YYY.energy > 400 ;"
    ],
    */

    vec![
        "SELECT category.* FROM category ",
        "WHERE categoryid > 40 ",
        "order BY 1;",
    ],
    vec![
        "SELECT * FROM recipe ",
        "WHERE maketime > 100 ",
        "order BY 1;",
    ],
    vec![
        "SELECT * FROM recipestep ",
        "WHERE minutes > 100 ",
        "order BY 1;",
    ],
    //which steps use a certain ingredientid
    vec![
        "SELECT * FROM recipestepingredient ",
        "WHERE ingredientid = 38 ",
        "order BY 1;",
    ],
    vec![
        "SELECT ingredientid AS NEWCOLNAME ",
        "FROM   ingredient ",
        "WHERE  ingredientname = 'mjölk';",
    ],
    // what recipes use ingredient 'mjölk'
    // slow
    /*
    vec![
        "SELECT recipename ",
        "FROM   recipe ",
        "       JOIN recipestepingredient ",
        "         ON recipe.recipeid = recipestepingredient.recipeid ",
        "WHERE  ingredientid = (SELECT ingredientid ",
        "                       FROM   ingredient ",
        "                       WHERE  ingredientname = 'mjölk') ;",
    ],
    */
    // same as above but with 3 joins. slow
    /*
    vec![
        "SELECT recipename ",
        "FROM   recipe ",
        "        JOIN recipestepingredient ",
        "          ON recipe.recipeid = recipestepingredient.recipeid ",
        "        JOIN ingredient ",
        "          ON ingredient.ingredientid = recipestepingredient.ingredientid ",
        "WHERE  ingredient.ingredientname = 'mjölk';",
    ],
    */
    // what are the most used category in recipe.
    vec![
        "SELECT categoryid, ",
        "       Count(*) ",
        "FROM   categoryandrecipe ",
        "GROUP  BY categoryid ",
        "ORDER BY 2;",
    ],
    // what are the most used category in recipe. answer recipename and count
    // will give Err 'TableFactorNotSupported'
    
   vec!["SELECT * FROM category JOIN categoryandrecipe ON category.categoryid = categoryandrecipe.categoryid;"],

vec![
        "SELECT * FROM ingredient AS XXXX ",
        "WHERE XXXX.energy > 400 ",
        "ORDER BY 1;",
    ],
    
    // subquery 
    /*
    vec![
        "SELECT energy",
        " FROM (SELECT energy AS EEE FROM ingredient)",
        " WHERE EEE > 400 ;"
    ],
    */
  
  /*
    vec![
        "SELECT derived_alias.categoryname, ",
        "       Count(*) ",
        "FROM   (SELECT category.categoryname ",
        "        FROM   category ",
        "               JOIN categoryandrecipe ",
        "                 ON category.categoryid = categoryandrecipe.categoryid) AS ",
        "       derived_alias ",
        "GROUP  BY derived_alias.categoryname ;",
    ],
    */
];
    for (i, statementpart) in statements.iter().enumerate() {
        eprintln!("sql({:?})=", i);
        for arow in statementpart {
            eprintln!("{}", arow);
        }
        eprintln!("");

        for query in parse(&statementpart.concat()).unwrap() {
            //      eprintln!("query={:#?}", query);
            match glue.execute(&query).unwrap() {
                //          Payload::Select(rows) => {
                Payload::SelectNEW {
                    column_names: colsnames,
                    rows,
                } => {
                    //            let _cols = get_projection_aliases(&query);
                    //              eprintln!("{:?}",colsnames.join("\t"));
                    for acol in colsnames {
                        eprint!("{}\t", acol);
                    }
                    eprintln!("");

                    for arow in rows {
                        let Row(values) = arow;
                        for acolumnvalue in values {
                            match acolumnvalue {
                                Value::Str(ref s) => eprint!("'{}'\t", s),
                                Value::F64(ref f) => eprint!("{:?}\t", f),
                                Value::I64(ref i) => eprint!("{:?}\t", i),
                                _ => eprint!("acolumnvalue={:?}", acolumnvalue),
                            };
                        }
                        eprintln!("");
                    }
                }
                _ => { /* Payload::Select*/ }
            }
        }
    }
}
