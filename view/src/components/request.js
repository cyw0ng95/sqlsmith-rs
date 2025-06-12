import axios from 'axios';

// Fetch profile data
 export const fetchProfile = async () => {
  try {
    const response = await axios.get('http://127.0.0.1:8080/profile/get');
    return response.data;
  } catch (error) {
    console.error('Failed to fetch profile:', error);
    throw error;
  }
};

// Update profile data
 export const updateProfileCall = async (profileData) => {
  try {
    console.log(profileData)
    await axios.post('http://127.0.0.1:8080/profile/put', profileData, { headers: { 'Content-Type': 'application/json' } });
  } catch (error) {
    console.error('Failed to update profile:', error);
    throw error;
  }
};

// 新增运行请求函数
export const runRequest = async () => {
  try {
    const response = await axios.get('http://127.0.0.1:8080/run');
    return response.data;
  } catch (error) {
    console.error('Failed to execute run request:', error);
    throw error;
  }
};

// Fetch server statistics
export const fetchStats = async () => {
  try {
    const response = await axios.get('http://127.0.0.1:8080/internal/stat/collect');
    return response.data;
  } catch (error) {
    console.error('Failed to fetch stats:', error);
    throw error;
  }
};