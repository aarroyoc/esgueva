member(X, [X|Xs]).
member(X, [Y|Xs]) :- member(X, Xs).

nextto(X, Y, List) :- iright(X, Y, List).
nextto(X, Y, List) :- iright(Y, X, List).

iright(Left, Right, [Left|[Right|Xs]]).
iright(Left, Right, [X|Xs]) :- iright(Left, Right, Xs).

eq(X, X).


zebra(H, W, Z) :- eq(H, [house(norwegian, X1, X2, X3, X4), X5, house(X6, X7, X8, milk, X9), X10, X11]), member(house(englishman, A1, A2, A3, red), H), member(house(spaniard, dog, B1, B2, B3), H), member(house(C1, C2, C3, coffee, green), H), member(house(ukrainian, D1, D2, tea, D3), H), iright(house(E1, E2, E3, E4, ivory), house(F1, F2, F3, F4, green), H), member(house(G1, snails, winston, G2, G3), H), member(house(H1, H2, kools, H3, yellow), H), nextto(house(I1, I2, chesterfield, I3, I4), house(J1, fox, J2, J3, J4), H), nextto(house(K1, K2, kools, K3, K4), house(L1, horse, L2, L3, L4), H), member(house(M1, M2, luckystrike, orangejuice, M3), H), member(house(japanese, N1, parliaments, N2, N3), H), nextto(house(norwegian, O1, O2, O3, O4), house(P1, P2, P3, P4, blue), H), member(house(W, R1, R2, water, R3), H), member(house(Z, zebra, S1, S2, S3), H).
