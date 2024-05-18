use std::collections::HashMap;

pub fn decode_to_hash_map(json_bytes: &[u8]) -> HashMap<String, usize> {
    let map: HashMap<String, usize> = serde_json::from_slice(json_bytes).unwrap();
    map
}

pub fn hash_map_to_json_bytes(map: HashMap<String, usize>) -> Vec<u8> {
    let json_string = serde_json::to_string(&map).unwrap();

    // Convert the JSON string to bytes
    let json_bytes = json_string.as_bytes();
    json_bytes.to_vec()
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::{decode_to_hash_map, hash_map_to_json_bytes};

    #[test]
    fn test_decode_to_hash_map() {
        let json_bytes = br#"{ "12345": 123, "22345": 355, "9762342343": 796987687 }"#;

        let map = decode_to_hash_map(json_bytes);

        for (key, value) in &map {
            println!("{}: {}", key, value);
        }
    }

    #[test]
    fn test1() {
        let mut map: HashMap<String, usize> = HashMap::new();
        map.insert(String::from("name"), 123);
        map.insert(String::from("age"), 456);
        map.insert(String::from("city"), 78009);

        let json_bytes = hash_map_to_json_bytes(map);

        let map = decode_to_hash_map(&json_bytes);

        for (key, value) in &map {
            println!("{}: {}", key, value);
        }
    }
}
