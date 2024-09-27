// Shared References
// borrow check 
//   relationship between a value and it's borrowed references has been clear and known at compile time.
// The value is owner and reference cannot alive it.

// eg. we have `struct Player` and `struct Role`, a player keeps a vector ref to Role object
// 

// `Rc` works like `Box`
// Heap memory is allocated and value is moved to it.
// If you clone `Box`, it allocates a full cloned copy of the value, cloning `Rc` is cheap, 
// because on each cloned it just updates reference count to the same data.

// When `Rc` is dropped ref count is decremented. when the count goes to zero owned value is droped and memory freed.

use std::rc::Rc;

// You may make as many references as you like to the original value - it's dynamic borrowing again
// you do not have to carefully track the relationship between the value `T` and it's reference `&T`
// there is some runtime cost involved so not first solution to choose.
// but it makes patterns of sharing possible which would fall foul of the borrow checker.

// NOTE: Rc gives us immutable shared references.

fn rc_one() {
    let s = "hello Rc".to_string();
    
    let rs1 = Rc::new(s);
    let rs2 = rs1.clone();
    
    println!("{:?} {:?}", rs1, rs2);
}

// so we can make use of this for role in  player as `Vec<Rc<Role>>`  so we can role as many as we want but we can not change once created role.

// Incase need to change the role we can go for `<RefCell>` which can be change with borrow_mut.

// type signer going to be scary so we can simplify with 
// `type PlayerRef = Rc<RefCell<Player>>`;

pub fn main() {
    rc_one()
}