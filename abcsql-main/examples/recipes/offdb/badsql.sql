SELECT energy
         FROM (SELECT energy FROM ingredient) 
         WHERE energy > 400 ;

30 seconds

SELECT recipe.recipename,ingredientid,ingredient.ingredientname FROM recipestepingredient join recipe 
on recipestepingredient.recipeid = recipe.recipeid join ingredient
on recipestepingredient.ingredientid = ingredient.ingredientid
        WHERE ingredientid = 38 
        order BY 1;

        slow
        
       SELECT recipename 
        FROM   recipe 
               JOIN recipestepingredient 
                 ON recipe.recipeid = recipestepingredient.recipeid 
        WHERE  ingredientid = (SELECT ingredientid 
                               FROM   ingredient 
                               WHERE  ingredientname = 'mjölk') ;
