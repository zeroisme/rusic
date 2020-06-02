mod toolbar;

extern crate gio;
extern crate gtk;

use toolbar::MusicToolbar;

use std::env;

use gio::{ApplicationExt, ApplicationExtManual, ApplicationFlags};
use gtk::{
    Application,
    ApplicationWindow,
    WidgetExt,
    GtkWindowExt,
    ContainerExt,
    SeparatorToolItem,
    Toolbar,
    ToolButton,
    ToolButtonExt,
};

const PLAY_STOCK: &str = "gtk-media-play";
const PAUSE_STOCK: &str = "gtk-media-pause";

struct App {
    toolbar: MusicToolbar,
    window: ApplicationWindow,
}

impl App {
    fn new(application: Application) -> Self {
        let window = ApplicationWindow::new(&application);
        window.set_title("Rusic");

        let toolbar = MusicToolbar::new();
        window.add(toolbar.toolbar());

        window.show_all();

        let app = App {
            toolbar,
            window,
        };

        app.connect_events();
        app.connect_toolbar_events();
        app
    }

    fn connect_events(&self) {
        
    }

    pub fn connect_toolbar_events(&self) {
        let window = self.window.clone();
        self.toolbar.quit_button.connect_clicked(move |_| {
            window.destroy();
        });

        let play_button = self.toolbar.play_button.clone();
        self.toolbar.play_button.connect_clicked( move |_| {
            if play_button.get_stock_id() == Some(PLAY_STOCK.to_string()) {
                play_button.set_stock_id(PAUSE_STOCK);
            } else {
                play_button.set_stock_id(PLAY_STOCK);
            }
        });
    }
}

fn main() {
    let application = Application::new("com.zero.rusic", ApplicationFlags::empty())
        .expect("Application initialization failed");
    application.connect_startup(|application| {
        let window = ApplicationWindow::new(&application);
        window.set_title("Rusic");

        let toolbar = Toolbar::new();
        window.add(&toolbar);

        let open_button = ToolButton::new_from_stock("gtk-open");
        toolbar.add(&open_button);

        toolbar.add(&SeparatorToolItem::new());

        let previous_button = ToolButton::new_from_stock("gtk-media-previous");
        toolbar.add(&previous_button);

        let play_button = ToolButton::new_from_stock(PLAY_STOCK);
        toolbar.add(&play_button);

        let stop_button = ToolButton::new_from_stock("gtk-media-stop");
        toolbar.add(&stop_button);

        let next_button = ToolButton::new_from_stock("gtk-media-next");
        toolbar.add(&next_button);

        toolbar.add(&SeparatorToolItem::new());

        let remove_button = ToolButton::new_from_stock("gtk-remove");
        toolbar.add(&remove_button);

        toolbar.add(&SeparatorToolItem::new());

        let quit_button = ToolButton::new_from_stock("gtk-quit");
        toolbar.add(&quit_button);

        window.show_all();
    });

    application.connect_activate(|_| {});
    application.run(&env::args().collect::<Vec<_>>());
}
