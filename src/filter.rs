use phf::phf_map;

static CYRILIC_TO_LATIN: phf::Map<&'static str, &'static str> = phf_map! {
    "Ч" => "Ch", "Ш" => "Sh", "Щ" => "Sht", "Ю" => "Yu", "Я" => "Ya", "Ж" => "Zh",
    "А" => "A", "Б" => "B", "В" => "V", "Г" => "G", "Д" => "D", "Е" => "E", "З" => "Z",
    "И" => "I", "Й" => "Y", "К" => "K", "Л" => "L", "М" => "M", "Н" => "N", "О" => "O",
    "П" => "P", "Р" => "R", "С" => "S", "Т" => "T", "У" => "U", "Ф" => "F", "Х" => "H",
    "Ц" => "C", "Ъ" => "A", "Ь" => "I", "Ы" => "I", "Ё" => "Yo", "Э" => "E", "ч" => "ch",
    "ш" => "sh", "щ" => "sht", "ю" => "yu", "я" => "ya", "ж" => "zh", "а" => "a", "б" => "b",
    "в" => "v", "г" => "g", "д" => "d", "е" => "e", "з" => "z", "и" => "i", "й" => "y",
    "к" => "k", "л" => "l", "м" => "m", "н" => "n", "о" => "o", "п" => "p", "р" => "r", "с" => "s",
    "т" => "t", "у" => "u", "ф" => "f", "х" => "h", "ц" => "c", "ъ" => "a", "ь" => "i", "ы" => "i",
    "ё" => "yo", "э" => "e", " " => "_", "ї" => "i",
};

// TODO ugly code!
pub fn filter(input: &str) -> String {
    let mut result: String = String::new();
    for ch in input.chars() {
        let mut b = [0; 4];
        let mut letter: &str = ch.encode_utf8(&mut b);
        let ascii: String;
        if CYRILIC_TO_LATIN.contains_key(letter) {
            letter = convert_to_latin(&mut letter).unwrap();
        } else {
            if letter.is_ascii() == false {
                ascii = "".to_string();
            } else {
                ascii = filter_alphanumeric(&letter);
            }
            letter = ascii.as_str();
        }
        result.push_str(&letter);
    }
    result
}

fn convert_to_latin(ch: &str) -> Option<&str> {
    let output = CYRILIC_TO_LATIN.get(ch).cloned();
    output
}

fn filter_alphanumeric(input: &str) -> String {
    let output: String = input
        .chars()
        .filter(|x| x.is_alphanumeric())
        .collect::<String>();
    output
}

#[cfg(test)]
mod filter_tests {
    use super::*;

    #[test]
    fn cyrilic_to_latin() {
        let cyrilic_phrase = "Это тестовая фраза на кирилике.";
        let latin_phrase = filter(cyrilic_phrase);
        println!("Result: {}", &latin_phrase);
        assert_eq!("Eto_testovaya_fraza_na_kirilike", &latin_phrase);
    }
}
