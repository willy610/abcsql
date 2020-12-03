#!/bin/bash
cd /Users/willy/MYRUST/sqltest
rm -r examples/recipes/db
./target/debug/abc_sql --newdb examples/recipes/db
./target/debug/abc_sql --insql examples/recipes/offdb/ingredient.sql --olddb examples/recipes/db
./target/debug/abc_sql --insql examples/recipes/offdb/category.sql --olddb examples/recipes/db
./target/debug/abc_sql --insql examples/recipes/offdb/categoryandrecipe.sql --olddb examples/recipes/db
./target/debug/abc_sql --insql examples/recipes/offdb/recipe.sql --olddb examples/recipes/db
./target/debug/abc_sql --insql examples/recipes/offdb/recipestep.sql --olddb examples/recipes/db
./target/debug/abc_sql --insql examples/recipes/offdb/recipestepingredient.sql --olddb examples/recipes/db


