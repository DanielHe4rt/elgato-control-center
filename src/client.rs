use crate::devices::{Light, LightBucket};

#[derive(Clone, Debug)]
pub struct ElgatoClient {
    endpoint: String,
    client: reqwest::blocking::Client,
    pub light: Light,
}


impl ElgatoClient {
    pub fn new(ip: &str) -> Self {
        let client = reqwest::blocking::Client::new();
        let endpoint = format!("http://{}:9123{}", ip, "/elgato/lights");

        let response = client.get(endpoint.clone()).send().unwrap();
        let light_bucket: LightBucket = response.json().unwrap();

        Self {
            endpoint,
            client,
            light: Light::from(light_bucket.lights[0].clone()),
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
            _ => panic!("Unknown light type")
        }

        self.client.put(&self.endpoint)
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
            _ => panic!("Unknown light type")
        }

        self.client.put(&self.endpoint)
            .json(&self.prepare_payload())
            .send()
            .unwrap();
    }

    fn prepare_payload(&self) -> LightBucket {
        LightBucket {
            number_of_lights: 1,
            lights: vec![self.light.clone()],
        }
    }
}