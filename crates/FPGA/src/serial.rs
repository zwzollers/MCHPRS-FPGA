pub fn read_serial () -> Option<u8>
{
    let mut res: Option<u8> = Option::None;
    let port_name: String = "COM4".to_string();
    let baud_rate: u32 = 2500000;

    let mut buf: Vec<u8> = vec![0; 1];
    
    let mut port = serialport::new(&port_name, baud_rate)
        .parity(serialport::Parity::None)
        .data_bits(serialport::DataBits::Eight)
        .stop_bits(serialport::StopBits::One)
        .open()
        .unwrap();
    
    loop 
    {
        match port.read(&mut buf)
        {
            Result::Ok(_) => 
            {
                //println!("{:08b}", buf[0]);
                res = Option::Some(buf[0]);
                buf[0] = 0;
            }
            Result::Err(_) => {return res}
        } 
    }
}