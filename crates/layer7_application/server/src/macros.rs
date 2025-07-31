// This file contains macros for SQL queries. Since SQL-related functionality is being temporarily disabled,
// these macros are replaced with dummy implementations that will cause a panic if called.

#[macro_export]
macro_rules! query {
    ($($args:tt)*) => {
        panic!("FIX ME LATER: SQL queries are temporarily disabled. Fix the bootstrap first and this code later.");
    };
}

#[macro_export]
macro_rules! query_as {
    ($($args:tt)*) => {
        panic!("FIX ME LATER: SQL queries are temporarily disabled. Fix the bootstrap first and this code later.");
    };
}