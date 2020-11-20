# Average Directional Movement Index Rate (ADXR)
```
use ta_common::traits::Indicator;
use dx_rs::ADXR;
let mut adxr = ADXR::new(5);
assert_eq!(adxr.next([82.15, 81.29, 81.59]), None);
assert_eq!(adxr.next([81.89, 80.64, 81.06]), None);
assert_eq!(adxr.next([83.03, 81.31, 82.87]), None);
assert_eq!(adxr.next([83.30, 82.65, 83.00]), None);
assert_eq!(adxr.next([83.85, 83.07, 83.61]), None);
assert_eq!(adxr.next([83.90, 83.11, 83.15]), None);
assert_eq!(adxr.next([83.33, 82.49, 82.84]), None);
assert_eq!(adxr.next([84.30, 82.30, 83.99]), None);
assert_eq!(adxr.next([84.84, 84.15, 84.55]), None);
assert_eq!(adxr.next([85.00, 84.11, 84.36]), None);
assert_eq!(adxr.next([85.90, 84.03, 85.53]), None);
assert_eq!(adxr.next([86.58, 85.39, 86.54]), None);
assert_eq!(adxr.next([86.98, 85.76, 86.89]), Some(51.49047068971688));
assert_eq!(adxr.next([88.00, 87.17, 87.77]), Some(55.63390965480972));
assert_eq!(adxr.next([87.87, 87.01, 87.29]), Some(59.25861713224057));
```

### Calculation
ADXR=(ADX<sub>t</sub> + ADX<sub>t-(period-1)</sub>)/2;