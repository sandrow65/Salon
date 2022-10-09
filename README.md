# Salon


## But  
* Créer une pièce définie par des cloisons et la dessiner
* Définir une quantité et une taille de carreaux puis remplir la pièce avec (sans les découper) en respectant l'espacement par un joint dont la longueur est initialisée
* Afficher les carreaux positionnés dans la pièce
* Mettre en évidence les zones qui nécessiteront une découpe en affichant les distances


## Description de la solution
* On créé un module nommé *salon* consitué de 2 sous-modules :
  - *definition* dans lequel on définit une structure ***Cloison*** (nom, côté haut-gauche, côté bas-droit, couleur) et ***ZoneACouvrir*** (backend de dessin et liste de cloisons) qui implémente 2 fonctions : 
    + ***dessiner*** qui dessine la pièce en fonction de la liste des cloisons
    + ***couvrir*** qui remplit la pièce de carreaux sans les couper et les ajoute au dessin principal, ligne par ligne, en colorant les surfaces non carrelées et affiche les distances "libres"
  - *materiau* dans lequel on définit une structure ***Carreau*** (position initiale du premier carreau, position courante, taille, quantité) et une fonction ***croise_cloison*** qui à partir d'une liste de cloisons dit si un carreau croise une cloison à la position où il est actuellement
* On crée une structure principale  dans le module *salon* :
  - A laquelle on implémente une fonction ***initialiser*** qui crée la liste de cloisons, puis crée et dessine la zone
* Dans la fonction ***main*** (appelée par la commande `cargo run`) il ne reste plus qu'à donner une position initiale, initialiser la pièce et couvrir la zone de carreaux => à la fin on récupère la quantité de carreaux restante !
* Le résultat s'enregistre dans un dossier ***test > svg> nom.svg*** (où ***nom*** est saisi par l'utilisateur)

## Pré-requis
<img src="https://w7.pngwing.com/pngs/114/914/png-transparent-rust-programming-language-logo-machine-learning-haskell-crab-animals-cartoon-crab.png" width="50">

* Installer ```RUST``` 
* Dépendances :
  + ```plotters = "0.3.1"```
  + ```plotters-svg = "0.3"```
  + ```plotters-backend = "0.3"```

## À savoir
* Le carrelage débute en bas à droite de la pièce
* Arbre de la librarie (sans les dépendances) :
```
dessin_salon
├── Cargo.lock
├── Cargo.toml
├── README.md
├── target
└── src
    ├── lib.rs
    ├── main.rs
    ├── salon
    │   ├── definition.rs
    │   └── materiau.rs
    └── salon.rs
```

## Aperçu

![image](https://user-images.githubusercontent.com/60396239/194763765-159e38b0-aee5-4b66-b11a-c391ce6dfd9b.png)


## Plus tard
* Faire en sorte de donner toutes les dimensions des aires libres non carrelées pour faciliter la découpe
* Permettre à l'utilisateur de donner la taille du carrelage et des joints
* Permettre à l'utilisateur de définir ses cloisons
