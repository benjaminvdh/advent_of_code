import Solver

import Data.List
import Data.Maybe

type Equation = (Int, [Int])

main = solve part1 part2

part1          = partX [(+), (*)]
part2          = partX [(+), (*), numConcat]
partX fs input = let es             = parse input
                     solvable' fs e = solvable fs (fst e) (snd e)
                 in sum $ map fst $ filter (solvable' fs) es

parse :: String -> [Equation]
parse = map parseLine . lines
  where parseLine l = let (v, ss) = splitAt (fromJust $ elemIndex ':' l) l
                          os      = drop 2 ss
                      in (read v, map read $ words os)

solvable :: [(Int -> Int -> Int)] -> Int -> [Int] -> Bool
solvable fs v (a:b:os) = any (\f -> solvable fs v (f a b:os)) fs
solvable _  v (a:_)    = v == a

numConcat :: Int -> Int -> Int
numConcat a b = let a' = show a
                    b' = show b
                in read (a' ++ b')
