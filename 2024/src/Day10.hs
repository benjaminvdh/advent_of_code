import Solver

import Data.Char
import qualified Data.Map.Strict as M
import Data.List

main = solve part1 part2

part1 input = let m = parse input
              in sum $ M.elems $ M.mapWithKey (trailheadScore m) m
part2 input = let m = parse input
              in sum $ M.elems $ M.mapWithKey (trailheadRating m) m

type Coord = (Int, Int)
type Map = M.Map Coord Int

parse :: String -> Map
parse = M.fromList . concat . map (\(y, cols) -> [((x, y), digitToInt h) | (x, h) <- cols]) . zip [0..] . map (zip [0..]) . lines

trailheadScore :: Map -> Coord -> Int -> Int
trailheadScore m (x, y) 0 = length $ nub $ [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] >>= walk 0 m
trailheadScore _ _ _ = 0

walk :: Int -> Map -> Coord -> [Coord]
walk hPrev m (x, y) = case M.lookup (x, y) m of 
                        Just h  -> if h == hPrev + 1
                                   then if h == 9
                                        then [(x, y)]
                                        else [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] >>= walk h m
                                   else []
                        Nothing -> []

trailheadRating :: Map -> Coord -> Int -> Int
trailheadRating m (x, y) 0 = length $ nub $ [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] >>= paths 0 m [(x, y)]
trailheadRating _ _ _ = 0

paths :: Int -> Map -> [Coord] -> Coord -> [[Coord]]
paths hPrev m p (x, y) = case M.lookup (x, y) m of 
                           Just h  -> if h == hPrev + 1
                                      then if h == 9
                                           then [(x, y):p]
                                           else [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] >>= paths h m ((x, y):p)
                                      else []
                           Nothing -> []
