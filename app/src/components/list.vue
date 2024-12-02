<template>
    <header class="bg-white shadow">
        <div class="mx-auto max-w-7xl px-4 py-6 sm:px-6 lg:px-8">
            <div class="flex justify-between items-center">
                <h1 class="text-3xl font-bold tracking-tight text-gray-900">最近打开的项目</h1>
                <button @click="handleButtonClick"
                    class="bg-blue-500 text-white p-2 rounded-full flex items-center justify-center">
                    <PlusIcon class="size-4" aria-hidden="true" />
                </button>
            </div>
        </div>
    </header>
    <main>
        <div class="mx-auto max-w-7xl px-4 py-6 sm:px-6 lg:px-8">
            <!-- Your content -->
            <!-- :key="index"
                :title="item.title"
                :thumbnail="item.thumbnail"
                :url="item.url" -->
            <videoItem v-for="(item, index) in videoItems" :key="index" :name="item.name" :path="item.path"
                :type="item.type" :size="item.size" :time="item.time" class="mb-5" />
        </div>
    </main>
</template>

<script setup>
import { PlusIcon } from '@heroicons/vue/20/solid'
import videoItem from './videoItem.vue';
import { defineEmits } from 'vue'

const emits = defineEmits(['play-video'])

const videoItems = [
    { name: "Video 1", path: "/videos/video1.mp4", type: "mp4", size: "20MB", time: "10:00" },
    { name: "Video 2", path: "/videos/video2.mp4", type: "mp4", size: "30MB", time: "15:00" }
]

import { open } from '@tauri-apps/plugin-dialog';
// Function to open file dialog
const openFileDialog = async () => {
    const file = await open({
        multiple: false,
        directory: false,
        defaultPath: '~/Videos',
        filters: [
            {
                name: 'Videos',
                extensions: ['mp4', 'avi', 'mkv', 'mov', 'wmv']
            }
        ]
    });
    return file;
    // console.log(file);
};

// Add the function to the button's click event
const handleButtonClick = () => {
    if (window.__TAURI__) {
        openFileDialog().await().then((result) => {
            emits('play-video', result);
        });
    } else {
        const input = document.createElement('input');
        input.type = 'file';
        input.accept = 'video/*';
        input.multiple = false;
        input.click();
        input.onchange = (event) => {
            const file = event.target.files[0];
            let path = URL.createObjectURL(file);
            let type = file.type;
            console.log("path: " + path);
            console.log("type: " + type);
            emits('play-video', { type, path });
        };
    };
};

</script>