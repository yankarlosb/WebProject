use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::Mutex;
use std::time::{Duration, Instant};

/// Configuración del rate limiter
const MAX_ATTEMPTS: u32 = 5; // Máximo de intentos fallidos
const BLOCK_DURATION_SECS: u64 = 300; // 5 minutos de bloqueo
const ATTEMPT_WINDOW_SECS: u64 = 60; // Ventana de tiempo para contar intentos

/// Estructura para rastrear intentos de login por IP
#[derive(Debug, Clone)]
pub struct LoginAttempt {
    pub attempts: u32,
    pub first_attempt: Instant,
    pub blocked_until: Option<Instant>,
}

impl Default for LoginAttempt {
    fn default() -> Self {
        Self {
            attempts: 0,
            first_attempt: Instant::now(),
            blocked_until: None,
        }
    }
}

/// Rate limiter para intentos de login
pub struct RateLimiter {
    attempts: Mutex<HashMap<IpAddr, LoginAttempt>>,
}

impl Default for RateLimiter {
    fn default() -> Self {
        Self::new()
    }
}

impl RateLimiter {
    pub fn new() -> Self {
        Self {
            attempts: Mutex::new(HashMap::new()),
        }
    }

    /// Verifica si una IP está bloqueada
    pub fn is_blocked(&self, ip: IpAddr) -> bool {
        let attempts = self.attempts.lock().unwrap();
        
        if let Some(attempt) = attempts.get(&ip) {
            if let Some(blocked_until) = attempt.blocked_until {
                if Instant::now() < blocked_until {
                    return true;
                }
            }
        }
        
        false
    }

    /// Obtiene el tiempo restante de bloqueo en segundos
    pub fn get_block_remaining(&self, ip: IpAddr) -> Option<u64> {
        let attempts = self.attempts.lock().unwrap();
        
        if let Some(attempt) = attempts.get(&ip) {
            if let Some(blocked_until) = attempt.blocked_until {
                let now = Instant::now();
                if now < blocked_until {
                    return Some((blocked_until - now).as_secs());
                }
            }
        }
        
        None
    }

    /// Checks if an IP is blocked and returns remaining time in a single lock acquisition
    /// More efficient than calling is_blocked + get_block_remaining separately
    /// Returns (is_blocked, optional_remaining_seconds)
    pub fn check_block_status(&self, ip: IpAddr) -> (bool, Option<u64>) {
        let attempts = self.attempts.lock().unwrap();
        
        if let Some(attempt) = attempts.get(&ip) {
            if let Some(blocked_until) = attempt.blocked_until {
                let now = Instant::now();
                if now < blocked_until {
                    return (true, Some((blocked_until - now).as_secs()));
                }
            }
        }
        
        (false, None)
    }

    /// Registra un intento fallido de login
    pub fn record_failed_attempt(&self, ip: IpAddr) -> bool {
        let mut attempts = self.attempts.lock().unwrap();
        
        let attempt = attempts.entry(ip).or_default();
        let now = Instant::now();
        
        // Si ya está bloqueado, verificar si el bloqueo expiró
        if let Some(blocked_until) = attempt.blocked_until {
            if now >= blocked_until {
                // Bloqueo expirado, reiniciar contadores
                attempt.attempts = 0;
                attempt.blocked_until = None;
            } else {
                // Aún bloqueado
                return true;
            }
        }
        
        // Verificar si la ventana de tiempo expiró
        if now.duration_since(attempt.first_attempt) > Duration::from_secs(ATTEMPT_WINDOW_SECS) {
            // Reiniciar contadores
            attempt.attempts = 1;
            attempt.first_attempt = now;
        } else {
            // Incrementar intentos
            attempt.attempts += 1;
        }
        
        // Verificar si se debe bloquear
        if attempt.attempts >= MAX_ATTEMPTS {
            attempt.blocked_until = Some(now + Duration::from_secs(BLOCK_DURATION_SECS));
            return true;
        }
        
        false
    }

    /// Registra un login exitoso (limpia los intentos)
    pub fn record_success(&self, ip: IpAddr) {
        let mut attempts = self.attempts.lock().unwrap();
        attempts.remove(&ip);
    }

    /// Limpia entradas antiguas del mapa (para liberar memoria)
    pub fn cleanup_old_entries(&self) {
        let mut attempts = self.attempts.lock().unwrap();
        let now = Instant::now();
        let cleanup_threshold = Duration::from_secs(BLOCK_DURATION_SECS * 2);
        
        attempts.retain(|_, attempt| {
            // Mantener solo entradas recientes o aún bloqueadas
            if let Some(blocked_until) = attempt.blocked_until {
                now < blocked_until
            } else {
                now.duration_since(attempt.first_attempt) < cleanup_threshold
            }
        });
    }

    /// Obtiene el número de intentos restantes antes del bloqueo
    pub fn get_remaining_attempts(&self, ip: IpAddr) -> u32 {
        let attempts = self.attempts.lock().unwrap();
        
        if let Some(attempt) = attempts.get(&ip) {
            if attempt.blocked_until.is_some() {
                return 0;
            }
            return MAX_ATTEMPTS.saturating_sub(attempt.attempts);
        }
        
        MAX_ATTEMPTS
    }

    /// Checks if cleanup is needed and performs it if the map has grown too large.
    /// Returns true if cleanup was performed.
    /// This is more efficient than calling cleanup_old_entries() on every request.
    pub fn maybe_cleanup(&self) -> bool {
        // Only cleanup if the map has grown beyond a threshold
        const CLEANUP_THRESHOLD: usize = 1000;
        
        let should_cleanup = {
            let attempts = self.attempts.lock().unwrap();
            attempts.len() > CLEANUP_THRESHOLD
        };
        
        if should_cleanup {
            self.cleanup_old_entries();
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_rate_limiter() {
        let limiter = RateLimiter::new();
        let ip = IpAddr::from_str("192.168.1.1").unwrap();
        
        // No debería estar bloqueado inicialmente
        assert!(!limiter.is_blocked(ip));
        
        // Registrar intentos fallidos
        for _ in 0..4 {
            assert!(!limiter.record_failed_attempt(ip));
        }
        
        // El 5to intento debería bloquear
        assert!(limiter.record_failed_attempt(ip));
        assert!(limiter.is_blocked(ip));
    }
}
