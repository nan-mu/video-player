<template>
    <div class="video-player">
      <h1 class="text-2xl font-bold mb-4 text-text">{{ video.title }}</h1>
      <div class="bg-black rounded-lg overflow-hidden shadow-lg mb-4">
        <video :src="video.src" controls class="w-full"></video>
      </div>
      <router-link to="/" class="inline-block px-4 py-2 bg-primary text-text rounded hover:bg-secondary transition-colors duration-200">返回列表</router-link>
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
  
  