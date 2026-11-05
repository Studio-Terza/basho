use crate::clock::{ClockTime, Minutes};
use crate::day_number::DayNumber;
use crate::geo::{Coordinate, Degrees, Distance};
use crate::guidance::leg_progress::LegProgress;
use crate::leg::LegIndex;
use std::cmp::Ordering;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Word<'text>(&'text str);

impl<'text> Word<'text> {
    #[must_use]
    pub const fn new(text: &'text str) -> Word<'text> {
        Word(text)
    }
}

#[derive(Clone, Copy, Debug)]
enum Slot {
    Is(Word<'static>),
    Free,
}

struct Mismatch;

impl Slot {
    fn take(self, word: Word<'_>) -> Result<Option<Word<'_>>, Mismatch> {
        match self {
            Slot::Free => Ok(Some(word)),
            Slot::Is(label) => Self::labelled(label.0.cmp(word.0)),
        }
    }

    fn labelled<'text>(order: Ordering) -> Result<Option<Word<'text>>, Mismatch> {
        match order {
            Ordering::Equal => Ok(None),
            Ordering::Less | Ordering::Greater => Err(Mismatch),
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Form(&'static [Slot]);

struct Reading<'text, 'words> {
    free: Vec<Word<'text>>,
    rest: &'words [Word<'text>],
}

impl Form {
    fn read<'text, 'words>(self, words: &'words [Word<'text>]) -> Option<Reading<'text, 'words>> {
        words
            .get(..self.0.len())
            .zip(words.get(self.0.len()..))
            .and_then(|(head, rest)| {
                self.0
                    .iter()
                    .zip(head)
                    .map(|(slot, word)| slot.take(*word))
                    .collect::<Result<Vec<Option<Word<'text>>>, Mismatch>>()
                    .ok()
                    .map(|taken| Reading {
                        free: taken.into_iter().flatten().collect(),
                        rest,
                    })
            })
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Words<'text, 'words> {
    words: &'words [Word<'text>],
}

impl<'text, 'words> Words<'text, 'words> {
    const HEAD: Form = Form(&[
        Slot::Is(Word::new("day")),
        Slot::Free,
        Slot::Is(Word::new("leg")),
        Slot::Free,
    ]);
    const SEEN: Form = Form(&[Slot::Is(Word::new("from")), Slot::Free, Slot::Free]);
    const OFFSETS: [Form; 3] = [
        Form(&[Slot::Is(Word::new("offset")), Slot::Free]),
        Form(&[Slot::Is(Word::new("behind")), Slot::Free]),
        Form(&[
            Slot::Is(Word::new("behind")),
            Slot::Free,
            Slot::Is(Word::new("started")),
            Slot::Free,
        ]),
    ];

    #[must_use]
    pub fn new(words: &'words [Word<'text>]) -> Words<'text, 'words> {
        Words { words }
    }

    #[must_use]
    pub fn progress(self) -> Option<LegProgress> {
        Self::HEAD.read(self.words).and_then(|head| {
            let rest = Words::new(head.rest);
            head.free
                .first()
                .zip(head.free.get(1))
                .zip(rest.offset())
                .and_then(|((day, leg), offset)| rest.kept(*day, *leg, offset))
        })
    }

    fn kept(self, day: Word<'_>, leg: Word<'_>, offset: Word<'_>) -> Option<LegProgress> {
        Some(LegProgress {
            day: Some(DayNumber::new(day.0.parse().ok()?)),
            leg: LegIndex::new(leg.0.parse::<usize>().ok()?.checked_sub(1)?),
            along: self.distance(Word::new("along")),
            furthest: self
                .distance(Word::new("furthest"))
                .max(self.distance(Word::new("along"))),
            offset: Minutes::new(offset.0.parse().ok()?),
            at: self
                .after(Word::new("at"))
                .map(|at| at.0.parse())
                .and_then(Result::ok)
                .map(Minutes::new)
                .map(ClockTime::at_minutes),
            seen: self.seen(),
            tracking: self
                .words
                .iter()
                .map(|word| word.0)
                .collect::<Vec<_>>()
                .join(" ")
                .parse()
                .unwrap_or_default(),
        })
    }

    #[must_use]
    pub fn distance(self, label: Word<'_>) -> Distance {
        self.after(label)
            .and_then(|along| along.0.parse().ok())
            .map_or(Distance::default(), Distance::new)
    }

    fn offset(self) -> Option<Word<'text>> {
        Self::OFFSETS
            .iter()
            .find_map(|form| {
                form.read(self.words)
                    .filter(|reading| reading.rest.is_empty())
            })
            .and_then(|reading| reading.free.last().copied())
            .or_else(|| self.after(Word::new("offset")))
    }

    fn after(self, label: Word<'_>) -> Option<Word<'text>> {
        self.words
            .iter()
            .zip(self.words.iter().skip(1))
            .find(|(word, _)| **word == label)
            .map(|(_, next)| *next)
    }

    fn seen(self) -> Option<Coordinate> {
        self.words
            .iter()
            .position(|word| *word == Word::new("from"))
            .and_then(|from| self.words.get(from..))
            .and_then(|tail| Self::SEEN.read(tail))
            .and_then(|reading| {
                Some(Coordinate::new(
                    Degrees::new(reading.free.first()?.0.parse().ok()?),
                    Degrees::new(reading.free.get(1)?.0.parse().ok()?),
                ))
            })
    }
}
