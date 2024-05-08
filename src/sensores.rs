use std::time::Duration;
use std::sync::mpsc;
use esp_idf_svc::timer::{EspTimerService, EspTimer};
use rand;
use anyhow::Result;
use serde_json::{Value as Json, json};

use crate::Message;

fn leer_temp() -> f32 {
    let mul: f32 = rand::random();
    let temp = mul * 30.0;
    log::info!("Temperatura leída: {temp}C");
    temp
}

fn leer_humedad() -> i32 {
    let mul: f32 = rand::random();
    let hum = (mul * 100.0) as i32;
    log::info!("Temperatura leída: {hum}%");
    hum
}

fn leer_sensores() -> Json {
    json!({
        "temperatura": leer_temp(),
        "humedad": leer_humedad()
    })
}

pub struct Sensores<'a> {
    timer: EspTimer<'a>,
}
impl<'a> Drop for Sensores<'a> {
    fn drop(&mut self) {
        let _ = self.timer.cancel();
    }
}

impl<'a> Sensores<'a> {

    pub fn new(duration: Duration, sender: mpsc::Sender<Message>) -> Result<Self> {let timer_service = EspTimerService::new()?;
        
        let timer = timer_service
            .timer(move || {
                let json = leer_sensores();
                let msg = Message::LecturaSensores(json);
                sender.send(msg).expect("Failed to send message.");
            })?;

        timer.every(duration)?;
        Ok(Self { timer })
    }
}


