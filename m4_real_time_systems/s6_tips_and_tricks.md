# Tips and Tricks
See which one of these can be moved into other sections, quite a few can be moved to m4::s2

* Check/validate everything before the hot loop
* Allocations in a hot loop
* Object Pools
* System calls - hoist out of the hot loop, use buffered file readers and writers
* Don't use abbreviations Don't talk moon man language to me, ya Blargon!
* When to care about software engineering and when to care about performance
* Don't use a string key/identifier or integer, when a type safe enum will do the job, can be a hassle when using
bindings, in which case it might be better with a string or integer
* Hard coding types
* Cognitive load, and delaying errors to after the first draft - deliberate development vs. debugging
* Use version control even for solo development
* Testing and Seeding RNG's [Faster RNG](https://youtu.be/5_RAHZQCPjE)
* Multi-resolution computing for making your real-time target, video streaming and image loading
* Static/Dynamic Dispatch - dyn, enum, trait, limitations good!
* The Markov Chain
* If using recursion and risking stack overflow, use a loop and a queue
* If you are on an HPC file system, it is likely faster to have a few really big files, load them into memory and get your files from that mapped file instead of having > 100 files and loading them individually
