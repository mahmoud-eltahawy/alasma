CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS cargo(
       id UUID DEFAULT uuid_generate_v4() PRIMARY KEY NOT NULL,
       the_name VARCHAR(500),
       cargo_number  BIGINT
);

CREATE TABLE IF NOT EXISTS bill(
       bill_number  BIGINT  PRIMARY KEY,
       the_date     DATE
);

CREATE TABLE IF NOT EXISTS cargo_bill(
       id UUID DEFAULT uuid_generate_v4() PRIMARY KEY NOT NULL,
       cargo_id UUID,
       bill_number BIGINT,
       quantity BIGINT,
       one_cost DECIMAL,
       FOREIGN KEY(cargo_id) REFERENCES cargo(id),
       FOREIGN KEY(bill_number) REFERENCES bill(bill_number)
);

CREATE TABLE IF NOT EXISTS buy_bill(
       id UUID DEFAULT uuid_generate_v4() PRIMARY KEY NOT NULL,
       cargo_id UUID,
       bill_number BIGINT,
       quantity BIGINT,
       one_cost DECIMAL,
       FOREIGN KEY(cargo_id) REFERENCES cargo(id),
       FOREIGN KEY(bill_number) REFERENCES bill(bill_number)
);

CREATE TABLE IF NOT EXISTS client(
       id UUID DEFAULT uuid_generate_v4() PRIMARY KEY NOT NULL,
       cargo_id UUID,
       the_name VARCHAR(500)
);

CREATE TABLE IF NOT EXISTS company(
       id UUID PRIMARY KEY,
       the_name VARCHAR(500)
);

CREATE TABLE IF NOT EXISTS sell_bill(
       bill_number BIGINT PRIMARY KEY,
       tax_number BIGINT,
       company_id UUID,
       client_id UUID,
       total_cost DECIMAL,
       discount DECIMAL DEFAULT 0,
       FOREIGN KEY(bill_number) REFERENCES bill(bill_number),
       FOREIGN KEY(company_id) REFERENCES company(id),
       FOREIGN KEY(client_id) REFERENCES client(id)
);
