use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

#[derive(Debug)]
struct Transform {
    input: usize,
    output: usize,
    new_char: usize,
    new_tf: [usize; 2],
}

fn find_or_add_key_to_count_index(new_key: String, index_vec: &mut HashMap<String, usize>, count_vec: &mut Vec<usize>) -> usize {
    let new_index: usize;
    if !index_vec.contains_key(&new_key.clone()) {
        new_index = count_vec.len();
        index_vec.insert(new_key.clone(), new_index);
        count_vec.push(0);
    }
    else {
        new_index = *index_vec.get(&new_key).unwrap();
    }
    new_index
}

fn find_or_add_key_to_tf_index(new_key: String, index_vec: &mut HashMap<String, usize>, tf_vec: &mut Vec<Transform>) -> usize {
    let new_index: usize;
    if !index_vec.contains_key(&new_key.clone()) {
        new_index = tf_vec.len();
        index_vec.insert(new_key.clone(), new_index);
        tf_vec.push( Transform { input: 0, output: 0, new_char: 0, new_tf: [0, 0] });
    }
    else {
        new_index = *index_vec.get(&new_key).unwrap();
    }
    new_index
}

fn main() {
    let f = File::open("./input.txt").expect("Unable to open input file");
    let f = BufReader::new(f);

    // Line input handling
    let mut line_count: usize = 0;
    let mut input_chars: Vec<char> = Vec::new();

    // String -> index -> count mappings
    let mut input_count_map: HashMap<String, usize> = HashMap::new();
    let mut input_count: Vec<usize> = Vec::new();

    let mut output_count_map: HashMap<String, usize> = HashMap::new();
    let mut output_count: Vec<usize> = Vec::new();

    let mut char_count_map: HashMap<String, usize> = HashMap::new();
    let mut char_count: Vec<usize> = Vec::new();

    // The transforms themselves
    let mut tf_map: HashMap<String, usize> = HashMap::new();
    let mut tf_rev_map: HashMap<String, usize> = HashMap::new();
    let mut tf_vec: Vec<Transform> = Vec::new();

    for line in f.lines() {
        let line = line.expect("Unable to read line");
        
        line_count += 1;

        if line_count == 1 {        
            input_chars = line.trim().chars().collect();
            continue;
        }
        
        if line.trim().is_empty() {
            continue;
        }

        let tf_pieces: Vec<&str> = line.trim().split(" -> ").collect();

        // Add input pair
        let input_index = find_or_add_key_to_count_index(String::from(tf_pieces[0].trim()), &mut input_count_map, &mut input_count);

        // Add inserted char
        let char_index = find_or_add_key_to_count_index(String::from(tf_pieces[1].trim()), &mut char_count_map, &mut char_count);

        // Add output triple
        let in_pieces: Vec<char> = tf_pieces[0].chars().collect();
        let triple = format!("{}{}{}", in_pieces[0], tf_pieces[1].trim(), in_pieces[1]);
        let output_index = find_or_add_key_to_count_index(triple.clone(), &mut output_count_map, &mut output_count);

        // Add input pieces
        find_or_add_key_to_count_index(String::from(in_pieces[0]), &mut char_count_map, &mut char_count);
        find_or_add_key_to_count_index(String::from(in_pieces[1]), &mut char_count_map, &mut char_count);

        // Add new transforms
        let new_tf_s: [String; 2] = [format!("{}{}", in_pieces[0], tf_pieces[1].trim()), format!("{}{}", tf_pieces[1].trim(), in_pieces[1])];
        let mut new_tf: [usize; 2] = [0, 0];

        for i in 0..2 {
            new_tf[i] = find_or_add_key_to_count_index(new_tf_s[i].clone(), &mut input_count_map, &mut input_count);
        }

        // Add this transform
        let my_tf = find_or_add_key_to_tf_index(String::from(tf_pieces[0].trim()), &mut tf_map, &mut tf_vec);

        tf_vec[my_tf].input = input_index;
        tf_vec[my_tf].new_char = char_index;
        tf_vec[my_tf].output = output_index;
        tf_vec[my_tf].new_tf = new_tf;

        // Add reverse triple
        tf_rev_map.insert(triple, my_tf);
    }

    // Mark each pair in the input string as present and set initial character counts
    let inchar_len = input_chars.len();
    for i in 0..inchar_len {
        if i < inchar_len - 1 {
            let pair = format!("{}{}", input_chars[i], input_chars[i+1]);
            let pair_count_idx = *input_count_map.get(&pair).unwrap();
            input_count[pair_count_idx] += 1;
        }
        let char_key = format!("{}", input_chars[i]);
        let char_count_idx = *char_count_map.get(&char_key).unwrap();
        char_count[char_count_idx] += 1;
    }

    let steps: usize = 40;

    /*
    for (tf, index) in &input_count_map {
        println!("{} - {}", tf, input_count[*index]);
    }
    */

    for _step in 1..=steps {

        // Go through each of the input transforms
        // Increase char counts and zero the input transform count
        for i in 0..tf_vec.len() {
            if input_count[tf_vec[i].input] > 0 {
                output_count[tf_vec[i].output] += input_count[tf_vec[i].input];
                char_count[tf_vec[i].new_char] += input_count[tf_vec[i].input];

                input_count[tf_vec[i].input] = 0
            }
        }

        /*
        for (tf, index) in &output_count_map {
            println!("{} - {}", tf, output_count[*index]);
        }
        */

        // Go through the triples and set the new input transform counts
        for i in 0..tf_vec.len() {
            if output_count[tf_vec[i].output] > 0 {
                input_count[tf_vec[i].new_tf[0]] += output_count[tf_vec[i].output];
                input_count[tf_vec[i].new_tf[1]] += output_count[tf_vec[i].output];

                output_count[tf_vec[i].output] = 0;
            }
        }

        /*
        for (tf, index) in &input_count_map {
            println!("{} - {}", tf, input_count[*index]);
        }
        */
    }

    let mut total_chars: usize = 0;
    let mut max_size: usize = 0;
    let mut min_size: usize = 0;
    let mut max_str: String = String::from("");
    let mut min_str: String = String::from("");

    for (letter, index) in char_count_map {
        println!("{} - {}", letter, char_count[index]);
        total_chars += char_count[index];

        if char_count[index] < min_size || min_size == 0 {
            min_size = char_count[index];
            min_str = letter;
        } 
        else if char_count[index] > max_size {
            max_size = char_count[index];
            max_str = letter;
        }
    }

    println!("After {} steps, Total chars: {}, Max char: {} - {}, Min char: {} - {}, Diff: {}", steps, total_chars, max_str, max_size, min_str, min_size, max_size - min_size);
    
}