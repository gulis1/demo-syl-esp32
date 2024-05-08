# Firmware demo para una ESP32 hecho en Rust.

Un pequeño firmare de ejemplo, utilizando los bindingds de ESP-IDF para Rust. El firmware:

1. Se conecta a una red WIFI.
2. Envía mensajes de sensores simulados por MQTT.
3. Se suscribe a un topic MQTT y muestra los mensajes que recibe.

## Requisitos

- [Toolchain de Rust para extensa y ESP](https://github.com/esp-rs/rust-build): Instrucciones de instalación en el enlace.

- [Ldproxy](https://lib.rs/crates/ldproxy):

      $ cargo install ldproxy

- [espflash](https://github.com/esp-rs/espflash/blob/main/espflash/README.md):

      $ cargo install espflash

## Ejecución

Tras instalar los requisitos, clonar el repositorio. Para compilar, flashear y monitorizar la placa, nos situamos en el directorio del repo y lanzamos el siguiente comando:

    $ cargo run

## Más información

La documentación de ESP-IDF para Rust y otros ejemplos se pueden encontrar en el repositorio [esp-idf-svc](https://github.com/esp-rs/esp-idf-svc).
