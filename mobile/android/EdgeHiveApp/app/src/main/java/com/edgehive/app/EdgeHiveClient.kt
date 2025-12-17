package com.edgehive.app

import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import kotlinx.serialization.*
import kotlinx.serialization.json.*
import okhttp3.*
import okhttp3.MediaType.Companion.toMediaType
import okhttp3.RequestBody.Companion.toRequestBody
import java.io.IOException

@Serializable
data class HealthResponse(
    val status: String,
    val version: String
)

@Serializable
data class EdgeFunction(
    val name: String,
    val runtime: String
)

@Serializable
data class OAuth2ClientResponse(
    val client_id: String,
    val client_secret: String,
    val name: String
)

@Serializable
data class OAuth2TokenResponse(
    val access_token: String,
    val token_type: String,
    val expires_in: Int
)

@Serializable
data class McpToolCallRequest(
    val jsonrpc: String = "2.0",
    val id: Int = 1,
    val method: String = "tools/call",
    val params: McpToolCallParams
)

@Serializable
data class McpToolCallParams(
    val name: String,
    val arguments: JsonObject
)

@Serializable
data class McpToolCallResponse(
    val jsonrpc: String,
    val id: Int,
    val content: List<McpContent>
)

@Serializable
data class McpContent(
    val type: String,
    val text: String
)

class EdgeHiveClient(
    private val baseUrl: String,
    private var clientId: String,
    private var clientSecret: String
) {
    private val client = OkHttpClient.Builder()
        .addInterceptor { chain ->
            val request = chain.request()
            println("HTTP Request: ${request.method} ${request.url}")
            val response = chain.proceed(request)
            println("HTTP Response: ${response.code}")
            response
        }
        .build()

    private val json = Json {
        ignoreUnknownKeys = true
        isLenient = true
        encodeDefaults = true
    }

    private var accessToken: String? = null

    private suspend fun getToken(): String = withContext(Dispatchers.IO) {
        if (accessToken != null) {
            return@withContext accessToken!!
        }

        // Create OAuth2 client if credentials don't exist
        if (clientId.isEmpty() || clientSecret.isEmpty()) {
            val createClientRequest = Request.Builder()
                .url("$baseUrl/mcp/auth/clients")
                .post(
                    """{"name":"android-app","scopes":["mcp:read","mcp:call"]}"""
                        .toRequestBody("application/json".toMediaType())
                )
                .build()

            val createResponse = client.newCall(createClientRequest).execute()
            if (!createResponse.isSuccessful) {
                throw IOException("Failed to create OAuth2 client: ${createResponse.code}")
            }

            val clientData = json.decodeFromString<OAuth2ClientResponse>(
                createResponse.body!!.string()
            )
            clientId = clientData.client_id
            clientSecret = clientData.client_secret
            println("✅ Created OAuth2 client: $clientId")
        }

        // Request access token
        val tokenRequest = Request.Builder()
            .url("$baseUrl/mcp/auth/token")
            .post(
                json.encodeToString(
                    mapOf(
                        "grant_type" to "client_credentials",
                        "client_id" to clientId,
                        "client_secret" to clientSecret,
                        "scope" to "mcp:call"
                    )
                ).toRequestBody("application/json".toMediaType())
            )
            .build()

        val tokenResponse = client.newCall(tokenRequest).execute()
        if (!tokenResponse.isSuccessful) {
            throw IOException("Failed to get access token: ${tokenResponse.code}")
        }

        val tokenData = json.decodeFromString<OAuth2TokenResponse>(
            tokenResponse.body!!.string()
        )
        accessToken = tokenData.access_token
        return@withContext accessToken!!
    }

    suspend fun getHealth(): HealthResponse = withContext(Dispatchers.IO) {
        val request = Request.Builder()
            .url("$baseUrl/health")
            .get()
            .build()

        val response = client.newCall(request).execute()
        if (!response.isSuccessful) {
            throw IOException("Health check failed: ${response.code}")
        }

        json.decodeFromString(response.body!!.string())
    }

    suspend fun listEdgeFunctions(): List<EdgeFunction> = withContext(Dispatchers.IO) {
        val request = Request.Builder()
            .url("$baseUrl/api/v1/edge")
            .get()
            .build()

        val response = client.newCall(request).execute()
        if (!response.isSuccessful) {
            throw IOException("Failed to list functions: ${response.code}")
        }

        json.decodeFromString(response.body!!.string())
    }

    suspend fun executeEdgeFunction(
        name: String,
        payload: Map<String, Any>
    ): JsonObject = withContext(Dispatchers.IO) {
        val request = Request.Builder()
            .url("$baseUrl/api/v1/edge/$name")
            .post(
                json.encodeToString(payload)
                    .toRequestBody("application/json".toMediaType())
            )
            .build()

        val response = client.newCall(request).execute()
        if (!response.isSuccessful) {
            throw IOException("Function execution failed: ${response.code}")
        }

        json.decodeFromString(response.body!!.string())
    }

    suspend fun mcpToolCall(
        toolName: String,
        arguments: Map<String, JsonElement>
    ): McpToolCallResponse = withContext(Dispatchers.IO) {
        val token = getToken()

        val requestBody = McpToolCallRequest(
            params = McpToolCallParams(
                name = toolName,
                arguments = JsonObject(arguments)
            )
        )

        val request = Request.Builder()
            .url("$baseUrl/mcp/tools/call")
            .header("Authorization", "Bearer $token")
            .post(
                json.encodeToString(requestBody)
                    .toRequestBody("application/json".toMediaType())
            )
            .build()

        val response = client.newCall(request).execute()
        if (!response.isSuccessful) {
            throw IOException("MCP tool call failed: ${response.code}")
        }

        json.decodeFromString(response.body!!.string())
    }

    suspend fun createEdgeFunction(
        name: String,
        templateJson: String
    ): McpToolCallResponse {
        val template = json.parseToJsonElement(templateJson) as JsonObject
        return mcpToolCall(
            "edge_function_create",
            mapOf(
                "name" to JsonPrimitive(name),
                "template" to template
            )
        )
    }
}
