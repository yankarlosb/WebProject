/**
 * AppButton - Componente de botón reutilizable
 * Botones con variantes y estados consistentes
 */
<template>
  <button
    :type="type"
    :disabled="disabled || loading"
    class="inline-flex items-center justify-center gap-2 font-semibold rounded-lg transition-all duration-200 focus:outline-none focus:ring-2 focus:ring-offset-2 disabled:opacity-50 disabled:cursor-not-allowed"
    :class="[sizeClass, variantClass, fullWidthClass]"
    @click="handleClick"
  >
    <!-- Loading spinner -->
    <svg
      v-if="loading"
      class="animate-spin h-5 w-5"
      xmlns="http://www.w3.org/2000/svg"
      fill="none"
      viewBox="0 0 24 24"
    >
      <circle
        class="opacity-25"
        cx="12"
        cy="12"
        r="10"
        stroke="currentColor"
        stroke-width="4"
      ></circle>
      <path
        class="opacity-75"
        fill="currentColor"
        d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
      ></path>
    </svg>

    <!-- Icon izquierdo -->
    <span v-if="$slots.icon && !loading" class="flex-shrink-0">
      <slot name="icon"></slot>
    </span>

    <!-- Contenido -->
    <span v-if="!loading || showTextWhileLoading">
      <slot></slot>
    </span>

    <!-- Icon derecho -->
    <span v-if="$slots.iconRight && !loading" class="flex-shrink-0">
      <slot name="iconRight"></slot>
    </span>
  </button>
</template>

<script setup lang="ts">
import { computed } from 'vue'

type ButtonVariant = 'primary' | 'secondary' | 'success' | 'danger' | 'warning' | 'ghost' | 'outline'
type ButtonSize = 'xs' | 'sm' | 'md' | 'lg' | 'xl'

interface Props {
  variant?: ButtonVariant
  size?: ButtonSize
  type?: 'button' | 'submit' | 'reset'
  disabled?: boolean
  loading?: boolean
  fullWidth?: boolean
  showTextWhileLoading?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  variant: 'primary',
  size: 'md',
  type: 'button',
  disabled: false,
  loading: false,
  fullWidth: false,
  showTextWhileLoading: false,
})

const emit = defineEmits<{
  click: [event: MouseEvent]
}>()

const sizeClass = computed(() => {
  const sizeMap = {
    xs: 'px-2.5 py-1.5 text-xs',
    sm: 'px-3 py-2 text-sm',
    md: 'px-4 py-2.5 text-base',
    lg: 'px-6 py-3 text-lg',
    xl: 'px-8 py-4 text-xl',
  }
  return sizeMap[props.size]
})

const variantClass = computed(() => {
  const variants = {
    primary: 'bg-blue-600 hover:bg-blue-700 text-white shadow-md hover:shadow-lg focus:ring-blue-500',
    secondary: 'bg-gray-600 hover:bg-gray-700 text-white shadow-md hover:shadow-lg focus:ring-gray-500',
    success: 'bg-green-600 hover:bg-green-700 text-white shadow-md hover:shadow-lg focus:ring-green-500',
    danger: 'bg-red-600 hover:bg-red-700 text-white shadow-md hover:shadow-lg focus:ring-red-500',
    warning: 'bg-yellow-500 hover:bg-yellow-600 text-white shadow-md hover:shadow-lg focus:ring-yellow-500',
    ghost: 'bg-transparent hover:bg-gray-100 text-gray-700 focus:ring-gray-400',
    outline: 'border-2 border-blue-600 text-blue-600 hover:bg-blue-50 focus:ring-blue-500',
  }
  return variants[props.variant]
})

const fullWidthClass = computed(() => props.fullWidth ? 'w-full' : '')

function handleClick(event: MouseEvent) {
  if (!props.disabled && !props.loading) {
    emit('click', event)
  }
}
</script>
