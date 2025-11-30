import Solver

import Data.List
import qualified Data.Set as Set

main = solve part1 part2

part1 input = let (g, s, os) = parse input
              in length $ nub $ walk g Up s os
part2 input = let (g, s, os) = parse input
                  path = nub $ walk g Up s os
              in length $ filter (\o -> loops g Up s (Set.insert o os) Set.empty) $ tail path

type Coord = (Int, Int)
type Coords = Set.Set Coord
type Pos = (Coord, Dir)
type PosSet = Set.Set Pos

data Dir = Up | Right | Down | Left deriving (Enum, Eq)

instance Ord Dir where
  Up <= _ = True
  Main.Right <= Up = False
  Main.Right <= _ = True
  Down <= Main.Right = False
  Down <= _ = True
  Main.Left <= Main.Left = True
  _ <= _ = True

parse :: String -> (Coord, (Int, Int), Coords)
parse input = let ls = lines input
              in (parseGuard ls, parseSize ls, parseObstacles ls)

parseGuard :: [String] -> Coord
parseGuard input = head $ concat $ map (parseLine '^') $ zip [0..] input

parseObstacles :: [String] -> Coords
parseObstacles input = Set.fromList $ concat $ map (parseLine '#') $ zip [0..] input

parseLine :: Char -> (Int, String) -> [Coord]
parseLine t (y, line) = map (\(x, _) -> (x, y)) $ filter (\(_, c) -> c == t) $ zip [0..] line

parseSize :: [String] -> (Int, Int)
parseSize input = (length (head input), length input)

walk :: Coord -> Dir -> (Int, Int) -> Coords -> [Coord]
walk g dir s os 
  | inside s g = let (g', dir') = update g dir os
                 in g:walk g' dir' s os
  | otherwise  = []

inside :: (Int, Int) -> Coord -> Bool
inside (w, h) (x, y) = 0 <= x && x < w && 0 <= y && y < h

update :: Coord -> Dir -> Coords -> (Coord, Dir)
update g dir os = let g' = step g dir
                  in if g' `elem` os then let d' = nextDir dir in update g d' os else (g', dir)

nextDir Main.Left = Up
nextDir dir       = succ dir

step :: Coord -> Dir -> Coord
step (gx, gy) dir = case dir of
  Up         -> (gx    , gy - 1)
  Main.Right -> (gx + 1, gy    )
  Down       -> (gx    , gy + 1)
  Main.Left  -> (gx - 1, gy    )

loops :: Coord -> Dir -> (Int, Int) -> Coords -> PosSet -> Bool
loops g dir s os prevs
  | inside s g = let (g', dir') = update g dir os
                 in if (g', dir') `Set.member` prevs then True else loops g' dir' s os (Set.insert (g', dir') prevs)
  | otherwise = False
