#!/bin/sh
set -e
chown status /data 2>/dev/null || true
exec su-exec status /usr/local/bin/spacecorps-status
