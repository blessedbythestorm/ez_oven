ECHO OFF
set ENGINE_DIR=%1
set PROJECT_DIR=%2
set PROJECT_NAME=%3
set CONTENT_DIR=%4
set CONTENT_NAME=%5
set CONTENT_LIST_FILE=%6
set OUTPUT_PAK=%PROJECT_DIR%\Oven\%CONTENT_NAME%.pak
set UAT=%ENGINE_DIR%\Engine\Build\BatchFiles\RunUAT.bat
set PAK=%ENGINE_DIR%\Engine\Binaries\Win64\UnrealPak.exe
set EDITOR=%ENGINE_DIR%\Engine\Binaries\Win64\UE4Editor-Cmd.exe
set PROJECT_FILE=%PROJECT_DIR%\%PROJECT_NAME%.uproject
set COOK_DIR=%PROJECT_DIR%\%PROJECT_NAME%.uproject

ECHO ========= COOK CONTENT ============

call "%UAT%" ^
-ScriptsForProject="%PROJECT_FILE%" ^
BuildCookRun ^
-project="%PROJECT_FILE%" ^
-nocompile ^
-nocompileeditor ^
-ue4exe=%EDITOR% ^
-cook ^
-cookdir=%COOK_DIR% ^
-skippak ^
-skipstage ^
-noP4 ^
-platform=Android ^
-cookflavor=ASTC ^
-unversionedcookedcontent

ECHO ========= PACKING CONTENT ============

"%PAK%" "%OUTPUT_PAK%" -Create="%CONTENT_LIST_FILE%" -platform=Android

PAUSE 

