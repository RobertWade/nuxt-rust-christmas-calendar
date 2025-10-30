import { defineStore } from 'pinia'
import type { User } from '@/types'

export const useUserStore = defineStore(
  "user",
  () => {
    const user = ref<User | null>(null);

    function setUser(payload: User) {
      user.value = payload;
    }

    function clearUser() {
      user.value = null;
    }

    return { user, setUser, clearUser };
  },
  {
    persist: true,
  }
);