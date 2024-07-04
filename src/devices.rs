use serde::{Serialize, Deserialize};

// Light Strip struct
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct LightStrip {
    pub on: u32,
    pub hue: f64,
    pub saturation: f64,
    pub brightness: u32,
}

// Keylight struct
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Keylight {
    pub on: u32,
    pub brightness: u32,
    pub temperature: i32,
}

// Enum to encapsulate both types
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum Light {
    LightStrip(LightStrip),
    Keylight(Keylight),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LightBucket {
    #[serde(rename = "numberOfLights")]
    pub number_of_lights: u32,
    pub lights: Vec<Light>,
}

trait BaseLight {
    fn toggle(&mut self);
    fn set_brightness(&mut self, brightness: u32);
}

trait KeylightTrait {
    fn set_temperature(&mut self, temperature: i32);
}

impl LightStrip {
    pub fn toggle(&mut self) {
        self.on = if self.on == 1 { 0 } else { 1 }
    }

    pub fn set_brightness(&mut self, brightness: u32) {
        self.brightness = brightness;
    }
}

impl Keylight {
    pub fn toggle(&mut self) {
        self.on = if self.on == 1 { 0 } else { 1 }
    }

    pub fn set_brightness(&mut self, brightness: u32) {
        self.brightness = brightness;
    }
}

impl KeylightTrait for Keylight {
    fn set_temperature(&mut self, temperature: i32) {
        self.temperature = temperature;
    }
}