; Installer hooks (`bundle.windows.nsis.installerHooks` in tauri.conf.json)
;
; The `compresso` command (see `src/sys/cli_install.rs`): a copy of the command line tool in the
; `bin` folder of the install folder, which is added to the user's PATH. The folder is passed to
; PowerShell in an environment variable, so no path needs quoting.

; rc-compress: its entry in Windows Explorer's right-click menu, added by the installer and removed
; by the uninstaller (inserted into the two hooks below). It is a submenu of the classic menu (on Windows 11: Show more options):
;
;   rc-compress > Quick Compress
;                 Compress by > 10% ... 90%
;                 Fit to size > 8 MB, 10 MB, 25 MB, your size
;                 Remove metadata
;                 Open in rc-compress...
;
; Each entry starts the app with the file or folder; `--quick-compress` must match QUICK_COMPRESS_ARG in
; src-tauri/src/sys/explorer_menu.rs. It is offered for every file and folder: the app skips what it
; can't compress. Written for the current user (HKCU), like the app's default install.
;
; The subkeys' names set the entries' order (Explorer sorts them); the texts are in English for now.

!define RC_MENU_KEY "Software\Classes\*\shell\rc-compress"
!define RC_FOLDER_MENU_KEY "Software\Classes\Directory\shell\rc-compress"

!macro RC_MENU_ENTRIES KEY
  WriteRegStr HKCU "${KEY}" "MUIVerb" "rc-compress"
  WriteRegStr HKCU "${KEY}" "Icon" "$INSTDIR\${MAINBINARYNAME}.exe"
  ; An empty SubCommands makes the entry a submenu of the keys under its `shell`
  WriteRegStr HKCU "${KEY}" "SubCommands" ""

  WriteRegStr HKCU "${KEY}\shell\10quick" "MUIVerb" "Quick Compress"
  ; Several selected files are each passed to the app (one launch per file reaches the running app)
  WriteRegStr HKCU "${KEY}\shell\10quick" "MultiSelectModel" "Player"
  WriteRegStr HKCU "${KEY}\shell\10quick\command" "" '"$INSTDIR\${MAINBINARYNAME}.exe" --quick-compress "%1"'

  ; Compress by > 10% ... 90%: a submenu of its own; --compress-by must match COMPRESS_BY_PERCENTS
  ; (explorer_menu.rs), checked by scripts/test-compress-by.mjs
  WriteRegStr HKCU "${KEY}\shell\20compressby" "MUIVerb" "Compress by"
  WriteRegDWORD HKCU "${KEY}\shell\20compressby" "CommandFlags" 0x20
  WriteRegStr HKCU "${KEY}\shell\20compressby" "SubCommands" ""
  WriteRegStr HKCU "${KEY}\shell\20compressby\shell\10" "MUIVerb" "10%"
  WriteRegStr HKCU "${KEY}\shell\20compressby\shell\10" "MultiSelectModel" "Player"
  WriteRegStr HKCU "${KEY}\shell\20compressby\shell\10\command" "" '"$INSTDIR\${MAINBINARYNAME}.exe" --compress-by=10 "%1"'
  WriteRegStr HKCU "${KEY}\shell\20compressby\shell\20" "MUIVerb" "20%"
  WriteRegStr HKCU "${KEY}\shell\20compressby\shell\20" "MultiSelectModel" "Player"
  WriteRegStr HKCU "${KEY}\shell\20compressby\shell\20\command" "" '"$INSTDIR\${MAINBINARYNAME}.exe" --compress-by=20 "%1"'
  WriteRegStr HKCU "${KEY}\shell\20compressby\shell\30" "MUIVerb" "30%"
  WriteRegStr HKCU "${KEY}\shell\20compressby\shell\30" "MultiSelectModel" "Player"
  WriteRegStr HKCU "${KEY}\shell\20compressby\shell\30\command" "" '"$INSTDIR\${MAINBINARYNAME}.exe" --compress-by=30 "%1"'
  WriteRegStr HKCU "${KEY}\shell\20compressby\shell\40" "MUIVerb" "40%"
  WriteRegStr HKCU "${KEY}\shell\20compressby\shell\40" "MultiSelectModel" "Player"
  WriteRegStr HKCU "${KEY}\shell\20compressby\shell\40\command" "" '"$INSTDIR\${MAINBINARYNAME}.exe" --compress-by=40 "%1"'
  WriteRegStr HKCU "${KEY}\shell\20compressby\shell\50" "MUIVerb" "50%"
  WriteRegStr HKCU "${KEY}\shell\20compressby\shell\50" "MultiSelectModel" "Player"
  WriteRegStr HKCU "${KEY}\shell\20compressby\shell\50\command" "" '"$INSTDIR\${MAINBINARYNAME}.exe" --compress-by=50 "%1"'
  WriteRegStr HKCU "${KEY}\shell\20compressby\shell\60" "MUIVerb" "60%"
  WriteRegStr HKCU "${KEY}\shell\20compressby\shell\60" "MultiSelectModel" "Player"
  WriteRegStr HKCU "${KEY}\shell\20compressby\shell\60\command" "" '"$INSTDIR\${MAINBINARYNAME}.exe" --compress-by=60 "%1"'
  WriteRegStr HKCU "${KEY}\shell\20compressby\shell\70" "MUIVerb" "70%"
  WriteRegStr HKCU "${KEY}\shell\20compressby\shell\70" "MultiSelectModel" "Player"
  WriteRegStr HKCU "${KEY}\shell\20compressby\shell\70\command" "" '"$INSTDIR\${MAINBINARYNAME}.exe" --compress-by=70 "%1"'
  WriteRegStr HKCU "${KEY}\shell\20compressby\shell\80" "MUIVerb" "80%"
  WriteRegStr HKCU "${KEY}\shell\20compressby\shell\80" "MultiSelectModel" "Player"
  WriteRegStr HKCU "${KEY}\shell\20compressby\shell\80\command" "" '"$INSTDIR\${MAINBINARYNAME}.exe" --compress-by=80 "%1"'
  WriteRegStr HKCU "${KEY}\shell\20compressby\shell\90" "MUIVerb" "90%"
  WriteRegStr HKCU "${KEY}\shell\20compressby\shell\90" "MultiSelectModel" "Player"
  WriteRegStr HKCU "${KEY}\shell\20compressby\shell\90\command" "" '"$INSTDIR\${MAINBINARYNAME}.exe" --compress-by=90 "%1"'

  ; Fit to size > 8 MB, 10 MB, 25 MB, your size: --fit-to must match FIT_SIZES_IN_MB (explorer_menu.rs)
  WriteRegStr HKCU "${KEY}\shell\30fit" "MUIVerb" "Fit to size"
  WriteRegDWORD HKCU "${KEY}\shell\30fit" "CommandFlags" 0x20
  WriteRegStr HKCU "${KEY}\shell\30fit" "SubCommands" ""
  WriteRegStr HKCU "${KEY}\shell\30fit\shell\08" "MUIVerb" "8 MB (Discord)"
  WriteRegStr HKCU "${KEY}\shell\30fit\shell\08" "MultiSelectModel" "Player"
  WriteRegStr HKCU "${KEY}\shell\30fit\shell\08\command" "" '"$INSTDIR\${MAINBINARYNAME}.exe" --fit-to=8 "%1"'
  WriteRegStr HKCU "${KEY}\shell\30fit\shell\10" "MUIVerb" "10 MB (Discord)"
  WriteRegStr HKCU "${KEY}\shell\30fit\shell\10" "MultiSelectModel" "Player"
  WriteRegStr HKCU "${KEY}\shell\30fit\shell\10\command" "" '"$INSTDIR\${MAINBINARYNAME}.exe" --fit-to=10 "%1"'
  WriteRegStr HKCU "${KEY}\shell\30fit\shell\25" "MUIVerb" "25 MB (Email)"
  WriteRegStr HKCU "${KEY}\shell\30fit\shell\25" "MultiSelectModel" "Player"
  WriteRegStr HKCU "${KEY}\shell\30fit\shell\25\command" "" '"$INSTDIR\${MAINBINARYNAME}.exe" --fit-to=25 "%1"'
  WriteRegStr HKCU "${KEY}\shell\30fit\shell\90custom" "MUIVerb" "Your size (set in the app)"
  WriteRegDWORD HKCU "${KEY}\shell\30fit\shell\90custom" "CommandFlags" 0x20
  WriteRegStr HKCU "${KEY}\shell\30fit\shell\90custom" "MultiSelectModel" "Player"
  WriteRegStr HKCU "${KEY}\shell\30fit\shell\90custom\command" "" '"$INSTDIR\${MAINBINARYNAME}.exe" --fit-to=custom "%1"'

  WriteRegStr HKCU "${KEY}\shell\40metadata" "MUIVerb" "Remove metadata"
  WriteRegStr HKCU "${KEY}\shell\40metadata" "MultiSelectModel" "Player"
  WriteRegStr HKCU "${KEY}\shell\40metadata\command" "" '"$INSTDIR\${MAINBINARYNAME}.exe" --remove-metadata "%1"'

  WriteRegStr HKCU "${KEY}\shell\90open" "MUIVerb" "Open in rc-compress..."
  ; 0x20: a separator above it
  WriteRegDWORD HKCU "${KEY}\shell\90open" "CommandFlags" 0x20
  WriteRegStr HKCU "${KEY}\shell\90open" "MultiSelectModel" "Player"
  WriteRegStr HKCU "${KEY}\shell\90open\command" "" '"$INSTDIR\${MAINBINARYNAME}.exe" "%1"'
!macroend

; The same menu on files and on folders (a folder's media is compressed, subfolders included; see
; `quick_compress` in explorer_menu.rs)
!macro RC_MENU_INSTALL
  !insertmacro RC_MENU_ENTRIES "${RC_MENU_KEY}"
  !insertmacro RC_MENU_ENTRIES "${RC_FOLDER_MENU_KEY}"
!macroend

!macro RC_MENU_UNINSTALL
  DeleteRegKey HKCU "${RC_MENU_KEY}"
  DeleteRegKey HKCU "${RC_FOLDER_MENU_KEY}"
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
