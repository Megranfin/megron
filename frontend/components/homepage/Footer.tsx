export default function Footer() {
  return (
    <footer className="border-t border-white/5 px-8 py-10">
      <div className="max-w-7xl mx-auto flex flex-col md:flex-row items-center justify-between gap-6">
        <span className="text-white font-semibold text-sm">Megron</span>

        <div className="flex items-center gap-6 text-zinc-500 text-sm">
          <a href="#" className="hover:text-white transition-colors">
            Docs
          </a>
          <a href="#" className="hover:text-white transition-colors">
            GitHub
          </a>
          <a href="#" className="hover:text-white transition-colors">
            Contributing
          </a>
          <a href="#" className="hover:text-white transition-colors">
            Twitter
          </a>
        </div>

        <p className="text-zinc-600 text-xs">© 2025 Megron MIT License</p>
      </div>
    </footer>
  );
}
