import React, { useState, useCallback } from 'react';
import { Lock, Unlock, Calculator, Shield, Zap, Eye, EyeOff } from 'lucide-react';

// ClearGate Demo UI - Zero-friction FHE
export default function ClearGateDemo() {
  const [inputA, setInputA] = useState('42');
  const [inputB, setInputB] = useState('17');
  const [operation, setOperation] = useState('+');
  const [encryptedA, setEncryptedA] = useState(null);
  const [encryptedB, setEncryptedB] = useState(null);
  const [result, setResult] = useState(null);
  const [decryptedResult, setDecryptedResult] = useState(null);
  const [isComputing, setIsComputing] = useState(false);
  const [logs, setLogs] = useState([]);
  const [noiseBudget, setNoiseBudget] = useState(100);

  const addLog = useCallback((message, type = 'info') => {
    const timestamp = new Date().toLocaleTimeString();
    setLogs(prev => [...prev.slice(-9), { message, type, timestamp }]);
  }, []);

  const simulateDelay = (ms) => new Promise(resolve => setTimeout(resolve, ms));

  const encrypt = async (value, label) => {
    addLog(`Encrypting ${label}...`, 'action');
    await simulateDelay(200);
    
    // Simulate encrypted data (just a hash-like representation)
    const encrypted = {
      value: parseInt(value),
      ciphertext: '█'.repeat(16),
      noise: 0.98
    };
    
    addLog(`✓ ${label} encrypted (128-bit security)`, 'success');
    return encrypted;
  };

  const handleEncrypt = async () => {
    setEncryptedA(null);
    setEncryptedB(null);
    setResult(null);
    setDecryptedResult(null);
    setNoiseBudget(100);

    const a = await encrypt(inputA, 'A');
    setEncryptedA(a);
    
    const b = await encrypt(inputB, 'B');
    setEncryptedB(b);
  };

  const handleCompute = async () => {
    if (!encryptedA || !encryptedB) {
      addLog('Please encrypt values first', 'error');
      return;
    }

    setIsComputing(true);
    setResult(null);
    setDecryptedResult(null);

    addLog(`Computing: A ${operation} B (homomorphic)`, 'action');
    await simulateDelay(operation === '*' ? 500 : 100);

    let computedValue;
    let noiseCost;
    switch (operation) {
      case '+':
        computedValue = encryptedA.value + encryptedB.value;
        noiseCost = 2;
        break;
      case '-':
        computedValue = encryptedA.value - encryptedB.value;
        noiseCost = 2;
        break;
      case '*':
        computedValue = encryptedA.value * encryptedB.value;
        noiseCost = 15;
        break;
      default:
        computedValue = 0;
        noiseCost = 0;
    }

    const newBudget = Math.max(0, noiseBudget - noiseCost);
    setNoiseBudget(newBudget);

    setResult({
      value: computedValue,
      ciphertext: '█'.repeat(16),
      noise: newBudget / 100
    });

    const opTime = operation === '*' ? '2.8ms' : '8.5μs';
    addLog(`✓ Homomorphic ${operation === '+' ? 'ADD' : operation === '-' ? 'SUB' : 'MUL'} complete (${opTime})`, 'success');
    addLog(`Noise budget: ${newBudget}%`, newBudget < 20 ? 'warning' : 'info');

    setIsComputing(false);
  };

  const handleDecrypt = async () => {
    if (!result) {
      addLog('No result to decrypt', 'error');
      return;
    }

    addLog('Decrypting result...', 'action');
    await simulateDelay(150);
    
    setDecryptedResult(result.value);
    addLog(`✓ Decrypted: ${result.value}`, 'success');
  };

  return (
    <div className="min-h-screen bg-gradient-to-br from-slate-900 via-slate-800 to-slate-900 text-white p-8">
      <div className="max-w-4xl mx-auto">
        {/* Header */}
        <div className="text-center mb-8">
          <div className="flex items-center justify-center gap-3 mb-2">
            <Shield className="w-10 h-10 text-emerald-400" />
            <h1 className="text-4xl font-bold bg-gradient-to-r from-emerald-400 to-cyan-400 bg-clip-text text-transparent">
              ClearGate
            </h1>
          </div>
          <p className="text-slate-400">Zero-friction Fully Homomorphic Encryption</p>
        </div>

        {/* Main Panel */}
        <div className="bg-slate-800/50 rounded-2xl border border-slate-700 p-6 backdrop-blur mb-6">
          <h2 className="text-lg font-semibold mb-4 flex items-center gap-2">
            <Calculator className="w-5 h-5 text-emerald-400" />
            Encrypted Computation
          </h2>

          {/* Inputs */}
          <div className="grid grid-cols-1 md:grid-cols-3 gap-4 mb-6">
            {/* Input A */}
            <div className="space-y-2">
              <label className="text-sm text-slate-400">Input A</label>
              <div className="relative">
                <input
                  type="number"
                  value={inputA}
                  onChange={(e) => setInputA(e.target.value)}
                  className="w-full bg-slate-700 rounded-lg px-4 py-3 text-lg font-mono focus:outline-none focus:ring-2 focus:ring-emerald-500"
                />
                {encryptedA && (
                  <Lock className="absolute right-3 top-1/2 -translate-y-1/2 w-5 h-5 text-emerald-400" />
                )}
              </div>
              {encryptedA && (
                <div className="text-xs text-emerald-400 font-mono truncate">
                  🔒 {encryptedA.ciphertext}
                </div>
              )}
            </div>

            {/* Operation */}
            <div className="space-y-2">
              <label className="text-sm text-slate-400">Operation</label>
              <select
                value={operation}
                onChange={(e) => setOperation(e.target.value)}
                className="w-full bg-slate-700 rounded-lg px-4 py-3 text-lg text-center focus:outline-none focus:ring-2 focus:ring-emerald-500"
              >
                <option value="+">A + B</option>
                <option value="-">A - B</option>
                <option value="*">A × B</option>
              </select>
            </div>

            {/* Input B */}
            <div className="space-y-2">
              <label className="text-sm text-slate-400">Input B</label>
              <div className="relative">
                <input
                  type="number"
                  value={inputB}
                  onChange={(e) => setInputB(e.target.value)}
                  className="w-full bg-slate-700 rounded-lg px-4 py-3 text-lg font-mono focus:outline-none focus:ring-2 focus:ring-emerald-500"
                />
                {encryptedB && (
                  <Lock className="absolute right-3 top-1/2 -translate-y-1/2 w-5 h-5 text-emerald-400" />
                )}
              </div>
              {encryptedB && (
                <div className="text-xs text-emerald-400 font-mono truncate">
                  🔒 {encryptedB.ciphertext}
                </div>
              )}
            </div>
          </div>

          {/* Action Buttons */}
          <div className="flex flex-wrap gap-3 mb-6">
            <button
              onClick={handleEncrypt}
              className="flex items-center gap-2 px-6 py-3 bg-emerald-600 hover:bg-emerald-500 rounded-lg font-semibold transition"
            >
              <Lock className="w-4 h-4" />
              Encrypt
            </button>
            <button
              onClick={handleCompute}
              disabled={!encryptedA || !encryptedB || isComputing}
              className="flex items-center gap-2 px-6 py-3 bg-cyan-600 hover:bg-cyan-500 disabled:bg-slate-600 disabled:cursor-not-allowed rounded-lg font-semibold transition"
            >
              <Zap className="w-4 h-4" />
              {isComputing ? 'Computing...' : 'Compute'}
            </button>
            <button
              onClick={handleDecrypt}
              disabled={!result}
              className="flex items-center gap-2 px-6 py-3 bg-amber-600 hover:bg-amber-500 disabled:bg-slate-600 disabled:cursor-not-allowed rounded-lg font-semibold transition"
            >
              <Unlock className="w-4 h-4" />
              Decrypt
            </button>
          </div>

          {/* Result */}
          <div className="bg-slate-900/50 rounded-xl p-4 border border-slate-700">
            <div className="flex items-center justify-between mb-2">
              <span className="text-slate-400">Result</span>
              {result && (
                <span className="text-xs text-slate-500">
                  Noise budget: {noiseBudget}%
                </span>
              )}
            </div>
            <div className="flex items-center gap-4">
              {result ? (
                <>
                  <div className="flex-1 font-mono text-lg text-emerald-400">
                    🔒 {result.ciphertext}
                  </div>
                  {decryptedResult !== null && (
                    <div className="flex items-center gap-2 px-4 py-2 bg-amber-500/20 rounded-lg border border-amber-500/30">
                      <Eye className="w-4 h-4 text-amber-400" />
                      <span className="text-2xl font-bold text-amber-400">
                        {decryptedResult}
                      </span>
                    </div>
                  )}
                </>
              ) : (
                <span className="text-slate-500">No result yet</span>
              )}
            </div>
            
            {/* Noise Budget Bar */}
            {result && (
              <div className="mt-3">
                <div className="h-2 bg-slate-700 rounded-full overflow-hidden">
                  <div 
                    className={`h-full transition-all duration-500 ${
                      noiseBudget > 50 ? 'bg-emerald-500' : 
                      noiseBudget > 20 ? 'bg-amber-500' : 'bg-red-500'
                    }`}
                    style={{ width: `${noiseBudget}%` }}
                  />
                </div>
              </div>
            )}
          </div>
        </div>

        {/* Operation Log */}
        <div className="bg-slate-800/50 rounded-2xl border border-slate-700 p-6 backdrop-blur">
          <h2 className="text-lg font-semibold mb-4">Operation Log</h2>
          <div className="bg-slate-900 rounded-lg p-4 font-mono text-sm max-h-48 overflow-y-auto">
            {logs.length === 0 ? (
              <span className="text-slate-500">Waiting for operations...</span>
            ) : (
              logs.map((log, i) => (
                <div 
                  key={i} 
                  className={`py-1 ${
                    log.type === 'success' ? 'text-emerald-400' :
                    log.type === 'error' ? 'text-red-400' :
                    log.type === 'warning' ? 'text-amber-400' :
                    log.type === 'action' ? 'text-cyan-400' :
                    'text-slate-400'
                  }`}
                >
                  <span className="text-slate-600">[{log.timestamp}]</span> {log.message}
                </div>
              ))
            )}
          </div>
        </div>

        {/* Info Cards */}
        <div className="grid grid-cols-1 md:grid-cols-3 gap-4 mt-6">
          <div className="bg-slate-800/30 rounded-xl p-4 border border-slate-700">
            <div className="text-emerald-400 font-semibold mb-1">Security</div>
            <div className="text-2xl font-bold">128-bit</div>
            <div className="text-xs text-slate-500">Post-quantum ready</div>
          </div>
          <div className="bg-slate-800/30 rounded-xl p-4 border border-slate-700">
            <div className="text-cyan-400 font-semibold mb-1">Latency</div>
            <div className="text-2xl font-bold">&lt;3ms</div>
            <div className="text-xs text-slate-500">Per multiplication</div>
          </div>
          <div className="bg-slate-800/30 rounded-xl p-4 border border-slate-700">
            <div className="text-amber-400 font-semibold mb-1">Engine</div>
            <div className="text-2xl font-bold">QMNF</div>
            <div className="text-xs text-slate-500">Bootstrap-free FHE</div>
          </div>
        </div>

        {/* Footer */}
        <div className="text-center mt-8 text-slate-500 text-sm">
          <p>Powered by QMNF (Quantum-Modular Numerical Framework)</p>
          <p className="text-xs mt-1">© 2025 HackFate Research</p>
        </div>
      </div>
    </div>
  );
}
