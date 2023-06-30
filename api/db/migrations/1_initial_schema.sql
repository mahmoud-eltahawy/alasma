CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS bill(
       id UUID DEFAULT uuid_generate_v4() PRIMARY KEY NOT NULL,
       bill_number  BIGINT,
       the_date     DATE,
       is_sell      BOOLEAN NOT NULL
);

CREATE TABLE IF NOT EXISTS cargo (
    id UUID DEFAULT uuid_generate_v4() PRIMARY KEY NOT NULL,
    cargo_name VARCHAR(300),
    cargo_number BIGINT
);

CREATE TABLE IF NOT EXISTS cargo_bill(
       id UUID DEFAULT uuid_generate_v4() PRIMARY KEY NOT NULL,
       cargo_id UUID,
       bill_id UUID,
       quantity BIGINT,
       one_cost DECIMAL,
       FOREIGN KEY(bill_id) REFERENCES bill(id),
       FOREIGN KEY(cargo_id) REFERENCES cargo(id)
);

CREATE TABLE IF NOT EXISTS buy_bill(
       id UUID DEFAULT uuid_generate_v4() PRIMARY KEY NOT NULL,
       cargo_name VARCHAR(300),
       bill_id UUID,
       quantity BIGINT,
       one_cost DECIMAL,
       FOREIGN KEY(bill_id) REFERENCES bill(id)
);

CREATE TABLE IF NOT EXISTS client(
       id UUID DEFAULT uuid_generate_v4() PRIMARY KEY NOT NULL,
       cargo_id UUID,
       the_name VARCHAR(500)
);

CREATE TABLE IF NOT EXISTS company(
       id UUID DEFAULT uuid_generate_v4() PRIMARY KEY NOT NULL,
       the_name VARCHAR(500)
);

CREATE TABLE IF NOT EXISTS sell_bill(
       bill_id UUID PRIMARY KEY,
       tax_number BIGINT,
       company_id UUID,
       client_id UUID,
       total_cost DECIMAL,
       discount DECIMAL DEFAULT 0 NOT NULL,
       FOREIGN KEY(bill_id) REFERENCES bill(id),
       FOREIGN KEY(company_id) REFERENCES company(id),
       FOREIGN KEY(client_id) REFERENCES client(id)
);
