# À FAIRE
 [ ] Sélectionner plusieurs éléments différents (avec shift ou controle)
 [ ] Quand on peut sélectionner une connection (et que donc si on appuie on
    part un fil) mettre le connecteur d'une autre couleur, autre forme et que la
    souris pointe.
 [ ] Peut-être refactor un peu pour que quand on ajoute un composante, on
    dispatch un event au lieu de faire plus de fonctions à la même place (dans
    entities par exemple alors que ça n'a pas nécessairement de lien avec ça).
    Mais c'est pas très Rust de faire ça...
 [ ] Refaire la façon que les connections sont identifiés pour que
    component.connected_to ne soit pas un vecteur vide mais un vecteur d'option de
    connection. On assume que la borne à l'index 0 est la borne positive.
 [ ] Faire en sorte que quand on sélectionne une connection, on ne sélectionne
    pas le fil qu'il y a en dessous. En gros, les fils finissent au milieu des
    connections. Si on a une connection à la même endroit qu'un fil, alors on
    sélectionne le fil en priorité mais on voudrait sélectionner la connection
    (faire un nouveau fil) en priorité.
 [ ] On ne devrait pas pouvoir changer le nom du ground
 [ ] Refaire les dessins des différentes composantes
 [ ] Dans certains cas que je ne suis vraiment pas certain, le voltmètre à
     l'envers ne se connecte pas.

ERREUR
 [x] Au lieu de crash quand on a une erreur dans le circuit, simplement
      afficher le message d'erreur.
 [ ] Détecter quand ngspice ne retourne aucune ligne, alors on dit qu'il y a
     un bug (et probablement un problème dans le circuit.)
 [ ] Peut-être essayer de détecter quand ngspice print les erreurs dans le
     terminal et essayer de la parse pour la retourner à l'utilisateur.
 [x] Ajouter une erreur (ou warning) quand on a pas de ground dans le
     circuit.
 [ ] Pour l'erreur du ground, on devrait vérifier que les circuits
     indépendanton chacun leurs grounds.
 [ ] Ajouter une vérification pour les inputs qui sont des nombres.
 [ ] Avoir des messages d'erreur avec des information qui a du sens. Par
     exemple, mettre en rouge le composante en erreur.

SIMULATIONS
 [ ] Ajouter le fait qu'on peut ajouter un variation de paramètre simplement en
     séparant plusieurs valeurs par une virgule.
 [x] Ajouter une simulation par variation de fréquence (voir comment
     l'implémenter selon comment ngspice l'implémente)
 [ ] Éventuellement un simulation des point d'opération. Il va falloir trouver
     comment les ajouter. Si on change le circuit on les retires directement
 [ ] Afficher le nom des nodes si les nodes ne sont pas nommées (une fois la
     simulation faite)
 [ ] Les labels y du graphique devraient plutôt être par rapport au nom des
     probes et pas le nom des colonnes de ngspice. (Peut-être moyen de faire
     des alias dans ngspice (alias de v(a,b) serait le nom de la probe)).
 [ ] Ajouter les ampèremètres

NEW FEATURES
 [ ] Pouvoir export/import un projet
 [ ] Quand on appuie sur shift, on ne sélectionne rien mais on commence un
     nouveau fil peut importe si on clique sur un fil ou non. (Comme ça pas
     besoin de plusieurs outils)
 [ ] Quand on survol un item, changer la couleur ou sélectionner la bounding
     box. Cela fait en sorte que ça a l'air plus dynamique.
 [ ] Faire en sorte que le text soit à la bonne position, soit beau et qu'il
     tourne quand on tourne le composant.
 [ ] Ajouter un undo/redo
 []

DESIGN
 [x] Changer le design pour que ça ressemble plus à un logiciel qu'un site web
     moche. Peut-être s'inspirer de figma si figma a un dark theme.
 [ ] Ajouter des sections rétractable pour la liste de compostantes
 [x] La toolbar devrait être intégré avec d'autre menu et devrait être collé en
     haut de l'écran.

PLOT
 [ ] Quand on a peut de point, on ne sélectionne pas le point le plus proche
     mais seulement les points qui sont à moins de 1px... c'est moyen.
 [ ] Si le tick 0 est juste à gauche à la mm endroit que où sont les chiffres
     alors la ligne du 0 est affiché en dessous des chiffres
 [ ] On a un problème si les chiffres sur l'axe des y sont trop long. Il faut
     faire en sorte de mieux détecter la largeur de l'axe.
 [x] À la place de choisir les nodes qu'on veut avant de faire la simulation,
     on fait la simulation pour tous les nodes et on les sélectionne par après dans
     résultats.
 [x] À la place d'avoir un graph et un espace vide, on a un graph avec les
     options d'affichage à sa gauche.
 [x] On pourrait aussi avoir plusieurs graphs dans des tabs différents.
 [ ] Logarithm graph (ajouter une option genre une case à cocher)
