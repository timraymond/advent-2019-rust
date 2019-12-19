#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let mut prg = vec!(1, 4, 5, 6, 1, 2, 0, 0);
        exec(&mut prg);
        assert_eq!(prg[6], 3);
    }

    #[test]
    fn test_mul() {
        let mut prg = vec!(2, 4, 5, 6, 1, 2, 0, 0);
        exec(&mut prg);
        assert_eq!(prg[6], 2);
    }

    #[test]
    fn test_halt() {
        let mut prg = vec!(1, 4, 5, 6, 1, 2, 0, 0, 99, 0, 0, 0, 2, 4, 5, 6);
        exec(&mut prg);
        assert_eq!(prg[6], 3);
    }
}

const WORD_SIZE: usize = 4;

enum Instruction {
    Add(usize, usize, usize),
    Mul(usize, usize, usize),
    Halt,
    Invalid,
}

#[derive(Clone,Copy)]
enum Condition {
    Okay,
    Halted,
    Exception,
}

impl Instruction {
    fn exec(&self, prg: &mut [usize], cond: &Condition) -> Condition {
        if let Condition::Okay = cond {
            match self {
                Instruction::Add(a, b, out) => {
                    prg[out.clone()] = prg[a.clone()] + prg[b.clone()];
                    return Condition::Okay;
                },
                Instruction::Mul(a, b, out) => {
                    prg[out.clone()] = prg[a.clone()] * prg[b.clone()];
                    return Condition::Okay;
                },
                Instruction::Halt => Condition::Halted,
                Instruction::Invalid => Condition::Exception,
            }
        } else {
            return *cond
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
    let mut cond = Condition::Okay;

    for idx in 0..prg.len()/WORD_SIZE {
        let start = idx*WORD_SIZE;
        let end = start+WORD_SIZE;
        cond = decode(&prg[start..end]).exec(&mut prg[..], &cond);
    }
}
