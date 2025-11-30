import Solver

import Data.List

main = solve part1 part2

parse :: String -> [[Int]]
parse contents = map (map read) (map words (lines contents))

part1 input = length (filter isSafe (parse input))
part2 input = length (filter isSafeWithDampener (parse input))

toPairs :: [Int] -> [(Int, Int)]
toPairs (xa:xb:xs) = (xa, xb):(toPairs (xb:xs))
toPairs _ = []

isSafe report = all decreasingSafe pairs || all increasingSafe pairs
                where pairs = toPairs report

decreasingSafe (a, b) = 1 <= diff && diff <= 3
                        where diff = a - b

increasingSafe (a, b) = decreasingSafe (b, a)

isSafeWithDampener report = any isSafe (subreports report)
                            where subreports report = map (subreport report) [0..length report]
                                  subreport report index = take index report ++ drop (index + 1) report
