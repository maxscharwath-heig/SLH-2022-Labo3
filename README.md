# Labo SLH - KING
> Maxime Scharwath

## Problèmes corrigés

### Mot de passe en clair dans le code
Pour résoudre ce problème, j'ai ajouté premièrement la liste des professeurs dans notre base de données. 
`db.json` Dans cette liste, j'ai ajouté un champ `password` qui contient le mot de passe hashé avec argon2.

### Utilisation d'un `Option<Teacher>` au lieu d'un 'boolean'
J'ai utilisé un `Option<Teacher>` pour savoir si le professeur est connecté ou non.
Ce qui me permettra de savoir si le professeur est connecté ou non tout au long de l'application,
et de pouvoir logguer les actions de du professeur.

### Mot de passe pour les élèves
Les notes étant des informations sensibles, il faut que seulement l'élève concerné puisse les voir.

Quand un élève veut voir ses notes, il doit d'abord se connecter avec son nom et son mot de passe.
Pour résoudre ce problème, j'ai ajouté une liste d'élèves dans la base de données, comme pour les professeurs.
Dans cette liste, j'ai ajouté un champ `password` et `grades` qui contient le mot de passe hashé avec argon2 et les notes de l'élève.

L'élève connecté n'est pas persité dans l'application, il doit se connecter à chaque fois qu'il veut voir ses notes.

### Logs
J'ai ajouté un logger qui permet de logguer les actions des professeurs et des élèves.
Les logs sont stockés dans un fichier `king.log` à la racine du projet.
Ce qui permet de garder une trace de toutes les actions, sans avoir à les afficher dans la console.

Pour logguer des actions liées à un utilisateur, j'ai anonymisé les données sensibles ( username ) en les hashant avec sha256.
Ce qui par la suite permet de retrouver les actions d'un utilisateur en cherchant son hash dans le fichier de log.

### Aller plus loin
J'ai utilisé la crate `rpassword` pour cacher le mot de passe quand l'utilisateur le tape.

J'aurais pu ajouter un système de logout pour les professeurs. 
Cela permettrait de ne pas avoir à redémarrer l'application pour se connecter avec un autre professeur.
Pour cela, j'aurais du passer le mutable `Option<Teacher>` et de le rendre `None` quand le professeur se déconnecte.

## Données de démo

### Professeurs pour la démo ( oups j'ai leaké les mots de passe )
danono - 3lves4ndH0b1ts
duc - l4crypt0C3stR1g0l0

### Elèves pour la démo ( oups j'ai encore leaké les mots de passe )
maxime - +mysup3rPassword+
nico - Argon2Hash
