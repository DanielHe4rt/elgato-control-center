use std::cell::RefCell;
use std::rc::Rc;
use std::str::FromStr;

use gtk::{Application, ApplicationWindow, glib};
use gtk4 as gtk;
use gtk4::{ColorChooser, ColorChooserDialog, ColorChooserWidget, Label, Orientation};
use gtk4::builders::ColorChooserWidgetBuilder;
use gtk4::cairo::{Gradient, RadialGradient};
use gtk::prelude::*;
use serde::{Deserialize, Serialize};

mod client;
mod network;
mod devices;
mod components;

fn main() -> glib::ExitCode {
    let application = Application::builder()
        .application_id("com.danielheart.elpobre")
        .build();

    application.connect_activate(application_interface());

    application.run()
}


fn application_interface() -> fn(&Application) {
    |app| {
        let container = gtk::Box::new(Orientation::Vertical, 10);
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Elgato Control Center de pobre")
            .default_width(400)
            .default_height(600)
            .build();


        //let devices = get_active_devices();
        let devices = vec![
            client::ElgatoClient::new("192.168.1.11"),
            client::ElgatoClient::new("192.168.1.63"),
        ];

        for device in devices {
            println!("Device: {:?}", device);
            let grid = gtk::Grid::builder()
                .margin_start(6)
                .margin_end(6)
                .margin_top(6)
                .margin_bottom(6)
                .halign(gtk::Align::Start)
                .valign(gtk::Align::Start)
                .row_spacing(6)
                .column_homogeneous(true)
                .column_spacing(6)
                .build();
            match &device.light {
                devices::Light::Keylight(light) => {
                    println!("Lightstrip: {:?}", light);
                    let client = Rc::new(RefCell::new(device));


                    let button = components::toggle_button(client.clone());

                    let brightness_box = gtk::Box::new(Orientation::Horizontal, 100);
                    brightness_box.append(&components::create_brightness_scale(client.clone()));

                    let temperature_box = gtk::Box::new(Orientation::Horizontal, 100);
                    temperature_box.append(&components::create_temperature_scale(client.clone()));

                    grid.attach(&button, 0, 1, 1, 4);
                    grid.attach(&brightness_box, 2, 2, 2, 1);
                    grid.attach(&temperature_box, 2, 3, 2, 1);
                }
                devices::Light::LightStrip(light) => {
                    println!("Lightstrip: {:?}", light);
                    let client = Rc::new(RefCell::new(device));

                    let button = components::toggle_button(client.clone());

                    let brightness_box = gtk::Box::new(Orientation::Horizontal, 100);
                    brightness_box.append(&components::create_brightness_scale(client.clone()));


                    grid.attach(&button, 0, 1, 1, 4);
                    grid.attach(&brightness_box, 2, 2, 2, 1);

                }
            }
            container.append(&grid);
        }

        let mut title = Label::default();


        // Create a new scale (volume bar)

        // Add the scale to the vertical box
        title.set_text("Elgato Control Pobre");



        window.set_child(Some(&container));
        window.present();
    }
}




