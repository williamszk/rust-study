
# How do we work with building images?

In building images we use the binaries produced by the host machine.

In my case the host "machine" is itself a docker container, which I'll call 
the host container. This is relevant because we can just send the executable to
the images.

Then we build the executable on the host and then just copy it to the new image.

---

About spinning up containers from inside another container. 

The host container has access to the host machine's images and containers.

It is not a container creating child containers but creating sibling containers.


