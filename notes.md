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

# LOG-4 0.1:
## Notes:
    Time to stress test.
    Timings are looking about the same for 10 entities

# LOG-4 0.2:
## Notes:
    Rendering time for 5000 entities is incredibly bad. Clearly drawing each one individually is not the way to go.
    Note that the `render` time displayed is just the time taken for the CPU to update the positions of our particles,
    not the actual render time taken by the GPU
    I'll try to use a MeshInstance to make use of GPU instancing & help shorted the cpu component of the draw time
    Good news though: even on a debug build, we're getting about 1ms physics processing timings for 5000 entities.
    At least that part is efficient enough at the moment

# LOG-5 0.1.3:
## Notes:
    Looks like using instancing has vastly improved our performance, but gravity seems to have inverted.
    Just flipping the gravity multiplier should be a quick fix for now.
    I'll also make the particles a bit smaller since getting up to 5000 is making it hard to see what's happening.
    Also, it's clear that the pseudorandom numbers I'm using the distribute the particles aren't uniformly distributed,
    which is visible now that we're sampling 5000 of them. I won't fix this since it's not really what I'm testing.
    Let's keep pushing the numbers to see what we need to do next

# LOG-5 0.1.4:
## Notes:
    25000 particles is starting to get a bit choppy.
    Even after our rendering overhaul, it still seems to be the bottleneck.
    Also, the compression is starting to really struggle with what is effectively visual noise at this point.
    I'll have to switch my benchmark soon for the sake of the image quality, but for now, we keep pushing.
    Next, I'll use Godot's RenderServer to directly set the mesh instance buffer.
    This will save quite a few calls & copies, all while being significantly more cache efficient.
    As a side note, I'm pretty surpirsed my initial collision function has held up so far.
    It's getting up to 4ms per frame, though, so something tells me it'll be our next target.


    