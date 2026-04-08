export default function Hero() {
  const transactions = [
    { pair: "USDY / JEFF", amount: "525.00 USDC", status: "Settled" },
    { pair: "USDK / JFU", amount: "83.50 USDU", status: "Settled" },
    { pair: "USDC / USDB", amount: "200.00 USDC", status: "Settled" },
  ];

  return (
    <section className="min-h-screen flex items-center px-8 pt-24 pb-16 max-w-7xl mx-auto w-full">
      <div className="flex flex-col lg:flex-row items-center justify-between w-full gap-16">
        {/* Left */}
        <div className="flex-1 max-w-xl">
          <h1 className="text-5xl md:text-6xl font-bold text-white leading-tight tracking-tight mb-6">
            Send money anywhere. No banks. No borders. No fees.
          </h1>
          <p className="text-zinc-400 text-base mb-8 max-w-sm leading-relaxed">
            Open-source payment infrastructure on Stellar. Instant settlement.
            Zero chargebacks.
          </p>
          <div className="flex items-center gap-4">
            <a
              href="#"
              className="bg-blue-600 hover:bg-blue-500 transition-colors text-white text-sm font-medium px-5 py-2.5 rounded-md"
            >
              Get Started
            </a>
            <a
              href="#"
              className="text-zinc-300 hover:text-white text-sm transition-colors"
            >
              Read the Docs
            </a>
          </div>
        </div>

        {/* Right — Live Transactions card */}
        <div className="w-full max-w-sm bg-[#111111] border border-white/10 rounded-xl p-5">
          <p className="text-zinc-400 text-xs font-medium mb-4 uppercase tracking-widest">
            Live Transactions
          </p>
          <div className="flex flex-col gap-3">
            {transactions.map((tx, i) => (
              <div key={i} className="flex items-center justify-between">
                <div>
                  <p className="text-white text-sm font-medium">{tx.pair}</p>
                  <p className="text-zinc-500 text-xs">{tx.amount}</p>
                </div>
                <span className="flex items-center gap-1.5 text-emerald-400 text-xs font-medium">
                  <span className="w-1.5 h-1.5 rounded-full bg-emerald-400 inline-block" />
                  {tx.status}
                </span>
              </div>
            ))}
          </div>
        </div>
      </div>
    </section>
  );
}
