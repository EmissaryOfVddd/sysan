//
//  output.swift
//  Definition of multitude of optimal strategies
//
//  Created by Евгений Канашкин on 15.11.2024.
//

import Surge

// Результатом функции данного файла является вывод стратегийе

//MARK: - Вывод результатов
func printResult(_ resultsXY: [(X: [Double], Y: [Double])], _ saddlePoints: (Vector<Double>, Vector<Double>)) -> Void {
        print()

        for res in resultsXY {
                print("Ɑ(\(saddlePoints.0)) + (1 - Ɑ)(\(res.X)), 0 ≤ Ɑ ≤ 1")
                print("β(\(saddlePoints.1)) + (1 - β)(\(res.Y)), 0 ≤ β ≤ 1")
                print()
        }
}
