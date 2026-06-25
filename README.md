# Pulse

**Pulse** is a simple website status checker built in Rust.

It checks whether a website is reachable, shows the HTTP status code, measures response time, and detects redirects.

## Features

* Check if a website is up or down
* Automatically adds `https://` if missing
* Shows HTTP status code
* Shows response time in milliseconds
* Detects redirects
* Uses a timeout so requests do not hang forever

## Usage

```bash
pulse <website>
```

### Examples

```bash
pulse google.com
```

```bash
pulse https://github.com
```

```bash
pulse help
```

## Example Output

```txt
Target:      https://www.google.com
Status:      UP
Code:        200 OK
Time:        84ms
Redirected:  No
```

For a redirect:

```txt
Target:      https://youtube.com
Status:      REDIRECTED
Code:        301 Moved Permanently
Time:        132ms
Redirected:  Yes
Location:    https://www.youtube.com/
```

## How It Works

Pulse sends an HTTP GET request to the target website using reqwest package.

It uses a custom HTTP client with:

* A 5 second timeout
* Redirect following disabled

This allows Pulse to detect redirects instead of automatically following them.

## Roadmap

Planned features:

* Better terminal colors
* JSON output
* Check multiple websites from a file
* Watch mode
* History tracking
* Redirect chain inspection
* API health checks

## Tech Stack

* Rust
* reqwest for making http requests
* colored for colored output in terminal

## Why I Built This

I built Pulse as a beginner-friendly but real CLI tool to learn Rust, HTTP, structs, clean error handling, and terminal-based developer tools.

## License

MIT
