import Solver

import qualified Data.Map.Strict as M
import qualified Data.Set as S

main = solve part1 part2

input = "AAAA\nBBCD\nBBCC\nEEEC"

type Area = Int
type Coord = (Int, Int)
type Coords = S.Set Coord
type Map = M.Map Coord Char
type Perimeter = Int
type Region = (Area, Perimeter)

parse :: String -> Map
parse = M.fromList . concat . map (\(y, l) -> [((x, y), c) | (x, c) <- zip [0..] l]) . zip [0..] . lines

part1 input = let m = parse input
              in fst $ foldr (addRegion m) (0, S.empty) (M.keys m)
part2 _ = "N/A"

addRegion :: Map -> Coord -> (Int, Coords) -> (Int, Coords)
addRegion m c (acc, cs)
  | c `S.member` cs = (acc, cs)
  | otherwise = let (x, y) = c
                    cs' = S.insert c cs
                    p = m M.! c
                    (l, cs'') = expandRegion m p (x - 1, y) cs'
                    (r, cs''') = expandRegion m p (x + 1, y) cs''
                    (u, cs'''') = expandRegion m p (x, y - 1) cs'''
                    (d, cs''''') = expandRegion m p (x, y + 1) cs''''
                    cost = (1, 0) <+> l <+> r <+> u <+> d
                in (acc + fst cost * snd cost, cs''''')

(<+>) :: Region -> Region -> Region
(aa, ap) <+> (ba, bp) = (aa + ba, ap + bp)

expandRegion :: Map -> Char -> Coord -> Coords -> (Region, Coords)
expandRegion m p c@(x, y) cs
  | c `S.member` cs = ((0, if m M.! c == p then 0 else 1), cs)
  | otherwise = case M.lookup c m of
                     Nothing -> ((0, 1), cs)
                     Just p' -> if p /= p'
                                then ((0, 1), cs)
                                else let cs' = S.insert c cs
                                         (l, cs'') = expandRegion m p (x - 1, y) cs'
                                         (r, cs''') = expandRegion m p (x + 1, y) cs''
                                         (u, cs'''') = expandRegion m p (x, y - 1) cs'''
                                         (d, cs''''') = expandRegion m p (x, y + 1) cs''''
                                     in ((1, 0) <+> l <+> r <+> u <+> d, cs''''')
