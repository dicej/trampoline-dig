#![deny(warnings)]

use clap::{App, Arg};
use failure::Error;
use std::f64::consts::PI;

fn main() -> Result<(), Error> {
    let matches = App::new("trampoline-dig")
        .about("Trampoline Dig Project Calculator")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .arg(
            Arg::with_name("trampoline height")
                .long("trampoline-height")
                .help("height of trampoline")
                .default_value("36"),
        )
        .arg(
            Arg::with_name("trampoline sides")
                .long("trampoline sides")
                .help("number of sides of regular polygon trampoline shape")
                .default_value("8"),
        )
        .arg(
            Arg::with_name("trampoline side length")
                .long("trampoline-side-length")
                .help("length of each side of regular polygon trampoline shape")
                .default_value("74"),
        )
        .arg(
            Arg::with_name("retaining wall thickness")
                .long("retaining-wall-thickness")
                .help("thickness of retaining wall")
                .default_value("8"),
        )
        .arg(
            Arg::with_name("ring sides")
                .long("ring-sides")
                .help("number of sides of regular polygon to be built around trampline")
                .default_value("8"),
        )
        .arg(
            Arg::with_name("apothem difference")
                .long("apothem-difference")
                .help("difference between the trampoline apothem plus retaining wall thickness and apothem of regular polygon to be built around trampoline")
                .default_value("36"),
        )
        .arg(
            Arg::with_name("air channel count")
                .long("air-channel-count")
                .help("number of air channels to include")
                .default_value("4"),
        )
        .arg(
            Arg::with_name("air channel width")
                .long("air-channel-width")
                .help("width of each air channel")
                .default_value("12"),
        )
        .get_matches();

    let trampoline_height = matches
        .value_of("trampoline height")
        .unwrap()
        .parse::<f64>()?;
    let trampoline_sides = f64::from(
        matches
            .value_of("trampoline sides")
            .unwrap()
            .parse::<u32>()?,
    );
    let trampoline_side_length = matches
        .value_of("trampoline side length")
        .unwrap()
        .parse::<f64>()?;
    let retaining_wall_thickness = matches
        .value_of("retaining wall thickness")
        .unwrap()
        .parse::<f64>()?;
    let ring_sides = f64::from(matches.value_of("ring sides").unwrap().parse::<u32>()?);
    let apothem_difference = matches
        .value_of("apothem difference")
        .unwrap()
        .parse::<f64>()?;
    let air_channel_count = f64::from(
        matches
            .value_of("air channel count")
            .unwrap()
            .parse::<u32>()?,
    );
    let air_channel_width = matches
        .value_of("air channel width")
        .unwrap()
        .parse::<f64>()?;

    let trampoline_apothem = trampoline_side_length / (2.0 * (PI / trampoline_sides).tan());

    let trampoline_plus_wall_apothem = trampoline_apothem + retaining_wall_thickness;
    let trampoline_plus_wall_side_length =
        trampoline_plus_wall_apothem * 2.0 * (PI / trampoline_sides).tan();
    let trampoline_plus_wall_surface_area =
        (trampoline_plus_wall_apothem * trampoline_plus_wall_side_length * trampoline_sides) / 2.0;

    let ring_apothem = trampoline_plus_wall_apothem + apothem_difference;
    let ring_side_length = ring_apothem * 2.0 * (PI / ring_sides).tan();
    let ring_surface_area = ((ring_apothem * ring_side_length * ring_sides) / 2.0)
        - (trampoline_plus_wall_surface_area
            + (((air_channel_width + (2.0 * retaining_wall_thickness)) * apothem_difference)
                * air_channel_count));

    let hole_depth =
        trampoline_height / ((trampoline_plus_wall_surface_area / ring_surface_area) + 1.0);
    let wall_height = trampoline_height - hole_depth;

    let wall_area = (((trampoline_plus_wall_side_length * trampoline_sides)
        - (air_channel_count * air_channel_width))
        * trampoline_height)
        + (((ring_side_length * ring_sides) - (air_channel_count * air_channel_width)
            + (2.0 * air_channel_count * apothem_difference))
            * wall_height)
        + (air_channel_count * air_channel_width * hole_depth);

    let volume_to_excavate = trampoline_plus_wall_surface_area * hole_depth;

    println!(
        "trampoline plus wall surface area: {} units squared\n\
         ring surface area: {} units squared\n\
         hole depth: {} units\n\
         wall height: {} units\n\
         wall area: {} units squared\n\
         volume to excavate: {} units cubed",
        trampoline_plus_wall_surface_area,
        ring_surface_area,
        hole_depth,
        wall_height,
        wall_area,
        volume_to_excavate
    );

    Ok(())
}
