use clap::ArgMatches;

pub fn bool(args: &ArgMatches, argname: &str) -> bool {
    let mut r = false;
    let n = args.occurrences_of(argname);
    if n > 0 {
        r = true;
    }
    return r;
}
