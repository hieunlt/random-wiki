use cursive::{
    views::{Dialog, LinearLayout, ScrollView, TextView},
    Cursive,
};

use crate::summary::Summary;

mod summary;

fn main() {
    let mut siv = cursive::default();
    let welcome_dialog = Dialog::text("A random wiki a day makes an erudite mind someday.")
        .title("Random wiki")
        .button("Quit", |s| s.quit())
        .button("Random", fetch_random);
    siv.add_layer(welcome_dialog);
    siv.run();
}

fn fetch_random(s: &mut Cursive) {
    let url = "https://en.wikipedia.org/api/rest_v1/page/random/summary";
    let res = reqwest::blocking::get(url).unwrap();
    let summary: Summary = res.json().unwrap();
    let full_url = summary.content_urls.get("desktop").unwrap();

    s.pop_layer();
    let view = LinearLayout::vertical()
        .child(TextView::new(&summary.description).center())
        .child(ScrollView::new(TextView::new(&summary.extract)))
        .child(TextView::new(format!("Full article: {}", full_url.page)));
    s.add_layer(
        Dialog::around(view)
            .title(&summary.title)
            .button("Quit", |s| s.quit())
            .button("Random", fetch_random),
    )
}
