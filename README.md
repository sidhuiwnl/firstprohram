# CommitAI

AI-powered git commit message generator using Google's Gemini API.

## Installation

```bash
cargo install commitai
```

## Setup

1. Get a Google API key from [Google AI Studio](https://makersuite.google.com/app/apikey)

2. Create a config file:

Linux/macOS:
```bash
mkdir -p ~/.config/commitai
echo "GOOGLE_API_KEY=your_api_key_here" > ~/.config/commitai/config
```

Windows:
```powershell
mkdir -p $env:APPDATA\commitai
echo "GOOGLE_API_KEY=your_api_key_here" > $env:APPDATA\commitai\config
```

## Usage

1. Stage your changes:
```bash
git add .
```

2. Generate commit message:
```bash
commitai
```

3. Choose what to do with the generated message:
- `y` - Use the message and commit
- `e` - Edit the message before committing
- `n` - Cancel without committing

## Features

- Generates contextual commit messages based on staged changes
- Follows conventional commit format
- Supports message editing with your preferred editor
- Color-coded status messages
- Automatic retry with backoff

## License

This project is licensed under the MIT License - see the LICENSE file for details.