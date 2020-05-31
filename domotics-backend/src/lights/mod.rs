use core::time::Duration;
use rocket::response::status::NotFound;
use rocket::State;
use rocket_contrib::json::Json;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashSet;
use std::io::Error;
use std::io::Write;
use std::net::Ipv4Addr;
use std::net::SocketAddrV4;
use std::net::TcpStream;
use std::net::UdpSocket;
use std::sync::Arc;
use std::sync::Mutex;

#[get("/")]
pub fn get_all(wifi_bulbs_state: State<WifiBulbs>) -> Json<Vec<WifiBulb>> {
    let wifi_bulbs = discover();
    let mut vec_arc = wifi_bulbs_state.lock().unwrap();
    let result = wifi_bulbs.unwrap();
    let cloned = result.clone();
    *vec_arc = Arc::new(result);
    Json(cloned)
}

#[get("/<id>")]
pub fn get_one(
    id: i64,
    wifi_bulbs_state: State<WifiBulbs>,
) -> Result<Json<WifiBulb>, NotFound<String>> {
    let vec_arc = wifi_bulbs_state.lock().unwrap();
    let candidates = (*vec_arc)
        .iter()
        .filter(|bulb| bulb.id == id)
        .collect::<Vec<&WifiBulb>>();
    if candidates.len() > 0 {
        Ok(Json(candidates[0].clone()))
    } else {
        Err(NotFound(format!("Unknown bulb id: {}", id)))
    }
}

#[get("/<id>/toggle")]
pub fn toggle(id: i64, wifi_bulbs_state: State<WifiBulbs>) -> () {
    let vec_arc = wifi_bulbs_state.lock().unwrap();
    let candidates = (*vec_arc)
        .iter()
        .filter(|bulb| bulb.id == id)
        .collect::<Vec<&WifiBulb>>();
    if candidates.len() > 0 {
        perform_action(&candidates[0].address, "toggle")
    } else {
        println!("Couldn't find the light bulb");
    }
}

pub type WifiBulbs = Arc<Mutex<Arc<Vec<WifiBulb>>>>;

fn discover() -> Result<Vec<WifiBulb>, Error> {
    let mut devices = vec![];
    let mut device_ids = HashSet::new();

    let port = 1982;
    let diagram = "M-SEARCH * HTTP/1.1\r\n MAN: \"ssdp:discover\"\r\n wifi_bulb";

    let local_address = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), port);
    let ssdp_multicast_address = SocketAddrV4::new(Ipv4Addr::new(239, 255, 255, 250), port);

    let socket = UdpSocket::bind(local_address)?;
    socket.set_read_timeout(Some(Duration::new(1, 0)))?;
    socket.set_multicast_ttl_v4(12)?;
    socket.join_multicast_v4(ssdp_multicast_address.ip(), local_address.ip())?;

    socket.send_to(diagram.as_bytes(), ssdp_multicast_address)?;

    loop {
        let mut buf = [0; 1048576];
        let socket_receive = socket.recv_from(&mut buf);

        match socket_receive {
            Ok((number_of_bytes, _)) => {
                let filled_buf = &mut buf[..number_of_bytes];

                let string_buffer = std::str::from_utf8(filled_buf).unwrap();
                let parsed_option = parse(string_buffer);
                for parsed in parsed_option.iter() {
                    if device_ids.contains(&parsed.id) {
                        continue;
                    }
                    device_ids.insert(parsed.id.clone());
                    devices.push(parsed.clone());
                }
            }
            Err(_) => {
                break;
            }
        }
    }
    Ok(devices)
}

fn perform_action(address: &SocketAddrV4, method: &str) {
    if let Ok(mut stream) = TcpStream::connect(address) {
        let msg = format!(
            "{{\"id\":{},\"method\":\"{}\",\"params\":[{}]}}\r\n",
            1, method, ""
        );
        stream.write(msg.as_bytes()).unwrap();
    } else {
        println!("Couldn't connect to server...");
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct WifiBulbCommand {
    id: u32,
    method: String,
    params: String,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct WifiBulb {
    pub id: i64,
    pub address: SocketAddrV4,
    pub power: bool,
    pub bright: u8,
    pub rgb: u32,
}

pub fn parse(raw_string: &str) -> Option<WifiBulb> {
    let mut id = None;
    let mut address = None;
    let mut power = None;
    let mut bright = None;
    let mut rgb = None;
    if !raw_string.trim().starts_with("HTTP") {
        return None;
    }

    let lines = raw_string.split("\n");
    for line in lines {
        if line.starts_with("HTTP") {
            continue;
        }
        let splitted: Vec<&str> = line.splitn(2, ":").collect();
        if splitted.len() != 2 {
            continue;
        }
        let name = splitted[0].trim().to_lowercase();
        let value = splitted[1].trim();
        match name.as_str() {
            "id" => {
                let without_prefix = value.trim_start_matches("0x");
                if let Ok(result) = i64::from_str_radix(without_prefix, 16) {
                    id = Some(result);
                }
            }
            "location" => {
                let address_string = value.replace("yeelight://", "");
                if let Ok(result) = address_string.parse() {
                    address = Some(result);
                }
            }
            "power" => power = Some(value == "on"),
            "bright" => {
                if let Ok(result) = value.parse() {
                    bright = Some(result);
                }
            }
            "rgb" => {
                if let Ok(result) = value.parse() {
                    rgb = Some(result);
                }
            }
            _ => (),
        }
    }
    if id.is_some() && address.is_some() && power.is_some() && bright.is_some() && rgb.is_some() {
        Some(WifiBulb {
            id: id.unwrap(),
            address: address.unwrap(),
            power: power.unwrap(),
            bright: bright.unwrap(),
            rgb: rgb.unwrap(),
        })
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_wifi_bulb_discovery_response() {
        let raw_response = "
        HTTP/1.1 200 OK\r\n
        Cache-Control: max-age=3600\r\n
        Date: \r\n
        Ext: \r\n
        Location: yeelight://192.168.1.56:55443\r\n
        Server: POSIX UPnP/1.0 YGLC/1\r\n
        id: 0x0000000007fb008f\r\n
        model: color\r\n
        fw_ver: 35\r\n
        support: get_prop set_default set_power toggle set_bright start_cf stop_cf set_scene cron_add cron_get cron_del set_ct_abx set_rgb set_hsv set_adjust adjust_bright adjust_ct adjust_color set_music set_name\r\n
        power: off\r\n
        bright: 99\r\n
        color_mode: 2\r\n
        ct: 3569\r\n
        rgb: 16444375\r\n
        hue: 34\r\n
        sat: 14\r\n
        name: \r\n
        ";
        assert_eq!(
            parse(raw_response),
            Some(WifiBulb {
                id: 133890191,
                address: SocketAddrV4::new(Ipv4Addr::new(192, 168, 1, 56), 55443),
                power: false,
                bright: 99u8,
                rgb: 16444375,
            })
        );
    }
}
