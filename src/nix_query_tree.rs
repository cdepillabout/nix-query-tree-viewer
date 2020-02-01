use ego_tree::{NodeMut, Tree};
use nom::character::complete::space1;
use nom::{do_parse, map, named, opt, tag, take_till, IResult};
use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
struct NixQueryDrv(PathBuf);

impl From<&str> for NixQueryDrv {
    fn from(item: &str) -> Self {
        NixQueryDrv(PathBuf::from(item))
    }
}

impl FromStr for NixQueryDrv {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

named!(parse_nix_query_drv<&str, NixQueryDrv>,
    map!(take_till!(char::is_whitespace), NixQueryDrv::from));

#[derive(Clone, Copy, Debug, PartialEq)]
enum Recurse {
    Yes,
    No,
}

named!(parse_recurse<&str, &str>,
    tag!("[...]"));

#[derive(Clone, Debug, PartialEq)]
struct NixQueryEntry(NixQueryDrv, Recurse);

named!(parse_nix_query_entry<&str, NixQueryEntry>,
    do_parse!(
        drv: parse_nix_query_drv >>
        opt_recurse: opt!(
            do_parse!(
                space1 >>
                parse_recurse >>
                (Recurse::Yes)
            )) >>
        (NixQueryEntry(drv, opt_recurse.unwrap_or(Recurse::No)))
    ));

struct NixQueryTree(Tree<NixQueryEntry>);

// named!(parse_nix_query_tree<&str, NixQueryTree>,
//     do_parse!(
//         top_drv: parse_nix_query_drv >>
//         space1 >>
//         (ego_tree

//     ));

named!(parse_branch_start<&str, &str>,
    tag!("+---"));

fn parse_branch(tree: &mut NodeMut<NixQueryEntry>, level: u32) -> impl Fn(&str) -> IResult<&str, ()> {
    |input| {
        let (input, _) = parse_branch_start(input)?;
    }
}

fn parse_nix_query_tree(input: &str) -> IResult<&str, NixQueryTree> {
    let (input, top_drv): (&str, NixQueryDrv) = parse_nix_query_drv(input)?;
    let (input, _) = space1(input)?;
    let top_entry = NixQueryEntry(top_drv, Recurse::No);
    let mut tree = Tree::new(top_entry);
    let (input
    Ok(("", NixQueryTree(tree)))
}

// pub fn take(count: usize) -> impl Fn(&[u8]) -> IResult<&[u8], &[u8]>
// where
// {
//   move |i: &[u8]| {
//     if i.len() < count {
//       Err(Err::Error((i, ErrorKind::Eof))
//     } else {
//       Ok(i.split_at(count))
//     }
//   }
// }

// named!(parse_inner_func_ar_2<&str, Expr>,
//     do_parse!(
//         func_ar_2: parse_func_ar_2_name >>
//         many1!(tag!(" ")) >>
//         expr1: parse_expr >>
//         many1!(tag!(" ")) >>
//         expr2: parse_expr >>
//         (Expr::FuncAr2(func_ar_2, bx(expr1), bx(expr2)))
//     ));

// named!(parse_var<&str, Expr>,
//     map!(char('x'), |_| Expr::Var));

// named!(parse_num<&str, Expr>,
//     map!(float, Expr::Num));

// named!(parse_func_ar_1_name<&str, FuncAr1>,
//     alt!(
//         tag!("ln")  => { |_| FuncAr1::Ln  } |
//         tag!("cos") => { |_| FuncAr1::Cos } |
//         tag!("sin") => { |_| FuncAr1::Sin } |
//         tag!("tan") => { |_| FuncAr1::Tan } |
//         tag!("exp") => { |_| FuncAr1::Exp }
//     ));

// named!(parse_func_ar_2_name<&str, FuncAr2>,
//     alt!(
//         tag!("+") => { |_| FuncAr2::Plus  } |
//         tag!("-") => { |_| FuncAr2::Minus } |
//         tag!("*") => { |_| FuncAr2::Times } |
//         tag!("/") => { |_| FuncAr2::Div } |
//         tag!("^") => { |_| FuncAr2::Pow }
//     ));

// named!(parse_inner_func_ar_1<&str, Expr>,
//     do_parse!(
//         func_ar_1: parse_func_ar_1_name >>
//         many1!(tag!(" ")) >>
//         expr: parse_expr >>
//         (Expr::FuncAr1(func_ar_1, bx(expr)))
//     ));

// named!(parse_inner_func_ar_2<&str, Expr>,
//     do_parse!(
//         func_ar_2: parse_func_ar_2_name >>
//         many1!(tag!(" ")) >>
//         expr1: parse_expr >>
//         many1!(tag!(" ")) >>
//         expr2: parse_expr >>
//         (Expr::FuncAr2(func_ar_2, bx(expr1), bx(expr2)))
//     ));

// named!(parse_inner_func<&str, Expr>,
//     alt!(parse_inner_func_ar_1 | parse_inner_func_ar_2));

// named!(parse_func<&str, Expr>,
//     delimited!(tag!("("), parse_inner_func, tag!(")")));

// named!(parse_expr<&str,Expr>,
//     alt!(parse_var | parse_num | parse_func));

pub fn exec_command() -> String {
    let nix_store_stdout_raw = Command::new("nix-store")
        .args(&[
            "--query",
            "--tree",
            "/nix/store/qy93dp4a3rqyn2mz63fbxjg228hffwyw-hello-2.10",
        ])
        .output()
        .expect("failed to execute nix-store")
        .stdout;

    let nix_store_stdout = String::from_utf8(nix_store_stdout_raw)
        .expect("failed to convert nix-store output to utf8");

    println!("nix-store output: {}", nix_store_stdout);

    nix_store_stdout
}

// /nix/store/qy93dp4a3rqyn2mz63fbxjg228hffwyw-hello-2.10
// +---/nix/store/pnd2kl27sag76h23wa5kl95a76n3k9i3-glibc-2.27
// |   +---/nix/store/pnd2kl27sag76h23wa5kl95a76n3k9i3-glibc-2.27 [...]
// +---/nix/store/qy93dp4a3rqyn2mz63fbxjg228hffwyw-hello-2.10 [...]

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_nix_query_drv() {
        let raw_input = "/nix/store/qy93dp4a3rqyn2mz63fbxjg228hffwyw-hello-2.10 [...]";
        let raw_path = "/nix/store/qy93dp4a3rqyn2mz63fbxjg228hffwyw-hello-2.10";
        let nix_query_drv: NixQueryDrv = raw_path.into();
        let r = parse_nix_query_drv(raw_input);
        assert_eq!(r, Ok((" [...]", nix_query_drv)));
    }

    #[test]
    fn test_parse_nix_query_entry_no_recurse() {
        let raw_input = "/nix/store/qy93dp4a3rqyn2mz63fbxjg228hffwyw-hello-2.10\n";
        let raw_path = "/nix/store/qy93dp4a3rqyn2mz63fbxjg228hffwyw-hello-2.10";
        let nix_query_entry: NixQueryEntry = NixQueryEntry(raw_path.into(), Recurse::No);
        let r = parse_nix_query_entry(raw_input);
        assert_eq!(r, Ok(("\n", nix_query_entry)));
    }

    #[test]
    fn test_parse_nix_query_entry_recurse() {
        let raw_input = "/nix/store/qy93dp4a3rqyn2mz63fbxjg228hffwyw-hello-2.10 [...]\n";
        let raw_path = "/nix/store/qy93dp4a3rqyn2mz63fbxjg228hffwyw-hello-2.10";
        let nix_query_entry: NixQueryEntry = NixQueryEntry(raw_path.into(), Recurse::Yes);
        let r = parse_nix_query_entry(raw_input);
        assert_eq!(r, Ok(("\n", nix_query_entry)));
    }
}
