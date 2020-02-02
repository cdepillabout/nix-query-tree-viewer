extern crate nix_query_tree_viewer;

use indoc::indoc;

use nix_query_tree_viewer::nix_query_tree::*;
use nix_query_tree_viewer::nix_query_tree::parsing::*;
use nix_query_tree_viewer::tree::*;

#[test]
fn test_parse_nix_query_tree_complicated() {
    let raw_input = indoc!(
        "/nix/store/jymg0kanmlgbcv35wxd8d660rw0fawhv-hello-2.10.drv
        +---/nix/store/9krlzvny65gdc8s7kpb6lkx8cd02c25b-default-builder.sh
        +---/nix/store/m3dzp25n0g4fwlygdhvak1kk8xz906n9-bash-4.4-p23.drv
        |   +---/nix/store/58y89v7rl254dc2cygcfd5wzhv0kjm4m-bash44-013.drv
        |   +---/nix/store/9krlzvny65gdc8s7kpb6lkx8cd02c25b-default-builder.sh [...]
        |   +---/nix/store/bfil786fxmnjcwc7mqpm0mk4xnm2cphg-bootstrap-tools.drv
        |   |   +---/nix/store/b7irlwi2wjlx5aj1dghx4c8k3ax6m56q-busybox.drv
        |   |   +---/nix/store/c0sr4qdy8halrdrh5dpm7hj05c6hyssa-unpack-bootstrap-tools.sh
        |   |   +---/nix/store/drsdq2ca1q1dj1hd0r1w2hl4s0fak1vh-bootstrap-tools.tar.xz.drv
        |   +---/nix/store/64si0sfawzz464jj6qljxn1brpqw20pi-bison-3.4.2.drv
        |   |   +---/nix/store/7c0yirypq720qgj2clyanqp3b18h1lj0-bison-3.4.2.tar.gz.drv
        |   |   +---/nix/store/9krlzvny65gdc8s7kpb6lkx8cd02c25b-default-builder.sh [...]
        |   |   +---/nix/store/bfil786fxmnjcwc7mqpm0mk4xnm2cphg-bootstrap-tools.drv [...]
        |   |   +---/nix/store/xd31j9jh72b8gz4gl1h0x9fzhmr52y8c-bootstrap-stage1-stdenv-linux.drv
        |   |   |   +---/nix/store/33sl3bqjcqzrdd9clgaad3ljlwyl1pkb-patch-shebangs.sh
        |   |   |   +---/nix/store/81ikflgpwzgjk8b5vmvg9gaw9mbkc86k-compress-man-pages.sh
        |   |   |   +---/nix/store/9ny6szla9dg61jv8q22qbnqsz37465n0-multiple-outputs.sh
        |   |   |   +---/nix/store/a92kz10cwkpa91k5239inl3fd61zp5dh-move-lib64.sh
        |   |   |   +---/nix/store/bfil786fxmnjcwc7mqpm0mk4xnm2cphg-bootstrap-tools.drv [...]
        |   |   |   +---/nix/store/dis04j4z66kv6w4snapg45zwq0afcpyv-prune-libtool-files.sh
        |   |   |   +---/nix/store/dlqbw00k0w0c00iw1jkhkbpzgm3pkncw-audit-tmpdir.sh
        |   |   |   +---/nix/store/dsyj1sp3h8q2wwi8m6z548rvn3bmm3vc-builder.sh
        |   |   |   +---/nix/store/jw961avfhaq38h828wnawqsqniasqfwz-strip.sh
        |   |   |   +---/nix/store/mchwn5gbcm4wc8344bm37lismhjagr4n-setup.sh
        |   |   |   +---/nix/store/mjjy30kxz775bhhi6j9phw81qh6dsbrf-move-docs.sh
        |   |   |   +---/nix/store/ngg1cv31c8c7bcm2n8ww4g06nq7s4zhm-set-source-date-epoch-to-latest.sh
        |   |   |   +---/nix/store/pdiysv9ph2da935zpmrvc2qc0qajpqss-bootstrap-stage1-gcc-wrapper.drv
        |   |   |   |   +---/nix/store/20ayqp8yqqyk7q0n1q9gs5flksphhiz1-utils.bash
        ");
    let actual_tree = 
        NixQueryTree
            ( Tree 
                { item: NixQueryEntry
                    ( "/nix/store/jymg0kanmlgbcv35wxd8d660rw0fawhv-hello-2.10.drv".into()
                    , Recurse::No
                    ) 
                , children: 
                    vec![ Tree 
                        { item: NixQueryEntry
                            ( "/nix/store/9krlzvny65gdc8s7kpb6lkx8cd02c25b-default-builder.sh".into()
                            , Recurse::No
                            ) 
                        , children: vec![] 
                        } 
                    , Tree 
                        { item: NixQueryEntry
                            ( "/nix/store/m3dzp25n0g4fwlygdhvak1kk8xz906n9-bash-4.4-p23.drv".into()
                            , Recurse::No
                            ) 
                        , children: 
                            vec![ Tree 
                                { item: NixQueryEntry
                                    ( "/nix/store/58y89v7rl254dc2cygcfd5wzhv0kjm4m-bash44-013.drv".into()
                                    , Recurse::No
                                    ) 
                                , children: vec![] 
                                } 
                            , Tree 
                                { item: NixQueryEntry
                                    ( "/nix/store/9krlzvny65gdc8s7kpb6lkx8cd02c25b-default-builder.sh".into()
                                    , Recurse::Yes
                                    ) 
                                , children: vec![] 
                                } 
                            , Tree 
                                { item: NixQueryEntry
                                    ( "/nix/store/bfil786fxmnjcwc7mqpm0mk4xnm2cphg-bootstrap-tools.drv".into()
                                    , Recurse::No
                                    ) 
                                , children: 
                                    vec![ Tree 
                                        { item: NixQueryEntry
                                            ( "/nix/store/b7irlwi2wjlx5aj1dghx4c8k3ax6m56q-busybox.drv".into()
                                            , Recurse::No
                                            ) 
                                        , children: vec![] 
                                        } 
                                    , Tree 
                                        { item: NixQueryEntry
                                            ( "/nix/store/c0sr4qdy8halrdrh5dpm7hj05c6hyssa-unpack-bootstrap-tools.sh".into()
                                            , Recurse::No
                                            ) 
                                        , children: vec![] 
                                        } 
                                    , Tree 
                                        { item: NixQueryEntry
                                            ( "/nix/store/drsdq2ca1q1dj1hd0r1w2hl4s0fak1vh-bootstrap-tools.tar.xz.drv".into()
                                            , Recurse::No
                                            ) 
                                        , children: vec![] 
                                        } 
                                    ] 
                                } 
                            , Tree 
                                { item: NixQueryEntry
                                    ( "/nix/store/64si0sfawzz464jj6qljxn1brpqw20pi-bison-3.4.2.drv".into()
                                    , Recurse::No
                                    ) 
                                , children: 
                                    vec![ Tree 
                                        { item: NixQueryEntry
                                            ( "/nix/store/7c0yirypq720qgj2clyanqp3b18h1lj0-bison-3.4.2.tar.gz.drv".into()
                                            , Recurse::No
                                            ) 
                                        , children: vec![] 
                                        } 
                                    , Tree 
                                        { item: NixQueryEntry
                                            ( "/nix/store/9krlzvny65gdc8s7kpb6lkx8cd02c25b-default-builder.sh".into()
                                            , Recurse::Yes
                                            ) 
                                        , children: vec![] 
                                        } 
                                    , Tree 
                                        { item: NixQueryEntry
                                            ( "/nix/store/bfil786fxmnjcwc7mqpm0mk4xnm2cphg-bootstrap-tools.drv".into()
                                            , Recurse::Yes
                                            ) 
                                        , children: vec![] 
                                        } 
                                    , Tree 
                                        { item: NixQueryEntry
                                            ( "/nix/store/xd31j9jh72b8gz4gl1h0x9fzhmr52y8c-bootstrap-stage1-stdenv-linux.drv".into()
                                            , Recurse::No
                                            ) 
                                        , children: 
                                            vec![ Tree 
                                                { item: NixQueryEntry
                                                    ( "/nix/store/33sl3bqjcqzrdd9clgaad3ljlwyl1pkb-patch-shebangs.sh".into()
                                                    , Recurse::No
                                                    ) 
                                                , children: vec![] 
                                                } 
                                            , Tree 
                                                { item: NixQueryEntry
                                                    ( "/nix/store/81ikflgpwzgjk8b5vmvg9gaw9mbkc86k-compress-man-pages.sh".into()
                                                    , Recurse::No
                                                    ) 
                                                , children: vec![] 
                                                } 
                                            , Tree 
                                                { item: NixQueryEntry
                                                    ( "/nix/store/9ny6szla9dg61jv8q22qbnqsz37465n0-multiple-outputs.sh".into()
                                                    , Recurse::No
                                                    ) 
                                                , children: vec![] 
                                                } 
                                            , Tree 
                                                { item: NixQueryEntry
                                                    ( "/nix/store/a92kz10cwkpa91k5239inl3fd61zp5dh-move-lib64.sh".into()
                                                    , Recurse::No
                                                    ) 
                                                , children: vec![] 
                                                } 
                                            , Tree 
                                                { item: NixQueryEntry
                                                    ( "/nix/store/bfil786fxmnjcwc7mqpm0mk4xnm2cphg-bootstrap-tools.drv".into()
                                                    , Recurse::Yes
                                                    ) 
                                                , children: vec![] 
                                                } 
                                            , Tree 
                                                { item: NixQueryEntry
                                                    ( "/nix/store/dis04j4z66kv6w4snapg45zwq0afcpyv-prune-libtool-files.sh".into()
                                                    , Recurse::No
                                                    ) 
                                                , children: vec![] 
                                                } 
                                            , Tree 
                                                { item: NixQueryEntry
                                                    ( "/nix/store/dlqbw00k0w0c00iw1jkhkbpzgm3pkncw-audit-tmpdir.sh".into()
                                                    , Recurse::No
                                                    ) 
                                                , children: vec![] 
                                                } 
                                            , Tree 
                                                { item: NixQueryEntry
                                                    ( "/nix/store/dsyj1sp3h8q2wwi8m6z548rvn3bmm3vc-builder.sh".into()
                                                    , Recurse::No
                                                    ) 
                                                , children: vec![] 
                                                } 
                                            , Tree 
                                                { item: NixQueryEntry
                                                    ( "/nix/store/jw961avfhaq38h828wnawqsqniasqfwz-strip.sh".into()
                                                    , Recurse::No
                                                    ) 
                                                , children: vec![] 
                                                } 
                                            , Tree 
                                                { item: NixQueryEntry
                                                    ( "/nix/store/mchwn5gbcm4wc8344bm37lismhjagr4n-setup.sh".into()
                                                    , Recurse::No
                                                    ) 
                                                , children: vec![] 
                                                } 
                                            , Tree 
                                                { item: NixQueryEntry
                                                    ( "/nix/store/mjjy30kxz775bhhi6j9phw81qh6dsbrf-move-docs.sh".into()
                                                    , Recurse::No
                                                    ) 
                                                , children: vec![] 
                                                } 
                                            , Tree 
                                                { item: NixQueryEntry
                                                    ( "/nix/store/ngg1cv31c8c7bcm2n8ww4g06nq7s4zhm-set-source-date-epoch-to-latest.sh".into()
                                                    , Recurse::No
                                                    ) 
                                                , children: vec![] 
                                                } 
                                            , Tree 
                                                { item: NixQueryEntry
                                                    ( "/nix/store/pdiysv9ph2da935zpmrvc2qc0qajpqss-bootstrap-stage1-gcc-wrapper.drv".into()
                                                    , Recurse::No
                                                    ) 
                                                , children: 
                                                    vec![ Tree 
                                                        { item: NixQueryEntry
                                                            ( "/nix/store/20ayqp8yqqyk7q0n1q9gs5flksphhiz1-utils.bash".into()
                                                            , Recurse::No
                                                            ) 
                                                        , children: vec![] 
                                                        } 
                                                    ] 
                                                } 
                                            ] 
                                        } 
                                    ] 
                                } 
                            ] 
                        } 
                    ] 
                } 
            );
    let r = nix_query_tree_parser(raw_input);
    assert_eq!(r, Ok(actual_tree));
}
