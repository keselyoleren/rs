enum TelemetryProtocol {
    ModbusTcp { ip: String, port: u16 }, 
    Mqtt(String),                        
    OpcUa,                               
    Bacnet { device_id: u32 },           
}

fn connect_to_device(protocol: TelemetryProtocol) {
    match protocol {
        TelemetryProtocol::ModbusTcp { ip, port } => {
            println!("Menghubungkan ke Modbus TCP di {}:{}", ip, port);
        }
        TelemetryProtocol::Mqtt(topic) => {
            println!("Subscribe ke broker MQTT pada topic: {}", topic);
        }
        TelemetryProtocol::OpcUa => {
            println!("Inisialisasi koneksi standar OPC UA...");
        }
        TelemetryProtocol::Bacnet { device_id } => {
            println!("Mencari BACnet device dengan ID: {}", device_id);
        }
    }
}

fn main() {
    let sensor_suhu = TelemetryProtocol::ModbusTcp {
        ip: String::from("192.168.1.50"),
        port: 502,
    };
    let sensor_getaran = TelemetryProtocol::Mqtt(String::from("pabrik/mesin1/getaran"));
    let gateway_utama = TelemetryProtocol::OpcUa;
    let hvac_controller = TelemetryProtocol::Bacnet { device_id: 4194303 };

    connect_to_device(sensor_suhu);
    connect_to_device(sensor_getaran);
    connect_to_device(gateway_utama);
    connect_to_device(hvac_controller);
}