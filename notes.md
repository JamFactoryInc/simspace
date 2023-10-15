# LOG-001:
## Notes:
    Got rust & bevy ECS working with godot
    Rendering with manual draw instead of sprite nodes to save memory (and leverage ECS)

## Observations:
    Our particle is losing inertia somehow even though there's nothing in place to dampen it.
    Seems like when our particle reaches dX = 0 or x = 0, the bounds checking breaks.
    Probably has to do with the sign being removed when multiplying by zero

# LOG-002-1:
## Notes:
    Looks like I just adjusted the potition to be exactly y=0 instead of making it bounce