const { expect } = require("chai");
describe("MyTest", function () {
it("should run", async function () {
const contract = await ethers.deployContract("Contract");
await expect(contract.func_a_b(-37, -41))
.to.emit(contract, "Verdict")
.withArgs(0);
await expect(contract.func_a_b(-20, 35))
.to.emit(contract, "Verdict")
.withArgs(0);
await expect(contract.func_a_b(34, 96))
.to.emit(contract, "Verdict")
.withArgs(1);
await expect(contract.func_a_b(48, -71))
.to.emit(contract, "Verdict")
.withArgs(1);
await expect(contract.func_a_b(91, 23))
.to.emit(contract, "Verdict")
.withArgs(1);
await expect(contract.func_a_b(80, -45))
.to.emit(contract, "Verdict")
.withArgs(1);
await expect(contract.func_a_b(66, 70))
.to.emit(contract, "Verdict")
.withArgs(1);
await expect(contract.func_a_b(-100, -26))
.to.emit(contract, "Verdict")
.withArgs(0);
await expect(contract.func_a_b(63, -41))
.to.emit(contract, "Verdict")
.withArgs(1);
await expect(contract.func_a_b(53, 5))
.to.emit(contract, "Verdict")
.withArgs(1);
await expect(contract.func_a_b(-76, -64))
.to.emit(contract, "Verdict")
.withArgs(0);
await expect(contract.func_a_b(73, 54))
.to.emit(contract, "Verdict")
.withArgs(1);
await expect(contract.func_a_b(-41, -43))
.to.emit(contract, "Verdict")
.withArgs(0);
await expect(contract.func_a_b(73, -51))
.to.emit(contract, "Verdict")
.withArgs(1);
await expect(contract.func_a_b(62, 62))
.to.emit(contract, "Verdict")
.withArgs(1);
await expect(contract.func_a_b(42, -96))
.to.emit(contract, "Verdict")
.withArgs(1);
await expect(contract.func_a_b(-62, -51))
.to.emit(contract, "Verdict")
.withArgs(0);
await expect(contract.func_a_b(-83, 98))
.to.emit(contract, "Verdict")
.withArgs(0);
await expect(contract.func_a_b(-1, -60))
.to.emit(contract, "Verdict")
.withArgs(0);
await expect(contract.func_a_b(-51, 86))
.to.emit(contract, "Verdict")
.withArgs(0);
await expect(contract.func_a_b(-12, 37))
.to.emit(contract, "Verdict")
.withArgs(0);
await expect(contract.func_a_b(40, 48))
.to.emit(contract, "Verdict")
.withArgs(1);
await expect(contract.func_a_b(-69, 15))
.to.emit(contract, "Verdict")
.withArgs(0);
await expect(contract.func_a_b(47, 15))
.to.emit(contract, "Verdict")
.withArgs(1);
await expect(contract.func_a_b(-91, 51))
.to.emit(contract, "Verdict")
.withArgs(0);
await expect(contract.func_a_b(99, 30))
.to.emit(contract, "Verdict")
.withArgs(1);
await expect(contract.func_a_b(-42, -69))
.to.emit(contract, "Verdict")
.withArgs(0);
await expect(contract.func_a_b(-14, -5))
.to.emit(contract, "Verdict")
.withArgs(0);
await expect(contract.func_a_b(-55, 88))
.to.emit(contract, "Verdict")
.withArgs(0);
await expect(contract.func_a_b(4, -37))
.to.emit(contract, "Verdict")
.withArgs(0);
});
});
