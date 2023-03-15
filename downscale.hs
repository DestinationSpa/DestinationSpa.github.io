import System.Environment

main = do
  [from,to] <- map read <$> getArgs
  putStrLn []
  putStr $ unlines $ reverse $ map (mediaQuery to) [from..to]
  putStrLn []

mediaQuery :: Int -> Int -> String
mediaQuery orig target = concat
  [ "@media "
  , "(max-width: " ++ show target ++ "mm) { "
  , "body { "
  , "width: " ++ show (100/ratio) ++ "%; "
  , "height: " ++ show (100/ratio) ++ "%; "
  , "transform-origin: top left; "
  , "transform: scale(" ++ show ratio ++ "); }}" ]
  where
    ratio = fromIntegral (target-1) / fromIntegral orig
