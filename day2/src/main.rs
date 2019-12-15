const WORD_SIZE: usize = 4;

enum Instruction {
    Add(usize, usize, usize),
    Mul(usize, usize, usize),
    Halt,
    Invalid,
}

impl Instruction {
    fn exec(&self, prg: &mut [usize]) {
        match self {
            Instruction::Add(a, b, out) => {
                println!("Add: {:?}", prg);
                prg[out.clone()] = prg[a.clone()] + prg[b.clone()];
                println!("Add: {:?}", prg);
            },
            Instruction::Mul(a, b, out) => {
                println!("Mul: {:?}", prg);
                prg[out.clone()] = prg[a.clone()] * prg[b.clone()];
                println!("Mul: {:?}", prg);
            },
            _ => {
                println!("Result: {:?}", prg)
            },
        }
    }
}

fn decode(ins: &[usize]) -> Instruction {
    if ins.len() != WORD_SIZE {
        return Instruction::Invalid
    }

    match ins[0] {
        1 => Instruction::Add(ins[1] as usize, ins[2] as usize, ins[3] as usize),
        2 => Instruction::Mul(ins[1] as usize, ins[2] as usize, ins[3] as usize),
        99 => Instruction::Halt,
        _ => Instruction::Invalid,
    }
}

fn main() {
    //let mut prg: Vec<usize> = vec![1, 0, 0, 0, 99];
    //let mut prg: Vec<usize> = vec![1,9,10,3,2,3,11,0,99,30,40,50];
    //let mut prg: Vec<usize> = vec![1,1,1,4,99,5,6,0,99];
    //let mut prg: Vec<usize> = vec![2,4,4,5,99,0];
    let mut prg: Vec<usize> = vec![1,12,2,3,1,1,2,3,1,3,4,3,1,5,0,3,2,10,1,19,1,5,19,23,1,23,5,27,1,27,13,31,1,31,5,35,1,9,35,39,2,13,39,43,1,43,10,47,1,47,13,51,2,10,51,55,1,55,5,59,1,59,5,63,1,63,13,67,1,13,67,71,1,71,10,75,1,6,75,79,1,6,79,83,2,10,83,87,1,87,5,91,1,5,91,95,2,95,10,99,1,9,99,103,1,103,13,107,2,10,107,111,2,13,111,115,1,6,115,119,1,119,10,123,2,9,123,127,2,127,9,131,1,131,10,135,1,135,2,139,1,10,139,0,99,2,0,14,0];
    for idx in 0..prg.len()/WORD_SIZE {
        let start = idx*WORD_SIZE;
        let end = start+WORD_SIZE;
        decode(&prg[start..end]).exec(&mut prg[..]);
    }
}
