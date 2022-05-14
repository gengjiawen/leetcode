import {underline} from "./generate_pools";

test("test underline file name", () => {
    const a = underline("minimum-swaps-to-group-all-1s-together")
    expect(a).toBe("minimum_swaps_to_group_all_1s_together")
})
