import useSWR from "swr";
import { getAddress, getClient, getSigningClient } from "../lib/client";
import { getContractAddr } from "../lib/state";

export const getCount = async () => {
  const client = await getClient();
  return await client.queryContractSmart(getContractAddr(), { get_count: {} });
};
export const delegate = async (delegateAddress: string) => {
  console.log(await getAddress())

  const client = await getSigningClient();
  return await client.execute(
    await getAddress(),
    getContractAddr(),
    { delegate_all: {delegate_address: delegateAddress} },
    "auto"
  );
};

export const revoke = async (delegateAddress: string) => {
  const client = await getSigningClient();
  return await client.execute(
    await getAddress(),
    getContractAddr(),
    { revoke_delegation: {delegate_address: delegateAddress} },
    "auto"
  );
};