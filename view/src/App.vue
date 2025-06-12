<script setup>
import ProfileForm from './components/ProfileForm.vue'
import StatsWatch from './components/StatsWatch'

</script>

<template>
  <div id="app">
    <main class="main-flex">
      <div class="left-panel">
        <ProfileForm />
      </div>
      <div class="right-panel">
        <StatsWatch />
      </div>
    </main>
  </div>
</template>

<style scoped>
.main-flex {
  display: flex;
  flex-direction: row;
  gap: 32px;
  align-items: flex-start;
  justify-content: flex-start;
  min-height: 100vh;
  padding: 20px;
}

.left-panel {
  flex: 1;
  min-width: 400px;
  max-width: 600px;
}

.right-panel {
  flex: 1;
  min-width: 400px;
}
</style>
