# Turtle Swarm Optimization

Novel, optimization aglorithm based on the biomimicry of Testudines (:turtle:).

## Overview

The turtle swarm optimizer (TSO) is similar to the famous particle swarm optimizer (PSO) in its operation. In both optimization algorithms many small agents work toward a common goal of finding the local minimum of a function. The major advantages that TSO offers over the PSO algorthm can be summarized as the following:

 - Turtle velocity is based on the smallest reportable 64 bit floating point number. 
   - This ensures that every nook and cranny of the objective function is inspected :mag:.
 - The social and personal motivation terms are not configurable.
   - We don't know exactly what motivates turtles to do what they do, so its honest to leave these terms unweighted and equal components of the turtles velocity. Staying true to the :turtle: is very important.
   - Although this new algorithm wasn't written for performance, this design decision greatly speeds up the velocity calculations by dropping slow calls to random number generators :rocket::rocket::rocket:.
 - The TSO does not allow a user to bail out of optimization early. No. The user must wait until their presdescribed goal is obtained.
   - This is done so that end-users get the feeling of what its like to rely on turtles to do the heavy lifting :muscle:.

### Why is this unpublished?

The work is too important and publishing would only cause delays. It must be shared with the open source community.

## Collaborators Welcome

There are a few rules to collaborating on this repository.

1. You cannot remove any of the advantages of the TSO algorithm.
2. You cannot change the `TURTLE_VELOCITY` constant unless it is discussed with the original author first.
3. You cannot introduce a performance regression :watch:.
4. You cannot introduce any logging or sense of progress to maintain the most important advantage that TSO has over PSO :lock:.
5. You must be reasonable and polite in all of your interactions on this project :two_hearts:. 
6. Stay true to the :turtle:
