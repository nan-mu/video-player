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
                    </div>
                    <div class="-mr-2 flex md:hidden">
                    </div>
                </div>
            </div>
        </Disclosure>
        <router-view @play-video="handlePlayVideo" />
    </div>
</template>

<script setup>
import { Disclosure } from '@headlessui/vue'
import { useRouter } from 'vue-router'

const router = useRouter()

function handlePlayVideo({ path, type }) {
    // console.log(res);
    router.push({ path: '/player', query: { src: path, type: type } });
}
</script>