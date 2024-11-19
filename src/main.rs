use std::io::{self, Write};
use std::ops::{Index, IndexMut};

const LARGEST64PRIME:u64 = 18446744073709551557;

fn main() {
    let mut stdout = io::stdout().lock();

    let mut state = State::new(0x80876363d18c506d);

    // for _ in 0..2{
    loop{
        stdout.write(&state.next().to_be_bytes());
    }
    // }

}

fn rotr_64(num:u64, shift:u8) -> u64{
    let s = shift % 64;
    let falloff_mask:u64 = (2 as u64).pow(s as u32)-1;
    let falloff:u64 = (num & falloff_mask) << (64-s)%64; //why, because even rust is weird sometimes shifting 0's 63 spaces is fine, wierd
    return falloff | (num >> s)
}

fn rotr_128(num:u128, shift:u8) -> u128{
    let s = shift % 128;
    let falloff_mask:u128 = (2 as u128).pow(s as u32)-1;
    let falloff:u128 = (num & falloff_mask) << (128-s)%128; //same weirdness as above
    return falloff | (num >> s)
}


//////////////////////////////////////////////////////////////////////////////////


struct State{
    data:u64,
    bits:[u64;7],
    bits7:u128,
}


impl Iterator for State{
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item>{
        return Some(self.next())
    }
}


impl State{

    fn new(data:u64) -> Self{

        let mut obj:Self = Self{
            data: data,
            bits: [
                0xFFFFFFFFFFFFFFFF,
                0xAAAAAAAAAAAAAAAA,
                0xCCCCCCCCCCCCCCCC,
                0xF0F0F0F0F0F0F0F0,
                0xFF00FF00FF00FF00,
                0xFFFF0000FFFF0000,
                0xFFFFFFFF00000000,
            ],
            bits7: 0xFFFFFFFFFFFFFFFF0000000000000000,
        };

        return obj

    }

    fn next(&mut self) -> u64{
        self.step();
        return self.output();
    }

    fn step(&mut self){
        self.data = self.next_number();
    }

    fn output(&self) -> u64{
        return self.data;
    }

    fn rot(self:&mut State, n64:u64) -> u64{
        self.data = rotr_64(self.data, (n64 & 0x3f) as u8 );
        return n64 >> 6
    }
        
    fn xor_and_rot(self:&mut State, n64:u64, interval:usize) -> u64{
        let num:u8 = (n64 & (2 as u64).pow(interval as u32)-1) as u8;
        self.bits[interval] = rotr_64(self.bits[interval], num);
        self.data ^= self.bits[interval];
        return n64 >> interval
    }

    fn next_number(self:&mut State) -> u64{
        let mut n64:u64;

        n64=self.data;

        self.data ^= 0xFFFFFFFFFFFFFFFF;

        for interval in 1..7{
            n64 = self.xor_and_rot(n64, interval);
            n64 = self.rot(n64);
        }

        self.bits7 = rotr_128(self.bits7, (n64 & 0x3f) as u8);
        self.data ^= self.bits7 as u64;

        self.data = ((self.data as u128*LARGEST64PRIME as u128) & 0xFFFFFFFFFFFFFFFF) as u64;

        return self.output()
    }

}