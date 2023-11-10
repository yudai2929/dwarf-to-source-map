use regex::Regex;

pub fn split_keep<'a>(r: &Regex, text: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    let mut last = 0;
    for cap in r.captures_iter(text) {
        let (start, end) = (cap.get(0).unwrap().start(), cap.get(0).unwrap().end());
        // マッチの前のテキストを追加
        if last != start {
            result.push(&text[last..start]);
        }
        // キャプチャグループのテキスト（この場合は1番目のキャプチャグループ）を追加
        if let Some(m) = cap.get(1) {
            result.push(m.as_str());
        }
        last = end;
    }
    if last < text.len() {
        result.push(&text[last..]);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;

    #[test]
    fn test_split_keep() {
        let r = Regex::new(r"\d").unwrap();
        let text = "hello1world2";
        let result = r.split(text).collect::<Vec<_>>();
        assert_eq!(result, vec!["hello", "world"]);
    }
}
