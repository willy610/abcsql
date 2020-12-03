DROP TABLE IF EXISTS fkmembers;
CREATE TABLE fkmembers (
  tablename text,
  fkname text,
  fromcolumn text,
  totable text,
  tocolumn text
);
INSERT INTO fkmembers (tablename,fkname,fromcolumn,totable,tocolumn) VALUES
("tablecolumns","fk_tablename_tablename","tablename","tablename","tablename"),

("fk","fk_tablename_tablename","tablename","tablename","tablename"),

("indexes","fk_tablename_tablename","tablename","tablename","tablename"),

("indexesmembers","fk_partlyprimary_primary","tablename","indexes","tablename"),
("indexesmembers","fk_partlyprimary_primary","indexname","indexes","indexname"),

("fkmembers","fk_partlyprimary_primary","tablename","fk","tablename"),
("fkmembers","fk_partlyprimary_primary","fkname","fk","fkname")

;



