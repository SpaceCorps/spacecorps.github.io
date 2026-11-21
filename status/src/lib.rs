//! spacecorps.tech status: the recorder of how many pilots are online, and the read API of the chart.
//!
//! * [`source`]: one look at the game's public server list.
//! * [`sampler`]: that look once a minute.
//! * [`store`]: the history, minute, 5-minute and hourly, in one SQLite file.
//! * [`api`]: the series per range as JSON.

pub mod api;
pub mod sampler;
pub mod source;
pub mod store;
