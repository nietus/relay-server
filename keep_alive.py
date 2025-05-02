#!/usr/bin/env python3
import requests
import time
import sys
import logging
from datetime import datetime

# Configure logging
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s',
    handlers=[
        logging.StreamHandler(sys.stdout)
    ]
)

logger = logging.getLogger('keep_alive')

# Default URL for Render deployment
DEFAULT_URL = "https://relay-server-nzhu.onrender.com/health"

def keep_alive(url, interval_minutes=4):
    """
    Ping a URL at regular intervals to keep a service alive.
    
    Args:
        url: The URL to ping
        interval_minutes: How often to ping (in minutes)
    """
    logger.info(f"Starting keep-alive service for {url}")
    logger.info(f"Will ping every {interval_minutes} minutes")
    
    interval_seconds = interval_minutes * 60
    
    while True:
        try:
            logger.info(f"Pinging {url}")
            response = requests.get(url, timeout=10)
            logger.info(f"Response: {response.status_code} - {response.text[:100]}")
        except Exception as e:
            logger.error(f"Failed to ping {url}: {e}")
        
        # Sleep until next interval
        next_time = datetime.now().timestamp() + interval_seconds
        next_time_str = datetime.fromtimestamp(next_time).strftime('%H:%M:%S')
        logger.info(f"Next ping at {next_time_str}")
        time.sleep(interval_seconds)

if __name__ == "__main__":
    if len(sys.argv) < 2:
        logger.info(f"No URL provided, using default: {DEFAULT_URL}")
        url = DEFAULT_URL
    else:
        url = sys.argv[1]
    
    interval = int(sys.argv[2]) if len(sys.argv) > 2 else 4
    
    keep_alive(url, interval) 