# 👨🏼‍💻 Exercises
These exercises are supposed to get you to write some Rust code yourself. Advent of Code is a good place to start
for learning the basic syntax and stuff like reading from files. You'll quickly see though that you start focusing
more and more on the problems themselves instead on Rust. At this point you can learn more about the challenges
of structure and the borrow checker through doing either m0::e1::a or m0::e1::b. One is more visually inclined,
wheras the other is more neural network oriented.

## m0::e0 - Advent of Code
In order to get a hang of the basic of Rust I recommend doing [Advent of Code][1]
problems until you feel comfortable moving forward. You can give yourself a bigger challenge by rewriting your
solutions to use iterators whenever possible.

## m0::e1::a - Ray Tracing in One Weekend
Doing this exercise will be quite challenging if you don't know how to read C++. To better practice
structure, smart pointers, dynamic dispatch and traits, I recommend moving on to doing
[Ray Tracing in One Weekend][2] in Rust. There is a code snippet which shows your image on screen
instead of writing it to file in ```m1_memory_hierarchies::code::image_on_screen``` or [online][0].

Once you have completed this part 1 out of 3, I suggest modifying the resulting code in the following ways -

* Remove the use of ```dyn```
* Remove the use of smart pointers
* Parallelize computation of pixels through ```rayon```

??? note "Hints"

    * You can remove the use of dyn through using enums to represent the material interface
    * My solution to remove smart pointers was to use indices and a geometry service provided
    through dependency injection
    * Formulate your rendering as a base iterator that calls the inner function with the correct
    pixel indices, then trade .iter() for .par_iter() 

## m0::e1::b - Learning to Fly
If you are more inclined towards neural networks, [Learning to Fly][3], which focuses on neural networks and genetic
algorithms, might be more stimulating. The code is already in Rust and should be fairly easy to follow.

## m0::e1 - Traits!
Find out which traits are needed for using BURN

[0]: https://github.com/absorensen/the-guide/tree/main/m1_memory_hierarchies/code/image_on_screen
[1]: https://adventofcode.com/
[2]: https://raytracing.github.io/books/RayTracingInOneWeekend.html
[3]: https://pwy.io/posts/learning-to-fly-pt1/
