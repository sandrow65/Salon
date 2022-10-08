# Salon


## But  
* Créer une pièce définie par des cloisons et la dessiner
* Définir une quantité et une taille de carreaux puis remplir la pièce avec (sans les découper) en respectant l'espacement par un joint dont la longueur est initialisée
* Afficher les carreaux positionnés dans la pièce
* Mettre en évidence les zones qui nécessiteront une découpe en affichant les distances


## Description de la solution
* On créé un module nommé *salon* consitué de 2 sous-modules :
  - *definition* dans lequel on définit une structure ***Cloison*** (nom, côté haut-gauche, côté bas-droit, couleur) et une fonction ***dessiner*** qui dessine la pièce en créant la liste de cloisons correspondantes
  - *materiau* dans lequel on définit une structure ***Carreau*** (position initiale du premier carreau, position courante, taille, quantité) et une fonction ***croise_cloison*** qui à partir d'une liste de cloisons dit si un carreau croise une cloison à la position où il est actuellement
* On crée une structure principale ***ZoneACouvrir*** dans le module *salon* :
  - Définie par le backend du dessin de la pièce *(et plus tard par la liste des cloisons définissant la pièce)*
  - A laquelle on implémente une fonction ***couvrir*** qui remplit la pièce de carreaux sans les couper et les ajoute au dessin principal, ligne par ligne, en colorant les surfaces non carrelées et affiche les distances "libres"
* Dans la fonction ***main*** (appelée par la commande *cargo run* il ne reste plus qu'à donner une position initiale, dessiner la pièce et couvrir la zone de carreaux => ç la fin on récupère la quantité de carreaux restante !
* Le résultat s'enregistre dans un dossier ***test > svg> salon.svg*** (dossiers créés automatiquement si non-existants)

## Pré-requis
* Installer **RUST** et **cargo**

## A savoir
* Le carrelage débute en bas à droite de la pièce

## Plus tard
* Revoir la fonction ***dessiner*** pour que la liste de cloisons soit en paramètre au lieu d'être définie dans la fonction
* Faire en sorte de donner toutes les dimensions des aires libres non carrelées pour faciliter la découpe
* Permettre à l'utilisateur de choisir un nom de fichier (voire un chemin de destination, mais pas une extension !)
* Permettre à l'utilisateur de donner la taille du carrelage et des joints
* Permettre à l'utilisateur de définir ses cloisons
