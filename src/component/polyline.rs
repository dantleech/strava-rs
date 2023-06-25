use geo_types::{Coord, LineString, Point};
use polyline;
use tui::{
    backend::Backend,
    style::{Style, Color},
    widgets::{Axis, Block, Chart, Dataset},
    Frame,
};

use crate::app::App;

pub fn draw<B: Backend>(
    app: &mut App,
    f: &mut Frame<B>,
    area: tui::layout::Rect,
) -> Result<(), anyhow::Error> {
    if let None = app.activity {
        ()
    }
    let activity = app.activity.clone().unwrap();

    if None == activity.summary_polyline {
        ()
    }
    let polyline = activity.summary_polyline.unwrap();

    if let Ok(decoded) = polyline::decode_polyline(polyline.as_str(), 5) {
        let coords = map_coords_to_area(decoded, area.width, area.height);
        let datasets = vec![Dataset::default()
            .graph_type(tui::widgets::GraphType::Scatter)
            .data(&coords)];
        let chart = Chart::new(datasets)
            .style(Style::default().fg(Color::Red))
            .x_axis(
                Axis::default()
                    .style(Style::default().fg(Color::DarkGray))
                    .bounds([0.0, area.width as f64])
            )
            .y_axis(
                Axis::default()
                    .style(Style::default().fg(Color::DarkGray))
                    .bounds([0.0, area.height as f64])
            );
        f.render_widget(chart, area);
    }
    Ok(())
}

fn map_coords_to_area(decoded: LineString, width: u16, height: u16) -> Vec<(f64, f64)> {
    let x_max = decoded
        .points()
        .into_iter()
        .map(|p| p.x())
        .reduce(f64::max)
        .or_else(|| Some(0 as f64))
        .unwrap();
    let x_min = decoded
        .points()
        .into_iter()
        .map(|p| p.x())
        .reduce(f64::min)
        .or_else(|| Some(0 as f64))
        .unwrap();
    let x_width = x_max - x_min;
    let y_max = decoded
        .points()
        .into_iter()
        .map(|p| p.y())
        .reduce(f64::max)
        .or_else(|| Some(0 as f64))
        .unwrap();
    let y_min = decoded
        .points()
        .into_iter()
        .map(|p| p.y())
        .reduce(f64::min)
        .or_else(|| Some(0 as f64))
        .unwrap();
    let y_width = y_max - y_min;

    let boundary_size = if width > height { height } else { width } as f64;
    let size = boundary_size / if x_width < y_width { x_width } else { y_width };
    let x_diff = 0.0 - x_min;
    let y_diff = 0.0 - y_min;

    let coords = decoded
        .coords()
        .map(|c| ((c.x + x_diff) * size, (c.y + y_diff) * size));

    coords.collect::<Vec<(f64, f64)>>()
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
        let coords = map_coords_to_area(decoded.unwrap(), 100, 100);
        assert_eq!(305, coords.len())
    }
}
