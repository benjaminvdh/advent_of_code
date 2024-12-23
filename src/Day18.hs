import Solver

import qualified Data.Map as M

type Coord = (Int, Int)

main = solve part1 part2
part1 input = let cs = parse input
                  isLarge = any (\(x, y) -> x > 6 || y > 6) cs
                  size = if isLarge then (70, 70) else (6, 6)
                  numBytes = if isLarge then 1024 else 12
                  m = buildMap size
                  m' = foldr (\c m -> M.insert c Corrupted m) m (take numBytes cs)
                  Fixed result = updateMap m' M.! size
              in result
part2 input = let cs = parse input
                  isLarge = any (\(x, y) -> x > 6 || y > 6) cs
                  size = if isLarge then (70, 70) else (6, 6)
                  numBytes = if isLarge then 1024 else 12
                  m = buildMap size
                  m' = foldr (\c m -> M.insert c Corrupted m) m (take numBytes cs)
                  cs' = drop numBytes cs
              in cs' !! binarySearch m' size cs'

parse :: String -> [Coord]
parse = map parseLine . lines
  where parseLine line = let a = takeWhile (/=',') line
                             b = drop (length a + 1) line
                         in (read a, read b)

data Distance = Unknown | Fixed Int | Corrupted deriving (Eq, Show)
type Size = (Int, Int)
type Map = M.Map Coord Distance

buildMap :: Size -> Map
buildMap (w, h) = let list = [((x, y), Unknown) | y <- [0..h], x <- [0..w]]
                      m = M.fromList list
                  in M.insert (0, 0) (Fixed 0) m

updateMap :: Map -> Map
updateMap m = let m' = updateMap' m
              in if m' == m
                 then m
                 else updateMap m'

updateMap' :: Map -> Map
updateMap' m = let reachable = M.keys $ M.filter (\a -> case a of Fixed _ -> True; _ -> False) m
               in foldr updateReachable m reachable

updateReachable :: Coord -> Map -> Map
updateReachable c@(x, y) m = case m M.! c of
                                  Fixed d -> let m' = updateReachable' d (x + 1, y) m
                                                 m'' = updateReachable' d (x - 1, y) m'
                                                 m''' = updateReachable' d (x, y + 1) m''
                                                 m'''' = updateReachable' d (x, y - 1) m'''
                                             in m''''
                                  _ -> m

updateReachable' :: Int -> Coord -> Map -> Map
updateReachable' d c m = case m M.!? c of
                              Just Unknown -> M.insert c (Fixed (d + 1)) m
                              Just (Fixed prev) | prev > d -> M.insert c (Fixed (d + 1)) m
                              _ -> m

binarySearch :: Map -> Size -> [Coord] -> Int
binarySearch _ _ [] = 0
binarySearch _ _ [_] = 0
binarySearch m s cs = let half = (length cs) `div` 2
                          m' = foldr (\c m -> M.insert c Corrupted m) m (take half cs)
                          m'' = updateMap m'
                      in case m'' M.! s of
                              Unknown -> binarySearch m s (take half cs)
                              Fixed _ -> half + binarySearch m' s (drop half cs)
