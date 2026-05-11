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
    'Pseudo data printer',
    'Prints fake metrics every 10 seconds using echo',

    -- install_script
    $$
#!/bin/bash

# No installation required
echo "Nothing to install"
$$,

    -- run_script
    $$
#!/bin/bash

# Infinite loop with pseudo data
while true; do
    echo "===== $(date) ====="

    echo "CPU Usage: $((RANDOM % 100))%"
    echo "RAM Usage: $((RANDOM % 32000)) MB"
    echo "Disk Usage: $((RANDOM % 100))%"
    echo "Network RX: $((RANDOM % 1000)) KB/s"
    echo "Network TX: $((RANDOM % 1000)) KB/s"

    echo ""
    sleep 10
done
$$,

    -- delete_script
    $$
#!/bin/bash

# No cleanup required
echo "Nothing to clean"
$$,

    'always',
    'stopped'
);