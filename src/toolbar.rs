
use gtk::{
    ApplicationWindow,
    ContainerExt,
    SeparatorToolItem,
    Toolbar,
    ToolButton,
    Image,
};

use gtk::{
    ImageExt,
};

use crate::playlist::Playlist;

use gtk::{FileChooserAction, FileChooserDialog, FileFilter};
use gtk::{FileFilterExt, FileChooserExt, DialogExt, WidgetExt};
use std::path::PathBuf;

use gtk_sys::{GTK_RESPONSE_ACCEPT, GTK_RESPONSE_CANCEL};

const PLAY_STOCK: &str = "gtk-media-play";
const RESPONSE_ACCEPT: i32 = GTK_RESPONSE_ACCEPT as i32;
const RESPONSE_CANCEL: i32 = GTK_RESPONSE_CANCEL as i32;

pub struct MusicToolbar {
    pub open_button: ToolButton,
    pub next_button: ToolButton,
    pub play_button: ToolButton,
    pub previous_button: ToolButton,
    pub quit_button: ToolButton,
    pub remove_button: ToolButton,
    pub stop_button: ToolButton,
    pub toolbar: Toolbar,
}

impl MusicToolbar {
    pub fn new() -> Self {
        let toolbar = Toolbar::new();
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

        MusicToolbar{
            open_button,
            next_button,
            play_button,
            previous_button,
            quit_button,
            remove_button,
            stop_button,
            toolbar,

        }
    }

    pub fn toolbar(&self) -> &Toolbar {
        &self.toolbar
    }
}

pub fn show_open_dialog(parent: &ApplicationWindow) -> Option<PathBuf> {
    let mut file = None;

    let dialog = FileChooserDialog::new(Some("Select an MP3 Audio file"), Some(parent), FileChooserAction::Open);
    let filter = FileFilter::new();
    filter.add_mime_type("audio/mp3");
    filter.set_name("MP3 audio file");
    dialog.add_filter(&filter);
    dialog.add_button("Cancel", RESPONSE_CANCEL);
    dialog.add_button("Accept", RESPONSE_ACCEPT);

    let result = dialog.run();
    if result == RESPONSE_ACCEPT {
        file = dialog.get_filename();
    }

    dialog.destroy();
    
    file
}

pub fn set_cover(cover: &Image, playlist: &Playlist) {
    cover.set_from_pixbuf(playlist.pixbuf().as_ref());
    cover.show();
}