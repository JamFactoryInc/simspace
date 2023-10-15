# LOG-001 1.0:
## Notes:
    Got rust & bevy ECS working with godot
    Rendering with manual draw instead of sprite nodes to save memory (and leverage ECS)

## Observations:
    Our particle is losing inertia somehow even though there's nothing in place to dampen it.
    Seems like when our particle reaches dX = 0 or x = 0, the bounds checking breaks.
    Probably has to do with the sign being removed when multiplying by zero

# LOG-2 0.1:
## Notes:
    Looks like I just adjusted the potition to be exactly y=0 instead of making it bounce

# LOG-2 0.2:
## Notes:
    Weird oscellating probably caused by per-frame multiplaction of velocity

# LOG-2 0.4:
## Notes:
    Looks like things are mostly fixed up now thanks to copying the sign of the velocity instead of the negative velocity
    There's most likely an issue with maintaining the energy of the system since gravity is being applied constantly
    while in fact for the frame where y <= 0 we aren't falling for the whole frame (because of the bounce adjustment)
    I think this will cause the energy to slowly increase, or at least not remain constant, but for now it's fine

# LOG-3 1.0:
## Notes:
    Added process timings to measure performance
    Total timings won't quite add up to the total microseconds per frame, but it'll be a good measure of how long our code is taking
    