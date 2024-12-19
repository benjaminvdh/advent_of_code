import Solver

import Data.List

main = solve part1 part2
part1 = length . uncurry filterPossible . parse
part2 _ = "N/A"

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
