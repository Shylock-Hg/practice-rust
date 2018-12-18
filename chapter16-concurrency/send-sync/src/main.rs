fn main() {
        println!("Hello, world!");
}

// Send trait -- to transmite ownership of data between threads.
// Any type compused with entirely Send types is automaticaly marked as Send.
// Almost all primitive type Impl Send except raw pointers.

// Sync trait -- to reference from multi threas
// Any T impl Sync means Any &T impl Send
// Any type compused with entirely Sync types is automaticaly marked as Sync.
// Almost all primitive type Impl Sync except raw pointers.

// Manually implementing these traits involves implementing unsafe Rust code.
