use std::error::Error;

use crate::sim_freq::{RadioDevices, RadioOptions, SimFreq};
use crate::device_select::{convert_from_device, convert_to_device};

use custom_can_protocol::{Packet, PacketHandler};

pub struct FreqHandler {
    sim_frequency_connection: SimFreq,
}

fn round_to_nearest_power_of_5(n: u32, power: u32) -> u32 {
    let multiple = 5u32.pow(power);
    ((n + multiple / 2) / multiple) * multiple
}

impl FreqHandler {
    pub fn new() -> Self {
        
        let sim_frequency_connection = SimFreq::new()
            .expect("Failed to connect to simulator try running the program again.");
        
        FreqHandler {
            sim_frequency_connection: sim_frequency_connection
        }
    }

    /// Check for frequency updates from the simulator
    /// 
    /// # Returns
    /// a vector of packets to send with the updated frequency data
    pub fn check_for_freq_updates(&mut self) -> Option<Vec<Packet>> {
        let mut packets: Vec<Packet> = Vec::new();

        if let Some(data) = self.sim_frequency_connection.get_freq_update() {
            if data.radio_type == RadioDevices::XPDR {
                return None;
            } else {
                let mut payload:Vec<u8> = Vec::new();
                let active_freq = round_to_nearest_power_of_5((*data.frequencies.get(&RadioOptions::ACTIVE).unwrap() * 1e3) as u32, 1);
                let standby_freq = round_to_nearest_power_of_5((*data.frequencies.get(&RadioOptions::STANDBY).unwrap() * 1e3) as u32, 1);
                payload.push(convert_from_device(data.radio_type));
                payload.push((standby_freq >> 24) as u8);
                payload.push((standby_freq >> 16) as u8);
                payload.push((standby_freq >> 8) as u8);
                payload.push((standby_freq >> 0) as u8);
                payload.push((active_freq >> 24) as u8);
                payload.push((active_freq >> 16) as u8);
                payload.push((active_freq >> 8) as u8);
                payload.push((active_freq >> 0) as u8);
                packets.push(Packet::new(1, payload));

                return Some(packets);
            }
        }

        None
    }
}

fn u32_to_bcd16(value: u32) -> u32 {
    let mut bcd: u32 = 0;
    let mut shift = 0;
    let mut num = value;

    while num > 0 {
        let digit = num % 10;
        bcd |= (digit as u32) << shift;
        num /= 10;
        shift += 4;
    }

    bcd
}

impl PacketHandler for FreqHandler {
    fn handle_packet(&mut self, packet: &Packet) -> Result<(), Box<dyn Error>> {
        let radio_type = convert_to_device(packet.payload[0]);

        let mut standby_freq: u32 = ((packet.payload[1] as u32) << 24)
                                + ((packet.payload[2] as u32) << 16)
                                + ((packet.payload[3] as u32) << 8)
                                + ((packet.payload[4] as u32) << 0);
        
        let mut active_freq: u32 = ((packet.payload[5] as u32) << 24)
                                + ((packet.payload[6] as u32) << 16)
                                + ((packet.payload[7] as u32) << 8)
                                + ((packet.payload[8] as u32) << 0);

        if radio_type == RadioDevices::XPDR {
            let xpdr_value = active_freq;
            println!("XPDR value set {}", xpdr_value);
            let xpdr_value = u32_to_bcd16(active_freq);
            self.sim_frequency_connection.set(RadioDevices::XPDR, RadioOptions::CODE, xpdr_value)?;
            return Ok(());
        } else {
            active_freq *= 1000;
            standby_freq *= 1000;
            println!("Set frequency Active: {}, Standby: {} to device {:?}", active_freq/1000, standby_freq/1000, radio_type);
        }
            

        self.sim_frequency_connection.set(radio_type, RadioOptions::ACTIVE, active_freq)?;
        self.sim_frequency_connection.set(radio_type, RadioOptions::STANDBY, standby_freq)?;

        Ok(())
    }

    fn get_packet_id(&self) -> u8 {
        1
    }
}