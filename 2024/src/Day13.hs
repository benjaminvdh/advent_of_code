import Solver

import Data.List

main = solve part1 part2

type Coord = (Integer, Integer)
data Machine = Machine { getA :: Coord, getB :: Coord, prize :: Coord } deriving Show

part1 = sum . map getTokens . parse
part2 = sum . map getManyTokens . map explodePrize . parse

parse :: String -> [Machine]
parse input@(_:_) = let (a, b) = splitAt (findBlankLine input) input
                    in parseMachine a:parse (drop 2 b)
parse [] = []

findBlankLine :: String -> Int
findBlankLine (a:b:rest) = if a == '\n' && b == '\n'
                           then 0
                           else 1 + findBlankLine (b:rest)
findBlankLine _ = 1

parseMachine :: String -> Machine
parseMachine input = let ls = lines input
                     in Machine { getA = parseCoords (ls !! 0), getB = parseCoords (ls !! 1), prize = parseCoords (ls !! 2) }
  where parseCoords input     = (getNumWithPrefix 'X' input, getNumWithPrefix 'Y' input)
        getNumWithPrefix c cs = read $ drop 2 (takeWhile (/=',') (dropWhile (/=c) cs))

getTokens :: Machine -> Integer
getTokens m = let buttons = [(a, b) | a <- [0..100], b <- [0..100]]
              in case find (\(a, b) -> (a *: getA m) +: (b *: getB m) == prize m) buttons of
                      Just (a, b) -> 3 * a + b
                      Nothing     -> 0

getManyTokens :: Machine -> Integer
getManyTokens m = let ma = getA m
                      mb = getB m
                      mp = prize m
                      ax :: Double = fromInteger (fst ma)
                      ay :: Double = fromInteger (snd ma)
                      bx :: Double = fromInteger (fst mb)
                      by :: Double = fromInteger (snd mb)
                      px :: Double = fromInteger (fst mp)
                      py :: Double = fromInteger (snd mp)
                      b :: Double = (py - px * ay / ax) / (by - bx * ay / ax)
                      a :: Double = (px - b * bx) / ax
                      ai = round a
                      bi = round b
                      in if (ai *: getA m) +: (bi *: getB m) == prize m
                         then 3 * ai + bi
                         else 0

explodePrize :: Machine -> Machine
explodePrize m = Machine { getA = getA m, getB = getB m, prize = (10000000000000, 10000000000000) +: prize m }

(+:) :: Coord -> Coord -> Coord
(x, y) +: (x', y') = (x + x', y + y')

(*:) :: Integer -> Coord -> Coord
a *: (x, y) = (a * x, a * y)
