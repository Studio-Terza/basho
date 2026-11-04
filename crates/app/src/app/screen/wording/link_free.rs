use super::markup::Markup;
use newtype::Newtype;
use std::fmt;

#[derive(Newtype)]
pub struct LinkFree<'text> {
    markup: &'text Markup,
}

impl LinkFree<'_> {
    const LINKS: [&'static str; 2] = ["https://", "http://"];
    const LINK_TAIL: [char; 4] = [')', '.', ',', ';'];

    fn host_only(word: Markup) -> Markup {
        let rest = Self::LINKS
            .iter()
            .find_map(|scheme| word.strip_prefix(scheme));
        let link = rest.map(|rest| rest.trim_end_matches(Self::LINK_TAIL));
        rest.zip(link)
            .map(|(rest, link)| {
                let host = link.split('/').next().unwrap_or(link);
                Markup::new(format!("{host}{}", &rest[link.len()..]))
            })
            .unwrap_or(word)
    }
}

impl fmt::Display for LinkFree<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let words: Vec<String> = self
            .markup
            .split(' ')
            .map(|word| Self::host_only(Markup::new(word.to_owned())).to_string())
            .collect();
        formatter.write_str(&words.join(" "))
    }
}
