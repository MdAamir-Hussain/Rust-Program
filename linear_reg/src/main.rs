extern crate nalgebra as na;

fn linear_regression(x: &na::DMatrix<f64>, y: &na::DVector<f64>) -> na::DVector<f64> {
    (x.transpose() * x).try_inverse().unwrap() * x.transpose() * y
}

fn main() {
    let x = na::DMatrix::from_row_slice(3, 2, &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    let y = na::DVector::from_row_slice(&[3.0, 5.0, 7.0]);
    println!("Linear regression coefficients: {:?}", linear_regression(&x, &y));
}
