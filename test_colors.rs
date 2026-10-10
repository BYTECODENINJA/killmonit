fn level_color(percent: f64) -> &'static str {
    if percent < 50.0 {
        "Green"
    } else if percent < 60.0 {
        "Yellow"
    } else {
        "Red"
    }
}

fn main() {
    println!("0: {}", level_color(0.0));
    println!("49: {}", level_color(49.0));
    println!("50: {}", level_color(50.0));
    println!("59: {}", level_color(59.0));
    println!("60: {}", level_color(60.0));
    println!("100: {}", level_color(100.0));
}
