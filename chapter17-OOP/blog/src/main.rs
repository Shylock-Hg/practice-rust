fn main() {
        let mut post = blog::Post::new();

        post.add_text("Hello World!");
        // assert_eq!("", post.content());

        let post = post.request_review();
        let post = post.approve();
        println!("{}", post.content());
}
