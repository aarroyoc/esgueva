member(X, [X|Xs]).
member(X, [Y|Xs]) :- member(X, Xs).

list([a,b,c]).
