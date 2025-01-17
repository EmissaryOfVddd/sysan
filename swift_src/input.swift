//
//  input.swift
//  Definition of multitude of optimal strategies
//
//  Created by Евгений Канашкин on 15.11.2024.
//

import Surge

// Результатами функций данного файла являются матрица, введенная через CLI и ее седловые точки

//MARK: - Ввод матрицы
func input() -> (matrix: Matrix<Double>, saddlePoints: (Vector<Double>, Vector<Double>)) {
        print("Введите размерность матрицы:")
        let dimension: Int = Int(readLine()!)!
        precondition(dimension > 1, "Размерность \(dimension) не имеет смысла")

        print("Теперь введите каждую строку матрицы через \"Enter\", разделяя элементы через пробел:")
        var rawMatrix: [[Double]] = []
        for _ in 0 ..< dimension {
                let rawRow: [String.SubSequence] = readLine()!.split(separator: " ")
                var doubleRow: [Double] = []
                for element in rawRow {
                        doubleRow.append(Double(element)!)
                }
                rawMatrix.append(doubleRow)
        }
        
        let matrix: Matrix<Double> = Matrix(rawMatrix)
        
        return (matrix: matrix, saddlePoints: saddlePoints(from: matrix))
}



//MARK: - Генерация седловых точек в матрице
func saddlePoints(from matrix: Matrix<Double>) -> (Vector<Double>, Vector<Double>) {
        ifdebug(.saddlePoints, .start, "Поиск седловых точек.")
        
        var minsInRows: [Double] = []
        for rows in 0 ..< matrix.rows {
                minsInRows.append(min(Array(matrix[rows])))
        }
        let maxOfMinsIndex = minsInRows.firstIndex(where: { $0 == max(minsInRows) })!
        
        var rawFirstVector: [Double] = []
        for i in 0 ..< matrix.rows {
                i == maxOfMinsIndex ? rawFirstVector.append(1.0) : rawFirstVector.append(0.0)
        }
        
        var maxsInRows: [Double] = []
        for columns in 0 ..< matrix.rows {
                maxsInRows.append(max(Array(transpose(matrix)[columns])))
        }
        let minOfMaxsIndex = maxsInRows.firstIndex(where: { $0 == min(maxsInRows) })!
        
        var rawSecondVector: [Double] = []
        for j in 0 ..< matrix.rows {
                j == minOfMaxsIndex ? rawSecondVector.append(1.0) : rawSecondVector.append(0.0)
        }
        
        ifdebug(.saddlePoints, .stop, "Седловые точки найдены. \(Vector<Double>(rawFirstVector)), \(Vector<Double>(rawSecondVector))")
        return (Vector<Double>(rawFirstVector), Vector<Double>(rawSecondVector))
}
