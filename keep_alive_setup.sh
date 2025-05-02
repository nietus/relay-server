#!/bin/bash

# This script sets up and runs the keep-alive service
# for the Relay Server on Render

# Install Python dependencies
echo "Installing required Python packages..."
pip install requests

# Run keep-alive script
echo "Starting keep-alive service for the Relay Server..."
echo "Press Ctrl+C to stop"
python keep_alive.py

# For running as a background service
# nohup python keep_alive.py > keep_alive.log 2>&1 & 