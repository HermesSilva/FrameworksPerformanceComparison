using Microsoft.AspNetCore.Mvc;
using Microsoft.Extensions.Logging;
using TFX.Core.Controllers;
using TFX.Core.Services;
using TFX.App.MDL.Servisses;

namespace TFX.App.RLE.Servisses.Rules
{
    public class RequestTestRule : BaseRequestTestRule
    {
        public RequestTestRule(XService pService)
               :base(pService)
        {
        }
    }
}