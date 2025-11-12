/**
 * AppCard - Componente de tarjeta reutilizable
 * Wrapper para contenido con estilos consistentes
 */
<template>
  <div 
    class="bg-white rounded-xl shadow-lg border border-blue-100 overflow-hidden"
    :class="[
      paddingClass,
      hoverClass,
      {
        'ring-2 ring-blue-400': focused
      }
    ]"
  >
    <!-- Header opcional -->
    <div 
      v-if="title || $slots.header" 
      class="border-b border-blue-100 px-6 py-4 bg-gradient-to-r from-blue-50 to-white"
    >
      <slot name="header">
        <div class="flex items-center justify-between">
          <h3 class="text-xl font-bold text-blue-700">{{ title }}</h3>
          <slot name="actions"></slot>
        </div>
      </slot>
    </div>

    <!-- Contenido principal -->
    <div :class="contentPadding">
      <slot></slot>
    </div>

    <!-- Footer opcional -->
    <div 
      v-if="$slots.footer" 
      class="border-t border-blue-100 px-6 py-4 bg-gray-50"
    >
      <slot name="footer"></slot>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

interface Props {
  title?: string
  padding?: 'none' | 'sm' | 'md' | 'lg'
  hover?: boolean
  focused?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  padding: 'md',
  hover: false,
  focused: false,
})

const paddingClass = computed(() => {
  const paddingMap = {
    none: '',
    sm: 'p-4',
    md: 'p-6',
    lg: 'p-8',
  }
  return paddingMap[props.padding]
})

const contentPadding = computed(() => {
  if (props.padding === 'none') return ''
  
  // Si tiene header o footer, usar padding específico
  return ''
})

const hoverClass = computed(() => 
  props.hover ? 'transition-all duration-200 hover:shadow-xl hover:-translate-y-0.5 cursor-pointer' : ''
)
</script>
