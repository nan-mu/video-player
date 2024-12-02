<template>
    <video-player height="680" ref="Player" class="video-player vjs-theme-city" :options="videoOptions"
        style="width: 100%; "></video-player>
</template>

<script setup>
import { VideoPlayer } from '@videojs-player/vue'
import 'video.js/dist/video-js.css'

const props = defineProps({
    path: {
        type: String,
        required: true
    },
    type: {
        type: String,
        required: true
    }
})

const videoOptions = {
    autoplay: false,
    controls: true,
    sources: [
        {
            src: props.path,
            type: props.type
        }
    ]
}

</script>