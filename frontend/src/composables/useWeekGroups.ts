/**
 * Composable for week groups computation in balance views
 * Shared between BalanceForm.vue and BalanceView.vue
 */

import { computed, type Ref } from 'vue'
import { generateWeekGroups, type WeekGroup } from '../utils/balance-table'

/**
 * Creates a computed property for week groups based on total weeks
 * 
 * @param totalWeeks - Reactive reference or getter for total weeks count
 * @returns Computed property containing array of week groups
 */
export function useWeekGroups(totalWeeks: Ref<number | undefined> | (() => number | undefined)) {
  return computed<WeekGroup[]>(() => {
    const weeks = typeof totalWeeks === 'function' ? totalWeeks() : totalWeeks.value
    return generateWeekGroups(weeks || 15)
  })
}
