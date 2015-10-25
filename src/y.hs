module Y where
fix :: (t -> t) -> t
fix f = f (fix f)

fix2 :: ((a->a) -> (a->a)) -> (a->a)
fix2 f = f (fix2 f)

fix3 :: ((a->a) -> (a->a)) -> (a->a)
fix3 f x = f (\y -> fix3 f y) x

