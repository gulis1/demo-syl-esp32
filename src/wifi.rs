use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::wifi::Configuration as WifiConfiguration;
use esp_idf_svc::wifi::{AuthMethod, BlockingWifi, ClientConfiguration, EspWifi};
use anyhow::Result;

pub fn wifi_connect<'a>(ssid: &'a str, passwd: &'a str) -> Result<BlockingWifi<EspWifi<'a>>> {
    let peripherals = Peripherals::take()?;
    let sys_loop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take()?;

    let mut wifi = BlockingWifi::wrap(
        EspWifi::new(peripherals.modem, sys_loop.clone(), Some(nvs))?,
        sys_loop,
    )?;

    wifi.start()?;
    wifi.set_configuration(&WifiConfiguration::Client(ClientConfiguration {
        ssid: ssid.try_into().unwrap(),
        password: passwd.try_into().unwrap(),
        auth_method: AuthMethod::WPA2Personal,
        ..Default::default()
    }))?;


    wifi.connect()?;
    wifi.wait_netif_up()?;
    Ok(wifi)
}

