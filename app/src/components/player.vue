<template>
    <div>
        <!-- <video ref="videoPlayer" id="videoPlayer" class="video-js vjs-default-skin" controls preload="auto" width="640"
            height="264">
        </video> -->
        <!-- :volume="0.6" @mounted="..." @ready="..." @play="..."
            @pause="..." @ended="..." @seeking="..." -->
        <video-player ref="Player" class="video-player vjs-custom-skin" :options="videoOptions"></video-player>
    </div>
</template>

<script setup>
import { VideoPlayer } from '@videojs-player/vue'
import videojs from 'video.js'
import 'video.js/dist/video-js.css'
import { ref, onMounted, onBeforeUnmount } from 'vue'

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

const Player = ref(null)
let player = null

function onPlayerPlay() {
    console.log('Video is playing')
}

function onPlayerPause() {
    console.log('Video is paused')
}

function onPlayerEnded() {
    console.log('Video has ended')
}

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

onMounted(() => {
    // console.log('Video path: ' + props.path);
    // console.log('Video type: ' + props.type);
    // player = videojs(Player.value, {
    //     autoplay: false,
    //     controls: true,
    //     sources: [
    //         {
    //             src: props.path,
    //             type: props.type
    //         }
    //     ]
    // })
    // player.on('play', onPlayerPlay)
    // player.on('pause', onPlayerPause)
    // player.on('ended', onPlayerEnded)
})

onBeforeUnmount(() => {
    if (player) {
        player.dispose()
    }
})

</script>

<style scoped>
.video-player {
    width: 100%;
    height: auto;
}
</style>