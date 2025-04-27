import './App.css';

function App() {
  return (
    <div className="gradient-background">
      <header className="main-header">
        <h1>Mon Header Transparent</h1>
      </header>

      <div className="layout-container">
        <aside className="sidebar">
          <ul>
            <li>Dashboard</li>
            <li>Explorer</li>
            <li>Team</li>
            <li>Settings</li>
          </ul>
        </aside>

        <main className="main-content">
          <div className="current-content"></div>
        </main>
      </div>
    </div>
  );
}

export default App;
