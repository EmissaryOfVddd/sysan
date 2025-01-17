//
//  x_y.swift
//  Definition of multitude of optimal strategies
//
//  Created by Евгений Канашкин on 15.11.2024.
//

import Surge

// Результатом данных функций являются все X и Y стратегии

//MARK: - Генерация Y (не инвертирует матрицу)
func getY(_ scalar: Double, _ matrix: Matrix<Double>, _ initialSize: Int, _ columns: [Int]) -> [Double] {
        ifdebug(.getY, .start, "Генерация Y")
        
        var vector: [Double] = []
        for row in 0 ..< matrix.rows {
                vector.append(sum(Array(matrix[row])))
        }
        
        for element in 0 ..< vector.count {
                vector[element] *= scalar
        }
        
        var wholeColumns: [Int] = []
        for i in 0 ..< initialSize {
                wholeColumns.append(i)
        }
        
        var result: [Double] = []
        
        var jindex: Int = 0
        for element in wholeColumns {
                if columns.contains(element) {
                        result.append(vector[jindex])
                        jindex += 1
                } else {
                        result.append(0.0)
                }
        }
        
        ifdebug(.getY, .stop, "Генерация Y завершена. \(result) -> \(sum(result))")
        return result
}

//MARK: - Генерация X (не инвертирует и не транспонирует матрицу)
func getX(_ scalar: Double, _ matrix: Matrix<Double>, _ initialSize: Int, _ rows: [Int]) -> [Double] {
        ifdebug(.getY, .start, "Генерация X")
        
        var vector: [Double] = []
        for row in 0 ..< matrix.rows {
                vector.append(sum(Array(matrix[row])))
        }
        
        for element in 0 ..< vector.count {
                vector[element] *= scalar
        }
        
        var wholeRows: [Int] = []
        for i in 0 ..< initialSize {
                wholeRows.append(i)
        }
        
        var result: [Double] = []
        
        var jindex: Int = 0
        for element in wholeRows {
                if rows.contains(element) {
                        result.append(vector[jindex])
                        jindex += 1
                } else {
                        result.append(0.0)
                }
        }
        
        ifdebug(.getY, .stop, "Генерация X завершена. \(result) -> \(sum(result))")
        return result
}
