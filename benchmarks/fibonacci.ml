
let rec fib n =
    match n with
    | 0 -> 0
    | (1 | 2) -> 1
    | x -> (fib (x-2) + fib (x-1));;

fib (int_of_string Sys.argv.(1) );;
