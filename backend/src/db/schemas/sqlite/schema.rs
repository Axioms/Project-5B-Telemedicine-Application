
table! {
    users (uuid) {
        uuid -> Char,
        created_at -> Timestamp,
        updated_at -> Timestamp,

        username -> Varchar,

        password_hash -> Blob,
        salt -> Blob,
        password_hint -> Nullable<Text>,

        akey -> Text,
        private_key -> Text,
        public_key -> Text,

        security_stamp -> Text,
        totp_enable -> Bool,

        password_iterations -> Integer,
        kdf_type -> Integer,
        kdf_iterations -> Integer,
        scope -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    users,
);