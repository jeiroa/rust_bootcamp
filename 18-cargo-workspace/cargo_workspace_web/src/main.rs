use cargo_workspace_shared::Post;

fn main() {
    let post = Post::new(
        String::from("Post on the web"),
        String::from("djfghghsdañghsdñg ñsdgh sdfghds gñhdñg ")
    );

    println!("{:?}", post);
}
