use newtype::Newtype;
use std::fmt;

#[derive(Newtype)]
pub struct LinkFree<'text> {
    markup: &'text str,
}

impl LinkFree<'_> {
    const LINKS: [&'static str; 2] = ["https://", "http://"];
    const LINK_TAIL: [char; 4] = [')', '.', ',', ';'];

    fn host_only(word: &str) -> String {
        let rest = Self::LINKS
            .iter()
            .find_map(|scheme| word.strip_prefix(scheme));
        let link = rest.map(|rest| rest.trim_end_matches(Self::LINK_TAIL));
        rest.zip(link).map_or_else(
            || word.to_owned(),
            |(rest, link)| {
                let host = link.split('/').next().unwrap_or(link);
                format!("{host}{}", &rest[link.len()..])
            },
        )
    }
}

impl fmt::Display for LinkFree<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let words: Vec<String> = self.markup.split(' ').map(Self::host_only).collect();
        formatter.write_str(&words.join(" "))
    }
}
