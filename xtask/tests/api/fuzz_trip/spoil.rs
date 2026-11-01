use laws::proptest::prelude::{Strategy, any, prop};
use laws::proptest::sample::{Index, select};

const SAMPLE: &str = include_str!("../../../../data/trips/sample.toml");
const AWKWARD: [&str; 15] = [
    "nan",
    "inf",
    "-inf",
    "0",
    "-1",
    "1e308",
    "18446744073709551615",
    "\"\"",
    "[]",
    "{}",
    "\"{漢字|}\"",
    "\"99:99\"",
    "true",
    "\"\\u0000\"",
    "[[[]]]",
];
const WORDS: &str = "[A-Za-z0-9 .,!?'-]{0,40}";
const REWORD_WEIGHT: u32 = 6;
const MOST_SPOILS: usize = 4;

#[derive(Clone, Debug)]
enum Spoil {
    Drop(Index),
    Repeat(Index),
    Cut(Index),
    Swap(Index, Index),
    Insert(Index, String),
    Replace(Index, &'static str),
    Reword(Index, String),
}

#[derive(Clone, Debug)]
pub struct Spoils(Vec<Spoil>);

impl Spoil {
    fn strategy() -> impl Strategy<Value = Spoil> {
        prop::strategy::Union::new_weighted(vec![
            (1, any::<Index>().prop_map(Spoil::Drop).boxed()),
            (1, any::<Index>().prop_map(Spoil::Repeat).boxed()),
            (1, any::<Index>().prop_map(Spoil::Cut).boxed()),
            (
                1,
                (any::<Index>(), any::<Index>())
                    .prop_map(|(a, b)| Spoil::Swap(a, b))
                    .boxed(),
            ),
            (
                1,
                (any::<Index>(), any::<String>())
                    .prop_map(|(at, junk)| Spoil::Insert(at, junk))
                    .boxed(),
            ),
            (
                1,
                (any::<Index>(), select(AWKWARD.to_vec()))
                    .prop_map(|(at, token)| Spoil::Replace(at, token))
                    .boxed(),
            ),
            (
                REWORD_WEIGHT,
                (any::<Index>(), WORDS)
                    .prop_map(|(at, words)| Spoil::Reword(at, words))
                    .boxed(),
            ),
        ])
    }

    fn applied(&self, text: &str) -> String {
        let mut lines: Vec<String> = text.lines().map(str::to_owned).collect();
        if lines.is_empty() {
            return text.to_owned();
        }
        match self {
            Spoil::Cut(at) => {
                let cut = text.char_indices().nth(at.index(text.chars().count()));
                return cut.map_or_else(|| text.to_owned(), |(byte, _)| text[..byte].to_owned());
            }
            Spoil::Drop(at) => {
                lines.remove(at.index(lines.len()));
            }
            Spoil::Repeat(at) => {
                let line = lines[at.index(lines.len())].clone();
                lines.insert(at.index(lines.len()), line);
            }
            Spoil::Swap(a, b) => {
                let (a, b) = (a.index(lines.len()), b.index(lines.len()));
                lines.swap(a, b);
            }
            Spoil::Insert(at, junk) => lines.insert(at.index(lines.len()), junk.clone()),
            Spoil::Reword(at, words) => {
                let count = lines.len();
                let line = &mut lines[at.index(count)];
                let mut quoted = line.splitn(3, '"');
                if let (Some(before), Some(_), Some(after)) =
                    (quoted.next(), quoted.next(), quoted.next())
                {
                    *line = format!("{before}\"{words}\"{after}");
                }
            }
            Spoil::Replace(at, token) => {
                let count = lines.len();
                let line = &mut lines[at.index(count)];
                *line = line.rsplit_once(" = ").map_or_else(
                    || (*token).to_owned(),
                    |(key, _)| format!("{key} = {token}"),
                );
            }
        }
        lines.join("\n")
    }
}

impl Spoils {
    pub fn strategy() -> impl Strategy<Value = Spoils> {
        prop::collection::vec(Spoil::strategy(), 1..=MOST_SPOILS).prop_map(Spoils)
    }

    pub fn applied_to_sample(&self) -> String {
        self.0
            .iter()
            .fold(SAMPLE.to_owned(), |text, spoil| spoil.applied(&text))
    }
}
