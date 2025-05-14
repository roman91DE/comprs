use std::collections::HashMap;

fn build_freq_table(s: &str) -> HashMap<char, usize> {
    let mut map: HashMap<char, usize> = HashMap::new();

    for c in s.chars() {
        if map.contains_key(&c) {
            let old = map.get(&c).expect("can't happen");
            let _ = map.insert(c, old + 1);
        } else {
            let _ = map.insert(c, 1);
        }
    }
    map
}

fn build_mapping(freq: HashMap<char, usize>) -> Result<HashMap<char, u8>, String> {
    let mut freq_vec: Vec<(char, usize)> = freq.into_iter().collect();
    if freq_vec.len() >= u8::MAX as usize {
        return Err(String::from("Too many unique Characters"));
    }
    freq_vec.sort_by(|a, b| b.1.cmp(&a.1)); // highest frequency first

    let result = freq_vec
        .into_iter()
        .enumerate()
        .map(|(i, (ch, _))| (ch, u8::try_from(i).expect("can't happen")))
        .collect();

    Ok(result)
}

fn encode(input: &str, mapping: &HashMap<char, u8>) -> Result<Vec<u8>, String> {
    let mut result: Vec<u8> = input
        .chars()
        .map(|c| mapping.get(&c).copied().expect("Something went wrong..."))
        .collect();
    
    result.shrink_to_fit();
    Ok(result)
}

fn decompress(data: &[u8], reverse_map: &HashMap<u8, char>) -> Result<String, String> {
    let mut result = String::new();
    for byte in data {
        match reverse_map.get(byte) {
            Some(&ch) => result.push(ch),
            None => return Err(format!("Invalid byte: {}", byte)),
        }
    }
    Ok(result)
}

fn main() -> Result<(), String> {
    let s = r#"
😀😁😂🤣😃😄😅😆😉😊😋😎😍😘🥰😗😙😚🙂🤗🤩🤔🤨😐😑😶🙄😏
🥲🫠🫢🫣🤐🫡🫥🤑😲☹️🙁😖😣😞😓😩😫😢😭😤😠😡🤬🤯😳🥵🥶😶‍🌫️
😀😁😂🤣😃😄😅😆😉😊😋😎😍😘🥰😗😙😚🙂🤗🤩🤔🤨😐😑😶🙄😏
🥲🫠🫢🫣🤐🫡🫥🤑😲☹️🙁😖😣😞😓😩😫😢😭😤😠😡🤬🤯😳🥵🥶😶‍🌫️
😀😁😂🤣😃😄😅😆😉😊😋😎😍😘🥰😗😙😚🙂🤗🤩🤔🤨😐😑😶🙄😏
🥲🫠🫢🫣🤐🫡🫥🤑😲☹️🙁😖😣😞😓😩😫😢😭😤😠😡🤬🤯😳🥵🥶😶‍🌫️
"#;


    println!("Input: {s:?}");
    let stack_input = std::mem::size_of_val(&s);
    let heap_input = s.len();

    let freqs: HashMap<char, usize> = build_freq_table(s);
    println!("Frequency-Table: {freqs:?}");

    let mapping = build_mapping(freqs)?;
    println!("Mapping: {mapping:?}");


    let encoded = encode(s, &mapping)?;
    let stack_output = std::mem::size_of_val(&encoded);
    let heap_output = encoded.len();
    println!("Encoded: {encoded:?}");

    let reverse_map: HashMap<u8, char> = mapping.iter().map(|(&k, &v)| (v, k)).collect();
    let decoded = decompress(&encoded, &reverse_map)?;
    assert_eq!(s, decoded);

    println!("Stack Memory Input: {stack_input}");
    println!("Stack Memory Output: {stack_output}");
    println!("Heap Memory Input (UTF-8 bytes): {heap_input}");
    println!("Heap Memory Output (compressed bytes): {heap_output}");



    Ok(())
}
