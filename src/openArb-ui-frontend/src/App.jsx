import { useState, useEffect } from "react";
import { motion } from "framer-motion";

function App() {
  const [darkMode, setDarkMode] = useState(false);

  // 👇 Optional: respect system dark mode on first load
  useEffect(() => {
    if (window.matchMedia("(prefers-color-scheme: dark)").matches) {
      setDarkMode(true);
    }
  }, []);

  return (
    <div
      className={
        darkMode
          ? "bg-gray-900 text-white min-h-screen transition-colors"
          : "bg-gray-50 text-gray-900 min-h-screen transition-colors"
      }
    >
      {/* Header */}
      <header className="flex items-center justify-between px-6 py-4 shadow-md bg-white dark:bg-gray-800">
        <h1 className="text-2xl font-bold">OpenArb Dashboard</h1>

        {/* Dark Mode Toggle */}
        <button
          onClick={() => setDarkMode(!darkMode)}
          className="relative w-14 h-7 flex items-center rounded-full p-1 bg-gray-300 dark:bg-gray-700 transition-colors"
        >
          <motion.div
            layout
            className="w-6 h-6 bg-white rounded-full shadow-md"
            transition={{ type: "spring", stiffness: 500, damping: 30 }}
          />
        </button>
      </header>

      {/* Main Content */}
      <main className="p-6">
        {/* Stats Grid */}
        <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
          <MetricCard title="Active Bots" value="12" badge="running" />
          <MetricCard title="Total Trades" value="3,487" badge="success" />
          <MetricCard title="Errors" value="5" badge="error" />
        </div>

        {/* Activity Section */}
        <motion.section
          initial={{ y: 30, opacity: 0 }}
          animate={{ y: 0, opacity: 1 }}
          transition={{ duration: 0.6 }}
          className="mt-10 bg-white dark:bg-gray-800 rounded-2xl shadow-lg p-6"
        >
          <h2 className="text-lg font-semibold mb-4">Recent Activity</h2>
          <ul className="space-y-2">
            <ActivityItem text="Bot A executed trade on ETH/USDT" />
            <ActivityItem text="Bot B stopped unexpectedly" type="error" />
            <ActivityItem text="Bot C reached profit target" type="success" />
          </ul>
        </motion.section>
      </main>
    </div>
  );
}

/* Metric Card */
function MetricCard({ title, value, badge }) {
  return (
    <motion.div
      whileHover={{ scale: 1.03 }}
      className="bg-white dark:bg-gray-800 rounded-2xl shadow-md p-6 flex flex-col justify-between transition-transform"
    >
      <h3 className="text-sm font-medium text-gray-500 dark:text-gray-400">
        {title}
      </h3>
      <p className="text-3xl font-bold my-2">{value}</p>
      <StatusBadge type={badge} />
    </motion.div>
  );
}

/* Status Badge */
function StatusBadge({ type }) {
  const styles = {
    running:
      "bg-blue-100 text-blue-700 dark:bg-blue-900 dark:text-blue-200",
    success:
      "bg-green-100 text-green-700 dark:bg-green-900 dark:text-green-200",
    error: "bg-red-100 text-red-700 dark:bg-red-900 dark:text-red-200",
  };

  return (
    <span
      className={`px-3 py-1 text-xs font-medium rounded-full ${styles[type]}`}
    >
      {type.toUpperCase()}
    </span>
  );
}

/* Activity Item */
function ActivityItem({ text, type }) {
  const textColor =
    type === "error"
      ? "text-red-600 dark:text-red-400"
      : type === "success"
      ? "text-green-600 dark:text-green-400"
      : "text-gray-700 dark:text-gray-300";

  return <li className={`text-sm ${textColor}`}>{text}</li>;
}

export default App;
