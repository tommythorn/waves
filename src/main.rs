/*
 * Sandbox for ways to visualize waves in the terminal
 * -- Tommy, 20210226
 *
 * $ cargo run -q
 * A 0101011101010110
 * B 1101101011100101
 * X 1000110110110011
 *
 * A _▇_▇_▇▇▇_▇_▇_▇▇_
 * B ▇▇_▇▇_▇_▇▇▇__▇_▇
 * X ▇___▇▇_▇▇_▇▇__▇▇
 *
 * A __/⎺\_/⎺\_/⎺⎺⎺⎺⎺\_/⎺\_/⎺\_/⎺⎺⎺\_
 * B /⎺⎺⎺\_/⎺⎺⎺\_/⎺\_/⎺⎺⎺⎺⎺\___/⎺\_/⎺
 * X /⎺\_____/⎺⎺⎺\_/⎺⎺⎺\_/⎺⎺⎺\___/⎺⎺⎺
 *
 *     ┌─┐ ┌─┐ ┌─────┐ ┌─┐ ┌─┐ ┌───┐
 * A ──┘ └─┘ └─┘     └─┘ └─┘ └─┘   └─
 *   ┌───┐ ┌───┐ ┌─┐ ┌─────┐   ┌─┐ ┌─
 * B ┘   └─┘   └─┘ └─┘     └───┘ └─┘
 *   ┌─┐     ┌───┐ ┌───┐ ┌───┐   ┌───
 * X ┘ └─────┘   └─┘   └─┘   └───┘
 *
 */

pub fn show_pulse0(signal: &str, p: &str) {
    print!("{} ", signal);
    for v in p.chars() {
        print!("{}", v);
    }
    println!();
}

pub fn show_pulse1(signal: &str, p: &str) {
    print!("{} ", signal);

    for v in p.chars() {
        match v {
            '0' => print!("_"),
            '1' => print!("▇"),
            _ => panic!(),
        }
    }
    println!();
}

pub fn show_pulse2(signal: &str, p: &str) {
    print!("{} ", signal);
    let mut prev = '0';
    for v in p.chars() {
        match (prev, v) {
            ('0', '0') => print!("__"),
            ('0', '1') => print!("/⎺"),
            ('1', '0') => print!("\\_"),
            ('1', '1') => print!("\u{23ba}\u{23ba}"),
            _ => panic!(),
        }
        prev = v;
    }
    println!();
}

pub fn show_pulse3(signal: &str, p: &str) {
    let mut top = String::new();
    let mut bot = String::new();
    let mut prev = '0';

    //   ┐ ┌─
    //   └─┘

    for v in p.chars() {
        match (prev, v) {
            ('0', '0') => {
                top.push_str("  ");
                bot.push_str("──");
            }
            ('0', '1') => {
                top.push_str("┌─");
                bot.push_str("┘ ");
            }
            ('1', '0') => {
                top.push_str("┐ ");
                bot.push_str("└─");
            }
            ('1', '1') => {
                top.push_str("──");
                bot.push_str("  ");
            }
            _ => panic!(),
        }
        prev = v;
    }
    println!("  {}", top);
    println!("{} {}", signal, bot);
}

fn xor(a: char, b: char) -> char {
    match (a, b) {
        ('0', '0') => '0',
        ('0', '1') => '1',
        ('1', '0') => '1',
        ('1', '1') => '0',
        _ => panic!(),
    }
}

fn main() {
    let a = "0101011101010110";
    let b = "1101101011100101";

    for f in &[show_pulse0, show_pulse1, show_pulse2, show_pulse3] {
        f("A", a);
        f("B", b);
        let mut x = String::new();
        for (a, b) in a.chars().zip(b.chars()) {
            x.push(xor(a, b));
        }
        f("X", &x);
        println!();
    }
}
