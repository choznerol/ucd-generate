use std::collections::{BTreeMap, BTreeSet};

use ucd_parse::{self, GraphemeClusterBreak};

use args::ArgMatches;
use error::Result;

pub fn command(args: ArgMatches) -> Result<()> {
    let ucd_dir = args.ucd_dir()?;
    let vals: Vec<GraphemeClusterBreak> = ucd_parse::parse(&ucd_dir)?;

    let mut byval: BTreeMap<String, BTreeSet<u32>> = BTreeMap::new();
    for x in &vals {
        byval
            .entry(x.value.clone())
            .or_insert(BTreeSet::new())
            .extend(x.codepoints.into_iter().map(|c| c.value()));
    }

    let mut wtr = args.writer("grapheme_cluster_break")?;
    if args.is_present("enum") {
        wtr.ranges_to_enum(args.name(), &byval)?;
    } else {
        wtr.names(byval.keys())?;
        for (val, set) in byval {
            wtr.ranges(&val, &set)?;
        }
    }
    Ok(())
}
