<template>
    <div class="video-player">
      <h1>{{ video.title }}</h1>
      <video :src="video.src" controls></video>
      <router-link to="/">返回列表</router-link>
    </div>
  </template>
  
  <script setup>
  import { ref, onMounted } from 'vue'
  import { useRoute } from 'vue-router'
  
  const route = useRoute()
  const video = ref({})
  
  const videos = [
    { id: 1, title: '视频 1', src: '/videos/video1.mp4' },
    { id: 2, title: '视频 2', src: '/videos/video2.mp4' },
    { id: 3, title: '视频 3', src: '/videos/video3.mp4' },
  ]
  
  onMounted(() => {
    const videoId = parseInt(route.params.id)
    video.value = videos.find(v => v.id === videoId) || {}
  })
  </script>
  
  <style scoped>
  .video-player {
    max-width: 800px;
    margin: 0 auto;
  }
  
  video {
    width: 100%;
    margin-bottom: 20px;
  }
  
  a {
    color: #2c3e50;
    text-decoration: none;
  }
  
  a:hover {
    text-decoration: underline;
  }
  </style>