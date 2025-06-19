# Live Layer
Live Layer is a wallpaper utility allowing users to set a live wallpaper.

> [!IMPORTANT]
> The current version of Live Layer only supports HTML wallpapers and wlroots-based Wayland compositors.

## Usage
### Local Wallpaper
```bash
$ livelayer path/to/wallpaper.html
```
### Remote Wallpaper
```bash
$ livelayer https://example.com
```
---
> [!TIP]
> If you have a multi-monitor setup and launch Live Layer with your compositor, it's possible they don't detect the monitor at first, thus rendering the wallpaper on a different monitor. In this case, chain the `livelayer` command with `sleep`:
> ```
> $ sleep 2; livelayer /path/to/wallpaper
> ```

### Output to Specific Monitor
```bash
$ livelayer path/to/wallpaper.html -o <monitor>
```