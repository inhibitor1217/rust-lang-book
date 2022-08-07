fn main() {
    assert_eq!(
        pig_latin("The quick brown fox jumps over the lazy dog"),
        String::from("Hetay uickqay rownbay oxfay umpsjay overhay hetay azylay ogday")
    );

    assert_eq!(pig_latin("Aerodynamics"), String::from("Aerodynamicshay"));

    assert_eq!(pig_latin(""), String::new())
}

fn pig_latin(original: &str) -> String {
    original
        .split_whitespace()
        .map(pig_latin_word)
        .collect::<Vec<String>>()
        .join(" ")
}

fn pig_latin_word(s: &str) -> String {
    let mut chars = s.chars();
    if let Some(c) = chars.next() {
        let mut translated = String::new();

        let c_lowercase = c.to_ascii_lowercase();
        let suffix = match c_lowercase {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                translated.push(c_lowercase);
                String::from("hay")
            }
            'a'..='z' => format!("{}ay", c_lowercase),
            _ => String::new(),
        };

        translated = translated + &chars.as_str() + &suffix;

        if ('A'..='Z').contains(&c) {
            capitalize(&translated)
        } else {
            translated
        }
    } else {
        String::new()
    }
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(first_ch) => first_ch.to_uppercase().collect::<String>() + c.as_str(),
    }
}
