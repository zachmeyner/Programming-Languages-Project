module View (renderBoard, currentPlayerText, parseAsCoord, winnerText) where
import Data.List (intercalate, intersperse)
import Data.Char (toUpper)
import Model (Board, Player(X, O), Winner(Player, Cat))

-- I don't like this file, I barely know what's going on in it.
surround :: a -> [a] -> [a]
surround value array = [value] ++ intersperse value array ++ [value]

-- Gets the right symbol for a cell
renderCell :: Maybe Player -> String
renderCell (Just X) = "X"
renderCell (Just O) = "O"
renderCell Nothing = " "

-- I know this one! It renders the board 
renderBoard :: Board -> String
renderBoard =
  let
    header  = "   A     B     C  "
    divider = " -----+-----+-----"
    padding = "      |     |     "
    renderRow :: Int -> [Maybe Player] -> String
    renderRow n = intercalate "" . (++) [show n] . surround "  " . intersperse "|" . map renderCell
  in
    unlines . (++) [header] . surround padding . intersperse divider . zipWith renderRow [1..]

-- Prases coordinates and returns their integer equivalents
parseAsCoord :: String -> Maybe (Int, Int)
parseAsCoord [number, letter] =
  let
    maybeX = case toUpper letter of { 'A' -> Just 0; 'B' -> Just 1; 'C' -> Just 2; _ -> Nothing }
    maybeY = case number of { '1' -> Just 0; '2' -> Just 1; '3' -> Just 2; _ -> Nothing }
  in case (maybeX, maybeY) of
    (Just x, Just y) -> Just (x, y)
    _ -> Nothing
parseAsCoord _ = Nothing


-- Who's turn is it?
currentPlayerText :: Player -> String
currentPlayerText X = "It's X's turn"
currentPlayerText O = "It's O's turn"

-- Gives winner of the match, or Cat if it's a tie.
winnerText :: Winner -> String
winnerText (Player X) = "X Wins!"
winnerText (Player O) = "O Wins!"
winnerText Cat = "Cat!"
