/**
 * Shared HTTP request helper
 * Provides a standardized way to make API requests with consistent error handling
 */

import { getApiUrl } from '../config/api'

// Standard API response structure from the backend
export interface ApiResponse {
  message: string
  alert: 'success' | 'error'
}

// API response with additional data payload
export interface ApiResponseWithData<T> extends ApiResponse {
  data: T | null
}

// Generic service response for internal use
export interface ServiceResponse<T = void> {
  success: boolean
  message?: string
  data?: T
}

type HttpMethod = 'GET' | 'POST' | 'PUT' | 'DELETE'

interface RequestOptions<T> {
  /** The API endpoint path (e.g., '/api/users') */
  endpoint: string
  /** HTTP method */
  method?: HttpMethod
  /** Request body (will be JSON stringified) */
  body?: unknown
  /** Default error message if request fails */
  errorMessage?: string
  /** Transform the response data */
  transform?: (data: unknown) => T
}

/**
 * Makes an HTTP request to the API with standardized error handling
 * @param options Request configuration options
 * @returns Promise with the service response
 */
export async function httpRequest<T = void>(
  options: RequestOptions<T>
): Promise<ServiceResponse<T>> {
  const {
    endpoint,
    method = 'GET',
    body,
    errorMessage = 'Error en la operación',
    transform,
  } = options

  try {
    const url = getApiUrl(endpoint)
    const fetchOptions: RequestInit = {
      method,
      credentials: 'include',
      headers: {
        'Content-Type': 'application/json',
      },
    }

    if (body !== undefined) {
      fetchOptions.body = JSON.stringify(body)
    }

    const response = await fetch(url, fetchOptions)

    // Check content type to ensure we're getting JSON
    const contentType = response.headers.get('content-type')
    if (!contentType || !contentType.includes('application/json')) {
      console.error(`Expected JSON response but got: ${contentType}`)
      return {
        success: false,
        message: 'El servidor no está disponible o devolvió una respuesta inválida',
      }
    }

    if (!response.ok) {
      // Try to parse error message from response
      try {
        const errorData = await response.json()
        return {
          success: false,
          message: errorData.message || errorMessage,
        }
      } catch {
        return {
          success: false,
          message: errorMessage,
        }
      }
    }

    const data: ApiResponse | ApiResponseWithData<unknown> = await response.json()

    if (data.alert === 'error') {
      return {
        success: false,
        message: data.message || errorMessage,
      }
    }

    // Handle response with data
    if ('data' in data && data.data !== null) {
      const transformedData = transform ? transform(data.data) : (data.data as T)
      return {
        success: true,
        message: data.message,
        data: transformedData,
      }
    }

    // Handle response without data
    return {
      success: true,
      message: data.message,
    }
  } catch (error) {
    console.error(`Error in ${endpoint}:`, error)
    return {
      success: false,
      message: `Error de conexión: ${errorMessage}`,
    }
  }
}

/**
 * Shorthand for GET requests
 */
export async function httpGet<T>(
  endpoint: string,
  errorMessage?: string
): Promise<ServiceResponse<T>> {
  return httpRequest<T>({ endpoint, method: 'GET', errorMessage })
}

/**
 * Shorthand for POST requests
 */
export async function httpPost<T = void>(
  endpoint: string,
  body?: unknown,
  errorMessage?: string
): Promise<ServiceResponse<T>> {
  return httpRequest<T>({ endpoint, method: 'POST', body, errorMessage })
}

/**
 * Shorthand for PUT requests
 */
export async function httpPut<T = void>(
  endpoint: string,
  body?: unknown,
  errorMessage?: string
): Promise<ServiceResponse<T>> {
  return httpRequest<T>({ endpoint, method: 'PUT', body, errorMessage })
}

/**
 * Shorthand for DELETE requests
 */
export async function httpDelete<T = void>(
  endpoint: string,
  errorMessage?: string
): Promise<ServiceResponse<T>> {
  return httpRequest<T>({ endpoint, method: 'DELETE', errorMessage })
}
