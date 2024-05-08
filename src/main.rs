mod mqtt;
mod wifi;
mod sensores;

use std::time::Duration;
use anyhow::Result;
use sensores::Sensores;
use std::sync::mpsc;
use embedded_svc::mqtt::client::QoS;
use mqtt::mqtt_init;
use wifi::wifi_connect;
use serde_json::Value as Json;


const WIFI_SSID: &str = "hotspotjulian";
const WIFI_PASS: &str = "12345678";

#[derive(Debug)]
pub enum Message {
    LecturaSensores(Json),
    MensajeRecibido(String)
}

fn main_loop() -> Result<()> {
    
    let (sender, recv) = mpsc::channel::<Message>();

    // Iniciamos el cliente MQTT y nos suscribimos al topic.
    let mut mqtt_client = mqtt_init("mqtt://test.mosquitto.org:1883", None, sender.clone())?;
    mqtt_client.subscribe("/SyL/rust/mensajes", QoS::AtLeastOnce)?;

    // Iniciamos los sensores simulados.
    let _sensores = Sensores::new(Duration::from_secs(5), sender);

    // Bucle principal.
    loop {
        
        let msg = recv.recv()?;
        log::info!("Mensaje recibido: {:?}", msg);
        match msg {
            Message::MensajeRecibido(msg) => log::info!("Mensaje recibido por MQTT: {msg}"),
            Message::LecturaSensores(json) => {
                let mqtt_payload = serde_json::to_string_pretty(&json)?;
                mqtt_client.publish("/SyL/rust/sensores", QoS::ExactlyOnce, false, mqtt_payload.as_bytes())?;
            }
        }

    }
}

fn main() -> Result<()> {

    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let wifi_connected = wifi_connect(WIFI_SSID, WIFI_PASS);
    match wifi_connected {
        Ok(_) => log::info!("WIFI connected!"),
        Err(e) => log::error!("Could not connect to WIFI: {e}"),
    };

    main_loop()?;
    Ok(())
}
