import { StartResponse, Symbol } from "../../generated/models";

const SYMBOL_TO_URL = {
    [Symbol.Cherry]: "/images/cherry.png",
    [Symbol.Bar]: "/images/bar.png",
    [Symbol.DoubleBar]: "/images/doubleBar.png",
    [Symbol.TripleBar]: "/images/tripleBar.png",
    [Symbol.Seven]: "/images/seven.png",
    [Symbol.MinorJackpot]: "/images/minorJackpot<REEL>.png",
    [Symbol.MajorJackpot]: "/images/majorJackpot.png",
    [Symbol.GrandJackpot]: "/images/grandJackpot.png",
};

export function getLayoutUrls(metadata: StartResponse): string[][] {
    const layoutUrls = new Array<string[]>();
    for (let i = 0; i < metadata.nWheels; i++) {
        layoutUrls.push(
            metadata.reelLayout.map((symbol) => {
                let url = SYMBOL_TO_URL[symbol];
                return url.replace("<REEL>", (i + 1).toString());
            }),
        );
    }

    return layoutUrls;
}

export function getStopIndexes(reelLayout: Symbol[], stops: Symbol[]): number[] {
    return stops.map((symbol) => {
        const indexes = reelLayout
            .map((x, i) => {
                if (x === symbol) return i;
            })
            .filter((x) => x !== undefined);

        // Note: segments are 1-indexed, not 0-indexed
        return indexes[Math.floor(Math.random() * indexes.length)] + 1;
    });
}
