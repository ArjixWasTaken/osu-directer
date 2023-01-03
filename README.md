# osu-directer

osu-directer is a simple utility for Windows that you configure as your default browser, which will then directly download any osu beatmap links, for non-beatmap links it will launch the configured browser.

Big thanks to [Joergen Tjernoe](https://github.com/jorgenpt) for making [bichrome](https://github.com/jorgenpt/bichrome), the project that I forked to make my life easier.

## Installation

1. Download `osu-directer-win64.exe` from [the latest release](https://github.com/ArjixWasTaken/osu-directer/releases/latest).
2. Move it to its permanent home -- e.g. creating a directory in `%localappdata%\Programs` called `osu-directer` and putting it there.
3. Run `osu-directer-win64.exe` once by double clicking it. This will register `osu!directer` as a potential browser.
4. Configure `osu!directer` as your default browser by opening "Default Apps" (You can open your start menu and just type "Default Apps") and clicking the icon under "Web browser", and picking `osu!directer`.

That's it!

## Configuration

The configuration file is named `osu_directer.json` and is auto created in the same folder as the exe.

This is the default config:

```json
{
    "browser_path": "auto",
    "custom_osu_path": "auto"
}
```

### Config explanation

-   `browser_path` when left to `auto` will attempt to find firefox, chrome and msedge in that order. <br />
    When it is not `auto` it will use that as the browser. <br />
    Make sure it is the absolute path to an exe.

-   `custom_osu_path` if you did not install osu to a custom directory, just leave it empty or "auto". <br />
    An attempt to find the `osu!.exe` on PATH is made, but by default osu is not on PATH, if it is not on PATH and not in the default install location, then you must specify the absolute path to the `osu!.exe`

`osu_directer.json` is auto generated if it does not exist.

# Contributors

<a href="https://github.com/ArjixWasTaken/osu-directer/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=ArjixWasTaken/osu-directer" />
</a>

## License

The source code is licensed under the [MIT](http://opensource.org/licenses/MIT) Licence
