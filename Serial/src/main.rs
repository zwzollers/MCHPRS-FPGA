use serialport;

fn main() {
    let port_name: String = "COM4".to_string();
    let baud_rate: u32 = 9600;
    
    let mut port = serialport::new(&port_name, baud_rate)
        .open()
        .unwrap();
    
    loop 
    {
        let res = port.read()
    }
    
}
