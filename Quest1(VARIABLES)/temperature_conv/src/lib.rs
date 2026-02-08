// recommendation: use Colorful Comments extension for better readability of the comments in this file

// Converts Fahrenheit to Celsius
pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) / (9.0 / 5.0)
}

// Converts Celsius to Fahrenheit
pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * (9.0 / 5.0) + 32.0
}

//* ═══════════════════════════════════════════════════════════════════════════════════
//* 🌡️ WHO INVENTED FAHRENHEIT AND WHY?
// Daniel Gabriel Fahrenheit (a German physicist living in the Netherlands) created it around 1724.
// 
// At that time (early 1700s), thermometers existed but were inconsistent — different ones gave
// different readings for the same temperature. Nobody had a reliable, reproducible way to mark a scale.
// 
// Fahrenheit wanted to fix that. He built better thermometers (especially the first good
// mercury-in-glass ones — mercury expands more predictably and has a wider range than alcohol).
// 
// To make a scale, he needed fixed reference points that anyone could recreate in a lab.
// 
//* 🔬 HIS ORIGINAL REFERENCE POINTS:
// • 0 °F — the lowest temperature he could reliably make in the lab: a mixture of ice + water +
//   ammonium chloride (a salt) — basically the coldest brine he could get.
// • 32 °F — freezing point of plain water/ice (a very reproducible point).
// • Around 96 °F — roughly human body temperature (he measured armpit temperature;
//   later adjusted to 98.6 °F).
//* ═══════════════════════════════════════════════════════════════════════════════════
//* Why did Kelvin create absolute zero?
// 
// In the early 1800s, scientists (including Gay-Lussac, Charles, and others) discovered
// something super important about gases:
// 
//* 🌡️ KEY DISCOVERIES:
// • When you cool a gas, its volume decreases in a straight line (Charles's/Gay-Lussac's law)
// • Keep cooling further, and the volume would reach zero at ~-273 °C
// • This reveals a lowest possible temperature — ABSOLUTE ZERO ❄️
// 
//* 🔑 THE CRUCIAL INSIGHT:
// Below absolute zero, volume can't be negative, and molecules basically stop moving
// completely. This point is where all thermal motion (molecular vibration) theoretically
// stops, and a system has the MINIMUM POSSIBLE ENERGY.
// 
//* 🎯 ARBITRARY vs ABSOLUTE SCALES:
// • Fahrenheit & Celsius → ARBITRARY/RELATIVE scales
//   (start from water freezing/boiling or human body temp — convenient for humans!)
// • Kelvin → ABSOLUTE scale
//   (based on fundamental physics — meaningless for humans, but CRITICAL for physics!)
//* ═══════════════════════════════════════════════════════════════════════════════════
