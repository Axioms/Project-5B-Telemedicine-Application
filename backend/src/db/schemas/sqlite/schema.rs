table! {
    users (uuid) {
        uuid -> Char,
        username -> Varchar,
        password_hint -> Nullable<Text>,
        password_hash -> Blob,
        salt -> Blob,
        password_iterations -> Integer,
        kdf_type -> Integer,
        kdf_iterations -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        totp_enable -> Bool,
        akey -> Text,
        private_key -> Text,
        public_key -> Text,
        security_stamp -> Text,
        scope -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    users,
);