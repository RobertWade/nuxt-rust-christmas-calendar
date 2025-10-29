<script lang="ts" setup>
const email = ref('')
const password = ref('')
</script>

<template>
    <CalPageGrid>
        <CalHeader icon="🎁" title="Christmas Calendar" />
        <section>
            <!-- email and password input fields -->
            <form class="flex flex-col gap-4 max-w-sm">
                <div class="flex flex-col">
                    <label for="email" class="mb-1">E-mail</label>
                    <CalTextBox id="email" v-model="email" type="email" autocomplete="email"
                        placeholder="Enter your email" />
                </div>
                <div class="flex flex-col">
                    <label for="password" class="mb-1">Password</label>
                    <CalTextBox id="password" v-model="password" type="password" placeholder="Enter your password" />
                </div>
                <CalButton to="/new-calendar">Sign In</CalButton>
                <CalButton alt-style>Sign up</CalButton>
            </form>
        </section>
    </CalPageGrid>
</template>
