use super::super::super::dispatch_bindings::*;

/// Convert vector of [Any] to vector of string.
pub fn any_vec_to_strings(values: &Vec<Any>) -> Vec<String> {
    values.iter().map(|any| any.to_string()).collect()
}

/// Convert vector of [Any] to string.
pub fn any_vec_to_string(any: &Vec<Any>) -> String {
    any_vec_to_strings(any).join(",")
}
