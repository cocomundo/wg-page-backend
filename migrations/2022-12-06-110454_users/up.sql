CREATE TABLE users (
    email character varying(255) NOT NULL,
    name character varying(255) NOT NULL,
    pwhash character varying(255) NOT NULL,
    CONSTRAINT users_pkey PRIMARY KEY (email)
);