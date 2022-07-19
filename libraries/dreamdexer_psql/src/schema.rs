table! {
    token_transfers (id) {
        id -> Int4,
        tx_hash -> Nullable<Varchar>,
        block_number -> Int4,
        value -> Varchar,
        from_address -> Varchar,
        to_address -> Varchar,
        token_name -> Nullable<Varchar>,
        token_address -> Varchar,
        decimals -> Nullable<Int4>,
    }
}

table! {
    txns (hash) {
        hash -> Varchar,
        block_number -> Int4,
        from_address -> Varchar,
        to_address -> Nullable<Varchar>,
        value -> Varchar,
        error -> Bool,
        fee -> Varchar,
    }
}

joinable!(token_transfers -> txns (tx_hash));

allow_tables_to_appear_in_same_query!(
    token_transfers,
    txns,
);
