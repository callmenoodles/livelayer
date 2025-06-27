# Live Layer
Live Layer is a wallpaper utility allowing users to set an HTML or web wallpaper.

> [!IMPORTANT]
> The current version of Live Layer only supports wlroots-based Wayland compositors.

> [!NOTE]
> Live Layer works on tiling Wayland compositors.

## Usage
### Local
```bash
$ livelayer path/to/wallpaper.html
```
### Remote
```bash
$ livelayer https://example.com
```
---
> [!TIP]
> If you have a multi-monitor setup and launch Live Layer with your compositor, it's possible it doesn't detect the monitor at first, thus rendering the wallpaper on a different monitor. In this case, chain the `livelayer` command with `sleep`:
> ```
> $ sleep 2; livelayer /path/to/wallpaper.html
> ```

### Output to Specific Monitor
```bash
$ livelayer path/to/wallpaper.html -o <monitor>
```
