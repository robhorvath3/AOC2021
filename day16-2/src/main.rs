use std::fs::File;
use std::io::{BufRead, BufReader, Write};

#[derive(Debug)]
struct Op {
    length_type: usize,
    length: usize,
    length_bits: usize,
    size: usize,
    sub_packets: Vec<usize>,
}

impl Op {
    fn new() -> Op {
        Op {
            length_type: usize::MAX,
            length: usize::MAX,
            length_bits: usize::MAX,
            size: usize::MAX,
            sub_packets: Vec::new(),
        }
    }
}

#[derive(Debug)]
struct Packet {
    my_idx: usize,
    version: usize,
    typ: usize,
    literal: Option<usize>,
    op: Option<Op>,
    parent: Option<usize>,
}

const BSM: [u8; 8] = [0xFF, 0x7F, 0x3F, 0x1F, 0x0F, 0x07, 0x03, 0x01];
const BEM: [u8; 8] = [0x80, 0xC0, 0xE0, 0xF0, 0xF8, 0xFC, 0xFE, 0xFF];

struct BinaryBuf {
    data: Vec<u8>
}

impl BinaryBuf {
    fn new() -> BinaryBuf {
        BinaryBuf {
            data: Vec::new(),
        }
    }

    fn push(&mut self, val: u8) {
        self.data.push(val);
    }

    fn read_int(&self, start_bit: usize, end_bit: usize) -> usize {
        let mut r: usize;                   // return value

        let si: usize = start_bit / 8;      // start index into data
        let sb: usize = start_bit % 8;      // start bit in start index
        let ei: usize = end_bit / 8;        // end index into data
        let eb: usize = end_bit % 8;        // end bit in end index

        let mut ci: usize = ei;             // current index
        let mut cb: usize = eb;             // current bit

        let mut bc: usize;                  // bit count

        if si == ei {
            //println!("self.data[{}] == {}, cb: {}, sb: {}, eb: {}", cb, ci, self.data[ci], sb, eb);
            r = ((self.data[ci] & (BSM[sb] & BEM[eb])) >> (7 - eb)) as usize;
        }
        else {
            r = ((self.data[ci] & BEM[cb]) >> (7 - cb)) as usize;        

            bc = cb + 1;

            loop {
                // go to the index before
                ci -= 1;

                if ci == si {
                    cb = sb;
                    r += ((self.data[ci] & BSM[cb]) as usize) << bc;
                    break;
                }
                else {
                    cb = 7; // all bits
                    r += ((self.data[ci] & BEM[cb]) as usize) << bc;
                    bc += 8;
                }
            }
        }
        r
    }
}

fn main() {
    let f = File::open("./input.txt").expect("Unable to open input file");
    let f = BufReader::new(f);

    let mut in_bin: BinaryBuf = BinaryBuf::new();
    let mut packets: Vec<Packet> = Vec::new();

    for line in f.lines() {
        let line = line.expect("Unable to read line");
        let in_chars: Vec<char> = line.chars().collect();

        {
            let mut bin_output = File::create("hex.txt").expect("Unable to open output file");
            for i in (0..in_chars.len()).step_by(2) {
                let bh: u8 = u8::from_str_radix(&format!("{}", in_chars[i]) as &str, 16).unwrap();
                let bl: u8 = u8::from_str_radix(&format!("{}", in_chars[i+1]) as &str, 16).unwrap();
                let b: u8 = (bh << 4) + bl;
                in_bin.push(b);
                let _ = bin_output.write_all(&[b]);
            }
        }
    }

    let mut cur_idx: usize = 0;

    loop {
        let _p = read_packet(&mut in_bin, &mut cur_idx, &mut packets, usize::MAX);
        
        if (in_bin.data.len() * 8) - cur_idx < 11  {
            println!("progress - cur_idx: {}, in_bin.data.len() in bits: {}", cur_idx, in_bin.data.len() * 8);
            break;
        }
    }

    fn read_packet(in_bin: &mut BinaryBuf, cur_idx: &mut usize, packets: &mut Vec<Packet>, parent_idx: usize) -> (usize, usize, usize) {
        let packet_start = *cur_idx;
        let packet_version = in_bin.read_int(packet_start, packet_start + 2);
        let packet_type = in_bin.read_int(packet_start + 3, packet_start + 5);
        
         let my_idx: usize;
        
        let mut sub_bits_read: usize = 0;
        let mut sub_packets_read: usize = 0;

        //print!("Version: {}, Type: {}, Packet Start Bit: {} - ", packet_version, packet_type, packet_start);
        if packet_type == 4 {
            let mut literal_val: usize = 0;
            let mut flag_bit: usize = in_bin.read_int(packet_start + 6, packet_start + 6);
            let mut chunk_index: usize = packet_start + 7;

            loop {
                //print!("Computing Literal: {}, flag_bit: {}", literal_val, flag_bit);
                literal_val = literal_val << 4;
                literal_val |= in_bin.read_int(chunk_index, chunk_index + 3);
                //println!(", New Literal: {}", literal_val);

                if flag_bit == 1 {
                    //println!("New Literal Chunk: {} -> {} @ {}", literal_val, in_bin.read_int(chunk_index, chunk_index + 3), chunk_index);
                    chunk_index += 5;
                    flag_bit = in_bin.read_int(chunk_index-1, chunk_index-1)
                }
                else {
                    *cur_idx = chunk_index + 4;
                    //println!("Last Literal Chunk: cur_idx: {}", cur_idx);
                    break;
                }
            }

            my_idx = packets.len();
            packets.push(
                Packet { 
                    my_idx: my_idx,
                    version: packet_version,
                    typ: packet_type, 
                    parent: {
                        if parent_idx != usize::MAX {
                            Some(parent_idx)
                        }
                        else {
                            None
                        }
                    },
                    literal: Some(literal_val),
                    op: None,
            });        
        }
        else {
            let length_type_id = in_bin.read_int(packet_start + 6, packet_start + 6);
            let total_length_bits: usize = match length_type_id {
                0 => 15,
                1 => 11,
                _ => 0,
            };
            let total_length: usize = in_bin.read_int(packet_start + 7, packet_start + 7 + total_length_bits - 1);
            
            my_idx = packets.len();
            packets.push(
                Packet { 
                    my_idx: my_idx,
                    version: packet_version, 
                    typ: packet_type,
                    parent: {
                        if parent_idx != usize::MAX {
                            Some(parent_idx)
                        }
                        else {
                            None
                        }
                    },
                    literal: None,
                    op: {
                        Some(Op { 
                            length_type: length_type_id,
                            length: total_length,
                            length_bits: total_length_bits,
                            size: 0,
                            sub_packets: Vec::new(),
                        })
                    },
            });
            
            *cur_idx = packet_start + 7 + total_length_bits;
            
            match length_type_id {
                0 => {
                    while sub_bits_read < total_length {
                        let pr = read_packet(in_bin, cur_idx, packets, my_idx);
                    
                        packets[my_idx].op.as_mut().unwrap().sub_packets.push(pr.0);
                        packets[my_idx].op.as_mut().unwrap().size += 1;

                        sub_bits_read += pr.1;
                    }
                },
                1 => {
                    for _i in 0..total_length {
                        let pr = read_packet(in_bin, cur_idx, packets, my_idx);
                
                        packets[my_idx].op.as_mut().unwrap().sub_packets.push(pr.0);
                        packets[my_idx].op.as_mut().unwrap().size += 1;

                        sub_packets_read += 1;
                    }
                },
                _ => {},
            }        
        }

        (my_idx, *cur_idx - packet_start, sub_packets_read)
    }

    // Decode & Execute - day2
    fn exec(packets: &mut Vec<Packet>, idx: usize) -> usize {
        let mut r: usize = 0;

        //println!("Processing Packet #{}", idx);

        if packets[idx].typ == 4 {
            //println!("Returning literal");
            return packets[idx].literal.unwrap();
        }

        let sp = packets[idx].op.as_ref().unwrap().sub_packets.clone();
        //println!("Processing {} sub packets", sp.len());

        match packets[idx].typ {
            0 => {  // SUM
                for i in 0..sp.len() {
                    r += exec(packets, sp[i]);
                }
            },
            1 => {  // PRODUCT
                r = 1;
                for i in 0..sp.len() {
                    r *= exec(packets, sp[i]);
                }
            },
            2 => {  // MINIMUM
                let mut min: usize = usize::MAX;
                for i in 0..sp.len() {
                    r = exec(packets, sp[i]);
                    min = usize::min(min, r);
                }
                r = min;
            },
            3 => {  // MAXIMUM
                let mut max: usize = 0;
                for i in 0..sp.len() {
                    r = exec(packets, sp[i]);
                    max = usize::max(max, r);
                }
                r = max;
            },
            5 => {  // GREATER THAN
                let sp1 = exec(packets, sp[0]);
                let sp2 = exec(packets, sp[1]);

                if sp1 > sp2 {
                    r = 1;
                }
                else {
                    r = 0;
                }
            },
            6 => {  // LESS THAN
                let sp1 = exec(packets, sp[0]);
                let sp2 = exec(packets, sp[1]);

                if sp1 < sp2 {
                    r = 1;
                }
                else {
                    r = 0;
                }
            },
            7 => {  // EQUAL TO
                let sp1 = exec(packets, sp[0]);
                let sp2 = exec(packets, sp[1]);

                if sp1 == sp2 {
                    r = 1;
                }
                else {
                    r = 0;
                }
            },
            _ => {},
        }
        
        //println!("Exiting {}", idx);
        r
    }

    //println!("{:?}", packets);

    println!("Result of packet execution ({} packets): {}", packets.len(), exec(&mut packets, 0));
}
