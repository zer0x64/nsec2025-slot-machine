// https://stackoverflow.com/questions/40031688/how-can-i-convert-an-arraybuffer-to-a-hexadecimal-string-hex

const byteToHex: any = [];

for (let n = 0; n <= 0xff; ++n) {
  const hexOctet = n.toString(16).padStart(2, "0");
  byteToHex.push(hexOctet);
}

export function hex(arrayBuffer: Uint8Array) {
  const buff = new Uint8Array(arrayBuffer);
  const hexOctets = [];

  for (let i = 0; i < buff.length; ++i) {
    hexOctets.push(byteToHex[buff[i]]);
  }

  return hexOctets.join("");
}

export function chunkString(str: string, chunkSize: number) {
  const numChunks = Math.ceil(str.length / chunkSize);
  const chunks = new Array<string>(numChunks);

  for (let i = 0, o = 0; i < numChunks; ++i, o += chunkSize) {
    chunks[i] = str.substring(o, o + chunkSize);
  }

  return chunks;
}
