<script setup>
import { ref, onMounted } from 'vue'
import HelloWorld from './components/HelloWorld.vue'
import TheWelcome from './components/TheWelcome.vue'

const profileData = ref(null)
const error = ref(null)

onMounted(async () => {
  try {
    const response = await fetch('http://127.0.0.1:8080/profile/get')
    if (!response.ok) {
      throw new Error('Network response was not ok')
    }
    profileData.value = await response.json()
  } catch (err) {
    error.value = err
  }
})
</script>

<template>
  <main>
    <el-button>Default</el-button>
    <div v-if="profileData">
      <pre>{{ JSON.stringify(profileData, null, 2) }}</pre>
    </div>
    <div v-else-if="error">
      <p>Error: {{ error.message }}</p>
    </div>
  </main>
</template>

<style scoped>
header {
  line-height: 1.5;
}

.logo {
  display: block;
  margin: 0 auto 2rem;
}

@media (min-width: 1024px) {
  header {
    display: flex;
    place-items: center;
    padding-right: calc(var(--section-gap) / 2);
  }

  .logo {
    margin: 0 2rem 0 0;
  }

  header .wrapper {
    display: flex;
    place-items: flex-start;
    flex-wrap: wrap;
  }
}
</style>
