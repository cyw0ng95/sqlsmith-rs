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
      <!-- Executor Stats -->
      <div class="stats-card">
        <h4>Executors</h4>
        <div class="stats-row">
          <span class="label">Total:</span>
          <span class="value">{{ stats.executor_stats?.total_executors || 0 }}</span>
        </div>
        <div class="stats-row">
          <span class="label">Active:</span>
          <span class="value active">{{ stats.executor_stats?.active_executors || 0 }}</span>
        </div>
        <div class="stats-row">
          <span class="label">Completed:</span>
          <span class="value">{{ stats.executor_stats?.completed_executors || 0 }}</span>
        </div>
      </div>

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
          <span class="label">Failed:</span>
          <span class="value error">{{ stats.execution_results?.failed_queries || 0 }}</span>
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
          <span class="label">Avg Exec Time:</span>
          <span class="value">{{ stats.performance?.avg_execution_time_ms || 0 }}ms</span>
        </div>
        <div class="stats-row">
          <span class="label">Queries/sec:</span>
          <span class="value">{{ stats.performance?.queries_per_second || 0 }}</span>
        </div>
        <div class="stats-row">
          <span class="label">Uptime:</span>
          <span class="value">{{ formatUptime(stats.performance?.uptime_seconds || 0) }}</span>
        </div>
      </div>

      <!-- System -->
      <div class="stats-card">
        <h4>System</h4>
        <div class="stats-row">
          <span class="label">Memory:</span>
          <span class="value">{{ (stats.system?.memory_usage_mb || 0).toFixed(1) }}MB</span>
        </div>
        <div class="stats-row">
          <span class="label">CPU Usage:</span>
          <span class="value">{{ (stats.system?.cpu_usage_percent || 0).toFixed(1) }}%</span>
        </div>
        <div class="stats-row">
          <span class="label">Last Update:</span>
          <span class="value small">{{ formatTimestamp(stats.timestamp) }}</span>
        </div>
      </div>
    </div>

    <div v-else class="loading">
      <p>Loading statistics...</p>
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
import { fetchStats as fetchStatsAPI } from './request.js';

const stats = ref(null);
const isConnected = ref(false);
const autoRefresh = ref(true);
let intervalId = null;

const errorRate = computed(() => {
  if (!stats.value?.execution_results) return 0;
  return parseFloat(stats.value.execution_results.error_rate || 0).toFixed(1);
});

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

const toggleAutoRefresh = () => {
  autoRefresh.value = !autoRefresh.value;
  if (autoRefresh.value) {
    startAutoRefresh();
  } else {
    stopAutoRefresh();
  }
};

const startAutoRefresh = () => {
  if (intervalId) clearInterval(intervalId);
  intervalId = setInterval(fetchStats, 1000); // Fetch every second
};

const stopAutoRefresh = () => {
  if (intervalId) {
    clearInterval(intervalId);
    intervalId = null;
  }
};

const formatUptime = (seconds) => {
  const hours = Math.floor(seconds / 3600);
  const minutes = Math.floor((seconds % 3600) / 60);
  const secs = seconds % 60;
  
  if (hours > 0) {
    return `${hours}h ${minutes}m ${secs}s`;
  } else if (minutes > 0) {
    return `${minutes}m ${secs}s`;
  } else {
    return `${secs}s`;
  }
};

const formatTimestamp = (timestamp) => {
  if (!timestamp) return '';
  return new Date(timestamp).toLocaleTimeString();
};

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

<style scoped>
.stats-watch {
  padding: 20px;
  background: #f8f9fa;
  border-radius: 8px;
  margin: 20px 0;
}

.stats-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
}

.stats-header h3 {
  margin: 0;
  color: #333;
}

.status-indicator {
  display: flex;
  align-items: center;
  gap: 8px;
}

.status-dot {
  width: 10px;
  height: 10px;
  border-radius: 50%;
  transition: background-color 0.3s;
}

.status-dot.online {
  background-color: #28a745;
}

.status-dot.offline {
  background-color: #dc3545;
}

.status-text {
  font-size: 14px;
  font-weight: 500;
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
  gap: 20px;
  margin-bottom: 20px;
}

.stats-card {
  background: white;
  padding: 16px;
  border-radius: 6px;
  border: 1px solid #e9ecef;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}

.stats-card h4 {
  margin: 0 0 12px 0;
  color: #495057;
  font-size: 16px;
  border-bottom: 1px solid #e9ecef;
  padding-bottom: 8px;
}

.stats-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 8px;
}

.stats-row:last-child {
  margin-bottom: 0;
}

.label {
  font-size: 14px;
  color: #6c757d;
}

.value {
  font-weight: 600;
  font-size: 14px;
  color: #333;
}

.value.small {
  font-size: 12px;
  font-weight: 400;
}

.value.active {
  color: #007bff;
}

.value.success {
  color: #28a745;
}

.value.error {
  color: #dc3545;
}

.value.warning {
  color: #ffc107;
}

.loading {
  text-align: center;
  padding: 40px;
  color: #6c757d;
}

.controls {
  display: flex;
  gap: 10px;
  justify-content: center;
  border-top: 1px solid #e9ecef;
  padding-top: 16px;
}

.controls button {
  padding: 8px 16px;
  border: 1px solid #007bff;
  background: white;
  color: #007bff;
  border-radius: 4px;
  cursor: pointer;
  font-size: 14px;
  transition: all 0.2s;
}

.controls button:hover {
  background: #007bff;
  color: white;
}

.controls button.active {
  background: #007bff;
  color: white;
}
</style>
