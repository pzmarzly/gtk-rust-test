extern crate gtk;

use gtk::prelude::*;

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let glade_src = include_str!("test.glade");
    let builder = gtk::Builder::new_from_string(glade_src);

    let window: gtk::ApplicationWindow = builder.get_object("w1").unwrap();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    let revealer: gtk::Revealer = builder.get_object("r").unwrap();
    let button: gtk::Button = builder.get_object("b").unwrap();
    button.connect_clicked(move |_| {
        let cur = revealer.get_reveal_child();
        revealer.set_reveal_child(!cur);
    });

    let css = include_bytes!("test.css");
    let screen = window.get_screen().unwrap();
    let style = gtk::CssProvider::new();
    style.load_from_data(css).unwrap();
    gtk::StyleContext::add_provider_for_screen(&screen, &style, gtk::STYLE_PROVIDER_PRIORITY_USER);

    window.show_all();
    gtk::main();
}
