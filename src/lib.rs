use std::collections::HashMap;
use maplit::hashmap;
use lazy_static::lazy_static;

lazy_static! {
    static ref map: HashMap<char, &'static str> = hashmap![
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
    for character in input.chars() {
        if let Some(c) = map.get(&character) {
            converted.push_str(c);
        }
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
    }
}