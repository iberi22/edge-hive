//! Cache metrics tracking (hit rate, miss rate, eviction rate)

use std::sync::atomic::{AtomicU64, Ordering};

/// Cache performance metrics
#[derive(Debug)]
pub struct CacheMetrics {
    /// Total cache hits (L1 or L2)
    hits_l1: AtomicU64,
    hits_l2: AtomicU64,
    
    /// Total cache misses
    misses: AtomicU64,
    
    /// Total writes
    writes: AtomicU64,
    
    /// Total evictions
    evictions: AtomicU64,
    
    /// Whether metrics are enabled
    enabled: bool,
}

impl CacheMetrics {
    /// Create a new metrics tracker
    pub fn new() -> Self {
        Self {
            hits_l1: AtomicU64::new(0),
            hits_l2: AtomicU64::new(0),
            misses: AtomicU64::new(0),
            writes: AtomicU64::new(0),
            evictions: AtomicU64::new(0),
            enabled: true,
        }
    }

    /// Create a disabled metrics tracker (no-op)
    pub fn disabled() -> Self {
        Self {
            hits_l1: AtomicU64::new(0),
            hits_l2: AtomicU64::new(0),
            misses: AtomicU64::new(0),
            writes: AtomicU64::new(0),
            evictions: AtomicU64::new(0),
            enabled: false,
        }
    }

    /// Record a cache hit
    pub fn record_hit(&self, level: u8) {
        if !self.enabled {
            return;
        }

        match level {
            1 => self.hits_l1.fetch_add(1, Ordering::Relaxed),
            2 => self.hits_l2.fetch_add(1, Ordering::Relaxed),
            _ => return,
        };
    }

    /// Record a cache miss
    pub fn record_miss(&self) {
        if !self.enabled {
            return;
        }
        self.misses.fetch_add(1, Ordering::Relaxed);
    }

    /// Record a cache write
    pub fn record_write(&self) {
        if !self.enabled {
            return;
        }
        self.writes.fetch_add(1, Ordering::Relaxed);
    }

    /// Record a cache eviction
    pub fn record_eviction(&self) {
        if !self.enabled {
            return;
        }
        self.evictions.fetch_add(1, Ordering::Relaxed);
    }

    /// Get total hits (L1 + L2)
    pub fn total_hits(&self) -> u64 {
        self.hits_l1.load(Ordering::Relaxed) + self.hits_l2.load(Ordering::Relaxed)
    }

    /// Get L1 hits
    pub fn l1_hits(&self) -> u64 {
        self.hits_l1.load(Ordering::Relaxed)
    }

    /// Get L2 hits
    pub fn l2_hits(&self) -> u64 {
        self.hits_l2.load(Ordering::Relaxed)
    }

    /// Get total misses
    pub fn total_misses(&self) -> u64 {
        self.misses.load(Ordering::Relaxed)
    }

    /// Get total writes
    pub fn total_writes(&self) -> u64 {
        self.writes.load(Ordering::Relaxed)
    }

    /// Get total evictions
    pub fn total_evictions(&self) -> u64 {
        self.evictions.load(Ordering::Relaxed)
    }

    /// Calculate hit rate (0.0 to 1.0)
    pub fn hit_rate(&self) -> f64 {
        let hits = self.total_hits();
        let misses = self.total_misses();
        let total = hits + misses;

        if total == 0 {
            return 0.0;
        }

        hits as f64 / total as f64
    }

    /// Calculate L1 hit rate (percentage of total requests served by L1)
    pub fn l1_hit_rate(&self) -> f64 {
        let l1_hits = self.l1_hits();
        let total = l1_hits + self.l2_hits() + self.total_misses();

        if total == 0 {
            return 0.0;
        }

        l1_hits as f64 / total as f64
    }

    /// Get all metrics as a tuple
    pub fn totals(&self) -> (u64, u64, u64, u64, u64) {
        (
            self.l1_hits(),
            self.l2_hits(),
            self.total_misses(),
            self.total_writes(),
            self.total_evictions(),
        )
    }

    /// Reset all metrics
    pub fn reset(&self) {
        self.hits_l1.store(0, Ordering::Relaxed);
        self.hits_l2.store(0, Ordering::Relaxed);
        self.misses.store(0, Ordering::Relaxed);
        self.writes.store(0, Ordering::Relaxed);
        self.evictions.store(0, Ordering::Relaxed);
    }
}

impl Default for CacheMetrics {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics_basic() {
        let metrics = CacheMetrics::new();

        metrics.record_hit(1);
        metrics.record_hit(1);
        metrics.record_hit(2);
        metrics.record_miss();

        assert_eq!(metrics.l1_hits(), 2);
        assert_eq!(metrics.l2_hits(), 1);
        assert_eq!(metrics.total_hits(), 3);
        assert_eq!(metrics.total_misses(), 1);
    }

    #[test]
    fn test_hit_rate() {
        let metrics = CacheMetrics::new();

        metrics.record_hit(1);
        metrics.record_hit(1);
        metrics.record_hit(2);
        metrics.record_miss();

        // 3 hits / 4 total = 0.75
        assert!((metrics.hit_rate() - 0.75).abs() < 0.001);
    }

    #[test]
    fn test_l1_hit_rate() {
        let metrics = CacheMetrics::new();

        metrics.record_hit(1);
        metrics.record_hit(1);
        metrics.record_hit(2);
        metrics.record_miss();

        // 2 L1 hits / 4 total = 0.5
        assert!((metrics.l1_hit_rate() - 0.5).abs() < 0.001);
    }

    #[test]
    fn test_disabled_metrics() {
        let metrics = CacheMetrics::disabled();

        metrics.record_hit(1);
        metrics.record_miss();
        metrics.record_write();

        assert_eq!(metrics.total_hits(), 0);
        assert_eq!(metrics.total_misses(), 0);
        assert_eq!(metrics.total_writes(), 0);
    }

    #[test]
    fn test_reset() {
        let metrics = CacheMetrics::new();

        metrics.record_hit(1);
        metrics.record_miss();
        metrics.record_write();

        assert!(metrics.total_hits() > 0);

        metrics.reset();

        assert_eq!(metrics.total_hits(), 0);
        assert_eq!(metrics.total_misses(), 0);
        assert_eq!(metrics.total_writes(), 0);
    }
}
