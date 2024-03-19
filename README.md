# Rust Snake Game
A very simple basic game of snake created in rust. Supports wall collisions, self collisions, and food with snake growth. Scores are recorded at the top. Highscores are tracked per game. A main menu to start the game and also after collisions.

<!-- TABLE OF CONTENTS -->

  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
    </li>
    <li>
      <a href="#release-notes">Release Notes</a>
      <ul>
        <li><a href="#v011">v0.1.1</a></li>
        <li><a href="#v100">v1.0.0</a></li>
      </ul>
    </li>
    <li>
      <a href="#references">References</a>
    </li>
  </ol>

## About The Project


This project is intended to be a simple game version of snake created in rust. Just a way for me to learn the rust programming languge and practice it.
The goal of this project is to create a snake game with collisions, score, snake growth. Future plans would be to keep track of highscores, and to improve potentially textures and assets to be used instead of just square shapes.

![Screenshot 2024-03-19 at 2 34 01 PM](https://github.com/Feromond/rust_snake_game/assets/53460081/7eddd3f0-ad81-4675-bf7b-b203a6670fb5)
![Screenshot 2024-03-19 at 2 32 05 PM](https://github.com/Feromond/rust_snake_game/assets/53460081/fc663a1d-b50e-4467-ba1b-bca7676699f2)


https://github.com/Feromond/rust_snake_game/assets/53460081/b3208f23-f60d-4da6-bd8d-c99e0a74654d


## Release Notes:

### v0.1.1
The first initial release includes a snake game with boarder collisions, self collisions, food, and snake growth, as well as a score that increases each time you get food.
The escape key can be used to quit the game at anytime. Currently the game auto quits when you crash / die in the game. I will want to add a end screen in the future.

### v1.0.0

The full first release of this simple rust game contains all of the same features as the initial pre-release v0.1.1 but with even more. Collisions have been changed to not close the game window but to revert back to a main menu. The main manu was added as an alternative window to the playing game state which allows for time to view the highscore, and to choose whether to play the game again upon a lose, or to quit the game fully. This lead to a code restructure and also improvements to the gameplay experience related to key inputs. High-scores have also now been implemented and will track between the same memory instance of the application running. The game is bundled for mac os and will include some windows release installers or .exe as well.

For more info, checkout the [Release Notes v1.0.0](https://github.com/Feromond/rust_snake_game/releases)

## References

[GGEZ](https://ggez.rs)
