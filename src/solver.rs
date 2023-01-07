
use num::{Num, Zero};
use num::traits::real::Real;
use std::ops::SubAssign;
use std::fmt;

mod error {
	use std::fmt;
	use std::error;

	#[derive(Debug)]
	pub enum SolveError {
		TooSmall(usize),
		UnfittingEquationAmount(usize, usize),
		UnfittingCoefficientAmount(usize, usize),
		DependentSolutionSet,
		EmptySolutionSet,
	}

	impl fmt::Display for SolveError {
		fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
			match self {
				SolveError::TooSmall(size) =>
					write!(f, "Matrix size of {} is too small", size),				
				SolveError::UnfittingEquationAmount(amount, size) =>
					write!(f, "Amount {} of equations does not fit in matrix of size {}", amount, size),
				SolveError::UnfittingCoefficientAmount(amount, size) =>
					write!(f, "Amount {} of coefficients does not fit in matrix of size {}", amount, size),
				SolveError::DependentSolutionSet =>
					write!(f, "The system of equations is dependent"),
				SolveError::EmptySolutionSet =>
					write!(f, "The system of equations has no solution"),
			}
		}
	}

	impl error::Error for SolveError {}
}

use error::SolveError;

type Result<T> = std::result::Result<CoefficientMatrix<T>, SolveError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Equation<T> {
	coefficients: Vec<T>,
	result: T,
}

impl<T: Num + Copy> Equation<T>
where
	T: Num + Copy
{
	pub fn new(coefficients: Vec<T>, result: T) -> Equation<T> {
		Equation {
			coefficients,
			result,
		}
	}

	fn get(&self, idx: usize) -> T {
		*self.coefficients.get(idx).unwrap()
	}

	fn get_mut(&mut self, idx: usize) -> &mut T {
		self.coefficients.get_mut(idx).unwrap()
	}

	fn get_result(&self) -> T {
		self.result
	}

	fn get_result_mut(&mut self) -> &mut T {
		&mut self.result
	}
} 

impl<T> Equation<T> {
	fn len(&self) -> usize {
		self.coefficients.len()
	}
}

impl<T> fmt::Display for Equation<T>
where
	T: Num + Copy + fmt::Display + fmt::Debug
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:?} = {}", self.coefficients, self.result)
	}

}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoefficientMatrix<T> {
	size: usize,
	matrix: Vec<Equation<T>>,
}


impl<T> CoefficientMatrix<T>
where
	T: Num + Zero + Copy + fmt::Display + fmt::Debug + Real + SubAssign
{
	pub fn new(size: usize) -> Self {
		CoefficientMatrix {
			size,
			matrix: Vec::with_capacity(size),
		}
	}

	pub fn add_equation(mut self, equation: Equation<T>) -> Self {
		self.matrix.push(equation);
		self
	}

	pub fn validate(self) -> Result<T> {
		if self.size < 1 {
			return Err(SolveError::TooSmall(self.size));
		}

		if self.matrix.len() == self.size {
			let mut unfitting_amount = None;
			for equation in self.matrix.iter() {
				if equation.len() != self.size {
					unfitting_amount = Some(equation.len());
				}
			}
			match unfitting_amount {
				Some(amount) => Err(SolveError::UnfittingCoefficientAmount(amount, self.size)),
				None => Ok(self),
			}
 		} else {
			Err(SolveError::UnfittingEquationAmount(self.matrix.len(), self.size))
		}
	}

	// Convert the matrix to upper triangular form
	pub fn convert(mut self) -> Result<T> {
		// at this point self needs to be validated == have a size of more than 0
		for a in 0..self.size-1 {
			let mut pivot = self.matrix[a].get(a);
			
			// Search for and set a better pivot in case there is one
			for i in a+1..self.size {
				if self.matrix[i].get(a).abs() > pivot.abs() {
					self.matrix.swap(i, a);
					pivot = self.matrix[a].get(a);
				}
			}

			for b in a+1..self.size {
				let ratio = self.matrix[b].get(a) / pivot;
				for c in a..self.size {
					let eliminator = self.matrix[a].get(c) * ratio;
					*self.matrix[b].get_mut(c) -= eliminator;
				}
				let eliminator = self.matrix[a].get_result() * ratio;
				*self.matrix[b].get_result_mut() -= eliminator;
			}
		}

		Ok(self)
	}

	pub fn solve(mut self) -> Result<T> {
		for i in (0..self.size).rev() {
			let divisor = self.matrix[i].get(i);

			if divisor.is_zero() {
				if self.matrix[i].get_result().is_zero() {
					return Err(SolveError::DependentSolutionSet);
				} else {
					return Err(SolveError::EmptySolutionSet);
				}
			}

			// Divide each value in the current row with the row's leading coefficient
			for j in 0..self.size {
				let quotient = self.matrix[i].get(j) / divisor;
				*self.matrix[i].get_mut(j) = quotient;
			}
			let result_quotient = self.matrix[i].get_result() / divisor;
			*self.matrix[i].get_result_mut() = result_quotient;

			// Eliminate all coefficients in the current row's leading coefficient's column
			for j in (0..i).rev() {
				let factor = self.matrix[j].get(i);
				for k in 0..self.size {
					let eliminator = self.matrix[i].get(k) * factor;
					*self.matrix[j].get_mut(k) -= eliminator;
				}
				let result_eliminator = self.matrix[i].get_result() * factor;
				*self.matrix[j].get_result_mut() -= result_eliminator;
			}
		}

		Ok(self)
	}
}

impl<T> fmt::Display for CoefficientMatrix<T>
where T: Num + fmt::Display + fmt::Debug + Copy {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		for equation in self.matrix.iter() {
			write!(f, "{}\n", equation)?;
		}
		Ok(())
	}
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_to_upper_triangular() {
	    let converted = CoefficientMatrix::new(2)
	        .add_equation(Equation::new(vec![8.0, -6.0], 2.0))
	        .add_equation(Equation::new(vec![2.0,  3.0], 2.0))
	        .validate().unwrap()
	        .convert().unwrap();
	    let expected_result = CoefficientMatrix::new(2)
	        .add_equation(Equation::new(vec![8.0, -6.0], 2.0))
	        .add_equation(Equation::new(vec![0.0,  4.5], 1.5))
	        .validate().unwrap();
	    assert_eq!(converted, expected_result);
    }

    #[test]
    fn solve_upper_triangular() {
    	let solved = CoefficientMatrix::new(2)
	        .add_equation(Equation::new(vec![8.0, -6.0], 2.0))
	        .add_equation(Equation::new(vec![0.0,  4.5], 1.5))
	        .validate().unwrap()
	        .solve().unwrap();
	    let expected_result = CoefficientMatrix::new(2)
	        .add_equation(Equation::new(vec![1.0, 0.0], 0.5))
	        .add_equation(Equation::new(vec![0.0, 1.0], 1.0/3.0))
	        .validate().unwrap();
	    assert_eq!(solved, expected_result);
    }

    #[test]
    #[should_panic]
    fn equation_too_long() {
        let _ = CoefficientMatrix::new(2)
            .add_equation(Equation::new(vec![8.0, -6.0, 3.0], 2.0))
            .add_equation(Equation::new(vec![0.0,  4.5], 1.5))
            .validate()
            .expect("{err}");
    }
    #[test]
    #[should_panic]
    fn equation_too_short() {
        let _ = CoefficientMatrix::new(2)
            .add_equation(Equation::new(vec![8.0], 2.0))
            .add_equation(Equation::new(vec![0.0,  4.5], 1.5))
            .validate()
            .expect("{err}");
    }
    #[test]
    #[should_panic]
    fn matrix_too_long() {
        let _ = CoefficientMatrix::new(2)
            .add_equation(Equation::new(vec![8.0, -6.0], 2.0))
            .add_equation(Equation::new(vec![0.0,  4.5], 1.5))
            .add_equation(Equation::new(vec![3.0,  0.0], 5.0))
            .validate()
            .expect("{err}");
    }
    #[test]
    #[should_panic]
    fn matrix_too_short() {
        let _ = CoefficientMatrix::new(2)
            .add_equation(Equation::new(vec![8.0, -6.0], 2.0))
            .validate()
            .expect("{err}");
    }
    #[test]
    fn matrix_valid() {
        let _ = CoefficientMatrix::new(2)
            .add_equation(Equation::new(vec![8.0, -6.0], 2.0))
            .add_equation(Equation::new(vec![0.0,  4.5], 1.5))
            .validate()
            .expect("{err}");
    }
}
