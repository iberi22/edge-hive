#!/usr/bin/env python3
"""
Edge Hive Telegram Bot

Interacts with the Edge Hive VPS via HTTP API and MCP tools.
Allows users to create edge functions, check status, and execute functions remotely.
"""

import os
import sys
import json
import asyncio
import httpx
from telegram import Update, InlineKeyboardButton, InlineKeyboardMarkup
from telegram.ext import (
    Application,
    CommandHandler,
    CallbackQueryHandler,
    ContextTypes,
    MessageHandler,
    filters,
)

# Configuration
TELEGRAM_BOT_TOKEN = os.getenv("TELEGRAM_BOT_TOKEN", "YOUR_BOT_TOKEN_HERE")
EDGE_HIVE_BASE_URL = os.getenv("EDGE_HIVE_URL", "http://localhost:8080")

# OAuth2 client credentials (create via: edge-hive auth client create)
EDGE_HIVE_CLIENT_ID = os.getenv("EDGE_HIVE_CLIENT_ID", "")
EDGE_HIVE_CLIENT_SECRET = os.getenv("EDGE_HIVE_CLIENT_SECRET", "")


class EdgeHiveClient:
    """HTTP client for Edge Hive MCP/API"""

    def __init__(self, base_url: str, client_id: str, client_secret: str):
        self.base_url = base_url
        self.client_id = client_id
        self.client_secret = client_secret
        self.access_token = None
        self.http_client = httpx.AsyncClient(timeout=30.0, verify=False)

    async def get_token(self) -> str:
        """Obtain OAuth2 access token"""
        if self.access_token:
            return self.access_token

        if not self.client_id or not self.client_secret:
            # Create a client if credentials don't exist
            resp = await self.http_client.post(
                f"{self.base_url}/mcp/auth/clients",
                json={"name": "telegram-bot", "scopes": ["mcp:read", "mcp:call"]},
            )
            resp.raise_for_status()
            client_data = resp.json()
            self.client_id = client_data["client_id"]
            self.client_secret = client_data["client_secret"]
            print(
                f"✅ Created OAuth2 client: {self.client_id} (save these credentials!)"
            )

        # Request token
        token_resp = await self.http_client.post(
            f"{self.base_url}/mcp/auth/token",
            json={
                "grant_type": "client_credentials",
                "client_id": self.client_id,
                "client_secret": self.client_secret,
                "scope": "mcp:call",
            },
        )
        token_resp.raise_for_status()
        token_data = token_resp.json()
        self.access_token = token_data["access_token"]
        return self.access_token

    async def mcp_tool_call(self, tool_name: str, arguments: dict) -> dict:
        """Call an MCP tool via HTTP"""
        token = await self.get_token()
        headers = {"Authorization": f"Bearer {token}"}
        payload = {
            "jsonrpc": "2.0",
            "id": 1,
            "method": "tools/call",
            "params": {"name": tool_name, "arguments": arguments},
        }
        resp = await self.http_client.post(
            f"{self.base_url}/mcp/tools/call",
            headers=headers,
            json=payload,
        )
        resp.raise_for_status()
        return resp.json()

    async def get_health(self) -> dict:
        """Check VPS health"""
        resp = await self.http_client.get(f"{self.base_url}/health")
        resp.raise_for_status()
        return resp.json()

    async def list_edge_functions(self) -> list:
        """List edge functions via HTTP API"""
        resp = await self.http_client.get(f"{self.base_url}/api/v1/edge")
        resp.raise_for_status()
        return resp.json()

    async def execute_edge_function(self, name: str, payload: dict) -> dict:
        """Execute an edge function"""
        resp = await self.http_client.post(
            f"{self.base_url}/api/v1/edge/{name}",
            json=payload,
        )
        resp.raise_for_status()
        return resp.json()


# Global client instance
edge_client = EdgeHiveClient(
    EDGE_HIVE_BASE_URL, EDGE_HIVE_CLIENT_ID, EDGE_HIVE_CLIENT_SECRET
)


async def start(update: Update, context: ContextTypes.DEFAULT_TYPE) -> None:
    """Send welcome message"""
    keyboard = [
        [
            InlineKeyboardButton("📊 Status", callback_data="status"),
            InlineKeyboardButton("🔧 Edge Functions", callback_data="edge_list"),
        ],
        [
            InlineKeyboardButton("➕ Create Function", callback_data="edge_create"),
            InlineKeyboardButton("❓ Help", callback_data="help"),
        ],
    ]
    reply_markup = InlineKeyboardMarkup(keyboard)

    await update.message.reply_text(
        "🚀 *Edge Hive VPS Bot*\n\n"
        "Control your Edge Hive node from Telegram!\n\n"
        "Choose an option below:",
        reply_markup=reply_markup,
        parse_mode="Markdown",
    )


async def button_callback(
    update: Update, context: ContextTypes.DEFAULT_TYPE
) -> None:
    """Handle button presses"""
    query = update.callback_query
    await query.answer()

    if query.data == "status":
        try:
            health = await edge_client.get_health()
            status_resp = await edge_client.mcp_tool_call("get_status", {})
            content = status_resp.get("content", [{}])[0].get("text", "No data")

            await query.edit_message_text(
                f"📊 *Node Status*\n\n"
                f"Health: `{health['status']}`\n"
                f"Version: `{health['version']}`\n\n"
                f"```\n{content}\n```",
                parse_mode="Markdown",
            )
        except Exception as e:
            await query.edit_message_text(f"❌ Error: {str(e)}")

    elif query.data == "edge_list":
        try:
            functions = await edge_client.list_edge_functions()
            mcp_list = await edge_client.mcp_tool_call("edge_function_list", {})
            mcp_content = (
                json.loads(mcp_list.get("content", [{}])[0].get("text", "{}"))
                .get("functions", [])
            )

            text = "🔧 *Edge Functions*\n\n"
            text += "*HTTP Endpoint Functions:*\n"
            for fn in functions:
                text += f"• `{fn['name']}` ({fn['runtime']})\n"

            text += f"\n*MCP-Created Functions:*\n"
            for fn_name in mcp_content:
                text += f"• `{fn_name}`\n"

            await query.edit_message_text(text, parse_mode="Markdown")
        except Exception as e:
            await query.edit_message_text(f"❌ Error: {str(e)}")

    elif query.data == "edge_create":
        await query.edit_message_text(
            "📝 To create an edge function, send:\n\n"
            "`/create <name> <template_json>`\n\n"
            "Example:\n"
            "`/create greet {\"message\": \"Hello from Telegram!\"}`",
            parse_mode="Markdown",
        )

    elif query.data == "help":
        await query.edit_message_text(
            "❓ *Help*\n\n"
            "*Commands:*\n"
            "/start - Show main menu\n"
            "/status - Get node status\n"
            "/list - List edge functions\n"
            "/create <name> <json> - Create function\n"
            "/run <name> <payload> - Execute function\n\n"
            "*Buttons:*\n"
            "Use the inline keyboard for quick actions",
            parse_mode="Markdown",
        )


async def create_function_command(
    update: Update, context: ContextTypes.DEFAULT_TYPE
) -> None:
    """Create an edge function"""
    if len(context.args) < 2:
        await update.message.reply_text(
            "Usage: /create <name> <template_json>\n"
            "Example: /create greet {\"message\": \"hello\"}"
        )
        return

    name = context.args[0]
    template_str = " ".join(context.args[1:])

    try:
        template = json.loads(template_str)
        result = await edge_client.mcp_tool_call(
            "edge_function_create", {"name": name, "template": template}
        )
        content_text = result.get("content", [{}])[0].get("text", "Unknown response")

        await update.message.reply_text(f"✅ {content_text}")
    except json.JSONDecodeError:
        await update.message.reply_text("❌ Invalid JSON template")
    except Exception as e:
        await update.message.reply_text(f"❌ Error: {str(e)}")


async def run_function_command(
    update: Update, context: ContextTypes.DEFAULT_TYPE
) -> None:
    """Execute an edge function"""
    if len(context.args) < 1:
        await update.message.reply_text(
            "Usage: /run <name> [payload_json]\n"
            "Example: /run greet {\"user\": \"Alice\"}"
        )
        return

    name = context.args[0]
    payload_str = " ".join(context.args[1:]) if len(context.args) > 1 else "{}"

    try:
        payload = json.loads(payload_str)
        result = await edge_client.execute_edge_function(name, payload)

        await update.message.reply_text(
            f"✅ *Execution Result*\n\n"
            f"```json\n{json.dumps(result, indent=2)}\n```",
            parse_mode="Markdown",
        )
    except json.JSONDecodeError:
        await update.message.reply_text("❌ Invalid JSON payload")
    except Exception as e:
        await update.message.reply_text(f"❌ Error: {str(e)}")


async def status_command(update: Update, context: ContextTypes.DEFAULT_TYPE) -> None:
    """Quick status check"""
    try:
        health = await edge_client.get_health()
        await update.message.reply_text(
            f"📊 *Status*\n\n"
            f"Health: `{health['status']}`\n"
            f"Version: `{health['version']}`",
            parse_mode="Markdown",
        )
    except Exception as e:
        await update.message.reply_text(f"❌ Error: {str(e)}")


async def list_command(update: Update, context: ContextTypes.DEFAULT_TYPE) -> None:
    """List edge functions"""
    try:
        functions = await edge_client.list_edge_functions()
        text = "🔧 *Edge Functions*\n\n"
        for fn in functions:
            text += f"• `{fn['name']}` ({fn['runtime']})\n"

        await update.message.reply_text(text, parse_mode="Markdown")
    except Exception as e:
        await update.message.reply_text(f"❌ Error: {str(e)}")


def main() -> None:
    """Run the bot"""
    if TELEGRAM_BOT_TOKEN == "YOUR_BOT_TOKEN_HERE":
        print("❌ Please set TELEGRAM_BOT_TOKEN environment variable")
        sys.exit(1)

    application = Application.builder().token(TELEGRAM_BOT_TOKEN).build()

    # Handlers
    application.add_handler(CommandHandler("start", start))
    application.add_handler(CommandHandler("status", status_command))
    application.add_handler(CommandHandler("list", list_command))
    application.add_handler(CommandHandler("create", create_function_command))
    application.add_handler(CommandHandler("run", run_function_command))
    application.add_handler(CallbackQueryHandler(button_callback))

    print("🚀 Edge Hive Telegram Bot started!")
    print(f"   Connecting to: {EDGE_HIVE_BASE_URL}")
    application.run_polling(allowed_updates=Update.ALL_TYPES)


if __name__ == "__main__":
    main()
