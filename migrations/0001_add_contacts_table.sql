CREATE TABLE Contact (
    contact_id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    phone_number TEXT,
    email TEXT,
    address TEXT,
    birthday TEXT
);
