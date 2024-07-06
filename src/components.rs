use std::cell::RefCell;
use std::rc::Rc;

use gtk4 as gtk;
use gtk4::prelude::{ButtonExt, RangeExt, WidgetExt};
use gtk4::{Button, Orientation, Scale};
use log::debug;

use crate::client::ElgatoClient;

pub fn create_temperature_scale(client: Rc<RefCell<ElgatoClient>>) -> Scale {
  let scale = Scale::new(Orientation::Horizontal, None::<&gtk::Adjustment>);
  scale.set_range(143.0, 344.0); // Set the range of the volume bar (0 to 100)

  scale.set_width_request(300);
  scale.set_value(50.0); // Set the initial value of the volume bar

  // Connect to the "value-changed" signal of the scale
  scale.connect_value_changed(move |_| {
    client.borrow_mut().toggle();
  });
  scale
}

pub fn create_brightness_scale(client: Rc<RefCell<ElgatoClient>>) -> Scale {
  let scale = Scale::new(Orientation::Horizontal, None::<&gtk::Adjustment>);
  scale.set_range(0.0, 100.0); // Set the range of the volume bar (0 to 100)

  scale.set_width_request(300);
  scale.set_value(50.0); // Set the initial value of the volume bar

  // Connect to the "value-changed" signal of the scale
  scale.connect_value_changed(move |s| {
    let value = s.value() as u32;
    debug!("Brightness: {}", value);
    client.borrow_mut().set_brightness(value);
  });
  scale
}

pub fn toggle_button(client: Rc<RefCell<ElgatoClient>>) -> Button {
  let button = Button::with_label("ON");

  button.connect_clicked(move |_| {
    client.borrow_mut().toggle();
  });

  button
}

pub fn to_kelvin(value: i32) -> i32 {
  ((-4100 * value) / 201) + 1993300 / 201
}
