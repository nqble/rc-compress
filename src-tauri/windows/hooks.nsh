; Installer hooks (`bundle.windows.nsis.installerHooks` in tauri.conf.json)
;
; The `compresso` command (see `src/sys/cli_install.rs`): a copy of the command line tool in the
; `bin` folder of the install folder, which is added to the user's PATH. The folder is passed to
; PowerShell in an environment variable, so no path needs quoting.

; rc-compress: its entry in Windows Explorer's right-click menu, added by the installer and removed
; by the uninstaller (inserted into the two hooks below). It is a submenu of the classic menu (on Windows 11: Show more options):
;
;   rc-compress > Quick Compress
;                 Open in rc-compress...
;
; Each entry starts the app with the file; `--quick-compress` must match QUICK_COMPRESS_ARG in
; src-tauri/src/sys/explorer_menu.rs. It is offered for every file: the app skips what it can't
; compress. Written for the current user (HKCU), like the app's default install.
;
; The subkeys' names set the entries' order (Explorer sorts them); the texts are in English for now.

!define RC_MENU_KEY "Software\Classes\*\shell\rc-compress"

!macro RC_MENU_INSTALL
  WriteRegStr HKCU "${RC_MENU_KEY}" "MUIVerb" "rc-compress"
  WriteRegStr HKCU "${RC_MENU_KEY}" "Icon" "$INSTDIR\${MAINBINARYNAME}.exe"
  ; An empty SubCommands makes the entry a submenu of the keys under its `shell`
  WriteRegStr HKCU "${RC_MENU_KEY}" "SubCommands" ""

  WriteRegStr HKCU "${RC_MENU_KEY}\shell\10quick" "MUIVerb" "Quick Compress"
  ; Several selected files are each passed to the app (one launch per file reaches the running app)
  WriteRegStr HKCU "${RC_MENU_KEY}\shell\10quick" "MultiSelectModel" "Player"
  WriteRegStr HKCU "${RC_MENU_KEY}\shell\10quick\command" "" '"$INSTDIR\${MAINBINARYNAME}.exe" --quick-compress "%1"'

  WriteRegStr HKCU "${RC_MENU_KEY}\shell\90open" "MUIVerb" "Open in rc-compress..."
  ; 0x20: a separator above it
  WriteRegDWORD HKCU "${RC_MENU_KEY}\shell\90open" "CommandFlags" 0x20
  WriteRegStr HKCU "${RC_MENU_KEY}\shell\90open" "MultiSelectModel" "Player"
  WriteRegStr HKCU "${RC_MENU_KEY}\shell\90open\command" "" '"$INSTDIR\${MAINBINARYNAME}.exe" "%1"'
!macroend

!macro RC_MENU_UNINSTALL
  DeleteRegKey HKCU "${RC_MENU_KEY}"
!macroend

!macro NSIS_HOOK_POSTINSTALL
  !insertmacro RC_MENU_INSTALL
  CreateDirectory "$INSTDIR\bin"
  CopyFiles /SILENT "$INSTDIR\compresso-cli.exe" "$INSTDIR\bin\compresso.exe"
  System::Call 'Kernel32::SetEnvironmentVariable(t "COMPRESSO_BIN", t "$INSTDIR\bin")i'
  nsExec::Exec `powershell.exe -NoProfile -NonInteractive -ExecutionPolicy Bypass -Command "$$bin = $$env:COMPRESSO_BIN; $$path = [Environment]::GetEnvironmentVariable('Path', 'User'); $$entries = @(([string]$$path) -split ';' | Where-Object { $$_ -ne '' }); if (-not ($$entries | Where-Object { $$_.TrimEnd('\') -ieq $$bin })) { [Environment]::SetEnvironmentVariable('Path', (($$entries + $$bin) -join ';'), 'User') }"`
  Pop $0
!macroend

!macro NSIS_HOOK_PREUNINSTALL
  !insertmacro RC_MENU_UNINSTALL
  System::Call 'Kernel32::SetEnvironmentVariable(t "COMPRESSO_BIN", t "$INSTDIR\bin")i'
  nsExec::Exec `powershell.exe -NoProfile -NonInteractive -ExecutionPolicy Bypass -Command "$$bin = $$env:COMPRESSO_BIN; $$path = [Environment]::GetEnvironmentVariable('Path', 'User'); if ($$path) { $$kept = @($$path -split ';' | Where-Object { $$_ -ne '' -and $$_.TrimEnd('\') -ine $$bin }); [Environment]::SetEnvironmentVariable('Path', ($$kept -join ';'), 'User') }"`
  Pop $0
  Delete "$INSTDIR\bin\compresso.exe"
  RMDir "$INSTDIR\bin"
!macroend
