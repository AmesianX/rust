// error-pattern: Unsatisfied precondition constraint

fn test() { let w: [int]; w.(5) = 0; }

fn main() { test(); }
