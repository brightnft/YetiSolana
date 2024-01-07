const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("NFT Minting", function () {
  it("Should mint an NFT", async function () {
    // Deploy the contract
    const NFT = await ethers.getContractFactory("YourNFTContract");
    const nft = await NFT.deploy();
    await nft.deployed();

    // Mint an NFT
    const mintTx = await nft.mint("0xYourWalletAddress"); // Replace with your wallet address

    // Wait for the mint transaction to be mined
    await mintTx.wait();

    // Check ownership of the minted NFT (tokenId 0)
    expect(await nft.ownerOf(0)).to.equal("0xYourWalletAddress");
  });
});