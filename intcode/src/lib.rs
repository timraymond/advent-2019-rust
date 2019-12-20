#[cfg(test)]
mod tests {
    use super::*;

    struct IOBuf {
        input: Vec<i32>,
        output: Vec<i32>,
    }

    impl NumProducer for IOBuf {
        fn output(&mut self, out: i32) {
            self.output.push(out);
        }
    }

    impl NumConsumer for IOBuf {
        fn input(&mut self) -> i32 {
            self.input.pop().unwrap()
        }
    }

    #[test]
    fn test_add() {
        let mut prg = vec!(1, 5, 6, 7, 99, 1, 2, 0, 0);
        let mut inbuf = IOBuf{
            input: Vec::new(),
            output: Vec::new(),
        };
        let mut outbuf = IOBuf{
            input: Vec::new(),
            output: Vec::new(),
        };
        exec(&mut prg, &mut inbuf, &mut outbuf);
        assert_eq!(prg[7], 3);
    }

    #[test]
    fn test_mul() {
        let mut prg = vec!(2, 5, 6, 7, 99, 1, 2, 0, 0);
        let mut inbuf = IOBuf{
            input: Vec::new(),
            output: Vec::new(),
        };
        let mut outbuf = IOBuf{
            input: Vec::new(),
            output: Vec::new(),
        };
        exec(&mut prg, &mut inbuf, &mut outbuf);
        assert_eq!(prg[7], 2);
    }

    #[test]
    fn test_input_output() {
        let mut prg = vec!(3, 0, 4, 0, 99);
        let mut inbuf = IOBuf{
            input: vec![42],
            output: Vec::new(),
        };
        let mut outbuf = IOBuf{
            input: Vec::new(),
            output: Vec::new(),
        };
        exec(&mut prg, &mut inbuf, &mut outbuf);
        assert_eq!(outbuf.output[0], 42);
    }
}

enum Instruction {
    Add(i32, i32, i32),
    Mul(i32, i32, i32),
    In(i32),
    Out(i32),
    Halt,
    Invalid,
}

impl Instruction {
    fn exec(&self, prg: &mut [i32], input: &mut impl NumConsumer, output: &mut impl NumProducer) {
        match self {
            Instruction::Add(a, b, out) => {
                prg[*out as usize] = prg[*a as usize] + prg[*b as usize];
            },
            Instruction::Mul(a, b, out) => {
                prg[*out as usize] = prg[*a as usize] * prg[*b as usize];
            },
            Instruction::In(dest) => {
                prg[*dest as usize] = input.input();
            }
            Instruction::Out(source) => {
                output.output(prg[*source as usize]);
            }
            _ => (),
        }
    }

    fn size(&self) -> usize {
        match self {
            Instruction::Add(_, _, _) => 4,
            Instruction::Mul(_, _, _) => 4,
            Instruction::In(_) => 2,
            Instruction::Out(_) => 2,
            _ => 1,
        }
    }
}

fn decode(ins: &[i32]) -> Instruction {
    match ins[0] {
        1 => Instruction::Add(ins[1], ins[2], ins[3]),
        2 => Instruction::Mul(ins[1], ins[2], ins[3]),
        3 => Instruction::In(ins[1]),
        4 => Instruction::Out(ins[1]),
        99 => Instruction::Halt,
        _ => Instruction::Invalid,
    }
}

pub trait NumProducer {
    fn output(&mut self, out: i32);
}

pub trait NumConsumer {
    fn input(&mut self) -> i32;
}

pub fn exec(prg: &mut [i32], input: &mut impl NumConsumer, out: &mut impl NumProducer) {
    let mut pc = 0;

    loop {
        println!("pc: {}, slice: {:?}", pc, &prg[pc..]);
        let ins = decode(&prg[pc..]);

        match ins {
            Instruction::Halt => {
                return;
            },
            Instruction::Invalid => {
                panic!("invalid instruction");
            },
            _ => {
                ins.exec(prg, input, out);
                pc += ins.size();
            },
        }
    }
}
