import React, { useState, useEffect } from 'react';
import { LineChart, Line, XAxis, YAxis, Tooltip, ResponsiveContainer, AreaChart, Area, BarChart, Bar, Cell, PieChart, Pie } from 'recharts';
import { Search, TrendingUp, TrendingDown, AlertCircle, CheckCircle, Clock, Wrench, Car, DollarSign, MapPin, Flame, Target, Filter, Bell, Settings, ChevronRight, ChevronDown, Star, Zap, Shield, Activity, BarChart3, PieChart as PieChartIcon, ArrowUpRight, ArrowDownRight, Plus, X, Eye, Phone, Mail } from 'lucide-react';

// Sample Data
const buildReadyOpportunities = [
  { id: 1, vehicle: '2024 Huracán Tecnica', location: 'Grand Prairie, TX', damage: 'Rear end', currentBid: 51500, estBuildCost: 45000, builtValue: 295000, margin: 45, score: 2450, status: 'active', daysListed: 3, miles: 3197, platform: 'Copart' },
  { id: 2, vehicle: '2023 Huracán Tecnica', location: 'Grand Prairie, TX', damage: 'Rear end', currentBid: 122000, estBuildCost: 40000, builtValue: 290000, margin: 35, score: 2180, status: 'active', daysListed: 5, miles: 4329, platform: 'IAAI' },
  { id: 3, vehicle: '2024 Purosangue', location: 'Chicago, IL', damage: 'All over', currentBid: 20000, estBuildCost: 100000, builtValue: 380000, margin: 52, score: 2650, status: 'active', daysListed: 2, miles: 0, platform: 'Abetter.bid' },
  { id: 4, vehicle: '2020 Huracán EVO', location: 'Martinez, CA', damage: 'Unknown', currentBid: 103000, estBuildCost: 42000, builtValue: 240000, margin: 32, score: 1950, status: 'active', daysListed: 4, miles: 5253, platform: 'SCA' },
  { id: 5, vehicle: '2024 GT3 RS', location: 'Graham, WA', damage: 'Salvage', currentBid: 145000, estBuildCost: 65000, builtValue: 310000, margin: 38, score: 2100, status: 'active', daysListed: 1, miles: 890, platform: 'Copart' },
  { id: 6, vehicle: '2021 GT-R Nismo', location: 'Houston, TX', damage: 'Flood', currentBid: 52000, estBuildCost: 45000, builtValue: 165000, margin: 41, score: 2280, status: 'active', daysListed: 6, miles: 8400, platform: 'IAAI' },
];

const turnkeyOpportunities = [
  { id: 1, vehicle: '2022 GT3 Touring 6MT', location: 'Dallas, TX', price: 239000, miles: 4614, compMedian: 258000, delta: -7.4, score: 1920, status: 'call', daysListed: 8, source: 'Private', spec: 'Ruby Star PTS, PCCB' },
  { id: 2, vehicle: '2024 Huracán Tecnica', location: 'Pompano, FL', price: 330000, miles: 1500, compMedian: 345000, delta: -4.3, score: 1780, status: 'monitor', daysListed: 12, source: 'Dealer', spec: 'Blu Cepheus, PPF' },
  { id: 3, vehicle: '2023 GT3 Touring PDK', location: 'Kentucky', price: 275000, miles: 1600, compMedian: 285000, delta: -3.5, score: 1750, status: 'monitor', daysListed: 5, source: 'BaT', spec: 'Racing Yellow, Chrono' },
  { id: 4, vehicle: '2024 Roma Spider', location: 'California', price: 335000, miles: 702, compMedian: 355000, delta: -5.6, score: 1820, status: 'call', daysListed: 3, source: 'BaT', spec: 'Verde British Racing' },
  { id: 5, vehicle: '2019 488 Spider', location: 'Miami, FL', price: 285000, miles: 8200, compMedian: 298000, delta: -4.4, score: 1690, status: 'monitor', daysListed: 15, source: 'Dealer', spec: 'Rosso Corsa, Carbon' },
];

const marketHeatData = [
  { model: 'GT3 Touring', trend: 'tightening', change: 3.2, volume: 45, avgDays: 28 },
  { model: 'GT3 RS', trend: 'stable', change: 0.5, volume: 23, avgDays: 35 },
  { model: 'GT4 RS', trend: 'tightening', change: 4.8, volume: 18, avgDays: 22 },
  { model: '488 Pista', trend: 'stable', change: -0.3, volume: 31, avgDays: 42 },
  { model: 'Huracán STO', trend: 'stable', change: 0.8, volume: 27, avgDays: 38 },
  { model: 'Huracán Tecnica', trend: 'softening', change: -2.1, volume: 52, avgDays: 45 },
];

const pipelineHistory = [
  { month: 'Jul', turnkey: 4, build: 2, margin: 142000 },
  { month: 'Aug', turnkey: 5, build: 3, margin: 198000 },
  { month: 'Sep', turnkey: 3, build: 4, margin: 245000 },
  { month: 'Oct', turnkey: 6, build: 3, margin: 278000 },
  { month: 'Nov', turnkey: 4, build: 5, margin: 312000 },
  { month: 'Dec', turnkey: 7, build: 4, margin: 385000 },
];

const wantedBoard = [
  { model: 'Huracán Tecnica/EVO', priority: 1, maxBid: 85000, targetMargin: 35, alerts: true },
  { model: 'Ferrari Purosangue', priority: 2, maxBid: 120000, targetMargin: 40, alerts: true },
  { model: '911 GT3 RS/Touring', priority: 3, maxBid: 160000, targetMargin: 35, alerts: true },
  { model: 'GT-R R35 Nismo', priority: 4, maxBid: 65000, targetMargin: 40, alerts: true },
];

export default function ExoticAcquisitionRadar() {
  const [activeTab, setActiveTab] = useState('dashboard');
  const [mounted, setMounted] = useState(false);
  
  useEffect(() => {
    setMounted(true);
  }, []);

  const totalBuildValue = buildReadyOpportunities.reduce((sum, o) => sum + (o.builtValue - o.currentBid - o.estBuildCost), 0);
  const totalTurnkeyValue = turnkeyOpportunities.reduce((sum, o) => sum + (o.compMedian - o.price), 0);
  const avgBuildMargin = Math.round(buildReadyOpportunities.reduce((sum, o) => sum + o.margin, 0) / buildReadyOpportunities.length);

  return (
    <div className="min-h-screen bg-zinc-950 text-zinc-100" style={{ fontFamily: 'system-ui, -apple-system, sans-serif' }}>
      {/* Background */}
      <div className="fixed inset-0 pointer-events-none">
        <div className="absolute inset-0 bg-gradient-to-br from-zinc-950 via-zinc-900 to-zinc-950" />
        <div className="absolute top-20 left-20 w-96 h-96 bg-amber-500/5 rounded-full blur-3xl" />
        <div className="absolute bottom-40 right-20 w-80 h-80 bg-orange-500/5 rounded-full blur-3xl" />
      </div>

      {/* Header */}
      <header className={`relative z-50 border-b border-zinc-800/50 backdrop-blur-xl bg-zinc-950/80 transition-all duration-700 ${mounted ? 'opacity-100' : 'opacity-0'}`}>
        <div className="max-w-7xl mx-auto px-6 py-4">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-4">
              <div className="w-12 h-12 bg-gradient-to-br from-amber-400 to-orange-600 rounded-lg flex items-center justify-center shadow-lg shadow-amber-500/20">
                <Target className="w-6 h-6 text-zinc-950" strokeWidth={2.5} />
              </div>
              <div>
                <h1 className="text-xl font-bold bg-gradient-to-r from-amber-200 via-amber-400 to-orange-400 bg-clip-text text-transparent">
                  EXOTIC ACQUISITION RADAR
                </h1>
                <p className="text-xs text-zinc-500 tracking-widest uppercase">San Antonio HQ • Deterministic Scoring</p>
              </div>
            </div>

            <nav className="flex items-center gap-1 bg-zinc-900/50 rounded-full p-1 border border-zinc-800/50">
              {[
                { id: 'dashboard', label: 'Dashboard', icon: Activity },
                { id: 'build', label: 'Build', icon: Wrench },
                { id: 'turnkey', label: 'Turnkey', icon: Car },
                { id: 'wanted', label: 'Wanted', icon: Target },
              ].map(tab => (
                <button
                  key={tab.id}
                  onClick={() => setActiveTab(tab.id)}
                  className={`flex items-center gap-2 px-4 py-2 rounded-full text-sm font-medium transition-all duration-300 ${
                    activeTab === tab.id 
                      ? 'bg-gradient-to-r from-amber-500 to-orange-500 text-zinc-950 shadow-lg shadow-amber-500/25' 
                      : 'text-zinc-400 hover:text-zinc-200'
                  }`}
                >
                  <tab.icon className="w-4 h-4" />
                  {tab.label}
                </button>
              ))}
            </nav>

            <div className="flex items-center gap-3">
              <button className="relative p-2 rounded-lg bg-zinc-900/50 border border-zinc-800">
                <Bell className="w-5 h-5 text-zinc-400" />
                <span className="absolute -top-1 -right-1 w-5 h-5 bg-amber-500 rounded-full text-xs font-bold text-zinc-950 flex items-center justify-center">3</span>
              </button>
            </div>
          </div>
        </div>
      </header>

      {/* Main Content */}
      <main className="relative z-10 max-w-7xl mx-auto px-6 py-8">
        {activeTab === 'dashboard' && (
          <div className="space-y-6">
            {/* KPI Cards */}
            <div className={`grid grid-cols-4 gap-4 transition-all duration-700 ${mounted ? 'opacity-100' : 'opacity-0'}`}>
              <KPICard title="Build Pipeline" value={`$${(totalBuildValue / 1000).toFixed(0)}K`} subtitle="Potential margin" trend={12.4} icon={Wrench} color="amber" />
              <KPICard title="Turnkey Opps" value={`$${(totalTurnkeyValue / 1000).toFixed(0)}K`} subtitle="Below market" trend={8.2} icon={Car} color="emerald" />
              <KPICard title="Avg Build Margin" value={`${avgBuildMargin}%`} subtitle="Active builds" trend={3.5} icon={TrendingUp} color="orange" />
              <KPICard title="Urgent Actions" value={buildReadyOpportunities.filter(o => o.score >= 2000).length + turnkeyOpportunities.filter(o => o.score >= 1800).length} subtitle="Score ≥ 2000" trend={0} icon={Flame} color="red" />
            </div>

            {/* Main Grid */}
            <div className="grid grid-cols-3 gap-6">
              {/* Immediate Action */}
              <div className="col-span-2">
                <div className="bg-zinc-900/50 rounded-2xl border border-zinc-800/50 overflow-hidden">
                  <div className="px-6 py-4 border-b border-zinc-800/50 flex items-center justify-between">
                    <div className="flex items-center gap-3">
                      <div className="w-10 h-10 bg-gradient-to-br from-red-500 to-orange-500 rounded-lg flex items-center justify-center">
                        <Flame className="w-5 h-5 text-white" />
                      </div>
                      <div>
                        <h2 className="font-semibold text-lg">Immediate Action</h2>
                        <p className="text-xs text-zinc-500">Score ≥ 2000 • Call within 1 hour</p>
                      </div>
                    </div>
                    <span className="px-3 py-1 bg-red-500/20 text-red-400 rounded-full text-sm font-medium">
                      {buildReadyOpportunities.filter(o => o.score >= 2000).length} urgent
                    </span>
                  </div>
                  <div className="divide-y divide-zinc-800/30">
                    {buildReadyOpportunities.filter(o => o.score >= 2000).slice(0, 4).map((opp) => (
                      <div key={opp.id} className="px-6 py-4 hover:bg-zinc-800/30 transition-all flex items-center justify-between group">
                        <div className="flex items-center gap-4">
                          <div className="w-12 h-12 rounded-xl bg-amber-500/20 flex items-center justify-center">
                            <Wrench className="w-6 h-6 text-amber-400" />
                          </div>
                          <div>
                            <div className="font-semibold">{opp.vehicle}</div>
                            <div className="text-sm text-zinc-500 flex items-center gap-2">
                              <MapPin className="w-3 h-3" />
                              {opp.location} • {opp.platform}
                            </div>
                          </div>
                        </div>
                        <div className="flex items-center gap-6">
                          <div className="text-right">
                            <div className="font-mono text-lg">${opp.currentBid.toLocaleString()}</div>
                            <div className="text-sm text-zinc-500">→ ${opp.builtValue.toLocaleString()} built</div>
                          </div>
                          <div className={`px-4 py-2 rounded-xl font-mono font-bold flex items-center gap-2 ${
                            opp.score >= 2500 ? 'bg-red-500/20 text-red-400' : 'bg-amber-500/20 text-amber-400'
                          }`}>
                            {opp.score >= 2500 && <Flame className="w-4 h-4" />}
                            {opp.score}
                          </div>
                          <button className="px-6 py-2 bg-gradient-to-r from-amber-500 to-orange-500 text-zinc-950 rounded-xl font-semibold opacity-0 group-hover:opacity-100 transition-all">
                            BID
                          </button>
                        </div>
                      </div>
                    ))}
                  </div>
                </div>
              </div>

              {/* Market Heat */}
              <div className="bg-zinc-900/50 rounded-2xl border border-zinc-800/50 p-6">
                <div className="flex items-center gap-3 mb-6">
                  <div className="w-10 h-10 bg-gradient-to-br from-violet-500 to-purple-600 rounded-lg flex items-center justify-center">
                    <Activity className="w-5 h-5 text-white" />
                  </div>
                  <div>
                    <h2 className="font-semibold text-lg">Market Heat</h2>
                    <p className="text-xs text-zinc-500">7-day trends</p>
                  </div>
                </div>
                <div className="space-y-3">
                  {marketHeatData.map((model) => (
                    <div key={model.model} className="flex items-center justify-between p-3 bg-zinc-800/30 rounded-lg hover:bg-zinc-800/50 transition-all">
                      <div className="flex items-center gap-3">
                        <div className={`w-2 h-2 rounded-full ${
                          model.trend === 'tightening' ? 'bg-emerald-400' :
                          model.trend === 'softening' ? 'bg-red-400' : 'bg-zinc-500'
                        }`} />
                        <span className="text-sm font-medium">{model.model}</span>
                      </div>
                      <div className="flex items-center gap-2">
                        <span className={`text-sm font-mono ${
                          model.change > 0 ? 'text-emerald-400' : model.change < 0 ? 'text-red-400' : 'text-zinc-500'
                        }`}>
                          {model.change > 0 ? '+' : ''}{model.change}%
                        </span>
                        {model.change > 0 ? <ArrowUpRight className="w-4 h-4 text-emerald-400" /> : 
                         model.change < 0 ? <ArrowDownRight className="w-4 h-4 text-red-400" /> : null}
                      </div>
                    </div>
                  ))}
                </div>
              </div>
            </div>

            {/* Chart */}
            <div className="bg-zinc-900/50 rounded-2xl border border-zinc-800/50 p-6">
              <div className="flex items-center gap-3 mb-6">
                <div className="w-10 h-10 bg-gradient-to-br from-cyan-500 to-blue-600 rounded-lg flex items-center justify-center">
                  <BarChart3 className="w-5 h-5 text-white" />
                </div>
                <div>
                  <h2 className="font-semibold text-lg">Pipeline Performance</h2>
                  <p className="text-xs text-zinc-500">6-month margin history</p>
                </div>
              </div>
              <div className="h-64">
                <ResponsiveContainer width="100%" height="100%">
                  <AreaChart data={pipelineHistory}>
                    <defs>
                      <linearGradient id="marginGradient" x1="0" y1="0" x2="0" y2="1">
                        <stop offset="5%" stopColor="#f59e0b" stopOpacity={0.3}/>
                        <stop offset="95%" stopColor="#f59e0b" stopOpacity={0}/>
                      </linearGradient>
                    </defs>
                    <XAxis dataKey="month" axisLine={false} tickLine={false} tick={{ fill: '#71717a', fontSize: 12 }} />
                    <YAxis axisLine={false} tickLine={false} tick={{ fill: '#71717a', fontSize: 12 }} tickFormatter={(v) => `$${v/1000}K`} />
                    <Tooltip contentStyle={{ backgroundColor: '#18181b', border: '1px solid #3f3f46', borderRadius: '8px' }} />
                    <Area type="monotone" dataKey="margin" stroke="#f59e0b" fill="url(#marginGradient)" strokeWidth={2} />
                  </AreaChart>
                </ResponsiveContainer>
              </div>
            </div>

            {/* Scoring Transparency */}
            <div className="grid grid-cols-2 gap-6">
              <div className="bg-zinc-900/50 rounded-2xl border border-zinc-800/50 p-6">
                <h3 className="font-semibold mb-4 flex items-center gap-2">
                  <Shield className="w-4 h-4 text-amber-400" />
                  BUILD SCORE (Integer-Only)
                </h3>
                <div className="p-4 bg-zinc-800/30 rounded-lg border border-amber-500/20">
                  <code className="text-sm text-zinc-400 font-mono leading-relaxed">
                    S = (4 × MARGIN) + (2 × PARTS) + PLATFORM<br />
                    &nbsp;&nbsp;&nbsp;&nbsp;- (3 × COMPLEXITY) - RISK<br /><br />
                    <span className="text-amber-400">≥ 2500: ACQUIRE NOW</span><br />
                    <span className="text-emerald-400">≥ 1800: STRONG BUILD</span><br />
                    <span className="text-zinc-500">&lt; 1200: PASS</span>
                  </code>
                </div>
              </div>
              <div className="bg-zinc-900/50 rounded-2xl border border-zinc-800/50 p-6">
                <h3 className="font-semibold mb-4 flex items-center gap-2">
                  <Shield className="w-4 h-4 text-emerald-400" />
                  TURNKEY SCORE (Integer-Only)
                </h3>
                <div className="p-4 bg-zinc-800/30 rounded-lg border border-emerald-500/20">
                  <code className="text-sm text-zinc-400 font-mono leading-relaxed">
                    S = (3 × MARGIN) + (2 × DEMAND) + TURN<br />
                    &nbsp;&nbsp;&nbsp;&nbsp;- (2 × RISK) - FRICTION<br /><br />
                    <span className="text-amber-400">≥ 2000: IMMEDIATE BUY</span><br />
                    <span className="text-emerald-400">≥ 1500: STRONG</span><br />
                    <span className="text-zinc-500">&lt; 1000: PASS</span>
                  </code>
                </div>
              </div>
            </div>
          </div>
        )}

        {activeTab === 'build' && (
          <div className="space-y-6">
            <div>
              <h1 className="text-2xl font-bold">Build-Ready Pipeline</h1>
              <p className="text-zinc-500">Salvage, shells, rebuildable exotics • 25-50% margin</p>
            </div>
            <div className="bg-zinc-900/50 rounded-2xl border border-zinc-800/50 overflow-hidden">
              <table className="w-full">
                <thead>
                  <tr className="border-b border-zinc-800/50 text-left">
                    <th className="px-6 py-4 text-xs text-zinc-500 uppercase">Vehicle</th>
                    <th className="px-6 py-4 text-xs text-zinc-500 uppercase">Location</th>
                    <th className="px-6 py-4 text-xs text-zinc-500 uppercase">Damage</th>
                    <th className="px-6 py-4 text-xs text-zinc-500 uppercase text-right">Bid</th>
                    <th className="px-6 py-4 text-xs text-zinc-500 uppercase text-right">Built Value</th>
                    <th className="px-6 py-4 text-xs text-zinc-500 uppercase text-right">Margin</th>
                    <th className="px-6 py-4 text-xs text-zinc-500 uppercase text-right">Score</th>
                  </tr>
                </thead>
                <tbody className="divide-y divide-zinc-800/30">
                  {buildReadyOpportunities.map((opp) => (
                    <tr key={opp.id} className="hover:bg-zinc-800/30 transition-all">
                      <td className="px-6 py-4">
                        <div className="font-medium">{opp.vehicle}</div>
                        <div className="text-xs text-zinc-500">{opp.miles.toLocaleString()} mi • {opp.platform}</div>
                      </td>
                      <td className="px-6 py-4">
                        <span className={opp.location.includes('TX') ? 'text-emerald-400' : 'text-zinc-400'}>{opp.location}</span>
                      </td>
                      <td className="px-6 py-4">
                        <span className={`px-2 py-1 rounded text-xs ${
                          opp.damage === 'Rear end' ? 'bg-emerald-500/20 text-emerald-400' :
                          opp.damage === 'Flood' ? 'bg-blue-500/20 text-blue-400' : 'bg-amber-500/20 text-amber-400'
                        }`}>{opp.damage}</span>
                      </td>
                      <td className="px-6 py-4 text-right font-mono">${opp.currentBid.toLocaleString()}</td>
                      <td className="px-6 py-4 text-right font-mono text-emerald-400">${opp.builtValue.toLocaleString()}</td>
                      <td className="px-6 py-4 text-right font-bold text-amber-400">{opp.margin}%</td>
                      <td className="px-6 py-4 text-right">
                        <span className={`px-3 py-1 rounded-full font-mono font-bold ${
                          opp.score >= 2500 ? 'bg-red-500/20 text-red-400' :
                          opp.score >= 2000 ? 'bg-amber-500/20 text-amber-400' : 'bg-zinc-800 text-zinc-400'
                        }`}>{opp.score}</span>
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          </div>
        )}

        {activeTab === 'turnkey' && (
          <div className="space-y-6">
            <div>
              <h1 className="text-2xl font-bold">Turnkey Pipeline</h1>
              <p className="text-zinc-500">Clean title, ready-to-retail • 8-15% margin</p>
            </div>
            <div className="bg-zinc-900/50 rounded-2xl border border-zinc-800/50 overflow-hidden">
              <table className="w-full">
                <thead>
                  <tr className="border-b border-zinc-800/50 text-left">
                    <th className="px-6 py-4 text-xs text-zinc-500 uppercase">Vehicle</th>
                    <th className="px-6 py-4 text-xs text-zinc-500 uppercase">Location</th>
                    <th className="px-6 py-4 text-xs text-zinc-500 uppercase">Spec</th>
                    <th className="px-6 py-4 text-xs text-zinc-500 uppercase text-right">Price</th>
                    <th className="px-6 py-4 text-xs text-zinc-500 uppercase text-right">Comp</th>
                    <th className="px-6 py-4 text-xs text-zinc-500 uppercase text-right">Delta</th>
                    <th className="px-6 py-4 text-xs text-zinc-500 uppercase text-right">Score</th>
                  </tr>
                </thead>
                <tbody className="divide-y divide-zinc-800/30">
                  {turnkeyOpportunities.map((opp) => (
                    <tr key={opp.id} className="hover:bg-zinc-800/30 transition-all">
                      <td className="px-6 py-4">
                        <div className="font-medium">{opp.vehicle}</div>
                        <div className="text-xs text-zinc-500">{opp.miles.toLocaleString()} mi • {opp.source}</div>
                      </td>
                      <td className="px-6 py-4 text-zinc-400">{opp.location}</td>
                      <td className="px-6 py-4 text-sm text-zinc-400">{opp.spec}</td>
                      <td className="px-6 py-4 text-right font-mono">${opp.price.toLocaleString()}</td>
                      <td className="px-6 py-4 text-right font-mono text-zinc-500">${opp.compMedian.toLocaleString()}</td>
                      <td className="px-6 py-4 text-right text-emerald-400 font-bold">{opp.delta}%</td>
                      <td className="px-6 py-4 text-right">
                        <span className={`px-3 py-1 rounded-full font-mono font-bold ${
                          opp.score >= 1800 ? 'bg-emerald-500/20 text-emerald-400' : 'bg-zinc-800 text-zinc-400'
                        }`}>{opp.score}</span>
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          </div>
        )}

        {activeTab === 'wanted' && (
          <div className="space-y-6">
            <div className="flex items-center justify-between">
              <div>
                <h1 className="text-2xl font-bold">Wanted Board</h1>
                <p className="text-zinc-500">Configure targets, max bids, alerts</p>
              </div>
              <button className="flex items-center gap-2 px-4 py-2 bg-gradient-to-r from-amber-500 to-orange-500 text-zinc-950 rounded-lg font-medium">
                <Plus className="w-4 h-4" />Add Model
              </button>
            </div>
            <div className="grid grid-cols-2 gap-6">
              {wantedBoard.map((item) => (
                <div key={item.model} className="bg-zinc-900/50 rounded-2xl border border-zinc-800/50 p-6 hover:border-amber-500/30 transition-all">
                  <div className="flex items-start justify-between mb-4">
                    <div className="flex items-center gap-4">
                      <div className={`w-12 h-12 rounded-xl flex items-center justify-center text-lg font-bold ${
                        item.priority === 1 ? 'bg-gradient-to-br from-amber-500 to-orange-500 text-zinc-950' :
                        item.priority === 2 ? 'bg-gradient-to-br from-zinc-400 to-zinc-500 text-zinc-950' :
                        'bg-zinc-800 text-zinc-400'
                      }`}>#{item.priority}</div>
                      <div>
                        <h3 className="font-semibold text-lg">{item.model}</h3>
                        <p className="text-sm text-zinc-500">Priority {item.priority}</p>
                      </div>
                    </div>
                    <button className={`p-2 rounded-lg ${item.alerts ? 'bg-amber-500/20 text-amber-400' : 'bg-zinc-800 text-zinc-500'}`}>
                      <Bell className="w-5 h-5" />
                    </button>
                  </div>
                  <div className="grid grid-cols-2 gap-4">
                    <div className="bg-zinc-800/50 rounded-lg p-4">
                      <div className="text-xs text-zinc-500 uppercase mb-1">Max Bid</div>
                      <div className="text-xl font-bold text-amber-400">${item.maxBid.toLocaleString()}</div>
                    </div>
                    <div className="bg-zinc-800/50 rounded-lg p-4">
                      <div className="text-xs text-zinc-500 uppercase mb-1">Target Margin</div>
                      <div className="text-xl font-bold text-emerald-400">{item.targetMargin}%</div>
                    </div>
                  </div>
                </div>
              ))}
            </div>
          </div>
        )}
      </main>

      {/* Footer */}
      <footer className="fixed bottom-0 left-0 right-0 z-50 border-t border-zinc-800/50 backdrop-blur-xl bg-zinc-950/90">
        <div className="max-w-7xl mx-auto px-6 py-3">
          <div className="flex items-center justify-between text-xs">
            <div className="flex items-center gap-2">
              <div className="w-2 h-2 bg-emerald-400 rounded-full animate-pulse" />
              <span className="text-zinc-500">Live • 347 sources</span>
            </div>
            <div className="text-zinc-600">Integer-Only Scoring v2.1 • No black-box guessing</div>
          </div>
        </div>
      </footer>
    </div>
  );
}

function KPICard({ title, value, subtitle, trend, icon: Icon, color }) {
  const colors = {
    amber: 'from-amber-500 to-orange-500',
    emerald: 'from-emerald-500 to-teal-500',
    orange: 'from-orange-500 to-red-500',
    red: 'from-red-500 to-rose-500',
  };
  return (
    <div className="bg-zinc-900/50 rounded-2xl border border-zinc-800/50 p-6">
      <div className="flex items-start justify-between mb-4">
        <div className={`w-12 h-12 rounded-xl bg-gradient-to-br ${colors[color]} flex items-center justify-center shadow-lg`}>
          <Icon className="w-6 h-6 text-white" />
        </div>
        {trend !== 0 && (
          <div className={`flex items-center gap-1 text-sm ${trend > 0 ? 'text-emerald-400' : 'text-red-400'}`}>
            {trend > 0 ? <TrendingUp className="w-4 h-4" /> : <TrendingDown className="w-4 h-4" />}
            <span>{trend > 0 ? '+' : ''}{trend}%</span>
          </div>
        )}
      </div>
      <div className="text-3xl font-bold mb-1">{value}</div>
      <div className="text-sm text-zinc-500">{subtitle}</div>
    </div>
  );
}
