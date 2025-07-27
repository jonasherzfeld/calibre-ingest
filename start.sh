#!/bin/sh

# Set default values if not provided
USER_ID=${USER_ID:-1000}
GROUP_ID=${GROUP_ID:-1000}

echo "Starting Calibre Ingest with USER_ID=$USER_ID and GROUP_ID=$GROUP_ID"

# Create group if it does not exist
if ! getent group $GROUP_ID >/dev/null 2>&1; then
    addgroup -g $GROUP_ID appgroup
    GROUP_NAME=appgroup
    echo "Created group: appgroup ($GROUP_ID)"
else
    GROUP_NAME=$(getent group $GROUP_ID | cut -d: -f1)
    echo "Using existing group: $GROUP_NAME ($GROUP_ID)"
fi

# Create user if it does not exist
if ! getent passwd $USER_ID >/dev/null 2>&1; then
    adduser -D -u $USER_ID -G $GROUP_NAME appuser
    USER_NAME=appuser
    echo "Created user: appuser ($USER_ID)"
else
    USER_NAME=$(getent passwd $USER_ID | cut -d: -f1)
    echo "Using existing user: $USER_NAME ($USER_ID)"
fi

# Set ownership of uploads directory
chown $USER_NAME:$GROUP_NAME /uploads
echo "Set ownership of /uploads to $USER_NAME:$GROUP_NAME"

# Start backend as the specified user
echo "Starting backend as user $USER_NAME..."
su-exec $USER_NAME /usr/local/bin/calibre-ingest-backend &

# Start nginx as root (required for port 80)
echo "Starting nginx..."
nginx -g "daemon off;"
