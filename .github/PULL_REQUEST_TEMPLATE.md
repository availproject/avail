# Pull Request type

<!-- Check the [contributing guide](../../CONTRIBUTING.md) -->

<!-- Please try to limit your pull request to one type; submit multiple pull requests if needed. -->

Please add the labels corresponding to the type of changes your PR introduces:

- [ ] Feature
- [ ] Bugfix
- [ ] Refactor
- [ ] Format
- [ ] Documentation
- [ ] Testing
- [ ] Other:

## Description
<!-- Summarize the changes made in this pull request. Include the motivation for these changes and highlight any key updates. -->

## Related Issues
<!-- List any related issues or bug numbers this pull request is intended to address. Use GitHub's linking feature to automatically close the issues when the pull request is merged (e.g., "Closes #123"). -->

## Testing Performed
<!-- Describe any testing you performed on these changes, including unit tests, integration tests, manual testing, etc. -->

## Checklist
- [ ] I have performed a self-review of my own code.
- [ ] The tests pass successfully with `cargo test`.
- [ ] The code was formatted with `cargo fmt`.
- [ ] The code compiles with no new warnings with `cargo build --release` and `cargo build --release --features runtime-benchmarks`.
- [ ] The code has no new warnings when using `cargo clippy`.
- [ ] If this change affects documented features or needs new documentation, I have created a PR with a [documentation update](https://github.com/availproject/availproject.github.io).
