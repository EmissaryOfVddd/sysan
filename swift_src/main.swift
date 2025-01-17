//
//  main.swift
//  Definition of multitude of optimal strategies
//
//  Created by Евгений Канашкин on 13.11.2024.
//

import Foundation
import Surge

let debug: Bool = true
enum functions {
        case main
        case submatrix
        case combinations
        case saddlePoints
        case scalar
        case getY
        case getX
        case checkXY
}
enum iftype {
        case start
        case stop
        case info
        case warn
        case endIteration
}

let matrixAndPoints: (matrix: Matrix<Double>, saddlePoints: (Vector<Double>, Vector<Double>)) = input()

let matrix: Matrix<Double> = matrixAndPoints.matrix
let saddlePoints: (Vector<Double>, Vector<Double>) = matrixAndPoints.saddlePoints

var resultsXY: [(X: [Double], Y: [Double])] = getResults(from: matrix)

printResult(resultsXY, saddlePoints)

//MARK: - 👇 Тут функции страшные внизу 👇


//MARK: - Основная функция
func getResults(from matrix: Matrix<Double>) -> [(X: [Double], Y: [Double])] {
        
        var results: [(X: [Double], Y: [Double])] = []
        
        for size in 2 ... matrix.rows {
                let submatrices = extractSubmatrices(from: matrix, withSize: size)
                for (index, _) in submatrices.matrices.enumerated() {
                        
                        let submatrix: Matrix<Double> = submatrices.matrices[index]
                        let combination: (rows: [Int], columns: [Int]) = submatrices.combinations[index]
                        
                        ifdebug(.main, .info, "Submatrix:")
                        if debug { print(submatrix) }
                        ifdebug(.main, .info, "Combination: \(combination) \n")
                        
                        let invSubmatrix: Matrix<Double> = inv(submatrix)
                        ifdebug(.main, .info, "Inverted Submatrix:")
                        if debug { print(invSubmatrix) }
                        
                        var v: Double = 0.0
                        
                        if let tempV: Double = getScalarV(invSubmatrix) {
                                v = tempV
                                
                                ifdebug(.main, .info, "V: \(v)")
                                
                        } else {
                                ifdebug(.main, .warn, "Невозможно получить V! Пропуск итерации!")
                                ifdebug(.main, .endIteration)
                                continue
                        }
                        
                        let Y: [Double] = getY(v, invSubmatrix, matrix.rows, combination.columns)
                        ifdebug(.main, .info, "Y: \(Y)")
                        
                        let X: [Double] = getX(v, inv(transpose(submatrix)), matrix.columns, combination.rows)
                        ifdebug(.main, .info, "X: \(X)")
                        
                        if !checkXY(matrix, X, Y) {
                                ifdebug(.main, .warn, "X и Y не являются решением! Пропуск итерации!")
                                ifdebug(.main, .endIteration)
                                continue
                        }
                        
                        let result: (X: [Double], Y: [Double]) = (X, Y)
                        results.append(result)
                
                        ifdebug(.main, .endIteration)
                }
        }
        
        return results
}


//MARK: - Дебожб 🤩
func ifdebug(_ function: functions, _ messageType: iftype, _ message: String = "", _ terminator: String = "\n") {
        if debug {
                
                switch function {
                case .checkXY:
                        print("\t\t\t\t\t\t\t", terminator: "")
                        print("[CHXY]", terminator: "")
                        
                case .getX:
                        print("\t\t\t\t\t\t", terminator: "")
                        print("[GETX]", terminator: "")
                        
                case .getY:
                        print("\t\t\t\t\t", terminator: "")
                        print("[GETY]", terminator: "")
                        
                case .scalar:
                        print("\t\t\t\t", terminator: "")
                        print("[SCAL]", terminator: "")
                        
                case .saddlePoints:
                        print("\t\t\t", terminator: "")
                        print("[SADD]", terminator: "")
                        
                case .combinations:
                        print("\t\t", terminator: "")
                        print("[COMB]", terminator: "")
                        
                case .submatrix:
                        print("\t", terminator: "")
                        print("[SUBM]", terminator: "")
                        
                case .main:
                        print("", terminator: "")
                        print("[MAIN]", terminator: "")
                }
                
                switch messageType {
                case .endIteration:
                        print()
                        print("—————————————————————————————————————————————————————————————————")
                        print()
                        _ = readLine()
                case .start:
                        print("[STAR]", terminator: "")
                        
                case .stop:
                        print("[STOP]", terminator: "")
                        
                case .info:
                        print("[INFO]", terminator: "")
                        
                case .warn:
                        print("[WARN]", terminator: "")
                }
                
                print(message, terminator: terminator)
        }
}
