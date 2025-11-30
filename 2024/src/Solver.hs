module Solver (solve) where

import System.Environment
import System.IO

solve :: (Show a, Show b) => (String -> a) -> (String -> b) -> IO ()
solve part1 part2 = do args <- getArgs
                       contents <- readFile (head args)
                       putStrLn ("Part 1:\n" ++ (show (part1 contents)) ++ "\n")
                       putStrLn ("Part 2:\n" ++ (show (part2 contents)))
