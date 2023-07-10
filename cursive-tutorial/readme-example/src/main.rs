use cursive::{
    CursiveRunnable,
    views::{Dialog, TextView}
};

fn main() {
    let mut siv: CursiveRunnable = cursive::default();
    siv.add_layer(Dialog::around(TextView::new("Hello, world!"))
        .title("Cursive")
        .button("Quit", |s| s.quit()));
    siv.run();
}
