# Output configuration
Output assets/demo.gif

# --- CINEMATIC SETTINGS ---
Set Width 1400
Set Height 800
Set LineHeight 1.2 
Set Padding 20 
Set Framerate 60
Set TypingSpeed 75ms

# --- FONT SETUP ---
Set FontFamily "JetBrains Mono"
Set FontSize 20

# --- THEME: Catppuccin Mocha ---
Set Theme "Catppuccin Mocha"

# --- COLOR FIDELITY SETTINGS ---
# Still looks pale after this for some reason
Env TERM "xterm-256color"
Env COLORTERM "truecolor"

# --- SETUP (Hidden) ---
Hide
Type "export PATH=$PATH:$(pwd)/target/release"
Enter
Type "clear"
Enter
Show

# ==========================================
# SCENE 1: The Main Dashboard
# ==========================================

# 1. Launch
Type "statui"
Sleep 500ms
Enter

# 2. The "Drama" Pause
Sleep 3s 

# 3. Smooth Navigation
Type "j"
Sleep 700ms
Type "j"
Sleep 700ms
Type "j"
Sleep 700ms
Type "k"
Sleep 1s

# 4. The Inspector
Type "i"
# Long pause here to let the viewer read the stats
Sleep 4s 

# Scroll the table while Inspector is open
Type "j"
Sleep 2s
Type "j"
Sleep 2s
Type "j"
Sleep 2s

# Close Inspector
Type "i"
Sleep 2s

# 5. Quit Scene 1
Type "q"
Sleep 1.5s

# ==========================================
# SCENE 2: The CLI & Presets (Cloud)
# ==========================================

# Clear the screen to signal a new context
Type "clear" 
Enter
Sleep 500ms

# 6. Run with Arguments
# This shows off the CLI parsing and the presets feature
Type "statui presets/cloud_infra.toml"
Sleep 1.5s 
Enter
Sleep 3s

Type "j"
Sleep 700ms
Type "j"
Sleep 700ms
Type "k"
Sleep 1s

# Open inspector
Type "i"
Sleep 2s 

# Scroll the table while Inspector is open
Type "j"
Sleep 1s
Type "j"
Sleep 1s
Type "k"
Sleep 500ms
Type "k"
Sleep 500ms
Type "k"
Sleep 2s
Type "k"
Sleep 300ms
Type "k"
Sleep 300ms
Type "k"
Sleep 2s

# Close inspector
Type "i"
Sleep 1s

# Quit Scene 2
Type "q"
Sleep 1.5s

# ==========================================
# SCENE 3: Dev Registries (Fast Scan)
# ==========================================

# Clear for final scene
Type "clear"
Enter
Sleep 500ms

# 7. Run with Dev Registries
Type "statui presets/dev_registries.toml"
Sleep 1.5s
Enter
Sleep 3s

# Fast scroll down (simulating scanning/searching visually)
Type "i"
Sleep 1s
Type "j"
Sleep 1s
Type "j"
Sleep 250ms
Type "j"
Sleep 250ms
Type "j"
Sleep 250ms
Sleep 3s

# Move back up to settle on an interesting item (e.g. PyPI or NPM)
Type "k"
Sleep 500ms
Type "k"
Sleep 3s

Type "i"
Sleep 1.5s

# 8. Fade Out / End
Type "q"
Sleep 700ms
Type "clear"
Sleep 500ms
Enter
Sleep 2s