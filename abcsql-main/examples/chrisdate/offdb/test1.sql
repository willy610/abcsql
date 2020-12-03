select * from S order by 1;
select * from SP order by "PNO";
select SNO,PNO, QTY/3 from SP;
select * from S left join P on S.CITY = P.CITY;

SELECT * FROM P;
SELECT * FROM SP LEFT JOIN S ON S.SNO = SP.SNO;
SELECT * FROM S LEFT JOIN SP ON S.SNO = SP.SNO;
select * from S FULL JOIN SP ON S.SNO = PNO;
select * from S FULL JOIN SP ON SP.SNO = S.SNO;
select * from S AS ESS LEFT JOIN SP AS ESSPEE ON ESS.SNO = ESSPEE.SNO;
select * from S FULL JOIN SP using (SNO);

select * from S LEFT JOIN SP ON S.SNO = SP.SNO and S.CITY = SP.PNO;
select * from S LEFT JOIN SP ON S.SNO = SP.SNO and S.CITY = SP.PNO and S.SNAME = SP.PNO;
select * from S LEFT JOIN P ON S.CITY = P.CITY JOIN J on  S.CITY = J.CITY;
select * from S JOIN P ON S.CITY = P.CITY  AND S.STATUS = P.WEIGHT JOIN J on  P.CITY = J.CITY;
SELECT * FROM SP AS SPECIAL;
SELECT * FROM P;
SELECT SNO FROM SP;
SELECT PNO FROM P;
select * from S NATURAL JOIN S;
