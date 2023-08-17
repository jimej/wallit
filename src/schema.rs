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
    login_history (id) {
        id -> Nullable<Integer>,
        company_id -> Text,
        username -> Text,
        password -> Text,
        history_id -> Integer,
    }
}

diesel::table! {
    logins (id) {
        id -> Integer,
        company_id -> Text,
        username -> Text,
        password -> Text,
        email -> Text,
        history_id -> Integer,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    companies,
    login_history,
    logins,
);
