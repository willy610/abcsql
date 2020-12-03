# ABCSQL

## A tryout of gluesql

This is a simple console application which will elaborate on gluesql. Might be useful for integration tests.

## Usage

```
>abc_sql  --newdb filename
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
            [--droptable] [--createtable] 
```

### --newdb

On its own will create a empty database. Initial and once usage for a db.

### --insql

Will use the ``--olddb`` and read the file ``--insql file`` which will hold any SQL statments. Any ``select`` will be reported via optional ``[--printsqlstm]``

### --olddb

Start an interactive session via ``--olddb filename``. End the row with a ``;`` and the statements will be executed. Empty statement will terminate the session.

### --incsv --outcsv
Import and export csv. Quite simple but there is an ``--append``

### --outsql
Will dump a table into restorable format

## Build

```
cargo build --features sled-storage
```

## Examples

There are 4 folders. Organized like the exampled ``chrisdate``

```
chrisdate/
-db
-offdb
```

``db`` is the database from create and ``offdb`` is whatever useful like intial scripts and verfication scripts. There are often also shell scripts for complete load and tests. Look into *.command

### Running an example interactively

```
here > ./target/debug/abcsql --olddb examples/chrisdate/db
abcsql  > select * from S left join P on S.CITY = P.CITY;


+-----+-------+--------+--------+------+-------+-------+--------+--------+
| SNO | SNAME | STATUS |  CITY  | PNO  | PNAME | COLOR | WEIGHT |  CITY  |
+-----+-------+--------+--------+------+-------+-------+--------+--------+
| S1  | Smith |     20 | London | P1   | Nut   | Red   |     12 | London |
| S1  | Smith |     20 | London | P4   | Screw | Red   |     14 | London |
| S1  | Smith |     20 | London | P6   | Cog   | Red   |     19 | London |
| S2  | Jones |     10 | Paris  | P2   | Bolt  | Green |     17 | Paris  |
| S2  | Jones |     10 | Paris  | P5   | Cam   | Blue  |     12 | Paris  |
| S3  | Blake |     30 | Paris  | P2   | Bolt  | Green |     17 | Paris  |
| S3  | Blake |     30 | Paris  | P5   | Cam   | Blue  |     12 | Paris  |
| S4  | Clark |     20 | London | P1   | Nut   | Red   |     12 | London |
| S4  | Clark |     20 | London | P4   | Screw | Red   |     14 | London |
| S4  | Clark |     20 | London | P6   | Cog   | Red   |     19 | London |
| S5  | Adams |     20 | Athens | null | null  | null  | null   | null   |
+-----+-------+--------+--------+------+-------+-------+--------+--------+

abcsql  >
```

### Running a script

```
here > ./target/debug/abcsql --olddb examples/chrisdate/db --insql examples/chrisdate/offdb/testorderby.sql --printsqlstm
```
gives

```
select * from S

+-----+-------+--------+--------+
| SNO | SNAME | STATUS |  CITY  |
+-----+-------+--------+--------+
| S1  | Smith |     20 | London |
| S2  | Jones |     10 | Paris  |
| S3  | Blake |     30 | Paris  |
| S4  | Clark |     20 | London |
| S5  | Adams |     20 | Athens |
+-----+-------+--------+--------+

select * from S order by CITY DESC,3

+-----+-------+--------+--------+
| SNO | SNAME | STATUS |  CITY  |
+-----+-------+--------+--------+
| S2  | Jones |     10 | Paris  |
| S3  | Blake |     30 | Paris  |
| S1  | Smith |     20 | London |
| S4  | Clark |     20 | London |
| S5  | Adams |     20 | Athens |
+-----+-------+--------+--------+


here >
```
