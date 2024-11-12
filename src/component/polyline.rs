use geo_types::{Coord, LineString};
use geoutils::{Distance, Location};

use log::debug;
use tui::{
    text::Span,
    widgets::canvas::{Canvas, Line},
    widgets::Widget, prelude::Buffer,
};

use crate::{
    app::App,
    store::{activity::Polyline, polyline_compare::length},
    ui::color::{gradient, Rgb},
};

use super::unit_formatter::KILOMETER_TO_MILE;

pub fn draw(
    app: &mut App,
    f: &mut Buffer,
    area: tui::layout::Rect,
) {
    let activity = app.activity.clone().unwrap();

    if activity.summary_polyline.is_none() {
        return;
    }

    if let Ok(decoded) = activity.polyline() {
        let mapped_polyline = ActivityMap::from_polyline(
            decoded,
            area.width.saturating_add(4),
            area.height.saturating_sub(4)
        );

        let length_per_split =
            mapped_polyline.length() / ((activity.distance / 1000.0) * KILOMETER_TO_MILE);

        let canvas = Canvas::default()
            .x_bounds([0.0, area.width as f64])
            .y_bounds([0.0, area.height as f64])
            .paint(|ctx| {
                let mut prev: Option<(f64, f64)> = None;
                let mut offset = 0;
                ctx.print(
                    0.0,
                    0.0,
                    Span::from(format!(
                        "{} → ",
                        app.unit_formatter
                            .distance(mapped_polyline.x_distance().meters())
                    )),
                );

                ctx.print(
                    0.0,
                    area.height as f64,
                    Span::from(
                        app.unit_formatter
                            .distance(mapped_polyline.y_distance().meters()),
                    ),
                );
                ctx.print(0.0, area.height as f64 - 2.0, Span::from("↓"));
                let mut running_length = 0.0;
                let mut next_break = length_per_split;
                let mut split: i64 = 0;
                let mut route_lines = vec![];
                let mut split_lines = vec![];

                for coord in &mapped_polyline.coords {
                    if prev.is_none() {
                        prev = Some(*coord);
                        continue;
                    }
                    let from = prev.unwrap();
                    let to = coord;

                    running_length += length(&LineString::new(vec![
                        Coord {
                            x: from.0,
                            y: from.1,
                        },
                        Coord { x: to.0, y: to.1 },
                    ]));

                    match in_selected_split(app.activity_view_state.selected_split, split) {
                        true => split_lines.push(Line {
                            x1: from.0 + 1.0,
                            y1: from.1 + 1.0,
                            x2: to.0 + 1.0,
                            y2: to.1 + 1.0,
                            color: gradient(
                                Rgb {
                                    red: 50,
                                    green: 255,
                                    blue: 255,
                                },
                                Rgb {
                                    red: 255,
                                    green: 50,
                                    blue: 255,
                                },
                                offset as f64,
                                mapped_polyline.coords.len() as f64,
                            )
                            .to_color(),
                        }),
                        false => route_lines.push(Line {
                            x1: from.0 + 1.0,
                            y1: from.1 + 1.0,
                            x2: to.0 + 1.0,
                            y2: to.1 + 1.0,
                            color: gradient(
                                Rgb {
                                    red: 0,
                                    green: 255,
                                    blue: 0,
                                },
                                Rgb {
                                    red: 255,
                                    green: 0,
                                    blue: 0,
                                },
                                offset as f64,
                                mapped_polyline.coords.len() as f64,
                            )
                            .to_color(),
                        }),
                    };

                    for line in &route_lines {
                        ctx.draw(line);
                    }
                    for line in &split_lines {
                        ctx.draw(line);
                    }

                    if running_length >= next_break {
                        next_break += length_per_split;
                        split += 1;
                    }

                    prev = Some(*to);
                    offset += 1;
                }
            });
        canvas.render(area, f);
    }
}

fn in_selected_split(selected_split: Option<i64>, split: i64) -> bool {
    match selected_split {
        Some(s) => s == split,
        None => false,
    }
}

struct ActivityMap {
    pub coords: Vec<(f64, f64)>,
    x_distance: f64,
    y_distance: f64,
}

impl ActivityMap {
    pub fn from_polyline(decoded: LineString, width: u16, height: u16) -> Self {
        let x_max = decoded
            .points()
            .map(|p| p.x())
            .reduce(f64::max)
            .unwrap_or(0 as f64);
        let x_min = decoded
            .points()
            .map(|p| p.x())
            .reduce(f64::min)
            .unwrap_or(0 as f64);
        let x_width = x_max - x_min;
        let y_max = decoded
            .points()
            .map(|p| p.y())
            .reduce(f64::max)
            .unwrap_or(0 as f64);
        let y_min = decoded
            .points()
            .map(|p| p.y())
            .reduce(f64::min)
            .unwrap_or(0 as f64);
        let y_width = y_max - y_min;

        let mut ratio = width as f64 / x_width;
        let x_diff = 0.0 - x_min;
        let y_diff = 0.0 - y_min;
        let y_length = y_width * ratio;

        if y_length > height as f64 {
            ratio = height as f64 / y_width;
        }


        let coords = decoded
            .coords()
            .map(|c| ((c.x + x_diff) * ratio, (c.y + y_diff) * ratio));


        debug!(
            target: "polyline",
            "container: {} x {}, map: {:.4} x {:.4}, norm: {:.2} x {:.2}",
            width,
            height,
            x_width,
            y_width,
            x_width * ratio,
            y_width * ratio,
        );
        ActivityMap {
            coords: coords.collect::<Vec<(f64, f64)>>(),
            x_distance: x_width,
            y_distance: y_width,
        }
    }

    pub fn to_polyline(&self) -> Polyline {
        Polyline {
            0: self
                .coords
                .iter()
                .map(|coord| Coord {
                    x: coord.0,
                    y: coord.1,
                })
                .collect(),
        }
    }

    pub fn x_distance(&self) -> Distance {
        Location::new(0.0, 0.0)
            .distance_to(&Location::new(self.x_distance, 0.0))
            .unwrap()
    }

    pub fn y_distance(&self) -> Distance {
        Location::new(0.0, 0.0)
            .distance_to(&Location::new(0.0, self.y_distance))
            .unwrap()
    }

    fn length(&self) -> f64 {
        length(&self.to_polyline())
    }
}

#[cfg(test)]
mod tests {
    use super::ActivityMap;

    #[test]
    fn test_decode_polyline() {
        let decoded = polyline::decode_polyline(
            r"s{zsHnp~MELHLAVk@PILYVO@g@PO@KBYCIEGQCBECQJGAMOAGE@?[UAEEM[EG]CCBO?G@GEAIKHQ?NUDS?UJcANe@\iCFqAq@eE[wAEIMc@MWyAqEWsAKgAECM?MHINEdAG\G?OB_@Js@\a@h@}@xAo@l@uAr@k@R_@R{@ZwA\i@HmAJi@Am@BsBEoAKeC[iB]e@S_Be@iCaAeAm@QMmBcA_@Ws@m@WMsAqAy@o@]e@_@[QS_BkAeA}@]Os@s@{@u@g@m@WQSSMEi@i@i@q@o@i@}@eAm@e@SUUOw@{@o@k@yCkDeAwAYAyAcA?UOKIKYk@]a@e@g@y@iAi@k@gA{AWS]i@Yi@[]eCqD}AoBc@s@]c@[i@o@w@u@eA]s@u@cAOc@]o@O[_AqAs@uAuAyBg@eAqFaLmA{Bq@eBMe@_@{@OQQM{AcCMMGAAHFbAHX`@l@tA`DLN`@x@d@jAl@lAL\hBtDR\LXXd@^fA\PFL`@nA^`AjBrDd@x@Zt@Zj@bBpCnA`B`BdCZ^Zd@fB~BNXDP@\FLVX\Nb@b@rC|DDDRBDBh@jAh@v@~@hBl@bA`@`Ad@n@Lb@JB\?NDFFj@l@b@n@XRj@l@x@~@TZNLbBhBtArAp@z@vArA\THNjCvBh@l@RP\^nC`Cn@t@TN`@NZl@VLn@LPHRZXTn@XRTf@RjAZf@T@HElAH\VPP?ZFZLh@HbA`@fBz@HBX?ZHAACJd@Tt@F^LTRz@`AZTPJdANZRBBAl@Qn@Ij@@VFJLHP@LAvAP@`CHP\LJJT`AE`C?tBCdA?X@Fb@Cd@LfA`@RLF`@HHZNTAXDpCt@PF`ALPLd@Lx@TpB^d@Pz@FRCl@OVOLCHMFXJRJPBJ_@fB",
            5,
        );
        let p = ActivityMap::from_polyline(decoded.unwrap(), 100, 100);
        assert_eq!(305, p.coords.len())
    }
}
