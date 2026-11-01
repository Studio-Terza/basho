use guide::place::RubyText;
use guide::reading::Reading;

use basho::app::screen::wording::Rewording;
use basho::app::screen::wording::Wording;

const TAGGED: &str = "Open 9:00 daily (OSM). OSM shows a path (shown), but OSM's tags differ.";
const CLEAN: &str = "Open 9:00 daily. The map shows a path (shown), but the map's tags differ.";
const LINKED: &str = "As on https://ja.wikipedia.org/wiki/%E8%9A%B6 (checked).";
const HOST: &str = "As on ja.wikipedia.org (checked).";
const TRIGGER: &str = "the time is after {鳴子|なるこ} 15:04.";
const CONDITION: &str = "The time is after 鳴子 15:04";

#[test]
fn source_names_become_the_map_and_links_show_their_host() {
    let cleaned: Vec<String> = [TAGGED, LINKED]
        .iter()
        .filter_map(|text| RubyText::try_from((*text).to_owned()).ok())
        .map(|ruby| {
            Wording::new(&ruby)
                .reworded(Rewording::ForWalker)
                .ruby()
                .text(Reading::Written)
                .to_string()
        })
        .collect();
    assert_eq!(cleaned, [CLEAN, HOST]);
}

#[test]
fn a_trigger_reads_as_a_capitalised_condition() {
    let condition = RubyText::try_from(TRIGGER.to_owned()).map(|ruby| {
        Wording::new(&ruby)
            .reworded(Rewording::ConditionClause)
            .ruby()
            .text(Reading::Written)
    });
    assert_eq!(condition.ok().as_deref(), Some(CONDITION));
}
