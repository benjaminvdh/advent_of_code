import Data.List
import Data.Maybe
import System.Environment
import System.IO

type Coord = (Int, Int)
type Size = Coord
type Vel = Coord
type Pos = Coord
type Robot = (Pos, Vel)

main = do args <- getArgs
          input <- readFile (head args)
          let (s, rs) = parse input
          putStrLn ("Part 1:\n" ++ show (part1 s rs))
          putStrLn "\nPart 2:"
          if length args >= 2
          then do ref <- readFile (args !! 1)
                  putStrLn (show (part2 s rs (init ref)))
          else do putStrLn (show "N/A")

part1 :: Size -> [Robot] -> Int
part1 s rs = countQuadrants $ update 100 (s, rs)

part2 :: Size -> [Robot] -> String -> Int
part2 s rs ref = fromJust $ find (\i -> printMap s rs i == ref) [0..]

parse :: String -> (Size, [Robot])
parse = parseSize . map parseRobot . lines
        where parseSize rs = if any (\((x, y), _) -> x >= 11 || y >= 7) rs
                             then ((101, 103), rs)
                             else (( 11,   7), rs)

parseRobot :: String -> Robot
parseRobot l = let l' = drop 2 l
                   sp = takeWhile (/=' ') l'
                   sv = drop (length sp + 3) l'
               in (parseCoord sp, parseCoord sv)

parseCoord :: String -> Coord
parseCoord l = let sx = takeWhile (/=',') l
                   sy = drop (length sx + 1) l
               in (read sx, read sy)

update :: Int -> (Size, [Robot]) -> (Size, [Robot])
update n (s, rs) = (s, map (update' n s) rs)
                   where update' n (sx, sy) ((px, py), (vx, vy)) = let x' = px + n * vx
                                                                       y' = py + n * vy
                                                                   in ((x' `mod` sx, y' `mod` sy), (vx, vy))

countQuadrants :: (Size, [Robot]) -> Int
countQuadrants ((sx, sy), rs) = let mx = sx `div` 2
                                    my = sy `div` 2
                                in countQuadrant       (     0,      0) (mx, my) rs
                                       * countQuadrant (mx + 1,      0) (sx, my) rs
                                       * countQuadrant (     0, my + 1) (mx, sy) rs
                                       * countQuadrant (mx + 1, my + 1) (sx, sy) rs

countQuadrant :: Coord -> Coord -> [Robot] -> Int
countQuadrant (fx, fy) (tx, ty) = length . filter (\((px, py), _) -> fx <= px && px < tx && fy <= py && py < ty)

printMap :: Size -> [Robot] -> Int -> String
printMap s@(sx, sy) rs i = let (_, rs') = update i (s, rs)
                           in concat $ intersperse "\n" $ map (\y -> map (\x -> if any (\((px, py), _) -> px == x && py == y) rs' then '*' else ' ') [0..sx]) [0..sy]
