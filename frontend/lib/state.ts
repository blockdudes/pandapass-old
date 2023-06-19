import state from "../.beaker/state.json";

const getState = () => {
  if (!process.env.NEXT_PUBLIC_NETWORK) {
    throw Error("`NEXT_PUBLIC_NETWORK` env variable not found, please set");
  }
  return {
    ...state,
  }[process.env.NEXT_PUBLIC_NETWORK];
};

export const getContractAddr = () => {
  const contractAddr = getState()?.pandapass.addresses.default;

  if (!contractAddr) {
    throw Error("Contract address not found, please check your state file");
  }

  return contractAddr;
};
