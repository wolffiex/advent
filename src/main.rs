#![allow(dead_code, unused_variables)]


fn main() {
    part1().unwrap();
}

#[derive(Debug, Default)]
struct BitStream {
    char_vec: Vec<char>,
    p: usize,
    m: u32,
}

impl BitStream {
    fn wrap(s: &str) -> BitStream {
        BitStream {
            char_vec: s.chars().collect(),
            p: 0,
            m: 0b1000,
        }
    }
    fn read(&mut self, num_bytes: u32) -> u32 {
        let mut r = 0;
        if num_bytes > 32 { panic!("Oops"); }
        for i in 0..num_bytes {
            r <<= 1;
            if self.m == 0 {
                self.p = self.p + 1;
                self.m = 0b1000
            }
            let c = self.char_vec.get(self.p).unwrap();
            let nibble = c.to_digit(16).unwrap();
            //println!("-- {} {:b} {:b}", c, nibble, self.m);
            if nibble & self.m > 0 { r |= 1; }
            self.m >>= 1;
        }
        return r;
    }
}

fn read_int(stream: &mut BitStream) -> (u32, u32) {
    let mut x = 0;
    let mut byte_count = 0;
    loop {
        x <<= 4;
        let keep_going = stream.read(1) > 0;
        x = x | stream.read(4);
        byte_count = byte_count + 5;
        if !keep_going { break; }
    }
    return (x, byte_count);
}

fn part1() -> Option<()> {
    //let mut stream = BitStream::wrap("D2FE28");
    let mut stream = BitStream::wrap("38006F45291200");
    let ((v, t), num_bytes) = read_packet(&mut stream);

    println!("numbytes: {}", num_bytes);
    println!("V: {:0>3b}", v);
    println!("T: {:0>3b}", t);
    Some(())
}

fn read_packet(stream: &mut BitStream) -> ((u32, u32), u32) {
    let header = (stream.read(3), stream.read(3));
    println!("read V:{:0>3b} T:{:0>3b}", header.0, header.1);
    let mut total_bytes_read = 6;
    if header.1 == 4 {
        let (x, bytes_read) = read_int(stream);
        total_bytes_read = total_bytes_read + bytes_read;
    } else {
        /*
        an operator packet can use one of two modes indicated by the bit immediately
        after the packet header; this is called the length type ID:

        If the length type ID is 0, then the next 15 bits are a number that represents
        the total length in bits of the sub-packets contained by this packet. If the
        length type ID is 1, then the next 11 bits are a number that represents the
        number of sub-packets immediately contained by this packet.[
        */

        let length_type_id = stream.read(1);
        let sub_packet_size_bit_count: u32 = if length_type_id == 0 { 15 } else { 11 };
        let sub_packet_bit_count = stream.read(sub_packet_size_bit_count);
        total_bytes_read = 1 + sub_packet_size_bit_count;
        println!("expect bits: {}", sub_packet_bit_count);
        let mut sub_bytes_read = 0;
        loop {
            let (header, bytes_read) = read_packet(stream);
            sub_bytes_read = sub_bytes_read + bytes_read;
            println!("sub read: {}", sub_bytes_read);
            if sub_bytes_read == sub_packet_bit_count { break; }
            if sub_bytes_read > sub_packet_bit_count {
                panic!("Read too many bytes");
            }
        }
        total_bytes_read += sub_packet_bit_count;
    }
    return (header, total_bytes_read);
}

fn get_input() -> &'static str {
    return "D2FE28";
}