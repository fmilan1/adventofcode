use regex::Regex;

fn count(text: &str) -> i64 {
    let mut sum: i64 = 0;
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    for cap in re.captures_iter(&text) {
        let x: &i64 = &cap[1].parse::<i64>().unwrap();
        let y: &i64 = &cap[2].parse::<i64>().unwrap();
        sum += x * y;
    }
    return sum;
}

fn main() {
    let mut file = std::fs::read_to_string("input.txt").unwrap();
    println!("{}", count(&file));
    let mut changed: String = file.clone().split_whitespace().collect();
    file = changed.clone();
    let re = Regex::new(r"don't\(\)(.*?)do\(\)|don't\(\).*$").unwrap();
    for cap in re.captures_iter(&file) {
        let text = &cap[0];
        changed = changed.replace(text, "");
    }
    println!("{}", count(&changed));
}