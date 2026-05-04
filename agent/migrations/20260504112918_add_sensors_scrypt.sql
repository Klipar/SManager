INSERT INTO tasks (
    core_id,
    name,
    description,
    install_script,
    run_script,
    delete_script,
    restart_policy,
    status
) VALUES (
    NULL,
    'CPU temperature monitor',
    'Prints CPU and sensor temperatures every 30 seconds',

    -- install_script
    $$
#!/bin/bash

# Detect OS
if [ -f /etc/arch-release ]; then
    sudo pacman -Sy --noconfirm lm_sensors
elif [ -f /etc/debian_version ]; then
    sudo apt update
    sudo apt install -y lm-sensors
else
    echo "Unsupported OS"
    exit 1
fi

sudo sensors-detect --auto
$$,

    -- run_script
    $$
#!/bin/bash

# Loop to print temperatures
while true; do
    echo "===== $(date) ====="
    sensors
    echo ""
    sleep 30
done
$$,

    -- delete_script
    $$
#!/bin/bash

# Optional cleanup (does nothing critical)
echo "No cleanup required"
$$,

    'always',
    'stopped'
);