# Output configuration
Output assets/demo.gif

# --- CINEMATIC SETTINGS ---
Set Width 1425
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
# SCENE 2: The CLI & Presets
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
Type "i"
Sleep 1s

# 7. Fade Out
Type "q"
Sleep 250ms
Type "clear"
Sleep 500ms
Enter
Sleep 2s
