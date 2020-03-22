table! {
    users (id) {
        id -> Int4,
        first_name -> Varchar,
        last_name -> Varchar,
        email -> Varchar,
        pseudo -> Varchar,
        password -> Varchar,
        activated -> Bool,
    }
}
