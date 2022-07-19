CREATE TABLE IF NOT EXISTS txns
(
    hash VARCHAR(100) NOT NULL,
    block_number INT NOT NULL,
    from_address VARCHAR(44) NOT NULL,
    to_address VARCHAR(44),
    value VARCHAR(50) NOT NULL,
    error boolean NOT NULL DEFAULT false,
    fee VARCHAR(50) NOT NULL,
    CONSTRAINT txns_pkey PRIMARY KEY (hash)
);


CREATE TABLE IF NOT EXISTS token_transfers
(
    id SERIAL NOT NULL,
    tx_hash VARCHAR(100) ,
    block_number INT NOT NULL,
    value VARCHAR(50)  NOT NULL,
    from_address VARCHAR(44)  NOT NULL,
    to_address VARCHAR(44)  NOT NULL,
    token_name VARCHAR(255) ,
	token_ticker VARCHAR(255),
    token_address VARCHAR(50)  NOT NULL,
    decimals INT
);