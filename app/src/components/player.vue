<template>
    <video-player height="680" ref="Player" class="video-player vjs-theme-city" :options="videoOptions"
        style="width: 100%; "></video-player>
</template>

<script setup>
import { VideoPlayer } from '@videojs-player/vue'
import 'video.js/dist/video-js.css'
import { onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core';

const props = defineProps({
    url: {
        type: String,
        required: true
    },
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
            src: props.url,
            type: props.type
        }
    ]
}

onMounted(async () => {
    try {
        await invoke('update_record', {
            hash: '',
            path: props.path,
            recode: 0.0
        });
    } catch (error) {
        console.error('Error:', error);
    }

    // const player = playerRef.value?.player;

    // if (player) {
    //     setInterval(() => {
    //         console.log(`Current Time: ${player.currentTime()}`);
    //     }, 1000);
    // }
});

</script>