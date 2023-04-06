# Wallpaper

A Rust program to set the desktop background on Windows 10

## Executables

wallpaper.exe -> The main program

check.exe -> Checks if wallpaper.exe should be able to run on your system

## Operation notes - Main program

To run the program: run wallpaper.exe

I recommend doing this in command prompt for debugging

The path must not have a space in it, or any other non-path characters. 

An example of a valid path is:

C:\Wallpapers\background.jpg

An example of an invalid path is:

"C:\Wallpapers\background.jpg"<br>
C:\Wallpapers Cool\background.jpg<br>
C:\\\\Wallpapers\\\\background.jpg<br>
C:/Wallpapers/background.jpg<br>
C:\Wallpapers\background.jpg\

## Issues

### I have bug tested this code to the best of my abilities, but as both the EXE and the code are open source, there is a good chance an organization who do not want you changing windows backgrounds will block this. In the event that this happens, please submit an issue with the title "Blocked" and any details you can provide

Any other problems, please submit a pull request or an issue.

## License, copyright, and contact

The code is licensed under a modified FPA MIT, but that may change in the future. If you'd like to do something with the software that isn't permitted under the license, please shoot me an email! (skye-7aUMex@proton.me)