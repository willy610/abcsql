#!/bin/bash
cd /Users/willy/MYRUST/sqltest
rm -r examples/personkonto/db
./target/debug/abc_sql --newdb examples/chrivsdate/db
./target/debug/abc_sql --incsv examples/personkonto/offdb/2020_07_02.csv --olddb examples/personkonto/db --table kontot
./target/debug/abc_sql --incsv examples/personkonto/offdb/2020_07_02.csv --olddb examples/personkonto/db --table kontot --append
./target/debug/abc_sql --outcsv examples/personkonto/offdb/dump.csv --olddb examples/personkonto/db --table kontot
./target/debug/abc_sql --outsql examples/personkonto/offdb/dump.sql --olddb examples/personkonto/db --table kontot

