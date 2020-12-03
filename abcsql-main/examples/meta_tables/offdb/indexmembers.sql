DROP TABLE IF EXISTS indexmembers;
CREATE TABLE indexmembers (
  tablename text,
  indexname text,
  columnname text
);
INSERT INTO indexmembers (tablename,indexname,columnname) VALUES
("tables","primary","tablename"),

("tablecolumns","primary","tablename"),
("tablecolumns","primary","columnname"),

("indexes","primary","tablename"),
("indexes","primary","indexname"),

("indexmembers","primary","tablename"),
("indexmembers","primary","indexname"),
("indexmembers","primary","columnname"),

("fk","primary","tablename"),
("fk","primary","fkname"),

("fkmembers","primary","tablename"),
("fkmembers","primary","fkname"),
("fkmembers","primary","fromcolumnname");

