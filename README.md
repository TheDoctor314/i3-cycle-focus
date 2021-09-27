## i3-cycle-focus

A simple tool to cyclically switch between the windows on the active workspace.

### Installing
```bash
git clone https://github.com/TheDoctor314/i3-cycle-focus.git
cd i3-cycle-focus
cargo install --path .
```
### Usage
Add this to your i3 config
>bindsym $mod+Tab exec --no-startup-id "/path/to/binary/ cycle"

>bindsym $mod+Shift+Tab exec --no-startup-id "/path/to/binary/ reverse"