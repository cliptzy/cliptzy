<template>
  <div class="h-full flex flex-col gap-6 w-full max-w-7xl mx-auto">
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
