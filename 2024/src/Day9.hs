import Solver

import Data.Char
import Data.List
import Data.Maybe

main = solve part1 part2

part1 = checksum . compact . parse
part2 = snd . foldl' checksumWhole (0, 0) . compactWhole . parse

parse :: String -> [Maybe Int]
parse input = let (_, list, _) = foldl expand (True, [], 0) input 
              in reverse list

expand :: (Bool, [Maybe Int], Int) -> Char -> (Bool, [Maybe Int], Int)
expand acc '\n'           = acc
expand  (True, acc, id) c = let n = digitToInt c
                            in (False, replicate n (Just id) ++ acc, id + 1)
expand (False, acc, id) c = let n = digitToInt c
                            in (True, replicate n Nothing ++ acc, id)

compact :: [Maybe Int] -> [Int]
compact ds = let n = length (filter isJust ds)
                 c = compact' ds (reverse ds)
             in take n c

compact' :: [Maybe Int] -> [Maybe Int] -> [Int]
compact'  (Just d:ds) rs = d:compact' ds rs
compact' (Nothing:ds) rs = let rs' = dropWhile isNothing rs
                           in case uncons rs' of
                                   Just (Just d, rest) -> d:compact' ds rest
                                   _                   -> []
compact' [] _ = []

checksum :: [Int] -> Int
checksum = sum . map (uncurry (*)) . zip [0..]

type Size  = Int
type Id    = Int
data Block = Empty Size | Data Size Id deriving (Show)

size :: Block -> Int
size (Empty s)  = s
size (Data s _) = s

isEmpty :: Block -> Bool
isEmpty (Empty _)  = True
isEmpty (Data _ _) = False

compactWhole :: [Maybe Int] -> [Block]
compactWhole ds = let blocks = transform ds
                  in foldl' findEmptySpot blocks (reverse blocks)

findEmptySpot :: [Block] -> Block -> [Block]
findEmptySpot (d@(Data _ i'):ds) b@(Data _ i) 
  | i /= i' = d:findEmptySpot ds b
  | i == i' = d:ds
findEmptySpot (e@(Empty s'):ds) b@(Data s i)
  | s < s'    = b:Empty (s' - s):(remove i ds)
  | s == s'   = b:(remove i ds)
  | otherwise = let nextEmpties = takeWhile isEmpty ds
                in if s <= s' + sum (map size nextEmpties)
                   then b:(remove i (drop (length nextEmpties) ds))
                   else e:findEmptySpot ds b
findEmptySpot ds b@(Empty _) = ds

transform :: [Maybe Int] -> [Block]
transform  (Just d:ds) = let n = length $ takeWhile (matches d) ds
                         in Data (n + 1) d:transform (drop n ds)
transform (Nothing:ds) = let n = length $ takeWhile isNothing ds
                         in Empty (n + 1):transform (drop n ds)
transform [] = []

remove :: Id -> [Block] -> [Block]
remove i (Data s i':bs)
  | i == i'   = Empty s:bs
  | otherwise = (Data s i'):remove i bs
remove i (Empty s:bs) = Empty s:remove i bs

matches :: Int -> Maybe Int -> Bool
matches d (Just m) = d == m
matches _ Nothing  = False

checksumWhole :: (Int, Int) -> Block -> (Int, Int)
checksumWhole (id, acc) (Data s i) = (id + s, acc + (sum $ map (*i) $ [id..id + s - 1]))
checksumWhole (id, acc)  (Empty s) = (id + s, acc)
