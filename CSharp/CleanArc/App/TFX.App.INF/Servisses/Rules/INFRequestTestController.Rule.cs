using Microsoft.AspNetCore.Mvc;
using Microsoft.Extensions.Logging;
using TFX.Core.Controllers;
using TFX.Core.Services;

namespace TFX.App.INF.Servisses.Rules
{
    public class INFRequestTestControllerRule : RequestTestController.BaseINFRequestTestControllerRule
    {
        public INFRequestTestControllerRule(RequestTestController pController)
               :base(pController)
        {
        }
    }
}