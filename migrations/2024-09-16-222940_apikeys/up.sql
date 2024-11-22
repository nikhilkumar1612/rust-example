-- Your SQL goes here
CREATE TABLE apikeys(
    api_key VARCHAR(255) NOT NULL,
    wallet_address VARCHAR(255) NOT NULL,
    transaction_limit INTEGER NOT NULL,
    whitelisted_addresses VARCHAR(255),
    CONSTRAINT apikeys_pkey PRIMARY KEY (api_key)
);