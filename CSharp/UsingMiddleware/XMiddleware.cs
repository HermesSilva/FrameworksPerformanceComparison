using System;
using System.Diagnostics;
using System.IO;
using System.Text.Json;
using System.Threading;
using System.Threading.Tasks;

using Common;

using Microsoft.AspNetCore.Http;

namespace UsingMiddleware
{
    public class XMiddleware
    {
        private readonly RequestDelegate _Next;

        public static XMetrics Metrics = new XMetrics();
        public XMiddleware(RequestDelegate pNext)
        {
            _Next = pNext;
        }

        public async Task InvokeAsync(HttpContext pContext)
        {
            if (pContext.Request.Path != "/")
            {
                await _Next(pContext);
                return;
            }
            var data = await pContext.Request.ReadFromJsonAsync<XPayload>(new JsonSerializerOptions
            {
                PropertyNameCaseInsensitive = true,
                AllowTrailingCommas = true,
                ReadCommentHandling = JsonCommentHandling.Skip
            });
            Metrics.Start();
            try
            {
                pContext.Response.ContentType = "application/json";
                await pContext.Response.WriteAsync("ok");
                await _Next(pContext);
            }
            catch
            {
                Metrics.Error();
                pContext.Response.StatusCode = 500;
                await pContext.Response.WriteAsync("Internal Server Error");
            }
            finally
            {
                Metrics.End(0);
            }
        }
    }
}
