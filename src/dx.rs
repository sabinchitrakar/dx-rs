use tr_rs::TrueRange;
use ta_common::traits::Indicator;
#[doc(include = "../docs/dx.md")]
pub struct DX {
    period: u32,
    tr: TrueRange,
    k: f64,
    index: u32,
    sum_atr: f64,
    sum_up: f64,
    sum_down: f64,

    prev: [f64; 3],
    init: bool,
}

impl DX {
    pub fn new(period: u32) -> DX {
        Self {
            period,
            tr: TrueRange::new(),
            k: (period - 1) as f64 / period as f64,
            index: 0,
            sum_atr: 0.0,
            sum_up: 0.0,
            sum_down: 0.0,

            prev: [0.0, 0.0, 0.0],
            init: false,
        }
    }

    pub fn calc_direction(high: f64, p_high: f64, low: f64, p_low: f64) -> [f64; 2] {
        let mut up = high - p_high;
        let mut down = p_low - low;
        if up < 0.0 || up < down {
            up = 0.0;
        }
        if down < 0.0 || down < up {
            down = 0.0;
        }
        return [up, down];
    }
    fn get_direction(&self, input: [f64; 3]) -> [f64; 2] {
        let high = input[0];
        let p_high = self.prev[0];
        let low = input[1];
        let p_low = self.prev[1];
        let dir = DX::calc_direction(high, p_high, low, p_low);
        return dir;
    }
}

impl Indicator<[f64; 3], Option<f64>> for DX {
    fn next(&mut self, input: [f64; 3]) -> Option<f64> {
        self.index = self.index + 1;


        if !self.init {
            self.prev = input;
            self.sum_atr = self.sum_atr + self.tr.next(input);
            self.init = true;
            return None;
        }

        if self.index < self.period {
            self.sum_atr = self.sum_atr + self.tr.next(input);
            let dir = self.get_direction(input);
            self.sum_up = self.sum_up + dir[0];
            self.sum_down = self.sum_down + dir[1];
            self.prev = input;
            None
        } else {
            let tr = self.tr.next(input);
            self.sum_atr = self.sum_atr * self.k + tr;

            let dir = self.get_direction(input);
            self.sum_up = self.sum_up * self.k + dir[0];
            self.sum_down = self.sum_down * self.k + dir[1];


            let norm_up = 100_f64*(self.sum_up / self.sum_atr);
            let norm_down = 100_f64*(self.sum_down / self.sum_atr);
            let norm_diff = (norm_up - norm_down).abs();
            let norm_sum = norm_up + norm_down;

            let dx = (norm_diff / norm_sum)*100_f64;

            self.prev = input;
            Some(dx)
        }
    }

    fn reset(&mut self) {
        self.index = 0;
        self.sum_atr = 0.0;
        self.sum_up = 0.0;
        self.sum_down = 0.0;

        self.prev = [0.0, 0.0, 0.0];
        self.init = false;
        self.tr.reset();
    }
}


#[cfg(test)]
mod tests {

    use ta_common::traits::Indicator;
    use crate::dx::DX;

    #[test]
    fn it_works() {
        let mut dmi = DX::new(5);
        assert_eq!(dmi.next([82.15, 81.29, 81.59, ]), None);
        assert_eq!(dmi.next([81.89, 80.64, 81.06, ]), None);
        assert_eq!(dmi.next([83.03, 81.31, 82.87, ]), None);
        assert_eq!(dmi.next([83.30, 82.65, 83.00, ]), None);
        assert_eq!(dmi.next([83.85, 83.07, 83.61, ]), Some(52.68425841674205));
        assert_eq!(dmi.next([83.90, 83.11, 83.15, ]), Some(53.99247953992465));
        assert_eq!(dmi.next([83.33, 82.49, 82.84, ]), Some(7.795927847023168));
        assert_eq!(dmi.next([84.30, 82.30, 83.99, ]), Some(41.88861985472129));
        assert_eq!(dmi.next([84.84, 84.15, 84.55, ]), Some(53.78089095967214));
        assert_eq!(dmi.next([85.00, 84.11, 84.36, ]), Some(57.03724746193731));
        assert_eq!(dmi.next([85.90, 84.03, 85.53, ]), Some(71.26977302069074));
        assert_eq!(dmi.next([86.58, 85.39, 86.54, ]), Some(78.11650076605797));
        assert_eq!(dmi.next([86.98, 85.76, 86.89, ]), Some(81.37936868971823));
        assert_eq!(dmi.next([88.00, 87.17, 87.77, ]), Some(87.37808356842487));
        assert_eq!(dmi.next([87.87, 87.01, 87.29, ]), Some(76.24512106323728));
    }
}
