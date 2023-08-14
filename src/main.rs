#[derive(Copy, Clone)]
enum InstructionSet {
    PSH(i32),
    ADD,
    POP,
    HLT
}

fn main() {
    let mut is_running: bool = true;
    let mut ip: usize = 0;
    let mut sp: usize = 0;

    let program: [InstructionSet; 5] = [
        InstructionSet::PSH(5),
        InstructionSet::PSH(6),
        InstructionSet::ADD,
        InstructionSet::POP,
        InstructionSet::HLT
    ];

    let mut stack: [i32; 256] = [0; 256];

    while is_running {
        let current_instr = program[ip];
        match current_instr{
            InstructionSet::HLT => {
                is_running = false;
                print!("Done\n");
            }

            InstructionSet::PSH(Val) => {
                sp += 1;
                stack[sp] = Val;
            }

            InstructionSet::POP => {
                let val_popped: i32 = stack[sp];
                sp = sp - 1;
                print!("Popped {:?} \n", val_popped);
            }

            InstructionSet::ADD => {
                let lhs = stack[sp];
                sp = sp - 1;

                let rhs = stack[sp];
                sp = sp - 1;

                let res = lhs + rhs;
                sp = sp + 1;

                stack[sp] = res;
            }
        }
        ip += 1;
    }
}
