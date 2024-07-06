use std::cell::RefCell;
use std::rc::Rc;

use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Label, Orientation};
use gtk4 as gtk;
use log::debug;

use crate::client::ElgatoClient;
use crate::components::create_color_chooser;
use crate::network::get_active_devices;

mod client;
mod components;
mod devices;
mod network;

fn main() -> glib::ExitCode {
  colog::init();
  debug!("Starting application");

  let application = Application::builder()
    .application_id("com.danielheart.elpobre")
    .build();

  application.connect_activate(application_interface);

  application.run()
}

fn application_interface(app: &Application) {
  let container = gtk::Box::new(Orientation::Vertical, 20);

  let window = ApplicationWindow::builder()
    .application(app)
    .title("Elgato Control Center de pobre")
    .default_width(400)
    .default_height(300)
    .build();

  let window = Rc::new(window);

  //let devices = get_active_devices();
  let devices = vec![
    ElgatoClient::new("192.168.1.11"),
    ElgatoClient::new("192.168.1.72"),
  ];

  for device in devices {
    debug!("Device: {:?}", device.light);
    let grid = gtk::FlowBox::builder()
      .margin_start(6)
      .margin_end(6)
      .margin_top(6)
      .margin_bottom(6)
      .min_children_per_line(4)
      .halign(gtk::Align::Start)
      .valign(gtk::Align::Start)

      .column_spacing(6)
      .build();

    let device = Rc::new(RefCell::new(device));

    let toggle_box = gtk::Box::new(Orientation::Vertical, 200);
    toggle_box.append(&components::toggle_button(device.clone()));

    let brightness_box = gtk::Box::new(Orientation::Horizontal, 100);
    brightness_box.append(&components::create_brightness_scale(device.clone()));

    match device.borrow().light {
      devices::Light::Keylight(_) => {
        let label = Label::new(Some("Keylight"));

        container.append(&label);

        let temperature_box = gtk::Box::new(Orientation::Horizontal, 100);

        temperature_box.append(&components::create_temperature_scale(device.clone()));

        grid.insert(&toggle_box, 1);
        grid.insert(&brightness_box, 2);
        grid.insert(&temperature_box, 2);
      }
      devices::Light::LightStrip(_) => {
        container.append(&Label::new(Some("LightStrip")));

        let rgb_button = create_color_chooser(device.clone(), window.clone());

        grid.insert(&toggle_box, 1);
        grid.insert(&brightness_box, 2);
        grid.insert(&rgb_button, 3);
      }
    }
    container.append(&grid);
  }

  let title = Label::default();

  // Create a new scale (volume bar)

  // Add the scale to the vertical box
  title.set_text("Elgato Control Pobre");

  window.set_child(Some(&container));
  window.show();
}
