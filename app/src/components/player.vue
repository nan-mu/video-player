<template>
    <div>
        <video ref="videoPlayer" id="videoPlayer" class="video-js vjs-default-skin" controls preload="auto" width="640"
            height="264"></video>
    </div>
</template>

<script setup>
import { onMounted, onBeforeUnmount, ref } from 'vue'
import videojs from 'video.js'
import 'video.js/dist/video-js.css'

const props = defineProps({
    src: {
        type: String,
        required: true
    }
})

const videoPlayer = ref(null)
let player = null

onMounted(() => {
    player = videojs(videoPlayer.value, {
        sources: [{
            src: props.src,
            type: 'video/mp4'
        }]
    })

    player.on('loadedmetadata', () => {
        if (props.startTime > 0) {
            player.currentTime(props.startTime)
        }
    })

    player.on('timeupdate', () => {
        const currentTime = player.currentTime()
        console.log('Current Time:', currentTime)
        // 你可以在这里处理每秒获取视频观看进度的逻辑
    })
})

onBeforeUnmount(() => {
    if (player) {
        player.dispose()
    }
})
</script>

<style scoped>
.video-js {
    width: 100%;
    height: auto;
}
</style>