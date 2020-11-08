# New Home MQTT Server

This is the backend side for the MQTT frontend. It is currently used to store all saved shortcuts and distribute the settings to the frontend.

## The Frontend

[The frontend can be found here](https://github.com/YannikSc/new-home-mqtt)

## Planned

- ~~Store dashboards~~ Done
- ~~Store groups used in the dashboard~~ Done
- Trigger MQTT topics when other topics arrive
  - + With payload checking

## Install (Cross-Compile for Raspberry PI 3b+) 

**Note:** For installing the server files with the provided scripts your PI has to have `make` installed

```bash
# Create the archive for the PI on your machine
make packInstallArchive TARGET=arm-unknown-linux-gnueabihf

# Copy it to your Raspberry PI
# This command assumes, that you have a working connection to your PI through the alias "raspberry"
scp ./new-home-mqtt-server.tar.gz raspberry:.

# Extract the archive on your PI
ssh raspberry
tar -xvzf new-home-mqtt-server.tar.gz
cd new-home-mqtt-server

# Install the files for the application
sudo make install

# Run the service
sudo systemctl daemon-reload
sudo systemctl start new-home-mqtt-server

# Optionally enable for autostart
sudo systemctl enable new-home-mqtt-server
```

---

# New Home

"New Home" is a project which is meant for automating your home with buttons sensors and more. It's a jung
project, but I'm trying my best to get it reasonably working.
