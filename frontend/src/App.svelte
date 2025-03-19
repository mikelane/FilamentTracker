<script lang="ts">
  import "./app.css";
  import FilamentCard from "./components/FilamentCard.svelte";
  import DarkModeToggle from "./components/DarkModeToggle.svelte";
  
  let pageTitle = "FilamentTracker";
  
  // Sample filament data
  const filaments = [
    {
      name: "Basic Black PLA",
      material: "PLA",
      color: "#000000",
      diameter: 1.75,
      weight: 1000,
      remainingWeight: 750,
      temperature: 210,
      manufacturer: "Hatchbox"
    },
    {
      name: "Silk Purple PLA",
      material: "PLA",
      color: "#800080",
      diameter: 1.75,
      weight: 1000,
      remainingWeight: 950,
      temperature: 205,
      manufacturer: "CC3D"
    },
    {
      name: "Transparent PETG",
      material: "PETG",
      color: "#a0c8e0",
      diameter: 1.75,
      weight: 750,
      remainingWeight: 200,
      temperature: 235,
      manufacturer: "eSun"
    }
  ];
</script>

<main class="min-h-screen bg-gray-100 dark:bg-gray-900 transition-colors duration-200">
  <div class="max-w-7xl mx-auto py-6 sm:px-6 lg:px-8">
    <!-- Header -->
    <header class="bg-white dark:bg-gray-800 shadow">
      <div class="max-w-7xl mx-auto py-6 px-4 sm:px-6 lg:px-8 flex justify-between items-center">
        <h1 class="text-3xl font-bold text-gray-900 dark:text-white">{pageTitle}</h1>
        <DarkModeToggle />
      </div>
    </header>
    
    <!-- Main content -->
    <div class="py-6">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div class="bg-white dark:bg-gray-800 shadow overflow-hidden sm:rounded-lg p-6 mb-6">
          <p class="text-lg text-gray-500 dark:text-gray-400">
            Welcome to FilamentTracker! This application will help you track your 3D printing filament inventory.
          </p>
          
          <div class="mt-6">
            <button class="inline-flex items-center px-4 py-2 border border-transparent text-base font-medium rounded-md shadow-sm text-white bg-blue-600 dark:bg-blue-700 hover:bg-blue-700 dark:hover:bg-blue-800 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 dark:focus:ring-offset-gray-800">
              Add New Filament
            </button>
          </div>
        </div>
        
        <h2 class="text-xl font-semibold text-gray-800 dark:text-gray-200 mb-4">Your Filament Collection</h2>
        
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          {#each filaments as filament}
            <FilamentCard {...filament} />
          {/each}
        </div>
      </div>
    </div>
  </div>
</main>
