import System.Environment (getArgs)

main = do
    args <- getArgs
    if (length args == 0) || (args!!0 == "1")
        then part1
        else part2

part1 :: IO ()
part1 = putStrLn . show =<<
    fst
    . applyDeltas moveDialPart1 0 50
    . map parseDelta
    . lines
    <$> getContents

part2 :: IO ()
part2 = putStrLn . show =<<
    fst
    . applyDeltas moveDialPart2 0 50
    . map parseDelta
    . lines
    <$> getContents

parseDelta :: String -> Int
parseDelta [] = 0
parseDelta (d:rest) =
    if d == 'L'
        then -1 * num
        else num
    where
        num = read rest

applyDeltas :: (Int -> Int -> Int -> (Int, Int))
                -> Int -> Int -> [Int] -> (Int, Int)
applyDeltas _ count dial [] = (count, dial)
applyDeltas moveDial count dial (delta:rest) =
    applyDeltas moveDial newCount newDial rest
    where
        (newCount, newDial) = moveDial count dial delta

moveDialPart1 :: Int -> Int -> Int -> (Int, Int)
moveDialPart1 count dial delta =
    if newDial == 0
        then (count + 1, newDial)
        else (count, newDial)
    where
        newDial = (((dial + delta) `mod` 100) + 100) `mod` 100

moveDialPart2 :: Int -> Int -> Int -> (Int, Int)
moveDialPart2 count dial delta =
    if dialPlusDelta == 0
        then (count + 1
             , newDial)
        else if dialPlusDelta < 0
            then (count + dialPlusDelta `div` (-100) + if dial > 0 then 1 else 0
                 , newDial)
            else (count + dialPlusDelta `div` 100
                 , newDial)
    where
        dialPlusDelta = dial + delta
        newDial = (((dialPlusDelta) `mod` 100) + 100) `mod` 100
