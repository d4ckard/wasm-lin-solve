import * as wasm from "mat-solve";

let solver = wasm.MatrixSolver.new(2);
solver.add_eq([8.0, -6.0], 2.0);
solver.add_eq([2.0, 3.0], 2.0);
solver.solve();
