CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS client(
       id UUID PRIMARY KEY,
       the_name VARCHAR(500) NOT NULL,
);


CREATE TABLE IF NOT EXISTS cargo(
       cargo_number  BIGINT  PRIMARY KEY,
       the_name VARCHAR(500) NOT NULL,
);

CREATE TABLE IF NOT EXISTS buy_bill(
       pill_number  BIGINT  PRIMARY KEY,
       client_id    UUID    NOT NULL,
       the_date     DATE    NOT NULL,
       tax_number   BIGINT  NOT NULL,
       the_value    BIGINT  NOT NULL,
       FOREIGN KEY(client_id) REFERENCES client(id)
);

CREATE TABLE IF NOT EXISTS sell_bill(
       pill_number  BIGINT  PRIMARY KEY,
       the_date     DATE    NOT NULL,
       tax_number   BIGINT  NOT NULL,
       the_value    BIGINT  NOT NULL,
       FOREIGN KEY(client_id) REFERENCES client(id)
);
