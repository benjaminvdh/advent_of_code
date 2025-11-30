import Solver

import Data.Char
import Data.List

main = solve part1 part2

part1 = multAll
part2 = multAllMaybe

multAll ('m':xs) = parseMul xs + multAll xs
multAll (_:xs) = multAll xs
multAll [] = 0

multAllMaybe ('m':xs) = parseMul xs + multAllMaybe xs
multAllMaybe l@(_:xs) = if "don't()" `isPrefixOf` l then skipInput l else multAllMaybe xs
multAllMaybe [] = 0

skipInput l@(x:xs) = if "do()" `isPrefixOf` l then multAllMaybe xs else skipInput xs
skipInput [] = 0

parseMul :: String -> Int
parseMul xs = if "ul(" `isPrefixOf` xs then
                  let a = takeWhile isDigit (drop 3 xs)
                      b = head (drop (3 + length a) xs)
                      c = takeWhile isDigit (drop (4 + length a) xs)
                      d = drop (4 + length a + length c) xs
                  in if b == ',' && head d == ')' then read a * read c else 0
              else 0
