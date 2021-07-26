use std::io;

fn fah_to_cel(num: f32) -> f32 {
    (num - 32.0) * 5.0 / 9.0
}

fn cel_to_fah(num: f32) -> f32 {
    (num * 9.0 / 5.0) + 32.0
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    // test cases from https://www.almanac.com/content/temperature-conversion

    #[test]
    fn test_fah_to_cel_munis_10() {
        assert_eq!(fah_to_cel(-10.0), -23.333334);
    }

    #[test]
    fn test_fah_to_cel_0() {
        assert_eq!(fah_to_cel(0.0), -17.777779);
    }

    #[test]
    fn test_fah_to_cel_100() {
        assert_eq!(fah_to_cel(100.0), 37.77778);
    }

    #[test]
    fn test_cel_to_fah_minus_10() {
        assert_eq!(cel_to_fah(-10.0), 14.0);
    }

    #[test]
    fn test_cel_to_fah_0() {
        assert_eq!(cel_to_fah(0.0), 32.0);
    }

    #[test]
    fn test_cel_to_fah_100() {
        assert_eq!(cel_to_fah(100.0), 212.0);
    }
}

fn main() {
    println!("Please enter temperature:");

    let mut temp = String::new();

    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line");

    let num: f32 = temp
        .trim()
        .parse()
        .expect("Temperature entered was not a number");

    println!("fah_to_cel: {}", fah_to_cel(num));
    println!("cel_to_fah: {}", cel_to_fah(num));
}
