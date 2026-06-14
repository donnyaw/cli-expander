# evdev and Uinput Platform Layer

## Context
Implemented keyboard detection via evdev and text injection with multiple fallback methods.

## Finding
- `evdev::Device::open(path)` returns `io::Result<Device>` — the path is a string like `/dev/input/event0`
- `device.fetch_events()` returns `Result<FetchEventsSynced>` which implements `Iterator<Item = InputEvent>`
- `InputEvent` has `.event_type()`, `.code()`, `.value()` methods
- `EventType::KEY` is the correct constant for keyboard events
- Value 1 = press, 0 = release, 2 = autorepeat
- `/dev/input/by-path/` contains symlinks to actual event devices — files with `-kbd` in name are keyboards
- uinput requires either `input` group or running as root
- ydotool is a good fallback when uinput is unavailable
- tmux `send-keys -l` works for tmux-native injection
- arboard crate provides cross-platform clipboard access

## Resolution
EvdevKeySource scans /dev/input/by-path/ for keyboards. Injector trait with UinputInjector (tmux/ydotool), TmuxInjector, ClipboardInjector.

## Tags
evdev, uinput, input, keyboard, injection, permissions
