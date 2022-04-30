{---
  This is the first ever thing I've written in a function language, I have no clue how to do it so I found a general outline 
  that I liked for making tic tac toe in haskell. After finishing this I'm going to attempt to do what I did here, but in some other functional langauge.
---}
import System.IO (hFlush, stdout)
import System.Random (randomIO)
import Model (initialState, doTurn, getWinner, isVacant, Player(X, O), State(boardState, turnState))
import View (renderBoard, currentPlayerText, parseAsCoord, winnerText)

-- Used to prompt user
prompt :: String -> IO String
prompt msg = do
  putStr msg >> hFlush stdout
  getLine

-- Prompts user for coordiante input and checks to see if that spot is empty.
-- If the spot is occupied or not valid it will ask again.
promptForVacantCoord :: State -> IO (Int, Int)
promptForVacantCoord state = do
  userInp <- prompt "Pick a spot (e.g. 2B): "
  case parseAsCoord userInp of
    Just coord | isVacant coord (boardState state) -> return coord
    _ -> putStrLn "Invalid input" >> promptForVacantCoord state

-- Steps through gameplay (renders board after moves)
-- Checks for winner after every turn
step :: State -> IO ()
step state = do
  putStrLn ""
  putStrLn $ renderBoard $ boardState state
  case getWinner (boardState state) of
    Just winner -> putStrLn (winnerText winner)
    Nothing -> do
      putStrLn $ currentPlayerText (turnState state)
      coord <- promptForVacantCoord state
      step $ doTurn state coord

-- Starts the board, stepping, and picks a random player to start.
main :: IO ()
main =
  let
    playerFromBool True = X
    playerFromBool False = O
  in
    step . initialState . playerFromBool =<< randomIO