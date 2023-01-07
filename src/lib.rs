
mod solver;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen(start)]
pub fn run() {
    /*let mat = solver::CoefficientMatrix::new(2)
        .add_equation(solver::Equation::new(vec![8.0, -6.0], 2.0))
        .add_equation(solver::Equation::new(vec![2.0,  3.0], 2.0))
        .validate().unwrap();
    let solved = mat.clone()
        .convert().unwrap()
        .solve().unwrap();

    console_log!("Matrix:\n{}", mat);
    console_log!("Solved:\n{}", solved);*/
    let mat = solver::CoefficientMatrix::build(2)  // build returns a Builder struct which contains
                                                   // the following methods!
        .add_equation(solver::Equation::new(vec![8.0, -6.0], 2.0))
        .add_equation(solver::Equation::new(vec![2.0,  3.0], 2.0))
        .validate().unwrap();
}

#[wasm_bindgen]
pub struct MatrixSolver {
    matrix: solver::CoefficientMatrix<f64>,
}

#[wasm_bindgen]
impl MatrixSolver {
    pub fn new(size: usize) -> MatrixSolver {
        MatrixSolver {
            matrix: solver::CoefficientMatrix::<f64>::new(size),
        }
    }

    pub fn add_eq(&mut self, val: JsValue, result: f64) -> Result<(), JsValue> {
        let coefficients: Vec<f64> = serde_wasm_bindgen::from_value(val)?;
        let temp = self.matrix.clone();
        self.matrix = temp.add_equation(solver::Equation::new(coefficients, result));
        Ok(())
    }

    pub fn solve(&mut self) {
        console_log!("Before:\n{}", self.matrix);
        let temp = self.matrix.clone();
        self.matrix = temp
            .validate().unwrap()
            .convert().unwrap()
            .solve().unwrap();
        console_log!("Solved:\n{}", self.matrix);
    }
}
