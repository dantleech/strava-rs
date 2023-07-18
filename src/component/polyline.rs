use geo_types::LineString;
use geoutils::Location;

use tui::{
    backend::Backend,
    text::Span,
    widgets::{
        canvas::{Canvas, Line},
    },
    Frame,
};

use crate::{
    app::App,
    ui::color::{gradiant, Rgb},
};

pub fn draw<B: Backend>(
    app: &mut App,
    f: &mut Frame<B>,
    area: tui::layout::Rect,
) -> Result<(), anyhow::Error> {
    if app.activity.is_none() {
        
    }
    let activity = app.activity.clone().unwrap();

    if activity.summary_polyline.is_none() {
        
    }

    if let Ok(decoded) = activity.polyline() {
        let (coords, x_width, y_width) =
            map_coords_to_area(decoded, area.width - 4, area.height - 4);
        let x_distance_meters = Location::new(0.0, 0.0)
            .distance_to(&Location::new(x_width, 0.0))
            .unwrap();
        let y_distance_meters = Location::new(0.0, 0.0)
            .distance_to(&Location::new(0.0, y_width))
            .unwrap();

        let canvas = Canvas::default()
            .x_bounds([0.0, area.width as f64])
            .y_bounds([0.0, area.height as f64])
            .paint(|ctx| {
                let mut prev: Option<(f64, f64)> = None;
                let mut offset = 0;
                for coord in &coords {
                    if prev.is_none() {
                        prev = Some(*coord);
                        continue;
                    }
                    let from = prev.unwrap();
                    let to = coord;

                    ctx.print(
                        0.0,
                        0.0,
                        Span::from(format!(
                            "{} → ",
                            app.unit_formatter
                                .distance(x_distance_meters.meters() as f32)
                        )),
                    );

                    ctx.print(
                        0.0,
                        area.height as f64,
                        Span::from(
                            app.unit_formatter
                                .distance(y_distance_meters.meters() as f32),
                        ),
                    );
                    ctx.print(0.0, area.height as f64 - 2.0, Span::from("↓"));

                    ctx.draw(&Line {
                        x1: from.0 + 1.0,
                        y1: from.1 + 1.0,
                        x2: to.0 + 1.0,
                        y2: to.1 + 1.0,
                        color: gradiant(
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
                            coords.len() as f64,
                        )
                        .to_color(),
                    });
                    prev = Some(*to);
                    offset += 1;
                }
            });
        f.render_widget(canvas, area);
    }
    Ok(())
}

fn map_coords_to_area(decoded: LineString, width: u16, height: u16) -> (Vec<(f64, f64)>, f64, f64) {
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

    let boundary_size = if width > height { height } else { width } as f64;
    let size = boundary_size / if x_width < y_width { x_width } else { y_width };
    let x_diff = 0.0 - x_min;
    let y_diff = 0.0 - y_min;

    let coords = decoded
        .coords()
        .map(|c| ((c.x + x_diff) * size, (c.y + y_diff) * size));

    (coords.collect::<Vec<(f64, f64)>>(), x_width, y_width)
}

#[cfg(test)]
mod tests {
    use super::map_coords_to_area;

    #[test]
    fn test_decode_polyline() {
        let decoded = polyline::decode_polyline(
            r"s{zsHnp~MELHLAVk@PILYVO@g@PO@KBYCIEGQCBECQJGAMOAGE@?[UAEEM[EG]CCBO?G@GEAIKHQ?NUDS?UJcANe@\iCFqAq@eE[wAEIMc@MWyAqEWsAKgAECM?MHINEdAG\G?OB_@Js@\a@h@}@xAo@l@uAr@k@R_@R{@ZwA\i@HmAJi@Am@BsBEoAKeC[iB]e@S_Be@iCaAeAm@QMmBcA_@Ws@m@WMsAqAy@o@]e@_@[QS_BkAeA}@]Os@s@{@u@g@m@WQSSMEi@i@i@q@o@i@}@eAm@e@SUUOw@{@o@k@yCkDeAwAYAyAcA?UOKIKYk@]a@e@g@y@iAi@k@gA{AWS]i@Yi@[]eCqD}AoBc@s@]c@[i@o@w@u@eA]s@u@cAOc@]o@O[_AqAs@uAuAyBg@eAqFaLmA{Bq@eBMe@_@{@OQQM{AcCMMGAAHFbAHX`@l@tA`DLN`@x@d@jAl@lAL\hBtDR\LXXd@^fA\PFL`@nA^`AjBrDd@x@Zt@Zj@bBpCnA`B`BdCZ^Zd@fB~BNXDP@\FLVX\Nb@b@rC|DDDRBDBh@jAh@v@~@hBl@bA`@`Ad@n@Lb@JB\?NDFFj@l@b@n@XRj@l@x@~@TZNLbBhBtArAp@z@vArA\THNjCvBh@l@RP\^nC`Cn@t@TN`@NZl@VLn@LPHRZXTn@XRTf@RjAZf@T@HElAH\VPP?ZFZLh@HbA`@fBz@HBX?ZHAACJd@Tt@F^LTRz@`AZTPJdANZRBBAl@Qn@Ij@@VFJLHP@LAvAP@`CHP\LJJT`AE`C?tBCdA?X@Fb@Cd@LfA`@RLF`@HHZNTAXDpCt@PF`ALPLd@Lx@TpB^d@Pz@FRCl@OVOLCHMFXJRJPBJ_@fB",
            5,
        );
        let (coords, _x_width, _y_width) = map_coords_to_area(decoded.unwrap(), 100, 100);
        assert_eq!(305, coords.len())
    }
}
