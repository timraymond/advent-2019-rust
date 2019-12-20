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

    #[test]
    fn test_modes() {
        let mut prg = vec!(1002, 4, 3, 4, 33);
        let mut inbuf = IOBuf{
            input: Vec::new(),
            output: Vec::new(),
        };
        let mut outbuf = IOBuf{
            input: Vec::new(),
            output: Vec::new(),
        };
        exec(&mut prg, &mut inbuf, &mut outbuf);
        assert_eq!(prg[4], 99);
    }

    #[test]
    fn test_jump_if_true() {
        let mut prg = vec!(5, 1, 5, 99, 0, 1101, 2, 2, 4, 99);
        let mut inbuf = IOBuf{
            input: Vec::new(),
            output: Vec::new(),
        };
        let mut outbuf = IOBuf{
            input: Vec::new(),
            output: Vec::new(),
        };
        exec(&mut prg, &mut inbuf, &mut outbuf);
        assert_eq!(prg[4], 4);
    }

    #[test]
    fn test_jump_if_false() {
        let mut prg = vec!(6, 0, 5, 99, 0, 1101, 2, 2, 4, 99);
        let mut inbuf = IOBuf{
            input: Vec::new(),
            output: Vec::new(),
        };
        let mut outbuf = IOBuf{
            input: Vec::new(),
            output: Vec::new(),
        };
        exec(&mut prg, &mut inbuf, &mut outbuf);
        assert_eq!(prg[4], 4);
    }

    #[test]
    fn test_less_than_true() {
        let mut prg = vec!(1107, 1, 2, 5, 99, 42);
        let mut inbuf = IOBuf{
            input: Vec::new(),
            output: Vec::new(),
        };
        let mut outbuf = IOBuf{
            input: Vec::new(),
            output: Vec::new(),
        };
        exec(&mut prg, &mut inbuf, &mut outbuf);
        assert_eq!(prg[5], 1);
    }

    #[test]
    fn test_less_than_false() {
        let mut prg = vec!(1107, 2, 1, 5, 99, 42);
        let mut inbuf = IOBuf{
            input: Vec::new(),
            output: Vec::new(),
        };
        let mut outbuf = IOBuf{
            input: Vec::new(),
            output: Vec::new(),
        };
        exec(&mut prg, &mut inbuf, &mut outbuf);
        assert_eq!(prg[5], 0);
    }

    #[test]
    fn test_equal_true() {
        let mut prg = vec!(1108, 1, 1, 5, 99, 42);
        let mut inbuf = IOBuf{
            input: Vec::new(),
            output: Vec::new(),
        };
        let mut outbuf = IOBuf{
            input: Vec::new(),
            output: Vec::new(),
        };
        exec(&mut prg, &mut inbuf, &mut outbuf);
        assert_eq!(prg[5], 1);
    }

    #[test]
    fn test_equal_false() {
        let mut prg = vec!(1108, 1, 2, 5, 99, 42);
        let mut inbuf = IOBuf{
            input: Vec::new(),
            output: Vec::new(),
        };
        let mut outbuf = IOBuf{
            input: Vec::new(),
            output: Vec::new(),
        };
        exec(&mut prg, &mut inbuf, &mut outbuf);
        assert_eq!(prg[5], 0);
    }
}

enum Instruction {
    Add(Param, Param, Param),
    Mul(Param, Param, Param),
    In(i32),
    Out(i32),
    JumpIfTrue(Param, Param),
    JumpIfFalse(Param, Param),
    Equal(Param, Param, i32),
    LessThan(Param, Param, i32),
    Halt,
    Invalid,
}

#[derive(Debug)]
enum Param {
    Immediate(i32),
    Address(i32),
}

impl Param {
    fn retrieve(&self, prg: &[i32]) -> i32 {
        match self {
            Param::Immediate(val) => (*val),
            Param::Address(adr) => (prg[*adr as usize]),
        }
    }
}

impl Instruction {
    fn exec(&self, pc: &usize, prg: &mut [i32], input: &mut impl NumConsumer, output: &mut impl NumProducer) -> usize {
        match self {
            Instruction::Add(a, b, out) => {
                let a_val = a.retrieve(&prg);
                let b_val = b.retrieve(&prg);

                if let Param::Address(adr) = out {
                    prg[*adr as usize] = a_val + b_val;
                } else {
                    panic!("decoding bug: out is not an address!");
                }
                return pc + 4;
            },
            Instruction::Mul(a, b, out) => {
                let a_val = a.retrieve(&prg);
                let b_val = b.retrieve(&prg);

                if let Param::Address(adr) = out {
                    prg[*adr as usize] = a_val * b_val;
                } else {
                    panic!("decoding bug: out is not an address!");
                }
                return pc + 4;
            },
            Instruction::In(dest) => {
                prg[*dest as usize] = input.input();
                return pc + 2;
            },
            Instruction::Out(src) => {
                output.output(prg[*src as usize]);
                return pc + 2;
            },
            Instruction::JumpIfTrue(condition, target) => {
                let cond = condition.retrieve(&prg);
                let tgt = target.retrieve(&prg);
                if cond != 0 {
                    return tgt as usize;
                } else {
                    return pc + 3;
                }
            },
            Instruction::JumpIfFalse(condition, target) => {
                let cond = condition.retrieve(&prg);
                let tgt = target.retrieve(&prg);
                if cond == 0 {
                    return tgt as usize;
                } else {
                    return pc + 3;
                }
            },
            Instruction::LessThan(first_operand, second_operand, target) => {
                let first = first_operand.retrieve(&prg);
                let second = second_operand.retrieve(&prg);

                if first < second {
                    prg[*target as usize] = 1;
                } else {
                    prg[*target as usize] = 0;
                }
                return pc + 4;
            },
            Instruction::Equal(first_operand, second_operand, target) => {
                let first = first_operand.retrieve(&prg);
                let second = second_operand.retrieve(&prg);

                if first == second {
                    prg[*target as usize] = 1;
                } else {
                    prg[*target as usize] = 0;
                }
                return pc + 4;
            }
            _ => (0)
        }
    }
}

fn decode_param(val: i32, mode: i32) -> Param {
    println!("mode: {}", mode);
    if mode == 1 {
        Param::Immediate(val)
    } else {
        Param::Address(val)
    }
}

fn decode(ins: &[i32]) -> Instruction {
    let opcode = ins[0] % 100;
    let p1_mode = (ins[0] / 100) % 10;
    let p2_mode = (ins[0] / 1_000) % 10;

    println!("op: {}, p1: {}, p2: {}", opcode, p1_mode, p2_mode);

    match opcode {
        1 => Instruction::Add(
            decode_param(ins[1], p1_mode),
            decode_param(ins[2], p2_mode),
            Param::Address(ins[3]),
        ),
        2 => Instruction::Mul(
            decode_param(ins[1], p1_mode),
            decode_param(ins[2], p2_mode),
            Param::Address(ins[3]),
        ),
        3 => Instruction::In(ins[1]),
        4 => Instruction::Out(ins[1]),
        5 => Instruction::JumpIfTrue(decode_param(ins[1], p1_mode), decode_param(ins[2], p2_mode)),
        6 => Instruction::JumpIfFalse(decode_param(ins[1], p1_mode), decode_param(ins[2], p2_mode)),
        7 => Instruction::LessThan(
            decode_param(ins[1], p1_mode),
            decode_param(ins[2], p2_mode),
            ins[3],
        ),
        8 => Instruction::Equal(
            decode_param(ins[1], p1_mode),
            decode_param(ins[2], p2_mode),
            ins[3],
        ),
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
                pc = ins.exec(&pc, prg, input, out);
                println!("next: {}", pc);
            },
        }
    }
}
