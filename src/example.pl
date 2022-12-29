likes(kim, robin).
likes(sandy, lee).
likes(sandy, kim).
likes(robin, cats).
likes(sandy, X) :- likes(X, cats).
likes(kim, X) :- likes(X, lee), likes(X, kim).
likes(X, X).
