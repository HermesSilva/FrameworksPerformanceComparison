use std::sync::{Mutex};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};
use std::fmt;

pub struct XMetrics {
    TotalRequests: AtomicU64,
    TotalErrors: AtomicU64,
    TotalDurationTicks: AtomicU64,
    TotalDataSize: AtomicU64,
    StartTimestamp: Instant,

    LatencyBuffer: Mutex<[f64; 10000]>,
    LatencyIndex: Mutex<usize>,
    LatencyCount: Mutex<usize>,

    CurrentStartTime: Mutex<Option<Instant>>,
}

impl XMetrics {
    pub fn new() -> Self {
        Self {
            TotalRequests: AtomicU64::new(0),
            TotalErrors: AtomicU64::new(0),
            TotalDurationTicks: AtomicU64::new(0),
            TotalDataSize: AtomicU64::new(0),
            StartTimestamp: Instant::now(),

            LatencyBuffer: Mutex::new([0.0; 10000]),
            LatencyIndex: Mutex::new(0),
            LatencyCount: Mutex::new(0),

            CurrentStartTime: Mutex::new(None),
        }
    }

    pub fn Start(&self) {
        *self.CurrentStartTime.lock().unwrap() = Some(Instant::now());
    }

    pub fn Error(&self) {
        self.TotalErrors.fetch_add(1, Ordering::Relaxed);
    }

    pub fn End(&self, data_size: u64) {
        let now = Instant::now();
        let mut current_start = self.CurrentStartTime.lock().unwrap();
        if let Some(start_time) = *current_start {
            let duration = now.duration_since(start_time);
            let duration_ms = duration.as_secs_f64() * 1000.0;

            self.TotalRequests.fetch_add(1, Ordering::Relaxed);
            self.TotalDurationTicks.fetch_add(duration.as_nanos() as u64, Ordering::Relaxed);
            self.TotalDataSize.fetch_add(data_size, Ordering::Relaxed);

            let mut buffer = self.LatencyBuffer.lock().unwrap();
            let mut index = self.LatencyIndex.lock().unwrap();
            let mut count = self.LatencyCount.lock().unwrap();

            let pos = *index % buffer.len();
            buffer[pos] = duration_ms;
            *index += 1;
            if *count < buffer.len() {
                *count += 1;
            }
        }
    }

    pub fn GetMetrics(&self) -> PerformanceMetrics {
        let total_requests = self.TotalRequests.load(Ordering::Relaxed);
        let total_errors = self.TotalErrors.load(Ordering::Relaxed);
        let total_ticks = self.TotalDurationTicks.load(Ordering::Relaxed);
        let total_data = self.TotalDataSize.load(Ordering::Relaxed);

        let avg_duration_ms = if total_requests > 0 {
            (total_ticks as f64 / 1_000_000.0) / total_requests as f64
        } else {
            0.0
        };

        let snapshot = self.GetLatencySnapshot();
        let mut sorted_snapshot = snapshot.clone();
        sorted_snapshot.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        let elapsed = self.StartTimestamp.elapsed().as_secs();
        let requests_per_sec = if elapsed > 0 { total_requests as f64 / elapsed as f64 } else { 0.0 };

        PerformanceMetrics {
            Timestamp: self.StartTimestamp.elapsed(),
            TotalRequests: total_requests as f64,
            TotalErrors: total_errors as i64,
            RequestsPerSecond: requests_per_sec,
            AverageLatencyMs: (avg_duration_ms * 1000.0).round() / 1000.0,
            P50LatencyMs: Self::GetPercentile(&sorted_snapshot, 50.0),
            P95LatencyMs: Self::GetPercentile(&sorted_snapshot, 95.0),
            P99LatencyMs: Self::GetPercentile(&sorted_snapshot, 99.0),
            DataSize: (total_data / 1024) as i64,
            DataSecond: if elapsed > 0 {
                ((total_data as f64 / elapsed as f64) / 1024.0).into()
            } else {
                0.0.into()
            },
            SuccessRate: if total_requests > 0 {
                (((total_requests - total_errors as u64) as f64 / total_requests as f64) * 100.0 * 100.0).round() / 100.0
            } else {
                100.0
            },
        }
    }

    fn GetLatencySnapshot(&self) -> Vec<f64> {
        let buffer = self.LatencyBuffer.lock().unwrap();
        let count = *self.LatencyCount.lock().unwrap();
        buffer[..count].to_vec()
    }

    fn GetPercentile(sorted_data: &[f64], percentile: f64) -> f64 {
        if sorted_data.is_empty() {
            return 0.0;
        }
        let index = ((percentile / 100.0) * sorted_data.len() as f64).ceil() as usize - 1;
        let clamped = index.min(sorted_data.len() - 1);
        (sorted_data[clamped] * 1000.0).round() / 1000.0
    }
}

pub struct PerformanceMetrics {
    pub Timestamp: Duration,
    pub TotalRequests: f64,
    pub TotalErrors: i64,
    pub RequestsPerSecond: f64,
    pub AverageLatencyMs: f64,
    pub P50LatencyMs: f64,
    pub P95LatencyMs: f64,
    pub P99LatencyMs: f64,
    pub DataSize: i64,
    pub DataSecond: f64,
    pub SuccessRate: f64,
}

impl fmt::Display for PerformanceMetrics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        write!(f, "\
Performance Metrics Report
========================
Timestamp: {:.2?}
Total Data: {} KB
Total DataSecond: {:.2} KB
Total Requests: {:.0}
Total Errors: {}
Requests Per Second: {:.0}
Average Latency: {:.3} ms
P50 Latency: {:.3} ms
P95 Latency: {:.3} ms
P99 Latency: {:.3} ms
Success Rate: {:.2}%

Generated at: {} UTC",
               self.Timestamp,
               self.DataSize,
               self.DataSecond,
               self.TotalRequests,
               self.TotalErrors,
               self.RequestsPerSecond,
               self.AverageLatencyMs,
               self.P50LatencyMs,
               self.P95LatencyMs,
               self.P99LatencyMs,
               self.SuccessRate,
               now
        )
    }
}
