use std::collections::{HashMap};

// https://asonix.dog/@asonix/102226712612355882
pub fn is_all_same(vec: &Vec<String>) -> bool {
    let mut iter = vec.iter();

    let first = iter.next();

    iter.fold(first, |acc, item| {
        acc.and_then(|stored| if stored == item { Some(stored) } else { None })
    })
    .is_some()
}

pub fn frequency_map(directions: Vec<String>) -> HashMap<String, usize> {
    let mut map = HashMap::new();

    for n in directions {
        *map.entry(n).or_insert(0) += 1;
    }
    map
}

pub fn remove_lowest_occurence(vec: &mut Vec<String>) {
    let freq_map = frequency_map(vec.clone());

    let mut lowest_occurence_count: usize = 6;
    let mut lowest_occurence_key: String = String::new();
    for (key, value) in &freq_map {
        if value < &lowest_occurence_count {
            lowest_occurence_count = value.clone();
            lowest_occurence_key = key.clone();
        }
    }

    vec.retain(|x| *x != lowest_occurence_key.clone());
}

pub fn oposite(data: String) -> String {
    match data.as_str() {
        "up" => "down".into(),
        "down" => "up".into(),
        "left" => "right".into(),
        &_ => "left".into(),
    }
}
