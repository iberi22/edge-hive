package com.edgehive.app

import android.os.Bundle
import android.widget.Toast
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.*
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.unit.dp
import androidx.lifecycle.lifecycleScope
import com.edgehive.app.ui.theme.EdgeHiveTheme
import kotlinx.coroutines.launch

class MainActivity : ComponentActivity() {
    private lateinit var edgeHiveClient: EdgeHiveClient

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        // Initialize client
        edgeHiveClient = EdgeHiveClient(
            baseUrl = "http://10.0.2.2:8080", // Android emulator localhost
            // baseUrl = "http://192.168.1.100:8080", // Real device - use your IP
            clientId = "",
            clientSecret = ""
        )

        setContent {
            EdgeHiveTheme {
                Surface(
                    modifier = Modifier.fillMaxSize(),
                    color = MaterialTheme.colorScheme.background
                ) {
                    MainScreen(edgeHiveClient)
                }
            }
        }
    }
}

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun MainScreen(client: EdgeHiveClient) {
    var selectedTab by remember { mutableStateOf(0) }
    val context = LocalContext.current

    Scaffold(
        topBar = {
            TopAppBar(
                title = { Text("Edge Hive VPS") },
                colors = TopAppBarDefaults.topAppBarColors(
                    containerColor = MaterialTheme.colorScheme.primaryContainer,
                    titleContentColor = MaterialTheme.colorScheme.primary,
                )
            )
        },
        bottomBar = {
            NavigationBar {
                NavigationBarItem(
                    icon = { Icon(Icons.Filled.Info, contentDescription = "Status") },
                    label = { Text("Status") },
                    selected = selectedTab == 0,
                    onClick = { selectedTab = 0 }
                )
                NavigationBarItem(
                    icon = { Icon(Icons.Filled.List, contentDescription = "Functions") },
                    label = { Text("Functions") },
                    selected = selectedTab == 1,
                    onClick = { selectedTab = 1 }
                )
                NavigationBarItem(
                    icon = { Icon(Icons.Filled.Add, contentDescription = "Create") },
                    label = { Text("Create") },
                    selected = selectedTab == 2,
                    onClick = { selectedTab = 2 }
                )
            }
        }
    ) { innerPadding ->
        Box(modifier = Modifier.padding(innerPadding)) {
            when (selectedTab) {
                0 -> StatusScreen(client)
                1 -> FunctionsScreen(client)
                2 -> CreateFunctionScreen(client)
            }
        }
    }
}

@Composable
fun StatusScreen(client: EdgeHiveClient) {
    var health by remember { mutableStateOf<HealthResponse?>(null) }
    var status by remember { mutableStateOf<String?>(null) }
    var loading by remember { mutableStateOf(false) }
    var error by remember { mutableStateOf<String?>(null) }
    val scope = rememberCoroutineScope()

    LaunchedEffect(Unit) {
        loading = true
        try {
            health = client.getHealth()
            val statusResult = client.mcpToolCall("get_status", emptyMap())
            status = statusResult.content.firstOrNull()?.text
            error = null
        } catch (e: Exception) {
            error = e.message ?: "Unknown error"
        } finally {
            loading = false
        }
    }

    Box(
        modifier = Modifier
            .fillMaxSize()
            .padding(16.dp),
        contentAlignment = Alignment.Center
    ) {
        when {
            loading -> CircularProgressIndicator()
            error != null -> {
                Column(horizontalAlignment = Alignment.CenterHorizontally) {
                    Text("Error: $error", color = MaterialTheme.colorScheme.error)
                    Spacer(modifier = Modifier.height(8.dp))
                    Button(onClick = {
                        scope.launch {
                            loading = true
                            try {
                                health = client.getHealth()
                                error = null
                            } catch (e: Exception) {
                                error = e.message
                            } finally {
                                loading = false
                            }
                        }
                    }) {
                        Text("Retry")
                    }
                }
            }
            health != null -> {
                Column(
                    modifier = Modifier.fillMaxWidth(),
                    horizontalAlignment = Alignment.Start
                ) {
                    Card(
                        modifier = Modifier.fillMaxWidth(),
                        elevation = CardDefaults.cardElevation(defaultElevation = 4.dp)
                    ) {
                        Column(modifier = Modifier.padding(16.dp)) {
                            Text(
                                "Node Health",
                                style = MaterialTheme.typography.headlineSmall,
                                color = MaterialTheme.colorScheme.primary
                            )
                            Spacer(modifier = Modifier.height(8.dp))
                            Text("Status: ${health!!.status}")
                            Text("Version: ${health!!.version}")
                        }
                    }

                    if (status != null) {
                        Spacer(modifier = Modifier.height(16.dp))
                        Card(
                            modifier = Modifier.fillMaxWidth(),
                            elevation = CardDefaults.cardElevation(defaultElevation = 4.dp)
                        ) {
                            Column(modifier = Modifier.padding(16.dp)) {
                                Text(
                                    "Node Status",
                                    style = MaterialTheme.typography.headlineSmall,
                                    color = MaterialTheme.colorScheme.primary
                                )
                                Spacer(modifier = Modifier.height(8.dp))
                                Text(
                                    status!!,
                                    style = MaterialTheme.typography.bodyMedium,
                                    modifier = Modifier.fillMaxWidth()
                                )
                            }
                        }
                    }
                }
            }
        }
    }
}

@Composable
fun FunctionsScreen(client: EdgeHiveClient) {
    var functions by remember { mutableStateOf<List<EdgeFunction>>(emptyList()) }
    var loading by remember { mutableStateOf(false) }
    var error by remember { mutableStateOf<String?>(null) }
    var selectedFunction by remember { mutableStateOf<String?>(null) }
    var executionResult by remember { mutableStateOf<String?>(null) }

    LaunchedEffect(Unit) {
        loading = true
        try {
            functions = client.listEdgeFunctions()
            error = null
        } catch (e: Exception) {
            error = e.message ?: "Unknown error"
        } finally {
            loading = false
        }
    }

    Column(
        modifier = Modifier
            .fillMaxSize()
            .padding(16.dp)
    ) {
        when {
            loading -> {
                Box(
                    modifier = Modifier.fillMaxSize(),
                    contentAlignment = Alignment.Center
                ) {
                    CircularProgressIndicator()
                }
            }
            error != null -> {
                Text("Error: $error", color = MaterialTheme.colorScheme.error)
            }
            functions.isEmpty() -> {
                Box(
                    modifier = Modifier.fillMaxSize(),
                    contentAlignment = Alignment.Center
                ) {
                    Text("No edge functions found")
                }
            }
            else -> {
                LazyColumn(
                    modifier = Modifier.weight(1f),
                    verticalArrangement = Arrangement.spacedBy(8.dp)
                ) {
                    items(functions) { function ->
                        Card(
                            modifier = Modifier.fillMaxWidth(),
                            onClick = { selectedFunction = function.name },
                            elevation = CardDefaults.cardElevation(defaultElevation = 2.dp)
                        ) {
                            Row(
                                modifier = Modifier
                                    .fillMaxWidth()
                                    .padding(16.dp),
                                horizontalArrangement = Arrangement.SpaceBetween,
                                verticalAlignment = Alignment.CenterVertically
                            ) {
                                Column {
                                    Text(
                                        function.name,
                                        style = MaterialTheme.typography.titleMedium
                                    )
                                    Text(
                                        "Runtime: ${function.runtime}",
                                        style = MaterialTheme.typography.bodySmall,
                                        color = MaterialTheme.colorScheme.secondary
                                    )
                                }
                                Icon(
                                    Icons.Filled.PlayArrow,
                                    contentDescription = "Execute"
                                )
                            }
                        }
                    }
                }

                if (executionResult != null) {
                    Spacer(modifier = Modifier.height(8.dp))
                    Card(
                        modifier = Modifier.fillMaxWidth(),
                        colors = CardDefaults.cardColors(
                            containerColor = MaterialTheme.colorScheme.secondaryContainer
                        )
                    ) {
                        Column(modifier = Modifier.padding(16.dp)) {
                            Text(
                                "Execution Result:",
                                style = MaterialTheme.typography.titleSmall
                            )
                            Spacer(modifier = Modifier.height(4.dp))
                            Text(
                                executionResult!!,
                                style = MaterialTheme.typography.bodySmall
                            )
                        }
                    }
                }
            }
        }
    }

    // Execute function dialog
    if (selectedFunction != null) {
        val scope = rememberCoroutineScope()
        AlertDialog(
            onDismissRequest = { selectedFunction = null },
            title = { Text("Execute: $selectedFunction") },
            text = {
                Text("Execute this edge function with empty payload?")
            },
            confirmButton = {
                TextButton(
                    onClick = {
                        scope.launch {
                            try {
                                val result = client.executeEdgeFunction(
                                    selectedFunction!!,
                                    emptyMap()
                                )
                                executionResult = result.toString()
                            } catch (e: Exception) {
                                executionResult = "Error: ${e.message}"
                            }
                            selectedFunction = null
                        }
                    }
                ) {
                    Text("Execute")
                }
            },
            dismissButton = {
                TextButton(onClick = { selectedFunction = null }) {
                    Text("Cancel")
                }
            }
        )
    }
}

@Composable
fun CreateFunctionScreen(client: EdgeHiveClient) {
    var functionName by remember { mutableStateOf("") }
    var templateJson by remember { mutableStateOf("""{"message": "Hello from Android!"}""") }
    var creating by remember { mutableStateOf(false) }
    var result by remember { mutableStateOf<String?>(null) }
    val scope = rememberCoroutineScope()
    val context = LocalContext.current

    Column(
        modifier = Modifier
            .fillMaxSize()
            .padding(16.dp),
        verticalArrangement = Arrangement.spacedBy(16.dp)
    ) {
        Text(
            "Create Edge Function",
            style = MaterialTheme.typography.headlineSmall,
            color = MaterialTheme.colorScheme.primary
        )

        OutlinedTextField(
            value = functionName,
            onValueChange = { functionName = it },
            label = { Text("Function Name") },
            modifier = Modifier.fillMaxWidth(),
            singleLine = true
        )

        OutlinedTextField(
            value = templateJson,
            onValueChange = { templateJson = it },
            label = { Text("Template JSON") },
            modifier = Modifier
                .fillMaxWidth()
                .height(200.dp),
            maxLines = 10
        )

        Button(
            onClick = {
                scope.launch {
                    creating = true
                    try {
                        val response = client.createEdgeFunction(
                            functionName,
                            templateJson
                        )
                        result = "✅ Function created successfully!"
                        Toast.makeText(
                            context,
                            "Function created!",
                            Toast.LENGTH_SHORT
                        ).show()
                        functionName = ""
                        templateJson = """{"message": "Hello from Android!"}"""
                    } catch (e: Exception) {
                        result = "❌ Error: ${e.message}"
                        Toast.makeText(
                            context,
                            "Error: ${e.message}",
                            Toast.LENGTH_LONG
                        ).show()
                    } finally {
                        creating = false
                    }
                }
            },
            modifier = Modifier.fillMaxWidth(),
            enabled = functionName.isNotBlank() && !creating
        ) {
            if (creating) {
                CircularProgressIndicator(
                    modifier = Modifier.size(20.dp),
                    color = MaterialTheme.colorScheme.onPrimary
                )
            } else {
                Text("Create Function")
            }
        }

        if (result != null) {
            Card(
                modifier = Modifier.fillMaxWidth(),
                colors = CardDefaults.cardColors(
                    containerColor = if (result!!.startsWith("✅"))
                        MaterialTheme.colorScheme.primaryContainer
                    else
                        MaterialTheme.colorScheme.errorContainer
                )
            ) {
                Text(
                    result!!,
                    modifier = Modifier.padding(16.dp)
                )
            }
        }
    }
}
