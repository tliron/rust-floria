use super::super::dispatch_bindings::Value;

/// Convert vector of normal values to vector of string.
pub fn normal_vec_to_strings(values: &Vec<Value>) -> Vec<String> {
    values.iter().map(|v| v.to_string()).collect()
}

/// Convert vector of normal values to string.
pub fn normal_vec_to_string(values: &Vec<Value>) -> String {
    normal_vec_to_strings(values).join(",")
}
