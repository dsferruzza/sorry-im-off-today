# Sorry I'm Off Today

[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![pipeline status](https://gitlab.com/dsferruzza/sorry-im-off-today/badges/master/pipeline.svg)](https://gitlab.com/dsferruzza/sorry-im-off-today/commits/master)
[![Crates.io Version](https://img.shields.io/crates/v/sorry-im-off-today.svg)](https://crates.io/crates/sorry-im-off-today)
[![Get help on Codementor](https://cdn.codementor.io/badges/get_help_github.svg)](https://www.codementor.io/dsferruzza?utm_source=github&utm_medium=button&utm_term=dsferruzza&utm_campaign=github)

Automatically update your Slack status if your Google Calendar says that you are off today.

⚠️ _Main repository is here: https://gitlab.com/dsferruzza/sorry-im-off-today_ ⚠️

## How to use

- compile or install the project
- just run the `sorry-im-off-today` executable

Your Google calendar will be fetched, parsed and searched for events beginning by `Absent` or `Congés`.
If such an event exists today, your Slack status will be updated.

### Configuration

Configuration is done using environment variables:

| Name | Default Value | Description |
|---|---|---|
| `CALENDAR_URL` |  | The private URL of a Google calendar. For example: `https://calendar.google.com/calendar/ical/xxxxx%40gmail.com/private-xxxxx/basic.ics`. |
| `SLACK_API_TOKEN` |  | A Slack access token. You need to create an app with the `users.profile:read` and `users.profile:write` scopes and add it to your Slack workspace. The OAuth token required here starts with `xoxp`. |


### Run automatically

Configure a Cron or a Systemd timer to run it every day (early in the morning) !

## License

MIT License Copyright (c) 2019 David Sferruzza
