import Solver

import Data.List
import qualified Data.Map as M
import Data.Maybe

main = solve part1 part2

part1   = partX  2
part2   = partX 25
partX n = sum . map (complexity n) . lines

complexity n code = read (init code) * minimum (map (snd . lengthAfterN n M.empty) $ getNumPadSequences 'A' code)

type Coord = (Int, Int)
type Map = M.Map (Int, [Char]) Int

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

getNumPadSequences :: Char -> [Char] -> [[Char]]
getNumPadSequences _ []     = [""]
getNumPadSequences p (c:cs) = let (px, py) = coord p
                                  (cx, cy) = coord c
                                  (dx, dy) = (cx - px, cy - py)
                                  hor      = replicate (abs dx) $ if dx > 0 then '>' else '<'
                                  vert     = replicate (abs dy) $ if dy > 0 then 'v' else '^'
                                  ls
                                    | (px + dx, py) == (0, 3) = [vert ++ hor ++ "a"]
                                    | (px, py + dy) == (0, 3) = [hor ++ vert ++ "a"]
                                    | otherwise = [vert ++ hor ++ "a", hor ++ vert ++ "a"]
                              in (++) <$> nub ls <*> getNumPadSequences c cs

getDPadSequences :: Char -> [Char] -> [[Char]]
getDPadSequences _ []     = [""]
getDPadSequences p (c:cs) = let (px, py) = coord p
                                (cx, cy) = coord c
                                (dx, dy) = (cx - px, cy - py)
                                hor      = replicate (abs dx) $ if dx > 0 then '>' else '<'
                                vert     = replicate (abs dy) $ if dy > 0 then 'v' else '^'
                                ls
                                  | (px + dx, py) == (0, 0) = [vert ++ hor ++ "a"]
                                  | (px, py + dy) == (0, 0) = [hor ++ vert ++ "a"]
                                  | otherwise = [vert ++ hor ++ "a", hor ++ vert ++ "a"]
                            in (++) <$> nub ls <*> getDPadSequences c cs

lengthAfterN :: Int -> Map -> [Char] -> (Map, Int)
lengthAfterN _ m []   = (m, 0)
lengthAfterN n m code = let index       = fromJust $ elemIndex 'a' code
                            (m', l)     = subseqAfterN n m $ take (index + 1) code
                            (m'', rest) = lengthAfterN n m' $ drop (index + 1) code
                        in (m'', l + rest)

subseqAfterN :: Int -> Map -> [Char] -> (Map, Int)
subseqAfterN 0 m subseq = (m, length subseq)
subseqAfterN n m subseq = case M.lookup (n, subseq) m of
                               Just l  -> (m, l)
                               Nothing -> let (m', l) = foldr (\alt (m, l) -> let (m', l') = lengthAfterN (n - 1) m alt in (m', min l l')) (m, maxBound) $ getDPadSequences 'a' subseq
                                          in (M.insert (n, subseq) l m', l)
