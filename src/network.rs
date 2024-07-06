use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream};
use std::time::Duration;

use anyhow::Result;
use log::{debug, error};
use pnet::datalink;

use crate::client;
use crate::client::ElgatoClient;

pub fn get_active_devices() -> Vec<ElgatoClient> {
  let mut active_devices = vec![];
  let network = get_interface();
  let devices = get_devices(network.unwrap());

  #[allow(clippy::never_loop)]
  for device in devices.iter() {
    debug!("Device: {:?}", device);
    active_devices.append(&mut vec![client::ElgatoClient::new(
      device.to_string().as_str(),
    )]);
  }
  active_devices
}

fn get_devices(base_ip: IpAddr) -> Vec<IpAddr> {
  debug!("My IP: {:?}", base_ip);
  let mut devices_addresses = vec![];

  debug!("Scanning network for devices...");
  for i in 1..255 {
    let my_ip = base_ip.to_string();
    let split_ip: Vec<&str> = my_ip.split('.').collect();
    let a = split_ip[0].to_string().parse::<u8>().unwrap();
    let b = split_ip[1].to_string().parse::<u8>().unwrap();
    let c = split_ip[2].to_string().parse::<u8>().unwrap();

    let ip = IpAddr::V4(Ipv4Addr::new(a, b, c, i));

    let timeout = Duration::from_millis(100);
    let socket_addr = &SocketAddr::new(ip, 9123);

    let is_open = TcpStream::connect_timeout(socket_addr, timeout);

    if is_open.is_ok() {
      debug!("Found device at: {:?}", ip);
      devices_addresses.append(&mut vec![ip]);
    }
  }

  devices_addresses
}

fn get_interface() -> Result<IpAddr> {
  // Get a list of all network interfaces available on the system
  let interfaces = datalink::interfaces();

  // Find the first interface that is up, not loopback, and has an IP address
  let default_interface = interfaces
    .into_iter()
    .find(|iface| iface.is_up() && !iface.is_loopback() && !iface.ips.is_empty());

  if default_interface.is_none() {
    error!("Something went wrong");
  }

  let interface = default_interface.unwrap();
  debug!("Default interface: {}", interface.name);
  let ipv4 = interface.ips.iter().find(|ip| ip.is_ipv4());

  Ok(ipv4.unwrap().clone().ip())
}
