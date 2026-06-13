use gtk4::prelude::*;
use gtk4::{
    gdk,
    glib,
    Box as GtkBox,
    Button,
    CssProvider,
    Image,
    Orientation,
    Window,
};

use gtk4_layer_shell::{Edge, Layer, LayerShell};

fn create_app_button(
    icon_name: &str,
    command: &'static str,
    icon_size: i32,
) -> Button {
    let image = Image::from_icon_name(icon_name);
    image.set_pixel_size(icon_size);

    let button = Button::new();
    button.set_child(Some(&image));

      button.connect_clicked(move |_| {
      let cmd = format!(
    "[workspace +1] {}",
    command
);

        let _ = std::process::Command::new("hyprctl")
            .args(["dispatch", "exec", &cmd])
            .spawn();
    });

    button
}

fn main() {
    gtk4::init().expect("Failed to initialize GTK");

    // CSS
    let provider = CssProvider::new();

provider.load_from_data(include_str!("dock.css"));

    gtk4::style_context_add_provider_for_display(
        &gdk::Display::default().unwrap(),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    // Window
    let window = Window::new();

    window.set_title(Some("cachydock"));
window.set_default_size(1150, 160);
   window.set_decorated(false);
   let edit_mode = true;
window.set_resizable(false);
    // Dock
    let dock = GtkBox::new(Orientation::Horizontal, 10);

    dock.set_margin_top(10);
    dock.set_margin_bottom(10);
    dock.set_margin_start(15);
    dock.set_margin_end(15);

    let icon_size = 48;

    let firefox =
    create_app_button("firefox", "firefox", icon_size);

let zen =
    create_app_button("zen-browser", "zen-browser", icon_size);

let brave =
    create_app_button("brave-browser", "brave-browser", icon_size);

let nautilus =
    create_app_button(
        "org.gnome.Nautilus",
        "nautilus",
        icon_size,
    );

let kitty =
    create_app_button("kitty", "kitty", icon_size);

let thunar =
    create_app_button(
        "org.xfce.thunar",
        "thunar",
        icon_size,
    );

let steam =
    create_app_button("steam", "steam", icon_size);

// Stremio Flatpak
let stremio_image =
    Image::from_icon_name("stremio");

stremio_image.set_pixel_size(icon_size);

let stremio = Button::new();

stremio.set_child(Some(&stremio_image));

stremio.connect_clicked(|_| {
    let _ = std::process::Command::new("flatpak")
        .args(["run", "com.stremio.Stremio"])
        .spawn();
});
let spotify =
    create_app_button(
        "spotify-client",
        "spotify",
        icon_size,
    );

let discord =
    create_app_button(
        "discord",
        "discord",
        icon_size,
    );



dock.append(&firefox);
dock.append(&zen);
dock.append(&brave);
dock.append(&nautilus);
dock.append(&kitty);

dock.append(&steam);
dock.append(&stremio);
dock.append(&spotify);
dock.append(&discord);

    window.set_child(Some(&dock));

    window.present();

    let main_loop = glib::MainLoop::new(None, false);
    main_loop.run();
}
