## Part 1: Semantics

- Let's talk about C
  - Show initing values on the stack in C
  - Show initing structs in C
  - Show copying structs in C
  - Show what copying structs is equivalent to
  - This is your basic automatically generated recursive copy operation
- Moving to C++
  - C++ just allows you to do custom functions when you create an object and when it goes out of scope
  - This allows us to write a dynamic array struct for instance, lets call it vector
  - But what about the copy? Can we just let the compiler do the same as in C?
  - (show the copy line with a question mark)
  - Now, at this line, both things point to the same memory, so they are sharing the resources! So both destructors would try to delete it!
  - Instead, C++ has to have the same constructor concept applied to copies. Note that it doesn't modify the original type, because in C, copying leaves the original intact. That might bite us later
- I like to move it move it
  - Copying like this is quite slow. What if we have a vector of vectors for example?
  - When we resize the vector, we have to deep copy every subvector over!
  - What we actually wanted was to do the c version, and copy the "values of the pointer" over. So we have to introduce a new type of constructor
  - But a mutable l value reference would just bind to the copy constructor for overload resolution... also, we don't want to accidentally reach for this one. We want to only do so intentionally. So we have to add a new type, called "Rvalue references". Binds to temporaries, and things that have been marked as "please move from me". We do that with static_cast<T&&>, more commonly known as std::move
- Move semantics are too weak.
  - Move interaction with exceptions
    - Remember how we said vector will move elements over when resizing, etc.? That's actually only true if the move constructor cannot throw an exception lmao
  - Moves are supposed to mimic copying data in C - shallow, doesn't require any allocation, just take the surface-level data within the struct itself, no indirections, but it has several key differences.
  - All of these problems stem from the fact that move semantics are _second class_ in C++. Let's go back to the C vector example:
    - In C, we copy the struct with the ptr and length over. Then, we just make sure to only call free on the new object, not the old one. How about in C++?
    - In the C++ version, we actually have to set the old one to nullptr _because we have to call both destructors at scope exit_.
    - This is _not_ what we intended. We wanted to "drop" the original object and _not_ call its destructor at all!
    - Here is the same code in Rust. In Rust, we don't have this issue! The Vec is moved here, but the compiler knows now it can ignore cleaning up the original object, and better yet, it won't allow us to use the object afterwards!
  - Move interaction with const - you can't move from const lmao. This is IMO the worst problem with move semantics.
    - shared_future
    - initializer_list
  - Copies in C don't actually copy every element over.
    - Let's look at the example code in godbolt. The assembly generated is somewhat different for the manually copying fields over versus struct copy, right? But this view is not so useful
    - Let's look instead at the LLVM IR generated. And here we see it! The struct version uses memcpy, an intrinsic that copies the bytes from one region to another!
    - Why is this fast? Well this lets the compiler use any tricks it knows to copy the bytes faster than specifically caring about the sizes of the fields. For example, watch what happens when we turn on AVX2 instructions:
    - Moves for a single object also can use memcpy. But what about vector<vector<>>?
    - We want bulk memcpy's, which is where we have to extend the concept of moving to "trivially relocatable", in other words memcpy it over and forget about the destructor from the first object. In fact, this is how it should be all along! It's crazy that we have to call destructors on moved-from objects!
- You didn't actually mean to move, anyway
  - Automatic move
  - If we construct an object once in a function, just to move/copy it out, why do we do that? The compiler should be able to prove that the object doesn't really need to exist as a temporary anyway. That is what RVO comes in
  - Be careful! RVO only elides the outermost construction!
- What can we do about it?
  - -Wmove, -Wpessimizing-move for clang
  - clang-tidy checks
    - Writing a custom clang-tidy check for vector init list for example
  - Detecting copies
    - Deleting the copy ctor
    - Marking copy ctor deprecated
    - Marking copy ctor explicit
  - Rule of 0 - just don't do it. If you need class invariants, try to use just a normal constructor. If it has one argument, please mark it explicit!