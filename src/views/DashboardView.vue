<template>
  <div class="h-full flex flex-col w-full mx-auto border-l border-neutral">
    <!-- Header / Stats -->
    <StatsOverview />

    <!-- Queue & Recent Activity Grid -->
    <ActivityGrid />
  </div>
</template>

<script setup lang="ts">
import StatsOverview from '../components/dashboard/StatsOverview.vue';
import ActivityGrid from '../components/dashboard/ActivityGrid.vue';
</script>


