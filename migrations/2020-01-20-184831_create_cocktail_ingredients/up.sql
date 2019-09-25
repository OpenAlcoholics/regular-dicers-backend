-- Your SQL goes here
CREATE TABLE IF NOT EXISTS cocktail_ingredients
(
    id            SERIAL UNIQUE PRIMARY KEY,
    cocktail_id   Integer NOT NULL,
    ingredient_id Integer NOT NULL,
    UNIQUE (cocktail_id, ingredient_id),
    FOREIGN KEY (ingredient_id) REFERENCES ingredients (id),
    FOREIGN KEY (cocktail_id) REFERENCES cocktails (id)
);
