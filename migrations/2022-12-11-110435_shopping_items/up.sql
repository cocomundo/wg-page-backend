CREATE TABLE shopping_items (
    id serial NOT NULL,
    name character varying(255) NOT NULL,
    quantity int NOT NULL,
    CONSTRAINT shopping_items_pkey PRIMARY KEY (id)
);