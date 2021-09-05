# Slack API

## Post example

First, create .json file for slack

```config.json
{
	"url": "<URL to post slack api>",
	"channel": "<channel name>",
	"username": "<username used in post message>"
}
```

And then, you have to pass it's file path as command line arguments.

```/bin/bash
cargo run --bin rust_library_sandbox -- slack post --config <slack config.json path>
```
