import Solver

import Data.List
import Data.Maybe
import qualified Data.Map.Strict as M

main = solve part1 part2

input = "########\n#..O.O.#\n##@.O..#\n#...O..#\n#.#.O..#\n#...O..#\n#......#\n########\n\n<^^>>>vv<v>>v<<"

part1 input = let (m, ds) = parse input
                  m' = foldl updateMap m ds
              in M.foldrWithKey gps 0 m'
part2 _ = "N/A"

type Coord = (Int, Int)
data Dir = L | R | U | D deriving (Eq, Show)
type Map = M.Map Coord Tile
data Tile = Robot | Box | Wall | Empty deriving (Eq, Show)

printMap m = printMap' m 0

printMap' m r = case M.lookup (0, r) m of
                     Just row -> printRow m (0, r) ++ printMap' m (r + 1)
                     Nothing -> ""

printRow m (x, y) = case M.lookup (x, y) m of
                          Just t -> printChar t:printRow m (x + 1, y)
                          Nothing -> "\n"

printChar Robot = '@'
printChar Box = 'O'
printChar Wall = '#'
printChar Empty = ' '

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
updateMap m d = moveRobot m (findRobot m) d

moveRobot :: Map -> Coord -> Dir -> Map
moveRobot m r d = let next = getNext d r
                  in case m M.! next of
                          Empty -> move Robot r next m
                          Wall -> m
                          Box -> let m' = moveBox m next d
                                 in if m' M.! next == Empty
                                    then move Robot r next m'
                                    else m

findRobot :: Map -> Coord
findRobot = fst . fromJust . find ((==Robot) . snd) . M.assocs

move :: Tile -> Coord -> Coord -> Map -> Map
move t from to = M.insert from Empty . M.insert to t

moveBox :: Map -> Coord -> Dir -> Map
moveBox m c d = let next = getNext d c
                in case m M.! next of
                        Empty -> move Box c next m
                        Wall -> m
                        Box -> let m' = moveBox m next d
                               in if m' M.! next == Empty
                                  then move Box c next m'
                                  else m

getNext :: Dir -> Coord -> Coord
getNext L (x, y) = (x - 1, y)
getNext R (x, y) = (x + 1, y)
getNext U (x, y) = (x, y - 1)
getNext D (x, y) = (x, y + 1)

gps :: Coord -> Tile -> Int -> Int
gps (x, y) Box acc = acc + 100 * y + x
gps _ _ acc = acc
