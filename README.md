# Health Reminder

**Stay hydrated. Keep moving. Stay in your flow.**

Health Reminder is a lightweight Windows tray app that nudges you to drink water and take movement breaks throughout the day. Set your own reminder intervals, keep it running quietly in the background, and get back to what you were doing.

## Why Health Reminder?

Long stretches at a desk make it easy to forget the basics. Health Reminder brings gentle, practical prompts to your Windows desktop without adding another dashboard or account to manage.

## Features

- **Hydration and movement reminders** with adjustable schedules
- **Purpose-specific notification icons** for water and movement reminders
- **Quiet tray experience** with a tooltip showing what reminder is next
- **Quick pause and resume** whenever you need uninterrupted time
- **Personalized intervals:** water every 15, 30, or 60 minutes; movement every 30, 60, or 120 minutes
- **English, Spanish, and Portuguese** interface and notifications
- **Optional Windows startup** so reminders are ready when you are
- **Test notification** to check that reminders are working
- **Dependency-free Rust app** using native Windows features

## Get started

### Build from source

Install Rust with a Windows target, then run:

```powershell
cargo build --release
```

The executable will be created at `target\release\health-reminder.exe`. Launch it to find Health Reminder in the Windows system tray.

### Choose a language on first launch

English is the default. To start in Spanish or Portuguese, set `HEALTH_LANG` before launching for the first time:

```powershell
$env:HEALTH_LANG = "es" # Spanish; use "pt" for Portuguese
```

You can also change the language later from the tray menu.

## How to use it

Right-click the tray icon to open the menu. From there, you can:

- Pause or resume reminders
- Open **Settings** to change language, toggle **Start with Windows**, or send a test notification
- Open **Intervals** to adjust hydration and movement schedules
- Open **About** to see the version and developer attribution
- Exit the app

Your preferences are saved for your Windows user and restored the next time you launch the app.

## Built with

Health Reminder is written in Rust and uses native Windows APIs for its tray menu, notifications, and startup option. It has no third-party Rust dependencies.

## About

Version **2026.01**

Developed by **Rafael Tinoco** using Claude.
