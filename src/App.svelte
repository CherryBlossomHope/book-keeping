<script lang="ts">
    import { Button, Modal, Label, Input, Checkbox } from "flowbite-svelte";
    // 打开模态框
    let formModal = false;
</script>

<div class="p-3.5">
    <p class="text-2xl font-semibold text-center">记账</p>
    <div class="text-right">
        <Button on:click={() => (formModal = true)} size="xs">新增账单</Button>
    </div>
</div>

<Modal bind:open={formModal} size="xs" autoclose={false} class="w-full">
    <form class="flex flex-col space-y-6" action="#">
        <Label class="space-y-2">
            <span>Email</span>
            <Input
                type="email"
                name="email"
                placeholder="name@company.com"
                required
            />
        </Label>
        <Label class="space-y-2">
            <span>Your password</span>
            <Input
                type="password"
                name="password"
                placeholder="•••••"
                required
            />
        </Label>
        <div class="flex items-start">
            <Checkbox>Remember me</Checkbox>
            <a
                href="/"
                class="ms-auto text-sm text-primary-700 hover:underline dark:text-primary-500"
            >
                Lost password?
            </a>
        </div>
        <Button type="submit" class="w-full1">Login to your account</Button>
        <div class="text-sm font-medium text-gray-500 dark:text-gray-300">
            Not registered? <a
                href="/"
                class="text-primary-700 hover:underline dark:text-primary-500"
            >
                Create account
            </a>
        </div>
    </form>
</Modal>
