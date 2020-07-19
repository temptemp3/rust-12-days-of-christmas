

fn name(n: u8) -> String {
    if n == 1 { String::from("first") }
    else if n == 2  { String::from("second") }
    else if n == 3  { String::from("third") }
    else if n == 4  { String::from("fourth") }
    else if n == 5  { String::from("fifth") }
    else if n == 6  { String::from("sixth") }
    else if n == 7  { String::from("seventh") }
    else if n == 8  { String::from("eigjth") }
    else if n == 9  { String::from("ninth") }
    else if n == 10  { String::from("tenth") }
    else if n == 11  { String::from("eleventh") }
    else if n == 12  { String::from("twelfth") }
    else { String::from("") }
}

fn gift(n: u8) {
    if n == 1 { println!("And a partridge in a pear tree"); }
    if n == 2 { println!("Two turtle doves"); }
    if n == 3 { println!("Three French hens"); }
    if n == 4 { println!("Four calling birds"); }
    if n == 5 { println!("Five gold rings"); }
    if n == 6 { println!("Size geese a laying"); }
    if n == 7 { println!("Seven swans a swimming"); }
    if n == 8 { println!("Eight maids a milking"); }
    if n == 9 { println!("Nine ladies dancing"); }
    if n == 10 { println!("Ten lords a leaping"); }
    if n == 11 { println!("Eleven pipers piping"); }
    if n == 12 { println!("12 drummers drumming"); }
}

fn verse(n: u8) {
    if n == 1 {
        println!("On the {} day of Christmas", name(n));
        println!("My true lover gave to me");
        println!("A partridge in a pear tree")
    } else {
        println!("On the {} day of Christmas", name(n));
        println!("My true lover gave to me");
        gift(n);
        let mut _day: u8 = n - 1;
        while _day > 0 {
            gift(_day);
            _day -= 1;
        }
    }
}

fn main() {
    println!("Hello, world!");

    let mut day: u8 = 1;
    while day < 13 {
        verse(day);
        println!("");
        day += 1;
    }
}
