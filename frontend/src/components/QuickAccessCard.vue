/**
 * QuickAccessCard - Tarjeta de acceso rápido
 * Botones grandes para navegación principal
 */
<template>
  <button
    class="bg-white rounded-xl shadow-lg border border-blue-100 p-6 text-left transition-all duration-200 hover:shadow-xl hover:-translate-y-1 hover:border-blue-300 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
    @click="$emit('click')"
  >
    <div class="flex items-start gap-4">
      <!-- Icon -->
      <div
        class="flex-shrink-0 w-12 h-12 rounded-lg flex items-center justify-center"
        :class="iconColorClass"
      >
        <component :is="iconComponent" class="w-6 h-6" />
      </div>

      <!-- Content -->
      <div class="flex-1 min-w-0">
        <h3 class="text-lg font-bold text-gray-900 mb-1">
          {{ title }}
        </h3>
        <p class="text-sm text-gray-600">
          {{ description }}
        </p>
      </div>

      <!-- Arrow -->
      <div class="flex-shrink-0">
        <svg
          class="w-5 h-5 text-gray-400 transform transition-transform group-hover:translate-x-1"
          fill="none"
          viewBox="0 0 24 24"
          stroke="currentColor"
        >
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
        </svg>
      </div>
    </div>
  </button>
</template>

<script setup lang="ts">
import { computed, h } from 'vue'

interface Props {
  title: string
  description: string
  icon: 'table' | 'book' | 'user' | 'settings' | 'document'
  color: 'blue' | 'green' | 'purple' | 'gray'
}

const props = defineProps<Props>()

defineEmits<{
  click: []
}>()

const iconColorClass = computed(() => {
  const colors = {
    blue: 'bg-blue-100 text-blue-600',
    green: 'bg-green-100 text-green-600',
    purple: 'bg-purple-100 text-purple-600',
    gray: 'bg-gray-100 text-gray-600',
  }
  return colors[props.color]
})

const iconComponent = computed(() => {
  const icons = {
    table: () => h('svg', {
      fill: 'none',
      viewBox: '0 0 24 24',
      stroke: 'currentColor',
    }, [
      h('path', {
        'stroke-linecap': 'round',
        'stroke-linejoin': 'round',
        'stroke-width': '2',
        d: 'M3 10h18M3 14h18m-9-4v8m-7 0h14a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v8a2 2 0 002 2z',
      }),
    ]),
    book: () => h('svg', {
      fill: 'none',
      viewBox: '0 0 24 24',
      stroke: 'currentColor',
    }, [
      h('path', {
        'stroke-linecap': 'round',
        'stroke-linejoin': 'round',
        'stroke-width': '2',
        d: 'M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253',
      }),
    ]),
    user: () => h('svg', {
      fill: 'none',
      viewBox: '0 0 24 24',
      stroke: 'currentColor',
    }, [
      h('path', {
        'stroke-linecap': 'round',
        'stroke-linejoin': 'round',
        'stroke-width': '2',
        d: 'M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z',
      }),
    ]),
    settings: () => h('svg', {
      fill: 'none',
      viewBox: '0 0 24 24',
      stroke: 'currentColor',
    }, [
      h('path', {
        'stroke-linecap': 'round',
        'stroke-linejoin': 'round',
        'stroke-width': '2',
        d: 'M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z',
      }),
      h('path', {
        'stroke-linecap': 'round',
        'stroke-linejoin': 'round',
        'stroke-width': '2',
        d: 'M15 12a3 3 0 11-6 0 3 3 0 016 0z',
      }),
    ]),
    document: () => h('svg', {
      fill: 'none',
      viewBox: '0 0 24 24',
      stroke: 'currentColor',
    }, [
      h('path', {
        'stroke-linecap': 'round',
        'stroke-linejoin': 'round',
        'stroke-width': '2',
        d: 'M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z',
      }),
    ]),
  }
  return icons[props.icon]
})
</script>
