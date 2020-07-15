use std::io::Cursor;
use wordcount::{count, CountOption};

#[macro_use]
mod utils;

#[test]
fn char_count_works() {
    let input = Cursor::new(b"abadracadabra");

    let freq = count(input, CountOption::Char);
    assert_map!(freq,
        {
            "a" => 6,
            "b" => 2,
            "c" => 1,
            "d" => 2,
            "r" => 2
        }
    );
}

#[test]
fn char_count_utf8() {
    let input = Cursor::new(
        r#"
一石二鳥
焼肉定食
森羅万象
魑魅魍魎
"#
    );

    let freq = count(input, CountOption::Char);

    assert_eq!(freq.len(), 16);
    for (_, count) in freq {
        assert_eq!(count, 1);
    }
}
