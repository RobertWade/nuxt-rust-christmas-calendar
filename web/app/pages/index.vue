<script setup lang="ts">
definePageMeta({
    name: 'CalendarHome'
});
</script>

<template>
    <div class="min-h-dvh bg-background text-text">
            <section class="relative overflow-hidden bg-gradient-to-b from-primary via-primary-dark to-accent text-white">
                <div
                    class="absolute inset-0 opacity-30 bg-[url('https://images.unsplash.com/photo-1482517967863-00e15c9b44be?auto=format&fit=crop&w=1200&q=80')] bg-cover bg-center"
                    aria-hidden="true"
                />
            <div class="relative z-10 px-6 py-16 sm:px-10">
                <div class="max-w-md mx-auto text-center space-y-6">
                    <span class="inline-flex items-center gap-2 rounded-full bg-white/20 px-4 py-1 text-sm font-medium uppercase tracking-widest">
                        <span>🎄</span>
                        Custom Love Calendar
                    </span>
                    <h1 class="text-4xl font-semibold leading-tight sm:text-5xl">
                        Create 24 magical moments for your loved one
                    </h1>
                    <p class="text-base text-white/85">
                        In just a few minutes, create a personalized advent calendar filled with messages, tasks, and surprises that celebrate your shared story every day.
                    </p>
                    <div class="flex flex-col gap-3 sm:flex-row sm:justify-center">
                        <NuxtLink
                            to="/signup"
                            class="inline-flex items-center justify-center rounded-full bg-secondary px-6 py-3 font-medium text-text shadow-soft transition hover:-translate-y-0.5 hover:shadow-elevated"
                        >
                            Create Calendar Now
                        </NuxtLink>
                        <NuxtLink
                            to="/"
                            class="inline-flex items-center justify-center rounded-full border border-white/40 px-6 py-3 font-medium text-white transition hover:bg-white/15"
                        >
                            View Demo
                        </NuxtLink>
                    </div>
                </div>
            </div>
        </section>

        <section class="px-6 py-16 sm:px-10">
            <div class="max-w-3xl mx-auto space-y-10">
                <div class="space-y-4 text-center">
                    <h2 class="text-2xl font-semibold">How It Works</h2>
                    <p class="text-text/80">
                        Three simple steps to your very own Christmas countdown full of love and surprises.
                    </p>
                </div>
                <div class="grid gap-5 sm:grid-cols-3">
                    <div class="rounded-2xl bg-white/80 p-6 shadow-soft ring-1 ring-border backdrop-blur">
                        <span class="inline-flex h-10 w-10 items-center justify-center rounded-full bg-primary/10 text-lg font-semibold text-primary">1</span>
                        <h3 class="mt-4 text-lg font-semibold">Gather Magical Ideas</h3>
                        <p class="mt-2 text-sm text-text/75">Log in, tell us about your rituals and highlights. We'll suggest suitable door contents for you.</p>
                    </div>
                    <div class="rounded-2xl bg-white/80 p-6 shadow-soft ring-1 ring-border backdrop-blur">
                        <span class="inline-flex h-10 w-10 items-center justify-center rounded-full bg-primary/10 text-lg font-semibold text-primary">2</span>
                        <h3 class="mt-4 text-lg font-semibold">Personalize the Doors</h3>
                        <p class="mt-2 text-sm text-text/75">Fill each door with messages, videos, or small tasks tailored to your daily life.</p>
                    </div>
                    <div class="rounded-2xl bg-white/80 p-6 shadow-soft ring-1 ring-border backdrop-blur">
                        <span class="inline-flex h-10 w-10 items-center justify-center rounded-full bg-primary/10 text-lg font-semibold text-primary">3</span>
                        <h3 class="mt-4 text-lg font-semibold">Wait & Enchant</h3>
                        <p class="mt-2 text-sm text-text/75">Share the calendar and let your partner open a new, loving chapter every morning.</p>
                    </div>
                </div>
            </div>
        </section>

        
        <!-- <section class="px-6 pb-16 sm:px-10">
            <div class="max-w-3xl mx-auto rounded-3xl bg-white p-8 shadow-elevated ring-1 ring-border">
                <div class="flex flex-col gap-6 sm:flex-row sm:items-center">
                    <div class="flex-1 space-y-3">
                        <h3 class="text-xl font-semibold text-primary">Love in Numbers</h3>
                        <p class="text-sm text-text/75">
                            Over 12,000 couples rediscovered their advent together last year. 95% say it helped them spend more mindful time together.
                        </p>
                        <ul class="space-y-2 text-sm text-text/80">
                            <li class="flex items-center gap-2"><span class="text-primary">✨</span> Personalized suggestions tailored to your story</li>
                            <li class="flex items-center gap-2"><span class="text-primary">💬</span> Reminders & pushes directly to your phone</li>
                            <li class="flex items-center gap-2"><span class="text-primary">🎁</span> Shared gallery for photos and videos</li>
                        </ul>
                    </div>
                    <div class="flex-1 rounded-2xl bg-background p-6 text-center shadow-soft">
                        <p class="text-sm uppercase tracking-wider text-text/70">Message from a User</p>
                        <blockquote class="mt-3 text-lg italic">
                            “My partner had tears in their eyes because every message was perfect for us. Advent was magical again.”
                        </blockquote>
                        <p class="mt-4 text-sm font-medium text-text">— Julia & Tom, together for 5 years</p>
                    </div>
                </div>
            </div>
        </section> -->

        <section class="px-6 pb-20 sm:px-10">
            <div class="max-w-3xl mx-auto rounded-3xl bg-primary text-white shadow-elevated">
                <div class="space-y-6 px-8 py-12 text-center">
                    <h2 class="text-3xl font-semibold">Ready for 24 days of heartbeats?</h2>
                    <p class="text-white/85">
                        Start for free today, secure exclusive design templates, and let’s make every morning special together.
                    </p>
                    <div class="flex flex-col gap-3 sm:flex-row sm:justify-center">
                        <NuxtLink
                            to="/signup"
                            class="inline-flex items-center justify-center rounded-full bg-secondary px-6 py-3 font-medium text-text shadow-soft transition hover:-translate-y-0.5 hover:shadow-glow-gold"
                        >
                            Create Calendar for Free
                        </NuxtLink>
                        <NuxtLink
                            to="/"
                            class="inline-flex items-center justify-center rounded-full border border-white/40 px-6 py-3 font-medium text-white transition hover:bg-white/15"
                        >
                            Discover Inspiration
                        </NuxtLink>
                    </div>
                </div>
            </div>
        </section>

        <footer class="bg-background text-text py-6">
            <div class="max-w-3xl mx-auto text-center space-y-4">
                <p class="text-sm text-text/75">&copy; 2025 Custom Love Calendar. All rights reserved.</p>
                <div class="flex justify-center space-x-4">
                    <NuxtLink to="/" class="text-sm text-primary hover:underline">Privacy Policy</NuxtLink>
                    <NuxtLink to="/" class="text-sm text-primary hover:underline">Terms of Service</NuxtLink>
                    <NuxtLink to="/" class="text-sm text-primary hover:underline">Contact Us</NuxtLink>
                </div>
            </div>
        </footer>
    </div>
</template>