<template>
    <div class="lg:flex lg:items-center lg:justify-between">
        <div class="min-w-0 flex-1 border border-gray-300 rounded-lg p-4">
            <h2 class="text-2xl/7 font-bold text-gray-900 sm:truncate sm:text-3xl sm:tracking-tight">
                {{ name }}
            </h2>
            <!-- 该页面计划简单展示一下视频文件的信息目前想到的有视频路径大小，格式，《如果上次播放过就显示上次播放的进度》 -->
            <div class="mt-1 flex flex-col sm:mt-0 sm:flex-row sm:flex-wrap sm:space-x-6">
                <div class="mt-2 flex items-center text-sm text-gray-500 w-full pl-6">
                    <!-- 文件路径 -->
                    <FilmIcon class="mr-1.5 size-5 shrink-0 text-gray-400" aria-hidden="true" />
                    {{ path }}
                </div>
                <div class="mt-2 flex items-center text-sm text-gray-500">
                    <!-- 文件格式 -->
                    <DocumentIcon class="mr-1.5 size-5 shrink-0 text-gray-400" aria-hidden="true" />
                    {{ type }}
                </div>
                <div class="mt-2 flex items-center text-sm text-gray-500">
                    <!-- 文件大小 -->
                    <ServerIcon class="mr-1.5 size-5 shrink-0 text-gray-400" aria-hidden="true" />
                    {{ size }}
                </div>
                <div v-if="time" class="mt-2 flex items-center text-sm text-gray-500">
                    <CalendarIcon class="mr-1.5 size-5 shrink-0 text-gray-400" aria-hidden="true" />
                    {{ time }}
                </div>
            </div>
        </div>
    </div>
</template>

<script setup>
import { FilmIcon, DocumentIcon, ServerIcon, CalendarIcon } from '@heroicons/vue/20/solid'

defineProps({
    name: String,
    path: String,
    type: String,
    size: String,
    time: String
});
</script>