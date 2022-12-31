# osu-directer

osu-directer is a simple utility for Windows that you configure as your default browser, which will then directly download any osu beatmap links, for non-beatmap links it will launch the configured browser.

Big thanks to [Joergen Tjernoe](https://github.com/jorgenpt) for making [bichrome](https://github.com/jorgenpt/bichrome), the project that I forked to make my life easier.

## Installation

### Windows

1. Download `osu-directer-win64.exe` from [the latest release](https://github.com/ArjixWasTaken/osu-directer/releases/latest).
2. Move it to its permanent home -- e.g. creating a directory in `%localappdata%\Programs` called `osu-directer` and putting it there.
3. Run `bichrome-win64.exe` once by double clicking it. This will register bichrome as a potential browser.
4. Configure bichrome as your default browser by opening "Default Apps" (You can open your start menu and just type "Default Apps") and clicking the icon under "Web browser", and picking bichrome.

That's it! Now just create a configuration file named `osu_directer_config.json` next to `osu-directer-win64.exe` (see [the configuration section](#config) for details).


## `osu_directer_config.json`

```json
{
   
}
```

`osu_directer_config.json` is auto generated if it does not exist, it can be found in the same directory as the exe.

## License

The source code is licensed under the [MIT](http://opensource.org/licenses/MIT) Licence
