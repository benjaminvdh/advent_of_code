import Solver

import Data.List

main = solve part1 part2

part1 = foldr (\code acc -> acc + complexity code) 0 . lines
part2 _ = "N/A"

type Coord = (Int, Int)

coord :: Char -> Coord
coord '7' = (0, 0)
coord '8' = (1, 0)
coord '9' = (2, 0)
coord '4' = (0, 1)
coord '5' = (1, 1)
coord '6' = (2, 1)
coord '1' = (0, 2)
coord '2' = (1, 2)
coord '3' = (2, 2)
coord '0' = (1, 3)
coord 'A' = (2, 3)
coord '^' = (1, 0)
coord 'a' = (2, 0)
coord '<' = (0, 1)
coord 'v' = (1, 1)
coord '>' = (2, 1)

complexity :: String -> Int
complexity code = (minimum . sort . map length $ return code >>= getNumPadSequence 'A' >>= getDPadSequence 'a' >>= getDPadSequence 'a') * (read $ init code)

getNumPadSequence :: Char -> [Char] -> [[Char]]
getNumPadSequence p (c:cs) = let (px, py) = coord p
                                 (cx, cy) = coord c
                                 (dx, dy) = (cx - px, cy - py)
                                 hor      = replicate (abs dx) $ if dx > 0 then '>' else '<'
                                 vert     = replicate (abs dy) $ if dy > 0 then 'v' else '^'
                                 ls       = if (px + dx, py) == (0, 3)
                                            then [vert ++ hor ++ "a"]
                                            else if (px, py + dy) == (0, 3)
                                                 then [hor ++ vert ++ "a"]
                                                 else [vert ++ hor ++ "a", hor ++ vert ++ "a"]
                             in (++) <$> nub ls <*> getNumPadSequence c cs
getNumPadSequence _ _ = [""]

getDPadSequence :: Char -> [Char] -> [[Char]]
getDPadSequence p (c:cs) = let (px, py) = coord p
                               (cx, cy) = coord c
                               (dx, dy) = (cx - px, cy - py)
                               hor      = replicate (abs dx) $ if dx > 0 then '>' else '<'
                               vert     = replicate (abs dy) $ if dy > 0 then 'v' else '^'
                               ls       = if (px + dx, py) == (0, 0)
                                          then [vert ++ hor ++ "a"]
                                          else if (px, py + dy) == (0, 0)
                                               then [hor ++ vert ++ "a"]
                                               else [vert ++ hor ++ "a", hor ++ vert ++ "a"]
                           in (++) <$> nub ls <*> getDPadSequence c cs
getDPadSequence _ _ = [""]
