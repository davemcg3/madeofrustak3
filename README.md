# MadeOfRustak3

## Intro/Inspiration

This program was inspired by a [twitter exchange](https://twitter.com/davemcg3/status/1413603332627730434) between myself and [@madeofmistak3](https://twitter.com/madeofmistak3) where I was challenged to overcome my disappointment in myself for not learning Rust over 2 years ago by doing something _this weekend_ to learn something about Rust. I committed to watching [@traversymedia](https://twitter.com/traversymedia) 's [Rust Crash Course](https://www.youtube.com/watch?v=zF34dRivLOw) and then I felt the need to put the things I learned into action, so I created this small, poorly written program to do that.

## What does it do?

It takes some encoded/obfuscated data (so that it's not easy to read in the source code because I want you to download the source and run the program), decodes it, and displays the decoded data. I used a caesar cipher because it's easy to implement and is exactly the level of "hard-to-crack" that I want (i.e., not hard to crack).

## How to run

1. [Install Rust.](https://www.rust-lang.org/tools/install)
2. Navigate to the root directory
3. `cargo run`

Note: I think I developed this on rustc version 1.33, but during writing this documentation I updated to 1.53 and it works fine. 

## Concepts from the video that I implemented

 1. Print & formatting
 2. Variables
 3. Data Types
 4. Strings
 5. Tuples
 6. Arrays
 7. Vectors
 8. Conditionals
 9. Loops
10. Functions
11. Pointers & Reference
12. Structs
13. Enums

## Other concepts I touched on (still don't really understand) that weren't in the video

1. Ownership/borrower-checking
2. Lifetimes (elision)
3. The other things that I used but I don't know that I don't know

## Things I am unhappy with (not going to number these because yikes)

* Data modeling

  The way the data is broken up is terrible. I think the struct of user data is fine, but the parts about following/not following and how to display the user name are not necessarily concerns of the tweeter struct. Those should be refactored out into separate domains. There are a couple of approaches to that but the one that I'm thinking of for followership would be a relationship entity that referenced 2 users and modeled their relationship to each other. I could potentially add other interesting data around that, like the last time they interacted with each other, or the total count of likes on each others' posts, etc. The parts about how to display the username should really belong to some sort of piece of the program that's concerned about views and output, so like a View Model perhaps. This would allow me to pass in references to the other bits of data that need to be displayed to the things concerned with displaying them. I'm thinking in particular about the bug I have where the user is shown as following or not following the other user but the name is not displayed the way the user intended. Known bug, will not fix because I'll be rewriting this whole program to do it and I'm not interested in taking that on. Filing it away as a lesson and moving on.

* Ownership of data
  
  I don't know what I'm doing in Rust so I know that I'm doing ownership and borrowing of data totally incorrectly from an idiomatic perspective. It would be good, after I understand these concepts, to come back and do a v2 here to restructure all of the way data is being stored and passed around the program.  

* Modulo
  
  1.33 complained about Euclidean math not being stable when I tried the `rem_euclid` function so I switched the approach to adding one whole iteration of the modulus to bring the number positive. This worked because I know the limited data set on which the program will operate but it's not a robust approach. If `rem_euclid` is stable now it would likely be a cleaner approach.