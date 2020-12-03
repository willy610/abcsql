#!/bin/bash
cd /Users/willy/MYRUST/sqltest
rm -r examples/meta_tables/db

./target/debug/abc_sql --newdb examples/meta_tables/db --newdb examples/meta_tables/db
./target/debug/abc_sql --insql examples/meta_tables/offdb/tables.sql --olddb examples/meta_tables/db
./target/debug/abc_sql --insql examples/meta_tables/offdb/tablecolumns.sql --olddb examples/meta_tables/db
./target/debug/abc_sql --insql examples/meta_tables/offdb/indexes.sql --olddb examples/meta_tables/db
./target/debug/abc_sql --insql examples/meta_tables/offdb/indexmembers.sql --olddb examples/meta_tables/db
./target/debug/abc_sql --insql examples/meta_tables/offdb/fk.sql --olddb examples/meta_tables/db
./target/debug/abc_sql --insql examples/meta_tables/offdb/fkmembers.sql --olddb examples/meta_tables/db
