import Solver

import qualified Data.Map.Strict as M
import Data.List
import qualified Data.Set as S

main = solve part1 part2

type Area = Int
type Coord = (Int, Int)
type Coords = S.Set Coord
data Fence = L Coord | R Coord | T Coord | B Coord deriving (Eq, Show)
type Map = M.Map Coord Char
type Perimeter = Int
type Region = ([Coord], [Fence])

parse :: String -> Map
parse = M.fromList . concat . map (\(y, l) -> [((x, y), c) | (x, c) <- zip [0..] l]) . zip [0..] . lines

part1 = partX (sum . map (\(a, f) -> length a * length f))
part2 = partX (sum . map (\(a, f) -> length a * length (filter (not . isPartOfSide f) f)))
partX cost input = let m = parse input
                   in cost $ fst $ foldr (addRegion m) ([], S.empty) (M.keys m)

addRegion :: Map -> Coord -> ([Region], Coords) -> ([Region], Coords)
addRegion m c (rs, cs)
  | c `S.member` cs = (rs, cs)
  | otherwise = let (x, y) = c
                    cs' = S.insert c cs
                    p = m M.! c
                    (l, cs'') = expandRegion m p (x - 1, y) c cs'
                    (r, cs''') = expandRegion m p (x + 1, y) c cs''
                    (u, cs'''') = expandRegion m p (x, y - 1) c cs'''
                    (d, cs''''') = expandRegion m p (x, y + 1) c cs''''
                in ((combineRegions [([c], []), l, r, u, d]):rs, cs''''')

combineRegions :: [Region] -> Region
combineRegions (ra:rb:rs) = let (rac, raf) = ra
                                (rbc, rbf) = rb
                            in combineRegions ((rac ++ rbc, raf ++ rbf):rs)
combineRegions (ra:[]) = ra
combineRegions [] = ([], [])

expandRegion :: Map -> Char -> Coord -> Coord -> Coords -> (Region, Coords)
expandRegion m p c@(x, y) prev cs
  | c `S.member` cs = (([], if Just p == M.lookup c m then [] else [getFence c prev]), cs)
  | otherwise = case M.lookup c m of
                     Nothing -> (([], [getFence c prev]), cs)
                     Just p' -> if p /= p'
                                then (([], [getFence c prev]), cs)
                                else let cs' = S.insert c cs
                                         (l, cs'') = expandRegion m p (x - 1, y) c cs'
                                         (r, cs''') = expandRegion m p (x + 1, y) c cs''
                                         (u, cs'''') = expandRegion m p (x, y - 1) c cs'''
                                         (d, cs''''') = expandRegion m p (x, y + 1) c cs''''
                                     in (combineRegions [([c], []), l, r, u, d], cs''''')

getFence :: Coord -> Coord -> Fence
getFence (cx, cy) (px, py)
  | px < cx = R (cx, cy)
  | cx < px = L (px, py)
  | py < cy = B (cx, cy)
  | cy < py = T (px, py)

isPartOfSide :: [Fence] -> Fence -> Bool
isPartOfSide fs f = any (neighbors f) fs
  where neighbors (L (x, y)) (L (x', y')) = x == x' && y' == y + 1
        neighbors (R (x, y)) (R (x', y')) = x == x' && y' == y + 1
        neighbors (T (x, y)) (T (x', y')) = y == y' && x' == x + 1
        neighbors (B (x, y)) (B (x', y')) = y == y' && x' == x + 1
        neighbors _ _ = False
