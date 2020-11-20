# Average Directional Movement Index (ADX)

```
use ta_common::traits::Indicator;
use dx_rs::ADX;

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
```
### Calculation

ADX<sub>t</sub>=k*ADX<sub>t-1</sub> + DX(input)  
where k=(period-1)/period
