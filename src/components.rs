use std::cell::RefCell;
use std::rc::Rc;
use std::str::FromStr;
use gtk4 as gtk;
use gtk4::prelude::{ButtonExt, ColorChooserExt, DialogExt, DrawingAreaExtManual, GdkCairoContextExt, GtkWindowExt, RangeExt, RendererExt, WidgetExt};
use gtk4::{ApplicationWindow, Button, ColorChooserDialog, gdk, Orientation, Scale};
use log::debug;
use palette::{Hsl, IntoColor, Srgb};
use palette::rgb::Rgb;
use crate::client::ElgatoClient;

pub fn create_temperature_scale(client: Rc<RefCell<ElgatoClient>>) -> Scale {
  let scale = Scale::new(Orientation::Horizontal, None::<&gtk::Adjustment>);
  scale.set_range(143.0, 344.0); // Set the range of the volume bar (0 to 100)

  scale.set_width_request(300);
  scale.set_value(50.0); // Set the initial value of the volume bar

  // Connect to the "value-changed" signal of the scale
  scale.connect_value_changed(move |scale| {
    client.borrow_mut().set_temperature(scale.value() as i32);
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

  let label = match client.borrow_mut().light.get_status() {
    true => "OFF",
    false => "ON"
  };

  button.set_label(label);


  button.connect_clicked(move |btn| {
    let label = match client.borrow_mut().light.get_status() {
      true => "ON",
      false => "OFF"
    };

    btn.set_label(label);
    client.borrow_mut().toggle();
  });

  button
}

pub fn to_kelvin(value: i32) -> i32 {
  ((-4100 * value) / 201) + 1993300 / 201
}


pub fn create_color_chooser(
  client: Rc<RefCell<ElgatoClient>>,
  window: Rc<ApplicationWindow>,
) -> Button {
  let rgb_button = Button::with_label("Choose Color");

  let drawing_area = gtk::DrawingArea::builder()
    .content_height(24)
    .content_width(24)
    .build();

  let rgba = gdk::RGBA::from_str("#FF0000").unwrap();
  drawing_area.set_draw_func(move |_, cr, _width, _height| {
    cr.set_source_color(&rgba);
    cr.paint().expect("Invalid cairo surface state");
  });
  rgb_button.set_child(Some(&drawing_area));


  rgb_button.connect_clicked(move |btn| {
    let color_dialog = ColorChooserDialog::new(Some("Choose a Color"), Some(window.as_ref()));
    let client = client.clone();

    let btn = btn.clone();

    color_dialog.connect_response(move |dialog, response| {
      debug!("Response: {:?}", response);

      if response == gtk::ResponseType::Ok {
        let color_rgb = ColorChooserExt::rgba(dialog);
        let hex = color_rgb.to_string();
        debug!("Color: {:?}", hex);
        let color_hsl: Hsl = Srgb::new(color_rgb.red(), color_rgb.green(), color_rgb.blue()).into_color();

        debug!("{:?}", color_hsl.hue);
        debug!("{:?}", color_hsl.saturation);
        client
          .borrow_mut()
          .set_color(color_hsl.saturation * 100.0, color_hsl.hue.into_inner());

        let red_hex = format!("{:02X}", (color_rgb.red() * 255.0) as u8);
        let green_hex = format!("{:02X}", (color_rgb.green() * 255.0) as u8);
        let blue_hex = format!("{:02X}", (color_rgb.blue() * 255.0) as u8);

        let hex = format!("#{}{}{}", red_hex, green_hex, blue_hex);
        debug!("{:?}", hex);

        let rgba = gdk::RGBA::from_str(hex.as_str()).unwrap();
        let drawing_area = gtk::DrawingArea::builder()
          .content_height(24)
          .content_width(24)
          .build();
        drawing_area.set_draw_func(move |_, cr, _width, _height| {
          cr.set_source_color(&rgba);
          cr.paint().expect("Invalid cairo surface state");
        });
        btn
          .set_child(Some(&drawing_area));
      }

      dialog.close();
    });
    color_dialog.show();
  });

  rgb_button
}
