# Parallelism opportunities
## Computing the string
For context-free systems, the computation tree is, indeed, a tree (not sure how to express it), but I could, for each symbol, compute in parallel what it becomes after a given amount of steps. For each of the symbol it divides into, I can compute their future in parallel also. There is an opportunity for some big parallel tree.
For context-sensitive systems, we need the information about other branches of the tree to proceed, so it's at least not embarrassingly parallelizable. Maybe, if we assume the context size is bounded and/or we design some clever scheme *à la* paraconc, we can find a way to obtain a good speed-up, but I don't feel like trying it out now. (I didn't even implement context-sensitive systems so...)

## Turtle-Drawing
The drawing method is fundamentally sequential. Maybe there's a way to do it, if I do it in reverse, or if I perform transformations on the end result? I'm not sure, but in the end, the transformations would probably cancel-out the speed gain as the sequential running time is linear anyway.
-> But, maybe, thanks to the representation used by SVG (sequence of LineBy operations, so agnostic to the cumulative state), if I find a way to efficiently join partial 'data' objects or someting akin to that, I can get good speed-ups. 

## GPU
It seems unlikely that it would be either doable or worth it to run it on the gpu. 
My ultimate goal would be to have an interface with things like sliders to vary parameters like recursion depth, draw them in real-time, etc. I don't know if GPU's could be used for that or not, maybe the solution would be to by-pass SVG entirely. 