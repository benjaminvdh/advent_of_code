import Solver

import Data.List
import qualified Data.Map.Strict as M

main = solve part1 part2

part1 input = let (map, size) = parse input
              in length $ nub $ filter (inside size) $ concat $ snd $ unzip $ M.toList $ fmap antinodes map
part2 input = let (map, size) = parse input
              in length $ nub $ filter (inside size) $ concat $ snd $ unzip $ M.toList $ fmap (moreAntinodes size) map

type Coord = (Int, Int)
type Map = M.Map Char [Coord]

parse :: String -> (Map, (Int, Int))
parse input = let m = foldr (\list map -> foldr addCoords map list) M.empty $ map parseLine $ zip [0..] (lines input)
                  size = getSize input
              in (m, size)

parseLine :: (Int, String) -> [(Char, Coord)]
parseLine (y, l) = map (\(x, c) -> (c, (x, y))) $ filter ((/=)'.' . snd) (zip [0..] l)

addCoords :: (Char, Coord) -> Map -> Map
addCoords (c, p) map = M.alter prependOrUpdate c map
  where prependOrUpdate Nothing = Just [p]
        prependOrUpdate (Just old) = Just (p:old)

getSize :: String -> (Int, Int)
getSize input = let ls = lines input
                in (length $ head ls, length ls)

antinodes :: [Coord] -> [Coord]
antinodes (c:cs) = antinodes' c cs ++ antinodes cs
antinodes [] = []

antinodes' :: Coord -> [Coord] -> [Coord]
antinodes' c (other:cs) = let d = c `subCoord` other
                          in [c `addCoord` d, other `subCoord` d] ++ antinodes' c cs
antinodes' c [] = []

moreAntinodes :: (Int, Int) -> [Coord] -> [Coord]
moreAntinodes s (c:cs) = moreAntinodes' s c cs ++ moreAntinodes s cs
moreAntinodes _ [] = []

moreAntinodes' :: (Int, Int) -> Coord -> [Coord] -> [Coord]
moreAntinodes' s c (other:cs) = let d = c `subCoord` other
                              in moreAntinodes'' s c d ++ moreAntinodes'' s c (negCoord d) ++ moreAntinodes' s c cs
moreAntinodes' _ _ [] = []

moreAntinodes'' :: (Int, Int) -> Coord -> Coord -> [Coord]
moreAntinodes'' s c d
  | inside s c = c:moreAntinodes'' s (c `addCoord` d) d
  | otherwise = []

addCoord :: Coord -> Coord -> Coord
(ax, ay) `addCoord` (bx, by) = (ax + bx, ay + by)

subCoord :: Coord -> Coord -> Coord
(ax, ay) `subCoord` (bx, by) = (ax - bx, ay - by)

negCoord :: Coord -> Coord
negCoord (x, y) = (-x, -y)

inside :: (Int, Int) -> Coord -> Bool
inside (w, h) (x, y) = 0 <= x && x < w && 0 <= y && y < h
