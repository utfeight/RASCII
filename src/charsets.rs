pub const BLOCK: &[&str] = &[" ", "░", "▒", "▓", "█"];
pub const DEFAULT: &[&str] = &[
    " ", ".", "`", "^", "\"", "\\", ",", ":", ";", "I", "l", "!", "i", ">", "<", "~", "+", "_",
    "-", "?", "]", "[", "}", "{", "1", ")", "(", "|", "\\", "\\/", "/", "t", "f", "j", "r", "x",
    "n", "u", "v", "c", "z", "X", "Y", "U", "J", "C", "L", "Q", "0", "O", "Z", "m", "w", "q", "p",
    "d", "b", "k", "h", "a", "o", "*", "#", "M", "W", "&", "8", "%", "B", "$", "@",
];
pub const EMOJI: &[&str] = &[
    " ", " ", "。", "，", "🧔", "👶", "🗣", "👥", "👤", "👀", "👁", "🦴", "🦷", "🫁", "🫀", "🧠",
    "👃", "🦻", "👂", "👅", "🦀", "👿", "🦀", "👄", "🤳", "💅", "🖖", "👆", "🙏", "🤝", "🦿", "🦾",
    "💪", "🤏", "👌", "🤘", "🤞", "👊", "🤚", "🤛", "🙌", "😾", "😿", "🙀", "😺", "👾", "👽", "👻",
    "💀", "👺", "🦀", "👹", "🤡", "💤", "😴", "🥸", "🥳", "🥶", "🥵", "🤮", "🤢", "🤕", "😭", "😓",
    "😯", "😰", "😨", "😱", "😮", "😩", "😫", "🙁", "😔", "😡", "🤬", "😠", "🙄", "😐", "😶", "🧐",
    "😛", "🤗", "🤐", "🤑", "😝", "🤩", "😋", "😊", "😉", "🤣", "😅", "😆",
];
pub const RUSSIAN: &[&str] = &[
    " ", " ", "Я", "Ю", "Э", "Ь", "Ы", "Ъ", "Щ", "Ш", "Ч", "Ц", "Х", "Ф", "У", "Т", "С", "P", "П",
    "О", "Н", "М", "Л", "К", "Й", "И", "З", "Ж", "Ё", "Е", "Д", "Г", "В", "Б", "А",
];
pub const SLIGHT: &[&str] = &[
    " ", " ", ".", "`", "\"", "\\", ":", "I", "!", ">", "~", "_", "?", "[", "{", "|", ")", "(",
    "\\", "\\\\", "/", "Y", "L", "p", "d", "a", "*", "W", "8", "%", "@", "$",
];
pub const CHINESE: &[&str] = &[
    " ", "一", "二", "十", "人", "丁", "口", "王", "日", "木", "金", "華", "爱", "黑", "墨", "龍", "龘"
];

pub fn from_str(s: &str) -> Option<&[&str]> {
    match s {
        "block" => Some(BLOCK),
        "default" => Some(DEFAULT),
        "emoji" => Some(EMOJI),
        "russian" => Some(RUSSIAN),
        "slight" => Some(SLIGHT),
        "chinese" => Some(CHINESE),
        _ => None,
    }
}
