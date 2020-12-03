DROP TABLE IF EXISTS S;
CREATE TABLE S (
  SNO text,
  SNAME text,
  STATUS integer,
  CITY text
);
INSERT INTO S (SNO, SNAME, STATUS, CITY) VALUES
("S1", "Smith", 20, "London"),
("S2", "Jones", 10, "Paris"),
("S3", "Blake", 30, "Paris"),
("S4", "Clark", 20, "London"),
("S5", "Adams", 20, "Athens");
