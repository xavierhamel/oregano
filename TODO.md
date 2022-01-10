# À FAIRE
 [ ] On veut que les propriétés soient toujours dans le même ordre.
 [ ] On veut ajouter des metadata aux propriétés qui sont standard selon le nom
    (peut être que c'est ici qu'on pourrait indiquer la priorité de la propriété
    si elle est présente).
 [ ] Sélectionner plusieurs éléments différents (avec shift ou controle)
 [ ] Quand on peut sélectionner une connection (et que donc si on appuie on
    part un fil) mettre le connecteur d'une autre couleur, autre forme et que la
    souris pointe.
 [ ] Peut-être refactor un peu pour que quand on ajoute un composante, on
    dispatch un event au lieu de faire plus de fonctions à la même place (dans
    entities par exemple alors que ça n'a pas nécessairement de lien avec ça).
    Mais c'est pas très Rust de faire ça...
 [ ] Régler le bug avec le voltmètre, une des connection marche pas bien quand
    il est inversé (et peut-être dans d'autres cas).
 [ ] Refaire la façon que les connections sont identifiés pour que
    component.connected_to ne soit pas un vecteur vide mais un vecteur d'option de
    connection. On assume que la borne à l'index 0 est la borne positive.
