
#[derive(Debug)]
pub struct Scence {
    sd: f32,
    emissive: f32
}

pub fn sample(x: f32, y: f32) -> f32 {
    let mut sum = 0.0;
    for i in 0..64 {
        let rand: f32 =  rand::random::<f32>();
        //let a: f32 = std::f32::consts::PI * 2.0 * rand;               // 均匀采样
        //let a: f32 = std::f32::consts::PI * 2.0 * i as f32 / 64.0;    // 分层采样
        let a: f32 = std::f32::consts::PI * 2.0 * (i as f32 + rand) / 64 as f32; // 抖动采样
        sum = sum + trace(x, y, a.cos(), a.sin());
    }
    sum / 64.0
}

pub fn scence(x: f32, y: f32) -> Scence {
    let r1 = Scence {
        sd: circle_sdf(x, y, 0.3f32, 0.3f32, 0.1f32),
        emissive: 2.0f32
    };
    let r2 = Scence {
        sd: circle_sdf(x, y, 0.3f32, 0.7f32, 0.05f32),
        emissive: 0.8f32
    };
    let r3 = Scence {
        sd: circle_sdf(x, y, 0.7f32, 0.5f32, 0.1f32),
        emissive: 0.0f32
    };

    union_op(union_op(r1, r2), r3)
}

pub fn union_op(a: Scence, b: Scence) -> Scence {
    if a.sd < b.sd { a } else { b }
}

fn trace(x: f32, y: f32, dx: f32, dy: f32) -> f32 {
    let mut t = 0.0;
    for _ in 0..10 {
        if t >= 2.0 {
            break;
        }
        let r = scence(x + dx * t, y + dy * t);
        if r.sd < std::f32::EPSILON {
            return r.emissive;
        }
        t = t + r.sd;
    }
    0.0
}

fn circle_sdf(x: f32, y: f32, cx: f32, cy: f32, r: f32) -> f32 {
    let ux = x - cx;
    let uy = y - cy;
    (ux * ux + uy * uy).sqrt() - r
}

