DROP TABLE IF EXISTS P;
CREATE TABLE P (
  PNO text,
  PNAME text,
  COLOR text,
  WEIGHT integer,
  CITY text
);

INSERT INTO P (PNO, PNAME, COLOR, WEIGHT, CITY) VALUES
("P1", "Nut", "Red", 12, "London"),
("P2", "Bolt", "Green", 17, "Paris"),
("P3", "Screw", "Blue", 17, "Rome"),
("P4", "Screw", "Red", 14, "London"),
("P5", "Cam", "Blue", 12, "Paris"),
("P6", "Cog", "Red", 19, "London");
