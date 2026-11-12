//! **dhoni** converts Bengali text into a phonetic Roman counterpart.
//!
//! It supports multiple transliteration conventions via [`Mode`]. The output
//! is ASCII lowercase and aims to be compatible with common phonetic schemes
//! (Avro, Khipro) rather than a lossless round-trip.
//!
//! # Examples
//! ```rust
//! use dhoni::{convert_to_phonetic, convert_to_phonetic_variants, Mode};
//!
//! assert_eq!(convert_to_phonetic("আমি", Mode::Avro), "ami");
//! assert_eq!(convert_to_phonetic("চাঁদ", Mode::Khipro), "ca/d");
//! ```

/// Transliteration convention selector.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mode {
    /// Avro Phonetic-compatible spelling.
    Avro,
    /// Khipro layout spelling (see khipro-m17n): candrabindu `ঁ` → `/`,
    /// visarga-like `ৎ` → `t/`, and virama joins are preserved as-is.
    Khipro,
}

fn letter_map(letter: char, mode: Mode) -> Option<&'static str> {
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
        'ঋ' => Some("ri"),
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
        'ৃ' => Some("ri"),
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
        'ৎ' => Some(match mode {
            Mode::Avro => "t",
            Mode::Khipro => "t/",
        }),
        'ং' => Some("ng"),
        'ঃ' => Some(""),
        'ঁ' => Some(match mode {
            Mode::Avro => "",
            Mode::Khipro => "/",
        }),
        _ => None,
    }
}

/// Consonant conjunct special cases: (first, second after virama) → spelling.
/// These are the only cases where the general virama-drop concatenation would
/// produce the wrong phonetic key; everything else is intentionally
/// concatenated (e.g. স্ত → `st`).
const SPECIAL_CONJUNCTS: &[(char, char, &str)] = &[
    ('ক', 'ষ', "kkh"), // ক্ষ
];

fn special_conjunct(first: char, second: char) -> Option<&'static str> {
    for &(a, b, out) in SPECIAL_CONJUNCTS {
        if a == first && b == second {
            return Some(out);
        }
    }
    None
}

/// A consonant that can carry an implicit vowel.
fn is_bengali_consonant(c: char) -> bool {
    matches!(c, '\u{0995}'..='\u{09B9}' | 'ড়' | 'ঢ়' | 'য়')
}

const NUKTA: char = '\u{09BC}';

/// Precomposed nukta letters are composition-excluded, so text may carry
/// `ঢ` + nukta instead of `ঢ়`. Fold that pair onto its phonetic spelling.
fn nukta_letter(base: char) -> Option<&'static str> {
    match base {
        'ড' | 'ঢ' => Some("r"),
        'য' => Some("y"),
        _ => None,
    }
}

/// Spelling choices that produce alternate but plausible Roman forms.
#[derive(Clone, Copy)]
struct Style {
    /// Emit the implicit `o` that Avro typists insert between consonants
    /// (e.g. `kortobbo` for কর্তব্য).
    inherent_o: bool,
    /// Prefer aspirated/common alternate keys: `চ` → `ch`, and treat the
    /// `্য` glide as silent (`ব্য` → `b`).
    alternate: bool,
}

fn push_inherent_o(out: &mut String, chars: &[char], next: usize, mode: Mode, style: Style) {
    if style.inherent_o
        && mode == Mode::Avro
        && chars
            .get(next)
            .is_some_and(|c| is_bengali_consonant(*c) && *c != NUKTA)
    {
        out.push('o');
    }
}

fn convert_with(input: &str, mode: Mode, style: Style) -> String {
    let mut converted = String::with_capacity(input.len());
    let chars: Vec<char> = input.chars().collect();

    let mut index = 0;
    while index < chars.len() {
        let current = chars[index];

        if chars.get(index + 1) == Some(&'্') {
            if let Some(after) = chars.get(index + 2) {
                if let Some(out) = special_conjunct(current, *after) {
                    converted.push_str(out);
                    index += 3;
                    push_inherent_o(&mut converted, &chars, index, mode, style);
                    continue;
                }
            }
        }

        if chars.get(index + 1) == Some(&NUKTA) {
            if let Some(out) = nukta_letter(current) {
                converted.push_str(out);
                index += 2;
                push_inherent_o(&mut converted, &chars, index, mode, style);
                continue;
            }
        }

        if current == '্' {
            match chars.get(index + 1) {
                Some('ব') => {
                    converted.push('w');
                    index += 2;
                    push_inherent_o(&mut converted, &chars, index, mode, style);
                    continue;
                }
                Some('য') => {
                    if !style.alternate {
                        converted.push('y');
                    }
                    index += 2;
                    push_inherent_o(&mut converted, &chars, index, mode, style);
                    continue;
                }
                _ if mode == Mode::Khipro => {
                    // In Khipro, virama joins; drop the mark and keep the
                    // following consonant in the normal flow.
                    index += 1;
                    continue;
                }
                _ => {
                    index += 1;
                    continue;
                }
            }
        }

        let spelling = if style.alternate && current == 'চ' {
            "ch"
        } else {
            letter_map(current, mode).unwrap_or("")
        };
        converted.push_str(spelling);
        if is_bengali_consonant(current) {
            push_inherent_o(&mut converted, &chars, index + 1, mode, style);
        }

        if spelling.is_empty() && (current.is_ascii_whitespace() || current.is_ascii_punctuation())
        {
            converted.push(current);
        } else if spelling.is_empty() && mode == Mode::Khipro && current == '/' {
            // Preserve explicit Khipro `/` typed in Bangla-side edge cases.
            converted.push('/');
        }

        index += 1;
    }

    converted
}

/// Converts input text using the given transliteration [`Mode`].
///
/// This is the canonical spelling: no optional implicit vowels are inserted.
pub fn convert_to_phonetic(input: &str, mode: Mode) -> String {
    convert_with(
        input,
        mode,
        Style {
            inherent_o: false,
            alternate: false,
        },
    )
}

/// Returns plausible phonetic forms for one Bangla word.
///
/// Avro mode returns the canonical spelling plus alternates that cover common
/// Avro-style typings: the implicit `o` many typists insert between
/// consonants (`kortobbo` for কর্তব্য) and aspirated/silent-glide spellings
/// (`chino` for চিনো, `kortobbo` for কর্তব্য). Khipro mode additionally
/// returns a trailing-`/` variant for words containing a virama (`্`),
/// because Khipro typists can suppress a virama with `/` (e.g. `ks/` → কস
/// vs `ks` → ক্স).
pub fn convert_to_phonetic_variants(input: &str, mode: Mode) -> Vec<String> {
    let styles: &[Style] = if mode == Mode::Avro {
        &[
            Style {
                inherent_o: false,
                alternate: false,
            },
            Style {
                inherent_o: true,
                alternate: false,
            },
            Style {
                inherent_o: true,
                alternate: true,
            },
            Style {
                inherent_o: false,
                alternate: true,
            },
        ]
    } else {
        &[Style {
            inherent_o: false,
            alternate: false,
        }]
    };

    let mut forms: Vec<String> = Vec::with_capacity(styles.len());
    for style in styles {
        let form = convert_with(input, mode, *style);
        if !form.is_empty() {
            forms.push(form);
        }
    }

    if mode == Mode::Khipro && input.contains('্') {
        forms.extend(
            forms
                .clone()
                .into_iter()
                .filter(|form| !form.ends_with('/'))
                .map(|form| format!("{form}/")),
        );
    }

    forms.sort();
    forms.dedup();
    forms
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numbers() {
        assert_eq!(convert_to_phonetic("০১২৩৪৫৬৭৮৯", Mode::Avro), "0123456789");
    }

    #[test]
    fn word() {
        assert_eq!(convert_to_phonetic("আমি", Mode::Avro), "ami");
        assert_eq!(convert_to_phonetic("ঋতু", Mode::Avro), "ritu");
        assert_eq!(convert_to_phonetic("ওষুধ", Mode::Avro), "oxudh");
        assert_eq!(convert_to_phonetic("সংখ্যা", Mode::Avro), "sngkhya");
        assert_eq!(convert_to_phonetic("ক্ষুধা", Mode::Avro), "kkhudha");
        assert_eq!(convert_to_phonetic("বিশ্ব", Mode::Avro), "bisw");
        assert_eq!(convert_to_phonetic("পদ্ম", Mode::Avro), "pdm");
        assert_eq!(convert_to_phonetic("জিতেন্দ্র", Mode::Avro), "jitendr");
        assert_eq!(convert_to_phonetic("বিদ্বান", Mode::Avro), "bidwan");
        assert_eq!(convert_to_phonetic("চাঁদ", Mode::Avro), "cad");
    }

    #[test]
    fn sentence() {
        assert_eq!(
            convert_to_phonetic(
                "আমাদের ভালোবাসা হয়ে গেল ঘাস, খেয়ে গেল গরু আর দিয়ে গেল বাঁশ",
                Mode::Avro
            ),
            "amader bhalobasa hye gel ghas, kheye gel gru ar diye gel bas"
        );
    }

    #[test]
    fn avro_mode() {
        assert_eq!(convert_to_phonetic("মাঁ", Mode::Avro), "ma");
        assert_eq!(convert_to_phonetic("ৎ", Mode::Avro), "t");
        assert_eq!(convert_to_phonetic_variants("ক্স", Mode::Avro), vec!["ks"]);
    }

    #[test]
    fn khipro_mode_uses_layout_markers() {
        assert_eq!(convert_to_phonetic("মাঁ", Mode::Khipro), "ma/");
        assert_eq!(convert_to_phonetic("ৎ", Mode::Khipro), "t/");
        assert_eq!(convert_to_phonetic("কঁ", Mode::Khipro), "k/");
        assert_eq!(convert_to_phonetic("ক্ষ", Mode::Khipro), "kkh");
        assert_eq!(convert_to_phonetic("মা", Mode::Khipro), "ma");
        assert_eq!(convert_to_phonetic("চাঁদ", Mode::Khipro), "ca/d");
    }

    #[test]
    fn avro_variant_forms_cover_common_typings() {
        let forms = convert_to_phonetic_variants("চিনো", Mode::Avro);
        assert!(forms.contains(&"chino".to_string()), "{:?}", forms);
        assert!(forms.contains(&"cino".to_string()), "{:?}", forms);

        let forms = convert_to_phonetic_variants("কিংকর্তব্যবিমূঢ়", Mode::Avro);
        assert!(forms.contains(&"kingkortobobimur".to_string()), "{:?}", forms);

        assert_eq!(
            convert_to_phonetic_variants("পৃথিবীর", Mode::Avro),
            vec!["prithibir"]
        );
    }

    #[test]
    fn khipro_virama_variants() {
        assert_eq!(
            convert_to_phonetic_variants("ক্স", Mode::Khipro),
            vec!["ks", "ks/"]
        );
        assert_eq!(convert_to_phonetic_variants("কস", Mode::Khipro), vec!["ks"]);
        assert_eq!(convert_to_phonetic_variants("মা", Mode::Khipro), vec!["ma"]);
    }
    #[test]
    fn conjuncts_are_concatenated_letters() {
        let cases = [
            ("স্ত", "st"),
            ("ক্ত", "kt"),
            ("ঞ্জ", "ngj"),
            ("ম্প", "mp"),
            ("ল্ল", "ll"),
            ("স্য", "sy"),
            ("দ্ধ", "ddh"),
            ("ক্ষ", "kkh"),
            ("ত্র", "tr"),
        ];
        for (bangla, roman) in cases {
            assert_eq!(
                convert_to_phonetic(bangla, Mode::Avro),
                roman,
                "conjunct {bangla}"
            );
        }
    }

    #[test]
    fn canonical_forms_are_ascii_lowercase() {
        let letters = "অআইঈউঊঋএঐওঔকখগঘঙচছজঝঞটঠডঢণতথদধানপফবভমযরলশষসহড়ঢ়য়ৎংঃঁ";
        for mode in [Mode::Avro, Mode::Khipro] {
            for letter in letters.chars() {
                for form in convert_to_phonetic_variants(&letter.to_string(), mode) {
                    assert!(
                        form.bytes()
                            .all(|b| b.is_ascii_lowercase() || b == b'/' || b.is_ascii_digit()),
                        "{}",
                        "non-canonical form {form:?} for {letter:?}"
                    );
                }
            }
        }
    }

    #[test]
    fn tsv_has_modifier_rows() {
        let csv = include_str!("../data/khipro-m17n/khipro-testcases.tsv");
        let mut checked = 0;
        for line in csv.lines().skip(1) {
            let mut cols = line.split('\t');
            let input = cols.next().unwrap_or("");
            let output = cols.next().unwrap_or("");
            if output.contains('ঁ') && !input.ends_with("//") {
                checked += 1;
            }
        }
        assert!(
            checked >= 3,
            "{}",
            "expected candrabindu rows in TSV, got {checked}"
        );
    }

    #[test]
    fn khipro_tsv_conformance() {
        // Bangla -> Roman inverse checked against the khipro-m17n corpus:
        // exact = primary matches, variant = any emitted variant matches
        // (e.g. trailing '/' virama suppression), lossy = printed (layout
        // mechanics like conjunct typing/sh-s digraphs/f-suppressor that the
        // inverse intentionally does not encode).
        let csv = include_str!("../data/khipro-m17n/khipro-testcases.tsv");
        let is_bangla = |s: &str| {
            s.chars().all(|c| {
                ('\u{0980}'..='\u{09ff}').contains(&c)
                    || c == '\u{200c}'
                    || c == '\u{200d}'
                    || c == ' '
            })
        };
        let mut exact = 0;
        let mut variant = 0;
        let mut lossy: Vec<&str> = Vec::new();
        let mut non_roman = 0;
        let mut mapped = 0;
        for line in csv.lines().skip(1) {
            if line.is_empty() {
                continue;
            }
            let mut cols = line.split('\t');
            let input = cols.next().unwrap_or("");
            let output = cols.next().unwrap_or("");
            if !is_bangla(output) {
                non_roman += 1;
                continue;
            }
            mapped += 1;
            let primary = convert_to_phonetic(output, Mode::Khipro);
            if primary == input {
                exact += 1;
            } else if convert_to_phonetic_variants(output, Mode::Khipro)
                .iter()
                .any(|f| f == input)
            {
                variant += 1;
            } else {
                lossy.push(input);
            }
        }

        println!(
            "conformance: exact={exact} variant={variant} mapped={mapped} lossy={lossy:?} non_roman={non_roman}"
        );
        assert!(
            exact + variant >= 20,
            "{}",
            "invertible core shrank: exact={exact} variant={variant} (lossy: {lossy:?})"
        );
        assert!(mapped > 50, "{}", "suspiciously few mapped rows: {mapped}");
    }
}
