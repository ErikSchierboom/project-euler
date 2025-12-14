:- use_module(library(clpfd)).

solve(Sum) :-
    findall(N, pandigital_divisible_number(N), Ns),
    sumlist(Ns, Sum).

pandigital_divisible_number(Number) :-
    Digits = [D1,D2,D3,D4,D5,D6,D7,D8,D9,D10],
    Digits ins 0..9,
    all_different(Digits),
    D1 #\= 0,

    D4 mod 2 #= 0,
    (D3*100 + D4*10 + D5) mod 3 #= 0,
    D6 mod 5 #= 0,
    (D5*100 + D6*10 + D7) mod 7 #= 0,
    (D6*100 + D7*10 + D8) mod 11 #= 0,
    (D7*100 + D8*10 + D9) mod 13 #= 0,
    (D8*100 + D9*10 + D10) mod 17 #= 0,

    labeling([], Digits),

    Number is
        D1*1000000000 + D2*100000000 + D3*10000000 +
        D4*1000000 + D5*100000 + D6*10000 +
        D7*1000 + D8*100 + D9*10 + D10.
