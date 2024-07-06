use crate::devices::{Light, LightBucket};
use log::debug;

#[derive(Clone, Debug)]
pub struct ElgatoClient {
  endpoint: String,
  client: reqwest::blocking::Client,
  pub light: Light,
}

impl Light {
  pub fn get_status(&self) -> bool {
    match self {
      Light::Keylight(light) => {
        light.on == 1
      }
      Light::LightStrip(light) => {
        light.on == 1
      }
    }
  }
}

impl ElgatoClient {
  pub fn new(ip: &str) -> Self {
    let client = reqwest::blocking::Client::new();
    let endpoint = format!("http://{ip}:9123/elgato/lights");

    let response = client.get(endpoint.clone()).send().unwrap();
    let light_bucket: LightBucket = response.json().unwrap();

    Self {
      endpoint,
      client,
      light: light_bucket.lights[0].clone(),
    }
  }

  pub fn toggle(&mut self) {
    match &self.light {
      Light::Keylight(light) => {
        let mut light = light.clone();
        light.toggle();
        self.light = Light::Keylight(light);
      }
      Light::LightStrip(light) => {
        let mut light = light.clone();
        light.toggle();
        self.light = Light::LightStrip(light);
      }
    }

    self
      .client
      .put(&self.endpoint)
      .json(&self.prepare_payload())
      .send()
      .unwrap();
  }

  pub fn set_brightness(&mut self, brightness: u32) {
    match &self.light {
      Light::Keylight(light) => {
        let mut light = light.clone();
        light.set_brightness(brightness);
        self.light = Light::Keylight(light);
      }
      Light::LightStrip(light) => {
        let mut light = light.clone();
        light.set_brightness(brightness);
        self.light = Light::LightStrip(light);
      }
    }

    self
      .client
      .put(&self.endpoint)
      .json(&self.prepare_payload())
      .send()
      .unwrap();
  }

  pub fn set_temperature(&mut self, temperature: i32) {
    match &self.light {
      Light::Keylight(light) => {
        let mut light = light.clone();
        light.set_temperature(temperature);
        self.light = Light::Keylight(light);
      }
      Light::LightStrip(light) => {
        debug!("Temperature is not supported for LightStrip");
      }
    }

    self
      .client
      .put(&self.endpoint)
      .json(&self.prepare_payload())
      .send()
      .unwrap();
  }

  pub fn set_color(&mut self, saturation: f32, hue: f32) {
    match &self.light {
      Light::Keylight(_) => {
        debug!("Color is not supported for Keylight");
      }
      Light::LightStrip(light) => {
        let mut light = light.clone();
        light.set_color(hue, saturation);
        self.light = Light::LightStrip(light);
      }
    }

    let response = self
      .client
      .put(&self.endpoint)
      .json(&self.prepare_payload())
      .send()
      .unwrap();

    debug!("{:?}", response.text());
  }

  fn prepare_payload(&self) -> LightBucket {
    LightBucket {
      number_of_lights: 1,
      lights: vec![self.light.clone()],
    }
  }
}
