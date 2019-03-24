use std::collections::HashMap;
use maplit::hashmap;
use lazy_static::lazy_static;

lazy_static! {
    static ref map: HashMap<char, &'static str> = hashmap! [
        // Numbers
        '০' => "0",
        '১' => "1",
        '২' => "2",
        '৩' => "3",
        '৪' => "4",
        '৫' => "5",
        '৬' => "6",
        '৭' => "7",
        '৮' => "8",
        '৯' => "9",
        // Vowels
        'অ' => "o",
        'আ' => "a",
        'ই' => "i",
        'ঈ' => "i",
        'উ' => "u",
        'ঊ' => "u",
        'ঋ' => "rri",
        'এ' => "e",
        'ঐ' => "oi",
        'ও' => "o",
        'ঔ' => "ou",
        // Kars
        'া' => "a",
        'ি' => "i",
        'ী' => "i",
        'ু' => "u",
        'ূ' => "u",
        'ৃ' => "rri",
        'ে' => "e",
        'ৈ' => "oi",
        'ো' => "o",
        'ৌ' => "ou",
        // Consonants
        'ক' => "k",
        'খ' => "kh",
        'গ' => "g",
        'ঘ' => "gh",
        'ঙ' => "ng",
        'চ' => "c",
        'ছ' => "ch",
        'জ' => "j",
        'ঝ' => "jh",
        'ঞ' => "ng",
        'ট' => "t",
        'ঠ' => "th",
        'ড' => "d",
        'ঢ' => "dh",
        'ণ' => "n",
        'ত' => "t",
        'থ' => "th",
        'দ' => "d",
        'ধ' => "dh",
        'ন' => "n",
        'প' => "p",
        'ফ' => "f",
        'ব' => "b",
        'ভ' => "bh",
        'ম' => "m",
        'য' => "z",
        'র' => "r",
        'ল' => "l",
        'শ' => "s",
        'ষ' => "x",
        'স' => "s",
        'হ' => "h",
        'ড়' => "r",
        'ঢ়' => "r",
        'য়' => "y",
        'ৎ' => "t",
        'ং' => "ng",
        'ঃ' => "",
        'ঁ' => "",
        //'' => "",
    ];
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

        if let Some(c) = map.get(&vec_str[index]) {
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
