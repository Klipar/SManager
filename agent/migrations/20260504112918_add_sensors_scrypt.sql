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
#!/usr/bin/env bash
set -e

echo "Checking for lm-sensors..."

# Check if sensors command exists
if command -v sensors &> /dev/null; then
    echo "lm-sensors is already installed"
    exit 0
fi

echo "lm-sensors not found - attempting to install..."

# Detect OS and try to install
if [ -f /etc/arch-release ]; then
    echo "Detected Arch Linux - installing via pacman..."
    pacman -Sy --noconfirm lm_sensors 2>/dev/null && echo "Successfully installed lm-sensors" || echo "Failed to install lm-sensors (may require sudo or different package manager)"
elif [ -f /etc/debian_version ]; then
    echo "Detected Debian/Ubuntu - installing via apt..."
    apt-get update 2>/dev/null && apt-get install -y lm-sensors 2>/dev/null && echo "Successfully installed lm-sensors" || echo "Failed to install lm-sensors (may require sudo)"
elif [ -f /etc/redhat-release ]; then
    echo "Detected RedHat/CentOS - installing via yum..."
    yum install -y lm_sensors 2>/dev/null && echo "Successfully installed lm-sensors" || echo "Failed to install lm-sensors (may require sudo)"
else
    echo "Unknown OS - cannot automatically install lm-sensors"
fi

# Final check
if command -v sensors &> /dev/null; then
    echo "lm-sensors is ready"
else
    echo "WARNING: lm-sensors still not available - run script may fail"
fi
$$,

    -- run_script
    $$
#!/usr/bin/env bash
set -e

# Loop to print temperatures every 30 seconds
while true; do
    echo "===== $(date) ====="
    if command -v sensors &> /dev/null; then
        sensors 2>/dev/null || echo "ERROR: Failed to read sensors"
    else
        echo "ERROR: sensors command not available - install script may not have completed"
    fi
    echo ""
    sleep 30
done
$$,

    -- delete_script
    $$
#!/usr/bin/env bash
set -e

echo "CPU temperature monitor stopped"
$$,

    'always',
    'stopped'
);