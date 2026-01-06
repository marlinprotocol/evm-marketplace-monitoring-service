diesel::table! {
    arbone_reachability_errors (id) {
        id -> Int4,
        job -> Varchar,
        operator -> Varchar,
        ip -> Varchar,
        error -> Varchar,
        timestamp -> Int8,
    }
}

diesel::table! {
    arbone_operator_errors (id) {
        id -> Int4,
        job -> Varchar,
        operator -> Varchar,
        ip -> Varchar,
        error -> Varchar,
        timestamp -> Int8,
    }
}
