#!/usr/bin/env python3
"""
Test script for K-Elimination integration in Grover Swarm MCP

Demonstrates:
1. Knowledge base initialization with K-Elimination facts
2. Reconnaissance queries
3. Innovation potential computation
4. Full swarm search
"""

import json
import subprocess
import sys


def send_request(method, params=None):
    """Send MCP request to server"""
    request = {
        "jsonrpc": "2.0",
        "method": method,
        "params": params or {},
        "id": 1
    }

    # Start server process
    proc = subprocess.Popen(
        ["python3", "/home/acid/Projects/NINE65/MANA-private/mcp-server/grover_swarm_server.py"],
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True
    )

    # Skip initialization message
    proc.stdout.readline()

    # Send request
    proc.stdin.write(json.dumps(request) + "\n")
    proc.stdin.flush()

    # Read response
    response_line = proc.stdout.readline()
    proc.terminate()

    return json.loads(response_line)


def test_initialize():
    """Test K-Elimination knowledge initialization"""
    print("=" * 70)
    print("TEST 1: Initialize K-Elimination Knowledge Base")
    print("=" * 70)

    response = send_request("tools/call", {
        "name": "grover_initialize_kelimination",
        "arguments": {}
    })

    result = json.loads(response["content"][0]["text"])
    print(json.dumps(result, indent=2))
    print()


def test_status():
    """Test graph status"""
    print("=" * 70)
    print("TEST 2: Get Knowledge Graph Status")
    print("=" * 70)

    response = send_request("tools/call", {
        "name": "grover_get_status",
        "arguments": {}
    })

    result = json.loads(response["content"][0]["text"])
    print(json.dumps(result, indent=2))
    print()


def test_recon():
    """Test reconnaissance query"""
    print("=" * 70)
    print("TEST 3: Reconnaissance Query - 'exact division'")
    print("=" * 70)

    response = send_request("tools/call", {
        "name": "grover_query_recon",
        "arguments": {
            "query": "exact division",
            "maxResults": 5
        }
    })

    result = json.loads(response["content"][0]["text"])
    print(json.dumps(result, indent=2))
    print()


def test_innovation_potential():
    """Test innovation potential computation"""
    print("=" * 70)
    print("TEST 4: Compute Innovation Potential - 'kelim_core'")
    print("=" * 70)

    response = send_request("tools/call", {
        "name": "grover_compute_potential",
        "arguments": {
            "atomId": "kelim_core",
            "lambda": "75"
        }
    })

    result = json.loads(response["content"][0]["text"])
    print(json.dumps(result, indent=2))

    if "innovationPotential" in result:
        print()
        print("INTERPRETATION:")
        print(f"  Weight Sum: {result['components']['weightSum']} (neighbor importance)")
        print(f"  Mix Term: {result['components']['mixTerm']} (cross-topic pairs)")
        print(f"  λ × mix: {result['components']['lambdaTimesMix']}")
        print(f"  Total I(v): {result['innovationPotential']}")
        print(f"  Diverse Crossroads: {result['isDiverseCrossroads']}")
    print()


def test_swarm_search():
    """Test full swarm search"""
    print("=" * 70)
    print("TEST 5: Launch Swarm Search - 'bootstrap-free'")
    print("=" * 70)

    response = send_request("tools/call", {
        "name": "grover_launch_swarm",
        "arguments": {
            "query": "bootstrap-free",
            "maxWaves": 10,
            "config": {
                "alpha": "150",
                "beta": "20",
                "lambda": "75"
            }
        }
    })

    result = json.loads(response["content"][0]["text"])
    print(json.dumps(result, indent=2))

    if "wavesExecuted" in result:
        print()
        print("SUMMARY:")
        print(f"  Converged: {result['converged']}")
        print(f"  Waves Executed: {result['wavesExecuted']}")
        print(f"  Total Discoveries: {result['totalDiscoveries']}")
        print(f"  Innovation Score: {result['innovationScore']}")
    print()


def test_validate():
    """Test discovery validation"""
    print("=" * 70)
    print("TEST 6: Validate Discovery")
    print("=" * 70)

    response = send_request("tools/call", {
        "name": "grover_validate_discovery",
        "arguments": {
            "atomId": "bootstrap_free",
            "witnesses": ["kelim_core", "crt_uniqueness"],
            "compositionRule": "K-Elimination exact division enables bootstrap-free rescaling"
        }
    })

    result = json.loads(response["content"][0]["text"])
    print(json.dumps(result, indent=2))
    print()


def main():
    """Run all tests"""
    print("\n")
    print("╔" + "=" * 68 + "╗")
    print("║" + " " * 10 + "GROVER SWARM MCP - K-ELIMINATION INTEGRATION TEST" + " " * 8 + "║")
    print("╚" + "=" * 68 + "╝")
    print()

    tests = [
        ("Initialize Knowledge Base", test_initialize),
        ("Graph Status", test_status),
        ("Reconnaissance Query", test_recon),
        ("Innovation Potential", test_innovation_potential),
        ("Swarm Search", test_swarm_search),
        ("Validate Discovery", test_validate)
    ]

    for i, (name, test_func) in enumerate(tests, 1):
        try:
            test_func()
        except Exception as e:
            print(f"ERROR in {name}: {e}")
            print()

    print("=" * 70)
    print("All tests completed!")
    print("=" * 70)
    print()
    print("Knowledge graph saved to:")
    print("  /home/acid/Projects/NINE65/MANA-private/mcp-server/knowledge_graph.json")
    print()


if __name__ == "__main__":
    main()
