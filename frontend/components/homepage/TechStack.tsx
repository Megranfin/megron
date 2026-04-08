const techs = ["Stellar", "Soroban", "USDC", "Rust", "Freighter", "Next.js"];

export default function TechStack() {
  return (
    <section className="py-16 px-8 border-y border-white/5">
      <div className="max-w-7xl mx-auto flex flex-wrap items-center justify-center gap-3">
        {techs.map((t) => (
          <span
            key={t}
            className="px-4 py-1.5 rounded-full border border-white/10 text-zinc-400 text-sm hover:border-white/20 hover:text-white transition-colors"
          >
            {t}
          </span>
        ))}
      </div>
    </section>
  );
}
