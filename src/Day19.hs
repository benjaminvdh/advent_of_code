import Solver

import Data.List
import qualified Data.Map as M

contents = readFile "data/Day19Input.txt"

main = solve part1 part2
part1 = length . uncurry filterPossible . parse
part2 input = let (ts, ps) = parse input
              in sum $ map fst $ map (numWays ts M.empty) ps

type Pattern = String

parse :: String -> ([Pattern], [Pattern])
parse input = let ls = lines input
                  ts = head ls
                  ps = drop 2 ls
              in (parseTowels ts, ps)

parseTowels :: String -> [Pattern]
parseTowels s = let t = takeWhile (/=',') s
                    l = length t
                in if l > 0
                   then t:parseTowels (drop (l + 2) s)
                   else []

filterPossible :: [Pattern] -> [Pattern] -> [Pattern]
filterPossible ts ps = filter (isPossible $ simplifyTowels [] ts) ps

simplifyTowels :: [Pattern] -> [Pattern] -> [Pattern]
simplifyTowels prev (t:next) = let rest = simplifyTowels (t:prev) next
                               in if isPossible (prev ++ next) t
                                  then rest
                                  else t:rest
simplifyTowels _ []          = []

isPossible :: [Pattern] -> Pattern -> Bool
isPossible ts p
  | null p    = True
  | otherwise = any (\t -> if t `isPrefixOf` p then isPossible ts (drop (length t) p) else False) ts

type Map = M.Map String Int

numWays :: [Pattern] -> Map -> Pattern -> (Int, Map)
numWays _ m [] = (1, m)
numWays ts m p = case M.lookup p m of 
                      Just n -> (n, m)
                      Nothing -> foldr numWays' (0, m) ts
                 where numWays' = \t (acc, m) -> case stripPrefix t p of 
                                                      Just p' -> let (n, m') = numWays ts m p' in (acc + n, M.insertWith (+) p n m')
                                                      Nothing -> (acc, m)
