extern crate gtk;

use gtk::prelude::*;

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    // Consider using asset_str from `static_assets` crate, so your
    // crate won't recompile in debug mode after an XML change.
    // As of static_assets 0.1.1:
    // let glade_src = &asset_str!("src/test.glade");
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

    // Gives GTK control over program execution:
    gtk::main();
    // Alternatively, to have Rust code control your program and its GTK+ parts, use:
    // loop {
    //     // do some work here, e.g.
    //     while let Ok(ev) = rx.try_recv() {
    //         // ...
    //     }
    //     // then:
    //     gtk::main_iteration_do(false);
    //     sleep_ms(4); // May be unnecessary in your use case.
    //                  // 4ms for consistency - some OS round values to multiplies of 4
    //                  // with e.g. 1ms turned into 4ms
    // }
}
