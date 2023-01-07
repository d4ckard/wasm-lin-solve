use std::str;
use std::fmt;
use num::Num;

pub enum Error {
    EvaluationError,
    BuildError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::EvaluationError => write!(f, "Failed to evalue function"),
            Error::BuildError => write!(f, "Invalid input coefficient"),
        }
    }
}

pub trait Function<T>: fmt::Display 
    where T: Num + Copy {
    fn coefficients<'a>(&'a self) -> &'a Vec<T>;
    fn eval(&self, x: T) -> Result<T, Error> {
        let mut coefficients = self.coefficients().iter();
        let mut sum = match coefficients.next() {
            Some(coefficient) => *coefficient,
            None => return Err(Error::EvaluationError),
        };
        for coefficient in coefficients {
            let product = sum * x;
            sum = *coefficient + product;
        }

        Ok(sum)
    }
}


pub struct Polynomial<T> {
    coefficients: Vec<T>,
}

impl<T: Num + str::FromStr> Polynomial<T> {
    pub fn build(mut args: impl Iterator<Item = String>)
    -> Result<Polynomial<T>, Error> {
        args.next();
    
        let mut coefficients = Vec::<T>::new();
        for arg in args {
            if let Ok(coefficient) = arg.parse::<T>() {
                coefficients.push(coefficient);
            } else {
                return Err(Error::BuildError);
            }
        }

        Ok(Polynomial::<T> {
            coefficients,
        })
    }
    pub fn new(coefficients: Vec<T>) -> Polynomial<T> {
        Polynomial{ coefficients }
    }
}

impl<T> Function<T> for Polynomial<T>
    where T: Num + fmt::Display + fmt::Debug + std::marker::Copy {
    fn coefficients<'a>(&'a self) -> &'a Vec<T> {
        &self.coefficients
    }
}

impl<T> fmt::Display for Polynomial<T>
    where T: Num + fmt::Display + fmt::Debug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.coefficients)
    }
}

// Macro to neatly instanciate a new polynomial
#[macro_export]
macro_rules! polynomial {
    ($($x:expr),+) => {
        {
            let mut coefficients = Vec::new();
            $(
                coefficients.push($x);
            )*
            Polynomial::new(coefficients)
        }
    };
}
