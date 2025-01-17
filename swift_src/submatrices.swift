//
//  submatrices.swift
//  Definition of multitude of optimal strategies
//
//  Created by Евгений Канашкин on 15.11.2024.
//

import Surge

// Результатом данных функция является вывод всех подходящих подматриц исходной матрицы

//MARK: - Генерация всех подматриц размерностью `size` матрицы
func extractSubmatrices(from matrix: Matrix<Double>, withSize size: Int) -> (matrices: [Matrix<Double>], combinations: [(rows: [Int], columns: [Int])]) {
        ifdebug(.submatrix, .start, "Генерация подматриц размерностью \(size) из матрицы размером \(matrix.rows).")
        
        var submatrices: [Matrix<Double>] = [] // вывод
        var resultCombinations: [(rows: [Int], columns: [Int])] = []
        
        let combinations = combinations(from: matrix.rows, withSize: size) // буфер
        
        
        for rowIndices in combinations {
//                ifdebug(.submatrix, .info, "Комбинация строк: \(rowIndices)")
                
                for colIndices in combinations {
//                        ifdebug(.submatrix, .info, "Комбинация колонн: \(colIndices)")
                        
                        var submatrix: [[Double]] = []
                        
                        for rowIndex in rowIndices {
                                
                                var subrow: [Double] = []
                                for colIndex in colIndices {
                                        
                                        subrow.append(Array(matrix[rowIndex])[colIndex])
                                }
                                submatrix.append(subrow)
                        }
                        
//                        ifdebug(.submatrix, .info, "Подматрица по комбинации: \(submatrix)")
                        
                        if !submatrices.contains(Matrix(submatrix)) {
                                if det(Matrix(submatrix)) != nil {
                                        submatrices.append(Matrix(submatrix))

                                        let resultCombination: (rows: [Int], columns: [Int]) = (rowIndices, colIndices)
                                        resultCombinations.append(resultCombination)
                                }
                        }
                }
        }
        ifdebug(.submatrix, .stop, "Генерация подматриц завершена.")
        return (submatrices, resultCombinations)
}



//MARK: - Генерация всех возможных комбинаций из `r` элементов для `n` элементов
func combinations(from n: Int, withSize r: Int) -> [[Int]] {
        ifdebug(.combinations, .start, "Генерация комбинаций из \(n) по \(r) элементов.")
        var result: [[Int]] = [] /// вывод
        
        var combination: [Int] = Array(0 ..< r) // буфер
        
        while true {
                result.append(combination)
                
                var i = r - 1
                while i >= 0 && combination[i] == n - r + i {
                        i -= 1
                }
                
                if i < 0 {
                        break
                }
                
                combination[i] += 1
                for j in i+1..<r {
                        combination[j] = combination[j-1] + 1
                }
        }
        
        ifdebug(.combinations, .stop, "Генерация комбинаций завершена: \(result)")
        return result
}
