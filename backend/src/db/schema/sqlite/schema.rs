table! {
    reports (uuid) {
        uuid -> Text,
        patient_uuid -> Text,
        provider_uuid -> Text,
        created_at -> Timestamp,
        report -> Text,
    }
}

table! {
    users (uuid) {
        uuid -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        username -> Text,
        password_hash -> Binary,
        salt -> Binary,
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
    reports,
    users,
);
