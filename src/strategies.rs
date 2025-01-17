use nalgebra::DMatrix;

use crate::{debug_info, Functions, IfType};

pub fn get_y(scalar: f64, matrix: &DMatrix<f64>, initial_size: usize, columns: &[usize]) -> Vec<f64> {
    debug_info(Functions::GetY, IfType::Start, "Генерация Y");
    
    let mut vector: Vec<f64> = Vec::with_capacity(matrix.nrows());

    // Суммируем по строкам
    for row in 0..matrix.nrows() {
        vector.push(matrix.row(row).sum());
    }

    // Умножаем на скаляр
    vector.iter_mut().for_each(|x| *x *= scalar);
    
    let mut result: Vec<f64> = vec![0.0; initial_size];
    let mut jindex: usize = 0;

    for i in 0..initial_size {
        if columns.contains(&(i as usize)) {
            result[i] = vector[jindex];
            jindex += 1;
        }
    }

    debug_info(Functions::GetY, IfType::Stop, &format!("Генерация Y завершена. {:?} -> {}", result, result.iter().sum::<f64>()));
    
    result
}

pub fn get_x(scalar: f64, matrix: &DMatrix<f64>, initial_size: usize, rows: &[usize]) -> Vec<f64> {
    debug_info(Functions::GetX, IfType::Start, "Генерация X");

    let mut vector: Vec<f64> = Vec::with_capacity(matrix.nrows());

    // Суммируем по строкам
    for row in 0..matrix.nrows() {
        vector.push(matrix.row(row).sum());
    }

    // Умножаем на скаляр
    vector.iter_mut().for_each(|x| *x *= scalar);
    
    let mut result: Vec<f64> = vec![0.0; initial_size];
    let mut jindex: usize = 0;

    for i in 0..initial_size {
        if rows.contains(&(i as usize)) {
            result[i] = vector[jindex];
            jindex += 1;
        }
    }

    debug_info(Functions::GetX, IfType::Stop, &format!("Генерация X завершена. {:?} -> {}", result, result.iter().sum::<f64>()));
    
    result
}

pub fn check_xy(matrix: &DMatrix<f64>, x: &[f64], y: &[f64]) -> bool {
    debug_info(Functions::CheckXY, IfType::Start, "Проверка на совпадение X и Y стратегий.");

    // Суммы по строкам
    let mut rows_sums: Vec<f64> = Vec::with_capacity(matrix.nrows());
    for row in 0..matrix.nrows() {
        let temp_row: Vec<f64> = matrix.row(row).iter()
        .map(|val| *val)
        .collect(); // Получаем строку в виде вектора
        let temp_res_row: Vec<f64> = temp_row.iter()
            .enumerate()
            .map(|(col, &val)| val * y[col])
            .collect();
        rows_sums.push(temp_res_row.iter().sum::<f64>());
    }

    let max_y = *rows_sums.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();

    // Транспонирование матрицы
    let transp_matrix = matrix.transpose();

    // Суммы по столбцам
    let mut cols_sums: Vec<f64> = Vec::with_capacity(transp_matrix.nrows());
    for row in 0..transp_matrix.nrows() {
        let temp_row: Vec<f64> = transp_matrix.row(row).iter()
        .map(|val| *val)
        .collect(); 
        
        // Получаем строку в виде вектора
        let temp_res_row: Vec<f64> = temp_row.iter()
            .enumerate()
            .map(|(col, &val)| val * x[col])
            .collect();
        cols_sums.push(temp_res_row.iter().sum::<f64>());
    }

    let min_x = *cols_sums.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();

    debug_info(Functions::CheckXY, IfType::Stop, &format!("Проверка завершена. maxY: {}, minX: {}", max_y, min_x));

    (max_y - min_x).abs() < 1e-3
}