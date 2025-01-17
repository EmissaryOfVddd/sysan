use nalgebra::DMatrix;

use crate::{debug_info, Functions, IfType};

pub fn extract_submatrices(
    matrix: &DMatrix<f64>,
    size: usize,
) -> (Vec<DMatrix<f64>>, Vec<(Vec<usize>, Vec<usize>)>) {
    debug_info(Functions::Submatrix, IfType::Start, &format!("Генерация подматриц размерностью {} из матрицы размером {}.", size, matrix.nrows()));
    
    let mut submatrices: Vec<DMatrix<f64>> = vec![]; // вывод
    let mut result_combinations: Vec<(Vec<usize>, Vec<usize>)> = vec![];
    
    let combinations = generate_combinations(matrix.nrows(), size); // буфер

    for row_indices in &combinations {
        for col_indices in &combinations {
            let mut submatrix: Vec<Vec<f64>> = vec![];

            for &row_index in row_indices {
                let mut subrow: Vec<f64> = vec![];
                for &col_index in col_indices {
                    subrow.push(matrix[(row_index, col_index)]);
                }
                submatrix.push(subrow);
            }

            let submatrix_matrix = DMatrix::from_vec(submatrix.len(), submatrix[0].len(), submatrix.iter().flat_map(|v| v.iter()).copied().collect());

            // Поскольку Rust не поддерживает проверку на содержание матриц напрямую, используйте сравнение
            if !submatrices.contains(&submatrix_matrix) {
                if submatrix_matrix.determinant() != 0.0 {
                    submatrices.push(submatrix_matrix);

                    let result_combination = (row_indices.clone(), col_indices.clone());
                    result_combinations.push(result_combination);
                }
            }
        }
    }
    debug_info(Functions::Submatrix, IfType::Stop, "Генерация подматриц завершена.");
    (submatrices, result_combinations)
}

pub fn generate_combinations(n: usize, r: usize) -> Vec<Vec<usize>> {
    debug_info(Functions::Combinations, IfType::Start, &format!("Генерация комбинаций из {} по {} элементов.", n, r));
    let mut result: Vec<Vec<usize>> = vec![]; // вывод

    let mut combination: Vec<usize> = (0..r).collect(); // буфер

    while combination[r - 1] < n {
        result.push(combination.clone());

        let mut i = r;
        while i > 0 && combination[i - 1] == n - r + (i - 1) {
            i -= 1;
        }

        if i == 0 {
            break;
        }

        combination[i - 1] += 1;
        for j in i..r {
            combination[j] = combination[j - 1] + 1;
        }
    }

    debug_info(Functions::Combinations, IfType::Stop, &format!("Генерация комбинаций завершена: {:?}", result));
    result
}