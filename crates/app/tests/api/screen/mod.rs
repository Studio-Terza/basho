mod disagreement_links;
mod inn_marks;
mod inn_page;
mod wording;

#[test]
#[expect(
    clippy::needless_for_each,
    reason = "iterators are driven by combinators here"
)]
fn the_page_reaches_the_screen_edges_so_the_safe_areas_hold_still() {
    [
        include_str!("../../../../../vendor/dioxus-desktop/src/assets/prod.index.html"),
        include_str!("../../../../../vendor/dioxus-desktop/src/assets/dev.index.html"),
    ]
    .into_iter()
    .for_each(|page| assert!(page.contains("viewport-fit=cover")));
}
