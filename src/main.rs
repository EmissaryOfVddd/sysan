use nalgebra::{DMatrix, DVector, Matrix3, Vector3};
use scalar::get_scalar_v;
use strategies::{check_xy, get_x, get_y};
use submatrices::extract_submatrices;

mod constants;
mod scalar;
mod solution;
mod strategies;
mod submatrices;

const DEBUG: bool = true;

#[derive(Debug)]
enum Functions {
    Main,
    Submatrix,
    Combinations,
    SaddlePoints,
    Scalar,
    GetY,
    GetX,
    CheckXY,
}

#[derive(Debug)]
enum IfType {
    Start,
    Stop,
    Info,
    Warn,
    EndIteration,
}

fn main() {
    let m = DMatrix::from_columns(&[
        DVector::from_vec(vec![3., 5., 3.]),
        DVector::from_vec(vec![4., -3., 2.]),
        DVector::from_vec(vec![3., 2., 3.]),
    ]);

    dbg!(get_results(&m));
}

fn get_results(matrix: &DMatrix<f64>) -> Vec<(Vec<f64>, Vec<f64>)> {
    let mut results = Vec::new();

    for size in 1..=matrix.nrows() {
        let (matrices, combinations) = extract_submatrices(&matrix, size);
        for (index, _) in matrices.iter().enumerate() {
            let submatrix = &matrices[index];
            let combination = &combinations[index];

            if debug_info(Functions::Main, IfType::Info, "Submatrix:") {
                println!("{:?}", submatrix);
            }
            debug_info(Functions::Main, IfType::Info, &format!("Combination: {:?} \n", combination));

            let inv_submatrix = submatrix.clone().try_inverse().expect("Cannot invert submatrix");

            if debug_info(Functions::Main, IfType::Info, "Inverted Submatrix:") {
                println!("{:?}", inv_submatrix);
            }

            let v;

            if let Some(temp_v) = get_scalar_v(&inv_submatrix) {
                v = temp_v;
                debug_info(Functions::Main, IfType::Info, &format!("V: {}", v));
            } else {
                debug_info(Functions::Main, IfType::Warn, "Cannot obtain V! Skipping iteration!");
                debug_info(Functions::Main, IfType::EndIteration, "");
                continue;
            }

            let y: Vec<f64> = get_y(v, &inv_submatrix, matrix.nrows(), &combination.0);
            debug_info(Functions::Main, IfType::Info, &format!("Y: {:?}", y));

            let x: Vec<f64> = get_x(v, &inv_submatrix.transpose(), matrix.ncols(), &combination.1);
            debug_info(Functions::Main, IfType::Info, &format!("X: {:?}", x));

            if !check_xy(matrix, &x, &y) {
                debug_info(Functions::Main, IfType::Warn, "X и Y не являются решением! Пропуск итерации!");
                debug_info(Functions::Main, IfType::EndIteration, "");
                continue;
            }

            results.push((x, y));
            debug_info(Functions::Main, IfType::EndIteration, "");
        }
    }

    results
}


fn debug_info(function: Functions, if_type: IfType, message: &str) -> bool {
    if !DEBUG { return false }

    let f = match function {
        Functions::Main => "MAIN",
        Functions::Submatrix => "SUBM",
        Functions::Combinations => "COMB",
        Functions::SaddlePoints => "SADD",
        Functions::Scalar => "SCAL",
        Functions::GetY => "GETY",
        Functions::GetX => "GETX",
        Functions::CheckXY => "CHCK",
    };

    let i = match if_type {
        IfType::Start => "STRT",
        IfType::Stop => "STOP",
        IfType::Info => "INFO",
        IfType::Warn => "WARN",
        IfType::EndIteration => "ENDI",
    };

    println!("[{f}] - [{i}]: {message}");
    true
}
