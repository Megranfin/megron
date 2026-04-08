export default function CTA() {
  return (
    <section className="py-28 px-8 text-center">
      <div className="max-w-2xl mx-auto flex flex-col items-center gap-8">
        <h2 className="text-white text-4xl md:text-5xl font-bold leading-tight">
          Open source. Permissionless. For everyone.
        </h2>
        <a
          href="https://github.com/Megron1/megron"
          target="_blank"
          rel="noopener noreferrer"
          className="inline-flex items-center gap-2 border border-white/20 hover:border-white/40 text-white text-sm font-medium px-6 py-3 rounded-md transition-colors"
        >
          View on GitHub →
        </a>
      </div>
    </section>
  );
}
