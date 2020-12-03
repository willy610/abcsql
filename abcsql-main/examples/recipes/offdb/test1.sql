select * from category;
select COUNT(*) from category;

SELECT ingredientid,ingredientid+2000 as PLUS2000 FROM ingredient 
WHERE energy > 500 
ORDER BY 1;

        SELECT recipeid,recipename,maketime,created FROM recipe 
        WHERE maketime > 100 
        order BY 1;

SELECT * FROM recipestep 
        WHERE minutes > 100 
        order BY 1;




        SELECT ingredientid AS NEWCOLNAME 
        FROM   ingredient 
        WHERE  ingredientname = 'mjölk';


 


        SELECT categoryid,
               Count(*) 
        FROM   categoryandrecipe 
        GROUP  BY categoryid 
        ORDER BY 2;

select recipename,max(minutes),min(minutes) from
recipe join recipestep on recipe.recipeid=recipestep.recipeid
group by recipename