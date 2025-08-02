using System;
using System.Diagnostics;
using System.Threading;

namespace Common
{
    public class XMetrics
    {
        private long _TotalRequests;
        private long _TotalErrors;
        private long _TotalDurationTicks;
        private long _TotalDataSize;
        private readonly long _StartTimestamp;

        private readonly double[] _LatencyBuffer = new double[10000];
        private int _LatencyIndex;
        private int _LatencyCount;

        private long _CurrentStartTime;
        Object _ToLock = new object();

        public XMetrics()
        {
            _StartTimestamp = Stopwatch.GetTimestamp();
        }

        public void Start()
        {
            _CurrentStartTime = Stopwatch.GetTimestamp();
        }

        public void Error()
        {
            Interlocked.Increment(ref _TotalErrors);
        }

        public void End(long dataSize)
        {
            lock (_ToLock)
            {
                _TotalDataSize += dataSize;

                var durationTicks = Stopwatch.GetTimestamp() - _CurrentStartTime;
                var durationMs = durationTicks * 1000.0 / Stopwatch.Frequency;

                _TotalRequests++;
                _TotalDurationTicks += durationTicks;
                _LatencyIndex++;
                var index = _LatencyIndex % _LatencyBuffer.Length;
                _LatencyBuffer[index] = durationMs;

                if (_LatencyCount < _LatencyBuffer.Length)
                    _LatencyCount++;
            }
        }

        public PerformanceMetrics GetMetrics()
        {
            var totalRequests = Interlocked.Read(ref _TotalRequests);
            var totalErrors = Interlocked.Read(ref _TotalErrors);
            var totalTicks = Interlocked.Read(ref _TotalDurationTicks);
            var totalData = Interlocked.Read(ref _TotalDataSize);

            double avgDurationMs = totalRequests > 0
                ? (totalTicks * 1000.0 / Stopwatch.Frequency) / totalRequests
                : 0;

            var latencySnapshot = GetLatencySnapshot();
            Array.Sort(latencySnapshot);

            var elapsedTicks = Stopwatch.GetTimestamp() - _StartTimestamp;
            var elapsedSeconds = elapsedTicks / Stopwatch.Frequency;

            return new PerformanceMetrics
            {
                Timestamp = new TimeSpan(elapsedTicks),
                TotalRequests = totalRequests,
                TotalErrors = totalErrors,
                RequestsPerSecond = elapsedSeconds > 0 ? totalRequests / elapsedSeconds : 0,
                AverageLatencyMs = Math.Round(avgDurationMs, 3),
                P50LatencyMs = Math.Round(GetPercentile(latencySnapshot, 50), 3),
                P95LatencyMs = Math.Round(GetPercentile(latencySnapshot, 95), 3),
                P99LatencyMs = Math.Round(GetPercentile(latencySnapshot, 99), 3),
                DataSize = totalData / 1024,
                DataSecond = (elapsedSeconds > 0 ? totalData / elapsedSeconds : 0M) / 1024.0M,

                SuccessRate = totalRequests > 0

                    ? Math.Round((double)(totalRequests - totalErrors) / totalRequests * 100, 2)
                    : 100.0
            };
        }

        private double[] GetLatencySnapshot()
        {
            var count = Math.Min(Volatile.Read(ref _LatencyCount), _LatencyBuffer.Length);
            var snapshot = new double[count];

            for (int i = 0; i < count; i++)
                snapshot[i] = Volatile.Read(ref _LatencyBuffer[i]);

            return snapshot;
        }

        private static double GetPercentile(double[] sortedData, double percentile)
        {
            if (sortedData.Length == 0)
                return 0;

            var index = (int)Math.Ceiling(percentile / 100.0 * sortedData.Length) - 1;
            index = Math.Clamp(index, 0, sortedData.Length - 1);
            return sortedData[index];
        }
    }

    public class PerformanceMetrics
    {
        public TimeSpan Timestamp
        {
            get; set;
        }
        public double TotalRequests
        {
            get; set;
        }
        public long TotalErrors
        {
            get; set;
        }
        public double RequestsPerSecond
        {
            get; set;
        }
        public double AverageLatencyMs
        {
            get; set;
        }
        public double P50LatencyMs
        {
            get; set;
        }
        public double P95LatencyMs
        {
            get; set;
        }
        public double P99LatencyMs
        {
            get; set;
        }
        public long DataSize
        {
            get; internal set;
        }
        public decimal DataSecond
        {
            get; internal set;
        }
        public double SuccessRate
        {
            get; set;
        }

        public override string ToString()
        {
            return $@"Performance Metrics Report
========================
Timestamp: {Timestamp}
Total Data: {DataSize:N0} KB
Total DataSecond: {DataSecond:N2} KB 
Total Requests: {TotalRequests:N0} 
Total Errors: {TotalErrors:N0}
Requests Per Second: {RequestsPerSecond:N0}
Average Latency: {AverageLatencyMs:F3} ms
P50 Latency: {P50LatencyMs:F3} ms
P95 Latency: {P95LatencyMs:F3} ms
P99 Latency: {P99LatencyMs:F3} ms
Success Rate: {SuccessRate:F2}%

Generated at: {DateTime.UtcNow:yyyy-MM-dd HH:mm:ss} UTC";
        }
    }
}
