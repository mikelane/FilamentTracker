<script lang="ts">
  export let name: string = "";
  export let material: string = "";
  export let color: string = "";
  export let diameter: number = 1.75;
  export let weight: number = 1000;
  export let remainingWeight: number = 1000;
  export let temperature: number = 200;
  export let manufacturer: string = "";
  
  $: percentRemaining = Math.round((remainingWeight / weight) * 100);
  $: colorStyle = `background-color: ${color};`;
  
  // Simple dropdown state without using Headless UI
  let isMenuOpen = false;
  
  function toggleMenu() {
    isMenuOpen = !isMenuOpen;
  }
  
  function handleClickOutside(event) {
    if (isMenuOpen && !event.target.closest('.dropdown-menu')) {
      isMenuOpen = false;
    }
  }
</script>

<svelte:window on:click={handleClickOutside} />

<div class="bg-white dark:bg-gray-800 shadow overflow-hidden sm:rounded-lg max-w-md">
  <div class="px-4 py-5 sm:px-6 flex justify-between items-center">
    <div>
      <h3 class="text-lg leading-6 font-medium text-gray-900 dark:text-white">{name}</h3>
      <p class="mt-1 max-w-2xl text-sm text-gray-500 dark:text-gray-400">{manufacturer} - {material}</p>
    </div>
    <div class="h-12 w-12 rounded-full border dark:border-gray-600" style={colorStyle}></div>
  </div>
  
  <div class="border-t border-gray-200 dark:border-gray-700">
    <dl>
      <div class="bg-gray-50 dark:bg-gray-700 px-4 py-4 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6">
        <dt class="text-sm font-medium text-gray-500 dark:text-gray-300">Material Type</dt>
        <dd class="mt-1 text-sm text-gray-900 dark:text-gray-100 sm:mt-0 sm:col-span-2">{material}</dd>
      </div>
      <div class="bg-white dark:bg-gray-800 px-4 py-4 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6">
        <dt class="text-sm font-medium text-gray-500 dark:text-gray-300">Diameter</dt>
        <dd class="mt-1 text-sm text-gray-900 dark:text-gray-100 sm:mt-0 sm:col-span-2">{diameter} mm</dd>
      </div>
      <div class="bg-gray-50 dark:bg-gray-700 px-4 py-4 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6">
        <dt class="text-sm font-medium text-gray-500 dark:text-gray-300">Print Temperature</dt>
        <dd class="mt-1 text-sm text-gray-900 dark:text-gray-100 sm:mt-0 sm:col-span-2">{temperature}°C</dd>
      </div>
      <div class="bg-white dark:bg-gray-800 px-4 py-4 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6">
        <dt class="text-sm font-medium text-gray-500 dark:text-gray-300">Remaining</dt>
        <dd class="mt-1 text-sm text-gray-900 dark:text-gray-100 sm:mt-0 sm:col-span-2">
          <div class="relative pt-1">
            <div class="flex mb-2 items-center justify-between">
              <div>
                <span class="text-xs font-semibold inline-block text-blue-600 dark:text-blue-400">
                  {remainingWeight}g / {weight}g
                </span>
              </div>
              <div class="text-right">
                <span class="text-xs font-semibold inline-block text-blue-600 dark:text-blue-400">
                  {percentRemaining}%
                </span>
              </div>
            </div>
            <div class="overflow-hidden h-2 text-xs flex rounded bg-blue-200 dark:bg-blue-900">
              <div 
                style="width: {percentRemaining}%" 
                class="shadow-none flex flex-col text-center whitespace-nowrap text-white justify-center bg-blue-500">
              </div>
            </div>
          </div>
        </dd>
      </div>
    </dl>
  </div>
  
  <div class="border-t border-gray-200 dark:border-gray-700 px-4 py-4 sm:px-6">
    <div class="flex justify-end space-x-3">
      <!-- Simple dropdown menu without using HeadlessUI -->
      <div class="relative">
        <button 
          type="button" 
          on:click={toggleMenu}
          class="inline-flex items-center px-3 py-2 border border-gray-300 dark:border-gray-600 shadow-sm text-sm leading-4 font-medium rounded-md text-gray-700 dark:text-gray-200 bg-white dark:bg-gray-700 hover:bg-gray-50 dark:hover:bg-gray-600 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 dark:focus:ring-offset-gray-800"
        >
          Actions
          <!-- Simple down arrow icon using HTML -->
          <svg class="ml-2 -mr-0.5 h-4 w-4" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor">
            <path fill-rule="evenodd" d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z" clip-rule="evenodd" />
          </svg>
        </button>
        
        {#if isMenuOpen}
          <div class="dropdown-menu origin-top-right absolute right-0 mt-2 w-48 rounded-md shadow-lg bg-white dark:bg-gray-800 ring-1 ring-black ring-opacity-5 dark:ring-gray-700 focus:outline-none">
            <div class="py-1">
              <button class="block w-full text-left px-4 py-2 text-sm text-gray-700 dark:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-700">
                Edit Filament
              </button>
              <button class="block w-full text-left px-4 py-2 text-sm text-gray-700 dark:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-700">
                Use Filament
              </button>
              <button class="block w-full text-left px-4 py-2 text-sm text-gray-700 dark:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-700">
                Delete
              </button>
            </div>
          </div>
        {/if}
      </div>
      
      <button type="button" class="inline-flex items-center px-3 py-2 border border-transparent text-sm leading-4 font-medium rounded-md shadow-sm text-white bg-blue-600 hover:bg-blue-700 dark:bg-blue-700 dark:hover:bg-blue-800 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 dark:focus:ring-offset-gray-800">
        Use Filament
      </button>
    </div>
  </div>
</div>
