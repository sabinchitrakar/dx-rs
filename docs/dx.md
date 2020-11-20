# Directional Movement Index (DX)
```
use ta_common::traits::Indicator;
use dx_rs::DX;

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
```
### Calculation
```
DX=(dm_diff/dm_sum)*100;
where:
dm_diff=|norm_up -norm_down|;
dm_sum=|norm_up+norm_down|;
norm_up=sm_up/ATR;
norm_down=sm_down/ATR;
sm_up=smoothed(sum of up);
sm_down=smoothed(sum of down);


```