
let rec fib n =
    match n with
    | (0 | 1) -> 1
    | x when x > 0 -> (fib (x-2) + fib (x-1))
    | _ -> raise (Invalid_argument "Negative value supplied to fib");;

fib (int_of_string Sys.argv.(1) );;
