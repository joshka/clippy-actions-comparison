# Comparison of Clippy GitHub Actions

As of August 2025, there are 13 [GitHub actions](https://github.com/marketplace?query=clippy) for
running Clippy, the Rust linter. This document compares them based on their features and usage.
11 of these are actual clippy actions, with 2 non-relevant results that appear in the search.

## Actions

|Action Name | Repository | Stars | Commit Activity | Last Commit | Description |
|------------|-----------|-------|--------------|-------------|-------------|
| [clippy-check][cc] | [LoliGothick/clippy-check][cc-repo] | ![cc-stars] | ![cc-commits] | ![cc-last] | Run clippy and annotate the diff with errors and warnings |
| [clippy-action][ca] | [auguwu/clippy-action][ca-repo] | ![ca-stars] | ![ca-commits] | ![ca-last] | 🐻‍❄️📦 GitHub action to run Clippy, an up-to-date and modern version of actions-rs/clippy |
| [rust-clippy-check][rcc] | [actions-rs/clippy-check][rcc-repo] | ![rcc-stars] | ![rcc-commits] | ![rcc-last] | Run clippy and annotate the diff with errors and warnings |
| [qs-rust-clippy][qrc] | [qernal/github-actions-rust-clippy][qrc-repo] | ![qrc-stars] | ![qrc-commits] | ![qrc-last] | Rust Clippy (linter) |
| [rust-clippy-reborn][rcr] | [crusty-pie/clippy][rcr-repo] | ![rcr-stars] | ![rcr-commits] | ![rcr-last] | Run clippy and annotate the diff with errors and warnings |
| [apply-clippy-lints][acl] | [fxwiegand/apply-clippy-lints][acl-repo] | ![acl-stars] | ![acl-commits] | ![acl-last] | Let apply-clippy-lints automatically apply the latest clippy lints to your project |
| [hamirmahal-clippy-check][hcc] | [hamirmahal/clippy-check][hcc-repo] | ![hcc-stars] | ![hcc-commits] | ![hcc-last] | Run clippy and annotate the diff with errors and warnings |
| [clippy-with-reviewdog][cwr] | [sksat/action-clippy][cwr-repo] | ![cwr-stars] | ![cwr-commits] | ![cwr-last] | run clippy with reviewdog |
| [rs-clippy-check][rscc] | [clechasseur/rs-clippy-check][rscc-repo] | ![rscc-stars] | ![rscc-commits] | ![rscc-last] | Run clippy and annotate the diff with errors and warnings |
| [run-clippy-with-reviewdog][rcwr] | [giraffate/clippy-action][rcwr-repo] | ![rcwr-stars] | ![rcwr-commits] | ![rcwr-last] | 🐶 Run Clippy with reviewdog on pull requests to improve code review experience |
| [actions-rs-plus-clippy-check][arscc] | [actions-rs-plus/clippy-check][arscc-repo] | ![arscc-stars] | ![arscc-commits] | ![arscc-last] | Run clippy and annotate the diff with errors and warnings |

**Not relevant results (appear in search but not clippy-focused):**

- [BluBracket Community Edition](https://github.com/marketplace/blubracket-community-edition): BluBracket is like Clippy for code security, but—not as annoying and a lot more effective.
- [rs-doc-check](https://github.com/unk1ndled/rs-doc-check): Run rustdoc and annotate the diff with errors and warnings (fork of rs-clippy-check).

This repo will have a branch for each action to test the CI output.

[cc]: https://github.com/marketplace/actions/clippy-check
[cc-repo]: https://github.com/LoliGothick/clippy-check
[cc-stars]: https://img.shields.io/github/stars/LoliGothick/clippy-check
[cc-commits]: https://img.shields.io/github/commit-activity/m/LoliGothick/clippy-check
[cc-last]: https://img.shields.io/github/last-commit/LoliGothick/clippy-check

[ca]: https://github.com/marketplace/actions/clippy-action
[ca-repo]: https://github.com/auguwu/clippy-action
[ca-stars]: https://img.shields.io/github/stars/auguwu/clippy-action
[ca-commits]: https://img.shields.io/github/commit-activity/m/auguwu/clippy-action
[ca-last]: https://img.shields.io/github/last-commit/auguwu/clippy-action

[rcc]: https://github.com/marketplace/actions/rust-clippy-check
[rcc-repo]: https://github.com/actions-rs/clippy-check
[rcc-stars]: https://img.shields.io/github/stars/actions-rs/clippy-check
[rcc-commits]: https://img.shields.io/github/commit-activity/m/actions-rs/clippy-check
[rcc-last]: https://img.shields.io/github/last-commit/actions-rs/clippy-check

[qrc]: https://github.com/marketplace/actions/qs-rust-clippy
[qrc-repo]: https://github.com/qernal/github-actions-rust-clippy
[qrc-stars]: https://img.shields.io/github/stars/qernal/github-actions-rust-clippy
[qrc-commits]: https://img.shields.io/github/commit-activity/m/qernal/github-actions-rust-clippy
[qrc-last]: https://img.shields.io/github/last-commit/qernal/github-actions-rust-clippy

[rcr]: https://github.com/marketplace/actions/rust-clippy-reborn
[rcr-repo]: https://github.com/crusty-pie/clippy
[rcr-stars]: https://img.shields.io/github/stars/crusty-pie/clippy
[rcr-commits]: https://img.shields.io/github/commit-activity/m/crusty-pie/clippy
[rcr-last]: https://img.shields.io/github/last-commit/crusty-pie/clippy

[acl]: https://github.com/marketplace/actions/apply-clippy-lints
[acl-repo]: https://github.com/fxwiegand/apply-clippy-lints
[acl-stars]: https://img.shields.io/github/stars/fxwiegand/apply-clippy-lints
[acl-commits]: https://img.shields.io/github/commit-activity/m/fxwiegand/apply-clippy-lints
[acl-last]: https://img.shields.io/github/last-commit/fxwiegand/apply-clippy-lints

[hcc]: https://github.com/marketplace/actions/hamirmahal-clippy-check
[hcc-repo]: https://github.com/hamirmahal/clippy-check
[hcc-stars]: https://img.shields.io/github/stars/hamirmahal/clippy-check
[hcc-commits]: https://img.shields.io/github/commit-activity/m/hamirmahal/clippy-check
[hcc-last]: https://img.shields.io/github/last-commit/hamirmahal/clippy-check

[cwr]: https://github.com/marketplace/actions/clippy-with-reviewdog
[cwr-repo]: https://github.com/sksat/action-clippy
[cwr-stars]: https://img.shields.io/github/stars/sksat/action-clippy
[cwr-commits]: https://img.shields.io/github/commit-activity/m/sksat/action-clippy
[cwr-last]: https://img.shields.io/github/last-commit/sksat/action-clippy

[rscc]: https://github.com/marketplace/actions/rs-clippy-check
[rscc-repo]: https://github.com/clechasseur/rs-clippy-check
[rscc-stars]: https://img.shields.io/github/stars/clechasseur/rs-clippy-check
[rscc-commits]: https://img.shields.io/github/commit-activity/m/clechasseur/rs-clippy-check
[rscc-last]: https://img.shields.io/github/last-commit/clechasseur/rs-clippy-check

[rcwr]: https://github.com/marketplace/actions/run-clippy-with-reviewdog
[rcwr-repo]: https://github.com/giraffate/clippy-action
[rcwr-stars]: https://img.shields.io/github/stars/giraffate/clippy-action
[rcwr-commits]: https://img.shields.io/github/commit-activity/m/giraffate/clippy-action
[rcwr-last]: https://img.shields.io/github/last-commit/giraffate/clippy-action

[arscc]: https://github.com/marketplace/actions/actions-rs-plus-clippy-check
[arscc-repo]: https://github.com/actions-rs-plus/clippy-check
[arscc-stars]: https://img.shields.io/github/stars/actions-rs-plus/clippy-check
[arscc-commits]: https://img.shields.io/github/commit-activity/m/actions-rs-plus/clippy-check
[arscc-last]: https://img.shields.io/github/last-commit/actions-rs-plus/clippy-check
