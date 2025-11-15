use frunk::hlist;

struct HNil;

struct HCons<Head, Tail> {
    head: Head,
    tail: Tail,
}

pub struct FString(&'static str);
pub struct FVar;

trait Format<ArgList> {
    fn format(&self, args: ArgList) -> String;
}

impl Format<HNil> for HNil {
    fn format(&self, _args: HNil) -> String {
        "".to_string()
    }
}

impl<ArgList, FmtList> Format<ArgList> for HCons<FString, FmtList>
where
    FmtList: Format<ArgList>,
{
    fn format(&self, args: ArgList) -> String {
        self.head.0.to_owned() + &self.tail.format(args)
    }
}

impl<T, ArgList, FmtList> Format<HCons<T, ArgList>> for HCons<FVar, FmtList>
where
    FmtList: Format<ArgList>,
    T: ToString,
{
    fn format(&self, args: HCons<T, ArgList>) -> String {
        args.head.to_string() + &self.tail.format(args.tail)
    }
}

fn check() {
    let example: HCons<i32, HCons<bool, HNil>> = HCons {
        head: 1,
        tail: HCons {
            head: true,
            tail: HNil,
        },
    };

    let example: frunk::HCons<
        FString,
        frunk::HCons<FVar, frunk::HCons<FString, frunk::HCons<FVar, frunk::HNil>>>,
    > = hlist![
        FString("Hello "),
        FVar,
        FString("! The first prime is "),
        FVar
    ];

    let args = hlist!["world", 2];
    // example.format(args);
}
