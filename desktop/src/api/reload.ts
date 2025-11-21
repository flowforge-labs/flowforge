export async function reloadLogs() {
    const response = await fetch("/api/reload", 
        {method: "POST"}
    );

    if (!response.ok) {
        throw new Error("Failed to reload logs");
    }

    return await response.json();
}