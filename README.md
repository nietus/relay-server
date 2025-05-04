# BlockChat Relay Server

A relay server for facilitating P2P connections for the BlockChat application.

## Local Development

```bash
cargo run
```

## Railway Deployment

This project is configured for deployment on Railway.

### Deployment Steps

1. Create a Railway account at [railway.app](https://railway.app)
2. Install the Railway CLI:
   ```bash
   npm i -g @railway/cli
   ```
3. Login to Railway:
   ```bash
   railway login
   ```
4. Initialize a new project:
   ```bash
   railway init
   ```
5. Link to your GitHub repository (optional):
   ```bash
   railway link
   ```
6. Deploy the project:
   ```bash
   railway up
   ```

### Environment Variables

No environment variables are required for basic functionality.

### Exposed Services

- The relay server runs on port 8080
- Default Railway domain: https://your-project-name.railway.app

## API Endpoints

- `POST /store` - Store peer information
- `POST /discover` - Discover peer information
- `POST /waiting_punch` - Set up hole punching
- `POST /keep_alive` - Keep peer connection alive
- `POST /passive_wait` - Wait for passive connections
