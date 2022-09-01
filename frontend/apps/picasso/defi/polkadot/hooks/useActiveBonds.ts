import { BondOffer } from "@/stores/defi/polkadot/bonds/types";
import { useEffect } from "react";
import { usePicassoProvider, useSelectedAccount } from "@/defi/polkadot/hooks/index";
import { ApiPromise } from "@polkadot/api";
import { unwrapNumberOrHex } from "shared";
import { useStore } from "@/stores/root";
import { Codec } from "@polkadot/types-codec/types";
import { ActiveBond } from "@/stores/defi/polkadot/bonds/slice";
import { ComposableTraitsVestingVestingScheduleIdSet } from "defi-interfaces";
import { u8aEmpty } from "@polkadot/util";

type VestingAccount = { name: string; address: string };

const bondedVestingSchedule = (bond: BondOffer) => (address: string) => (
  api: ApiPromise
) => {
  return async () => {
    const vestingScheduleResponse: Codec = await api.query.vesting.vestingSchedules(
      address,
      bond.reward.assetId
    );

    if (vestingScheduleResponse.isEmpty) {
      return null;
    }
    const fromCodec: ComposableTraitsVestingVestingScheduleIdSet = vestingScheduleResponse.toJSON() as any;


    return Object.values(fromCodec).flatMap(vs => {
      if (vs) {
        const perPeriod = unwrapNumberOrHex((vs as any).perPeriod);
        const periodCount = unwrapNumberOrHex((vs as any).periodCount);
        const window = {
          blockNumberBased: {
            start: unwrapNumberOrHex((vs as any).window.blockNumberBased.start),
            period: unwrapNumberOrHex(
              (vs as any).window.blockNumberBased.period
            )
          }
        };
        return {
          bond,
          perPeriod,
          periodCount,
          window,
          type: window.blockNumberBased ? "block" : "time"
        };
      }
    });
  };
};

export function useActiveBonds() {
  const account = useSelectedAccount();
  const {
    bonds,
    updateOpenPositions,
    activeBonds,
  } = useStore<{
    bonds: BondOffer[];
    updateOpenPositions: (openPositions: any) => void;
    activeBonds: ActiveBond[];
  }>(state => ({
    bonds: state.bonds.bonds,
    updateOpenPositions: state.bonds.updateOpenPositions,
    activeBonds: state.bonds.openPositions,
  }));
  const { parachainApi } = usePicassoProvider();

  async function fetchVestingSchedules(api: ApiPromise, acc: VestingAccount) {
    const schedules = [];
    for (const bond of bonds) {
      const vestingSchedule = await api.query.vesting.vestingSchedules(
        api.createType("AccountId32", acc.address),
        api.createType("Option<u128>", )
      );
      // Skip if empty
      if (vestingSchedule.isEmpty) continue;

      //bond: BondOffer;
      //   periodCount: BigNumber;
      //   perPeriod: BigNumber;
      //   vestingScheduleId: number;
      //   window: {
      //     blockNumberBased: {
      //       start: BigNumber;
      //       period: BigNumber;
      //     };
      //   };
      schedules.push(Object.values(vestingSchedule.toJSON()));
    }

    return schedules;
  }

  async function fetchAndStore(factoryFn: () => Promise<unknown>) {
    const result: any = await factoryFn();
    updateOpenPositions(result.filter(Boolean).flat());
  }

  useEffect(() => {
    let unsub = () => {};
    if (parachainApi && account) {
      fetchAndStore(() => fetchVestingSchedules(parachainApi, account));
    }

    return () => unsub();
  }, [parachainApi, bonds, account]); // eslint-disable-line react-hooks/exhaustive-deps

  return activeBonds;
}
