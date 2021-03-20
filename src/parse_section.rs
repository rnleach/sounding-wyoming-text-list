//! Parse a SoundingText into a (sounding_analysis::Sounding, HashMap<&'static str, f64>).
use crate::text_iter::SoundingText;
use metfor::{Celsius, HectoPascal, Kelvin, Knots, Meters, Quantity, WindSpdDir};
use optional::{none, Optioned};
use sounding_analysis::{Sounding, StationInfo};
use std::collections::HashMap;

macro_rules! parse_token {
    ($len:expr, $start:expr, $end:expr, $line:expr, $units:tt, $res_vec:expr) => {
        let val: Optioned<$units> = if $end <= $len {
            (&$line[$start..$end])
                .trim()
                .parse::<f64>()
                .ok()
                .map($units)
                .into()
        } else {
            none()
        };
        $res_vec.push(val);
    };
}

pub fn parse(
    text: SoundingText,
    source_description: &str,
) -> Option<(Sounding, HashMap<&'static str, f64>)> {
    let (pres, hgt, temp, dp, theta_e, wind) = parse_profile(text.upper_air)?;

    let station_info = parse_station_info(text.stn_info_and_indexes)?;
    let valid_time = parse_valid_time(text.stn_info_and_indexes)?;
    let provider_data = parse_indexes(text.stn_info_and_indexes);

    let sounding = Sounding::new()
        .with_source_description(source_description.to_string())
        .with_station_info(station_info)
        .with_valid_time(valid_time)
        .with_pressure_profile(pres)
        .with_height_profile(hgt)
        .with_temperature_profile(temp)
        .with_dew_point_profile(dp)
        .with_theta_e_profile(theta_e)
        .with_wind_profile(wind);

    Some((sounding, provider_data))
}

macro_rules! parse_f64_and_add {
    ($key:expr, $val:expr, $map:ident) => {
        if let Ok(val) = $val.parse::<f64>() {
            $map.insert($key, val);
        }
    };
}

fn parse_indexes(text: &str) -> HashMap<&'static str, f64> {
    const LAT_KEY: &str = "Station latitude";
    const LON_KEY: &str = "Station longitude";
    const SHOW_KEY: &str = "Showalter index";
    const LI_KEY: &str = "Lifted index";
    const LIV_KEY: &str = "LIFT computed using virtual temperature";
    const K_KEY: &str = "K index";
    const CT_KEY: &str = "Cross totals index";
    const VT_KEY: &str = "Vertical totals index";
    const TT_KEY: &str = "Totals totals index";
    const CAPE_KEY: &str = "Convective Available Potential Energy";
    const CAPEV_KEY: &str = "CAPE using virtual temperature";
    const CIN_KEY: &str = "Convective Inhibition";
    const CINV_KEY: &str = "CINS using virtual temperature";
    const LCLT_KEY: &str = "Temp [K] of the Lifted Condensation Level";
    const LCLP_KEY: &str = "Pres [hPa] of the Lifted Condensation Level";
    const LCLTE_KEY: &str = "Equivalent potential temp [K] of the LCL";
    const POT_KEY: &str = "Mean mixed layer potential temperature";
    const MR_KEY: &str = "Mean mixed layer mixing ratio";
    const THICK_KEY: &str = "1000 hPa to 500 hPa thickness";
    const PW_KEY: &str = "Precipitable water [mm] for entire sounding";
    const BR_KEY: &str = "Bulk Richardson Number";
    const BR_CAPV_KEY: &str = "Bulk Richardson Number using CAPV";
    const ELEV_KEY: &str = "Station elevation";
    const SWEAT_KEY: &str = "SWEAT index";

    let mut provider_anal = HashMap::new();

    for (key, value_str) in text.lines().filter_map(|line| {
        line.find(':')
            .map(|idx| line.split_at(idx))
            .map(|(key, val)| (key.trim(), val.trim()))
            .map(|(key, val)| (key, val[1..].trim()))
    }) {
        match key {
            LAT_KEY => parse_f64_and_add!(LAT_KEY, value_str, provider_anal),
            LON_KEY => parse_f64_and_add!(LON_KEY, value_str, provider_anal),
            SHOW_KEY => parse_f64_and_add!(SHOW_KEY, value_str, provider_anal),
            LI_KEY => parse_f64_and_add!(LI_KEY, value_str, provider_anal),
            LIV_KEY => parse_f64_and_add!(LIV_KEY, value_str, provider_anal),
            K_KEY => parse_f64_and_add!(K_KEY, value_str, provider_anal),
            CT_KEY => parse_f64_and_add!(CT_KEY, value_str, provider_anal),
            VT_KEY => parse_f64_and_add!(VT_KEY, value_str, provider_anal),
            TT_KEY => parse_f64_and_add!(TT_KEY, value_str, provider_anal),
            CAPE_KEY => parse_f64_and_add!(CAPE_KEY, value_str, provider_anal),
            CAPEV_KEY => parse_f64_and_add!(CAPEV_KEY, value_str, provider_anal),
            CIN_KEY => parse_f64_and_add!(CIN_KEY, value_str, provider_anal),
            CINV_KEY => parse_f64_and_add!(CINV_KEY, value_str, provider_anal),
            LCLT_KEY => parse_f64_and_add!(LCLT_KEY, value_str, provider_anal),
            LCLP_KEY => parse_f64_and_add!(LCLP_KEY, value_str, provider_anal),
            LCLTE_KEY => parse_f64_and_add!(LCLTE_KEY, value_str, provider_anal),
            POT_KEY => parse_f64_and_add!(POT_KEY, value_str, provider_anal),
            MR_KEY => parse_f64_and_add!(MR_KEY, value_str, provider_anal),
            THICK_KEY => parse_f64_and_add!(THICK_KEY, value_str, provider_anal),
            PW_KEY => parse_f64_and_add!(PW_KEY, value_str, provider_anal),
            BR_KEY => parse_f64_and_add!(BR_KEY, value_str, provider_anal),
            BR_CAPV_KEY => parse_f64_and_add!(BR_CAPV_KEY, value_str, provider_anal),
            ELEV_KEY => parse_f64_and_add!(ELEV_KEY, value_str, provider_anal),
            SWEAT_KEY => parse_f64_and_add!(SWEAT_KEY, value_str, provider_anal),
            _ => {
                //dbg!(key, value_str);
                continue;
            }
        }
    }

    provider_anal
}

fn parse_valid_time(text: &str) -> Option<chrono::NaiveDateTime> {
    let str_val = text
        .find("Observation time:")
        .map(|idx| text.split_at(idx).1)
        .and_then(|sub_str| sub_str.find(':').map(|idx| (sub_str, idx)))
        .map(|(sub_str, idx)| sub_str.split_at(idx + 1).1)
        .and_then(|sub_str| sub_str.lines().next())
        .map(|vt_str| vt_str.trim())?;

    let mut year = str_val[..2].parse::<i32>().ok()?;
    if year < 70 {
        year += 2000;
    } else {
        year += 1900;
    }

    let month = str_val[2..4].parse::<u32>().ok()?;
    let day = str_val[4..6].parse::<u32>().ok()?;
    let hour = str_val[7..9].parse::<u32>().ok()?;

    let vt = chrono::NaiveDate::from_ymd(year, month, day).and_hms(hour, 0, 0);

    Some(vt)
}

fn parse_station_info(text: &str) -> Option<StationInfo> {
    let mut station_num: Option<i32> = None;
    let mut station_id: Option<String> = None;
    let mut latitude: Option<f64> = None;
    let mut longitude: Option<f64> = None;
    let mut elevation: Option<Meters> = None;

    for (key, value_str) in text.lines().filter_map(|line| {
        line.find(':')
            .map(|idx| line.split_at(idx))
            .map(|(key, val)| (key.trim(), val.trim()))
    }) {
        let value_str = value_str[1..].trim();

        if station_id.is_some()
            && station_num.is_some()
            && latitude.is_some()
            && longitude.is_some()
            && elevation.is_some()
        {
            break;
        }
        match key {
            "Station identifier" => station_id = Some(value_str.to_string()),
            "Station number" => station_num = Some(value_str.parse::<i32>().ok()?),
            "Station latitude" => latitude = Some(value_str.parse::<f64>().ok()?),
            "Station longitude" => longitude = Some(value_str.parse::<f64>().ok()?),
            "Station elevation" => elevation = Some(value_str.parse::<f64>().ok().map(Meters)?),
            _ => continue,
        }
    }

    let location = latitude.and_then(|lat| longitude.map(|lon| (lat, lon)));

    Some(StationInfo::new_with_values(
        station_num,
        station_id,
        location,
        elevation,
    ))
}

fn parse_profile(
    text: &str,
) -> Option<(
    Vec<Optioned<HectoPascal>>,
    Vec<Optioned<Meters>>,
    Vec<Optioned<Celsius>>,
    Vec<Optioned<Celsius>>,
    Vec<Optioned<Kelvin>>,
    Vec<Optioned<WindSpdDir<Knots>>>,
)> {
    let mut pressure: Vec<Optioned<HectoPascal>> = Vec::with_capacity(100);
    let mut temperature: Vec<Optioned<Celsius>> = Vec::with_capacity(100);
    let mut dew_point: Vec<Optioned<Celsius>> = Vec::with_capacity(100);
    let mut theta_e: Vec<Optioned<Kelvin>> = Vec::with_capacity(100);
    let mut wind: Vec<Optioned<WindSpdDir<Knots>>> = Vec::with_capacity(100);
    let mut height: Vec<Optioned<Meters>> = Vec::with_capacity(100);

    for line in text
        .lines()
        .skip_while(|line| !line.trim().starts_with(|c: char| c.is_digit(10)))
    {
        let len = line.len();

        // pressure 0..7
        const PSTART: usize = 0;
        const PEND: usize = 7;
        parse_token!(len, PSTART, PEND, line, HectoPascal, pressure);

        // hgt 8..14
        const HSTART: usize = 8;
        const HEND: usize = 14;
        parse_token!(len, HSTART, HEND, line, Meters, height);

        // temp 15..21
        const TSTART: usize = 15;
        const TEND: usize = 21;
        parse_token!(len, TSTART, TEND, line, Celsius, temperature);

        // dwpt 22..28
        const DPSTART: usize = 22;
        const DPEND: usize = 28;
        parse_token!(len, DPSTART, DPEND, line, Celsius, dew_point);

        // drc 45..49
        const DIR_START: usize = 45;
        const DIR_END: usize = 49;
        let direction: Option<f64> = if DIR_END <= len {
            (&line[DIR_START..DIR_END]).trim().parse::<f64>().ok()
        } else {
            None
        };

        // spd 52..56
        const SPD_START: usize = 52;
        const SPD_END: usize = 56;
        let speed: Option<Knots> = if SPD_END <= len {
            (&line[SPD_START..SPD_END])
                .trim()
                .parse::<f64>()
                .ok()
                .map(Knots)
        } else {
            None
        };

        let speed_dir: Optioned<WindSpdDir<Knots>> = direction
            .and_then(|dir| {
                speed.map(|spd| WindSpdDir {
                    speed: spd,
                    direction: dir,
                })
            })
            .into();
        wind.push(speed_dir);

        // theta_e 65..70
        const THETA_E_START: usize = 65;
        const THETA_E_END: usize = 70;
        parse_token!(len, THETA_E_START, THETA_E_END, line, Kelvin, theta_e);
    }

    if !has_min_num(&pressure) || !has_min_num(&height) {
        return None;
    }

    Some((pressure, height, temperature, dew_point, theta_e, wind))
}

fn has_min_num<'a, I, T>(i: I) -> bool
where
    I: IntoIterator<Item = &'a Optioned<T>>,
    T: Quantity + optional::Noned + 'a,
{
    const MIN_NUM_LEVELS: usize = 5;

    i.into_iter()
        .filter(|opt| opt.is_some())
        .enumerate()
        .any(|(c, _)| c >= MIN_NUM_LEVELS)
}
