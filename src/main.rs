mod toolbar;
mod playlist;
mod mp3;
mod player;

extern crate gio;
extern crate gtk;
extern crate gdk_pixbuf;
extern crate id3;
extern crate gtk_sys;
extern crate crossbeam;
extern crate pulse_simple;
extern crate simplemad;

use toolbar::{MusicToolbar, show_open_dialog};
use playlist::Playlist;

use std::env;
use std::rc::Rc;
use std::time::Duration;
use std::sync::{Arc, Mutex};

use gio::{ApplicationExt, ApplicationExtManual, ApplicationFlags};
use gtk::{
    Application,
    ApplicationWindow,
    WidgetExt,
    GtkWindowExt,
    ContainerExt,
    ToolButtonExt,
};

use gtk::{
    Adjustment,
    Image,
    ImageExt,
    Scale,
    ScaleExt,
};

use crate::toolbar::set_cover;
use crate::player::State;

use gtk::Orientation::{Horizontal, Vertical};


const PLAY_STOCK: &str = "gtk-media-play";
const PAUSE_STOCK: &str = "gtk-media-pause";


struct App {
    adjustment: Adjustment,
    cover: Image,
    playlist: Rc<Playlist>,
    state: Arc<Mutex<State>>,
    toolbar: MusicToolbar,
    window: ApplicationWindow,
}

impl App {
    fn new(application: Application) -> Self {
        let window = ApplicationWindow::new(&application);
        window.set_title("Rusic");

        let vbox = gtk::Box::new(Vertical, 0);
        window.add(&vbox);

        let toolbar = MusicToolbar::new();
        vbox.add(toolbar.toolbar());

        let state = Arc::new(Mutex::new(State {
            stopped: true,
        }));

        let playlist = Rc::new(Playlist::new(state.clone()));
        vbox.add(playlist.view());

        let cover = Image::new();
        vbox.add(&cover);

        let adjustment = Adjustment::new(0.0, 0.0, 10.0, 0.0, 0.0, 0.0);
        let scale = Scale::new(Horizontal, &adjustment);
        scale.set_draw_value(false);
        vbox.add(&scale);

        window.show_all();

        let app = App {
            adjustment,
            cover,
            playlist,
            state,
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

        let playlist = self.playlist.clone();
        let cover = self.cover.clone();
        let state = self.state.clone();

        let play_button = self.toolbar.play_button.clone();
        self.toolbar.play_button.connect_clicked( move |_| {
            // if play_button.get_stock_id() == Some(PLAY_STOCK.to_string()) {
            //     play_button.set_stock_id(PAUSE_STOCK);
            //     set_cover(&cover, &playlist);
            // } else {
            //     play_button.set_stock_id(PLAY_STOCK);
            // }

            if state.lock().unwrap().stopped {
                if playlist.play() {
                    play_button.set_stock_id(PAUSE_STOCK);
                    set_cover(&cover, &playlist);
                } else {
                    play_button.set_stock_id(PLAY_STOCK);
                }
            }
        });

        let parent = self.window.clone();
        let playlist = self.playlist.clone();
        self.toolbar.open_button.connect_clicked(move |_| {
            let file = show_open_dialog(&parent);
            if let Some(file) = file {
                playlist.add(&file);
            }
        });

        let playlist = self.playlist.clone();
        self.toolbar.remove_button.connect_clicked(move |_| {
            playlist.remove_selection();
        });
    }
}

fn to_millis(duration: Duration) -> u64 {
    duration.as_secs() * 1000 + duration.subsec_nanos() as u64 / 1_000_000
}

fn main() {
    let application = Application::new("com.zero.rusic", ApplicationFlags::empty())
        .expect("Application initialization failed");
        
    application.connect_startup(|application| {
        App::new(application.clone());
    });

    application.connect_activate(|_| {});
    application.run(&env::args().collect::<Vec<_>>());
}
