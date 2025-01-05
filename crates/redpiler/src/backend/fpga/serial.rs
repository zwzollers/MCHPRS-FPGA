use serialport::SerialPort;
use std::time::{Instant, Duration};

pub enum SerialCommands
{
    update_lever = 0x2,
    request_outputs = 0x1,
}

#[derive(Default)]
pub struct SerialConnection
{
    pub port_name: String,
    pub baud_rate: u32,
    pub conn: Option<Box<dyn SerialPort>>
}



impl SerialConnection
{
    pub fn start_conn (&mut self)
    {
        self.conn = Some(serialport::new(&self.port_name, self.baud_rate)
            .timeout(Duration::from_millis(50))
            .parity(serialport::Parity::None)
            .data_bits(serialport::DataBits::Eight)
            .stop_bits(serialport::StopBits::One)
            .open()
            .unwrap());
    }

    pub fn read_serial (&mut self, len:usize) -> Option<Vec<u8>>
    {
        self.transmit(SerialCommands::request_outputs as u8);

        let port_name: String = "COM4".to_string();
        let baud_rate: u32 = 2500000;

        let mut buf: Vec<u8> = vec![0; (len+7)/8];
        let time = Instant::now();
        let mut conn = self.conn.as_mut().unwrap();
        match conn.read_exact(&mut buf)
        {
            Result::Ok(_) => 
            {
                conn.clear(serialport::ClearBuffer::Input);
                return Some(buf);
            }
            Result::Err(e) => 
            {
                conn.clear(serialport::ClearBuffer::Input);
                return None
            }
        }
        
    }

    pub fn transmit (&mut self, data: u8) 
    {
        self.conn.as_mut().unwrap().write(&vec![data]);
    }
}

