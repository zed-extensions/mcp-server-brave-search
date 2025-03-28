# mcp-server-brave-search

Zed extension that wraps the [`@modelcontextprotocol/server-brave-search`](https://www.npmjs.com/package/@modelcontextprotocol/server-brave-search) MCP server.

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
              "brave_api_key": "YOUR_API_KEY"
          }
        }
    }
}

```
