DROP TABLE IF EXISTS account CASCADE;
DROP TABLE IF EXISTS category CASCADE;
DROP TABLE IF EXISTS entry CASCADE;

CREATE TABLE account
(
    id           UUID PRIMARY KEY,
    name         TEXT NOT NULL,
    clear_amount INT  NOT NULL DEFAULT (0)
);

CREATE TABLE category
(
    id   UUID PRIMARY KEY,
    name TEXT NOT NULL
);

CREATE TABLE clearing
(

);

CREATE TABLE entry
(
    id        UUID PRIMARY KEY,
    account   UUID      NOT NULL REFERENCES account (id),
    category  UUID REFERENCES category (id),
    timestamp TIMESTAMP NOT NULL,
    memo      TEXT      NOT NULL DEFAULT (''),
    amount    INT,
    cleared   BOOLEAN   NOT NUlL DEFAULT (FALSE)
);
SELECT now();


CREATE OR REPLACE PROCEDURE new_entry(
    p_id UUID,
    p_account UUID
)
    LANGUAGE plpgsql
AS
$$
BEGIN
    INSERT INTO entry (id, account, timestamp)
    VALUES (p_id, p_account, now());
END;
$$;

/*

ALTER TABLE data_field
    ADD CONSTRAINT fk_field_category
        FOREIGN KEY (category)
            REFERENCES category (id);

CREATE OR REPLACE PROCEDURE upsert_field(
    p_name TEXT,
    p_source UUID,
    p_table TEXT,
    p_types TEXT[],
    p_subfield BOOLEAN,
    p_nonce UUID
)
    LANGUAGE plpgsql
AS
$$
BEGIN
    INSERT INTO data_field(name, data_source, data_table, types, nonce, is_subfield)
    VALUES (p_name, p_source, p_table, p_types, p_nonce, p_subfield)
    ON CONFLICT ON CONSTRAINT data_field_pkey
        DO UPDATE SET types = p_types,
                      nonce = p_nonce;

END;
$$;

CREATE OR REPLACE FUNCTION get_fields_tmp(
    p_page_size INT,
    p_page_number INT
)
    RETURNS SETOF data_field
AS
$$
BEGIN
    RETURN QUERY
        SELECT df.*
        FROM data_field df
        ORDER BY df.data_source, df.data_table, df.name
        LIMIT p_page_size OFFSET (p_page_number - 1) * p_page_size;
END;
$$ LANGUAGE plpgsql;
*/