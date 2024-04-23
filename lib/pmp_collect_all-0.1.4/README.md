# Collect All tool

A tool that allow you to detect and collect all the items you require.

## Features

- Scan the nearby area in a cross pattern.
- Detect all the resources or the ones specified.
- Collect the resources previously detected in an easy to use way.

## Usage
```rust
use std::collections::HashMap;

use collect_all::CollectAll;
use robotics_lib::world::tile::Content;

let mut requirements = HashMap::new(); // Insert all your requirements in here
requirements.insert(Content::Tree(0), 5);
requirements.insert(Content::Coin(0), 0); // If the quantity is set to zero it will try to collect all the available coins

let range = 5; // Define a range

// To retrieve the coordinates of all specified resources and their quantities
let positions = CollectAll::detect_items(self, world, range, requirements);

// To retrieve the coordinates of all resources and their quantities
let positions = CollectAll::detect_all(self, world, range);

// To collect all the specified resources that are in the reachable range
let _ = CollectAll::collect_items(self, world, range, requirements);

// To collect all the resources that are in the reachable range
let _ = CollectAll::collect_all(self, world, range);
```

## Support
Feel free to contact `@poss03251` and `@Sim02R` on Telegram for customer support!