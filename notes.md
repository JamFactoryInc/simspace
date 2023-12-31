# LOG-1: Getting started with some simulation

## 0.1.0:
    Got rust & bevy ECS working with godot
    Rendering with manual draw instead of sprite nodes to save memory (and leverage ECS)
    Our particle is losing inertia somehow even though there's nothing in place to dampen it.
    Seems like when our particle reaches dX = 0 or x = 0, the bounds checking breaks.
    Probably has to do with the sign being removed when multiplying by zero

# LOG-2: Making a particle

## 2.0.1:
    Looks like I just adjusted the potition to be exactly y=0 instead of making it bounce

## 2.0.2:
    Weird oscellating probably caused by per-frame multiplaction of velocity

## 2.0.4:
    Looks like things are mostly fixed up now thanks to copying the sign of the velocity instead of the negative velocity
    There's most likely an issue with maintaining the energy of the system since gravity is being applied constantly
    while in fact for the frame where y <= 0 we aren't falling for the whole frame (because of the bounce adjustment)
    I think this will cause the energy to slowly increase, or at least not remain constant, but for now it's fine

# LOG-2: Basic physics

## 3.1.0:
    Added process timings to measure performance
    Total timings won't quite add up to the total microseconds per frame, but it'll be a good measure of how long our code is taking

## 4.0.1:
    Time to stress test.
    Timings are looking about the same for 10 entities

## 4.0.2:
    Rendering time for 5000 entities is incredibly bad. Clearly drawing each one individually is not the way to go.
    Note that the `render` time displayed is just the time taken for the CPU to update the positions of our particles,
    not the actual render time taken by the GPU
    I'll try to use a MeshInstance to make use of GPU instancing & help shorted the cpu component of the draw time
    Good news though: even on a debug build, we're getting about 1ms physics processing timings for 5000 entities.
    At least that part is efficient enough at the moment

# LOG-5: Optimize the fuck out of it

## 5.0.1.3:
    Looks like using instancing has vastly improved our performance, but gravity seems to have inverted.
    Just flipping the gravity multiplier should be a quick fix for now.
    I'll also make the particles a bit smaller since getting up to 5000 is making it hard to see what's happening.
    Also, it's clear that the pseudorandom numbers I'm using the distribute the particles aren't uniformly distributed,
    which is visible now that we're sampling 5000 of them. I won't fix this since it's not really what I'm testing.
    Let's keep pushing the numbers to see what we need to do next

## 5.0.1.4:
    25000 particles is starting to get a bit choppy.
    Even after our rendering overhaul, it still seems to be the bottleneck.
    Also, the compression is starting to really struggle with what is effectively visual noise at this point.
    I'll have to switch my benchmark soon for the sake of the image quality, but for now, we keep pushing.
    Next, I'll use Godot's RenderServer to directly set the mesh instance buffer.
    This will save quite a few calls & copies, all while being significantly more cache efficient.
    As a side note, I'm pretty surpirsed my initial collision function has held up so far.
    It's getting up to 4ms per frame, though, so something tells me it'll be our next target.

## 5.0.2.0:
    Well it seems to have made things slower, if anything. I didn't really expect that.
    I'm assuming it's because it's a debug build,
    and things I'd expect to be optimised are skipped for the sake of build time

## 5.0.2.2:
    Well it was certainly worth the wait!
    (Although this was the first optimized compilation, so subsequent ones will be very fast)
    Who knew optimizing your build made things go faster. I'm guessing llvm figured out how to vectorize my shitty
    physics code, and it definitely did so with the render logic
    Looks like we can still take things a little bit farther.
    I was lazy with the render buffer and let rust figure out how to allocate the vector,
    but it's clearly doing it naively & doing a bunch of re-allocations up to a capactiy of 200000 floats.
    Not surprised it's struggling
    Additionally, I'll take the liberty of slowing down the particles' acceleration and initial speed,
    so they are less like static and more observable.
    Also this will help greatly with video compression
    
## 5.0.2.3:
    I'm persisting the render buffer across frames now, and just overwriting the necessary position values
    It seems to be running about 5 times faster,
    although I feel like I might be able to squeeze just a little out of it
    Something interesting that I've started to notice is that recording the simulation adds about 100 microseconds
    to the render time.
    I'm encoding via AV1 with my CPU since my GPU doesn't support AV1, and it's rather intensive.
    I would have thought I had enough cores for it to not impact performance, but evidently not, so from now I'll be
    using NVENC to encode.
    I'll miss these beautifully small video files and readable text.

## 5.0.2.4:
    As an apples-to-apples comparison, this is the same simulation using NVENC.
    The numbers are a little hard to read,
    but they're much more representitive of the normal performance at a render time of 720-800 microseconds.
    In contrast to the AV1-recorded clip reporting over 1000 microseconds.
    I didn't expect an optimization step to be changing my OBS settings.

## 5.0.2.5:
    I discovered that the Godot glue type PackedFloat32Array does an array copy when being created from a slice,
    which makes sense, as it would be a blatent violation of Rust's borrowing rules if it didn't.
    This bascially means I was allocating 800KB worth of transform buffer each frame,
    just to convert between rust's native Vec<f32> and gdextension's PackedFloat32Array.
    Needless to say, I replaced my persistent Vector with a PackedFloat32Array directly,
    completely avoiding this glue code overhead, and look, we saved about 200 microseconds per frame.
    Easy as

## 5.1.0.1:
    Well, I got the numbers I was hoping for without ever needing to optimize my physics.
    It is a grossly over-simplified version of physics, though, where particles have no knowledge of one-another,
    and non-axis-aligned surfaces do not exist.
    Next, we'll be overhauling our physics,
    and make use of some of the CPU power that we've been working so hard to save
    Now, just for fun, I'll push the simulation as far as it can go, with absolutely no regard for video quality.

## 5.1.0.2:
    Looks Like around 250k particles we start to struggle, and at 500k I'd hesitate to call it real-time.
    I beleive at this quantity, my CPU simple doesn't have enough cache to make this efficient.
    I'm using a Ryzen 3900x, which has 64MB of L3 cache
    The mesh buffer alone is 16MB at 500k particles, and the fact that this is running on a single core means
    that we only have 512 KB of L2, and even less L1 to work with.
    Thankfully, my end-goal will probably be more cache friendly
    
    
    
    
    
    