use geo_types::{LineString, Coord};

use super::activity::Polyline;

pub fn compare(p1: &Polyline, p2: &Polyline, segments: i64) -> f64 {
    let n1 = normalize(p1, segments);
    let mut n2 = normalize(p2, segments);

    if n1.0.len() != n2.0.len() {
        return f64::MAX;
    }
    
    let mut distance = 0.0; 
    for (c1, c2) in n1.0.iter().zip(n2.0.iter_mut()) {
        distance += length(&LineString::new(vec![*c1, *c2]));
    }

    distance / (segments as f64)
}

pub fn length(p: &Polyline) -> f64 {
    let mut distance = 0.0;
    let mut c1 = None;
    for c2 in p.points() {
        if c1.is_none() {
            c1 = Some(c2);
            continue;
        }
        let a = c1.unwrap().y() - c2.y();
        let b = c1.unwrap().x() - c2.x();
        let c = ((a * a) + (b * b)).sqrt();
        distance += c;
        c1 = Some(c2);
    }
    distance
}

pub fn normalize(p: &Polyline, segments: i64) -> Polyline {
    let d = length(p) / (segments as f64);
    let mut segd = d;
    let mut c1 = None;
    let mut new = vec![];

    for c2 in p.coords() {
        if c1.is_none() {
            c1 = Some(c2);
            new.push(*c2);
            continue;
        }

        let cl = length(&LineString::new(vec![*c1.unwrap(), *c2]));

        segd -= cl;

        if segd > 0.0 {
            c1 = Some(c2);
            continue;
        }

        let ratio = segd / cl;
        let mut p;

        // while the remaining distance is less than the segment length
        while segd < d {
            if new.len() == (segments as usize) {
                return LineString::new(new);
            }
            // push point at remaining length in the same direction
            // println!("{}", ratio);
            let dx = (c1.unwrap().x - c2.x) * ratio;
            let dy = (c1.unwrap().y - c2.y) * ratio;
            p = Coord{
                x: c1.unwrap().x + dx,
                y: c1.unwrap().y + dy
            };

            new.push(p);

            segd += d;
            c1 = Some(&p);
        }
        c1 = Some(c2);
    }
    LineString::new(new)
}

#[cfg(test)]
mod test {
    use std::f64::consts::SQRT_2;

    use geo_types::{Coord, LineString};
    use polyline::decode_polyline;

    #[test]
    pub fn polyline_length_simple() {
        let polyline: LineString =
            LineString::new(vec![Coord { x: 0.0, y: 0.0 }, Coord { x: 1.0, y: 1.0 }]);
        let length = super::length(&polyline);
        assert_eq!(SQRT_2, length);

        let polyline: LineString = LineString::new(vec![
            Coord { x: 0.0, y: 0.0 },
            Coord { x: 1.0, y: 1.0 },
            Coord { x: 2.0, y: 2.0 },
        ]);
        let length = super::length(&polyline);
        assert_eq!(2.8284271247461903, length);
    }



    #[test]
    pub fn polyline_length_real() {
        let polyline = decode_polyline(POLYLINE_PORTLAND, 5).unwrap();
        let portland = super::length(&polyline);

        let polyline2 = decode_polyline(POLYLINE_PARKRUN, 5).unwrap();
        let parkrun = super::length(&polyline2);

        assert_eq!(0.1045498079107501, portland);
        assert_eq!(0.05239245566274975, parkrun);
    }

    #[test]
    pub fn normalize_smoke() {
        let polyline = decode_polyline(POLYLINE_PARKRUN, 5).unwrap();
        let normlaized = super::normalize(&polyline, 10);
        assert_eq!(10, normlaized.0.len());
    }

    #[test]
    pub fn normalize_fewer_polylines_than_grain() {
        let polyline: LineString = LineString::new(vec![
            Coord { x: 0.0, y: 0.0 },
            Coord { x: 1.0, y: 1.0 },
            Coord { x: 2.0, y: 2.0 },
            Coord { x: 3.0, y: 3.0 },
        ]);
        let normlaized = super::normalize(&polyline, 10);
        assert_eq!(10, normlaized.0.len());
    }
    #[test]
    pub fn distance_none() {
        let p1: LineString = LineString::new(vec![
            Coord { x: 0.0, y: 0.0 },
            Coord { x: 1.0, y: 1.0 },
            Coord { x: 2.0, y: 2.0 },
        ]);
        let p2: LineString = LineString::new(vec![
            Coord { x: 0.0, y: 0.0 },
            Coord { x: 1.0, y: 1.0 },
            Coord { x: 2.0, y: 2.0 },
        ]);
        let distance = super::compare(&p1, &p2, 4);
        assert_eq!(0.0, distance);
    }

    #[test]
    pub fn distance_some() {
        let p1: LineString = LineString::new(vec![
            Coord { x: 0.0, y: 0.0 },
            Coord { x: 1.0, y: 1.0 },
            Coord { x: 2.0, y: 2.0 },
        ]);
        let p2: LineString = LineString::new(vec![
            Coord { x: 1.0, y: 1.0 },
            Coord { x: 2.0, y: 2.0 },
            Coord { x: 3.0, y: 3.0 },
        ]);
        let distance = super::compare(&p1, &p2, 10);
        assert_eq!(1.4142135623730954, distance);
    }

    #[test]
    pub fn compare_different_sizes() {
        let p1: LineString = LineString::new(vec![
            Coord { x: 0.0, y: 0.0 },
            Coord { x: 1.0, y: 1.0 },
            Coord { x: 2.0, y: 2.0 },
            Coord { x: 3.0, y: 2.0 },
        ]);
        let p2: LineString = LineString::new(vec![
            Coord { x: 1.0, y: 1.0 },
            Coord { x: 2.0, y: 2.0 },
            Coord { x: 3.0, y: 3.0 },
        ]);
        let _ = super::compare(&p1, &p2, 10);
    }

    const POLYLINE_PORTLAND: &str = r#"m|ysH|h_NBr@CfBEx@I`A@FZVJ@NK\EPSLAb@SdAUFEh@@h@Fz@Rj@Hr@@h@TZHfB|@nAx@THp@n@dAr@`@\^^n@~@\X|@nA|@fB`@h@b@|@h@x@h@dAnBxBrAnAxA|Ah@f@hCxAt@Vl@Px@d@j@Tj@Pj@HVJ`@Fj@ZbAb@bAf@~BbAVPXFd@P`CfAb@JVJf@d@vCtAXJd@VhAb@`@XCjB?xBFx@H^Z@nAYrAi@l@[bAAn@@RC`FwA|A[tAm@NOh@gAH[JMh@UnAy@pC{Bd@YpCwBtByAdBwAvCoBfB{Ax@k@h@i@n@a@~CkCnA_Aj@_@rBeBv@s@~@sAn@q@vBoC\o@dAcBb@}@\}@\q@Xa@Pq@Nc@Ju@E_@QmDLs@Pq@FYv@eHZcBPo@HMnAmB|@iA^w@?o@Ms@[w@a@y@_@{@{@oECmBEk@@E\Yb@UHKh@}Av@qE^}ATgAf@aBZ{@Xk@^e@XIf@Dl@?NCFGzAwFBo@AMUm@a@o@Cs@QuAPIXYR@HEN?VUJLNWJGF?JIPa@r@s@JUHKp@e@\Oh@]VYl@g@pAsATQdAm@x@}@LEd@]NW`@QTQd@k@xAmAXGb@E\@lAYT\XDf@Ah@B`@GpAEf@GR[Xs@V_@DK^oAX_BHqAHcC@mBJaFCg@B[?eBd@wALoA@y@?SIQCAPCBEH{@CSOo@C[He@Vc@vHqHPzBDTJTf@l@V`@L^?tAFnAEREhAER}ClCCTY^Ff@VdAE^Bz@RpAV|AFTb@dA~@lA`@jANt@?v@Jh@Rl@NZZ^h@|AHJR@LARFl@|@FRHl@WfCD|@GR\bAT`@FVL@`@GVk@XSn@HTJX\FPCDB`@J`@Xh@F\BZXb@@RGl@Ol@MP_@NWTSDI@QECSEAK^UXE@UMc@e@YKKAUOITKBQC_@PQDe@^G?UHm@j@]h@q@TYV[b@[bAuAnCy@fCq@b@OFGLFBJA@@A@UBw@@s@L}@^qBzAc@DIR[PKGGMGBi@f@a@PsBX]He@Xc@?SQIAs@n@c@d@"#;
    const POLYLINE_PARKRUN: &str = r#"q~~sHr||Mg@z@Qp@Wh@Gn@YzAa@jAWhBGt@YRk@EuATe@Kk@Ie@CMUc@aDBu@~@uBXWXg@^g@lAiB^_@P]FWTu@Ra@DOf@s@R]|@kAZ@h@h@B?d@Qf@@\f@?^Ef@_@d@sAjCDZAZu@jASl@a@~BOf@Yn@Yx@Ov@Cj@Q`@oATg@Bs@Ms@GcAXGAi@c@[@EEc@?e@FeAH{@Tc@FgBUi@CSBY`@Mr@E~@CnB@bALr@DFFZ`@b@hAfDl@vBCLQTe@R{@n@_@Pc@\w@z@uA`AQ\Cr@`@zB?z@a@`A]pAGj@Sh@]Zw@f@o@n@c@|@YnA_@h@k@ZsBZc@JMFa@^_@l@QH?CNa@XG^[j@Y|@ObBMRE^[Vu@RkA\k@~BoBLUp@iDVw@DUEm@QcAMcA?UL_@~@i@h@c@^_@`@[d@Sj@c@r@}@Vi@?KQo@s@{Bk@uAg@cAIi@DuA?u@@_AD}@Hc@La@^Mf@F|ALd@Gb@QrAKhAGf@Df@\LALQDu@[mAIaAFg@R]Lm@z@{Ax@cAb@s@T]RMPSh@eBNo@Zq@nAeB`@QFB"#;
}
