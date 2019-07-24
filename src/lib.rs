//! **dhoni** is a crate for converting Bengali text into their phonetic counterpart.
//! 
//! It's output doesn't strictly follow the Avro Phonetic specification, but tries to be
//! compatible with Avro Phonetic's scheme.
//! 
//! # Example: Converting a Bengali word
//! ```rust
//! use dhoni::convert_to_phonetic;
//! 
//! let banglish = convert_to_phonetic("আমি");
//! assert_eq!(banglish, "ami");
//! ```
//! 

fn letter_map(letter: char) -> Option<&'static str> {
    match letter {
        // Numbers
        '০' => Some("0"),
        '১' => Some("1"),
        '২' => Some("2"),
        '৩' => Some("3"),
        '৪' => Some("4"),
        '৫' => Some("5"),
        '৬' => Some("6"),
        '৭' => Some("7"),
        '৮' => Some("8"),
        '৯' => Some("9"),
        // Vowels
        'অ' => Some("o"),
        'আ' => Some("a"),
        'ই' => Some("i"),
        'ঈ' => Some("i"),
        'উ' => Some("u"),
        'ঊ' => Some("u"),
        'ঋ' => Some("rri"),
        'এ' => Some("e"),
        'ঐ' => Some("oi"),
        'ও' => Some("o"),
        'ঔ' => Some("ou"),
        // Kars
        'া' => Some("a"),
        'ি' => Some("i"),
        'ী' => Some("i"),
        'ু' => Some("u"),
        'ূ' => Some("u"),
        'ৃ' => Some("rri"),
        'ে' => Some("e"),
        'ৈ' => Some("oi"),
        'ো' => Some("o"),
        'ৌ' => Some("ou"),
        // Consonants
        'ক' => Some("k"),
        'খ' => Some("kh"),
        'গ' => Some("g"),
        'ঘ' => Some("gh"),
        'ঙ' => Some("ng"),
        'চ' => Some("c"),
        'ছ' => Some("ch"),
        'জ' => Some("j"),
        'ঝ' => Some("jh"),
        'ঞ' => Some("ng"),
        'ট' => Some("t"),
        'ঠ' => Some("th"),
        'ড' => Some("d"),
        'ঢ' => Some("dh"),
        'ণ' => Some("n"),
        'ত' => Some("t"),
        'থ' => Some("th"),
        'দ' => Some("d"),
        'ধ' => Some("dh"),
        'ন' => Some("n"),
        'প' => Some("p"),
        'ফ' => Some("f"),
        'ব' => Some("b"),
        'ভ' => Some("bh"),
        'ম' => Some("m"),
        'য' => Some("z"),
        'র' => Some("r"),
        'ল' => Some("l"),
        'শ' => Some("s"),
        'ষ' => Some("x"),
        'স' => Some("s"),
        'হ' => Some("h"),
        'ড়' => Some("r"),
        'ঢ়' => Some("r"),
        'য়' => Some("y"),
        'ৎ' => Some("t"),
        'ং' => Some("ng"),
        'ঃ' => Some(""),
        'ঁ' => Some(""),
        _ => None,
    }
}

/// Converts input text into their Bengali phonetic counterpart.
pub fn convert_to_phonetic(input: &str) -> String {
    let mut converted = String::with_capacity(input.len());
    let vec_str: Vec<char> = input.chars().collect();

    let mut index = 0;
    while index < vec_str.len() {
        // Zukta Kha
        if vec_str[index] == 'ক' {
            if let Some(c) = vec_str.get(index + 1) {
                if *c == '্' {
                    if let Some(c) = vec_str.get(index + 2) {
                        if *c == 'ষ' {
                            converted.push_str("kkh");
                            index += 3;
                            continue;
                        }
                    }
                }
            }
        }

        if vec_str[index] == '্' {
            if let Some(c) = vec_str.get(index + 1) {
                // B Fola
                if *c == 'ব' {
                    converted.push_str("w");
                    index += 2;
                    continue;
                } else if *c == 'য' {
                    // Z Fola
                    converted.push_str("y");
                    index += 2;
                    continue;
                }
            }
        }

        if let Some(c) = letter_map(vec_str[index]) {
            converted.push_str(c);
        }

        if vec_str[index].is_ascii_whitespace() || vec_str[index].is_ascii_punctuation() {
            converted.push(vec_str[index]);
        }

        index += 1;
    }

    converted
}

#[cfg(test)]
mod tests {
    use super::convert_to_phonetic;

    #[test]
    fn numbers() {
        assert_eq!(convert_to_phonetic("০১২৩৪৫৬৭৮৯"), "0123456789");
    }

    #[test]
    fn word() {
        assert_eq!(convert_to_phonetic("আমি"), "ami");
        assert_eq!(convert_to_phonetic("ঋতু"), "rritu");
        assert_eq!(convert_to_phonetic("ওষুধ"), "oxudh");
        assert_eq!(convert_to_phonetic("সংখ্যা"), "sngkhya");
        assert_eq!(convert_to_phonetic("ক্ষুধা"), "kkhudha");
        assert_eq!(convert_to_phonetic("বিশ্ব"), "bisw");
        assert_eq!(convert_to_phonetic("পদ্ম"), "pdm");
        assert_eq!(convert_to_phonetic("জিতেন্দ্র"), "jitendr");
        assert_eq!(convert_to_phonetic("বিদ্বান"), "bidwan");
        assert_eq!(convert_to_phonetic("চাঁদ"), "cad");
    }

    #[test]
    fn sentence() {
        assert_eq!(convert_to_phonetic("আমাদের ভালোবাসা হয়ে গেল ঘাস, খেয়ে গেল গরু আর দিয়ে গেল বাঁশ"), "amader bhalobasa hye gel ghas, kheye gel gru ar diye gel bas");
    }
}
