use nalgebra::{DMatrix, DVector};
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

// Функция для поиска седловых точек
fn saddle_points(matrix: &DMatrix<f64>) -> (Vec<f64>, Vec<f64>) {
    debug_info(Functions::SaddlePoints, IfType::Start, "Поиск седловых точек");

    // Находим минимальные значения в строках
    let mins_in_rows: Vec<f64> = matrix.row_iter()
        .map(|row| row.min())
        .collect();

    // Находим индекс максимального из минимальных значений
    let max_of_mins = mins_in_rows.iter().map(|&n| n).reduce(f64::min).unwrap();
    let max_of_mins_index = mins_in_rows.iter().position(|&x| x == max_of_mins).unwrap();

    // Создаем первый вектор
    let mut raw_first_vector = vec![0.; matrix.nrows()];
    raw_first_vector[max_of_mins_index] = 1.0;

    // Транспонируем матрицу
    let transposed_matrix = matrix.transpose();

    // Находим максимальные значения в столбцах (строках транспонированной матрицы)
    let maxs_in_columns: Vec<f64> = transposed_matrix.row_iter()
    .map(|row| row.max())
        .collect();

    // Находим индекс минимального из максимальных значений
    let min_of_maxs = maxs_in_columns.iter().map(|&n| n).reduce(f64::max).unwrap();
    let min_of_maxs_index = maxs_in_columns.iter().position(|&x| x == min_of_maxs).unwrap();

    // Создаем второй вектор
    let mut raw_second_vector = vec![0.; matrix.nrows()];
    raw_second_vector[min_of_maxs_index] = 1.0;

    debug_info(Functions::SaddlePoints, IfType::Stop, &format!("Седловые точки найдены: {raw_first_vector:?}, {raw_second_vector:?}"));
    (raw_first_vector, raw_second_vector)
}


fn main() {
    let m = DMatrix::from_columns(&[
        DVector::from_vec(vec![3., 5., 3.]),
        DVector::from_vec(vec![4., -3., 2.]),
        DVector::from_vec(vec![3., 2., 3.]),
    ]);

    // dbg!(get_results(&m));
    let saddle_points = saddle_points(&m);
    let results = get_results(&m);

    for res in results {
        println!("Ɑ({:.2?}) + (1 - Ɑ)({:.2?}), 0 ≤ Ɑ ≤ 1", saddle_points.0, res.0);
        println!("β({:.2?}) + (1 - β)({:.2?}), 0 ≤ Ɑ ≤ 1", saddle_points.1, res.1);
    }
}

fn get_results(matrix: &DMatrix<f64>) -> Vec<(Vec<f64>, Vec<f64>)> {
    let mut results = Vec::new();

    for size in 1..=matrix.nrows() {
        let (matrices, combinations) = extract_submatrices(&matrix, size);
        for (index, _) in matrices.iter().enumerate() {
            let submatrix = &matrices[index];
            let combination = &combinations[index];

            if debug_info(Functions::Main, IfType::Info, "Submatrix:") {
                println!("{:.2}", submatrix);
            }
            debug_info(Functions::Main, IfType::Info, &format!("Combination: {:?} \n", combination));

            let inv_submatrix = submatrix.clone().try_inverse().expect("Cannot invert submatrix");

            if debug_info(Functions::Main, IfType::Info, "Inverted Submatrix:") {
                println!("{:.2}", inv_submatrix);
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
            debug_info(Functions::Main, IfType::EndIteration, "Добавляем результаты");
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
