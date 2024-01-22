# ZCue

ZCue is a cross-platform, open-source project to extract or apply cue points from wave/wav files of certain games developed by Zipper Interactive™:

* the Recoil™ game (1999)
* the MechWarrior 3™ base game (1999)
* the MechWarrior 3 Pirate's Moon™ expansion (1999)

Zipper Interactive™ was trademark or registered trademark of Sony Computer Entertainment America LLC. Other trademarks belong to the respective rightsholders.

Obviously, this is an unofficial fan effort and not connected to the developers, publishers, or rightsholders. [Join us on MW3 Discord](https://discord.gg/Be53gMy), or the Recoil Discord!

## How do I use this?

Command-line knowledge is required. The following examples are for bash-like shells:

Extracting cue points from `br340000.wav` and write the result to `br340000.json`:

```bash
zcue extract "br340000.wav" "br340000.json"
```

Applying cue points from `br340000.json` to `br340000.wav` and write the result to `br340000-edit.wav`:

```bash
zcue apply "br340000.wav" "br340000.json" "br340000-edit.wav"
```

## Which sounds files are supported?

The low quality sounds seem to have weird cue points. Therefore, only sounds from these archives (ZBDs) are supported:

* For MechWarrior 3, this will only work with sounds from `soundsH`.
* For Recoil, this will only work with sounds from `soundsm` or `soundsh`.
* For Pirate's Moon, this will only work with sounds from `soundsH`.

## Changelog

### [0.1.0] - 2024-01-21

* Initial release

## Release procedure

1. Review changelog, and add the date
1. Bump version in `Cargo.toml`
1. Commit, push, and wait for CI
1. Create a tag of the version with the date (e.g. `git tag -a v0.1.0 -m "2024-01-21"`)
1. Push the tag (`git push origin v0.1.0`)
1. The build will automatically create a release as a draft
1. Add changelog items to the release notes via the GitHub web interface
1. Publish the release via the GitHub web interface

## License

Licensed under the European Union Public Licence (EUPL) 1.2 ([LICENSE](LICENSE) or https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12).
