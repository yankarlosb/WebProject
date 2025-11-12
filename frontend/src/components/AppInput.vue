/**
 * AppInput - Componente de input reutilizable
 * Inputs con validación y estilos consistentes
 */
<template>
  <div class="w-full">
    <!-- Label -->
    <label 
      v-if="label" 
      :for="inputId"
      class="block text-sm font-medium text-gray-700 mb-2"
      :class="{ 'text-red-600': error }"
    >
      {{ label }}
      <span v-if="required" class="text-red-500 ml-1">*</span>
    </label>

    <!-- Input wrapper para iconos -->
    <div class="relative">
      <!-- Icon izquierdo -->
      <div 
        v-if="$slots.iconLeft" 
        class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none text-gray-400"
      >
        <slot name="iconLeft"></slot>
      </div>

      <!-- Input principal -->
      <input
        v-if="type !== 'textarea'"
        :id="inputId"
        :type="type"
        :value="modelValue"
        :placeholder="placeholder"
        :disabled="disabled"
        :required="required"
        :min="min"
        :max="max"
        :step="step"
        class="block w-full rounded-lg border transition-colors duration-200 focus:outline-none focus:ring-2 focus:ring-offset-1"
        :class="[
          inputSizeClass,
          inputStateClass,
          $slots.iconLeft ? 'pl-10' : '',
          $slots.iconRight ? 'pr-10' : '',
        ]"
        @input="handleInput"
        @blur="$emit('blur', $event)"
        @focus="$emit('focus', $event)"
      />

      <!-- Textarea -->
      <textarea
        v-else
        :id="inputId"
        :value="modelValue"
        :placeholder="placeholder"
        :disabled="disabled"
        :required="required"
        :rows="rows"
        class="block w-full rounded-lg border transition-colors duration-200 focus:outline-none focus:ring-2 focus:ring-offset-1 resize-none"
        :class="[inputSizeClass, inputStateClass]"
        @input="handleInput"
        @blur="$emit('blur', $event)"
        @focus="$emit('focus', $event)"
      ></textarea>

      <!-- Icon derecho -->
      <div 
        v-if="$slots.iconRight" 
        class="absolute inset-y-0 right-0 pr-3 flex items-center pointer-events-none text-gray-400"
      >
        <slot name="iconRight"></slot>
      </div>
    </div>

    <!-- Mensaje de ayuda o error -->
    <p 
      v-if="error || hint"
      class="mt-2 text-sm"
      :class="error ? 'text-red-600' : 'text-gray-500'"
    >
      {{ error || hint }}
    </p>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

type InputType = 'text' | 'email' | 'password' | 'number' | 'date' | 'time' | 'tel' | 'url' | 'textarea'
type InputSize = 'sm' | 'md' | 'lg'

interface Props {
  modelValue?: string | number | null
  type?: InputType
  label?: string
  placeholder?: string
  hint?: string
  error?: string
  disabled?: boolean
  required?: boolean
  size?: InputSize
  min?: string | number
  max?: string | number
  step?: string | number
  rows?: number
}

const props = withDefaults(defineProps<Props>(), {
  type: 'text',
  size: 'md',
  rows: 3,
})

const emit = defineEmits<{
  'update:modelValue': [value: string | number]
  blur: [event: FocusEvent]
  focus: [event: FocusEvent]
}>()

// ID único para accesibilidad
const inputId = computed(() => `input-${Math.random().toString(36).substr(2, 9)}`)

const inputSizeClass = computed(() => {
  const sizeMap = {
    sm: 'px-3 py-2 text-sm',
    md: 'px-4 py-2.5 text-base',
    lg: 'px-5 py-3 text-lg',
  }
  return sizeMap[props.size]
})

const inputStateClass = computed(() => {
  if (props.error) {
    return 'border-red-300 focus:border-red-500 focus:ring-red-500 bg-red-50'
  }
  
  if (props.disabled) {
    return 'border-gray-200 bg-gray-100 cursor-not-allowed text-gray-500'
  }
  
  return 'border-gray-300 focus:border-blue-500 focus:ring-blue-500 bg-white'
})

function handleInput(event: Event) {
  const target = event.target as HTMLInputElement | HTMLTextAreaElement
  const value = props.type === 'number' ? Number(target.value) : target.value
  emit('update:modelValue', value)
}
</script>
