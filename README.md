# Standup parser
sup is an IRC standup parser. It aims to replace the
[sup script](https://github.com/tompreston/dotfiles/blob/426d3bb430830fd4423e768f29eac9cdbd88115d/local/bin/sup)
and also serves as a vehicle for learning Rust.

Goals:
- [x] Replace the [sup script](https://github.com/tompreston/dotfiles/blob/master/local/bin/sup)
- Support multiple IRC log formats
- [x] Weechat
- [ ] Quassel
- [ ] irssi

Out of/removed from scope:
- Push standup logs to wiki. This logic should live in a Makefile in the wiki.
- Create new standup notes. Not much point automating a `cp sup-template.md
  ab001.md`, when it happens so infrequently.

Contributions and review welcome.

## Getting started
The sup program expects the following environment variables:

	export SUP_PATTERN_BEGIN="## Thomas Preston (tpreston)"
	export SUP_DIR_IRC_LOGS="$WEECHAT_HOME/logs"
	export SUP_DIR_NOTES="$HOME/w/standup"

Install:

	git clone https://github.com/tompreston/sup.git
	cd sup
	cargo install --path .

Run:
	
	sup edit ab001
	sup format
	sup format ab001 "Discussion"
	sup format celduin "William Salmon" > path/to/wiki/logs/standup-20200604.md
