//
//  scalar.swift
//  Definition of multitude of optimal strategies
//
//  Created by Евгений Канашкин on 15.11.2024.
//

import Surge

// Результатом данной функции является скаляр

//MARK: - Генерация v (не инвертирует матрицу)
func getScalarV(_ matrix: Matrix<Double>) -> Double? {
        
        ifdebug(.scalar, .start, "Генерация скаляра матрицы.")
        
        var colSum: [Double] = []
        for row in 0 ..< matrix.rows {
                colSum.append(sum(Array(transpose(matrix)[row])))
        }
        
        ifdebug(.scalar, .info, "\(colSum) -> \(sum(colSum)).")
        
        if abs(sum(colSum)) < 1e-10 {
                ifdebug(.scalar, .warn, "Неудачная генерация скаляра. v == nil.")
                return nil
        }
        
        let result = 1 / sum(colSum)
        
        ifdebug(.scalar, .stop, "Генерация скаляра завершена. \(result).")
        return result
}
