### Number guessing game 
Project idea from [roadmap.sh](https://roadmap.sh/projects/number-guessing-game). Built with Rust 1.83.0. 

#### Description
The program generates a random number between 1 and 100 (inclusive) that the user needs to guess. The maximum number of guesses allowed is defined by a selected difficulty level (between four available choices), which can be changed before initiating a new round. The game keeps track of the number of attempts and time the player took to guess correctly. 

#### Example
```
Welcome to the Number Guessing Game!
I'm thinking of a number between 1 and 100.
You have a few chances to guess the correct number.


Please select the difficulty level:
0. Gaming Journalist (unlimited)
1. Easy (10 chances)
2. Medium (5 chances)
3. Hard (3 chances)
Enter your choice or q/Ctrl+C to quit at any point: 
1
Great! You have selected the Easy difficulty level.
Let's start the game!

Enter your guess: 
50
Incorrect! The number is greater than 50.
Enter your guess: 
90
Incorrect! The number is less than 90.
Enter your guess: 
75
Incorrect! The number is less than 75.
Enter your guess: 
65
Incorrect! The number is greater than 65.
Enter your guess: 
70
Incorrect! The number is less than 70.
Enter your guess: 
66
Congratulations! You guessed the correct number in 6 attempts.
Additionally, it took you 14 seconds to guess correctly.
Do you want to keep playing? [y/n]
y
Your current best score is: 6

Please select the difficulty level:
0. Gaming Journalist (unlimited)
1. Easy (10 chances)
2. Medium (5 chances)
3. Hard (3 chances)
Enter your choice or q/Ctrl+C to quit at any point: 
2
Great! You have selected the Medium difficulty level.
Let's start the game!

Enter your guess: 
q
Quitting game...
```