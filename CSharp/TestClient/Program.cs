using System.Diagnostics;
using System.Net.Http.Json;

using Common;

using UsingMiddleware;

namespace TestClient
{
    internal class Program
    {
        static XMetrics _Metrics = new XMetrics();
        private static List<XPayload> _Payloads;
        static Timer _Timer;
        static void Main(string[] args)
        {
            var clients = 1;
            if (args?.Length > 0)
                int.TryParse(args[0], out clients);
            _Payloads = XPayloadGenerator.CreateMultiplePayloads(10_000);
            _Timer = new Timer(state =>
            {
                Console.Clear();
                Console.WriteLine(_Metrics.GetMetrics());
            }, null, TimeSpan.Zero, TimeSpan.FromSeconds(5));

            for (int i = 0; i < clients; i++)
            {
                ThreadPool.QueueUserWorkItem(state => SendData());
            }
            Console.WriteLine("Press any key to exit...");
            Console.ReadKey();
        }

        private static void SendData()
        {
            using var client = new HttpClient();
            client.BaseAddress = new Uri("http://localhost:21012");
            client.Timeout = TimeSpan.FromSeconds(20);
            var pl = 0;
            for (int i = 0; i < 10_000_000; i++)
            {
                Int64 dsize = 0;
                try
                {

                    _Metrics.Start();
                    if (pl >= _Payloads.Count)
                        pl = 0;
                    using var content = JsonContent.Create(_Payloads[pl], typeof(XPayload));
                    
                    var response = client.PostAsync("/", content).Result;
                    response.EnsureSuccessStatusCode();

                }
                catch (Exception ex)
                {
                    _Metrics.Error();
                    Console.WriteLine($"Request failed with exception: {ex.Message}");
                }
                finally
                {
                    _Metrics.End(dsize);
                }
            }
        }
    }
}
