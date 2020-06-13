use core::time::Duration;
use lazy_static::lazy_static;
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
use std::sync::PoisonError;

#[get("/?<refresh>")]
pub fn get_all(refresh: Option<bool>) -> Result<Json<Vec<WifiBulb>>, Error> {
    let force_refresh = match refresh {
        Some(some_refresh) => some_refresh,
        None => false,
    };
    if force_refresh {
        let wifi_bulbs = discover();
        let result = wifi_bulbs?;
        let cloned = result.clone();
        let mut vec_arc = LIGHTS_STORAGE
            .lock()
            .unwrap_or_else(PoisonError::into_inner);
        *vec_arc = Arc::new(result);
        Ok(Json(cloned))
    } else {
        let vec_arc = LIGHTS_STORAGE
            .lock()
            .unwrap_or_else(PoisonError::into_inner);
        Ok(Json((*vec_arc).to_vec()))
    }
}

#[get("/<id>")]
pub fn get_one(id: i64) -> Result<Option<Json<WifiBulb>>, Error> {
    let vec_arc = LIGHTS_STORAGE
        .lock()
        .unwrap_or_else(PoisonError::into_inner);
    let candidates = (*vec_arc)
        .iter()
        .filter(|bulb| bulb.id == id)
        .collect::<Vec<&WifiBulb>>();
    if candidates.len() > 0 {
        Ok(Some(Json(candidates[0].clone())))
    } else {
        Ok(None)
    }
}

#[get("/<id>/toggle")]
pub fn toggle(id: i64) -> Result<Option<()>, Error> {
    let vec_arc = LIGHTS_STORAGE
        .lock()
        .unwrap_or_else(PoisonError::into_inner);
    let candidates = (*vec_arc)
        .iter()
        .filter(|bulb| bulb.id == id)
        .collect::<Vec<&WifiBulb>>();
    if candidates.len() > 0 {
        perform_action(&candidates[0].address, "toggle")?;
        Ok(Some(()))
    } else {
        Ok(None)
    }
}

lazy_static! {
    static ref LIGHTS_STORAGE: WifiBulbs = Arc::new(Mutex::new(Arc::new(vec![])));
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

                let string_buffer_result = std::str::from_utf8(filled_buf);
                match string_buffer_result {
                    Ok(string_buffer) => {
                        let parsed_option = parse(string_buffer);
                        for parsed in parsed_option.iter() {
                            if device_ids.contains(&parsed.id) {
                                continue;
                            }
                            device_ids.insert(parsed.id.clone());
                            devices.push(parsed.clone());
                        }
                    }
                    Err(error) => {
                        println!("Exiting discovery loop {}", error);
                        break;
                    }
                }
            }
            Err(error) => {
                println!("Exiting discovery loop {}", error);
                break;
            }
        }
    }
    Ok(devices)
}

fn perform_action(address: &SocketAddrV4, method: &str) -> Result<(), Error> {
    let mut stream = TcpStream::connect(address)?;
    let msg = format!(
        "{{\"id\":{},\"method\":\"{}\",\"params\":[{}]}}\r\n",
        1, method, ""
    );
    stream.write(msg.as_bytes())?;
    Ok(())
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
    let mut id_option = None;
    let mut address_option = None;
    let mut power_option = None;
    let mut bright_option = None;
    let mut rgb_option = None;
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
                    id_option = Some(result);
                }
            }
            "location" => {
                let address_string = value.replace("yeelight://", "");
                if let Ok(result) = address_string.parse() {
                    address_option = Some(result);
                }
            }
            "power" => power_option = Some(value == "on"),
            "bright" => {
                if let Ok(result) = value.parse() {
                    bright_option = Some(result);
                }
            }
            "rgb" => {
                if let Ok(result) = value.parse() {
                    rgb_option = Some(result);
                }
            }
            _ => (),
        }
    }
    if let (Some(id), Some(address), Some(power), Some(bright), Some(rgb)) = (
        id_option,
        address_option,
        power_option,
        bright_option,
        rgb_option,
    ) {
        Some(WifiBulb {
            id: id,
            address: address,
            power: power,
            bright: bright,
            rgb: rgb,
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
