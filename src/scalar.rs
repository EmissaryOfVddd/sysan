use nalgebra::DMatrix;

use crate::{debug_info, Functions, IfType};

pub fn get_scalar_v(matrix: &DMatrix<f64>) -> Option<f64> {
    debug_info(Functions::Scalar, IfType::Start, "Генерация скаляра матрицы.");

    // Сумма по столбцам
    let mut col_sum: Vec<f64> = vec![0.0; matrix.ncols()]; // Инициализируем вектор для суммы столбцов

    for row in 0..matrix.nrows() {
        for col in 0..matrix.ncols() {
            col_sum[col] += matrix[(row, col)];
        }
    }

    let total_sum = col_sum.iter().sum::<f64>();
    debug_info(Functions::Scalar, IfType::Info, &format!("{:?} -> {}", col_sum, total_sum));

    if total_sum.abs() < 1e-10 {
        debug_info(Functions::Scalar, IfType::Warn, "Неудачная генерация скаляра. v == None.");
        return None; // Возвращаем None, если сумма близка к нулю
    }

    let result = 1.0 / total_sum;

    debug_info(Functions::Scalar, IfType::Stop, &format!("Генерация скаляра завершена. {}.", result));
    Some(result)
}