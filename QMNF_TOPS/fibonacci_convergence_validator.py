def fibonacci_convergence_validator(sequence: list[int], tolerance_percent: int = 1) -> dict:
    """
    Validate if a sequence converges to the golden ratio φ.
    
    Args:
        sequence: List of integers to check for Fibonacci-like convergence
        tolerance_percent: How close ratios need to be to φ to count as convergence
        
    Returns:
        {
            "converges_to_phi": bool,
            "convergence_start_index": int,  # Index where convergence begins
            "convergent_ratios": list[float],  # Ratios that match φ
            "phi_approximation": float,  # Best approximation of φ in sequence
            "convergence_strength": int  # 0-100 score of how well it converges
        }
    """
    PHI_SCALED = 1618033988749895  # φ × 10^15
    SCALE = 1000000000000000       # 10^15
    
    if len(sequence) < 2:
        return {
            "converges_to_phi": False,
            "convergence_start_index": -1,
            "convergent_ratios": [],
            "phi_approximation": 0.0,
            "convergence_strength": 0
        }
    
    # Calculate ratios between consecutive elements
    ratios_scaled = []
    for i in range(len(sequence) - 1):
        if sequence[i] != 0:  # Avoid division by zero
            # Use integer arithmetic to calculate ratio
            ratio_scaled = (sequence[i+1] * SCALE) // sequence[i]
            ratios_scaled.append((i, ratio_scaled))
    
    if not ratios_scaled:
        return {
            "converges_to_phi": False,
            "convergence_start_index": -1,
            "convergent_ratios": [],
            "phi_approximation": 0.0,
            "convergence_strength": 0
        }
    
    # Calculate tolerance
    tolerance_scaled = (PHI_SCALED * tolerance_percent) // 100
    
    # Find sequences of consecutive ratios that match φ
    convergent_sequences = []
    current_sequence = []
    
    for idx, (orig_idx, ratio_scaled) in enumerate(ratios_scaled):
        diff = abs(ratio_scaled - PHI_SCALED)
        
        if diff <= tolerance_scaled:
            current_sequence.append((orig_idx, ratio_scaled))
        else:
            if len(current_sequence) >= 2:  # At least 2 consecutive matches
                convergent_sequences.append(current_sequence[:])
            current_sequence = []
    
    # Check if the last sequence was convergent
    if len(current_sequence) >= 2:
        convergent_sequences.append(current_sequence)
    
    # Determine convergence
    converges_to_phi = len(convergent_sequences) > 0
    convergence_start_index = -1
    convergent_ratios = []
    phi_approximation = 0.0
    convergence_strength = 0
    
    if converges_to_phi:
        # Use the first (and longest) convergent sequence
        best_sequence = max(convergent_sequences, key=len)
        convergence_start_index = best_sequence[0][0]
        
        for orig_idx, ratio_scaled in best_sequence:
            convergent_ratios.append(ratio_scaled / SCALE)
        
        # Calculate average ratio as best φ approximation
        total_ratio = sum(ratio_scaled for _, ratio_scaled in best_sequence)
        phi_approximation = total_ratio / len(best_sequence) / SCALE
        
        # Calculate strength based on number of consecutive matches
        length = len(best_sequence)
        if length >= 5:
            convergence_strength = 95
        elif length >= 3:
            convergence_strength = 80
        elif length == 2:
            convergence_strength = 50
    else:
        # Check if any individual ratios match φ even if not consecutive
        matching_ratios = [(idx, ratio) for idx, ratio in ratios_scaled if abs(ratio - PHI_SCALED) <= tolerance_scaled]
        if len(matching_ratios) > 0:
            # For non-consecutive matches, set basic convergence info
            converges_to_phi = True  # Let's say if we have some matches, there's potential convergence
            convergence_strength = min(30, len(matching_ratios) * 15)  # Base strength on number of matches
            convergent_ratios = [ratio/SCALE for _, ratio in matching_ratios]
            phi_approximation = sum([ratio for _, ratio in matching_ratios])/len(matching_ratios)/SCALE if matching_ratios else 0.0
            if matching_ratios:
                convergence_start_index = min([idx for idx, _ in matching_ratios])

    return {
        "converges_to_phi": converges_to_phi,
        "convergence_start_index": convergence_start_index,
        "convergent_ratios": convergent_ratios,
        "phi_approximation": phi_approximation,
        "convergence_strength": convergence_strength
    }


# Test the function
if __name__ == "__main__":
    print("FIBONACCI CONVERGENCE VALIDATOR TESTS")
    print("=" * 50)
    
    # Test with actual Fibonacci sequence
    fib_sequence = [1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144]
    result = fibonacci_convergence_validator(fib_sequence)
    print(f"Fibonacci sequence: {fib_sequence[:7]}...")
    print(f"Converges to φ: {result['converges_to_phi']}")
    print(f"φ approximation: {result['phi_approximation']:.6f}")
    print(f"Convergence strength: {result['convergence_strength']}%")
    
    # Test with random sequence
    random_seq = [10, 15, 20, 25, 30]
    result2 = fibonacci_convergence_validator(random_seq)
    print(f"\nRandom sequence: {random_seq}")
    print(f"Converges to φ: {result2['converges_to_phi']}")
    print(f"Convergence strength: {result2['convergence_strength']}%")
    
    # Test with custom sequence that should converge
    custom_seq = [100, 162, 262, 424, 686, 1110]  # Approximates Fibonacci ratios
    result3 = fibonacci_convergence_validator(custom_seq)
    print(f"\nCustom sequence: {custom_seq}")
    print(f"Converges to φ: {result3['converges_to_phi']}")
    print(f"φ approximation: {result3['phi_approximation']:.6f}")
    print(f"Convergent ratios: {result3['convergent_ratios']}")
    print(f"Convergence strength: {result3['convergence_strength']}%")