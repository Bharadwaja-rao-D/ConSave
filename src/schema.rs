table! {
    post (post_id) {
        post_id -> Int4,
        title -> Text,
        content -> Text,
        user_id -> Int4,
    }
}

table! {
    user_info (id) {
        id -> Int4,
        name -> Text,
        password -> Text,
    }
}

joinable!(post -> user_info (user_id));

allow_tables_to_appear_in_same_query!(
    post,
    user_info,
);
