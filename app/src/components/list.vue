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
            <videoItem v-for="(item, index) in videoItems" :key="index" :name="item.name" :path="item.path"
                :type="item.type" :size="item.size" :time="item.time" class="mb-5" />
        </div>
    </main>
</template>

<script setup>
import { PlusIcon } from '@heroicons/vue/20/solid'
import { open } from '@tauri-apps/plugin-dialog';
import { onMounted, ref } from 'vue'
import { invoke } from '@tauri-apps/api/core';
import VideoItem from './VideoItem.vue'; // 确保正确导入 VideoItem 组件

const emits = defineEmits(['play-video'])
const videoItems = ref([]);

onMounted(async () => {
    try {
        let response = await invoke('get_list', {
            offset: 0,
        });
        for (const item of response) {
            videoItems.value.push({
                name: item.path.split('\\').pop(),
                path: item.path,
                type: item.type_,
                size: item.size,
                time: item.record
            });
        }
        console.log('Response:', videoItems.value[0]);
    } catch (error) {
        console.error('Error:', error);
    }
})

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

const handleButtonClick = async () => {
    try {
        let path = await openFileDialog();
        const input = document.createElement('input');
        input.type = 'file';
        input.accept = 'video/*';
        input.multiple = false;
        input.click();
        input.onchange = (event) => {
            const file = event.target.files[0];
            let url = URL.createObjectURL(file);
            let type = file.type;
            let res = { url, path, type };
            console.log(res);
            emits('play-video', res);
        };
    } catch (error) {
        console.error('Error selecting file:', error);
    }
};

</script>