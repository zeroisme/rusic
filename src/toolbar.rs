use gtk::{
    ContainerExt,
    SeparatorToolItem,
    Toolbar,
    ToolButton,
};

const PLAY_STOCK: &str = "gtk-media-play";

pub struct MusicToolbar {
    open_button: ToolButton,
    next_button: ToolButton,
    pub play_button: ToolButton,
    previous_button: ToolButton,
    pub quit_button: ToolButton,
    remove_button: ToolButton,
    stop_button: ToolButton,
    toolbar: Toolbar,
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