diesel::table! {
    reachability_errors (id) {
        id -> Int4,
        job -> Varchar,
        operator -> Varchar,
        ip -> Varchar,
        error -> Varchar,
        timestamp -> Int8,
    }
}