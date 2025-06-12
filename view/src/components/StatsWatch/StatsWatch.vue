<template>
  <div class="stats-watch">
    <div class="stats-header">
      <h3>Server Statistics</h3>
      <div class="status-indicator">
        <span class="status-dot" :class="{ 'online': isConnected, 'offline': !isConnected }"></span>
        <span class="status-text">{{ isConnected ? 'Connected' : 'Disconnected' }}</span>
      </div>
    </div>
    
    <div v-if="stats" class="stats-grid">
      <!-- Execution Results -->
      <div class="stats-card">
        <h4>Query Results</h4>
        <div class="stats-row">
          <span class="label">Total Queries:</span>
          <span class="value">{{ stats.execution_results?.total_queries || 0 }}</span>
        </div>
        <div class="stats-row">
          <span class="label">Successful:</span>
          <span class="value success">{{ stats.execution_results?.successful_queries || 0 }}</span>
        </div>
        <div class="stats-row">
          <span class="label">Failed (Expected):</span>
          <span class="value warning">{{ stats.execution_results?.failed_expected_queries || 0 }}</span>
        </div>
        <div class="stats-row">
          <span class="label">Failed (New):</span>
          <span class="value error">{{ stats.execution_results?.failed_new_queries || 0 }}</span>
        </div>
        <div class="stats-row">
          <span class="label">Error Rate:</span>
          <span class="value" :class="{ 'error': errorRate > 10, 'warning': errorRate > 5 }">
            {{ errorRate }}%
          </span>
        </div>
      </div>

      <!-- Performance -->
      <div class="stats-card">
        <h4>Performance</h4>
        <div class="stats-row">
          <span class="label">Max Exec Time:</span>
          <span class="value">{{ stats.performance?.max_execution_time_ms || 0 }}ms</span>
        </div>
        <div class="stats-row">
          <span class="label">Queries/sec:</span>
          <span class="value">{{ (stats.performance?.queries_per_second || 0).toFixed(2) }}</span>
        </div>
        <div class="stats-row">
          <span class="label">Total Threads:</span>
          <span class="value">{{ stats.performance?.total_thread_count || 0 }}</span>
        </div>
        <div class="stats-row">
          <span class="label">Last Update:</span>
          <span class="value small">{{ formatTimestamp(stats.timestamp) }}</span>
        </div>
      </div>

      <!-- Statement Types (if available) -->
      <div v-if="hasStatementTypes" class="stats-card">
        <h4>Statement Types</h4>
        <div v-for="(count, type) in stats.execution_results?.stmt_type_counts" :key="type" class="stats-row">
          <span class="label">{{ type }}:</span>
          <span class="value">{{ count }}</span>
          <span class="percentage">
            {{ ((count / totalStatements) * 100).toFixed(1) }}%
          </span>
        </div>
      </div>
    </div>

    <div v-else-if="stats === null" class="loading">
      <p>Loading statistics...</p>
    </div>

    <div v-else class="no-data">
      <p>{{ stats.message || 'No executor statistics collected yet' }}</p>
    </div>

    <div class="controls">
      <button @click="toggleAutoRefresh" :class="{ 'active': autoRefresh }">
        {{ autoRefresh ? 'Pause' : 'Resume' }} Auto Refresh
      </button>
      <button @click="fetchStats">Refresh Now</button>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted, computed } from 'vue';
import { fetchStats as fetchStatsAPI } from '../request.js';

const stats = ref(null);
const isConnected = ref(false);
const autoRefresh = ref(true);
let intervalId = null;

// Compute error rate with fallback
const errorRate = computed(() => {
  const errorRateValue = stats.value?.execution_results?.error_rate || 0;
  return parseFloat(errorRateValue).toFixed(1);
});

// Check if statement types are available
const hasStatementTypes = computed(() => {
  const stmtCounts = stats.value?.execution_results?.stmt_type_counts;
  return stmtCounts && Object.keys(stmtCounts).length > 0;
});

// Total statements for percentage calculation
const totalStatements = computed(() => {
  return stats.value?.execution_results?.total_queries || 1;
});

// Fetch stats from the server
const fetchStats = async () => {
  try {
    const data = await fetchStatsAPI();
    stats.value = data;
    isConnected.value = true;
  } catch (error) {
    console.error('Failed to fetch stats:', error);
    isConnected.value = false;
  }
};

// Toggle auto-refresh functionality
const toggleAutoRefresh = () => {
  autoRefresh.value = !autoRefresh.value;
  if (autoRefresh.value) {
    startAutoRefresh();
  } else {
    stopAutoRefresh();
  }
};

// Start auto-refresh interval
const startAutoRefresh = () => {
  if (intervalId) clearInterval(intervalId);
  intervalId = setInterval(fetchStats, 1000); // Fetch every second
};

// Stop auto-refresh interval
const stopAutoRefresh = () => {
  if (intervalId) {
    clearInterval(intervalId);
    intervalId = null;
  }
};

// Format timestamp for display
const formatTimestamp = (timestamp) => {
  if (!timestamp) return 'N/A';
  return new Date(timestamp).toLocaleTimeString();
};

// Lifecycle hooks
onMounted(() => {
  fetchStats(); // Initial fetch
  if (autoRefresh.value) {
    startAutoRefresh();
  }
});

onUnmounted(() => {
  stopAutoRefresh();
});
</script>

<style src="./StatsWatch.css" scoped></style>
