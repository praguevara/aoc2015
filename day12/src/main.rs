use serde_json::Value;

fn traverse_values(input: &Value) -> Vec<Value> {
    match input {
        Value::Array(a) => a.iter().flat_map(traverse_values).collect(),
        Value::Object(map) => map.values().flat_map(traverse_values).collect(),
        v => vec![v.clone()],
    }
}

fn traverse_values_skip_reds(input: &Value) -> Vec<Value> {
    match input {
        Value::Array(a) => a.iter().flat_map(traverse_values_skip_reds).collect(),
        Value::Object(map) => {
            if !map.values().any(|v| v == "red") {
                map.values().flat_map(traverse_values_skip_reds).collect()
            } else {
                vec![]
            }
        }
        v => vec![v.clone()],
    }
}

fn extract_numbers(values: &[Value]) -> Vec<i64> {
    values
        .iter()
        .filter_map(|v| match v {
            Value::Number(n) => Some(n.as_i64()),
            _ => None,
        })
        .flatten()
        .collect()
}

fn main() {
    let input = include_str!("../input.txt");
    let data: Value = serde_json::from_str(input).unwrap();

    let all_values = traverse_values(&data);
    let numbers = extract_numbers(&all_values);
    println!("{}", numbers.iter().sum::<i64>());

    let values_skip_reds = traverse_values_skip_reds(&data);
    let numbers_skip_reds = extract_numbers(&values_skip_reds);
    println!("{}", numbers_skip_reds.iter().sum::<i64>());

    dbg!(numbers.len(), numbers_skip_reds.len());
}
