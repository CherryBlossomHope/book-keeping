<script lang="ts">
    import { Button, Modal, Label, Input, Select } from "flowbite-svelte";
    // 打开模态框
    let formModal = false;
    let countries = [{ value: "traffic", name: "交通" }];
    const form = {
        date: "",
        billList: [
            {
                type: "",
                amount: 0,
            },
            {
                type: "traffic",
                amount: 1,
            },
        ],
        totalAmount: 0,
    };

    /** 提交表单 */
    const submit = () => {
        console.log(form);
    };
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
            <span>日期</span>
            <Input
                bind:value={form.date}
                id="date"
                name="date"
                required
                type="date"
            />
        </Label>
        {#each form.billList as item}
            <div class="flex justify-between">
                <Label class="space-y-2 w-2/5">
                    <span>类别</span>
                    <Select bind:value={item.type} items={countries} />
                </Label>
                <Label class="space-y-2 w-2/5">
                    <span>金额</span>
                    <Input
                        bind:value={item.amount}
                        id="amount"
                        name="amount"
                        required
                        type="number"
                    />
                </Label>
            </div>
        {/each}
        <div class="flex justify-between">
            <div>总金额（元）</div>
            <div class="font-semibold">{form.totalAmount}</div>
        </div>
        <Button type="submit" class="w-full1" on:click={() => submit()}
            >提交</Button
        >
    </form>
</Modal>
