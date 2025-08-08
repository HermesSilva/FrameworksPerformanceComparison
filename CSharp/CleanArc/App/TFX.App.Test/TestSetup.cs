using Microsoft.AspNetCore.Builder;
using Microsoft.Extensions.DependencyInjection;

using TFX.Core.Test.Setup;

namespace TFX.App.INF.Test
{
    public class TestSetup : XBaseTest
    {
        public TestSetup(XServerSidePrepare pPrepare)
            : base(pPrepare)
        {
        }

        protected override void AppPrepare(WebApplicationBuilder pBuilder)
        {
            
            base.AppPrepare(pBuilder);
        }
    }
}
