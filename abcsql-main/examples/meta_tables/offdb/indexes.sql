DROP TABLE IF EXISTS indexes;
CREATE TABLE indexes (
  tablename text,
  indexname text,
  indexkind text
);
INSERT INTO indexes (tablename,indexname,indexkind) VALUES
("tables","primary","unique"),
("tablecolumns","primary","unique"),
("indexes","primary","unique"),
("indexmembers","primary","unique"),
("fk","primary","unique"),
("fkmembers","primary","unique")
;


