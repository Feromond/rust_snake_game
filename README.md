# Rust Snake Game
A very simple basic game of snake created in rust. Supports wall collisions, self collisions, and food with snake growth. Scores are recorded at the top. Highscores are tracked per game. A main menu to start the game and also after collisions.

The game has 4 different difficulties. The first three are static changes just in the speed of the snake. The last special difficulty is a dynamically changing speed based on the current score in the game.

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
        <li><a href="#v120">v1.2.0</a></li>
      </ul>
    </li>
    <li>
      <a href="#references">References</a>
    </li>
  </ol>

## About The Project


This project is intended to be a simple game version of snake created in rust. Just a way for me to learn the rust programming languge and practice it.
The goal of this project is to create a snake game with collisions, score, snake growth. Future plans would be to keep track of highscores, and to improve potentially textures and assets to be used instead of just square shapes.

<img width="868" alt="Screenshot 2024-09-09 at 10 02 37 PM" src="https://github.com/user-attachments/assets/fe54f6b9-4676-42c2-a4bf-a400a16e3e25">
<img width="867" alt="Screenshot 2024-09-09 at 9 54 55 PM 1" src="https://github.com/user-attachments/assets/296ec378-f428-42dc-b810-e74973f3d3ff">


https://github.com/user-attachments/assets/350fd480-24f2-4dbd-a21f-1bbe7465f7b5


## Release Notes:

### v0.1.1
The first initial release includes a snake game with boarder collisions, self collisions, food, and snake growth, as well as a score that increases each time you get food.
The escape key can be used to quit the game at anytime. Currently the game auto quits when you crash / die in the game. I will want to add a end screen in the future.

### v1.0.0

The full first release of this simple rust game contains all of the same features as the initial pre-release v0.1.1 but with even more. Collisions have been changed to not close the game window but to revert back to a main menu. The main manu was added as an alternative window to the playing game state which allows for time to view the highscore, and to choose whether to play the game again upon a lose, or to quit the game fully. This lead to a code restructure and also improvements to the gameplay experience related to key inputs. High-scores have also now been implemented and will track between the same memory instance of the application running. The game is bundled for mac os and will include some windows release installers or .exe as well.

For more info, checkout the [Release Notes v1.0.0](https://github.com/Feromond/rust_snake_game/releases)


### v1.2.0

The next big update to the rust snake game. This has all of the previous features but now also includes some major upgrades to the gameplay and user experience. I have added support for window resizing so that users can play on whatever screen resolutions they want, but I maintain a game border to ensure smooth gameplay. The menu has been upgraded to include new game difficulties that can be selected. There are now 4 game modes including easy, normal, hard, and special. The modes impact the speed of the snake making the game faster as it gets harder. The special mode changes the snake speed dynamically based on how much food is consumed during the round. The left over changes are to still implement and persistently save high score data. The game has been bundled for macos and there is an installer that will be provided for windows. 


## References

[GGEZ](https://ggez.rs)
