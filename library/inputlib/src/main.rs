pub mod input {
    pub fn to_ctrl_byte(value : char) -> u8 {
        let byte = value as u8;
        byte & 0b0001_1111
    }
    pub fn die(error : std::io::Error) {
        panic!("{}",error);
    }
}
