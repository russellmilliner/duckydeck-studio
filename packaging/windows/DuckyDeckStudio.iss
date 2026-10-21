; Inno Setup script for DuckyDeck Studio (Windows x64).

#ifndef AppVersion
  #define AppVersion "0.1.6"
#endif
#ifndef SourceDir
  #define SourceDir "..\..\target\windows-dist\bundle"
#endif
#ifndef OutputDir
  #define OutputDir "..\..\target\release"
#endif

#define AppName "DuckyDeck Studio"
#define AppExe "duckydeck-studio.exe"
#define AppUserModelId "io.github.duckydeck.Studio"
#define AppUrl "https://github.com/russellmilliner/duckydeck-studio"

[Setup]
AppId={{8D4F6A2E-5C3B-4E91-B2A7-9F1C3D8E7A60}
AppName={#AppName}
AppVersion={#AppVersion}
AppVerName={#AppName} {#AppVersion}
AppPublisher=Russell Milliner
AppPublisherURL={#AppUrl}
AppSupportURL={#AppUrl}/issues
AppUpdatesURL={#AppUrl}/releases
VersionInfoVersion={#AppVersion}
VersionInfoProductName={#AppName}
VersionInfoDescription={#AppName} Setup
PrivilegesRequired=lowest
PrivilegesRequiredOverridesAllowed=dialog
DefaultDirName={autopf}\{#AppName}
DefaultGroupName={#AppName}
DisableProgramGroupPage=yes
OutputDir={#OutputDir}
OutputBaseFilename=DuckyDeck-Studio-Setup-v{#AppVersion}-x86_64
SetupIconFile=duckydeck-studio.ico
UninstallDisplayIcon={app}\{#AppExe}
UninstallDisplayName={#AppName}
Compression=lzma2/max
SolidCompression=yes
WizardStyle=modern
ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible
MinVersion=10.0
CloseApplications=force
RestartApplications=no

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[Tasks]
Name: "autostart"; Description: "Start {#AppName} when I sign in to Windows (minimized to system tray)"; GroupDescription: "Startup:"
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked

[Files]
Source: "{#SourceDir}\*"; DestDir: "{app}"; Flags: ignoreversion recursesubdirs createallsubdirs

[Icons]
Name: "{autoprograms}\{#AppName}"; Filename: "{app}\{#AppExe}"; AppUserModelID: "{#AppUserModelId}"; Comment: "Configurator, DSVM v2 Compiler & Auto-Switcher for duckyPad Pro"
Name: "{autodesktop}\{#AppName}"; Filename: "{app}\{#AppExe}"; AppUserModelID: "{#AppUserModelId}"; Tasks: desktopicon

[Registry]
Root: HKCU; Subkey: "Software\Microsoft\Windows\CurrentVersion\Run"; ValueType: string; ValueName: "DuckyDeckStudio"; ValueData: """{app}\{#AppExe}"" --minimized"; Flags: uninsdeletevalue; Tasks: autostart
Root: HKCU; Subkey: "Software\Classes\AppUserModelId\{#AppUserModelId}"; ValueType: string; ValueName: "DisplayName"; ValueData: "{#AppName}"; Flags: uninsdeletekey

[Run]
Filename: "{app}\{#AppExe}"; Description: "{cm:LaunchProgram,{#AppName}}"; Flags: nowait postinstall skipifsilent

[UninstallRun]
Filename: "{sys}\taskkill.exe"; Parameters: "/IM {#AppExe} /F"; Flags: runhidden; RunOnceId: "StopDuckyDeckStudio"

[Code]
procedure CurUninstallStepChanged(CurUninstallStep: TUninstallStep);
begin
  if CurUninstallStep = usPostUninstall then
  begin
    RegDeleteValue(HKEY_CURRENT_USER, 'Software\Microsoft\Windows\CurrentVersion\Run', 'DuckyDeckStudio');
    if MsgBox('Also delete your DuckyDeck Studio profiles cache and settings?' + #13#10 +
              '(' + ExpandConstant('{userappdata}\duckydeck-studio') + ')',
              mbConfirmation, MB_YESNO or MB_DEFBUTTON2) = IDYES then
    begin
      DelTree(ExpandConstant('{userappdata}\duckydeck-studio'), True, True, True);
      DelTree(ExpandConstant('{localappdata}\duckydeck-studio'), True, True, True);
    end;
  end;
end;
