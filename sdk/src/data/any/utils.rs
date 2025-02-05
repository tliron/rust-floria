use super::super::super::dispatch_bindings::*;

/// Convert vector of normal values to vector of string.
pub fn normal_vec_to_strings(values: &Vec<Any>) -> Vec<String> {
    values.iter().map(|value| value.to_string()).collect()
}

/// Convert vector of normal values to string.
pub fn normal_vec_to_string(values: &Vec<Any>) -> String {
    normal_vec_to_strings(values).join(",")
}
