use serde_json;
use std::fs::File;

fn main() {
    let path = "./arrays/1.json";
    let file = read_json(path);

    let array = parse_json(file);

    println!("{:?}", array);
    println!("{:?}", buble_sort(array));
}

fn read_json(path: &str) -> File {
    let file = File::open(path).unwrap();
    return file;
}

/**
 * Json format:
 *
 * {
 *  "unordered": [1, 2, 3, 4, 5],
 * }
 */

fn parse_json(file: File) -> Vec<i32> {
    let json: serde_json::Value = serde_json::from_reader(file).unwrap();
    let unordered = json["unordered"].as_array().unwrap();

    let mut array: Vec<i32> = Vec::new();

    for value in unordered {
        array.push(value.as_i64().unwrap() as i32);
    }

    return array;
}

fn buble_sort(array: Vec<i32>) -> Vec<i32> {
    let mut array = array;
    let mut n = array.len();
    let mut swapped = true;

    while swapped {
        swapped = false;
        for i in 1..n {
            if array[i - 1] > array[i] {
                array.swap(i - 1, i);
                swapped = true;
            }
        }
        n -= 1;
    }

    return array;
}
