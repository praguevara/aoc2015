use serde_json::Value;

fn iterate_values<'a>(root: &'a Value) -> Box<dyn Iterator<Item = &Value> + 'a> {
    match root {
        Value::Array(a) => Box::new(a.iter().flat_map(iterate_values)),
        Value::Object(map) => Box::new(map.values().flat_map(iterate_values)),
        v => Box::new(std::iter::once(v)),
    }
}

fn iterate_values_skip_reds<'a>(root: &'a Value) -> Box<dyn Iterator<Item = &Value> + 'a> {
    match root {
        Value::Array(a) => Box::new(a.iter().flat_map(iterate_values_skip_reds)),
        Value::Object(map) => {
            if !map.values().any(|v| v == "red") {
                Box::new(map.values().flat_map(iterate_values_skip_reds))
            } else {
                Box::new(std::iter::empty())
            }
        }
        v => Box::new(std::iter::once(v)),
    }
}

fn extract_numbers<'a>(
    values: impl IntoIterator<Item = &'a Value> + 'a,
) -> impl Iterator<Item = i64> + 'a {
    values
        .into_iter()
        .filter_map(|v| match v {
            Value::Number(n) => Some(n.as_i64()),
            _ => None,
        })
        .flatten()
}

fn main() {
    let input = include_str!("../input.txt");
    let data: Value = serde_json::from_str(input).unwrap();

    let all_values = iterate_values(&data);
    let numbers = extract_numbers(all_values);
    println!("{}", numbers.sum::<i64>());

    let values_skip_reds = iterate_values_skip_reds(&data);
    let numbers_skip_reds = extract_numbers(values_skip_reds);
    println!("{}", numbers_skip_reds.sum::<i64>());
}
