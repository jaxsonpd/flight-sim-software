use std::error::Error;

use customCANProtocol::{Packet, PacketHandler};

pub struct FreqHandler {
    set_freq: fn(u16, u16, u16, u16) -> Result<(), Box<dyn Error>>,
}

impl FreqHandler {
    pub fn new(set_freq: fn(u16, u16, u16, u16) -> Result<(), Box<dyn Error>>) -> Self {
        FreqHandler {
            set_freq
        }
    }
}

impl PacketHandler for FreqHandler {
    fn handle_packet(&mut self, packet: &Packet) -> Result<(), Box<dyn Error>> {
        
        println!("Freq packet: {:?}", packet);
        
        let standby_freq_mhz: u16 = ((packet.payload[0] as u16) << 8) | (packet.payload[1] as u16);
        let standby_freq_khz: u16 = ((packet.payload[2] as u16) << 8) | (packet.payload[3] as u16);

        let active_freq_mhz: u16 = ((packet.payload[4] as u16) << 8) | (packet.payload[5] as u16);
        let active_freq_khz: u16 = ((packet.payload[6] as u16) << 8) | (packet.payload[7] as u16);

        let _ = (self.set_freq)(active_freq_mhz, active_freq_khz, standby_freq_mhz, standby_freq_khz);

        return Ok(());
    }

    fn get_id(&self) -> u8 {
        1
    }
}