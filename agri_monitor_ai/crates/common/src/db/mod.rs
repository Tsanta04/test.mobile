//! Database connection and operations module
//!
//! This module provides connections to different databases used in the system:
//! - InfluxDB for time series data (sensor readings)
//! - MongoDB for image storage and metadata
//! - PostgreSQL for relational data (economic analysis, etc.)

pub mod influx;
pub mod mongo;
pub mod postgres;

