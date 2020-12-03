use std::env;
#[derive(Debug)]
pub struct Seen {
    pub newdb: bool,
    pub insql: bool,
    pub outsql: bool,
    pub olddb: bool,
    pub incsv: bool,
    pub outcsv: bool,
    pub tablename: bool,
    pub printsqlstm: bool,
    pub append: bool,
    pub droptable: bool,
    pub createtable: bool,
}
#[derive(Debug)]
pub struct TheInput {
    pub db_name: Option<String>,
    pub sql_filename: Option<String>,
    pub csv_filename: Option<String>,
    pub tablename: Option<String>,
}
#[derive(Debug)]
pub enum What {
    Help(&'static str),
    NewDB(TheInput, Seen),
    InSql(TheInput, Seen),
    InCsv(TheInput, Seen),
    InteractiveSQL(TheInput, Seen),
    OutCsv(TheInput, Seen),
    OutSql(TheInput, Seen),
}
pub enum HowToOpenStorage {
    NewStorage(String),
    OldStorage(String),
}

pub fn compileargs() -> Result<What, &'static str> {
    let help = r#"
abc_sql   --newdb filename
        | --insql file --olddb filename [--printsqlstm]
        | --incsv file --olddb filename --table tablename 
            [ --append ]
            #NOT_YET ( 
              (--headers "col1,col2" [ --ignorefirtslineheaders ] )
              | --headersfirstlineincsv 
            ) 
            #NOT_YET [--colsep ','] [ --linsep ' ']
        | --olddb filename #interactive usage !
        | --outcsv filename --olddb filename --table tablename
        | --outsql filename --olddb filename --table tablename
            [--droptable] [--createtable] "#;

    let mut seen = Seen {
        newdb: false,
        insql: false,
        outsql: false,
        olddb: false,
        incsv: false,
        outcsv: false,
        tablename: false,
        printsqlstm: false,
        append: false,
        droptable: false,
        createtable: false,
    };
    let mut my_input = TheInput {
        db_name: None,
        sql_filename: None,
        csv_filename: None,
        tablename: None,
    };
    let args: Vec<String> = env::args().collect();
    let mut pos = 1;
    let args_len = args.len();
    while pos < args_len {
        match args[pos].as_str() {
            "--help" => return Result::Ok(What::Help(help)),
            "--printsqlstm" => seen.printsqlstm = true,
            "--append" => seen.append = true,
            "--droptable" => seen.droptable = true,
            "--createtable" => seen.createtable = true,
            "--newdb" => {
                pos += 1;
                if pos < args_len {
                    my_input.db_name = Some(args[pos].clone());
                    seen.newdb = true;
                } else {
                    return Result::Err("--newdb requires 'filename'");
                }
            }
            "--olddb" => {
                pos += 1;
                if pos < args_len {
                    my_input.db_name = Some(args[pos].clone());
                    seen.olddb = true;
                } else {
                    return Result::Err("--olddb requires 'filename'");
                }
            }
            "--insql" => {
                pos += 1;
                if pos < args_len {
                    my_input.sql_filename = Some(args[pos].clone());
                    seen.insql = true;
                } else {
                    return Result::Err("--insql requires 'filename'");
                }
            }
            "--incsv" => {
                pos += 1;
                if pos < args_len {
                    my_input.csv_filename = Some(args[pos].clone());
                    seen.incsv = true;
                } else {
                    return Result::Err("--incsv requires 'filename'");
                }
            }
            "--outcsv" => {
                pos += 1;
                if pos < args_len {
                    my_input.csv_filename = Some(args[pos].clone());
                    seen.outcsv = true;
                } else {
                    return Result::Err("--outcsv requires 'filename'");
                }
            }
            "--table" => {
                pos += 1;
                if pos < args_len {
                    my_input.tablename = Some(args[pos].clone());
                    seen.tablename = true;
                } else {
                    return Result::Err("--table requires 'tablename'");
                }
            }
            "--outsql" => {
                pos += 1;
                if pos < args_len {
                    my_input.sql_filename = Some(args[pos].clone());
                    seen.outsql = true;
                } else {
                    return Result::Err("--insql requires 'filename'");
                }
            }
            _ => return Result::Err("Missing options"),
        }
        pos += 1;
    }
    // Horrible forward
    if seen.newdb {
        if seen.insql || seen.olddb || seen.incsv || seen.outcsv || seen.outsql {
            Result::Err("--newdb set but perhaps insql, olddb, incsv, outcsv or outsql set too")
        } else {
            Ok(What::NewDB(my_input, seen))
        }
    } else if seen.insql && seen.olddb {
        // required
        if seen.incsv || seen.outcsv || seen.outsql || seen.tablename {
            Result::Err("--insql but perhaps  olddb, incsv, outcsv or outsql set too")
        } else {
            Ok(What::InSql(my_input, seen))
        }
    } else if seen.incsv && seen.olddb && seen.tablename {
        // required
        if seen.newdb || seen.outcsv || seen.outsql {
            Result::Err("--incsv but perhaps newdb, outcsv or outsql set too")
        } else {
            Ok(What::InCsv(my_input, seen))
        }
    } else if seen.outcsv && seen.olddb && seen.tablename {
        // alone
        if seen.newdb || seen.incsv || seen.outsql {
            Result::Err("--outcvs but perhaps newdb, incsv or outsql set too")
        } else {
            Ok(What::OutCsv(my_input, seen))
        }
    } else if seen.olddb && seen.outsql && seen.tablename {
        // alone
        if seen.newdb || seen.incsv || seen.insql {
            Result::Err("--outsql but --newdb set too")
        } else {
            Ok(What::OutSql(my_input, seen))
        }
    } else if seen.olddb {
        // alone
        if seen.newdb || seen.incsv || seen.insql || seen.outsql || seen.outcsv || seen.tablename {
            Result::Err("--olddb but others set too")
        } else {
            Ok(What::InteractiveSQL(my_input, seen))
        }
    } else {
        Ok(What::Help(help))
    }
}
