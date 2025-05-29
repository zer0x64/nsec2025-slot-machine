# Wonderlight Slot Machine
## Flag
FLAG-0f84e7c4569c26ffa72e21a48162de833ac57eb5bfffdddf9031603fca3792fc

## Controls
SpaceBar - Lever + select button
Up Arrow - Up button
Down Arrow - Down button
B - Insert badge
Shift+B - Toggle badge "insert/remove" badge

## TroubleShooting
You need to install FUSE to run AppImages.

Tested on Ubuntu 24.04.2 LTS

Depending on your setup, the following environment variables could help:
____NV_DISABLE_EXPLICIT_SYNC=1 (for Nvidia GPUs)
LIBGL_ALWAYS_SOFTWARE=1
WEBKIT_DISABLE_COMPOSITING_MODE=1
WEBKIT_DISABLE_DMABUF_RENDERER=1
WAYLAND_DISPLAY= (delete the variable, forces use of Xorg instead of Wayland)

If nothing works, try runninng it on a new Ubuntu VM.
