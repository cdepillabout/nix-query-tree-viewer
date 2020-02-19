use nom::character::complete::{newline, space1};
use nom::combinator::complete;
use nom::multi::{many0, many_m_n};
use nom::{alt, do_parse, eof, map, named, opt, tag, take_till, IResult};

use super::super::tree::Tree;
use super::{NixQueryDrv, NixQueryEntry, NixQueryTree, Recurse};

named!(parse_nix_query_drv<&str, NixQueryDrv>,
    map!(take_till!(char::is_whitespace), NixQueryDrv::from));

named!(parse_recurse<&str, &str>,
    tag!("[...]"));

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

pub fn nix_query_entry_parser(
    input: &str
) -> Result<NixQueryEntry, nom::Err<(&str, nom::error::ErrorKind)>> {
    parse_nix_query_entry(input).map(|(_, nix_query_entry)| nix_query_entry)
}

named!(parse_branch_start<&str, &str>,
    tag!("+---"));

named!(parse_extra_level<&str, &str>,
    alt!(tag!("|   ") | tag!("    ")));

fn parse_extra_levels(level: u32) -> impl Fn(&str) -> IResult<&str, ()> {
    move |input| {
        let (input, _) =
            many_m_n(level as usize, level as usize, parse_extra_level)(input)?;
        Ok((input, ()))
    }
}

fn parse_single_branch(
    level: u32,
) -> impl Fn(&str) -> IResult<&str, NixQueryEntry> {
    move |input| {
        let (input, _) = parse_extra_levels(level)(input)?;
        let (input, _) = parse_branch_start(input)?;
        let (input, nix_query_entry) = parse_nix_query_entry(input)?;
        let (input, _) = newline(input)?;
        Ok((input, nix_query_entry))
    }
}

fn parse_branch_with_children(
    level: u32,
) -> impl Fn(&str) -> IResult<&str, Tree<NixQueryEntry>> {
    move |input| {
        let (input, nix_query_entry) = parse_single_branch(level)(input)?;
        let (input, children) = parse_branches(level + 1)(input)?;
        Ok((input, Tree::new(nix_query_entry, children)))
    }
}

fn parse_branches(
    level: u32,
) -> impl Fn(&str) -> IResult<&str, Vec<Tree<NixQueryEntry>>> {
    move |input| {
        let (input, children) =
            many0(complete(parse_branch_with_children(level)))(input)?;
        Ok((input, children))
    }
}

fn parse_nix_query_tree(input: &str) -> IResult<&str, NixQueryTree> {
    let (input, top_drv): (&str, NixQueryDrv) = parse_nix_query_drv(input)?;
    let (input, _) = newline(input)?;
    let top_entry = NixQueryEntry(top_drv, Recurse::No);
    let (input, children) = parse_branches(0)(input)?;
    let tree = Tree::new(top_entry, children);
    Ok((input, NixQueryTree(tree)))
}

named!(parse_nix_query_tree_final<&str, NixQueryTree>,
    do_parse!(
        nix_query_tree: parse_nix_query_tree >>
        eof!() >>
        (nix_query_tree)));

pub fn nix_query_tree_parser(
    input: &str,
) -> Result<NixQueryTree, nom::Err<(&str, nom::error::ErrorKind)>> {
    parse_nix_query_tree(input).map(|(_, nix_query_tree)| nix_query_tree)
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    #[test]
    fn test_parse_nix_query_drv() {
        let raw_input =
            "/nix/store/qy93dp4a3rqyn2mz63fbxjg228hffwyw-hello-2.10 [...]";
        let raw_path = "/nix/store/qy93dp4a3rqyn2mz63fbxjg228hffwyw-hello-2.10";
        let nix_query_drv: NixQueryDrv = raw_path.into();
        let r = parse_nix_query_drv(raw_input);
        assert_eq!(r, Ok((" [...]", nix_query_drv)));
    }

    #[test]
    fn test_parse_nix_query_entry_no_recurse() {
        let raw_input =
            "/nix/store/qy93dp4a3rqyn2mz63fbxjg228hffwyw-hello-2.10\n";
        let raw_path = "/nix/store/qy93dp4a3rqyn2mz63fbxjg228hffwyw-hello-2.10";
        let nix_query_entry: NixQueryEntry =
            NixQueryEntry(raw_path.into(), Recurse::No);
        let r = parse_nix_query_entry(raw_input);
        assert_eq!(r, Ok(("\n", nix_query_entry)));
    }

    #[test]
    fn test_parse_nix_query_entry_recurse() {
        let raw_input =
            "/nix/store/qy93dp4a3rqyn2mz63fbxjg228hffwyw-hello-2.10 [...]\n";
        let raw_path = "/nix/store/qy93dp4a3rqyn2mz63fbxjg228hffwyw-hello-2.10";
        let nix_query_entry: NixQueryEntry =
            NixQueryEntry(raw_path.into(), Recurse::Yes);
        let r = parse_nix_query_entry(raw_input);
        assert_eq!(r, Ok(("\n", nix_query_entry)));
    }

    #[test]
    fn test_parse_nix_query_tree_simple() {
        let raw_input = indoc!(
            "/nix/store/qy93dp4a3rqyn2mz63fbxjg228hffwyw-hello-2.10
            +---/nix/store/qy93dp4a3rqyn2mz63fbxjg228hffwyw-hello-2.10 [...]
            "
        );
        let hello_drv: NixQueryDrv =
            "/nix/store/qy93dp4a3rqyn2mz63fbxjg228hffwyw-hello-2.10".into();
        let actual_tree = Tree::new(
            NixQueryEntry(hello_drv.clone(), Recurse::No),
            vec![Tree::singleton(NixQueryEntry(hello_drv, Recurse::Yes))],
        );

        let r = parse_nix_query_tree(raw_input);
        assert_eq!(r, Ok(("", NixQueryTree(actual_tree))));
    }

    #[test]
    fn test_parse_branch_start() {
        let raw_input = "+---";
        let r = parse_branch_start(raw_input);
        assert_eq!(r, Ok(("", "+---")));
    }

    #[test]
    fn test_parse_single_branch() {
        let raw_input = indoc!(
            "
                +---/nix/store/qy93dp4a3rqyn2mz63fbxjg228hffwyw-hello-2.10 [...]
                "
        );
        let hello_drv: NixQueryDrv =
            "/nix/store/qy93dp4a3rqyn2mz63fbxjg228hffwyw-hello-2.10".into();
        let actual_tree = NixQueryEntry(hello_drv.clone(), Recurse::Yes);

        let r = parse_single_branch(0)(raw_input);
        assert_eq!(r, Ok(("", actual_tree)));
    }

    #[test]
    fn test_parse_branch_with_children_no_children() {
        let raw_input = indoc!(
            "
                +---/nix/store/qy93dp4a3rqyn2mz63fbxjg228hffwyw-hello-2.10 [...]
                "
        );
        let hello_drv: NixQueryDrv =
            "/nix/store/qy93dp4a3rqyn2mz63fbxjg228hffwyw-hello-2.10".into();
        let actual_tree =
            Tree::singleton(NixQueryEntry(hello_drv.clone(), Recurse::Yes));

        let r = parse_branch_with_children(0)(raw_input);
        assert_eq!(r, Ok(("", actual_tree)));
    }

    #[test]
    fn test_parse_empty_branches() {
        let raw_input = "foobar";
        let actual_children = vec![];

        let r = parse_branches(0)(raw_input);
        assert_eq!(r, Ok(("foobar", actual_children)));
    }

    #[test]
    fn test_parse_nix_query_tree_simple_multi_children() {
        let raw_input = indoc!(
            "/nix/store/qy93dp4a3rqyn2mz63fbxjg228hffwyw-hello-2.10
            +---/nix/store/pnd2kl27sag76h23wa5kl95a76n3k9i3-glibc-2.27
            +---/nix/store/pnd2kl27sag76h23wa5kl95a76n3k9i3-glibc-2.27 [...]
            +---/nix/store/qy93dp4a3rqyn2mz63fbxjg228hffwyw-hello-2.10 [...]
            "
        );
        let hello_drv: NixQueryDrv =
            "/nix/store/qy93dp4a3rqyn2mz63fbxjg228hffwyw-hello-2.10".into();
        let glibc_drv: NixQueryDrv =
            "/nix/store/pnd2kl27sag76h23wa5kl95a76n3k9i3-glibc-2.27".into();
        let actual_tree = Tree::new(
            NixQueryEntry(hello_drv.clone(), Recurse::No),
            vec![
                Tree::singleton(NixQueryEntry(glibc_drv.clone(), Recurse::No)),
                Tree::singleton(NixQueryEntry(glibc_drv, Recurse::Yes)),
                Tree::singleton(NixQueryEntry(hello_drv, Recurse::Yes)),
            ],
        );

        let r = parse_nix_query_tree(raw_input);
        assert_eq!(r, Ok(("", NixQueryTree(actual_tree))));
    }

    #[test]
    fn test_parse_nix_query_tree_simple_multi_levels() {
        let raw_input = indoc!(
            "/nix/store/qy93dp4a3rqyn2mz63fbxjg228hffwyw-hello-2.10
            +---/nix/store/pnd2kl27sag76h23wa5kl95a76n3k9i3-glibc-2.27
            |   +---/nix/store/pnd2kl27sag76h23wa5kl95a76n3k9i3-glibc-2.27 [...]
            +---/nix/store/qy93dp4a3rqyn2mz63fbxjg228hffwyw-hello-2.10 [...]
            "
        );
        let hello_drv: NixQueryDrv =
            "/nix/store/qy93dp4a3rqyn2mz63fbxjg228hffwyw-hello-2.10".into();
        let glibc_drv: NixQueryDrv =
            "/nix/store/pnd2kl27sag76h23wa5kl95a76n3k9i3-glibc-2.27".into();
        let actual_tree = Tree::new(
            NixQueryEntry(hello_drv.clone(), Recurse::No),
            vec![
                Tree::new(
                    NixQueryEntry(glibc_drv.clone(), Recurse::No),
                    vec![Tree::singleton(NixQueryEntry(
                        glibc_drv,
                        Recurse::Yes,
                    ))],
                ),
                Tree::singleton(NixQueryEntry(hello_drv, Recurse::Yes)),
            ],
        );

        let r = parse_nix_query_tree(raw_input);
        assert_eq!(r, Ok(("", NixQueryTree(actual_tree))));
    }
}
