fn main() {
    println!("Hello, world!");
    // 1. Each object has an "owner" which is a variable binding the object
    // 2. For each object there is only one owner.
    // 3. If the owner goes out of scope the object is deleted.

    {
        let s = "some string";
        println!("{}", s);
    }
    // println!("{}", s); // the variable s is not found in this scope
    // &str are string literals, they fixed in size and are written in the binary

    // a string that is dynamic in size is the String
    {
        let s = String::from("some string");
        println!("{}", s);
        // The String type is stored in the heap.
        // <----- At this point the owner s goes out of scope and the object
        // is also dropped. And the memory in heap is freed automatically.
    }

    {
        let x = 10;
        let y = x;
        // integers are scalar data types, hence they are copied by default
        println!("{}", x);
        println!("{}", y);
    }

    {
        // The String type is a compound type, hence they are not copied
        // automatically, we need to use the "clone" method to create a new
        // object that has the same values as the original one.
        let s1 = String::from("some text");
        let s2 = s1; // <--- s2 has the ownership of the object and s1 losses the ownership
                     // println!("{}", s1); // <---- We can't use s1 from down here
    }
    {
        let s1 = String::from("some text");
        let s2 = s1.clone(); // <---- use the clone method
        println!("{} {}", s1, s2); // <---- We can't use s1 from down here
                                   // s2 is the owner of an object
    }
}
