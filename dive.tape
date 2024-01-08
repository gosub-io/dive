# Generates a gif of the dive terminal UI through VHS
# For more information, see; https://github.com/charmbracelet/vhs
# Note that this script requires a non-standard version of vhs
# which enables F1-F12 keys to be recorded.
#
Output "dive.gif"
Set Theme "Catppuccin Frappe"
Set Width 1024
Set Height 768
Set FontSize 12
Set TypingSpeed 400ms

# Start dive
Hide
Type@1ms "./target/debug/dive"
Enter
Show

# Wait a bit on the splash screen
Sleep 3
Escape

# Show and scroll help
Sleep 1
F1
Up 8
Down 8
Escape

# Cycle through tab and tab list
Sleep 1
Tab 3
F2
Down
Down
Enter

Sleep 1
F2
Up
Up
Enter

# Bookmark list
Sleep 1
F8
Right
Down
Down
Tab
Down
Down

# And quit program
Sleep 1
Ctrl+Q