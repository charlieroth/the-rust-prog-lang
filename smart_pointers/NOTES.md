# Smart Pointers
_Smart pointers_ are data structures that act like a pointer but also have
additional metadata and capabilities

References only borrow data while smart pointers _own_ the data they point to

SP are usually implemented using structs and implement the `Deref` and
`Drop` traits

`Deref` trait allows an instance of a SP struct to behave like a reference so code
can work with either references or SP

`Drop` trait allows customization of code that runs when an instance of the SP
goes out of scope


#### Most Common Smart Pointers
- `Box<T>`, allocates value on the heap
- `Rc<T>`, a reference to a counting type that enables ownership
- `Ref<T>` and `RefMut<T>` accessed through `RefCell<T>`
- `RefCell<T>` a type that enforces the borrowing rules at runtime instead of 
compile time


## Interior Mutability
Pattern where an immutable type exposes an API for mutating an interior value


## Reference Cycles
How they can leak memory and prevention


## `Box<T>`

Store data on the heap rather than stack

No performance overhead other than storage on heap
