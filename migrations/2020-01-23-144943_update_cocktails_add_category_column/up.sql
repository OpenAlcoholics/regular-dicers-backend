-- Your SQL goes here
BEGIN TRANSACTION;

ALTER TABLE cocktails
    ADD COLUMN category SMALLINT NOT NULL DEFAULT 0;

UPDATE cocktails
SET category = 1
WHERE name = 'Big Ben'
   OR name = 'Cala Serena'
   OR name = 'Captain''s Hurricane'
   OR name = 'Cloud 9'
   OR name = 'Enchilada Cooler'
   OR name = 'Fresh Summer'
   OR name = 'Killer Cool Aid'
   OR name = 'Long Island Mint'
   OR name = 'Long Paloma'
   OR name = 'Long Island Bull'
   OR name = 'Long Island Ice Tea'
   OR name = 'Long Island Beach'
   OR name = 'Lectric Lemonade'
   OR name = 'Spring Fever';
UPDATE cocktails
SET category = 2
WHERE name = 'Sex on the Beach'
   OR name = 'Touchdown';
UPDATE cocktails
SET category = 3
WHERE name = 'Pina Colada'
   OR name = 'Strawberry Colada'
   OR name = 'Swimming Pool'
   OR name = 'Coconut Kiss';
UPDATE cocktails
SET category = 4
WHERE name = 'Cuba Libre'
   OR name = 'Mai Tai'
   OR name = 'Mojito'
   OR name = 'Strawberry Mojito'
   OR name = 'Zombie';
UPDATE cocktails
SET category = 5
WHERE name = 'Tequila Sunrise';
UPDATE cocktails
SET category = 6
WHERE name = 'Strawberry Margarita'
   OR name = 'Mango Margarita'
   OR name = 'Virgin Strawberry';
UPDATE cocktails
SET category = 7
WHERE name = 'Gin Fizz'
   OR name = 'Lady Killer';
END;
