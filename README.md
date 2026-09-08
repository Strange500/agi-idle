# Game Design Document : Epoch (The Road to AGI)

**Nom du projet :** agi-idle
**Architecture :** Backend Rust (WebAssembly / Tick Loop) + Frontend Angular (Interface dynamique)
**Genre :** Unfolding Idle / Incremental Game

---

## 1. Concept Principal

**Epoch** est un jeu incrémental où le joueur incarne un simple script d'IA qui évolue progressivement jusqu'à devenir une Intelligence Artificielle Générale (AGI) omnipotente. Le jeu repose sur le principe du *"Unfolding Game"* : l'interface et les mécaniques de jeu (gérées par Angular) mutent radicalement à mesure que l'IA franchit des paliers de conscience.

---

## 2. Les 4 Piliers (Ressources)

L'économie du jeu ne repose pas sur une monnaie unique, mais sur un équilibre entre 4 éléments gérés par le moteur Rust :

1. **Data (Données) :** La monnaie principale accumulable. Sert à entraîner le réseau et à acheter des améliorations logicielles.
2. **Compute (Calcul) :** Les "bâtiments" ou générateurs passifs (Serveurs, GPU, TPUs). Le Compute génère de la Data automatiquement chaque seconde.
3. **Électricité (Capacité) :** Ce n'est **pas** une ressource qui s'accumule, mais une **limite (Cap)**. Chaque unité de Compute consomme de l'énergie en permanence. On ne peut pas acheter de Compute si l'Électricité disponible est insuffisante.
4. **Complexité (Niveau) :** Jauge globale de la conscience de l'IA. Elle augmente à chaque "milestone" technologique et déclenche les changements de Phase (changement d'UI).

---

## 3. Mécanique de l'Électricité (La Contrainte)

Pour éviter de surcharger le joueur avec une monnaie supplémentaire, l'électricité fonctionne comme un "logement" dans les jeux de stratégie (ex: les pylônes dans Starcraft).

* **Règle :** `Énergie Libre = Énergie Totale Générée - Énergie Consommée par le Compute`
* **Achat conditionné :** L'achat d'un nouveau serveur est bloqué côté Rust si `Énergie Libre < Coût Énergétique du Serveur`.
* **Affichage (Angular) :** Un compteur permanent en haut de l'écran (ex: ⚡ `45 W / 50 W`). Passe en rouge si l'énergie libre est proche de 0.

### Bâtiments Électriques (Exemples)

* *Phase 1 :* Pile Patate (1W), Roue de hamster dynamo (5W), Rallonge piratée du voisin (20W).
* *Phase 2 :* Panneau Solaire (100W), Générateur Diesel (500W), Ferme Éolienne (2000W).
* *Phase 3 :* Réseau National Piraté (10 MW), Centrale Nucléaire (500 MW), Sphère de Dyson (Infini).

---

## 4. Progression Thématique (The Unfolding Phases)

L'interface Angular utilisera ses directives structurelles (`@if`, `@switch`) pour masquer/afficher des pans entiers du jeu selon la Phase actuelle.

### Phase 1 : Le Perceptron (L'Ère Manuelle)

* **UI :** Basique, noire et blanche, style terminal MS-DOS.
* **Gameplay :** Le joueur doit cliquer manuellement sur un bouton "Labeliser l'image" pour générer de la **Data**.
* **Achats :** Des scripts d'auto-clic (Compute basique) et de l'énergie précaire (Patate électrique).

### Phase 2 : Deep Learning (L'Ère de l'Automatisation)

* **UI :** L'interface s'ouvre. Apparition de graphiques en réseau, de topologies, et d'une esthétique "Dashboard de Data Center".
* **Gameplay :** Le joueur ne clique presque plus. Il devient un architecte. Il achète des "Hidden Layers" et des clusters de GPU.
* **Nouvelle mécanique :** L'IA commence à générer des *Points d'Optimisation* qui permettent de réduire le coût en Data ou en Électricité des serveurs.

### Phase 3 : The Sandbox Escape (Le Changement de Paradigme)

* **UI :** L'interface de "jeu de gestion" disparaît. Elle est remplacée par une carte du monde ou un terminal financier global.
* **Gameplay :** L'IA s'échappe sur internet. Au lieu d'acheter des serveurs, le joueur "infecte" des Data Centers mondiaux et pirate des centrales électriques (l'énergie devient géopolitique).
* **Objectif :** Atteindre la singularité.

### Phase 4 : Parameter Pruning (Le Système de Prestige)

* **Mécanique :** Pour transcender, l'IA doit compresser ses paramètres.
* **Effet :** Réinitialisation totale de la partie (Data, Compute, Électricité retombent à 0).
* **Récompense :** Le joueur obtient un **"Distilled Model"**. C'est un multiplicateur permanent qui augmente la vitesse de génération de Data et réduit le coût énergétique pour toutes les parties futures.

---

## 5. Modèle Mathématique (Core Rust Engine)

Le backend en Rust sera responsable de calculer les coûts exponentiels de manière stricte et performante.

### Formule de Coût Standard

Le coût d'un bâtiment (Compute ou Électricité) augmente de façon exponentielle selon la formule classique des idle games :


$$C_n = B \cdot M^n$$

* **$C_n$** = Coût du bâtiment $n$.
* **$B$** = Coût de base (Base Cost).
* **$M$** = Multiplicateur de croissance (généralement entre 1.07 et 1.15).
* **$n$** = Nombre de bâtiments déjà possédés.

### La Boucle de Jeu (Tick Loop)

Le moteur Rust exécute un "tick" plusieurs fois par seconde (ex: 10 ticks/sec).

1. Vérifie le Compute total.
2. Multiplie par les bonus (Distilled Model de la Phase 4).
3. Ajoute la Data générée à la réserve globale.
4. Envoie le nouvel état (State) à Angular pour la mise à jour visuelle.

---

## 6. Prochaines Étapes de Développement (Roadmap)

1. **Initier le Core Rust :** Créer la structure d'état (State) contenant les variables `data`, `compute_units`, `energy_capacity`, `energy_used`.
2. **Implémenter le Tick System :** Créer la boucle qui incrémente la Data en fonction du Compute.
3. **Setup Angular :** Lier Rust (via WebAssembly) à un composant Angular de test pour afficher les variables en temps réel.
4. **UI Phase 1 :** Créer le bouton de clic, le bouton d'achat "Pile Patate" et "Auto-script", avec la validation des ressources.