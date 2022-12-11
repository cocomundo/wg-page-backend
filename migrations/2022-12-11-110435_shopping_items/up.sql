CREATE TABLE shopping_items (
    id serial NOT NULL,
    name character varying(255) NOT NULL,
    quantity int CHECK (quantity > 0),
    CONSTRAINT shopping_items_pkey PRIMARY KEY (id)
);