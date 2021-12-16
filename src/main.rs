#![allow(dead_code, unused_variables)]


fn main() {
    println!("Expect 3: {}",part2("C200B40A82").unwrap());
    println!("Expect 54: {}",part2("04005AC33890").unwrap());
    println!("Expect 7: {}",part2("880086C3E88112").unwrap());
    println!("Expect 9: {}",part2("CE00C43D881120").unwrap());
    //println!("Expect 1: {}",part2("D8005AC2A8F0").unwrap());
    //println!("Expect 0: {}",part2("F600BC2D8F").unwrap());
}

fn part2(input:&str) -> Option<u32> {
    //let mut stream = BitStream::wrap("D2FE28");
    let mut stream = BitStream::wrap(input);
    let (v, num_bytes) = read_packet(&mut stream);

    //println!("numbytes: {}", num_bytes);
    //println!("v: {}", v);
    Some(v)
}

fn read_packet(stream: &mut BitStream) -> (u32, u32) {
    let (ver, type_id) = (stream.read(3), stream.read(3));
    let mut total_bytes_read = 6;
    return if type_id == 4 {
        let (x, bytes_read) = read_int(stream);
        total_bytes_read = total_bytes_read + bytes_read;
        (x, total_bytes_read)
    } else {
        let mut read_subpackets = || -> Vec<u32> {
            let length_type_id = stream.read(1);
            total_bytes_read = total_bytes_read + 1;
            let mut subpackets:Vec<u32> = Vec::new();
            if length_type_id == 0 {
                let sub_packet_bit_count = stream.read(15);
                total_bytes_read += 15;
                //println!("expect bits: {}", sub_packet_bit_count);
                let mut sub_bytes_read = 0;
                loop {
                    let (subvalue, bytes_read) = read_packet(stream);
                    subpackets.push(subvalue);
                    sub_bytes_read = sub_bytes_read + bytes_read;
                    //println!("sub read: {}", sub_bytes_read);
                    if sub_bytes_read == sub_packet_bit_count { break; }
                    if sub_bytes_read > sub_packet_bit_count {
                        panic!("Read too many bytes");
                    }
                }
                total_bytes_read += sub_packet_bit_count;
            } else {
                let mut sub_packet_count = stream.read(11);
                total_bytes_read = total_bytes_read + 11;
                //println!("sub count: {}", sub_packet_count);

                loop {
                    let (subvalue, bytes_read) = read_packet(stream);
                    subpackets.push(subvalue);
                    total_bytes_read = total_bytes_read + bytes_read;
                    sub_packet_count = sub_packet_count - 1;
                    if sub_packet_count == 0 { break; }
                }
            };
            return subpackets;
        };
        let values = read_subpackets();
        let value = match type_id {
            t if t == 0 => values.into_iter().sum(),
            t if t == 1 => values.into_iter().product(),
            t if t == 2 => values.into_iter().min().unwrap(),
            t if t == 3 => values.into_iter().max().unwrap(),
            t if t == 4 => {
                let is_gt = t == 4;
                if values.get(0).unwrap() > values.get(1).unwrap() {
                    if is_gt {1} else {0}
                } else {
                    if is_gt {0} else {1}
                }
            },
            _ => {
                panic!("Unimplemented");
            }
        };
        (value, total_bytes_read)
    };
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


fn xget_input() -> &'static str {
    return "420D5A802122FD25C8CD7CC010B00564D0E4B76C7D5A59C8C014E007325F116C958F2C7D31EB4EDF90A9803B2EB5340924CA002761803317E2B4793006E28C2286440087C5682312D0024B9EF464DF37EFA0CD031802FA00B4B7ED2D6BD2109485E3F3791FDEB3AF0D8802A899E49370012A926A9F8193801531C84F5F573004F803571006A2C46B8280008645C8B91924AD3753002E512400CC170038400A002BCD80A445002440082021DD807C0201C510066670035C00940125D803E170030400B7003C0018660034E6F1801201042575880A5004D9372A520E735C876FD2C3008274D24CDE614A68626D94804D4929693F003531006A1A47C85000084C4586B10D802F5977E88D2DD2898D6F17A614CC0109E9CE97D02D006EC00086C648591740010C8AF14E0E180253673400AA48D15E468A2000ADCCED1A174218D6C017DCFAA4EB2C8C5FA7F21D3F9152012F6C01797FF3B4AE38C32FFE7695C719A6AB5E25080250EE7BB7FEF72E13980553CE932EB26C72A2D26372D69759CC014F005E7E9F4E9FA7D3653FCC879803E200CC678470EC0010E82B11E34080330D211C663004F00101911791179296E7F869F9C017998EF11A1BCA52989F5EA778866008D8023255DFBB7BD2A552B65A98ECFEC51D540209DFF2FF2B9C1B9FE5D6A469F81590079160094CD73D85FD2699C5C9DCF21F0700094A1AC9EDA64AE3D37D34200B7B401596D678A73AFB2D0B1B88057230A42B2BD88E7F9F0C94F1ECB7B0DD393489182F9802D3F875C00DC40010F8911C61F8002111BA1FC2E400BEA5AA0334F9359EA741892D81100B83337BD2DDB4E43B401A800021F19A09C1F1006229C3F8726009E002A12D71B96B8E49BB180273AA722468002CC7B818C01B04F77B39EFDF53973D95ADB5CD921802980199CF4ADAA7B67B3D9ACFBEC4F82D19A4F75DE78002007CD6D1A24455200A0E5C47801559BF58665D80";
}

fn get_input() -> &'static str {
    return "C200B40A82";
}