use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use indicatif::ProgressBar;

use crate::{io::logging::Logger, utils::strings::decode_hex};

use super::{
    evm::vm::VM,
    signatures::{resolve_function_signature, ResolvedFunction},
};

// find all function selectors in the given EVM assembly.
pub fn find_function_selectors(assembly: String) -> Vec<String> {
    let mut function_selectors = Vec::new();

    // search through assembly for PUSH4 instructions, optimistically assuming that they are function selectors
    let assembly: Vec<String> = assembly.split('\n').map(|line| line.trim().to_string()).collect();
    for line in assembly.iter() {
        let instruction_args: Vec<String> = line.split(' ').map(|arg| arg.to_string()).collect();

        if instruction_args.len() >= 2 {
            let instruction = instruction_args[1].clone();

            if instruction == "PUSH4" {
                let function_selector = instruction_args[2].clone();
                function_selectors.push(function_selector);
            }
        }
    }
    function_selectors.sort();
    function_selectors.dedup();
    function_selectors
}

// resolve a selector's function entry point from the EVM bytecode
pub fn resolve_entry_point(evm: &VM, selector: String) -> u128 {
    let mut vm = evm.clone();
    let mut handled_jumps = HashSet::new();

    // execute the EVM call to find the entry point for the given selector
    vm.calldata = decode_hex(&selector).unwrap();
    while vm.bytecode.len() >= vm.instruction as usize {
        let call = vm.step();

        // if the opcode is an JUMPI and it matched the selector, the next jumpi is the entry point
        if call.last_instruction.opcode == 0x57 {
            let jump_condition = call.last_instruction.input_operations[1].solidify();
            let jump_taken = call.last_instruction.inputs[1].try_into().unwrap_or(1);

            if jump_condition.contains(&selector)
                && jump_condition.contains("msg.data[0]")
                && jump_condition.contains(" == ")
                && jump_taken == 1
            {
                return call.last_instruction.inputs[0].try_into().unwrap_or(0);
            } else if jump_taken == 1 {
                // if handled_jumps contains the jumpi, we have already handled this jump.
                // loops aren't supported in the dispatcher, so we can just return 0
                if handled_jumps.contains(&call.last_instruction.inputs[0].try_into().unwrap_or(0))
                {
                    return 0;
                } else {
                    handled_jumps.insert(call.last_instruction.inputs[0].try_into().unwrap_or(0));
                }
            }
        }

        if vm.exitcode != 255 || !vm.returndata.is_empty() {
            break;
        }
    }

    0
}

// resolve a function signature from the given selectors
pub fn resolve_function_selectors(
    selectors: Vec<String>,
    logger: &Logger,
) -> HashMap<String, Vec<ResolvedFunction>> {
    let resolved_functions: Arc<Mutex<HashMap<String, Vec<ResolvedFunction>>>> =
        Arc::new(Mutex::new(HashMap::new()));
    let resolve_progress: Arc<Mutex<ProgressBar>> =
        Arc::new(Mutex::new(ProgressBar::new_spinner()));

    let mut threads = Vec::new();

    resolve_progress.lock().unwrap().enable_steady_tick(Duration::from_millis(100));
    resolve_progress.lock().unwrap().set_style(logger.info_spinner());

    for selector in selectors {
        let function_clone = resolved_functions.clone();
        let resolve_progress = resolve_progress.clone();

        // create a new thread for each selector
        threads.push(thread::spawn(move || {
            if let Some(function) = resolve_function_signature(&selector) {
                let mut _resolved_functions = function_clone.lock().unwrap();
                let mut _resolve_progress = resolve_progress.lock().unwrap();
                _resolve_progress
                    .set_message(format!("resolved {} selectors...", _resolved_functions.len()));
                _resolved_functions.insert(selector, function);
            }
        }));
    }

    // wait for all threads to finish
    for thread in threads {
        thread.join().unwrap();
    }

    resolve_progress.lock().unwrap().finish_and_clear();

    let x = resolved_functions.lock().unwrap().clone();
    x
}
