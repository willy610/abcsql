#!/bin/bash
cd /Users/willy/MYRUST/sqltest
rm -r examples/chrisdate/db
./target/debug/abc_sql --newdb examples/chrisdate/db
./target/debug/abc_sql --insql examples/chrisdate/offdb/s.sql --olddb examples/chrisdate/db
./target/debug/abc_sql --insql examples/chrisdate/offdb/p.sql --olddb examples/chrisdate/db
./target/debug/abc_sql --insql examples/chrisdate/offdb/sp.sql --olddb examples/chrisdate/db
./target/debug/abc_sql --insql examples/chrisdate/offdb/j.sql --olddb examples/chrisdate/db
./target/debug/abc_sql --insql examples/chrisdate/offdb/spj.sql --olddb examples/chrisdate/db
