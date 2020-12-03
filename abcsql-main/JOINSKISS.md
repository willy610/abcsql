# Some notes on Join (and indexes)

1. FULL [ OUTER ] joins could be compiled into 'select ... left join(...) union right join(...)'
2. INNER join. If indexes are present chose the smallest index one for outer scan.
3. LEFT or RIGHT. Look into pseuocode here. Outer must scan whole table. ``iter()`` on inner could be a 'match' on index
4. I think moving all joins from select.rs into a new from.rs would make things more maintainable

See also 
[Visual-Representation-of-SQL-Joins](https://www.codeproject.com/Articles/33052/Visual-Representation-of-SQL-Joins)

```
from(first_table,joins)
{
// joins is AST or some kind of optimzed 'RPN'
let final_result =
	joins.iter().fold(first_table,|joined_sofar,the_next_join_clause|{
		let Joiner{next_table,join_operator}= the_next_join_clause;
		let swap =if join_operator.contains("right"){true}else{false};
		let result_to_fold = 
		if swap {joined_sofar}else{next_table}
		.iter()
		.map(|outer_row|){
			let the_join_on_one_left_with_all_on_right = 
				if swap {next_table}else{joined_sofar}.iter()
				.filter_map(|inner_row|{
					let compare = check(outer_row + inner_row);
					// return to filter
					if compare{Some(inner_row)}else{None}
				});
			// anything joined?
			// inner join ->skip if empty
			let inner_prod = 
				if the_join_on_one_left_with_all_on_right.len() != 0
				{
					if swap
						{prod(the_join_on_one_left_with_all_on_right, outer_row}
					else
						{prod(outer_row ,the_join_on_one_left_with_all_on_right}
				}
				else
				{
					if swap
						{prod('null', outer_row}
					else
						{prod(outer_row ,'null'}
				};
		};
		flatten(result_to_fold)
	});
	final_result
}
```