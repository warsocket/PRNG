use std::io::{self, Write};
// use std::mem::size_of;

const BITS: [u64;7] = [
0xFFFFFFFFFFFFFFFF,
0xAAAAAAAAAAAAAAAA,
0xCCCCCCCCCCCCCCCC,
0xF0F0F0F0F0F0F0F0,
0xFF00FF00FF00FF00,
0xFFFF0000FFFF0000,
0xFFFFFFFF00000000
]; //0xFFFFFFFFFFFFFFFF0000000000000000] //we just basiclly shift right the last one AN TAKE LOWEEST 64 BITS

const LARGEST64PRIME:u64 = 18446744073709551557;

fn main() {
    // let mut state: u64 = 1;

    // let mut stdout = io::stdout().lock();
    // for _ in 0u64..0xFFFFFFFu64{
    //     state = next_number(state);
    //     // println!("{}", state);
    //     stdout.write(&state.to_be_bytes());
    // }

    let mut stdout = io::stdout().lock();


    // for i in 0..255{
    //     // state.debug();
    //     let mut state:State = Default::default();//State{ seeds: Seeds{ byte: [0 as u8; 64] } };
    //     state.init( &[i as u8; 1] );
    //     // println!("{}", state.output());
    //     stdout.write(&state.output().to_be_bytes());
    // }    

    let mut state:State = Default::default();
    state.init( &[0 as u8; 1] );
    stdout.write(&state.output().to_be_bytes());

}

fn rotr_64(num:u64, shift:u8) -> u64{
    let s = shift % 64;
    let falloff_mask:u64 = (2 as u64).pow(s as u32)-1;
    let falloff:u64 = (( ((num & falloff_mask) as u128) << (64-shift) )) as u64;
    return falloff | (num >> shift)
}

fn rot(n64:u64, ret64:u64) -> (u64,u64){
    return (n64 >> 6, rotr_64(ret64, (n64 & 0x3f) as u8 ))
}
    
fn xor(n64:u64, ret64:u64, interval:usize) -> (u64, u64){
    let num:u8 = (n64 & (2 as u64).pow(interval as u32)-1) as u8;
    return (n64 >> interval, ret64 ^ rotr_64(BITS[interval], num))
}


fn next_number(num:u64) -> u64{
    let mut n64:u64;
    let mut ret64:u64;

    n64=num;
    ret64 = n64;

    ret64 ^= 0xFFFFFFFFFFFFFFFF;

    for interval in 1..7{
        (n64, ret64) = xor(n64, ret64, interval);
        (n64, ret64) = rot(n64, ret64);
    }

    ret64 ^= (0xFFFFFFFFFFFFFFFF0000000000000000 as u128 >> (n64 & 0x3f))  as u64;
    ret64 = ((ret64 as u128*LARGEST64PRIME as u128) & 0xFFFFFFFFFFFFFFFF) as u64;

    return ret64
}





union Seeds{
    byte:[u8;64],
    qword:[u64;8],
}

impl Default for Seeds{
    fn default()-> Self{
        Self{byte:[0;64]}
    }
}


#[derive(Default)]
struct State{
    seeds: [u64;8]
}


impl State{

    // fn new() -> Self {
    //     Default::default()
    // }

    fn init(&mut self, key:&[u8]){
        //copy as bytes
        let mut seeds:Seeds = Default::default();
        for ptr in 0..(std::mem::size_of::<Seeds>()){
            // println!("{} {}", ptr, ptr % key.len())
            unsafe{ seeds.byte[ptr] = key[ptr%key.len()] }
        }

        //write back as u86 arrays
        for i in 0..(self.seeds.len()){
            self.seeds[i] = unsafe{seeds.qword[i]};
        }
        self.step(); //initial shuffle prevent multiple smae keys producing same value
        self.step(); //shuffle so 1st answer is ready
        self.step(); //last shuffle to break last symmetries when enumerating repeating keys

    }

    fn step(&mut self){
        for i in 0..(self.seeds.len()){
            self.seeds[i] = next_number( rotr_64(self.seeds[i], (i<<3) as u8) );
        }            
    }

    fn output(&self) -> u64{
        let mut acc:u64 = 0;

        for i in 0..(self.seeds.len()){
            acc ^= self.seeds[i];
        }

        return acc
    }


    fn debug(&self) -> u64{
        let mut acc:u64 = 0;

        for i in 0..(self.seeds.len()){
            println!("{} {}", i, self.seeds[i]);
        }

        return acc
    }


}

