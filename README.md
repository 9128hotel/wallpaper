![Wallpaper](/resource/logo-medium.png)


# <center>A Rust program to set the desktop background on Windows 10</center>

## Executables

`wallpaper.exe` -> The main program

`check.exe` -> Checks if wallpaper.exe should be able to run on your system<br><br>

## Operation notes - Main program

To run the program: run `wallpaper.exe`

I recommend doing this in command prompt for debugging

The path must not have a space in it, or any other non-path characters. 

**An example of a valid path is:**<br>
`C:\Wallpapers\background.jpg`<br>
`"C:\Wallpapers\background.jpg"`<br>

**An example of an invalid path is:**<br>
`C:\Wallpapers Cool\background.jpg`<br>
`C:\\\\Wallpapers\\\\background.jpg`<br>
`C:/Wallpapers/background.jpg`<br>
`C:\Wallpapers\background.jpg\`<br><br>
