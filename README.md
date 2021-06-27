# Things to make:

## Renderer object
### Objects
 - Camera
    - Projection matrix (perspective/orthoganal)
    - View matrix (position/rotation in the world)
 - Object(s)
    - Model matrix (position/scale/rotation in the world)
    - Shader
    - Vertices
### On draw:
   - Reach in and select object model matrix and create mvp matrix
   - Bind the shader used for the object
   - Create the vertex buffer with all required information (position, ect)
   - Draw!

# Todo:
 - [ ] Link shaders with the actual renderer, and objects select what shader to
       use
 - [ ] Implement batch rendering so that any objects that are they same are
       rendered in one call (model matricies passed in to gpu, and ID of the
       matrix is stored in the vertex buffer for each vertex)
 - [ ] Point type