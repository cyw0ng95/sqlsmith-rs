<template>
  <div class="profile-form">
    <h2>Profile Settings</h2>
    <el-form :model="profile" :rules="rules" ref="profileForm" @submit.native.prevent>
      <el-form-item label="Driver" prop="driver">
        <el-select v-model="profile.driver">
          <el-option label="SQLITE_IN_MEM" value="SQLITE_IN_MEM"></el-option>
          <el-option label="LIMBO_IN_MEM" value="LIMBO_IN_MEM"></el-option>
        </el-select>
      </el-form-item>
      <el-form-item label="Run Count" prop="count">
        <el-input-number v-model="profile.count" :min="1"></el-input-number>
      </el-form-item>
      <el-form-item label="Executor Count" prop="executor_count">
        <el-input-number v-model="profile.executor_count" :min="1"></el-input-number>
      </el-form-item>
      <el-form-item label="SELECT Probability" prop="stmt_prob.SELECT">
        <el-input-number v-model="profile.stmt_prob.SELECT" :min="0"></el-input-number>
      </el-form-item>
      <el-form-item label="INSERT Probability" prop="stmt_prob.INSERT">
        <el-input-number v-model="profile.stmt_prob.INSERT" :min="0"></el-input-number>
      </el-form-item>
      <el-form-item label="UPDATE Probability" prop="stmt_prob.UPDATE">
        <el-input-number v-model="profile.stmt_prob.UPDATE" :min="0"></el-input-number>
      </el-form-item>
      <el-form-item label="DELETE Probability" prop="stmt_prob.DELETE">
        <el-input-number v-model="profile.stmt_prob.DELETE" :min="0"></el-input-number>
      </el-form-item>
      <el-form-item label="VACUUM Probability" prop="stmt_prob.VACUUM">
        <el-input-number v-model="profile.stmt_prob.VACUUM" :min="0"></el-input-number>
      </el-form-item>
      <el-form-item label="PRAGMA Probability" prop="stmt_prob.PRAGMA">
        <el-input-number v-model="profile.stmt_prob.PRAGMA" :min="0"></el-input-number>
      </el-form-item>
      <!-- 新增 CreateTrigger 表单项 -->
      <el-form-item label="CREATE_TRIGGER Probability" prop="stmt_prob.CREATE_TRIGGER">
        <el-input-number v-model="profile.stmt_prob.CREATE_TRIGGER" :min="0"></el-input-number>
      </el-form-item>
      <el-form-item label="Show Success SQL" prop="debug.show_success_sql">
        <el-switch v-model="profile.debug.show_success_sql"></el-switch>
      </el-form-item>
      <el-form-item label="Show Failed SQL" prop="debug.show_failed_sql">
        <el-switch v-model="profile.debug.show_failed_sql"></el-switch>
      </el-form-item>
      <el-form-item label="Seed" prop="seed">
        <el-input-number v-model="profile.seed" :min="0"></el-input-number>
      </el-form-item>
      <el-form-item>
        <el-button type="primary" @click="updateProfile">Update Profile</el-button>
        <!-- 新增运行按钮 -->
        <el-button type="success" @click="runAction">Run</el-button>
      </el-form-item>
    </el-form>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue';
import { fetchProfile, updateProfileCall, runRequest } from './request.js'; // 引入新函数

const profile = ref({
  driver: 'SQLITE_IN_MEM',
  count: 8,
  executor_count: 5,
  stmt_prob: {
    SELECT: 100,
    INSERT: 50,
    UPDATE: 50,
    DELETE: 20,
    VACUUM: 20,
    PRAGMA: 10,
    // 新增 CreateTrigger 默认值
    CREATE_TRIGGER: 10
  },
  debug: {
    show_success_sql: false,
    show_failed_sql: true
  },
  seed: 0  // Added seed field with default value 0
});

const profileForm = ref(null);

const rules = {
  driver: [
    { required: true, message: 'Please select a driver', trigger: 'change' }
  ],
  count: [
    { required: true, message: 'Please enter the run count', trigger: 'blur' },
    { type: 'number', min: 1, message: 'Run count must be at least 1', trigger: 'blur' }
  ],
  executor_count: [
    { required: true, message: 'Please enter the executor count', trigger: 'blur' },
    { type: 'number', min: 1, message: 'Executor count must be at least 1', trigger: 'blur' }
  ],
  seed: [
    { required: true, message: 'Please enter a seed value', trigger: 'blur' },
    { type: 'number', min: 0, message: 'Seed must be a positive number', trigger: 'blur' }
  ]
};

onMounted(async () => {
  try {
    const data = await fetchProfile();
    profile.value = data;
  } catch (error) {
    console.error('Failed to fetch profile:', error);
  }
});

const updateProfile = async () => {
  try {
    await profileForm.value.validate();
    const updateJson = JSON.stringify(profile.value);
    await updateProfileCall(updateJson);
    console.log('Profile updated successfully');
  } catch (error) {
    console.error('Failed to update profile:', error);
  }
};

const runAction = async () => {
  try {
    await runRequest();
    console.log('Run request executed successfully');
  } catch (error) {
    console.error('Failed to execute run request:', error);
  }
};
</script>

<style scoped>
.profile-form {
  max-width: 600px;
  margin: 0 auto;
}
</style>