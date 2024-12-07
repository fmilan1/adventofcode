fn main() {
    let file = std::fs::read_to_string("input.txt").unwrap();
    let mut safe: i32 = 0;
    for line in file.lines() {
        let numbers: Vec<&str> = line.split(" ").collect();
        let mut asc = numbers.clone();
        asc.sort();
        if (numbers == asd)
        for i in 1..numbers.len() {
            let dif = (numbers[i - 1].parse::<i32>().unwrap() - numbers[i].parse::<i32>().unwrap()).abs();
            if dif <= 3 && dif >= 1 {
                if i == numbers.len() - 1 {
                    safe += 1;
                }
            }
        }
    }
    println!("{safe}");
}
szoval, koszi
print ("Szia Világ!")
Nyomtasd ki
bibibibibibibibi
Meg is nyertem
huhuuhuhuhu
nyárvégi vásááááááááár
túl sok space ez
tokeletes k