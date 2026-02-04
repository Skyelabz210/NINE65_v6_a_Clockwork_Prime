import React, { useState, useEffect } from 'react';
import { LineChart, Line, XAxis, YAxis, Tooltip, ResponsiveContainer, AreaChart, Area, BarChart, Bar, Cell, PieChart, Pie } from 'recharts';
import { Search, TrendingUp, TrendingDown, AlertCircle, CheckCircle, Clock, Wrench, Car, DollarSign, MapPin, Flame, Target, Filter, Bell, Settings, ChevronRight, ChevronDown, Star, Zap, Shield, Activity, BarChart3, PieChart as PieChartIcon, ArrowUpRight, ArrowDownRight, Plus, X, Eye, Phone, Mail } from 'lucide-react';

// ═══════════════════════════════════════════════════════════════════════════════
// EXOTIC ACQUISITION RADAR — San Antonio HQ
// Deterministic, Integer-Only Scoring System
// ═══════════════════════════════════════════════════════════════════════════════

// Sample Data - Live Opportunities
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
  { model: 'Aventador SVJ', trend: 'appreciating', change: 5.5, volume: 12, avgDays: 25 },
  { model: '720S', trend: 'softening', change: -3.8, volume: 38, avgDays: 52 },
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
  { model: 'McLaren 720S/765LT', priority: 5, maxBid: 95000, targetMargin: 35, alerts: false },
  { model: 'Roma/Roma Spider', priority: 6, maxBid: 75000, targetMargin: 30, alerts: false },
];

// Scoring calculation (integer-only, deterministic)
const calculateScore = (type, data) => {
  if (type === 'build') {
    // BUILD_SCORE = (4 × MARGIN_POTENTIAL) + (2 × PARTS) + PLATFORM - (3 × COMPLEXITY) - RISK
    const marginPotential = Math.floor((data.margin / 100) * 1000);
    const parts = data.damage === 'Rear end' ? 850 : data.damage === 'Mechanical' ? 900 : 650;
    const platform = data.vehicle.includes('GT-R') ? 950 : data.vehicle.includes('Huracán') ? 850 : 750;
    const complexity = data.damage === 'All over' ? 850 : data.damage === 'Flood' ? 700 : 350;
    const risk = data.location.includes('TX') ? 200 : 400;
    return (4 * marginPotential) + (2 * parts) + platform - (3 * complexity) - risk;
  } else {
    // TURNKEY_SCORE = (3 × MARGIN) + (2 × DEMAND) + TURN - (2 × RISK) - FRICTION
    const margin = Math.floor(Math.abs(data.delta) * 100);
    const demand = data.vehicle.includes('GT3') ? 950 : data.vehicle.includes('Huracán') ? 800 : 750;
    const turn = Math.max(0, 1000 - data.daysListed * 20);
    const risk = 200;
    const friction = data.source === 'Private' ? 150 : data.source === 'BaT' ? 250 : 400;
    return (3 * margin) + (2 * demand) + turn - (2 * risk) - friction;
  }
};

// Main App Component
export default function ExoticAcquisitionRadar() {
  const [activeTab, setActiveTab] = useState('dashboard');
  const [selectedPipeline, setSelectedPipeline] = useState('all');
  const [showScoreBreakdown, setShowScoreBreakdown] = useState(null);
  const [notifications, setNotifications] = useState(3);
  const [searchQuery, setSearchQuery] = useState('');

  // Animation on mount
  const [mounted, setMounted] = useState(false);
  useEffect(() => {
    setMounted(true);
  }, []);

  const totalBuildValue = buildReadyOpportunities.reduce((sum, o) => sum + (o.builtValue - o.currentBid - o.estBuildCost), 0);
  const totalTurnkeyValue = turnkeyOpportunities.reduce((sum, o) => sum + (o.compMedian - o.price), 0);
  const avgBuildMargin = Math.round(buildReadyOpportunities.reduce((sum, o) => sum + o.margin, 0) / buildReadyOpportunities.length);
  const avgTurnkeyDelta = Math.round(turnkeyOpportunities.reduce((sum, o) => sum + Math.abs(o.delta), 0) / turnkeyOpportunities.length * 10) / 10;

  return (
    <div className="min-h-screen bg-zinc-950 text-zinc-100 font-sans overflow-x-hidden">
      {/* Atmospheric Background */}
      <div className="fixed inset-0 pointer-events-none">
        <div className="absolute inset-0 bg-gradient-to-br from-zinc-950 via-zinc-900 to-zinc-950" />
        <div className="absolute top-0 left-0 w-full h-full opacity-30">
          <div className="absolute top-20 left-20 w-96 h-96 bg-amber-500/10 rounded-full blur-3xl" />
          <div className="absolute bottom-40 right-20 w-80 h-80 bg-orange-500/8 rounded-full blur-3xl" />
        </div>
        {/* Grid overlay */}
        <div className="absolute inset-0 opacity-5" style={{
          backgroundImage: `linear-gradient(rgba(251,191,36,0.3) 1px, transparent 1px), linear-gradient(90deg, rgba(251,191,36,0.3) 1px, transparent 1px)`,
          backgroundSize: '60px 60px'
        }} />
      </div>

      {/* Header */}
      <header className={`relative z-50 border-b border-zinc-800/50 backdrop-blur-xl bg-zinc-950/80 transition-all duration-700 ${mounted ? 'opacity-100 translate-y-0' : 'opacity-0 -translate-y-4'}`}>
        <div className="max-w-screen-2xl mx-auto px-6 py-4">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-4">
              <div className="relative">
                <div className="w-12 h-12 bg-gradient-to-br from-amber-400 to-orange-600 rounded-lg flex items-center justify-center shadow-lg shadow-amber-500/20">
                  <Target className="w-6 h-6 text-zinc-950" strokeWidth={2.5} />
                </div>
                <div className="absolute -top-1 -right-1 w-3 h-3 bg-emerald-400 rounded-full animate-pulse" />
              </div>
              <div>
                <h1 className="text-xl font-bold tracking-tight bg-gradient-to-r from-amber-200 via-amber-400 to-orange-400 bg-clip-text text-transparent">
                  EXOTIC ACQUISITION RADAR
                </h1>
                <p className="text-xs text-zinc-500 tracking-widest uppercase">San Antonio HQ • Deterministic Scoring</p>
              </div>
            </div>

            <nav className="flex items-center gap-1 bg-zinc-900/50 rounded-full p-1 border border-zinc-800/50">
              {[
                { id: 'dashboard', label: 'Dashboard', icon: Activity },
                { id: 'build', label: 'Build Pipeline', icon: Wrench },
                { id: 'turnkey', label: 'Turnkey', icon: Car },
                { id: 'wanted', label: 'Wanted Board', icon: Target },
                { id: 'analytics', label: 'Analytics', icon: BarChart3 },
              ].map(tab => (
                <button
                  key={tab.id}
                  onClick={() => setActiveTab(tab.id)}
                  className={`flex items-center gap-2 px-4 py-2 rounded-full text-sm font-medium transition-all duration-300 ${
                    activeTab === tab.id 
                      ? 'bg-gradient-to-r from-amber-500 to-orange-500 text-zinc-950 shadow-lg shadow-amber-500/25' 
                      : 'text-zinc-400 hover:text-zinc-200 hover:bg-zinc-800/50'
                  }`}
                >
                  <tab.icon className="w-4 h-4" />
                  <span className="hidden md:inline">{tab.label}</span>
                </button>
              ))}
            </nav>

            <div className="flex items-center gap-3">
              <div className="relative">
                <input
                  type="text"
                  placeholder="Search opportunities..."
                  value={searchQuery}
                  onChange={(e) => setSearchQuery(e.target.value)}
                  className="w-64 bg-zinc-900/50 border border-zinc-800 rounded-lg px-4 py-2 text-sm placeholder-zinc-600 focus:outline-none focus:border-amber-500/50 focus:ring-1 focus:ring-amber-500/25 transition-all"
                />
                <Search className="absolute right-3 top-1/2 -translate-y-1/2 w-4 h-4 text-zinc-600" />
              </div>
              <button className="relative p-2 rounded-lg bg-zinc-900/50 border border-zinc-800 hover:border-amber-500/30 transition-all">
                <Bell className="w-5 h-5 text-zinc-400" />
                {notifications > 0 && (
                  <span className="absolute -top-1 -right-1 w-5 h-5 bg-amber-500 rounded-full text-xs font-bold text-zinc-950 flex items-center justify-center">
                    {notifications}
                  </span>
                )}
              </button>
              <button className="p-2 rounded-lg bg-zinc-900/50 border border-zinc-800 hover:border-amber-500/30 transition-all">
                <Settings className="w-5 h-5 text-zinc-400" />
              </button>
            </div>
          </div>
        </div>
      </header>

      {/* Main Content */}
      <main className="relative z-10 max-w-screen-2xl mx-auto px-6 py-8">
        {activeTab === 'dashboard' && (
          <DashboardView 
            buildOpps={buildReadyOpportunities}
            turnkeyOpps={turnkeyOpportunities}
            heatData={marketHeatData}
            historyData={pipelineHistory}
            totalBuildValue={totalBuildValue}
            totalTurnkeyValue={totalTurnkeyValue}
            avgBuildMargin={avgBuildMargin}
            avgTurnkeyDelta={avgTurnkeyDelta}
            mounted={mounted}
          />
        )}
        {activeTab === 'build' && (
          <BuildPipelineView 
            opportunities={buildReadyOpportunities}
            mounted={mounted}
          />
        )}
        {activeTab === 'turnkey' && (
          <TurnkeyPipelineView 
            opportunities={turnkeyOpportunities}
            mounted={mounted}
          />
        )}
        {activeTab === 'wanted' && (
          <WantedBoardView 
            wantedBoard={wantedBoard}
            mounted={mounted}
          />
        )}
        {activeTab === 'analytics' && (
          <AnalyticsView 
            heatData={marketHeatData}
            historyData={pipelineHistory}
            mounted={mounted}
          />
        )}
      </main>

      {/* Footer Status Bar */}
      <footer className="fixed bottom-0 left-0 right-0 z-50 border-t border-zinc-800/50 backdrop-blur-xl bg-zinc-950/90">
        <div className="max-w-screen-2xl mx-auto px-6 py-3">
          <div className="flex items-center justify-between text-xs">
            <div className="flex items-center gap-6">
              <div className="flex items-center gap-2">
                <div className="w-2 h-2 bg-emerald-400 rounded-full animate-pulse" />
                <span className="text-zinc-500">Live • 347 sources monitored</span>
              </div>
              <div className="text-zinc-600">Last scan: 2 min ago</div>
            </div>
            <div className="flex items-center gap-6">
              <div className="flex items-center gap-2 text-zinc-500">
                <Zap className="w-3 h-3 text-amber-400" />
                <span>{buildReadyOpportunities.length + turnkeyOpportunities.length} active opportunities</span>
              </div>
              <div className="text-zinc-600">Integer-Only Scoring v2.1</div>
            </div>
          </div>
        </div>
      </footer>
    </div>
  );
}

// ═══════════════════════════════════════════════════════════════════════════════
// DASHBOARD VIEW
// ═══════════════════════════════════════════════════════════════════════════════

function DashboardView({ buildOpps, turnkeyOpps, heatData, historyData, totalBuildValue, totalTurnkeyValue, avgBuildMargin, avgTurnkeyDelta, mounted }) {
  return (
    <div className="space-y-6 pb-20">
      {/* KPI Cards */}
      <div className={`grid grid-cols-4 gap-4 transition-all duration-700 delay-100 ${mounted ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-8'}`}>
        <KPICard 
          title="Build Pipeline Value"
          value={`$${(totalBuildValue / 1000).toFixed(0)}K`}
          subtitle="Potential margin"
          trend={12.4}
          icon={Wrench}
          color="amber"
        />
        <KPICard 
          title="Turnkey Opportunities"
          value={`$${(totalTurnkeyValue / 1000).toFixed(0)}K`}
          subtitle="Below market delta"
          trend={8.2}
          icon={Car}
          color="emerald"
        />
        <KPICard 
          title="Avg Build Margin"
          value={`${avgBuildMargin}%`}
          subtitle="Across active builds"
          trend={3.5}
          icon={TrendingUp}
          color="orange"
        />
        <KPICard 
          title="Avg Price Delta"
          value={`-${avgTurnkeyDelta}%`}
          subtitle="Below comp median"
          trend={1.8}
          icon={DollarSign}
          color="cyan"
        />
      </div>

      {/* Main Grid */}
      <div className="grid grid-cols-3 gap-6">
        {/* Immediate Action */}
        <div className={`col-span-2 transition-all duration-700 delay-200 ${mounted ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-8'}`}>
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
                {buildOpps.filter(o => o.score >= 2000).length + turnkeyOpps.filter(o => o.score >= 1800).length} urgent
              </span>
            </div>
            <div className="divide-y divide-zinc-800/30">
              {buildOpps.filter(o => o.score >= 2000).slice(0, 3).map((opp, idx) => (
                <OpportunityRow key={opp.id} opp={opp} type="build" index={idx} />
              ))}
              {turnkeyOpps.filter(o => o.score >= 1800).slice(0, 2).map((opp, idx) => (
                <OpportunityRow key={opp.id} opp={opp} type="turnkey" index={idx + 3} />
              ))}
            </div>
          </div>
        </div>

        {/* Market Heat Map */}
        <div className={`transition-all duration-700 delay-300 ${mounted ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-8'}`}>
          <div className="bg-zinc-900/50 rounded-2xl border border-zinc-800/50 p-6 h-full">
            <div className="flex items-center gap-3 mb-6">
              <div className="w-10 h-10 bg-gradient-to-br from-violet-500 to-purple-600 rounded-lg flex items-center justify-center">
                <Activity className="w-5 h-5 text-white" />
              </div>
              <div>
                <h2 className="font-semibold text-lg">Market Heat</h2>
                <p className="text-xs text-zinc-500">7-day trend analysis</p>
              </div>
            </div>
            <div className="space-y-3">
              {heatData.slice(0, 6).map((model, idx) => (
                <div key={model.model} className="flex items-center justify-between p-3 bg-zinc-800/30 rounded-lg hover:bg-zinc-800/50 transition-all cursor-pointer">
                  <div className="flex items-center gap-3">
                    <div className={`w-2 h-2 rounded-full ${
                      model.trend === 'tightening' || model.trend === 'appreciating' ? 'bg-emerald-400' :
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
                    {model.change > 0 ? (
                      <ArrowUpRight className="w-4 h-4 text-emerald-400" />
                    ) : model.change < 0 ? (
                      <ArrowDownRight className="w-4 h-4 text-red-400" />
                    ) : (
                      <span className="w-4 h-4" />
                    )}
                  </div>
                </div>
              ))}
            </div>
          </div>
        </div>
      </div>

      {/* Pipeline Performance Chart */}
      <div className={`transition-all duration-700 delay-400 ${mounted ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-8'}`}>
        <div className="bg-zinc-900/50 rounded-2xl border border-zinc-800/50 p-6">
          <div className="flex items-center justify-between mb-6">
            <div className="flex items-center gap-3">
              <div className="w-10 h-10 bg-gradient-to-br from-cyan-500 to-blue-600 rounded-lg flex items-center justify-center">
                <BarChart3 className="w-5 h-5 text-white" />
              </div>
              <div>
                <h2 className="font-semibold text-lg">Pipeline Performance</h2>
                <p className="text-xs text-zinc-500">6-month acquisition + margin history</p>
              </div>
            </div>
            <div className="flex items-center gap-4 text-sm">
              <div className="flex items-center gap-2">
                <div className="w-3 h-3 bg-amber-400 rounded" />
                <span className="text-zinc-400">Build Pipeline</span>
              </div>
              <div className="flex items-center gap-2">
                <div className="w-3 h-3 bg-emerald-400 rounded" />
                <span className="text-zinc-400">Turnkey</span>
              </div>
              <div className="flex items-center gap-2">
                <div className="w-3 h-3 bg-gradient-to-r from-amber-400 to-emerald-400 rounded" />
                <span className="text-zinc-400">Total Margin</span>
              </div>
            </div>
          </div>
          <div className="h-64">
            <ResponsiveContainer width="100%" height="100%">
              <AreaChart data={historyData}>
                <defs>
                  <linearGradient id="marginGradient" x1="0" y1="0" x2="0" y2="1">
                    <stop offset="5%" stopColor="#f59e0b" stopOpacity={0.3}/>
                    <stop offset="95%" stopColor="#f59e0b" stopOpacity={0}/>
                  </linearGradient>
                </defs>
                <XAxis dataKey="month" axisLine={false} tickLine={false} tick={{ fill: '#71717a', fontSize: 12 }} />
                <YAxis axisLine={false} tickLine={false} tick={{ fill: '#71717a', fontSize: 12 }} tickFormatter={(v) => `$${v/1000}K`} />
                <Tooltip 
                  contentStyle={{ 
                    backgroundColor: '#18181b', 
                    border: '1px solid #3f3f46',
                    borderRadius: '8px',
                    boxShadow: '0 4px 20px rgba(0,0,0,0.5)'
                  }}
                  labelStyle={{ color: '#a1a1aa' }}
                />
                <Area type="monotone" dataKey="margin" stroke="#f59e0b" fill="url(#marginGradient)" strokeWidth={2} />
                <Line type="monotone" dataKey="build" stroke="#f59e0b" strokeWidth={2} dot={{ fill: '#f59e0b', strokeWidth: 0, r: 4 }} />
                <Line type="monotone" dataKey="turnkey" stroke="#34d399" strokeWidth={2} dot={{ fill: '#34d399', strokeWidth: 0, r: 4 }} />
              </AreaChart>
            </ResponsiveContainer>
          </div>
        </div>
      </div>

      {/* Recent Activity */}
      <div className="grid grid-cols-2 gap-6">
        <div className={`transition-all duration-700 delay-500 ${mounted ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-8'}`}>
          <div className="bg-zinc-900/50 rounded-2xl border border-zinc-800/50 p-6">
            <h3 className="font-semibold mb-4 flex items-center gap-2">
              <Clock className="w-4 h-4 text-amber-400" />
              Recent Activity
            </h3>
            <div className="space-y-3">
              {[
                { action: 'New opportunity', detail: '2024 Purosangue salvage in Chicago', time: '2 min ago', type: 'new' },
                { action: 'Price drop', detail: 'GT3 Touring reduced by $15K', time: '18 min ago', type: 'alert' },
                { action: 'Bid placed', detail: 'Huracán Tecnica @ $51,500', time: '1 hour ago', type: 'action' },
                { action: 'Watchlist match', detail: 'GT-R Nismo flood in Houston', time: '3 hours ago', type: 'match' },
              ].map((activity, idx) => (
                <div key={idx} className="flex items-center justify-between p-3 bg-zinc-800/30 rounded-lg">
                  <div className="flex items-center gap-3">
                    <div className={`w-8 h-8 rounded-lg flex items-center justify-center ${
                      activity.type === 'new' ? 'bg-emerald-500/20' :
                      activity.type === 'alert' ? 'bg-amber-500/20' :
                      activity.type === 'action' ? 'bg-blue-500/20' : 'bg-violet-500/20'
                    }`}>
                      {activity.type === 'new' ? <Plus className="w-4 h-4 text-emerald-400" /> :
                       activity.type === 'alert' ? <Bell className="w-4 h-4 text-amber-400" /> :
                       activity.type === 'action' ? <Zap className="w-4 h-4 text-blue-400" /> :
                       <Target className="w-4 h-4 text-violet-400" />}
                    </div>
                    <div>
                      <p className="text-sm font-medium">{activity.action}</p>
                      <p className="text-xs text-zinc-500">{activity.detail}</p>
                    </div>
                  </div>
                  <span className="text-xs text-zinc-600">{activity.time}</span>
                </div>
              ))}
            </div>
          </div>
        </div>

        <div className={`transition-all duration-700 delay-600 ${mounted ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-8'}`}>
          <div className="bg-zinc-900/50 rounded-2xl border border-zinc-800/50 p-6">
            <h3 className="font-semibold mb-4 flex items-center gap-2">
              <Shield className="w-4 h-4 text-emerald-400" />
              Scoring Transparency
            </h3>
            <div className="space-y-4">
              <div className="p-4 bg-zinc-800/30 rounded-lg border border-amber-500/20">
                <div className="flex items-center justify-between mb-3">
                  <span className="text-sm font-medium text-amber-400">BUILD SCORE FORMULA</span>
                  <code className="text-xs text-zinc-500 bg-zinc-900 px-2 py-1 rounded">Integer-Only</code>
                </div>
                <code className="text-xs text-zinc-400 font-mono leading-relaxed">
                  S = (4 × MARGIN) + (2 × PARTS) + PLATFORM<br />
                  &nbsp;&nbsp;&nbsp;&nbsp;- (3 × COMPLEXITY) - RISK
                </code>
              </div>
              <div className="p-4 bg-zinc-800/30 rounded-lg border border-emerald-500/20">
                <div className="flex items-center justify-between mb-3">
                  <span className="text-sm font-medium text-emerald-400">TURNKEY SCORE FORMULA</span>
                  <code className="text-xs text-zinc-500 bg-zinc-900 px-2 py-1 rounded">Integer-Only</code>
                </div>
                <code className="text-xs text-zinc-400 font-mono leading-relaxed">
                  S = (3 × MARGIN) + (2 × DEMAND) + TURN<br />
                  &nbsp;&nbsp;&nbsp;&nbsp;- (2 × RISK) - FRICTION
                </code>
              </div>
              <p className="text-xs text-zinc-500 text-center">
                No black-box guessing. Every score is auditable.
              </p>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}

// ═══════════════════════════════════════════════════════════════════════════════
// BUILD PIPELINE VIEW
// ═══════════════════════════════════════════════════════════════════════════════

function BuildPipelineView({ opportunities, mounted }) {
  const [filter, setFilter] = useState('all');
  const [sortBy, setSortBy] = useState('score');

  const filtered = opportunities
    .filter(o => filter === 'all' || (filter === 'tx' && o.location.includes('TX')))
    .sort((a, b) => sortBy === 'score' ? b.score - a.score : b.margin - a.margin);

  return (
    <div className="space-y-6 pb-20">
      {/* Header */}
      <div className={`flex items-center justify-between transition-all duration-700 ${mounted ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-8'}`}>
        <div>
          <h1 className="text-2xl font-bold">Build-Ready Pipeline</h1>
          <p className="text-zinc-500">Salvage, shells, and rebuildable exotics • 25-50% margin potential</p>
        </div>
        <div className="flex items-center gap-3">
          <select 
            value={filter}
            onChange={(e) => setFilter(e.target.value)}
            className="bg-zinc-900 border border-zinc-800 rounded-lg px-4 py-2 text-sm focus:outline-none focus:border-amber-500/50"
          >
            <option value="all">All Locations</option>
            <option value="tx">Texas Only</option>
          </select>
          <select 
            value={sortBy}
            onChange={(e) => setSortBy(e.target.value)}
            className="bg-zinc-900 border border-zinc-800 rounded-lg px-4 py-2 text-sm focus:outline-none focus:border-amber-500/50"
          >
            <option value="score">Sort by Score</option>
            <option value="margin">Sort by Margin</option>
          </select>
        </div>
      </div>

      {/* Stats Row */}
      <div className={`grid grid-cols-4 gap-4 transition-all duration-700 delay-100 ${mounted ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-8'}`}>
        <div className="bg-zinc-900/50 rounded-xl border border-zinc-800/50 p-4">
          <div className="text-2xl font-bold text-amber-400">{filtered.length}</div>
          <div className="text-sm text-zinc-500">Active Opportunities</div>
        </div>
        <div className="bg-zinc-900/50 rounded-xl border border-zinc-800/50 p-4">
          <div className="text-2xl font-bold text-emerald-400">{filtered.filter(o => o.score >= 2000).length}</div>
          <div className="text-sm text-zinc-500">Immediate Action</div>
        </div>
        <div className="bg-zinc-900/50 rounded-xl border border-zinc-800/50 p-4">
          <div className="text-2xl font-bold text-orange-400">{Math.round(filtered.reduce((s,o) => s + o.margin, 0) / filtered.length)}%</div>
          <div className="text-sm text-zinc-500">Avg Margin</div>
        </div>
        <div className="bg-zinc-900/50 rounded-xl border border-zinc-800/50 p-4">
          <div className="text-2xl font-bold text-cyan-400">${(filtered.reduce((s,o) => s + (o.builtValue - o.currentBid - o.estBuildCost), 0) / 1000).toFixed(0)}K</div>
          <div className="text-sm text-zinc-500">Total Potential</div>
        </div>
      </div>

      {/* Opportunities Table */}
      <div className={`bg-zinc-900/50 rounded-2xl border border-zinc-800/50 overflow-hidden transition-all duration-700 delay-200 ${mounted ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-8'}`}>
        <div className="overflow-x-auto">
          <table className="w-full">
            <thead>
              <tr className="border-b border-zinc-800/50">
                <th className="text-left px-6 py-4 text-xs font-medium text-zinc-500 uppercase tracking-wider">Vehicle</th>
                <th className="text-left px-6 py-4 text-xs font-medium text-zinc-500 uppercase tracking-wider">Location</th>
                <th className="text-left px-6 py-4 text-xs font-medium text-zinc-500 uppercase tracking-wider">Damage</th>
                <th className="text-right px-6 py-4 text-xs font-medium text-zinc-500 uppercase tracking-wider">Current Bid</th>
                <th className="text-right px-6 py-4 text-xs font-medium text-zinc-500 uppercase tracking-wider">Build Cost</th>
                <th className="text-right px-6 py-4 text-xs font-medium text-zinc-500 uppercase tracking-wider">Built Value</th>
                <th className="text-right px-6 py-4 text-xs font-medium text-zinc-500 uppercase tracking-wider">Margin</th>
                <th className="text-right px-6 py-4 text-xs font-medium text-zinc-500 uppercase tracking-wider">Score</th>
                <th className="text-center px-6 py-4 text-xs font-medium text-zinc-500 uppercase tracking-wider">Action</th>
              </tr>
            </thead>
            <tbody className="divide-y divide-zinc-800/30">
              {filtered.map((opp, idx) => (
                <tr key={opp.id} className="hover:bg-zinc-800/30 transition-all group">
                  <td className="px-6 py-4">
                    <div className="flex items-center gap-3">
                      <div className={`w-10 h-10 rounded-lg flex items-center justify-center ${
                        opp.score >= 2500 ? 'bg-red-500/20' :
                        opp.score >= 2000 ? 'bg-amber-500/20' : 'bg-zinc-800'
                      }`}>
                        <Wrench className={`w-5 h-5 ${
                          opp.score >= 2500 ? 'text-red-400' :
                          opp.score >= 2000 ? 'text-amber-400' : 'text-zinc-500'
                        }`} />
                      </div>
                      <div>
                        <div className="font-medium">{opp.vehicle}</div>
                        <div className="text-xs text-zinc-500">{opp.miles.toLocaleString()} mi • {opp.platform}</div>
                      </div>
                    </div>
                  </td>
                  <td className="px-6 py-4">
                    <div className="flex items-center gap-2">
                      <MapPin className="w-4 h-4 text-zinc-500" />
                      <span className={opp.location.includes('TX') ? 'text-emerald-400' : 'text-zinc-400'}>
                        {opp.location}
                      </span>
                    </div>
                  </td>
                  <td className="px-6 py-4">
                    <span className={`px-2 py-1 rounded text-xs font-medium ${
                      opp.damage === 'Rear end' ? 'bg-emerald-500/20 text-emerald-400' :
                      opp.damage === 'Flood' ? 'bg-blue-500/20 text-blue-400' :
                      'bg-amber-500/20 text-amber-400'
                    }`}>
                      {opp.damage}
                    </span>
                  </td>
                  <td className="px-6 py-4 text-right font-mono">${opp.currentBid.toLocaleString()}</td>
                  <td className="px-6 py-4 text-right font-mono text-zinc-500">${opp.estBuildCost.toLocaleString()}</td>
                  <td className="px-6 py-4 text-right font-mono text-emerald-400">${opp.builtValue.toLocaleString()}</td>
                  <td className="px-6 py-4 text-right">
                    <span className={`font-bold ${opp.margin >= 40 ? 'text-emerald-400' : 'text-amber-400'}`}>
                      {opp.margin}%
                    </span>
                  </td>
                  <td className="px-6 py-4 text-right">
                    <div className={`inline-flex items-center gap-1 px-3 py-1 rounded-full font-mono font-bold ${
                      opp.score >= 2500 ? 'bg-red-500/20 text-red-400' :
                      opp.score >= 2000 ? 'bg-amber-500/20 text-amber-400' :
                      opp.score >= 1500 ? 'bg-emerald-500/20 text-emerald-400' : 'bg-zinc-800 text-zinc-400'
                    }`}>
                      {opp.score >= 2500 && <Flame className="w-3 h-3" />}
                      {opp.score}
                    </div>
                  </td>
                  <td className="px-6 py-4 text-center">
                    <button className={`px-4 py-2 rounded-lg text-sm font-medium transition-all ${
                      opp.score >= 2000 
                        ? 'bg-gradient-to-r from-amber-500 to-orange-500 text-zinc-950 hover:shadow-lg hover:shadow-amber-500/25' 
                        : 'bg-zinc-800 text-zinc-400 hover:bg-zinc-700'
                    }`}>
                      {opp.score >= 2500 ? 'BID NOW' : opp.score >= 2000 ? 'Inspect' : 'Watch'}
                    </button>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </div>

      {/* Build Economics Calculator */}
      <div className={`bg-zinc-900/50 rounded-2xl border border-zinc-800/50 p-6 transition-all duration-700 delay-300 ${mounted ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-8'}`}>
        <h3 className="text-lg font-semibold mb-4 flex items-center gap-2">
          <DollarSign className="w-5 h-5 text-amber-400" />
          Build Economics Reference
        </h3>
        <div className="grid grid-cols-3 gap-6">
          <div className="space-y-3">
            <h4 className="text-sm font-medium text-zinc-400">Rear Collision Repair</h4>
            <div className="space-y-2 text-sm">
              <div className="flex justify-between"><span className="text-zinc-500">Light (bumper)</span><span className="text-emerald-400">$8-15K</span></div>
              <div className="flex justify-between"><span className="text-zinc-500">Medium (quarter)</span><span className="text-amber-400">$15-30K</span></div>
              <div className="flex justify-between"><span className="text-zinc-500">Heavy (structural)</span><span className="text-orange-400">$30-60K</span></div>
            </div>
          </div>
          <div className="space-y-3">
            <h4 className="text-sm font-medium text-zinc-400">Fire Damage (Rear)</h4>
            <div className="space-y-2 text-sm">
              <div className="flex justify-between"><span className="text-zinc-500">Light (tail)</span><span className="text-emerald-400">$15-25K</span></div>
              <div className="flex justify-between"><span className="text-zinc-500">Medium (bay)</span><span className="text-amber-400">$25-50K</span></div>
              <div className="flex justify-between"><span className="text-zinc-500">Heavy (interior)</span><span className="text-orange-400">$50-80K</span></div>
            </div>
          </div>
          <div className="space-y-3">
            <h4 className="text-sm font-medium text-zinc-400">Twin-Turbo Conversion</h4>
            <div className="space-y-2 text-sm">
              <div className="flex justify-between"><span className="text-zinc-500">GT-R R35</span><span className="text-cyan-400">$15-30K</span></div>
              <div className="flex justify-between"><span className="text-zinc-500">Huracán</span><span className="text-cyan-400">$30-55K</span></div>
              <div className="flex justify-between"><span className="text-zinc-500">Gallardo</span><span className="text-cyan-400">$23-42K</span></div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}

// ═══════════════════════════════════════════════════════════════════════════════
// TURNKEY PIPELINE VIEW
// ═══════════════════════════════════════════════════════════════════════════════

function TurnkeyPipelineView({ opportunities, mounted }) {
  return (
    <div className="space-y-6 pb-20">
      <div className={`flex items-center justify-between transition-all duration-700 ${mounted ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-8'}`}>
        <div>
          <h1 className="text-2xl font-bold">Turnkey Pipeline</h1>
          <p className="text-zinc-500">Clean title, ready-to-retail exotics • 8-15% margin</p>
        </div>
      </div>

      <div className={`grid grid-cols-4 gap-4 transition-all duration-700 delay-100 ${mounted ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-8'}`}>
        <div className="bg-zinc-900/50 rounded-xl border border-zinc-800/50 p-4">
          <div className="text-2xl font-bold text-emerald-400">{opportunities.length}</div>
          <div className="text-sm text-zinc-500">Active Opportunities</div>
        </div>
        <div className="bg-zinc-900/50 rounded-xl border border-zinc-800/50 p-4">
          <div className="text-2xl font-bold text-amber-400">{opportunities.filter(o => o.status === 'call').length}</div>
          <div className="text-sm text-zinc-500">Call Now</div>
        </div>
        <div className="bg-zinc-900/50 rounded-xl border border-zinc-800/50 p-4">
          <div className="text-2xl font-bold text-orange-400">-{(opportunities.reduce((s,o) => s + Math.abs(o.delta), 0) / opportunities.length).toFixed(1)}%</div>
          <div className="text-sm text-zinc-500">Avg Below Market</div>
        </div>
        <div className="bg-zinc-900/50 rounded-xl border border-zinc-800/50 p-4">
          <div className="text-2xl font-bold text-cyan-400">${(opportunities.reduce((s,o) => s + (o.compMedian - o.price), 0) / 1000).toFixed(0)}K</div>
          <div className="text-sm text-zinc-500">Total Delta</div>
        </div>
      </div>

      <div className={`bg-zinc-900/50 rounded-2xl border border-zinc-800/50 overflow-hidden transition-all duration-700 delay-200 ${mounted ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-8'}`}>
        <div className="overflow-x-auto">
          <table className="w-full">
            <thead>
              <tr className="border-b border-zinc-800/50">
                <th className="text-left px-6 py-4 text-xs font-medium text-zinc-500 uppercase tracking-wider">Vehicle</th>
                <th className="text-left px-6 py-4 text-xs font-medium text-zinc-500 uppercase tracking-wider">Location</th>
                <th className="text-left px-6 py-4 text-xs font-medium text-zinc-500 uppercase tracking-wider">Spec</th>
                <th className="text-right px-6 py-4 text-xs font-medium text-zinc-500 uppercase tracking-wider">Price</th>
                <th className="text-right px-6 py-4 text-xs font-medium text-zinc-500 uppercase tracking-wider">Comp Median</th>
                <th className="text-right px-6 py-4 text-xs font-medium text-zinc-500 uppercase tracking-wider">Delta</th>
                <th className="text-right px-6 py-4 text-xs font-medium text-zinc-500 uppercase tracking-wider">Score</th>
                <th className="text-center px-6 py-4 text-xs font-medium text-zinc-500 uppercase tracking-wider">Action</th>
              </tr>
            </thead>
            <tbody className="divide-y divide-zinc-800/30">
              {opportunities.map((opp) => (
                <tr key={opp.id} className="hover:bg-zinc-800/30 transition-all">
                  <td className="px-6 py-4">
                    <div className="flex items-center gap-3">
                      <div className={`w-10 h-10 rounded-lg flex items-center justify-center ${
                        opp.status === 'call' ? 'bg-emerald-500/20' : 'bg-zinc-800'
                      }`}>
                        <Car className={`w-5 h-5 ${opp.status === 'call' ? 'text-emerald-400' : 'text-zinc-500'}`} />
                      </div>
                      <div>
                        <div className="font-medium">{opp.vehicle}</div>
                        <div className="text-xs text-zinc-500">{opp.miles.toLocaleString()} mi • {opp.source}</div>
                      </div>
                    </div>
                  </td>
                  <td className="px-6 py-4">
                    <div className="flex items-center gap-2">
                      <MapPin className="w-4 h-4 text-zinc-500" />
                      <span className="text-zinc-400">{opp.location}</span>
                    </div>
                  </td>
                  <td className="px-6 py-4 text-sm text-zinc-400 max-w-48 truncate">{opp.spec}</td>
                  <td className="px-6 py-4 text-right font-mono">${opp.price.toLocaleString()}</td>
                  <td className="px-6 py-4 text-right font-mono text-zinc-500">${opp.compMedian.toLocaleString()}</td>
                  <td className="px-6 py-4 text-right">
                    <span className="text-emerald-400 font-bold">{opp.delta}%</span>
                  </td>
                  <td className="px-6 py-4 text-right">
                    <span className={`px-3 py-1 rounded-full font-mono font-bold ${
                      opp.score >= 1800 ? 'bg-emerald-500/20 text-emerald-400' :
                      opp.score >= 1500 ? 'bg-amber-500/20 text-amber-400' : 'bg-zinc-800 text-zinc-400'
                    }`}>
                      {opp.score}
                    </span>
                  </td>
                  <td className="px-6 py-4 text-center">
                    <div className="flex items-center justify-center gap-2">
                      <button className="p-2 rounded-lg bg-emerald-500/20 text-emerald-400 hover:bg-emerald-500/30 transition-all">
                        <Phone className="w-4 h-4" />
                      </button>
                      <button className="p-2 rounded-lg bg-zinc-800 text-zinc-400 hover:bg-zinc-700 transition-all">
                        <Eye className="w-4 h-4" />
                      </button>
                    </div>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </div>
    </div>
  );
}

// ═══════════════════════════════════════════════════════════════════════════════
// WANTED BOARD VIEW
// ═══════════════════════════════════════════════════════════════════════════════

function WantedBoardView({ wantedBoard, mounted }) {
  return (
    <div className="space-y-6 pb-20">
      <div className={`flex items-center justify-between transition-all duration-700 ${mounted ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-8'}`}>
        <div>
          <h1 className="text-2xl font-bold">Wanted Board</h1>
          <p className="text-zinc-500">Configure target models, max bids, and alert preferences</p>
        </div>
        <button className="flex items-center gap-2 px-4 py-2 bg-gradient-to-r from-amber-500 to-orange-500 text-zinc-950 rounded-lg font-medium hover:shadow-lg hover:shadow-amber-500/25 transition-all">
          <Plus className="w-4 h-4" />
          Add Model
        </button>
      </div>

      <div className={`grid grid-cols-2 gap-6 transition-all duration-700 delay-100 ${mounted ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-8'}`}>
        {wantedBoard.map((item, idx) => (
          <div key={idx} className="bg-zinc-900/50 rounded-2xl border border-zinc-800/50 p-6 hover:border-amber-500/30 transition-all">
            <div className="flex items-start justify-between mb-4">
              <div className="flex items-center gap-4">
                <div className={`w-12 h-12 rounded-xl flex items-center justify-center text-lg font-bold ${
                  item.priority === 1 ? 'bg-gradient-to-br from-amber-500 to-orange-500 text-zinc-950' :
                  item.priority === 2 ? 'bg-gradient-to-br from-zinc-400 to-zinc-500 text-zinc-950' :
                  item.priority === 3 ? 'bg-gradient-to-br from-amber-700 to-amber-800 text-zinc-100' :
                  'bg-zinc-800 text-zinc-400'
                }`}>
                  #{item.priority}
                </div>
                <div>
                  <h3 className="font-semibold text-lg">{item.model}</h3>
                  <p className="text-sm text-zinc-500">Priority {item.priority}</p>
                </div>
              </div>
              <button className={`p-2 rounded-lg transition-all ${
                item.alerts ? 'bg-amber-500/20 text-amber-400' : 'bg-zinc-800 text-zinc-500'
              }`}>
                <Bell className="w-5 h-5" />
              </button>
            </div>
            
            <div className="grid grid-cols-2 gap-4 mt-6">
              <div className="bg-zinc-800/50 rounded-lg p-4">
                <div className="text-xs text-zinc-500 uppercase tracking-wider mb-1">Max Bid</div>
                <div className="text-xl font-bold text-amber-400">${item.maxBid.toLocaleString()}</div>
              </div>
              <div className="bg-zinc-800/50 rounded-lg p-4">
                <div className="text-xs text-zinc-500 uppercase tracking-wider mb-1">Target Margin</div>
                <div className="text-xl font-bold text-emerald-400">{item.targetMargin}%</div>
              </div>
            </div>

            <div className="flex items-center gap-2 mt-4 pt-4 border-t border-zinc-800/50">
              <button className="flex-1 py-2 bg-zinc-800 rounded-lg text-sm text-zinc-400 hover:bg-zinc-700 transition-all">
                Edit
              </button>
              <button className="flex-1 py-2 bg-zinc-800 rounded-lg text-sm text-zinc-400 hover:bg-zinc-700 transition-all">
                View Matches
              </button>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}

// ═══════════════════════════════════════════════════════════════════════════════
// ANALYTICS VIEW
// ═══════════════════════════════════════════════════════════════════════════════

function AnalyticsView({ heatData, historyData, mounted }) {
  const pieData = [
    { name: 'Build Pipeline', value: 45, color: '#f59e0b' },
    { name: 'Turnkey', value: 35, color: '#34d399' },
    { name: 'Wholesale', value: 20, color: '#60a5fa' },
  ];

  return (
    <div className="space-y-6 pb-20">
      <div className={`transition-all duration-700 ${mounted ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-8'}`}>
        <h1 className="text-2xl font-bold">Analytics</h1>
        <p className="text-zinc-500">Performance metrics and market intelligence</p>
      </div>

      <div className="grid grid-cols-3 gap-6">
        {/* Margin by Source */}
        <div className={`col-span-2 bg-zinc-900/50 rounded-2xl border border-zinc-800/50 p-6 transition-all duration-700 delay-100 ${mounted ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-8'}`}>
          <h3 className="font-semibold mb-6">Monthly Margin Performance</h3>
          <div className="h-72">
            <ResponsiveContainer width="100%" height="100%">
              <BarChart data={historyData}>
                <XAxis dataKey="month" axisLine={false} tickLine={false} tick={{ fill: '#71717a', fontSize: 12 }} />
                <YAxis axisLine={false} tickLine={false} tick={{ fill: '#71717a', fontSize: 12 }} tickFormatter={(v) => `$${v/1000}K`} />
                <Tooltip 
                  contentStyle={{ 
                    backgroundColor: '#18181b', 
                    border: '1px solid #3f3f46',
                    borderRadius: '8px'
                  }}
                />
                <Bar dataKey="margin" radius={[4, 4, 0, 0]}>
                  {historyData.map((entry, index) => (
                    <Cell key={`cell-${index}`} fill={`url(#barGradient)`} />
                  ))}
                </Bar>
                <defs>
                  <linearGradient id="barGradient" x1="0" y1="0" x2="0" y2="1">
                    <stop offset="0%" stopColor="#f59e0b" />
                    <stop offset="100%" stopColor="#ea580c" />
                  </linearGradient>
                </defs>
              </BarChart>
            </ResponsiveContainer>
          </div>
        </div>

        {/* Pipeline Split */}
        <div className={`bg-zinc-900/50 rounded-2xl border border-zinc-800/50 p-6 transition-all duration-700 delay-200 ${mounted ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-8'}`}>
          <h3 className="font-semibold mb-6">Pipeline Distribution</h3>
          <div className="h-48">
            <ResponsiveContainer width="100%" height="100%">
              <PieChart>
                <Pie
                  data={pieData}
                  cx="50%"
                  cy="50%"
                  innerRadius={50}
                  outerRadius={80}
                  paddingAngle={4}
                  dataKey="value"
                >
                  {pieData.map((entry, index) => (
                    <Cell key={`cell-${index}`} fill={entry.color} />
                  ))}
                </Pie>
              </PieChart>
            </ResponsiveContainer>
          </div>
          <div className="space-y-2 mt-4">
            {pieData.map((item, idx) => (
              <div key={idx} className="flex items-center justify-between text-sm">
                <div className="flex items-center gap-2">
                  <div className="w-3 h-3 rounded" style={{ backgroundColor: item.color }} />
                  <span className="text-zinc-400">{item.name}</span>
                </div>
                <span className="font-mono">{item.value}%</span>
              </div>
            ))}
          </div>
        </div>
      </div>

      {/* Model Performance Table */}
      <div className={`bg-zinc-900/50 rounded-2xl border border-zinc-800/50 p-6 transition-all duration-700 delay-300 ${mounted ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-8'}`}>
        <h3 className="font-semibold mb-6">Model Market Analysis</h3>
        <div className="overflow-x-auto">
          <table className="w-full">
            <thead>
              <tr className="border-b border-zinc-800/50">
                <th className="text-left px-4 py-3 text-xs font-medium text-zinc-500 uppercase">Model</th>
                <th className="text-center px-4 py-3 text-xs font-medium text-zinc-500 uppercase">Trend</th>
                <th className="text-right px-4 py-3 text-xs font-medium text-zinc-500 uppercase">7-Day Change</th>
                <th className="text-right px-4 py-3 text-xs font-medium text-zinc-500 uppercase">Volume</th>
                <th className="text-right px-4 py-3 text-xs font-medium text-zinc-500 uppercase">Avg Days</th>
              </tr>
            </thead>
            <tbody className="divide-y divide-zinc-800/30">
              {heatData.map((model, idx) => (
                <tr key={idx} className="hover:bg-zinc-800/30 transition-all">
                  <td className="px-4 py-3 font-medium">{model.model}</td>
                  <td className="px-4 py-3 text-center">
                    <span className={`px-3 py-1 rounded-full text-xs font-medium ${
                      model.trend === 'tightening' || model.trend === 'appreciating' ? 'bg-emerald-500/20 text-emerald-400' :
                      model.trend === 'softening' ? 'bg-red-500/20 text-red-400' : 'bg-zinc-800 text-zinc-400'
                    }`}>
                      {model.trend}
                    </span>
                  </td>
                  <td className={`px-4 py-3 text-right font-mono ${
                    model.change > 0 ? 'text-emerald-400' : model.change < 0 ? 'text-red-400' : 'text-zinc-500'
                  }`}>
                    {model.change > 0 ? '+' : ''}{model.change}%
                  </td>
                  <td className="px-4 py-3 text-right font-mono text-zinc-400">{model.volume}</td>
                  <td className="px-4 py-3 text-right font-mono text-zinc-400">{model.avgDays}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </div>
    </div>
  );
}

// ═══════════════════════════════════════════════════════════════════════════════
// SHARED COMPONENTS
// ═══════════════════════════════════════════════════════════════════════════════

function KPICard({ title, value, subtitle, trend, icon: Icon, color }) {
  const colorClasses = {
    amber: 'from-amber-500 to-orange-500 shadow-amber-500/20',
    emerald: 'from-emerald-500 to-teal-500 shadow-emerald-500/20',
    orange: 'from-orange-500 to-red-500 shadow-orange-500/20',
    cyan: 'from-cyan-500 to-blue-500 shadow-cyan-500/20',
  };

  return (
    <div className="bg-zinc-900/50 rounded-2xl border border-zinc-800/50 p-6 hover:border-zinc-700/50 transition-all">
      <div className="flex items-start justify-between mb-4">
        <div className={`w-12 h-12 rounded-xl bg-gradient-to-br ${colorClasses[color]} flex items-center justify-center shadow-lg`}>
          <Icon className="w-6 h-6 text-white" />
        </div>
        <div className={`flex items-center gap-1 text-sm ${trend > 0 ? 'text-emerald-400' : 'text-red-400'}`}>
          {trend > 0 ? <TrendingUp className="w-4 h-4" /> : <TrendingDown className="w-4 h-4" />}
          <span>{trend > 0 ? '+' : ''}{trend}%</span>
        </div>
      </div>
      <div className="text-3xl font-bold mb-1">{value}</div>
      <div className="text-sm text-zinc-500">{subtitle}</div>
    </div>
  );
}

function OpportunityRow({ opp, type, index }) {
  const isBuild = type === 'build';
  
  return (
    <div className="px-6 py-4 hover:bg-zinc-800/30 transition-all flex items-center justify-between group" style={{ animationDelay: `${index * 100}ms` }}>
      <div className="flex items-center gap-4">
        <div className={`w-12 h-12 rounded-xl flex items-center justify-center ${
          isBuild ? 'bg-amber-500/20' : 'bg-emerald-500/20'
        }`}>
          {isBuild ? (
            <Wrench className="w-6 h-6 text-amber-400" />
          ) : (
            <Car className="w-6 h-6 text-emerald-400" />
          )}
        </div>
        <div>
          <div className="font-semibold">{opp.vehicle}</div>
          <div className="text-sm text-zinc-500 flex items-center gap-2">
            <MapPin className="w-3 h-3" />
            {opp.location}
            <span className="text-zinc-700">•</span>
            {isBuild ? opp.platform : opp.source}
          </div>
        </div>
      </div>
      
      <div className="flex items-center gap-8">
        <div className="text-right">
          <div className="font-mono text-lg">
            ${isBuild ? opp.currentBid?.toLocaleString() : opp.price?.toLocaleString()}
          </div>
          <div className="text-sm text-zinc-500">
            {isBuild ? `→ $${opp.builtValue?.toLocaleString()} built` : `${opp.delta}% below market`}
          </div>
        </div>
        
        <div className={`px-4 py-2 rounded-xl font-mono font-bold ${
          opp.score >= 2500 ? 'bg-red-500/20 text-red-400' :
          opp.score >= 2000 ? 'bg-amber-500/20 text-amber-400' :
          opp.score >= 1800 ? 'bg-emerald-500/20 text-emerald-400' : 'bg-zinc-800 text-zinc-400'
        }`}>
          <div className="flex items-center gap-2">
            {opp.score >= 2500 && <Flame className="w-4 h-4" />}
            {opp.score}
          </div>
        </div>
        
        <button className="px-6 py-2 bg-gradient-to-r from-amber-500 to-orange-500 text-zinc-950 rounded-xl font-semibold hover:shadow-lg hover:shadow-amber-500/25 transition-all opacity-0 group-hover:opacity-100">
          {isBuild ? 'BID' : 'CALL'}
        </button>
      </div>
    </div>
  );
}
