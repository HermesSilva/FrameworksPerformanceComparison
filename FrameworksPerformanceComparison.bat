set RelativePath=%~dp0%

cd %RelativePath%
if exist "%ProgramFiles%\Microsoft Visual Studio\2022\Community\Common7\IDE\devenv.exe" "%ProgramFiles%\Microsoft Visual Studio\2022\Community\Common7\IDE\devenv.exe" /rootSuffix  FrameworksPerformanceComparison FrameworksPerformanceComparison.sln

goto fim

:ERRO
pause
:fim

