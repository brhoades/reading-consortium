# Agenda and topics
## Opening: What's a reference?
### Summary
 * References are nonowning "pointers" to some data.
 * References can't outlive referrants.
 * Reference safety is asserted at compile-time via lifetimes.
 * This is the hard part of Rust.

Example shows a author <-> vec of works hashmap.

Takeaways:
 * Moves prevent further use of a value.
  * Moves effectively destroy passed values when going down in scope.
 * *Shared references* are immutable; you can have as many as you life.
   * &value
 * *Mutable references* are exclusive; they aren't Copy and you can have only one.
   * &mut value
 * Constant, immutable references are implicit; mutability is explict.
 * References avoid the need to move and lose the value in the parent scope.

### Additional topics / resources

## References as Values
### Summary
 * References are machine addresses; they ain't your mother's C++ references.
 * References are not used explicitly; there's no guesswork in pass by value vs pass by reference.
 * . operator implicitly dereferences the left operand, as deep as needed. It also implicitly borrows.
 * * operator explicitly dereferences reference values.
 * Assignment to references update the addresses, not the value. C++ does the opposite.
 * References of references are a thing (&&v).
 * Comparison operators implicitly dereference operands, as long as their types are equal (&v == &z).
 * References can't be null, they're Option<T&> at the machine level, None is a nullptr (transparently).
 * References to slices or traits are fat pointers.
   * The former is described last chapter.
   * The latter is dynamic dispatch (right?); it's a pointer to the impl. and the data object.
 
### Additional topics / resources

## Reference Safety (lifetimes)
### Lifetimes
Lifetimes can be lexical blocks, statements, and expressions.
Compile-time parachutes against memory safety issues. Not around at runtime.

Review pg 102 example.
#### Example
Constraints:
 * &x's lifetime can't leave innermost block.
 * r's lifetime must be at least its final assignment of &x to its dereferencing.

#### Discuss

### References in Functions
 * Statics must be initialized. Mutable statics are not thread safe and must be accessed by threads in unsafe blocks.
 * Statics are life-of-the-program lifetimes. "Global" lifetimes.

#### Example (receiving)
Multiple renditions of trying to pass a static lifetime value into a function which takes a smaller lifetime.

Summary:
 * Explicit lifetimes look like tick-prefixed generics.
 * Can't pass a lifetime with a greater scope ('static) into a more limited one ('a).
 * "Fix" the function by using `<'static>`
 * Function signature _must_ match the behavior of the body.

#### Example (passing)
Summary:
 * Lifetime parameters are only needed in function signatures and type definitions.
 * When being used, Rust will infer variable lifetimes from the call / initialization.

#### Example (returning)
Function works!

Summary:
 * Returned value's lifetime must match the lifetime of the passed, source value.
 * Can't use returned values to violate lifetime rules of values; hoisting isn't allowed.

### References in Structs
If a reference is in a type definition, it must have an explicit lifetime.

Summary (Example):
 * S can only take r's which are statics.
 * Nested structs with references must pass lifetimes from the enclosing struct to the enclosed.

### Other lifetime parameters

Summary:
 * (Distinct lifetime parameters): You can share lifetimes between objects, but it's limiting.
   * Shared lifetimes parameters must have matching variable lifetimes; you can't mismatch scopes.
   * Function signatures and structs behave the same way here.
 * In simple cases, Rust will define as many lifetime parameters as it needs.
   * It'll not share lifetime parameters typically?

### <Discussion>

## Sharing Versus Mutation
This section details other ways to make dangling pointers.

Summary:
 * Moving values to separate variables, then using the original value, can cause this.
   * Explicit scoping helps. The shared reference must have expired.
 * (pg 115) Vector resizing causes slices to be dangling pointers after reallocation.
   * Can't insert on the vector while a shared reference is alive.
   * Java hashtable about iterators throwing crap when you change the hashtable.
 * Again: shared access is read-only, mutable access exclusive.
   * Java HashTable violates ^.
 * (pg 119) Collections that support unrestricted, simultaneous iteration is difficult, complex, and likely slow(er).
   * std::map (BT) promises iterators aren't affected by inserts; this is at a cost.
 * C++ assignment operator example that makes me cry.
   * Rust doesn't let this if you wanted to write it.
   * See also: overlapping memcpy/strcpy.

### Example: C++ pointer to const vs Rust shared reference.
Rust locks the referrant down, pointer to const just creates a variable without locking down the source.
### <Discussion>

## Taking Arms Against a Sea of Objects
Summary:
 * Garbage compilers are super common. 
 * Quick to spin up and hack, horrible to maintain.
 * Rust ownership is a speedbump on that highway to hell.

### <Discussion>


## Merge sort
There's two distinct, high-level approaches: in place and not?

### <Discussion>
### Topics and Resources













**Raw notes below:**
# Chapter 3: References
Pointers like Box<T> for heaps and Vec/String values are owning: when they are dripped the referant goes with it.

*References* have no effects on their referant's lifetimes. In fact, it's opposite: references can't outlive referant's lifetimes.
The rules that govern reference lifetimes are what take the most effort and understanding to master in Rust.

## Renaissance Example
Table: HashMap of name of artist to array of works.

pg94 we have a show function which iterates over the table to print the artist then loops over their works to print those.
Constructing the table is fairly straightfoward. We insert with static vectors.

Keeping in mind the previous chapter on moves, you'll notice the call to show at the bottom of main is a move. You can't do anything
further once you do it because the value was moved.

If you dig further, you can see our show function did the same thing internally: it destroyed table by unpacking its contents and then unpacked the vectors
of those contents too. Table is gone.


This is where references come into play.
*Shared Reference*: immutable reference. You can have as many as you like. Syntax: &e (ref e). They are Copy.
*Mutable Reference*: exclusive read/write access to a reference. Cannot have more than one at a time. Not Copy.

Going back to our example, we don't need to wrtie to the table so an immutable reference works the whole way down.
We pass the table with a reference value that's a lot like how you get a pointer from a variable in C. Method signatures are updated to reflect this an all is well.


### Alphabetize work of artists.
If you wanted to modify the table, you must pass a mutable reference in. Syntax adds a mut in front of the original type and passed value. This lets us sort works as we please.

### Summary
Where initially we were passing table by value, that is we moved the value in to the function and lost it, now we passed the value by reference.

### Questions, thoughts, comments?

## References as Values
### Refs vs C++ Refs
Rust refs are just addresses at the machine-level, but they feel different from C++ refs.

In C++, refs are created implicitly and dereferenced implicitly too; very c++. Rust has them be very explicit for dereferencing and making references.
Mutability in rust is also explicit.

The . operator in Rust implicitly dereferencs the left operand, as-needed. You can also manually dereference ahead of time if you are so inclined.
. operator frequently implicitly borrows references to the left operand as too, for calls with self.

### Assigning references
On assignment, mutable rust references update the address not the value. C++ does the opposite.

### References to references
Can have reference references, and . will dereference all the way down.

### Comparing references
Rust's comparison operators see through references too as long as types are the same. Can't compare a &r to &&&r.

To compare addresses, std::ptr::eq allows you to compare the memory addresses.

### References are never null
nullptr doesn't exist for references nor are there default values. Use Option<T&> if you want "nullable" references.
Some(r) would be non-null, None would be null reference.

At the machine-level, None is a nullptr, so Option<T&> is as-efficient-as C/C++ NULL/nullptr.


### Borrowing references to arbitrary shit
Rust lets you borrow a reference to anything at all, unlike C++.

You can have an anonymous variable's reference stored. "r" on pg 100 is a reference to an anonymous variable that'll die at end of scope.
&1009 for example dies at the end of the call to assert_eq!.

Rust will protect you.

### References to Slices and Trait objects
slices / traits: Fat pointers. two-word values that carry an address AND some metadata.

Ref to a slice: start address of slice + length. Chap 3.
Trait object: reference to a value that impl's a specific trait. ~ dynamic dispatch ~.
  It contains a pointer to the trait object itself and a pointer to the implemtantion for that value, appropriate to the trait.

Trait objects / slices are just like errthing else above; they don't own the value so they can't outlive their referants.

TODO: C++ analogy of reference to an abstract class vs a Rust trait ref?

## Reference Safety
Contrasts Rust references, which walk and talk like C(++) pointers, by showing how they prevent shooting yourself in the foot.

*[NOTE]* We should step-by-step these examples with the group and discuss them. They're important.

### Borrowing local var
Can't hoist values out of scopes via borrowing. This ain't JS/Python/Ruby.

x is in th emiddle block and can't be pulled out of it without it being deallocated by the inner block / being null.

#### Lifetimes (pg 102)
We go into details of how rust prevents this from happening: lifetimes. Lifetimes can be lexical blocks, statements, expressions, or explicit.
  These are compile-time parachutes which evaporate when built. You can't inspect lifetimes at runtime (easily, I'm sure you could weave yourself a sick web if you wanted).

there are 3 lifetimes in this example:

```rust
{
    let r;
    {
        let x = 1;
        r = &x;
    }

    assert_eq!(*r, 1);
}
```

Constraints:
&x can't leave the innermost block. This is a lifetime constraint that the book describes as limiting "how large can the lifetime be."

```rust
// ...
        r = &x;
    }

    assert_eq!(*r, 1);
// ...
```
Lifetime of r must exceed anything in the scope outlined above. Book describes this lifetime constraint as limiting "how small can the lifetime be."


In our example, the compiler tries to find a lifetime that covers the innermost block while simultaneously allowing the final assert_eq to be called. This doesn't exist.


The book closes up by a simpler example, showing a reference to a vector element that can't exceed the scope of the referrant.

### Receiving references as parameters
How does rust make sure functions uses values passed as references safely?

Statics: rust's globals; the only thing global about them is their lifetime is the program (not their visibility).
Rules:
 * Statics must be initialized.
 * Mutable statics are not thread-safe. These statics must be accessed by threads in an unsafe block.

Explicit lifetimes shown now. 'a "Tick A". We're defining a function which takes a value of p with a lifetime of 'a. 'a must be the smallest enclosing lifetime for f to work out.
  However, since it's a static, it has 'static lifetime; so this is a no-go.
  But it should be able to do this since you aren't going to make a 'static variable be a dangling pointer?

If we explicitly make the function have a 'static lifetime as the first parameter, it works. 

The function's signature must match the body's behavior. We were modifying a reference that was passed, so it was STATIC and should have been labeled static?

### Passing references as arguments
Lifetime parameters are only necessary when defining function signatures and types. On use, Rust will infer them.

Our example has a function identical to what we previously started with. We tried to make it take a static instead of a smaller lifetime and it errors when passing a non-static value.

### Returning references
Example for a fn which takes a slice and returns a reference to an element.
  It's inferred that the returned value must not have a lifetime which exceeds that of the parameter.

This function works. If you attempt to misue it by hoising a value out of the parameter, Rust catches it.
Lifetimes are used to catch this.

### Structs containing References
When a reference type appears inside a type definition, you must explicitly write out its lifetime.
We have an example of a "S" which can only take "r"s which are statics vs lifetimes which are tied to the enclosing struct.

Nested structs must also pass lifetime parameters from the enclosing to the child struct, should the child struct contain references.

### Distinct lifetime parameters
Example where we defined a struct with two references with matching lifetimes. This is limiting: the passed references must live in the same scope for this to work.
  Using 'b and 'a makes this possible.

This also applies to function signatures, where you pass references with matching lifetimes; this can be too tight too.

### Omitting lifetime parameters
In the simplest case, rust doesn't need lifetime specified. It'll do it for you.

## Sharing Versus Mutation
There are other ways, outside of lifetimes (hoisting specifically?) that you can make dangling pointers.

Moving values to separate variables, then attemping to use the original value, can do this. Rust stops this and complains about borrowing.
  This example is fixed by explicitly scoping the borrowed value.

---
pg 115 ex 2 (extend vec)
Vector resizing causes referring slices to be dangling pointers after reallocation.

Muh java has this in the hashtable doc. Iterators do bad crap when you modify the hashtable.


Rust's two things here on sharing:
 * shared access is read-only
 * mutable access ie exclulsive.
 
Extend violates the second.

Ownership tree is neat, showing how shared references mark swathes of the ownership tree RO.

---
pg 119
Designling collections that support unrestricted, siumultaneous iteration is difficult and precludes simpler and efficient implementations

C++'s std::map (binary tree) promises that iterators aren't invalidated via inserts. But that promise makes it less efficient and simple than BTreeMap.
  idk about "multiple entries in each node?"

Ugh C++ assignment operator. Yeah it does something dumb on self-assign.
  Rust example, which isn't idiomatic, that errors because you can't mutably operate _and_ borrow itself.
  
This is analogous to C++ bugs around memcpy/strcpy overlap.

### Rust's shared references vs C's pointer to const
`*const int` prevents mutable changes to the int via the pointer, but it doesn't prevent changes of what it's pointing at elsewhere.
In rust, shared reference forbids _all_ changes to the referrant until the lifetime of the reference expires.

## Taking Arms Against a Sea of Objects
Since garbage ~compilers~ collectors arose in '90s, the sea of objects has become very common. 

**Tears**. Quick to spin up code, but goddawful to maintain and easy to justify rewriting.

Rust ownership is a speedbump on the highway to hell.
