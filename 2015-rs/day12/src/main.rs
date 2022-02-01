use fancy_regex::Regex;
use serde_json::{self, Map, Number};

fn problem_1(input: &str) -> isize {
    input
        .lines()
        .map(|l| {
            Regex::new(r"(\-)?([0-9]+)")
                .unwrap()
                .find_iter(l)
                .map(|m| m.unwrap().as_str().parse::<isize>().unwrap())
                .sum::<isize>()
        })
        .sum::<isize>()
}

fn process_object(obj: Map<String, serde_json::Value>) -> isize {
    obj.values().map(|v| {
        return if v.is_number() {
            process_number(v.to_owned())
        } else if v.is_array() {
            process_array(v.as_array().unwrap().to_vec())
        } else if v.is_object() {
            process_object(v.as_object().unwrap().to_owned())
        } else {
            0
        };
    });
    0
}
fn process_array(arr: Vec<serde_json::Value>) -> isize {
    arr.iter()
        .map(|v| {
            return if v.is_number() {
                process_number(v.to_owned())
            } else if v.is_array() {
                process_array(v.as_array().unwrap().to_vec())
            } else if v.is_object() {
                process_object(v.as_object().unwrap().to_owned())
            } else {
                0
            };
        })
        .sum::<isize>()
}
fn process_number(val: serde_json::Value) -> isize {
    val.as_str().unwrap().parse::<isize>().unwrap()
}

fn problem_2(input: &str) -> isize {
    let json: serde_json::Value = serde_json::from_str(input).unwrap();

    return match json {
        serde_json::Value::Array(x) => process_array(x),
        serde_json::Value::Object(x) => process_object(x),
        _ => 0,
    };
}

fn main() {
    let input = include_str!("../input.txt");
    println!("Solution 1 -> {}", problem_1(input));
}

#[test]
fn test_slution_1() {
    assert_eq!(6, problem_1("[1,2,3]"));
    assert_eq!(6, problem_1(r#"{"a":2, "b":4}"#));

    assert_eq!(3, problem_1("[[[3]]]"));
    assert_eq!(3, problem_1(r#"{"a":{"b":4},"c":-1}"#));

    assert_eq!(0, problem_1(r#"{"a": [-1, 1]}"#));
    assert_eq!(0, problem_1(r#"[-1,{"a":1}]"#));

    assert_eq!(0, problem_1("[]"));
    assert_eq!(0, problem_1("{}"));
}

#[test]
fn test_solution_2() {
    assert_eq!(6, problem_2("[1,2,3]"));
    assert_eq!(6, problem_2(r#"[1,"red",5]"#));
    assert_eq!(0, problem_2(r#"{"d":"red","e":[1,2,3,4],"f":5}"#));
    assert_eq!(4, problem_2(r#"[1,{"c":"red","b":2},3]"#));
}
