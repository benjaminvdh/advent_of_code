import Solver

import Data.Maybe
import qualified Data.Map.Strict as M

data Tile = Start | End | Wall | Empty { left :: Maybe Int, right :: Maybe Int, up :: Maybe Int, down :: Maybe Int } deriving (Eq, Show)

charToTile :: Char -> Tile
charToTile 'S' = Start
charToTile 'E' = Empty { left = Just 0, right = Just 0, up = Just 0, down = Just 0 }
charToTile '#' = Wall
charToTile '.' = Empty { left = Nothing, right = Nothing, up = Nothing, down = Nothing }

type Coord = (Int, Int)
type Map = M.Map Coord Tile

main = solve part1 part2

part1 input = let m = parse input
                  s = findStart m
                  m' = M.insert s Empty { left = Nothing, right = Nothing, up = Nothing, down = Nothing } m
              in case (fillDistances m') M.! s of
                      Empty { left, right, up, down } -> minimum $ catMaybes [left, right, up, down]   
part2 _ = "N/A"

parse :: String -> Map
parse = M.fromList . concat . map (\(y, cs) -> map (\(x, c) -> ((x, y), charToTile c)) cs) . zip [0..] . map (zip [0..]) . lines

findStart :: Map -> Coord
findStart = fst . M.elemAt 0 . M.filter (==Start)

fillDistances m = let m' = fillDistances' m
                  in if m' == m
                     then m
                     else fillDistances m'

fillDistances' m = foldr updateEntry m (M.keys m)

updateEntry :: Coord -> Map -> Map
updateEntry c@(x, y) m = case m M.! c of
                              Empty { left, right, up, down } -> let
                                  l' = case M.lookup (x - 1, y) m of 
                                            Just Empty { left, right, up, down } -> let l = fmap (\x -> x + 1) left
                                                                                        u = fmap (\x -> x + 1001) up
                                                                                        d = fmap (\x -> x + 1001) down
                                                                                        all = catMaybes [l, u, d]
                                                                                    in if null all then Nothing else Just (minimum all)
                                            _ -> Nothing
                                  r' = case M.lookup (x + 1, y) m of 
                                            Just Empty { left, right, up, down } -> let r = fmap (\x -> x + 1) right
                                                                                        u = fmap (\x -> x + 1001) up
                                                                                        d = fmap (\x -> x + 1001) down
                                                                                        all = catMaybes [r, u, d]
                                                                                    in if null all then Nothing else Just (minimum all)
                                            _ -> Nothing
                                  u' = case M.lookup (x, y + 1) m of 
                                            Just Empty { left, right, up, down } -> let u = fmap (\x -> x + 1) up
                                                                                        l = fmap (\x -> x + 1001) left
                                                                                        r = fmap (\x -> x + 1001) right
                                                                                        all = catMaybes [u, l, r]
                                                                                    in if null all then Nothing else Just (minimum all)
                                            _ -> Nothing
                                  d' = case M.lookup (x, y - 1) m of 
                                            Just Empty { left, right, up, down } -> let d = fmap (\x -> x + 1) down
                                                                                        l = fmap (\x -> x + 1001) left
                                                                                        r = fmap (\x -> x + 1001) right
                                                                                        all = catMaybes [d, l, r]
                                                                                    in if null all then Nothing else Just (minimum all)
                                            _ -> Nothing
                                 in M.insert c Empty { left = maybeMin left l', right = maybeMin right r', up = maybeMin up u', down = maybeMin down d' } m
                              _ -> m

maybeMin :: Maybe Int -> Maybe Int -> Maybe Int
maybeMin (Just a) (Just b) = Just $ min a b
maybeMin (Just a) Nothing  = Just a
maybeMin Nothing  (Just b) = Just b
maybeMin Nothing  Nothing  = Nothing
