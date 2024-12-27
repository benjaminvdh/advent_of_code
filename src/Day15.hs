import Solver

import Control.Monad
import Data.List
import Data.Maybe
import qualified Data.Map.Strict as M

main = solve part1 part2

part1 input = let (m, ds) = parse input
              in partX m ds
part2 input = let (m, ds) = parse input
              in partX (widen m) ds
partX m ds = M.foldrWithKey gps 0 $ foldl updateMap m ds

type Coord = (Int, Int)
data Dir = L | R | U | D deriving (Eq, Show)
type Map = M.Map Coord Tile
data Tile = Robot | Box | BoxL | BoxR | Wall | Empty deriving (Eq, Show)

parse :: String -> (Map, [Dir])
parse input = (parseMap $ takeWhile (/="") $ lines input, foldr (:) [] $ map parseInstruction $ concat $ dropWhile (/="") $ lines input)

parseMap :: [String] -> Map
parseMap = M.fromList . concat . map (\(y, l) -> map (\(x, c) -> ((x, y), parseTile c)) $ zip [0..] l) . zip [0..]

parseTile :: Char -> Tile
parseTile '@' = Robot
parseTile 'O' = Box
parseTile '#' = Wall
parseTile '.' = Empty

parseInstruction :: Char -> Dir
parseInstruction '<' = L
parseInstruction '>' = R
parseInstruction '^' = U
parseInstruction 'v' = D

updateMap :: Map -> Dir -> Map
updateMap m = moveRobot m (findRobot m)

moveRobot :: Map -> Coord -> Dir -> Map
moveRobot m r d = let next@(x', y') = getNext d r
                  in case m M.! next of
                          Empty -> move Robot r next m
                          Wall  -> m
                          Box   -> maybe m (move Robot r next) $ moveBox m next d
                          BoxL  -> maybe m (move Robot r next) $ moveBoxL next d m
                          BoxR  -> maybe m (move Robot r next) $ moveBoxL (x' - 1, y') d m

findRobot :: Map -> Coord
findRobot = fst . fromJust . find ((==Robot) . snd) . M.assocs

move :: Tile -> Coord -> Coord -> Map -> Map
move BoxL from@(x, y) to@(x', y') = M.insert to BoxL . M.insert (x' + 1, y') BoxR . M.insert from Empty . M.insert (x + 1, y) Empty
move t from to = M.insert from Empty . M.insert to t

moveBox :: Map -> Coord -> Dir -> Maybe Map
moveBox m c d = let next = getNext d c
                in case m M.! next of
                        Empty -> Just $ move Box c next m
                        Wall  -> Nothing
                        Box   -> fmap (move Box c next) $ moveBox m next d

moveBoxL :: Coord -> Dir -> Map -> Maybe Map
moveBoxL c d m = let bs = getBoxes c d m
                     m' = foldr (\b m -> join $ fmap (moveBoxL b d) m) (Just m) bs
                 in if maybe False (canMoveBoxL c d) m'
                    then fmap (move BoxL c (getNext d c)) m'
                    else Nothing

getBoxes :: Coord -> Dir -> Map -> [Coord]
getBoxes (x, y) L m = [(x', y)     | x' <- [x - 2],           m M.! (x', y)     == BoxL]
getBoxes (x, y) R m = [(x', y)     | x' <- [x + 2],           m M.! (x', y)     == BoxL]
getBoxes (x, y) U m = [(x', y - 1) | x' <- [x - 1, x, x + 1], m M.! (x', y - 1) == BoxL]
getBoxes (x, y) D m = [(x', y + 1) | x' <- [x - 1, x, x + 1], m M.! (x', y + 1) == BoxL]

canMoveBoxL :: Coord -> Dir -> Map -> Bool
canMoveBoxL (x, y) L m = m M.! (x - 1, y) == Empty
canMoveBoxL (x, y) R m = m M.! (x + 2, y) == Empty
canMoveBoxL (x, y) U m = m M.! (x, y - 1) == Empty && m M.! (x + 1, y - 1) == Empty
canMoveBoxL (x, y) D m = m M.! (x, y + 1) == Empty && m M.! (x + 1, y + 1) == Empty

getNext :: Dir -> Coord -> Coord
getNext L (x, y) = (x - 1, y)
getNext R (x, y) = (x + 1, y)
getNext U (x, y) = (x, y - 1)
getNext D (x, y) = (x, y + 1)

gps :: Coord -> Tile -> Int -> Int
gps (x, y) Box acc = acc + 100 * y + x
gps (x, y) BoxL acc = gps (x, y) Box acc
gps _ _ acc = acc

widen :: Map -> Map
widen m = M.foldrWithKey widenTile M.empty m
  where widenTile (x, y) t = let (l, r) = doubleTile t
                             in (M.insert (x * 2, y) l . M.insert (x * 2 + 1, y) r)

doubleTile :: Tile -> (Tile, Tile)
doubleTile Robot = (Robot, Empty)
doubleTile Box   = (BoxL, BoxR)
doubleTile Wall  = (Wall, Wall)
doubleTile Empty = (Empty, Empty)
