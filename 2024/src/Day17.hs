import Solver

import Control.Monad
import Data.Bits
import Data.Char
import Data.Maybe
import Data.List

type Registers = (Int, Int, Int)
type Instr = Int
type Op = Int
type Pc = Int

main = solve part1 part2

part1 input = let (regs, instrs) = parse input
                  outs = exec regs instrs 0
              in concat $ intersperse "," $ map show outs
part2 input = let (_, instrs) = parse input
              in getSelfReplicatingValue instrs instrs

parse :: String -> (Registers, [Int])
parse input = let ls = lines input
                  [la, lb, lc] = take 3 ls
                  li = head $ drop 4 ls
                  a = read $ drop 12 la
                  b = read $ drop 12 lb
                  c = read $ drop 12 lc
                  is = parseInstr $ drop 9 li
              in ((a, b, c), is)

parseInstr :: String -> [Int]
parseInstr (a:_:b:',':xs) = digitToInt a:digitToInt b:parseInstr xs
parseInstr (a:_:b:[]) = digitToInt a:digitToInt b:[]

exec :: Registers -> [Int] -> Pc -> [Int]
exec r is pc = if length is > pc
               then let i = is !! pc
                        (r', pc', out) = exec' r i (is !! (pc + 1)) pc
                    in case out of
                            Just x  -> x:exec r' is pc'
                            Nothing -> exec r' is pc'
               else []

exec' :: Registers -> Instr -> Op -> Pc -> (Registers, Pc, Maybe Int)
exec' r@(ra, rb, rc) i op pc = case i of
  0 -> ((r `comboDiv` op, rb, rc), pc + 2, Nothing)
  1 -> ((ra, rb `xor` op, rc), pc + 2, Nothing)
  2 -> ((ra, (comboOp r op) `mod` 8, rc), pc + 2, Nothing)
  3 -> if ra == 0
       then (r, pc + 2, Nothing)
       else (r, op, Nothing)
  4 -> ((ra, rb `xor` rc, rc), pc + 2, Nothing)
  5 -> (r, pc + 2, Just ((comboOp r op) `mod` 8))
  6 -> ((ra, r `comboDiv` op, rc), pc + 2, Nothing)
  7 -> ((ra, rb, r `comboDiv` op), pc + 2, Nothing)

comboOp :: Registers -> Int -> Int
comboOp (ra, rb, rc) op = case op of
  4 -> ra
  5 -> rb
  6 -> rc
  x -> x

comboDiv :: Registers -> Int -> Int
comboDiv r@(ra, _, _) op = let c = comboOp r op
                               p = 2 ^ c
                           in ra `div` p

getSelfReplicatingValue :: [Instr] -> [Instr] -> Maybe Int
getSelfReplicatingValue is (i:is') = let prev = fmap (\p -> p `shiftL` 3) $ getSelfReplicatingValue is is'
                                         curr = join $ fmap (\p -> find (\x -> head (exec (x + p, 0, 0) is 0) == i) [0..7]) prev
                                     in join $ fmap (\prev -> fmap (\curr -> prev + curr) curr) prev
getSelfReplicatingValue _ [] = Just 0
