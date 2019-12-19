#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

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
                prg[out.clone()] = prg[a.clone()] + prg[b.clone()];
            },
            Instruction::Mul(a, b, out) => {
                prg[out.clone()] = prg[a.clone()] * prg[b.clone()];
            },
            _ => {
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

pub fn exec(prg: &mut [usize]) {
    for idx in 0..prg.len()/WORD_SIZE {
        let start = idx*WORD_SIZE;
        let end = start+WORD_SIZE;
        decode(&prg[start..end]).exec(&mut prg[..]);
    }
}
