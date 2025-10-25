# self-or


Defines SoR (self-or-ref) and SoM (self-or-mut) traits that are useful in reducing code duplication and pushing forward the ownership
transfer decision from the type designer to the consumer.

## SoR and SoM Traits

`SoR<T>` trait represents two variants of type `T`:
* self: `T`, and 
* shared reference: `&T`.

It has a single method producing a shared references of `T`, which is the common behavior of both variants:

```rust ignore
fn get_ref(&self) -> &T;
```

`SoM<T>` trait is more demanding, stronger and represents the following variants of `T`:
* self: `T`, and
* mutable reference: `&mut T`.

In addition to `get_ref`, it is also capable of creating mutable references of `T`:

```rust ignore
fn get_mut(&mut self) -> &mut T;
```

## Motivation

These two simple traits can be very useful in certain situations.

Consider the following scenario:
* We are creating a type `X` which contains a field of type `Y`.
    * It is straightforward to define this as `struct X(Y)`.
* However, assume that `X` does not need to own `Y`, all it needs is to access certain fields or methods through a shared reference `&Y`.
    * In this case, we might also consider `struct X<'a>(&'a Y)`.

Which one is better? We might not know. It is possible that both have their advantages in different situations. Yet, as the designer of type `X`, we are required to decide, or duplicate code to allow for both variants.

If we don't want to care or we want to leave the choice to the consumer, we can use `SoR` trait as follows:

```rust
use self_or::SoR;

struct Y(usize);

struct X<S: SoR<Y>>(S);

impl <S: SoR<Y>> X(S) {
    fn magic_num(&self) -> usize {
        self.0.get_ref().0
    }
}

// The called can now create X that owns the Y
let y = Y(42);
let x = X(y);
assert_eq!(x.magic_num(), 42);

// or only hold a reference to Y
let y = Y(42);
let x = X(&y);
assert_eq!(x.magic_num(), 42);

```

The mutable counterpart `SoM` has the same motivation as demonstrated below.

```rust
use self_or:SoM;

struct Y(usize);

struct X<S: SoM<Y>>(S);

impl <S: SoM<Y>> X<S> {
    fn magic_num(&self) -> usize {
        self.0.get_ref().0
    }

    fn new_magic_number(&mut self, magic: usize) {
        self.0.get_mut().0 = magic;
    }
}

let y = Y(42);
let mut x = X(y);
assert_eq!(x.magic_num(), 42);
x.new_magic_number(21);
assert_eq!(x.magic_num(), 21);

let mut y = Y(42);
let mut x = X(&mut y);
assert_eq!(x.magic_num(), 42);
x.new_magic_number(21);
assert_eq!(x.magic_num(), 21);

assert_eq!(y.0, 21);

```