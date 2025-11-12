/**
 * AppTabs - Componente de tabs reutilizable
 * Sistema de pestañas para organizar contenido
 */
<template>
  <div>
    <!-- Tab Headers -->
    <div class="border-b border-gray-200 bg-white rounded-t-xl overflow-hidden">
      <nav class="flex space-x-1 p-2" aria-label="Tabs">
        <button
          v-for="tab in tabs"
          :key="tab.id"
          @click="selectTab(tab.id)"
          class="px-4 py-2.5 text-sm font-medium rounded-lg transition-all duration-200"
          :class="activeTab === tab.id
            ? 'bg-blue-600 text-white shadow-md'
            : 'text-gray-600 hover:bg-gray-100'
          "
        >
          <span class="flex items-center gap-2">
            <component v-if="tab.icon" :is="tab.icon" class="w-4 h-4" />
            {{ tab.label }}
          </span>
        </button>
      </nav>
    </div>

    <!-- Tab Content -->
    <div class="bg-white rounded-b-xl">
      <slot :activeTab="activeTab"></slot>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'

interface Tab {
  id: string
  label: string
  icon?: any
}

interface Props {
  tabs: Tab[]
  modelValue?: string
}

const props = defineProps<Props>()

const emit = defineEmits<{
  'update:modelValue': [value: string]
  'tab-change': [tabId: string]
}>()

const activeTab = ref(props.modelValue || props.tabs[0]?.id || '')

// Sincronizar con modelValue
watch(() => props.modelValue, (newValue) => {
  if (newValue && newValue !== activeTab.value) {
    activeTab.value = newValue
  }
}, { immediate: true })

function selectTab(tabId: string) {
  activeTab.value = tabId
  emit('update:modelValue', tabId)
  emit('tab-change', tabId)
}
</script>
