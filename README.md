# Farcaster Frames Template

Template for jumpstarting production-ready Farcaster frames quickly.

— no fuss, all fun!

## Features

- **Seamless Integration**: Easily create frames for Farcaster with Rust's performance.
- **Rapid Deployment**: Utilize Vercel for quick and effortless deployment of your frames.
- **Local Testing Support**: Includes setup for `localtunnel` to facilitate easy local testing and development.

## Prerequisites

Ensure you have the following installed to use this template effectively:

- Rust and Cargo
- Localtunnel
- Vercel CLI

## Getting Started

### Clone and Setup

1. Clone the repository to get started:

    ```bash
    git clone https://github.com/jpgonzalezra/farcaster-frames-template.git
    cd farcaster-frames-template
    ```

2. Install `vercel` globally:

    ```bash
    npm install -g vercel
    ```

### Run Locally

Use the provided `Makefile` to streamline the local development process:

- when you run `make dev` if you do not have "localtunnel" installed it is installed

```bash
make dev
```

Each time you run `make dev`, a unique URL is generated, such as https://{something}.loca.lt. This URL can be directly copied and used to test your frame with the [Warpcast Embed Tools](https://warpcast.com/~/developers/frames), significantly streamlining the development and testing workflow.

### Deployment

Deploy your frame to Vercel with ease:

1. Log in to your Vercel account through the CLI:

```bash
vercel login
```

2. Deploy your project with Vercel:

```bash
vercel
```

Follow the prompts to link and deploy your project.

### Contributions

We welcome contributions to improve this template further. Please feel free to submit pull requests or open issues with your suggestions and feedback.

