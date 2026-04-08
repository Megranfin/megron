export default function Navbar() {
  return (
    <nav className="fixed top-0 left-0 right-0 z-50 flex items-center justify-between px-8 py-4 bg-[#0a0a0a]/80 backdrop-blur-sm border-b border-white/5">
      <span className="text-white font-semibold text-base tracking-tight">
        Megron
      </span>

      <div className="hidden md:flex items-center gap-8 text-sm text-zinc-400">
        <a href="#features" className="hover:text-white transition-colors">
          Features
        </a>
        <a href="#why-stellar" className="hover:text-white transition-colors">
          Why Stellar
        </a>
        <a href="#docs" className="hover:text-white transition-colors">
          Docs
        </a>
        <a
          href="https://github.com/Megron1/megron"
          target="_blank"
          rel="noopener noreferrer"
          className="hover:text-white transition-colors"
        >
          GitHub
        </a>
      </div>

      <a
        href="#"
        className="bg-blue-600 hover:bg-blue-500 transition-colors text-white text-sm font-medium px-4 py-2 rounded-md"
      >
        Launch App
      </a>
    </nav>
  );
}
