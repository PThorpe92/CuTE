# Type:
#   Type[@<time>] "<characters>"    Type <characters> into the terminal with a
#                                   <time> delay between each character
#
# Keys:
#   Escape[@<time>] [number]        Press the Escape key
#   Backspace[@<time>] [number]     Press the Backspace key
#   Down[@<time>] [number]          Press the Down key
#   Enter[@<time>] [number]         Press the Enter key
#   Space[@<time>] [number]         Press the Space key
#   Tab[@<time>] [number]           Press the Tab key
#   Left[@<time>] [number]          Press the Left Arrow key
#   Right[@<time>] [number]         Press the Right Arrow key
#   Up[@<time>] [number]            Press the Up Arrow key
#   Down[@<time>] [number]          Press the Down Arrow key
#   PageUp[@<time>] [number]        Press the Page Up key
#   PageDown[@<time>] [number]      Press the Page Down key
#   Ctrl+<key>                      Press the Control key + <key> (e.g. Ctrl+C)
#
# Display:
#   Hide                            Hide the subsequent commands from the output
#   Show                            Show the subsequent commands in the output

Output examples/demo.gif

Require echo

Set Shell "bash"
Set FontSize 32
Set Width 1800
Set Height 1200

Show Type "/home/dvektor/Projects/cURLtui-rs/target/debug/cURLtui-rs"
Sleep 900ms
Enter
Down
Sleep 900ms
Down
Sleep 900ms
Up
Sleep 900ms
Up
Sleep 900ms
Enter
Sleep 900ms
Down
Sleep 900ms
Enter
Sleep 900ms
Down
Sleep 700ms
Down
Sleep 700ms
Up
Sleep 500ms
Up
Sleep 700ms
Enter
Type "i"
Sleep 800ms
Show Type "https://github.com/eza-community/eza"
Sleep 900ms
Enter
Sleep 900ms
Down
Sleep 300ms
Down 
Sleep 300ms
Down
Sleep 300ms
Enter
Sleep 900ms
Down
Sleep 400ms
Down 
Sleep 400ms
Up
Sleep 400ms
Up 
Sleep 400ms
Enter
Sleep 4000ms
Enter
Sleep 4000ms
Type "h"
Sleep 2000ms 
Type "q"

