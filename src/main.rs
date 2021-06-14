mod strs;
mod types;
mod tuples;
mod vectors;
mod conditionals;
mod loopsfizzbuzz;
mod enums;
mod functions;
mod pointer_refs;
mod structs;
fn main() {
    types :: run();
    strs::run();
    tuples::run();
    vectors::run();
    conditionals::run();
    loopsfizzbuzz::main();
    functions::run();
    pointer_refs::run();
    structs::run();
    enums::run();
}