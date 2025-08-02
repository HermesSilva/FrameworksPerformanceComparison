using System;
using System.Diagnostics;
using System.Security.Cryptography.X509Certificates;
using System.Text.Json;
using System.Threading;
using System.Threading.Tasks;

namespace Common
{
    public class XMetrics
    {

        private long _TotalRequests = 0;
        private long _TotalErrors = 0;
        private long _TotalDurationTicks = 0;
        private long _Start = 0;

        private readonly double[] _LatencyBuffer = new double[10000];
        private int _LatencyIndex = 0;
        private int _LatencyCount = 0;
        private long _StartTime;
        private long _DataSise;
        private readonly object _LatencyLock = new object();

        public XMetrics()
        {
            _Start = Stopwatch.GetTimestamp();
        }

        public void Start()
        {
            _StartTime = Stopwatch.GetTimestamp();
        }

        public void Error()
        {
            Interlocked.Increment(ref _TotalErrors);
        }

        public void End(long pDataSise)
        {
            Interlocked.Add(ref _DataSise, pDataSise);

            var durationTicks = Stopwatch.GetTimestamp() - _StartTime;
            var durationMs = durationTicks * 1000.0 / Stopwatch.Frequency;

            Interlocked.Increment(ref _TotalRequests);
            Interlocked.Add(ref _TotalDurationTicks, durationTicks);

            AddLatencySample(durationMs);
        }

        private void AddLatencySample(double pDurationMs)
        {
            lock (_LatencyLock)
            {
                _LatencyBuffer[_LatencyIndex] = pDurationMs;
                _LatencyIndex = (_LatencyIndex + 1) % _LatencyBuffer.Length;
                if (_LatencyCount < _LatencyBuffer.Length)
                    _LatencyCount++;
            }
        }

        public PerformanceMetrics GetMetrics()
        {
            var totalRequests = Interlocked.Read(ref _TotalRequests);
            var totalErrors = Interlocked.Read(ref _TotalErrors);
            var totalTicks = Interlocked.Read(ref _TotalDurationTicks);

            var avgDurationMs = totalRequests > 0 ? (totalTicks * 1000.0 / Stopwatch.Frequency) / totalRequests : 0;

            var (p50, p95, p99) = CalculatePercentiles();
            var elapsedtk = (Stopwatch.GetTimestamp() - _Start);
            var elapsedms = (elapsedtk / Stopwatch.Frequency);

            return new PerformanceMetrics
            {
                Timestamp = new TimeSpan(elapsedtk),
                TotalRequests = totalRequests,
                TotalErrors = totalErrors,
                RequestsPerSecond = elapsedms > 0 ? (_TotalRequests / elapsedms) : 0,
                AverageLatencyMs = Math.Round(avgDurationMs, 3),
                P50LatencyMs = Math.Round(p50, 3),
                P95LatencyMs = Math.Round(p95, 3),
                P99LatencyMs = Math.Round(p99, 3),
                DataSise = _DataSise / 1024,
                DataSecond = (elapsedms > 0 ? _DataSise / elapsedms : 0) / 1024.0M,
                SuccessRate = totalRequests > 0 ? Math.Round((double)(totalRequests - totalErrors) / totalRequests * 100, 2) : 100.0
            };
        }

        private (double p50, double p95, double p99) CalculatePercentiles()
        {
            lock (_LatencyLock)
            {
                if (_LatencyCount == 0)
                    return (0, 0, 0);

                var validCount = Math.Min(_LatencyCount, _LatencyBuffer.Length);
                var samples = new double[validCount];
                Array.Copy(_LatencyBuffer, samples, validCount);
                Array.Sort(samples);

                return (GetPercentile(samples, 50), GetPercentile(samples, 95), GetPercentile(samples, 99));
            }
        }

        private static double GetPercentile(double[] pSortedData, double pPercentile)
        {
            if (pSortedData.Length == 0)
                return 0;

            var index = (int)Math.Ceiling(pPercentile / 100.0 * pSortedData.Length) - 1;
            index = Math.Max(0, Math.Min(index, pSortedData.Length - 1));
            return pSortedData[index];
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
        public double SuccessRate
        {
            get; set;
        }
        public long DataSise
        {
            get;
            internal set;
        }
        public decimal DataSecond
        {
            get;
            internal set;
        }

        public override string ToString()
        {
            return $@"Performance Metrics Report
========================
Timestamp: {Timestamp}
Total Data: {DataSise:N0} KB
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
