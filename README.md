<div align = "center">

<img src="assets/banner.png" alias="banner" width="450"/>

_An open-source, customizable cheat for Counter-Strike 2._

</div>

## ⚠️ About This Project

This project was originally based on the code from [snipcola/ProExt](https://github.com/snipcola/ProExt), which is now archived and can’t be updated anymore. To continue maintaining and developing related features, I created this new repository.

## 🖼️ Preview

<div align="center">

![preview](assets/preview.png)

</div>

## 📝 Instructions:

To use ProExt, you can follow two methods.

### Method 1: Download the prebuilt binary

This method is the easiest and is recommended for most. You can find the latest binary [here](https://git.snipcola.com/snipcola/ProExt/raw/branch/main/bin/proext.exe).

### Method 2: Build the application

#### Requirements:

- [PowerShell](https://winget.run/pkg/Microsoft/PowerShell)
- [Git](https://winget.run/pkg/Git/MinGit)
- [Rust](https://rustup.rs)

#### Installation:

1. Clone the repository:

   ```
   git clone https://git.snipcola.com/snipcola/ProExt.git
   ```

2. Enter the directory:

   ```
   cd ProExt
   ```

3. Build the application:

   ```
   ./scripts/deploy.ps1
   ```

4. The binary should be located inside of the `bin` folder.

## ⌨️ Shortcuts:

- `Home` - Show/hide the menu.
- `End` - Exits the application.

## 📋 Features:

- ESP
- RCS
- Aimbot
- Triggerbot
- Crosshair
- Radar
- Bomb Timer
- Spectator List
- Styling
- Configuration

## 💬 Q&A:

- **Does it work in fullscreen?**

  No.

- **Game lags when toggled, what's the fix?**

  Run the following, using the developer console:

  ```
  engine_no_focus_sleep 0
  ```
