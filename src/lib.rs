pub use std::ops::Range;

pub fn search(range: Range<f64>, ltz: f64, eps: f64, f: impl Fn(f64) -> f64) -> f64 {
    assert!(ltz > 0.0);
    assert!(eps > 0.0);

    let twoltz = 2.0 * ltz;
    let invtwoltz = 1.0 / twoltz;

    let mut pts = vec![
        (range.start, f(range.start)),
        (range.end, f(range.end)),
    ];
    let mut minp = (
        invtwoltz * (f(pts[0].0) - f(pts[1].0) + ltz * (pts[0].0 + pts[1].0)),
        0.5 * (f(pts[0].0) + f(pts[1].0) + ltz * (pts[0].0 - pts[1].0)),
    );
    let mut delta = invtwoltz * (f(minp.0) - minp.1);

    while twoltz * delta > eps {
        let x1 = minp.0 - delta;
        let x2 = minp.0 + delta;
        let y12 = 0.5 * (f(minp.0) + minp.1);
        pts.extend_from_slice(&[
            (x1, y12), (x2, y12)
        ]);
        
        let mut minidx = 0;
        let mut minval = pts[minidx].1;
        for (i, p) in pts.iter().enumerate() {
            if p.1 < minval {
                minidx = i;
                minval = p.1;
            }
        }
        minp = (pts[minidx].0, minval);
        pts.swap_remove(minidx);
        delta = invtwoltz * (f(minp.0) - minp.1);
    }

    minp.0
}
