use std::io::{self, Write};


const LARGEST64PRIME:u64 = 18446744073709551557;

fn main() {
    let mut stdout = io::stdout().lock();

    let mut state:State = State::new(0);

    // for i in 0u64..0x10u64{
    //     println!("{}", state.next());
    // }


    for item in state.take(0xFFFFFFF){
        // println!("{}", item);
        stdout.write(&item.to_be_bytes());

    }

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


union Seeds{
    byte:[u8;64],
    qword:[u64;8],
}

impl Default for Seeds{
    fn default()-> Self{
        Self{byte:[0;64]}
    }
}


// #[derive(Default)]
struct State{
    data:u64,
    bits:[u64;7],
    bits7:u128,
}


// struct CipherState{
//     states:[State;8],
// }


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

        obj.next_number();
        obj.next_number();
        obj.next_number();
        obj.next_number();

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
        self.data =  self.data ^ self.bits[interval];
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




// impl State{

//     fn init(&mut self, key:&[u8]){
//         //copy as bytes
//         let mut seeds:Seeds = Default::default();
//         for ptr in 0..(std::mem::size_of::<Seeds>()){
//             // println!("{} {}", ptr, ptr % key.len())
//             unsafe{ seeds.byte[ptr] = key[ptr%key.len()] }
//         }

//         //write back as u86 arrays
//         for i in 0..(self.seeds.len()){
//             self.seeds[i] = unsafe{seeds.qword[i]};
//         }
//         self.step(); //initial shuffle prevent multiple smae keys producing same value
//         self.step(); //shuffle so 1st answer is ready
//         self.step(); //last shuffle to break last symmetries when enumerating repeating keys

//         self.step();
//         self.step();
//         self.step();
//         self.step();

//         self.step();
//         self.step();
//         self.step();
//         self.step();

//         self.step();
//         self.step();
//         self.step();
//         self.step();

//         self.step();
//         self.step();
//         self.step();
//         self.step();                        

//     }

//     fn new(key:&[u8]) -> Self {
//         let mut s:Self = Default::default();
//         s.init(key);

//         return s
//     }

    // fn step(&mut self){
    //     for i in 0..(self.seeds.len()){
    //         self.seeds[i] = next_number( rotr_64(self.seeds[i], (i<<3) as u8) );
    //     }            
    // }

//     fn output(&self) -> u64{
//         let mut acc:u64 = 0;

//         for i in 0..(self.seeds.len()){
//             acc ^= self.seeds[i];
//         }

//         return acc
//     }


//     fn debug(&self) -> u64{
//         let mut acc:u64 = 0;

//         for i in 0..(self.seeds.len()){
//             println!("{} {}", i, self.seeds[i]);
//         }

//         return acc
//     }


// }

