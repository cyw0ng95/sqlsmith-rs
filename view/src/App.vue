<script setup>
import ProfileForm from './components/ProfileForm.vue'
import StatsWatch from './components/StatsWatch'

</script>

<template>
  <div id="app">
    <main class="main-flex">
      <ProfileForm />
      <StatsWatch />
    </main>
  </div>
</template>

<style scoped>
.main-flex {
  display: flex;
  flex-direction: row;
  gap: 32px;
  align-items: flex-start;
  justify-content: center;
  min-height: 80vh;
}
</style>
