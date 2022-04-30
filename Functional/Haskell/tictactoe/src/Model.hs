module Model (doTurn, getWinner, initialState, isVacant, State(..), Board, Player(..), Winner(..)) where
import Data.List (transpose)
import Safe (atMay)
import Data.Maybe (isJust, isNothing, catMaybes)
import Control.Monad (join)

-- Data models for the Player, Winner, Board, and State of the board
data Player = X | O deriving (Eq)

data Winner = Player Player | Cat

type Board = [[Maybe Player]]

data State = State
  {
    boardState :: Board,
    turnState :: Player
  }

-- Opponent of a player
opponentOf :: Player -> Player
opponentOf X = O
opponentOf O = X

-- Checks for a win along a diagonal
diagonal :: [[a]] -> [a]
diagonal board =
  catMaybes $ zipWith atMay board [0..]

-- Replaces part of board
replaceAt :: (a -> a) -> Int -> [a] -> [a]
replaceAt updateFn index array =
  case splitAt index array of
    (left, val:right) -> left ++ [updateFn val] ++ right
    _ -> array

-- Checks if part of the board has nothing in it
isVacant :: (Int, Int) -> Board -> Bool
isVacant (x, y) board =
  isNothing $ join $ (`atMay` x) =<< (`atMay` y) board

-- Places X or O inside of a vacant spot
placeInBoard :: (Int, Int) -> Maybe Player -> Board -> Board
placeInBoard (x, y) =
  (`replaceAt` y) . (`replaceAt` x) . const

-- Check if a player has won
isPlayerWinner :: Player -> Board -> Bool
isPlayerWinner player board =
  (any . all) (Just player ==) $
    board ++
    transpose board ++
    map diagonal [board, transpose board]

-- Checks for Cat
isCat :: Board -> Bool
isCat =
  (all . all) isJust

-- gets the winner for the match.
getWinner :: Board -> Maybe Winner
getWinner board
  | isPlayerWinner X board = Just (Player X)
  | isPlayerWinner O board = Just (Player O)
  | isCat board = Just Cat
  | otherwise = Nothing

-- does the next turn on the board
doTurn :: State -> (Int, Int) -> State
doTurn state coord =
  State {
    boardState = placeInBoard coord (Just $ turnState state) (boardState state),
    turnState = opponentOf (turnState state)
  }

-- Initial state of the board
initialState :: Player -> State
initialState firstPlayer =
  State {
    boardState = replicate 3 (replicate 3 Nothing),
    turnState = firstPlayer
  }