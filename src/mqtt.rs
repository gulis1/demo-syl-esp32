use embedded_svc::mqtt::client::EventPayload;
use esp_idf_svc::{
    mqtt::client::{EspMqttClient, MqttClientConfiguration},
    sys::EspError,
};
use std::sync::mpsc;
use crate::Message;
use std::thread;

pub fn mqtt_init(url: &str, client: Option<&str>, sender: mpsc::Sender<Message>) 
    -> Result<EspMqttClient<'static>, EspError>
{
    let (client, mut conn) = EspMqttClient::new(
        url,
        &MqttClientConfiguration {
            client_id: client,
            ..Default::default()
        },
    )?;
    
    thread::Builder::new()
        .stack_size(6000)
        .spawn(move || {

            log::info!("A la espera de mensajes MQTT."); 
            while let Ok(event) = conn.next() {
                log::info!("Evento MQTT recibido: {:?}", event.payload());
                match event.payload() {
                    EventPayload::Subscribed(_) => log::info!("Suscrito a tópic correctamente."),
                    EventPayload::Received { id: _, topic: _, data, details: _ } => {
                        let msg = std::str::from_utf8(data).unwrap().to_string();
                        sender.send(Message::MensajeRecibido(msg)).unwrap();
                    },
                    _ => {}
                }
            }

            log::info!("Connection MQTT terminada.");

        })
        .unwrap();



    Ok(client)
}
