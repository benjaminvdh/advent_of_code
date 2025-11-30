import Solver

import Data.List

main = solve part1 part2

parse :: String -> ([Int], [Int])
parse = toPairOfLists . toInts
        where toInts = map (map read) . map words . lines
              toPairOfLists = foldr (\xs (accl, accr) -> (head xs:accl, last xs:accr)) ([], [])

part1 input = let (a, b) = parse input
              in sum (map distance (zip (sort a) (sort b)))
                 where distance (a, b) = abs (a - b)

part2 input = let (a, b) = parse input
              in sum (map (countOccurrences a) b)
                 where countOccurrences a n = sum (filter (==n) a)
