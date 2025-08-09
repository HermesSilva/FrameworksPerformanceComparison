set RelativePath=%~dp0%
set BinPath=D:\Tootega\DASE4VSBin
set home=C:\Users\Hermes\AppData\Local\Microsoft\VisualStudio\17.0_9c90c9bdFrameworksPerformanceComparison\Extensions\zez3cjsb.jy3
if exist %work% set VSIX=%work%
if exist %home% set VSIX=%home%
if exist %work2% set VSIX=%work2%

if [%BinPath%] == [] goto ERRO
if [%VSIX%] == [] goto NOCOPY

set DeployType=None

cd %BinPath%\


copy TFX.DASE*.* %VSIX%\
IF ERRORLEVEL 1 GOTO ERRO 

copy DASE.VSIX.Core.* %VSIX%\
IF ERRORLEVEL 1 GOTO ERRO

copy *.dll %VSIX%\
IF ERRORLEVEL 1 GOTO ERRO
copy *.pdb %VSIX%\
IF ERRORLEVEL 1 GOTO ERRO
copy DASE4VS.pkgdef %VSIX%\
IF ERRORLEVEL 1 GOTO ERRO

cd %RelativePath%
if exist "%ProgramFiles%\Microsoft Visual Studio\2022\Community\Common7\IDE\devenv.exe" "%ProgramFiles%\Microsoft Visual Studio\2022\Community\Common7\IDE\devenv.exe" /rootSuffix  FrameworksPerformanceComparison FrameworksPerformanceComparison.sln

goto fim

:ERRO
pause
:fim

