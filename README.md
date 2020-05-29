# MusicIO
A program that combines multiple audio sources and plays it back to an output (e.g. a sink).
## Starting
On installation, a systemd user unit (musicio.service) will be created. Start/enable it by doing `systemctl --user start musicio.service`.

Also check `systemctl --user status musicio.service` to ensure there were no errors.
## Configuration

    [[sources]]
    name = 'alsa_output.pci-0000_00_1f.3.analog-stereo.monitor'
    description = 'Monitor of Output'

    [[sources]]
    name = 'alsa_input.pci-0000_00_1f.3.analog-stereo'
    description = 'Input'

    [output]
    name = 'null'
This is the default configuration file. It will be created if it doesn't exist in `~/.config/musicio/config`. The configuration file itself is pretty self explanatory.

Each `source` table requires the key/value pairs `name` and `description`. `description` can be left blank. `name` refers to the name of the source (which you can get by running `pacmd list-sources` - do not include the angle brackets).

There can only be one `output` table. Support for multiple outputs and more advanced setups might be added later. `output` requires the key/value pair `name`, which refers to the name of the sink (which you can get by running `pacmd list-sinks` - again, do not include the angle brackets).
