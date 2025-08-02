using System;
using System.Threading;

using Common;

using Microsoft.AspNetCore.Builder;
using Microsoft.AspNetCore.Http;
using Microsoft.Extensions.Logging;

namespace UsingMiddleware
{
    public class Program
    {
        static Timer _Timer;
        public static void Main(string[] args)
        {
            var builder = WebApplication.CreateBuilder(args);
            builder.Logging.ClearProviders();
            builder.Logging.AddConsole().SetMinimumLevel(LogLevel.Warning);
            var app = builder.Build();

            app.UseMiddleware<XMiddleware>();
            _Timer = new Timer(state =>
            {
                Console.Clear();
                Console.WriteLine(XMiddleware.Metrics.GetMetrics());
            }, null, TimeSpan.Zero, TimeSpan.FromSeconds(10));
            app.Run("http://+:21012");
        }
    }
}
