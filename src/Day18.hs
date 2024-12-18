import Solver

import Data.Char
import Data.List
import qualified Data.Map as M
import Data.Maybe
import qualified Data.Set as S

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
part2 _ = "N/A"

printMap :: Size -> Map -> IO ()
printMap (w, h) m = let s = map (\y -> foldr (rowToString y) [] [0..h]) [0..w]
                    in putStrLn $ concat $ intersperse "\n" s
                       where rowToString y x acc = let c = case m M.! (x, y) of
                                                                Fixed d -> intToDigit (d `mod` 10)
                                                                Corrupted -> '#'
                                                                Unknown -> ' '
                                                   in c:acc

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
