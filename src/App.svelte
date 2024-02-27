<script lang="ts">
  import { formatDate } from "./utils";
  import { invoke } from "@tauri-apps/api/tauri";
  import { Button, Modal, Label, Input, Select } from "flowbite-svelte";
  import {
    Table,
    TableBody,
    TableBodyCell,
    TableBodyRow,
    TableHead,
    TableHeadCell,
  } from "flowbite-svelte";

  // 打开模态框
  let formModal = false;
  // 账单数据
  let billData: Bill[] = [];
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

  /** 获取账单数据 */
  const getBillData = async () => {
    billData = await invoke("render_get_bill");
  };

  // 获取账单数据
  getBillData();

  const getDetails = async (id: number) => {
    const res = await invoke("render_get_bill_details", { id });
    console.log(res);
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
  <Table striped={true}>
    <TableHead>
      <TableHeadCell>Id</TableHeadCell>
      <TableHeadCell>时间</TableHeadCell>
      <TableHeadCell class="text-center">金额</TableHeadCell>
      <TableHeadCell>
        <span class="sr-only">Edit</span>
      </TableHeadCell>
    </TableHead>
    <TableBody>
      {#each billData as item}
        <TableBodyRow>
          <TableBodyCell>{item.id}</TableBodyCell>
          <TableBodyCell>{formatDate(item.date)}</TableBodyCell>
          <TableBodyCell class="text-center"
            >￥{item.total_amount}</TableBodyCell
          >
          <TableBodyCell class="text-center">
            <Button
              on:click={() => getDetails(item.id)}
              size="xs"
              color="light"
              pill>详情</Button
            >
          </TableBodyCell>
        </TableBodyRow>
      {/each}
    </TableBody>
  </Table>
</div>

<Modal bind:open={formModal} size="xs" autoclose={false} class="w-full">
  <form class="flex flex-col space-y-6">
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
    <Button class="w-full1" on:click={() => submit()}>提交</Button>
  </form>
</Modal>
