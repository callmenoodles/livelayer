use std::fs;
use std::ffi::OsStr;
use std::path::PathBuf;
use gtk4::{Application, ApplicationWindow};
use gtk4::gdk::{prelude::{Cast, MonitorExt}, Display, Monitor};
use gtk4::prelude::{
    ApplicationExt,
    ApplicationExtManual,
    DisplayExt,
    GtkWindowExt,
    ListModelExt,
    WidgetExt
};
use gtk4_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};
use webkit6::{WebView};
use webkit6::prelude::WebViewExt;
use clap::Parser;
use url as yurl;
use yurl::{Url, ParseError};
use anyhow::{Error, Result};
use gdk4_wayland::glib::ExitCode;
use gtk4::gio::ListModel;
use gtk4::glib::GString;

#[derive(Parser, Debug)]
#[command(author, version, about = "HTML Wallpaper Setter for Wayland", long_about = None)]
struct Args {
    #[arg(short = 'o', long = "output", help = "Output Monitor")]
    output: Option<String>,

    url: String,
}

fn parsed_url(url: &str) -> Result<String> {
    let parsed_url: Result<Url, ParseError> = Url::parse(&url);

    match parsed_url {
        Ok(url) => {
            if url.scheme() == "http" || url.scheme() == "https" {
                return Ok(url.to_string());
            }

            Err(Error::from(ParseError::RelativeUrlWithoutBase))
        }
        Err(_) => {
            let path: std::io::Result<PathBuf> = fs::canonicalize(url);

            match path {
                Ok(path) => {
                    let extension: Option<&OsStr> = path.extension();

                    if let Some(extension) = extension {
                        if extension == "html" {
                            Ok(format!("file://{}", path.display()))
                        } else {
                            Err(Error::from(ParseError::RelativeUrlWithoutBase))
                        }
                    } else {
                        Err(Error::from(ParseError::RelativeUrlWithoutBase))
                    }
                },
                Err(e) => Err(Error::from(e))
            }
        }
    }
}

fn get_monitor(name: Option<String>) -> Option<Monitor> {
    let display: Display = Display::default().expect("Could not connect to a display");
    let monitors: ListModel = display.monitors();


    if let Some(output_name) = name {
        for i in 0..monitors.n_items() {
            if let Some(obj) = monitors.item(i) {
                match obj.downcast::<Monitor>() {
                    Ok(monitor) => {
                        let connector: Option<GString> = monitor.connector();
                        
                        match connector {
                            Some(connector) => {
                                if connector.as_str() == output_name.as_str() {
                                    return Some(monitor)
                                }
                            }
                            None => return None
                        }
                    }
                    Err(_) => {
                        return None
                    }
                }
            }
            else {
                return None
            }
        }
    }

    None
}

fn main() -> Result<ExitCode> {
    gtk4::init().expect("Failed to initialize GTK");

    let output_name: Option<String> = Args::parse().output;
    let output_monitor: Option<Monitor> = get_monitor(output_name);

    let application: Application = Application::builder()
        .flags(gtk4::gio::ApplicationFlags::HANDLES_OPEN)
        .build();

    application.connect_activate(move |app: &Application| {
        let window: ApplicationWindow = ApplicationWindow::builder()
            .application(app)
            .build();

        window.init_layer_shell();
        window.set_can_target(true);
        window.set_can_focus(false);
        window.set_hide_on_close(false);
        window.set_destroy_with_parent(true);

        window.set_monitor(output_monitor.as_ref());
        window.set_layer(Layer::Background);
        window.set_anchor(Edge::Top, true);
        window.set_anchor(Edge::Bottom, true);
        window.set_anchor(Edge::Left, true);
        window.set_anchor(Edge::Right, true);
        window.set_keyboard_mode(KeyboardMode::None);

        let webview: WebView = WebView::new();
        webview.set_size_request(window.width(), window.height());
        webview.set_can_focus(false);
        webview.set_editable(false);
        webview.set_can_target(true);

        let url: String = Args::parse().url;
        let parsed_url: Result<String> = parsed_url(&url);

        if let Ok(url) = parsed_url {
            webview.load_uri(&url);
        }

        window.set_child(Some(&webview));
        window.present();
    });

    Ok(application.run_with_args::<String>(&[]))  // Prevent GTK from parsing arguments
}