//
//  check_x_y.swift
//  Definition of multitude of optimal strategies
//
//  Created by Евгений Канашкин on 15.11.2024.
//

import Surge

// Пупупу

//MARK: - Чекает X и Y на решение игры
func checkXY(_ matrix: Matrix<Double>, _ x: [Double], _ y: [Double]) -> Bool {
        
        ifdebug(.checkXY, .start, "Проверка на совпадение X и Y стратегий.")
        
        
        var rowsSums: [Double] = []
        for row in 0 ..< matrix.rows {
                
                let tempRow: [Double] = Array(matrix[row])
                
                var tempResRow: [Double] = []
                for col in 0 ..< matrix.columns {
                        tempResRow.append( tempRow[col] * y[col] )
                }
                
                rowsSums.append(sum(tempResRow))
        }
        
        let maxY: Double = max(rowsSums)
        
        let transpMatrix: Matrix<Double> = transpose(matrix)
        var colsSums: [Double] = []
        for row in 0 ..< transpMatrix.rows {
                
                let tempRow: [Double] = Array(transpMatrix[row])

                var tempResRow: [Double] = []
                for col in 0 ..< transpMatrix.columns {
                        tempResRow.append( tempRow[col] * x[col])
                }
                
                colsSums.append(sum(tempResRow))
        }
        let minX: Double = min(colsSums)
        
        ifdebug(.checkXY, .stop, "Проверка завершена. maxY:\(maxY), minX:\(minX)")
        
        return abs(maxY - minX) < 1e-3
}
