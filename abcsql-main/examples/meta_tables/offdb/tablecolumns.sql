DROP TABLE IF EXISTS tablecolumns;
CREATE TABLE tablecolumns (
  tablename text,
  columnname text,
  kind text,
  defaultvalue text,
  nonull  text,
  comment text,
);
INSERT INTO tablecolumns (tablename,columnname,kind,defaultvalue,nonull,comment) VALUES
("tables","tablename","text","","true","nocomment"),

("tablecolumns","tablename","text","","true","nocomment"),
("tablecolumns","columnname","text","","true","nocomment"),
("tablecolumns","kind","text","","true","nocomment"),
("tablecolumns","default","text","","true","nocomment"),
("tablecolumns","nonull","text","","true","nocomment"),

("indexes","tablename","text","","true","nocomment"),
("indexes","indexname","text","","true","nocomment"),
("indexes","indexkind","text","","true","nocomment"),

("indexmembers","tablename","text","","true","nocomment"),
("indexmembers","indexname","text","","true","nocomment"),
("indexmembers","columnname","text","","true","nocomment"),

("fk","tablename","text","","true","nocomment"),
("fk","fkname","text","","true","nocomment"),

("fkmembers","tablename","text","","true","nocomment"),
("fkmembers","fkname","text","","true","nocomment"),
("fkmembers","fromcolumn","text","","true","nocomment"),
("fkmembers","totable","text","","true","nocomment"),
("fkmembers","tocolumn","text","","true","nocomment");



