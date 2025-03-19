#!/bin/bash

# Start the Vite dev server in the background using nohup
# This will keep it running even if you close the terminal
nohup npm run dev > dev-server.log 2>&1 &

# Display the process ID so you can stop it later if needed
echo "Development server started with PID: $!"
echo "To stop the server, run: kill $!"
echo "Log file available at: $(pwd)/dev-server.log"
echo "Your application should be accessible at http://poepi.local/filament-tracker/"
