<template>
    <CalPageGrid>
        <CalHeader title="Create New Calendar" icon="🎄" />
        <form class="flex flex-col gap-4 max-w-sm">
            <div class="flex flex-col">
                <label for="calendar-name" class="mb-1">Calendar Name</label>
                <CalTextBox id="calendar-name" placeholder="Enter calendar name" />
            </div>
            <div class="flex flex-col">
                <label for="recipient" class="mb-1">Recipient</label>
                <CalTextBox id="recipient" placeholder="Enter recipient name" />
            </div>
            <div class="flex flex-col">
                <label for="description" class="mb-1">Description (optional)</label>
                <CalTextBox id="description" placeholder="Enter calendar description" />
            </div>
            <CalButton to="view-calendar">Next</CalButton>
        </form>
    </CalPageGrid>
</template>