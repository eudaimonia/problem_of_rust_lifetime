struct Context<'s>(&'s str);
struct Parser<'a, 's: 'a> {
    context: &'a Context<'s>,
}
impl<'a, 's> Parser<'a, 's> {
    fn new(con: &'s Context) -> Self {
        Parser { context: con }
    }

    fn parse(&'a self) -> Result<(), &'s str> {
        Err(&self.context.0[1..])
    }
}

fn parse_context(context: Context) -> Result<(), &str> {
    // let parser = Parser::new(&context);
    // 被commented out 的statement无法编译通过，但是下面
    // 的statement却能编译通过，两者的区别在哪里呢？
    let parser = Parser { context: &context };
    parser.parse()
}

fn main() {
    let con = Context("hello");
    let res = parse_context(con);
    assert!(res.is_err());
}
