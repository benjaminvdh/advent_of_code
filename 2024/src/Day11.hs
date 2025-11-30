import Solver

import qualified Data.IntMap as M

type Map = M.IntMap Int

main = solve part1 part2

part1 = partX 25
part2 = partX 75
partX n = countStones n . parse

parse :: String -> [Int]
parse = map read . words

countStones :: Int -> [Int] -> Int
countStones n xs = let m = M.fromList $ map (,1) xs
                   in sum $ M.elems $ blinkStones n m

blinkStones :: Int -> Map -> Map
blinkStones 0 m = m
blinkStones n m = M.foldrWithKey addStone M.empty $ blinkStones (n - 1) m

addStone :: Int -> Int -> Map -> Map
addStone x o m = let xs = blinkStone x
                 in foldl (\m x -> M.alter (updateOccurrences o) x m) m xs

blinkStone :: Int -> [Int]
blinkStone 0 = [1]
blinkStone s = let s' = show s
                   l  = length s'
               in if even l
                  then let (a, b) = splitAt (l `div` 2) s'
                       in [read a, read b]
                  else [s * 2024]

updateOccurrences :: Int -> Maybe Int -> Maybe Int
updateOccurrences o (Just o') = Just (o + o')
updateOccurrences o Nothing   = Just o
