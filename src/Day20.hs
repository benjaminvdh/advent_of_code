import Solver

import Data.List
import qualified Data.Map.Strict as M
import Data.Maybe
import qualified Data.Set as S

main = solve part1 part2

input = "###############\n\
\#...#...#.....#\n\
\#.#.#.#.#.###.#\n\
\#S#...#.#.#...#\n\
\#######.#.#.###\n\
\#######.#.#...#\n\
\#######.#.###.#\n\
\###..E#...#...#\n\
\###.#######.###\n\
\#...###...#...#\n\
\#.#####.#.###.#\n\
\#.#...#.#.#...#\n\
\#.#.#.#.#.#.###\n\
\#...#...#...###\n\
\###############"
m = parse input
r = toPath m
r' :: Path = M.fromList r

part1 = partX nShortcuts
part2 = partX nShortcuts20
partX f input = let m = parse input
                    r = M.fromList $ toPath m
                in f r (if length (lines input) == 15 then 64 else 100)

type Coord = (Int, Int)
type Map = M.Map Coord Char
type Path = M.Map Coord Int

parse :: String -> Map
parse = M.fromList . concat . map (\(y, cs) -> [((x, y), c) | (x, c) <- zip [0..] cs]) . zip [0..] . lines

toPath :: Map -> [(Coord, Int)]
toPath m = traversePath m (findStart m) (0, 0) 0

findStart :: Map -> Coord
findStart m = fst $ fromJust $ find ((=='S') . snd) $ M.assocs m

traversePath :: Map -> Coord -> Coord -> Int -> [(Coord, Int)]
traversePath m c p d = if m M.! c == 'E'
                       then [(c, d)]
                       else (c, d):traversePath m (getNext m c p) c (d + 1)

getNext :: Map -> Coord -> Coord -> Coord
getNext m (x, y) p = head $ [n | n <- [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)], n /= p && m M.! n `elem` ['.', 'E']]

nShortcuts :: Path -> Int -> Int
nShortcuts m min = M.foldrWithKey (addShortcutsAt m min) 0 m

nShortcuts20 :: Path -> Int -> Int
nShortcuts20 m min = M.foldrWithKey (addShortcutsAt20 m min) 0 m

addShortcutsAt :: Path -> Int -> Coord -> Int -> Int -> Int
addShortcutsAt m min (x, y) d acc = acc + length [n | n <- [(x + 2, y), (x - 2, y), (x, y + 2), (x, y - 2)], maybe False (\d' -> d' - d >= min + 2) (M.lookup n m)]

addShortcutsAt20 :: Path -> Int -> Coord -> Int -> Int -> Int
addShortcutsAt20 m min (x, y) d acc = acc + length [(x', y') | x' <- [-20..20], y' <- [-20..20], abs x' + abs y' <= 20, maybe False (\d' -> d' - d >= min + abs x' + abs y') (M.lookup (x + x', y + y') m)]
