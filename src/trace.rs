//! Runtime tracing module for coldbrew.
use core::fmt;
use std::collections::HashSet;
use std::fmt::Write;

use crate::bytecode::OPCode;
use crate::runtime::{Instruction, ProgramCounter, Value};

/// Trace recording involves capturing an execution trace of the program in
/// various places. Each record entry in the trace is a tuple of (pc, inst)
/// where pc is the program counter (position of the entry in the bytecode)
/// and inst is the instruction executed there.
#[derive(Debug, Clone)]
pub struct Record {
    pc: ProgramCounter,
    inst: Instruction,
}

impl Record {
    pub fn instruction(&self) -> Instruction {
        self.inst.clone()
    }

    pub fn pc(&self) -> ProgramCounter {
        self.pc
    }
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:} @ {:}", self.inst, self.pc)
    }
}

#[derive(Debug, Clone)]
pub struct Trace {
    pub start: ProgramCounter,
    pub trace: Vec<Record>,
    // PC's of branch targets inside the trace.
    inner_branch_targets: HashSet<ProgramCounter>,
    // PC's of branch targets outside the trace.
    outer_branch_targets: HashSet<ProgramCounter>,
}

/// Recorder is the runtime component responsible for recording traces.
pub struct Recorder {
    trace_start: ProgramCounter,
    loop_header: ProgramCounter,
    is_recording: bool,
    last_instruction_was_branch: bool,
    pub trace: Vec<Record>,
    inner_branch_targets: HashSet<ProgramCounter>,
    outer_branch_targets: HashSet<ProgramCounter>,
}

impl Default for Recorder {
    fn default() -> Self {
        Self::new()
    }
}

impl Recorder {
    pub fn new() -> Self {
        Self {
            trace_start: ProgramCounter::default(),
            loop_header: ProgramCounter::default(),
            is_recording: false,
            last_instruction_was_branch: false,
            trace: Vec::new(),
            inner_branch_targets: HashSet::new(),
            outer_branch_targets: HashSet::new(),
        }
    }

    /// Check if we are recording a trace already.
    pub fn is_recording(&self) -> bool {
        self.is_recording
    }

    /// Check if we finished recording a trace.
    pub fn is_done_recording(&mut self, pc: ProgramCounter) -> bool {
        if self.trace.is_empty() {
            return false;
        }
        match self.trace.last() {
            Some(entry) => match entry.inst.get_mnemonic() {
                OPCode::Return
                | OPCode::IReturn
                | OPCode::LReturn
                | OPCode::FReturn
                | OPCode::DReturn => {
                    // If we found a recursive call we need to exit.
                    if pc.get_method_index() == entry.pc.get_method_index() {
                        self.is_recording = false;
                        return false;
                    }
                    pc == self.loop_header
                }
                _ => pc == self.loop_header,
            },
            None => false,
        }
    }

    /// Record the bytecode instruction at the given `pc` and `inst`
    /// the final recorded traces are linear, straight line code with
    /// no loops or function calls (ideally some calls could be inlined).
    ///
    /// During the recording phase if any aborting condition is met we stop
    /// recording and return. The aborting conditions are (1) jumps to outer
    /// branches, (2) function calls or (3) conditional branches.
    pub fn record(&mut self, pc: ProgramCounter, mut inst: Instruction) {
        // FIXME: This is not needed since we want to insert guards when
        // running traces. The only way to insert a guard is to interpret
        // the branching instruction.
        // Branch flip if the last recorded instruction was a branch.
        if self.last_instruction_was_branch {
            // self.flip_branch(pc);
        }
        match inst.get_mnemonic() {
            OPCode::Goto => {
                // println!("Found Goto instruction");
                let offset = match inst.nth(0) {
                    Some(Value::Int(v)) => v,
                    _ => panic!(
                        "Expected Goto to have at least one integer parameter"
                    ),
                };
                if offset > 0 {
                    println!("Found forward branch, aborting");
                    return;
                } else {
                    let mut branch_target = pc;
                    branch_target.inc_instruction_index(offset);
                    if self.trace_start == branch_target {
                        self.inner_branch_targets.insert(branch_target);
                    } else {
                        self.outer_branch_targets.insert(branch_target);
                    }
                }
            }
            OPCode::IfNe
            | OPCode::IfEq
            | OPCode::IfGt
            | OPCode::IfICmpGe
            | OPCode::IfICmpGt
            | OPCode::IfICmpLt
            | OPCode::IfICmpLe
            | OPCode::IfICmpNe
            | OPCode::IfICmpEq => {
                self.last_instruction_was_branch = true;
            }
            OPCode::InvokeStatic => {
                // Check for recursive function calls by comparing the invoked
                // method index with the one we are currently recording.
                let method_index = match inst.nth(0) {
                    Some(Value::Int(v)) => v,
                    _ => panic!(
                        "Expected InvokeStatic to have at least one parameter"
                    ),
                };
                if self.trace_start.get_method_index() == method_index as usize
                {
                    self.is_recording = false;
                    println!("Found recursive call -- abort recording");
                    return;
                }
            }
            OPCode::Iconst0
            | OPCode::Iconst1
            | OPCode::Iconst2
            | OPCode::Iconst3
            | OPCode::Iconst4
            | OPCode::Iconst5
            | OPCode::IconstM1
            | OPCode::Lconst0
            | OPCode::Lconst1
            | OPCode::Fconst0
            | OPCode::Fconst1
            | OPCode::Fconst2
            | OPCode::Dconst0
            | OPCode::Dconst1
            | OPCode::ILoad0
            | OPCode::ILoad1
            | OPCode::ILoad2
            | OPCode::ILoad3
            | OPCode::DLoad0
            | OPCode::DLoad1
            | OPCode::DLoad2
            | OPCode::DLoad3
            | OPCode::FLoad0
            | OPCode::FLoad1
            | OPCode::FLoad2
            | OPCode::FLoad3
            | OPCode::LLoad0
            | OPCode::LLoad1
            | OPCode::LLoad2
            | OPCode::LLoad3
            | OPCode::IStore0
            | OPCode::IStore1
            | OPCode::IStore2
            | OPCode::IStore3
            | OPCode::FStore0
            | OPCode::FStore1
            | OPCode::FStore2
            | OPCode::FStore3
            | OPCode::DStore0
            | OPCode::DStore1
            | OPCode::DStore2
            | OPCode::DStore3 => {
                if let Some(value) = Self::get_params(inst.get_mnemonic()) {
                    inst = Instruction::new(
                        inst.get_mnemonic(),
                        Some(vec![value]),
                    );
                }
            }
            _ => (),
        }
        self.trace.push(Record {
            pc,
            inst: inst.clone(),
        });
    }

    /// Returns the `jvm::Value` from a given mnemonic.
    const fn get_params(opcode: OPCode) -> Option<Value> {
        match opcode {
            OPCode::ILoad0
            | OPCode::FLoad0
            | OPCode::LLoad0
            | OPCode::DLoad0
            | OPCode::IStore0
            | OPCode::FStore0
            | OPCode::LStore0
            | OPCode::DStore0
            | OPCode::Iconst0 => Some(Value::Int(0)),
            OPCode::ILoad1
            | OPCode::FLoad1
            | OPCode::LLoad1
            | OPCode::DLoad1
            | OPCode::IStore1
            | OPCode::FStore1
            | OPCode::LStore1
            | OPCode::DStore1
            | OPCode::Iconst1 => Some(Value::Int(1)),
            OPCode::ILoad2
            | OPCode::FLoad2
            | OPCode::LLoad2
            | OPCode::DLoad2
            | OPCode::IStore2
            | OPCode::FStore2
            | OPCode::LStore2
            | OPCode::DStore2
            | OPCode::Iconst2 => Some(Value::Int(2)),
            OPCode::ILoad3
            | OPCode::FLoad3
            | OPCode::LLoad3
            | OPCode::DLoad3
            | OPCode::IStore3
            | OPCode::FStore3
            | OPCode::LStore3
            | OPCode::DStore3
            | OPCode::Iconst3 => Some(Value::Int(3)),
            OPCode::Iconst4 => Some(Value::Int(4)),
            OPCode::Iconst5 => Some(Value::Int(5)),
            OPCode::IconstM1 => Some(Value::Int(-1)),
            OPCode::Fconst0 => Some(Value::Float(0.)),
            OPCode::Fconst1 => Some(Value::Float(1.)),
            OPCode::Fconst2 => Some(Value::Float(2.)),
            OPCode::Lconst0 => Some(Value::Long(0)),
            OPCode::Lconst1 => Some(Value::Long(1)),
            OPCode::Dconst0 => Some(Value::Double(0.)),
            OPCode::Dconst1 => Some(Value::Double(1.)),
            _ => None,
        }
    }

    /// Init a trace recording.
    pub fn init(&mut self, loop_header: ProgramCounter, start: ProgramCounter) {
        if self.is_recording && self.trace_start == start {
            return;
        }
        self.is_recording = true;
        self.last_instruction_was_branch = false;
        self.trace_start = start;
        self.loop_header = loop_header;
        // Clear existing traces.
        self.trace.clear();
        self.inner_branch_targets.clear();
        self.outer_branch_targets.clear();
    }

    /// Return the last recorded trace.
    pub fn recording(&mut self) -> Trace {
        self.is_recording = false;
        Trace {
            start: self.trace_start,
            trace: self.trace.clone(),
            inner_branch_targets: self.inner_branch_targets.clone(),
            outer_branch_targets: self.outer_branch_targets.clone(),
        }
    }

    /// Prints the recorded trace to stdout.
    ///
    /// # Errors
    /// Returns an error if the underlying calls to `write!` fail.
    pub fn debug(&self) -> std::fmt::Result {
        let mut s = String::new();
        writeln!(&mut s, "---- ------ TRACE ------ ----")?;
        for record in &self.trace {
            let inst = &record.inst;
            write!(&mut s, "{} ", inst.get_mnemonic())?;
            for param in &inst.get_params() {
                write!(&mut s, "{param:?} ")?;
            }
            writeln!(&mut s)?;
        }
        writeln!(&mut s, "---- ------------------- ----")?;

        println!("{s}");
        Ok(())
    }

    /// Flip branch condition so the jump occurs if the execution doesn't
    /// follow the trace.
    fn flip_branch(&mut self, pc: ProgramCounter) {
        self.last_instruction_was_branch = false;
        let Some(branch_entry) = self.trace.pop() else {
            return;
        };
        let mut branch_target = branch_entry.pc;
        let mut offset = branch_entry.inst.get_params().map_or_else(
            || panic!("Expected branch target to have parameters"),
            |params| match &params[0] {
                Value::Int(m) => m.to_owned(),
                _ => panic!("Expected branch target index to be i32"),
            },
        );
        branch_target.inc_instruction_index(offset);
        if branch_target == pc {
            println!("Flipping branch @ {}", branch_entry.inst.get_mnemonic());
            offset = 3;
            branch_target = branch_entry.pc;
            branch_target.inc_instruction_index(offset);
            branch_entry.inst.get_params().map_or_else(
                || panic!("Expected branch target to have parameters"),
                |mut params| {
                    if let Some(Value::Int(m)) = params.get_mut(0) {
                        *m = offset;
                    }
                },
            );
            let flipped = match branch_entry.inst.get_mnemonic() {
                OPCode::IfNe => OPCode::IfEq,
                OPCode::IfGt => OPCode::IfLe,
                OPCode::IfICmpGe => OPCode::IfICmpLt,
                OPCode::IfICmpGt => OPCode::IfICmpLe,
                OPCode::IfICmpLe => OPCode::IfICmpGt,
                OPCode::IfICmpNe => OPCode::IfICmpEq,
                _ => unreachable!(
                    "Found unsupported branch entry {}",
                    branch_entry.inst
                ),
            };
            println!("Flipped branch is {}", flipped);
            let new_branch_taget =
                Instruction::new(flipped, branch_entry.inst.get_params());
            self.trace.push(Record {
                pc: branch_entry.pc,
                inst: new_branch_taget,
            });

            if offset < 0 {
                self.inner_branch_targets.insert(branch_target);
            } else {
                self.outer_branch_targets.insert(branch_target);
            }
        }
    }
}
