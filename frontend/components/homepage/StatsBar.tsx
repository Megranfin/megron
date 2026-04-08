const stats = [
  { value: "~0.00001", unit: "XLM", label: "PER TRANSACTION" },
  { value: "3–5s", unit: "", label: "FINALITY" },
  { value: "1.4B", unit: "", label: "UNBANKED ADULTS" },
  { value: "$0", unit: "", label: "ON MIDDLEMEN" },
];

export default function StatsBar() {
  return (
    <section className="bg-[#111111] border-y border-white/5 py-12">
      <div className="max-w-7xl mx-auto px-8 grid grid-cols-2 md:grid-cols-4 gap-8">
        {stats.map((s, i) => (
          <div key={i} className="flex flex-col gap-1">
            <p className="text-white text-3xl font-bold tracking-tight">
              {s.value}
              {s.unit && <span className="text-xl ml-1">{s.unit}</span>}
            </p>
            <p className="text-zinc-500 text-xs tracking-widest uppercase">
              {s.label}
            </p>
          </div>
        ))}
      </div>
    </section>
  );
}
