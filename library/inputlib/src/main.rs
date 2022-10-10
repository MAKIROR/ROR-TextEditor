pub mod Input {
    pub mod RawMode {
        pub fn ToCtrlByte(value : char) -> u8 {
            let byte = value as u8;
            byte & 0b0001_1111
        }
    }
}
