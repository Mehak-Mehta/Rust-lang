mod strs;
mod types;
mod tuples;
mod vectors;
mod conditionals;
mod loopsfizzbuzz;
mod functions;
fn main() {
    types :: run();
    strs::run();
    tuples::run();
    vectors::run();
    conditionals::run();
   loopsfizzbuzz::main();
   functions::run();
}