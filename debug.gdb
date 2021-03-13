# Connect to the remote target.
target extended-remote :3333

# Enable semi-hosting.
monitor arm semihosting enable

# Load the application.
load

# Set a breakpoint on our entry to main.
break main

# Start execution.
continue
