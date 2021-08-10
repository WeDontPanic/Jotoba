@echo off

echo.
echo -------------------
echo Start parsing JS Files
echo -------------------
echo.

for /f "delims=" %%A in (
  'dir /b /s /a-d "*.js"'
) do for %%B in ("%%A\..") do ECHO %%A & CALL minify %%A > %%A.min.js & DEL %%A & REN %%A.min.js %%~nxA

echo.
echo -------------------
echo Done!
echo -------------------
echo.