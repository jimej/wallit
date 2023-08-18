// @generated automatically by Diesel CLI.

diesel::table! {
    companies (id) {
        id -> Integer,
        company_id -> Text,
        url -> Text,
        testcol -> Text,
    }
}

diesel::table! {
    history (id) {
        id -> Nullable<Integer>,
        company_id -> Text,
        login -> Text,
        password -> Text,
        history_id -> Integer,
        url -> Nullable<Text>,
        description -> Nullable<Text>,
        loginLastModified -> Nullable<Text>,
        mode -> Nullable<Text>,
        lastModified -> Nullable<Text>,
    }
}

diesel::table! {
    logins (id) {
        id -> Integer,
        company_id -> Text,
        login -> Text,
        password -> Text,
        email -> Text,
        url -> Text,
        description -> Text,
        lastModified -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    companies,
    history,
    logins,
);
