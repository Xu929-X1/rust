# Prelude for Collections

In Rust, there are tuples and arrays that requires you to specify the size when assigning values to them. Unlike tuples and arrays, Rust provides another type of data structure, named collections, that is stored in heap. This means that when using collections, users do not need to allocate a fixed size of memory for storage, and users can grow or shrink as the program runs. Here is the list of different collection types:

- ```Vector```
- ```String```
- ```Hash Map```

And there are some other data structure we will not cover in this basics chapter, they are:

- ```Linked List```
- ```Binary Tree Map```
- ```Hash Set```
- ```Binary Tree Set```
- ```Binary Heap```

To give an overview of ```vector```, ```String```, and ```Hash Map```, here is what they are in a nutshell:

- Vector is a data structure that allows you to store a variable number of values next to each other, it is categorized under the ```Sequence```
- String is a collection of characters, remember there is ```char``` that is usually encoded in UTF-8
- Hash Map allows users to associate values to specific keys, it is a more specific implementation of a general data structure: ```map```

We will start with the ```vector``` collection type.
