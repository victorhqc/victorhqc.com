#!/bin/bash

# Configuration
SERVER="<url>"
USER="<user>"
REMOTE_FILE="/<path>/analytics.db"
LOCAL_DESTINATION="./analytics.db"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${YELLOW}Starting download from ${USER}@${SERVER}...${NC}"

# Download the file using scp
scp "${USER}@${SERVER}:${REMOTE_FILE}" "${LOCAL_DESTINATION}"

# Check if download was successful
if [ $? -eq 0 ]; then
    echo -e "${GREEN}✓ File downloaded successfully to ${LOCAL_DESTINATION}${NC}"
    ls -lh "${LOCAL_DESTINATION}"
else
    echo -e "${RED}✗ Error: Failed to download file${NC}"
    exit 1
fi
