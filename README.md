Maybe (aka Option) types for wurst, using tuples for storage.

Features:

- support for `vec2`, `vec3`, and `angle` (as well as the expected primitives)
- `map`/`flat_map` between supported types
- constructors that infer presence based on nullity

Example:

```wurst
import Abilities
import ClosureTimers
import GameTimer

import WurstMaybePrimitives


constant DUMMY = createUnit(players[0], 'hfoo', ZERO2, angle(0.))

/**
    Asynchronous.  Adds an effect on the provided unit, in its facing direction.
    If the unit is `null`, then a global dummy is instead utilised, and the
    angle is chosen randomly. If the provided effect is null, holy bolt is used.
*/
function expressiveEffect(unit onWho, string fx)
    let u = maybe_unit(onWho).or_else(DUMMY)
    let ang = maybe_unit(onWho)
        .map(u -> u.getFacingAngle())
        .get_or_else(() -> angle(getElapsedGameTime()))
    let path = maybe_string(fx).get_or(Abilities.holyBoltMissile)

    doPeriodicallyCounted(ANIMATION_PERIOD, 30) _cb ->
        flashEffect(path, u.getPos().polarOffset(64., ang))
```

# Contributing - how to render:

```bash
./render.sh
```
