// DO NOT EDIT THIS FILE. IT WAS AUTOMATICALLY GENERATED BY:
//
//   ucd-generate jamo-short-name ucd-15.0.0 --fst-dir benches/tables/fst
//
// Unicode version: 15.0.0.
//
// ucd-generate 0.2.13 is available on crates.io.

pub static JAMO_SHORT_NAME: ::once_cell::sync::Lazy<
    ::fst::Map<&'static [u8]>,
> = ::once_cell::sync::Lazy::new(|| {
    ::fst::Map::from(
        ::fst::raw::Fst::new(&include_bytes!("jamo_short_name.fst")[..])
            .unwrap(),
    )
});
