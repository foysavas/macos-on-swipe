/// Represents a swipe direction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SwipeDirection {
    Left,
    Right,
    Up,
    Down,
}

impl SwipeDirection {
    /// Convert direction to string argument for script execution
    pub fn as_arg(&self) -> &'static str {
        match self {
            SwipeDirection::Left => "left",
            SwipeDirection::Right => "right",
            SwipeDirection::Up => "up",
            SwipeDirection::Down => "down",
        }
    }
    
    /// Determine swipe direction from delta values
    /// 
    /// NSEvent swipe deltas:
    /// - deltaX > 0 means swipe right (content moves left)
    /// - deltaX < 0 means swipe left (content moves right)
    /// - deltaY > 0 means swipe down (content moves up)
    /// - deltaY < 0 means swipe up (content moves down)
    pub fn from_deltas(delta_x: f64, delta_y: f64) -> Option<Self> {
        // Need some movement to register as a swipe
        if delta_x.abs() < 0.001 && delta_y.abs() < 0.001 {
            return None;
        }
        
        // Determine primary direction based on larger delta
        if delta_x.abs() >= delta_y.abs() {
            // Horizontal swipe
            if delta_x > 0.0 {
                Some(SwipeDirection::Right)
            } else {
                Some(SwipeDirection::Left)
            }
        } else {
            // Vertical swipe
            if delta_y > 0.0 {
                Some(SwipeDirection::Down)
            } else {
                Some(SwipeDirection::Up)
            }
        }
    }
}

impl std::fmt::Display for SwipeDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_arg())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_direction_from_deltas() {
        assert_eq!(SwipeDirection::from_deltas(1.0, 0.0), Some(SwipeDirection::Right));
        assert_eq!(SwipeDirection::from_deltas(-1.0, 0.0), Some(SwipeDirection::Left));
        assert_eq!(SwipeDirection::from_deltas(0.0, 1.0), Some(SwipeDirection::Down));
        assert_eq!(SwipeDirection::from_deltas(0.0, -1.0), Some(SwipeDirection::Up));
        assert_eq!(SwipeDirection::from_deltas(0.0, 0.0), None);
    }
    
    #[test]
    fn test_diagonal_swipe_picks_dominant() {
        // More horizontal than vertical -> horizontal
        assert_eq!(SwipeDirection::from_deltas(2.0, 1.0), Some(SwipeDirection::Right));
        assert_eq!(SwipeDirection::from_deltas(-2.0, 1.0), Some(SwipeDirection::Left));
        
        // More vertical than horizontal -> vertical
        assert_eq!(SwipeDirection::from_deltas(1.0, 2.0), Some(SwipeDirection::Down));
        assert_eq!(SwipeDirection::from_deltas(1.0, -2.0), Some(SwipeDirection::Up));
    }
    
    #[test]
    fn test_as_arg() {
        assert_eq!(SwipeDirection::Left.as_arg(), "left");
        assert_eq!(SwipeDirection::Right.as_arg(), "right");
        assert_eq!(SwipeDirection::Up.as_arg(), "up");
        assert_eq!(SwipeDirection::Down.as_arg(), "down");
    }
}

