const points = [
  "3–5 second finality",
  "Fractions of a cent per transaction",
  "Native USDC integration",
  "Soroban smart contract platform",
  "Built-in decentralized exchange (SDEX)",
];

export default function WhyStellar() {
  return (
    <section id="why-stellar" className="py-24 px-8 bg-[#0d0d0d]">
      <div className="max-w-7xl mx-auto flex flex-col md:flex-row items-start justify-between gap-16">
        <div className="flex-1 max-w-xs">
          <h2 className="text-white text-3xl font-bold leading-snug">
            Built on the fastest payment blockchain.
          </h2>
        </div>

        <div className="flex-1 flex flex-col gap-4">
          {points.map((p, i) => (
            <div key={i} className="flex items-center gap-3">
              <span className="w-2 h-2 rounded-full bg-blue-500 shrink-0" />
              <span className="text-zinc-300 text-sm">{p}</span>
            </div>
          ))}
        </div>
      </div>
    </section>
  );
}
