## Containers In Rust


Here’s a list of common **containers** in Rust, along with a quick snippet on how to create each:

### 1. **Vec (Vector)**
A growable, dynamically sized array.

```rust
let v: Vec<i32> = Vec::new();  // Empty vector
let v = vec![1, 2, 3];         // Vector with initial values
```

### 2. **VecDeque (Double-Ended Queue)**
A double-ended queue implemented with a growable ring buffer.

```rust
use std::collections::VecDeque;

let mut deque: VecDeque<i32> = VecDeque::new();  // Empty deque
deque.push_back(1);
deque.push_front(0);
```

### 3. **LinkedList**
A doubly-linked list.

```rust
use std::collections::LinkedList;

let mut list: LinkedList<i32> = LinkedList::new();  // Empty linked list
list.push_back(1);
list.push_front(0);
```

### 4. **HashMap**
A hash map that stores key-value pairs.

```rust
use std::collections::HashMap;

let mut map: HashMap<&str, i32> = HashMap::new();  // Empty hashmap
map.insert("apple", 3);
map.insert("banana", 2);
```

### 5. **BTreeMap**
A sorted map implemented as a binary search tree.

```rust
use std::collections::BTreeMap;

let mut map: BTreeMap<i32, &str> = BTreeMap::new();  // Empty BTreeMap
map.insert(3, "apple");
map.insert(1, "banana");
```

### 6. **HashSet**
A hash set, a collection of unique values.

```rust
use std::collections::HashSet;

let mut set: HashSet<i32> = HashSet::new();  // Empty hash set
set.insert(1);
set.insert(2);
```

### 7. **BTreeSet**
A sorted set implemented as a binary search tree.

```rust
use std::collections::BTreeSet;

let mut set: BTreeSet<i32> = BTreeSet::new();  // Empty BTreeSet
set.insert(1);
set.insert(3);
```

### 8. **BinaryHeap**
A binary max-heap priority queue.

```rust
use std::collections::BinaryHeap;

let mut heap: BinaryHeap<i32> = BinaryHeap::new();  // Empty binary heap
heap.push(1);
heap.push(10);
```

### 9. **Array**
A fixed-size collection of elements.

```rust
let arr: [i32; 3] = [1, 2, 3];  // Array with 3 elements
```

### 10. **Tuple**
A finite heterogeneous collection.

```rust
let t: (i32, f64, &str) = (42, 3.14, "hello");  // Tuple with 3 elements
```

### 11. **String**
A growable, UTF-8 encoded string.

```rust
let s: String = String::new();  // Empty string
let s = String::from("Hello");  // String with initial value
```

### 12. **&[T] (Slice)**
A dynamically sized view into a contiguous sequence.

```rust
let arr = [1, 2, 3];
let slice: &[i32] = &arr[1..];  // Slice of the array
```

### 13. **Box**
A smart pointer for heap allocation.

```rust
let boxed_value = Box::new(5);  // Allocate an integer on the heap
```

### 14. **Rc (Reference Counted)**
A single-threaded reference-counting pointer.

```rust
use std::rc::Rc;

let rc_value = Rc::new(5);  // Reference-counted value
```

### 15. **Arc (Atomic Reference Counted)**
A thread-safe reference-counting pointer.

```rust
use std::sync::Arc;

let arc_value = Arc::new(5);  // Thread-safe reference-counted value
```

### 16. **RefCell**
A type that enforces borrowing rules at runtime.

```rust
use std::cell::RefCell;

let ref_cell_value = RefCell::new(5);  // Mutable value in RefCell
```

### 17. **Mutex**
A thread-safe mutual exclusion primitive.

```rust
use std::sync::Mutex;

let mutex_value = Mutex::new(5);  // Value protected by a mutex
```

### 18. **RwLock**
A reader-writer lock allowing concurrent reads and exclusive write access.

```rust
use std::sync::RwLock;

let rwlock_value = RwLock::new(5);  // Value protected by RwLock
```
