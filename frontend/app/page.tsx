import Navbar from "@/components/homepage/Navbar";
import Hero from "@/components/homepage/Hero";
import StatsBar from "@/components/homepage/StatsBar";
import Features from "@/components/homepage/Features";
import WhyStellar from "@/components/homepage/WhyStellar";
import TechStack from "@/components/homepage/TechStack";
import CTA from "@/components/homepage/CTA";
import Footer from "@/components/homepage/Footer";

export default function Home() {
  return (
    <div className="bg-[#0a0a0a] min-h-screen flex flex-col">
      <Navbar />
      <main className="flex flex-col flex-1">
        <Hero />
        <StatsBar />
        <Features />
        <WhyStellar />
        <TechStack />
        <CTA />
      </main>
      <Footer />
    </div>
  );
}
