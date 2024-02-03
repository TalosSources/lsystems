# Stuff to implement
* cleanly implement the iteration using flatmaps. then, see if that can be done in a recursive-parallel fashion (if not, have fun with scala oneliners)
* Do a flexible and modular system for storing and representing systems. (maybe parsing from string, json, ...?) 
* Support more general l-systems. I think my implementation currently covers exclusively the simplest case -> context-dependant and non-deterministic
* expose more drawing options (fine grained color, width), and be smarter about scale, and canvas dimensions.
* 3D!!!!!
* create cool l-systems myself

# Ideas to improve the logic/system/visuals
* find a smart way to animate the growth of a shape. the 2 ways I'm aware of now would be to 
    * animate through the depth iterations
        -> but that would yield steep change (and require some smart scaling, also in general the shapes just don't match)
        fundamentally, that's because the iterations don't seem to map to the growth stage of the plant (but maybe it does with appropriate scaling, idk, investigate ->todo)
    * animate through the drawing of the turtle. that is interesting (and there's one such animation on wikipedia), but it most definitely don't map at all to the growth stage
    ## better idea
    * do something smart with the stack operations to draw the shape in its logical order

* maybe add some state to the turtle to improve the drawing. example : have a depth value, that increases at each drawing step, and gets saved/reset when push/pop happen
* have some variable associated with symbols, or something like that. for example, we could have a number associated with "Turn", representing the angle (Turn(pi/4)). That could be combined with context-sensitive stuff, i.e., the angle could be a function of the angles of the surrounding symbols. (this seem like it could be approximately represented without such a feature, but that would probably lead to a way more efficient representation for algebraic like behavior).