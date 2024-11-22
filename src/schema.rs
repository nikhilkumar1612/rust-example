// @generated automatically by Diesel CLI.

diesel::table! {
    apikeys (api_key) {
        #[max_length = 255]
        api_key -> Varchar,
        #[max_length = 255]
        wallet_address -> Varchar,
        transaction_limit -> Int4,
        #[max_length = 255]
        whitelisted_addresses -> Nullable<Varchar>,
    }
}
