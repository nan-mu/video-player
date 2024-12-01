<template>
    <div class="min-h-full">
        <Disclosure as="nav" class="bg-gray-800" v-slot="{ open }">
            <div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
                <div class="flex h-16 items-center justify-between">
                    <div class="flex items-center">
                        <div class="shrink-0">
                            <img class="size-8"
                                src="https://tailwindui.com/plus/img/logos/mark.svg?color=indigo&shade=500"
                                alt="logo" />
                        </div>
                        <!-- <div class="hidden md:block">
                            <div class="ml-10 flex items-baseline space-x-4">
                                <a v-for="item in navigation" :key="item.name" :href="item.href"
                                    :class="[item.current ? 'bg-gray-900 text-white' : 'text-gray-300 hover:bg-gray-700 hover:text-white', 'rounded-md px-3 py-2 text-sm font-medium']"
                                    :aria-current="item.current ? 'page' : undefined">{{ item.name }}</a>
                            </div>
                        </div> -->
                    </div>
                    <div class="-mr-2 flex md:hidden">
                        <!-- Mobile menu button -->
                        <!-- <DisclosureButton
                            class="relative inline-flex items-center justify-center rounded-md bg-gray-800 p-2 text-gray-400 hover:bg-gray-700 hover:text-white focus:outline-none focus:ring-2 focus:ring-white focus:ring-offset-2 focus:ring-offset-gray-800">
                            <span class="absolute -inset-0.5" />
                            <span class="sr-only">Open main menu</span>
                            <Bars3Icon v-if="!open" class="block size-6" aria-hidden="true" />
                            <XMarkIcon v-else class="block size-6" aria-hidden="true" />
                        </DisclosureButton> -->
                    </div>
                </div>
            </div>
        </Disclosure>

        <header class="bg-white shadow">
            <div class="mx-auto max-w-7xl px-4 py-6 sm:px-6 lg:px-8">
                <h1 class="text-3xl font-bold tracking-tight text-gray-900">最近打开的项目</h1>
            </div>
        </header>
        <main>
            <div class="mx-auto max-w-7xl px-4 py-6 sm:px-6 lg:px-8">
                <!-- Your content -->
                <!-- :key="index"
                :title="item.title"
                :thumbnail="item.thumbnail"
                :url="item.url" -->
                <videoItem v-for="(item, index) in videoItems" 
                    :key="index" 
                    :video_name="item.name" 
                    :file_name="item.path"
                class="mb-5" />
            </div>
        </main>
    </div>
</template>

<script setup>
import { Disclosure } from '@headlessui/vue'
import videoItem from './components/videoItem.vue';

const videoItems = [
    { name: "test", path: "/test" },
    { name: "test", path: "/test" }
]
</script>