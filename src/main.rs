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
extern crate m3u;

use toolbar::{MusicToolbar, show_open_dialog, show_save_dialog};
use playlist::Playlist;

use std::env;
use std::rc::Rc;
use std::time::Duration;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

use gio::{ApplicationExt, ApplicationExtManual, ApplicationFlags};
use gtk::{
    Application,
    ApplicationWindow,
    WidgetExt,
    GtkWindowExt,
    ContainerExt,
    ToolButtonExt,
    Label,
    LabelExt,
};

use gtk::{
    Adjustment,
    Image,
    Scale,
    ScaleExt,
    AdjustmentExt,
    Continue,
    ButtonsType,
    DialogFlags,
    DialogExt,
    MessageDialog,
    MessageType,
};

use crate::toolbar::set_cover;
use crate::player::State;

use gtk::Orientation::{Horizontal, Vertical};


const PLAY_STOCK: &str = "gtk-media-play";
const PAUSE_STOCK: &str = "gtk-media-pause";


struct App {
    adjustment: Adjustment,
    cover: Image,
    current_time_label: Label,
    duration_label: Label,
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

        let current_time = 0;
        let durations = HashMap::new();
        let state = Arc::new(Mutex::new(State {
            current_time,
            durations,
            stopped: true,
        }));

        let playlist = Rc::new(Playlist::new(state.clone()));
        vbox.add(playlist.view());

        let cover = Image::new();
        vbox.add(&cover);

        let hbox = gtk::Box::new(Horizontal, 10);
        vbox.add(&hbox);

        let adjustment = Adjustment::new(0.0, 0.0, 10.0, 0.0, 0.0, 0.0);
        let scale = Scale::new(Horizontal, &adjustment);
        scale.set_draw_value(false);
        scale.set_hexpand(true);
        hbox.add(&scale);

        let current_time_label = Label::new(None);
        hbox.add(&current_time_label);

        let slash_label = Label::new("/");
        hbox.add(&slash_label);

        let duration_label = Label::new(None);
        duration_label.set_margin_right(10);
        hbox.add(&duration_label);

        window.show_all();

        let app = App {
            adjustment,
            cover,
            current_time_label,
            duration_label,
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
        let current_time_label = self.current_time_label.clone();
        let duration_label = self.duration_label.clone();
        let playlist = self.playlist.clone();
        let adjustment = self.adjustment.clone();
        let state = self.state.clone();
        let play_button = self.toolbar.play_button.clone();
        gtk::timeout_add(100, move || {
            let state = state.lock().unwrap();
            if let Some(path) = playlist.path() {
                if let Some(&duration) = state.durations.get(&path) {
                    adjustment.set_upper(duration as f64);
                    duration_label.set_text(&millis_to_minutes(duration));
                }
            }

            if state.stopped {
                play_button.set_stock_id(PLAY_STOCK);
            } else {
                play_button.set_stock_id(PAUSE_STOCK);
                current_time_label.set_text(&millis_to_minutes(state.current_time));

            }

            adjustment.set_value(state.current_time as f64);
            Continue(true)
        });
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

            if state.lock().unwrap().stopped {
                if playlist.play() {
                    play_button.set_stock_id(PAUSE_STOCK);
                    set_cover(&cover, &playlist);
                } 
            } else {
                playlist.pause();
                play_button.set_stock_id(PLAY_STOCK);
            }
        });

        let parent = self.window.clone();
        let playlist = self.playlist.clone();
        self.toolbar.open_button.connect_clicked(move |_| {
            let file = show_open_dialog(&parent);
            if let Some(file) = file {
                if let Some(ext) = file.extension() {
                    match ext.to_str().unwrap() {
                        "mp3" => playlist.add(&file),
                        "m3u" => playlist.load(&file),
                        extension => {
                            let dialog = MessageDialog::new(Some(&parent), DialogFlags::empty(), 
                                                            MessageType::Error,
                                                            ButtonsType::Ok, 
                                                            &format!("Cannot open file with extension .{}", extension));
                            dialog.run();
                            dialog.destroy();
                        }
                    }
                }
            }
        });

        let parent = self.window.clone();
        let playlist = self.playlist.clone();
        self.toolbar.save_button.connect_clicked(move |_| {
            let file = show_save_dialog(&parent);
            if let Some(file) = file {
                playlist.save(&file);
            }
        });

        let playlist = self.playlist.clone();
        self.toolbar.remove_button.connect_clicked(move |_| {
            playlist.remove_selection();
        });

        let current_time_label = self.current_time_label.clone();
        let duration_label = self.current_time_label.clone();
        let playlist = self.playlist.clone();
        let cover = self.cover.clone();
        let play_button = self.toolbar.play_button.clone();
        self.toolbar.stop_button.connect_clicked(move |_| {
            current_time_label.set_text("");
            duration_label.set_text("");
            playlist.stop();
            cover.hide();
            play_button.set_stock_id(PLAY_STOCK);
        });

        let playlist = self.playlist.clone();
        let cover = self.cover.clone();
        let play_button = self.toolbar.play_button.clone();
        self.toolbar.next_button.connect_clicked(move |_| {
            if playlist.next() {
                play_button.set_stock_id(PAUSE_STOCK);
                set_cover(&cover, &playlist);
            }
        });

        let playlist = self.playlist.clone();
        let cover = self.cover.clone();
        self.toolbar.previous_button.connect_clicked(move |_| {
            if playlist.previous() {
                set_cover(&cover, &playlist);
            }
        });
    }
}

fn to_millis(duration: Duration) -> u64 {
    duration.as_secs() * 1000 + duration.subsec_nanos() as u64 / 1_000_000
}

fn millis_to_minutes(millis: u64) -> String {
    let mut seconds = millis / 1_000;
    let minutes = seconds / 60;
    seconds %= 60;
    format!("{}:{:02}", minutes, seconds)
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
