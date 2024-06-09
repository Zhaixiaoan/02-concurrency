use core::fmt;
use std::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Mul},
};

use anyhow::{anyhow, Result};

pub struct Matrix<T> {
    data: Vec<T>,
    row: usize,
    col: usize,
}

pub fn multiply<T>(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>>
where
    T: Copy + Add<Output = T> + Mul<Output = T> + AddAssign + Default,
{
    if a.col != b.row {
        return Err(anyhow!("Matrix dimensions do not match"));
    }

    let mut data = vec![T::default(); a.row * b.col];
    for i in 0..a.row {
        for j in 0..b.col {
            for k in 0..a.col {
                data[i * b.col + j] += a.data[i * a.col + k] * b.data[k * b.col + j];
            }
        }
    }
    Ok(Matrix {
        data,
        row: a.row,
        col: b.col,
    })
}
impl<T: Debug> Matrix<T> {
    pub fn new(data: impl Into<Vec<T>>, row: usize, col: usize) -> Self {
        Matrix {
            data: data.into(),
            row,
            col,
        }
    }
}

impl<T> fmt::Display for Matrix<T>
where
    T: Display,
{
    //display a 2x3 as {{1 2 3}, {4 5 6}},3x2 as {{1 2}, {3 4}, {5 6}}
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{")?;
        for i in 0..self.row {
            for j in 0..self.col {
                write!(f, "{}", self.data[i * self.col + j])?;
                if j != self.col - 1 {
                    write!(f, " ")?;
                }
            }
            if i != self.row - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, "}}")?;
        Ok(())
    }
}

impl<T> Debug for Matrix<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Matrix(row={}, col={}, {})", self.row, self.col, self)
    }
}

#[test]
fn test_multiply() -> Result<()> {
    let a = Matrix::new(vec![1, 2, 3, 4], 2, 2);
    let b = Matrix::new(vec![1, 2, 3, 4], 2, 2);
    let c = multiply(&a, &b)?;
    assert_eq!(format!("{:?}", c), "Matrix(row=2, col=2, {7 10, 15 22})");
    Ok(())
}
