# mcp-server-brave-search

Zed extension that wraps the official Brave Search MCP server: [`@brave/brave-search-mcp-server`](https://www.npmjs.com/package/@brave/brave-search-mcp-server). See the upstream project for features and usage details: [brave/brave-search-mcp-server](https://github.com/brave/brave-search-mcp-server/).

## Configuration

This MCP server requires an API key.

1. Sign up for a [Brave Search API account](https://brave.com/search/api/)
2. Choose a plan (Free tier available with 2,000 queries/month)
3. Generate your API key [from the developer dashboard](https://api.search.brave.com/app/keys)

In your Zed settings:
```json
{
    "context_servers": {
        "mcp-server-brave-search": {
          "settings": {
              "brave_api_key": "YOUR_API_KEY",
              "transport": "stdio"
          }
        }
    }
}

```
