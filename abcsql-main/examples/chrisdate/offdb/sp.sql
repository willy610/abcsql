DROP TABLE IF EXISTS SP;
CREATE TABLE SP (
  SNO text,
  PNO text,
  QTY integer
);
INSERT INTO SP (SNO, PNO, QTY) VALUES
("S1", "P1", 300),
("S1", "P2", 200),
("S1", "P3", 400),
("S1", "P4", 200),
("S1", "P5", 100),
("S1", "P6", 100),
("S2", "P1", 300),
("S2", "P2", 400),
("S3", "P2", 200),
("S4", "P2", 200),
("S4", "P4", 300),
("S4", "P5", 400);
