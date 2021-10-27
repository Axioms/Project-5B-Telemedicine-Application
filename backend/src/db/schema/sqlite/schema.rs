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
        username -> Text,
        password_hint -> Nullable<Text>,
        password_hash -> Binary,
        salt -> Binary,
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
    reports,
    users,
);
