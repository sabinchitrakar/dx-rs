use ta_common::fixed_queue::FixedQueue;
use ta_common::traits::Indicator;
use crate::DX;

#[doc(include = "../docs/adx.md")]
pub struct ADX {
    dmi_history: FixedQueue<f64>,
    period: u32,
    sum_dmi: f64,
    dmi: DX,
    adx: Option<f64>,
}

impl ADX {
    pub fn new(period: u32) -> ADX {
        Self {
            dmi_history: FixedQueue::new(period),
            sum_dmi: 0.0,
            period,
            dmi: DX::new(period),
            adx: None,
        }
    }
}


impl Indicator<[f64; 3], Option<f64>> for ADX {
    fn next(&mut self, input: [f64; 3]) -> Option<f64> {
        let dmi = self.dmi.next(input);
        match dmi {
            None => (),
            Some(val) => {
                self.sum_dmi = self.sum_dmi + val;
                self.dmi_history.add(val);

            }
        }
        if self.dmi_history.is_full() {
            let period = self.period as f64;
            self.adx = match self.adx {
                None => Some(self.sum_dmi / period),
                Some(adx) => Some((adx * (period - 1_f64) + dmi.unwrap()) / period)
            }
        }
        self.adx
    }

    fn reset(&mut self) {
        self.dmi_history.clear();
        self.dmi.reset();
    }
}


#[cfg(test)]
mod tests {

    use ta_common::traits::Indicator;
    use crate::adx::ADX;

    #[test]
    fn it_works() {
        let mut adx = ADX::new(5);
        assert_eq!(adx.next([82.15, 81.29, 81.59, ]), None);
        assert_eq!(adx.next([81.89, 80.64, 81.06, ]), None);
        assert_eq!(adx.next([83.03, 81.31, 82.87, ]), None);
        assert_eq!(adx.next([83.30, 82.65, 83.00, ]), None);
        assert_eq!(adx.next([83.85, 83.07, 83.61, ]), None);
        assert_eq!(adx.next([83.90, 83.11, 83.15, ]), None);
        assert_eq!(adx.next([83.33, 82.49, 82.84, ]), None);
        assert_eq!(adx.next([84.30, 82.30, 83.99, ]), None);
        assert_eq!(adx.next([84.84, 84.15, 84.55, ]), Some(42.02843532361666));
        assert_eq!(adx.next([85.00, 84.11, 84.36, ]), Some(45.030197751280795));
        assert_eq!(adx.next([85.90, 84.03, 85.53, ]), Some(50.278112805162785));
        assert_eq!(adx.next([86.58, 85.39, 86.54, ]), Some(55.845790397341815));
        assert_eq!(adx.next([86.98, 85.76, 86.89, ]), Some(60.952506055817096));
        assert_eq!(adx.next([88.00, 87.17, 87.77, ]), Some(66.23762155833865));
        assert_eq!(adx.next([87.87, 87.01, 87.29, ]), Some(68.23912145931837));
    }
}
