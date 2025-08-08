using Microsoft.AspNetCore.Mvc;
using Microsoft.Extensions.Logging;
using TFX.Core.Controllers;
using TFX.Core.Services;

namespace TFX.App.INF.Servisses.Rules
{
    public class INFRequestTestServiceRule : RequestTestService.BaseINFRequestTestServiceRule
    {
        public INFRequestTestServiceRule(RequestTestService pService)
               :base(pService)
        {
        }
    }
}