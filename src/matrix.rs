use core::fmt;
use std::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Mul},
};

use anyhow::{anyhow, Result};

use crate::Vector;

pub struct Matrix<T> {
    data: Vec<T>,
    row: usize,
    col: usize,
}

// pub struct MsgInput<T> {
//     idx: usize,
//     row: Vector<T>,
//     col: Vector<T>,
// }

// pub struct MsgOutput<T> {
//     idx: usize,
//     value: T,
// }

fn dot_product<T>(a: Vector<T>, b: Vector<T>) -> Result<T>
where
    T: Copy + Add<Output = T> + Mul<Output = T> + AddAssign + Default,
{
    if a.len() != b.len() {
        return Err(anyhow!("Vector dimensions do not match"));
    }
    let mut sum = T::default();
    for i in 0..a.len() {
        sum += a[i] * b[i];
    }
    Ok(sum)
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
            let row = Vector::new(&a.data[i * a.col..(i + 1) * a.col]);
            let col_data = b.data[j..]
                .iter()
                .step_by(b.col)
                .copied()
                .collect::<Vec<_>>();
            let col = Vector::new(col_data);
            data[i * b.col + j] = dot_product(row, col)?;
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
