# 3b1b

## basics

- its really easy for humans to do certain things, and neural networks try to copy that but make it digital
- you cant solve certain problems with normal turing complete stuff like loops and conditionals and functions
- no specific instructions are given to an NN, the computer modifies itself

### structure

- a neuron is an object that holds a number (between 0 and 1)
    - this number is the activation of the neuron 
- each input and output gets its own neuron
- it assigns a value to each output, and the higher the number the more confidient it is in that value

#### hidden layers

- there can be many hidden layers with as many neurons as they want, and its up to choice
- every neuron in one layer influences the behaviour of the neurons in the next layer
    - this influence isnt the same across neurons or layers; they can vary
- layers break down big problems into small maths problems

### info transfer

- goal is a system that can turn patterns into digits
- each connection between 2 neurons has a strength (a weight)
    - this is a _parameter_, which lets us tweak the NN's properties.
