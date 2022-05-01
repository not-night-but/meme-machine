CREATE TABLE memes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    path TEXT NOT NULL
);

CREATE TABLE templates (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    image_path TEXT NOT NULL,
    text_instances TEXT NOT NULL,
    text_color TEXT NOT NULL,
    text_scale TEXT NOT NULL
);

INSERT INTO templates (name, image_path, text_instances, text_color, text_scale) VALUES
    ('flextape', '/templates/flextape.jpg', '[[1798, 583, 20], [1633, 2431, 20]]', '[0, 0, 0]', '[120, 120]'),
    ('sword', '/templates/sword.jpg', '[[57, 50, 18], [91, 300, 17]]', '[255, 255, 255]', '[50, 50]'),
    ('useless-paper', '/templates/uselesspaper.jpg', '[[87, 165, 12]]', '[0, 0, 0]', '[40, 40]');