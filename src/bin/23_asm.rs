type Offset = i16;

enum Arg {
    Number(Offset),
    Register(usize),
}

#[derive(Debug)]
enum Inst {
    Hlf(usize),
    Tpl(usize),
    Inc(usize),
    Jmp(Offset),
    Jie(usize, Offset),
    Jio(usize, Offset),
}

fn run(program: &[Inst], regs: &[u32]) -> Vec<u32> {
    let mut regs = regs.to_vec();
    let mut pc: usize = 0;

    loop {
        let offset = match program[pc] {
            Inst::Hlf(r) => {
                regs[r] /= 2;
                1
            }
            Inst::Tpl(r) => {
                regs[r] *= 3;
                1
            }
            Inst::Inc(r) => {
                regs[r] += 1;
                1
            }
            Inst::Jmp(offset) => offset,
            Inst::Jie(r, offset) => {
                if regs[r] % 2 == 0 {
                    offset
                } else {
                    1
                }
            }
            Inst::Jio(r, offset) => {
                if regs[r] == 1 {
                    offset
                } else {
                    1
                }
            }
        };

        if offset < 0 {
            let noffset = -offset as usize;
            if noffset > pc {
                return regs;
            } else {
                pc -= noffset;
            }
        } else {
            pc += offset as usize;
            if pc >= program.len() {
                return regs;
            }
        }
    }
}

fn parse_arg(s: &str) -> Arg {
    if let Ok(i) = s.parse::<Offset>() {
        Arg::Number(i)
    } else if s == "a" {
        Arg::Register(0)
    } else if s == "b" {
        Arg::Register(1)
    } else {
        panic!("Unknown arg {}", s)
    }
}

fn main() {
    let insts = adventofcode::read_input_lines(|l| {
        let op_and_args: Vec<&str> = l.splitn(2, ' ').collect();
        let args: Vec<Arg> = op_and_args[1].split(", ").map(parse_arg).collect();
        match (op_and_args[0], &args[..]) {
            ("hlf", &[Arg::Register(r)]) => Inst::Hlf(r),
            ("tpl", &[Arg::Register(r)]) => Inst::Tpl(r),
            ("inc", &[Arg::Register(r)]) => Inst::Inc(r),
            ("jmp", &[Arg::Number(off)]) => Inst::Jmp(off),
            ("jie", &[Arg::Register(r), Arg::Number(off)]) => Inst::Jie(r, off),
            ("jio", &[Arg::Register(r), Arg::Number(off)]) => Inst::Jio(r, off),
            _ => panic!("unknown inst {}", l),
        }
    });

    println!("{}", run(&insts, &[0, 0])[1]);
    println!("{}", run(&insts, &[1, 0])[1]);
}
