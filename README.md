# goat-problem-rs
Saw the first few minutes of "The Goat Problem" numberphile video and thought it would be an interesting coding exercise to help me learn rust.
The goal was to find my own solution and then finish the video to see how close I got. 

I failed 😅

The issue was a missing pow operation on some of the terms in the area_overlap calculation, and once I fixed it the approximation matched the video.

The solution is found using the bisection method: https://en.wikipedia.org/wiki/Bisection_method
It was fairly easy to implement and should work with other continuous ```fn(f64) -> f64``` functions if you want to try it on something else

numberphile video: https://www.youtube.com/watch?v=ZdQFN2XKeKI&t=122s

the program can be ran with just ```cargo run```, there are no command line arguments at the moment

I'm open to contributions and critiques if you have any!
