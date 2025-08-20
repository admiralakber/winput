use clap::Parser;
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, CssProvider, Entry};
use gtk4::{Align, Orientation};
use gtk4::{Box as GtkBox};
use std::io::{self, Write};
use std::process;

#[derive(Parser, Debug)]
#[command(name = "winput", version, about = "Slick minimal GUI prompt for Wayland")] 
struct Args {
    /// Placeholder text inside the input
    #[arg(short = 'p', long = "placeholder", default_value = "Type and press Enter")] 
    placeholder: String,

    /// Initial text value
    #[arg(short = 't', long = "text", default_value = "")]
    initial_text: String,

    /// Tooltip text (shown on hover)
    #[arg(short = 'T', long = "tooltip", default_value = "")]
    tooltip: String,

    /// Window width in pixels
    #[arg(short = 'w', long = "width", default_value_t = 520i32)]
    width: i32,

    /// Window height in pixels
    #[arg(short = 'h', long = "height", default_value_t = 56i32)]
    height: i32,

    /// Font family name to use
    #[arg(short = 'f', long = "font", default_value = "Inconsolata")]
    font_family: String,

    /// Font size in points
    #[arg(short = 's', long = "font-size", default_value_t = 14i32)]
    font_size: i32,

    /// Dark mode background (CSS color)
    #[arg(long = "bg", default_value = "#1e1e2e")]
    bg: String,

    /// Foreground text color (CSS color)
    #[arg(long = "fg", default_value = "#cdd6f4")]
    fg: String,

    /// Accent color for focus ring/caret
    #[arg(long = "accent", default_value = "#89b4fa")]
    accent: String,

    /// Corner radius
    #[arg(long = "radius", default_value_t = 8i32)]
    radius: i32,

    /// Window titlebar decorations
    #[arg(long = "decorated", default_value_t = false)]
    decorated: bool,

    /// Print trailing newline after text
    #[arg(long = "newline", default_value_t = false)]
    newline: bool,
}

fn main() {
    let args = Args::parse();

    // Use a fixed app id for Wayland portals focus handling; not strictly required.
    let app = Application::builder().application_id("dev.brat.winput").build();

    app.connect_activate(move |app| {
        // CSS styling: flat, dark, rounded, monospace, modern.
        let css = format!(
            "
            window#winput {{
                background: transparent;
            }}
            box#winput-box {{
                background: {bg};
                border-radius: {radius}px;
                border: 1px solid rgba(255,255,255,0.08);
                box-shadow: 0 8px 30px rgba(0,0,0,0.45);
                padding: 8px 10px;
            }}
            entry#winput-entry {{
                background: transparent;
                border: none;
                color: {fg};
                caret-color: {accent};
                selection-background-color: {accent};
                selection-color: {bg};
                font-family: '{font}', monospace;
                font-size: {font_size}pt;
                min-height: 28px;
            }}
            entry#winput-entry:focus {{
                outline: none;
            }}
            ",
            bg = args.bg,
            fg = args.fg,
            accent = args.accent,
            radius = args.radius,
            font = args.font_family,
            font_size = args.font_size
        );

        let provider = CssProvider::new();
        provider.load_from_data(&css);
        gtk4::style_context_add_provider_for_display(
            &gtk4::gdk::Display::default().unwrap(),
            &provider,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        // Container to allow rounded background around the entry
        let container = GtkBox::new(Orientation::Vertical, 0);
        container.set_widget_name("winput-box");
        container.set_margin_top(8);
        container.set_margin_bottom(8);
        container.set_margin_start(8);
        container.set_margin_end(8);

        // Create the entry
        let entry = Entry::new();
        entry.set_widget_name("winput-entry");
        entry.set_placeholder_text(Some(&args.placeholder));
        if !args.initial_text.is_empty() { entry.set_text(&args.initial_text); }
        if !args.tooltip.is_empty() { entry.set_tooltip_text(Some(&args.tooltip)); }
        entry.set_halign(Align::Fill);
        entry.set_hexpand(true);
        container.append(&entry);

        let window = ApplicationWindow::builder()
            .application(app)
            .title("winput")
            .decorated(args.decorated)
            .resizable(false)
            .default_width(args.width)
            .default_height(args.height)
            .child(&container)
            .build();

        // Pressing Enter should print the content and exit (0). Escape exits (1).
        {
            let window = window.clone();
            entry.connect_activate(move |e| {
                let text = e.text().to_string();
                // Print to stdout; optional trailing newline.
                if args.newline { println!("{}", text); } else { print!("{}", text); }
                let _ = io::stdout().flush();
                // Exit successfully
                window.close();
                process::exit(0);
            });
        }

        // Allow Escape to cancel (exit code 1) via key controller.
        {
            use gtk4::gdk::Key;
            let window_weak = window.downgrade();
            entry.add_controller({
                let key = gtk4::EventControllerKey::new();
                key.connect_key_pressed(move |_, keyval, _keycode, _state| {
                    if keyval == Key::Escape {
                        if let Some(window) = window_weak.upgrade() {
                            window.close();
                        }
                        process::exit(1);
                    }
                    gtk4::glib::Propagation::Proceed
                });
                key
            });
        }

        // Ensure the window is tiny and centered where possible, and focus entry.
        window.set_widget_name("winput");
        window.present();
        entry.grab_focus();
    });

    app.run();
}
