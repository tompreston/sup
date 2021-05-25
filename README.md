# Standup parser (archived)
No code is better, just store standups as pre-formatted text and copy-paste using ctrl+shift block
copy.

---

sup is an IRC standup parser. It aims to replace the
[sup script](https://github.com/tompreston/dotfiles/blob/426d3bb430830fd4423e768f29eac9cdbd88115d/local/bin/sup)
and also serves as a vehicle for learning Rust - so please pull me up on
anything which is nonidiomatic.

Tasks:
- [x] Replace the [sup script](https://github.com/tompreston/dotfiles/blob/master/local/bin/sup)
- [x] Add clap alias for single letter variants.
- Support multiple IRC log formats
    - [x] Weechat
    - [ ] Quassel
    - [ ] irssi

Out of/removed from scope:
- Push standup logs to wiki. This logic should live in a Makefile in the wiki.
- Create new standup notes. Not much point automating a `cp sup-template.md
  ab001.md`, when it happens so infrequently.

## Getting started
The sup program expects the following environment variables:

	export SUP_PATTERN_BEGIN="## Thomas Preston (tpreston)"
	export SUP_DIR_IRC_LOGS="$WEECHAT_HOME/logs"
	export SUP_DIR_NOTES="$HOME/w/standup"

Install:

	git clone https://github.com/tompreston/sup.git
	cd sup
	cargo install --path .

Edit project standup notes throughout the day:
	
	sup edit ab001

Print project standup notes, for pasting in IRC standup:

	sup show ab001 Discussion
	sup show ab001 Di
	sup show ab001 William Salmon
	sup show ab001 W

Print formatted project standup notes, for pasting in the project wiki:

	sup format
	sup format ab001
	sup format celduin > path/to/wiki/logs/2020-06-04-standup.md
