import Solver

import Data.Char
import Data.List
import qualified Data.Map.Strict as M

main = solve part1 part2

part1 = sum . map countMatches . concat . morph . lines
part2 = countMAS . toMap

morph :: [String] -> [[String]]
morph input = let vrev = transpose input 
                  hrev = map reverse input
                  vhrev = map reverse vrev
                  diag = diagonaliseBlock input
                  vdiag = diagonaliseBlock $ reverse hrev
                  hdiag = diagonaliseBlock hrev
                  vhdiag = diagonaliseBlock vhrev
              in [input, hrev, vrev, vhrev, diag, vdiag, hdiag, vhdiag]

diagonaliseBlock ss = diagonaliseRows ss ++ diagonaliseRows (transpose (map tail ss))

diagonaliseRows :: [String] -> [String]
diagonaliseRows (l:ls) = diagonaliseOnce (l:ls):diagonaliseRows ls
diagonaliseRows [] = []

diagonaliseOnce :: [String] -> String
diagonaliseOnce ((l:_):lss) = l:diagonaliseOnce (map tail lss)
diagonaliseOnce _ = []

countMatches :: String -> Int
countMatches a@(_:ss) = countMatches ss + if "XMAS" `isPrefixOf` a then 1 else 0
countMatches [] = 0

type Coord = (Int, Int)
type Map = M.Map Coord Char

toMap :: String -> Map
toMap input = M.fromList $ concat $ map (\(y, cs) -> [((x, y), c) | (x, c) <- cs]) $ zip [0..] (map (zip [0..]) (lines input))

countMAS :: Map -> Int
countMAS map = M.size $ M.filterWithKey isMAS map
  where isMAS (x, y) 'A' = case (get (x + 1) (y + 1), get (x + 1) (y - 1), get (x - 1) (y + 1), get (x - 1) (y - 1)) of
                             (Just br, Just tr, Just bl, Just tl) -> ((br == 'M' && tl == 'S') || (br == 'S' && tl == 'M')) && ((bl == 'M' && tr == 'S') || (bl == 'S' && tr == 'M'))
                             _ -> False
        isMAS _ _ = False
        get x y = M.lookup (x, y) map
