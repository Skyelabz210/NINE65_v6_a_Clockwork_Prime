//! EPRAM Core Types
//!
//! Foundational types for the EPRAM system with Rust safety guarantees.
//! All types properly derive Copy where needed for pattern matching.

use crate::circular::{signed_geodesic, modular_add, fourth_attractor_step_dithered};

/// EPRAM Field Topology
///
/// Defines how cells are connected to their neighbors.
/// Must be Copy for ergonomic pattern matching in Rust.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Topology {
    /// 2D Grid with specified width (height inferred from cell count)
    Grid { width: usize },
    /// Ring topology: each cell connected to adjacent cells
    Ring,
    /// All cells connected to all other cells
    FullyConnected,
}

impl Topology {
    /// Get neighbors for a given cell index
    pub fn neighbors(&self, index: usize, total: usize) -> Vec<usize> {
        match *self {
            Topology::Grid { width } => {
                let height = (total + width - 1) / width;
                let row = index / width;
                let col = index % width;
                
                let mut neighbors = Vec::with_capacity(4);
                
                // Up
                if row > 0 {
                    neighbors.push((row - 1) * width + col);
                }
                // Down
                if row < height - 1 && (row + 1) * width + col < total {
                    neighbors.push((row + 1) * width + col);
                }
                // Left
                if col > 0 {
                    neighbors.push(row * width + col - 1);
                }
                // Right
                if col < width - 1 && row * width + col + 1 < total {
                    neighbors.push(row * width + col + 1);
                }
                
                neighbors
            }
            Topology::Ring => {
                if total <= 1 {
                    vec![]
                } else if total == 2 {
                    vec![(index + 1) % 2]
                } else {
                    vec![
                        (index + total - 1) % total,  // Previous
                        (index + 1) % total,          // Next
                    ]
                }
            }
            Topology::FullyConnected => {
                (0..total).filter(|&i| i != index).collect()
            }
        }
    }
}

/// Coupling mode for neighbor influence
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CouplingMode {
    /// No neighbor coupling (independent convergence)
    Independent,
    /// All cells pull toward global consensus
    Global,
    /// Per-cell weighted coupling
    Hybrid {
        /// Weight for target pull (milliunits, 0-1000)
        target_weight: u64,
        /// Weight for neighbor pull (milliunits, 0-1000)
        neighbor_weight: u64,
    },
}

impl Default for CouplingMode {
    fn default() -> Self {
        CouplingMode::Hybrid {
            target_weight: 500,
            neighbor_weight: 500,
        }
    }
}

/// Blend two values using integer weights (safe from division by zero)
///
/// Uses geodesic interpolation on the modular ring.
///
/// # Safety
/// If both weights are zero, returns the target value (safe fallback).
pub fn blend_weighted(
    target: u64,
    neighbor: u64,
    target_weight: u64,
    neighbor_weight: u64,
    m: u64,
) -> u64 {
    let total = target_weight + neighbor_weight;
    
    // Guard: if no weight, default to target
    if total == 0 {
        return target;
    }
    
    // If neighbor weight is zero, just return target
    if neighbor_weight == 0 {
        return target;
    }
    
    // If target weight is zero, just return neighbor
    if target_weight == 0 {
        return neighbor;
    }
    
    // Compute geodesic interpolation
    let delta = signed_geodesic(neighbor, target, m);
    let weighted_delta = (delta * neighbor_weight as i64) / total as i64;
    
    modular_add(target, weighted_delta, m)
}

/// Coupled transition with safety guarantees
pub fn coupled_transition(
    current: u64,
    target: u64,
    neighbor_mean: u64,
    coupling: CouplingMode,
    m: u64,
) -> u64 {
    match coupling {
        CouplingMode::Independent => {
            fourth_attractor_step_dithered(current, target, m)
        }
        CouplingMode::Global => {
            // Pull toward neighbor mean only
            fourth_attractor_step_dithered(current, neighbor_mean, m)
        }
        CouplingMode::Hybrid { target_weight, neighbor_weight } => {
            // Compute both pulls
            let target_pull = fourth_attractor_step_dithered(current, target, m);
            let neighbor_pull = fourth_attractor_step_dithered(current, neighbor_mean, m);
            
            // Blend with weights
            blend_weighted(target_pull, neighbor_pull, target_weight, neighbor_weight, m)
        }
    }
}

/// EPRAM Cell trait
///
/// All permanent residents must implement this trait.
pub trait EPRAMCell: Clone + Send + Sync {
    /// The value type stored in the cell
    type Value: Copy + Clone + Send + Sync;
    
    /// Get the current value
    fn value(&self) -> Self::Value;
    
    /// Get the modulus
    fn modulus(&self) -> u64;
    
    /// Perform a transition step toward target
    fn transition(&self, neighbors: &[Self::Value], target: Self::Value) -> Self;
    
    /// Check if at equilibrium (at target)
    fn at_equilibrium(&self, target: Self::Value) -> bool;
}

/// Generic EPRAM Field
pub struct EPRAMField<C: EPRAMCell> {
    cells: Vec<C>,
    targets: Vec<C::Value>,
    topology: Topology,
    coupling: CouplingMode,
}

impl<C: EPRAMCell> EPRAMField<C> {
    /// Create a new field with given cells and targets
    pub fn new(cells: Vec<C>, targets: Vec<C::Value>, topology: Topology) -> Self {
        assert_eq!(cells.len(), targets.len(), "Cells and targets must match");
        Self {
            cells,
            targets,
            topology,
            coupling: CouplingMode::default(),
        }
    }
    
    /// Set coupling mode
    pub fn with_coupling(mut self, coupling: CouplingMode) -> Self {
        self.coupling = coupling;
        self
    }
    
    /// Get reference to cells
    pub fn cells(&self) -> &[C] {
        &self.cells
    }
    
    /// Get targets
    pub fn targets(&self) -> &[C::Value] {
        &self.targets
    }
    
    /// Get topology (Copy so no borrowing issues)
    pub fn topology(&self) -> Topology {
        self.topology
    }
    
    /// Get coupling mode
    pub fn coupling(&self) -> CouplingMode {
        self.coupling
    }
    
    /// Number of cells
    pub fn len(&self) -> usize {
        self.cells.len()
    }
    
    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.cells.is_empty()
    }
    
    /// Check if all cells at equilibrium
    pub fn at_equilibrium(&self) -> bool {
        self.cells.iter()
            .zip(self.targets.iter())
            .all(|(cell, &target)| cell.at_equilibrium(target))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_topology_grid_neighbors() {
        // 3x3 grid
        let topo = Topology::Grid { width: 3 };
        
        // Center cell (index 4) should have 4 neighbors
        let neighbors = topo.neighbors(4, 9);
        assert_eq!(neighbors.len(), 4);
        assert!(neighbors.contains(&1)); // Up
        assert!(neighbors.contains(&7)); // Down
        assert!(neighbors.contains(&3)); // Left
        assert!(neighbors.contains(&5)); // Right
        
        // Corner cell (index 0) should have 2 neighbors
        let neighbors = topo.neighbors(0, 9);
        assert_eq!(neighbors.len(), 2);
    }
    
    #[test]
    fn test_topology_ring_neighbors() {
        let topo = Topology::Ring;
        
        let neighbors = topo.neighbors(0, 5);
        assert_eq!(neighbors.len(), 2);
        assert!(neighbors.contains(&4)); // Previous (wraps)
        assert!(neighbors.contains(&1)); // Next
        
        let neighbors = topo.neighbors(2, 5);
        assert!(neighbors.contains(&1));
        assert!(neighbors.contains(&3));
    }
    
    #[test]
    fn test_topology_fully_connected() {
        let topo = Topology::FullyConnected;
        
        let neighbors = topo.neighbors(0, 5);
        assert_eq!(neighbors.len(), 4);
        assert!(!neighbors.contains(&0)); // Not self
    }
    
    #[test]
    fn test_blend_weighted_zero_guard() {
        let m = 256u64;
        
        // Both weights zero should return target
        let result = blend_weighted(100, 200, 0, 0, m);
        assert_eq!(result, 100);
        
        // Neighbor weight zero should return target
        let result = blend_weighted(100, 200, 500, 0, m);
        assert_eq!(result, 100);
        
        // Target weight zero should return neighbor
        let result = blend_weighted(100, 200, 0, 500, m);
        assert_eq!(result, 200);
    }
    
    #[test]
    fn test_blend_weighted_equal() {
        let m = 256u64;
        
        // Equal weights should give midpoint
        let result = blend_weighted(0, 100, 500, 500, m);
        assert_eq!(result, 50);
    }
    
    #[test]
    fn test_blend_weighted_wrap() {
        let m = 256u64;
        
        // Test wrap-around: blend 250 and 10 should give ~2
        let result = blend_weighted(250, 10, 500, 500, m);
        // Midpoint of short arc from 250 to 10 is near 2
        assert!(
            result >= 250 || result <= 14,
            "Blend {} should be near wrap point",
            result
        );
    }
    
    #[test]
    fn test_topology_is_copy() {
        // This test verifies Topology derives Copy by using it in a Copy context
        let topo = Topology::Grid { width: 4 };
        let copy = topo; // Would fail if not Copy
        let _ = topo; // Can still use original
        let _ = copy;
    }
    
    #[test]
    fn test_coupling_mode_default() {
        let coupling = CouplingMode::default();
        match coupling {
            CouplingMode::Hybrid { target_weight, neighbor_weight } => {
                assert_eq!(target_weight, 500);
                assert_eq!(neighbor_weight, 500);
            }
            _ => panic!("Default should be Hybrid"),
        }
    }
}
