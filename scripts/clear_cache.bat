ECHO OFF

set PROJECT_DIR=%1

ECHO ========= DELETE CACHE FILES ============
rmdir /s /q %PROJECT_DIR%\Build
rmdir /s /q %PROJECT_DIR%\DerivedDataCache
rmdir /s /q %PROJECT_DIR%\Intermediate
rmdir /s /q %PROJECT_DIR%\Saved