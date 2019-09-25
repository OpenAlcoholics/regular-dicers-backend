-- Your SQL goes here
BEGIN TRANSACTION;
    INSERT INTO cocktails (name, jumbo, alcoholic) VALUES ('Pina Colada', False, True);
    INSERT INTO cocktails (name, jumbo, alcoholic) VALUES ('Strawberry Colada', False, True);
    INSERT INTO cocktails (name, jumbo, alcoholic) VALUES ('Swimming Pool', False, True);
    INSERT INTO cocktails (name, jumbo, alcoholic) VALUES ('Cuba Libre', False, True);
    INSERT INTO cocktails (name, jumbo, alcoholic) VALUES ('Mai Tai', False, True);
    INSERT INTO cocktails (name, jumbo, alcoholic) VALUES ('Mojito', False, True);
    INSERT INTO cocktails (name, jumbo, alcoholic) VALUES ('Strawberry Mojito', False, True);
    INSERT INTO cocktails (name, jumbo, alcoholic) VALUES ('Zombie', False, True);
    INSERT INTO cocktails (name, jumbo, alcoholic) VALUES ('Tequila Sunrise', False, True);
    INSERT INTO cocktails (name, jumbo, alcoholic) VALUES ('Strawberry Margarita', False, True);
    INSERT INTO cocktails (name, jumbo, alcoholic) VALUES ('Mango Margarita', False, True);
    INSERT INTO cocktails (name, jumbo, alcoholic) VALUES ('Gin Fizz', False, True);
    INSERT INTO cocktails (name, jumbo, alcoholic) VALUES ('Lady Killer', False, True);
    INSERT INTO cocktails (name, jumbo, alcoholic) VALUES ('Caipirinha', False, True);
    INSERT INTO cocktails (name, jumbo, alcoholic) VALUES ('Caipirinha Espanol', False, True);
    INSERT INTO cocktails (name, jumbo, alcoholic) VALUES ('Caipirol', False, True);
    INSERT INTO cocktails (name, jumbo, alcoholic) VALUES ('Strawberry Caipirovska', False, True);
    INSERT INTO cocktails (name, jumbo, alcoholic) VALUES ('Caipi Melon', False, True);
    INSERT INTO cocktails (name, jumbo, alcoholic) VALUES ('Caribbean Caipi', False, True);
    INSERT INTO cocktails (name, jumbo, alcoholic) VALUES ('Sex on the Beach', False, True);
    INSERT INTO cocktails (name, jumbo, alcoholic) VALUES ('Touchdown', False, True);
    -- jumbos
    INSERT INTO cocktails (name, jumbo, alcoholic) VALUES ('Big Ben', True, True);
    INSERT INTO cocktails (name, jumbo, alcoholic) VALUES ('Captain''s Hurricane', True, True);
    INSERT INTO cocktails (name, jumbo, alcoholic) VALUES ('Cloud 9', True, True);
    INSERT INTO cocktails (name, jumbo, alcoholic) VALUES ('Enchilada Cooler', True, True);
    INSERT INTO cocktails (name, jumbo, alcoholic) VALUES ('Killer Cool Aid', True, True);
    INSERT INTO cocktails (name, jumbo, alcoholic) VALUES ('Long Island Mint', True, True);
    INSERT INTO cocktails (name, jumbo, alcoholic) VALUES ('Long Paloma', True, True);
    INSERT INTO cocktails (name, jumbo, alcoholic) VALUES ('Long Island Bull', True, True);
    INSERT INTO cocktails (name, jumbo, alcoholic) VALUES ('Long Island Ice Tea', True, True);
    INSERT INTO cocktails (name, jumbo, alcoholic) VALUES ('Long Island Beach', True, True);
    INSERT INTO cocktails (name, jumbo, alcoholic) VALUES ('Lectric Lemonade', True, True);
    -- pussy drinks (jumbo)
    INSERT INTO cocktails (name, jumbo, alcoholic) VALUES ('Cala Serena', True, False);
    INSERT INTO cocktails (name, jumbo, alcoholic) VALUES ('Fresh Summer', True, False);
    -- pussy drinks (normal)
    INSERT INTO cocktails (name, jumbo, alcoholic) VALUES ('Spring Fever', False, False);
    INSERT INTO cocktails (name, jumbo, alcoholic) VALUES ('Coconut Kiss', False, False);
    INSERT INTO cocktails (name, jumbo, alcoholic) VALUES ('Virgin Strawberry', False, False);
    INSERT INTO cocktails (name, jumbo, alcoholic) VALUES ('Caipi Ginger', False, False);
END;
