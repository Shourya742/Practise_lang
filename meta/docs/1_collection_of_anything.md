# Collect of Anything

How do we have a collection of arbitrary types in rust?

# Dynamic collection of Any thing

We can actually have many collections of elements of arbitrary types byt defining our elements as trait objects of they `Any` trait.


```rust
use std::any::Any;

let mut any_vec: Vec<Box<dyn Any>> = vec![];
any_vec.push(Box::new(42));
any_vec.push(Box::new(true));
any_vec.push(Box::new('x'));
any_vec.push(Box::new("foo"));
```

Our vector contains four elements of types `i32`, `bool`, `char` and `&str`.

We cannot do much with elements of `Vec<Box<dyn Any>>` because `Any` does not have any abilities :)All we know about an element is its type id. 
This might suffice to make it useful, but only with advanced dynamic techniques.

```rust
let element = any_vec.pop().unwrap();
println!("{:?}", element.type_id());
```

# Statically-typed collection of anything

The `queue` module of the meta crate defines three types:

* `StQueue` trait defines meta information and push operation of queues.
* `QueueSingle` is the first queue implementation to represent a queue with one element.
* `Queue` is a multiple-element (>=2) queue.

Let's see how we could define a collection of four elements of type `i32`, `bool`, `char` and `&str`.

```rust
use meta::queue::*;
let queue = Queue::new(42).push(true).push('x').push("foo");
```

Now, this is quite different from the `any_vec`.

It is statically typed in the sense that you may observe the types of its elements from its signature. The type of the queue is `Queue<i32, Queue<bool, Queue<char, QueueSingle<&'static str>>>>`.

You may keep pushing elements to the queue. Unlike pushing to a vec that requires `&mut self`, pushing to the queue requires `self`.

This is nice, the function is pure and the signature allows chaining push calls.

But also this is the only way because everytime we push to the queue, its type changes. The following break down of the calls reveals the
changes in the queue type.

```rust
use meta::queue::*;

let queue: QueueSingle<i32> = queue.push(42);
let queue: Queue<i32, QueueSingle<bool>> = queue.push(true);
let queue: Queue<i32, Queue<bool, QueueSingle<char>>> = queue.push('x');
```

Since we know the types of its elements, we can use them with their natural properties. First, we need to represent the queue with two
components:
* `front`: This is the element in the front of the queue, the item to be popped.
* `back`: This is the queue containing all elements except for the one in the front.

You may then access the third element of the queue with queue.back().back().front().

```rust
use meta::queue::*;

let mut queue = Queue::new(42).push(true).push('x').push("foo");

let num = queue.front() * 2;
assert_eq!(num, 84);

*queue.back_mut().front_mut() = false;

assert_eq!(queue.back().back().front(), &'x');

*queue.back_mut().back_mut().back_mut().front_mut() = "bar";
```

Since it is a queue, we can pop elements from the front, which breaks the queue into two pieces: (i) its front or the popped element, and (ii) its
back or the remaining queue.

```rust
use meta::queue::*;

let queue = Queue::new(42).push(true).push(`x`).push("foo");

let (num, queue) = queue.pop();
assert_eq!(num, 42);

let (flag, queue) = queue.pop();
assert_eq!(flag, true);

let (ch, queue) = queue.pop();
assert_eq!(ch, `x`);

let name = queue.pop(); // no remaining queue left.
assert_eq!(name, "foo"); 
```

# Interpretation as an ad-hoc struct

To recap the queue:
* it can hold elements of arbitrary types,
* its type signature defines the types of all of its elements.

This is nothing but a `struct`.


Under the hood, `MyStruct` and `MyQueue` are equivalent. And `push` calls are equivalent to setting the fields of a struct.

```rust
use meta::queue::*;

struct MyStruct(i32, bool, char, &'static str);
let my_struct = MyStruct(42, true, 'x', "foo");

type MyQueue = Queue<i32, Queue<bool, Queue<char, QueueSingle<&'static str>>>>;
let my_queue = Queue::new(42).push(true).push('x').push("foo");
```

The difference is that `MyStruct` is a named type while `MyQueue` are equivalent. `QueueSingle` and `Queue` together can represent
all possible structs. Therefore, it can be considered as an ad-hoc struct.

Now consider, a type `A` with three fields `i32, bool, char`; and another type B including A and an additional field `&str`. Queues are
useful for representing such incremental differences, converting one type to another.

```rust
use meta::queue::*;

type A =  Queue<i32, Queue<bool, QueueSingle<char>>>;
type B = <A as StQueue>::PushBack<'static str>;

let a: A = Queue::new(42).psuh(true).push('x');
let b: B = Queue::new(42).push(true).push('x').push("foo");
let b: B = a.push("foo");
```

So far, this has been fun with types!

You might also notice that:

* we could've represented ad-hoc structs as a stack, rather than a queue so that we could go `to A from B`.
* or even better, as a double-ended queue so that we could go in both directions.
* or we could've just worked on extending capabilities of tuples with macros, which are already meant to represent ad-hoc structs.

