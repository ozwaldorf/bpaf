//! A somewhat fancy example of a typical `bpaf` usage.

use bpaf::*;
use std::str::FromStr;

#[derive(Clone, Debug)]
#[allow(dead_code)]
enum Segment {
    Time(f64),
    SpeedDistance { speed: f64, dist: f64 },
}

fn main() {
    // Suppose we want to calcualte a total time of a travel, where parts of
    // a travel can be given either as pairs of speed and distance or just by time.
    // Speed can be given by KPH or MPH. Distance - either miles or km.

    // parsers for speeds. Both speeds are converted to the same units
    let mph = long("mph")
        .argument("SPEED")
        .parse(|s| f64::from_str(&s))
        .map(|x| x * 1.6);
    let kph = long("kph").argument("SPEED").parse(|s| f64::from_str(&s));

    // speed is either kph or mph, conversion to mph is handled by the parser
    let speed = mph.or_else(kph);

    // parsers for distances, both are converted to the same units
    let km = long("km").argument("SPEED").parse(|s| f64::from_str(&s));
    let mi = long("mi")
        .argument("SPEED")
        .parse(|s| f64::from_str(&s))
        .map(|x| x * 1.6);
    let dist = mi.or_else(km);

    // time, presumably in seconds
    let time = long("time").argument("TIME").parse(|s| f64::from_str(&s));

    // parsed time is trivially converted to time segment
    let segment_time = time.map(Segment::Time);

    // parsed speed/distance is packed into SpeedDistance segment
    let segment_speed = construct!(Segment::SpeedDistance { speed, dist });

    // segment can be either of two defined
    let segment = segment_speed.or_else(segment_time);

    // and we have several of them.
    let parser = segment
        .many()
        .guard(|x| !x.is_empty(), "need at least one segment")
        .guard(
            |x| x.len() < 10,
            "for more than 9 segments you need to purchase a premium subscription",
        );

    let descr = "You need to specify one or more travel segments, segment is defined by
a pair of speed and distance or by time.

This example defines two separate travel segments, one given by speed/distance combo and one by time
    travel --km 180 --kph 35 --time";
    let decorated = Info::default().descr(descr).for_parser(parser);

    // help message tries to explain what's needed:
    // either --time OR one speed and one distance, both can be given in miles or km.
    // number of speed flags must correspond to number of distance flags, more or
    // less results in parser error messages
    let opt = decorated.run();

    println!("{:#?}", opt);
}
