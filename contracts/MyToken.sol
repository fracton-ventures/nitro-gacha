// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "./@openzeppelin/contracts/token/ERC721/ERC721.sol";
import "./@openzeppelin/contracts/access/Ownable.sol";

contract MyToken is ERC721, Ownable {
    uint256 private _tokenId;
    mapping(uint256 => uint256) public randomValues;

    constructor(address initialOwner) ERC721("MyToken", "MTK") Ownable(initialOwner) {}

    function safeMint(address to) public onlyOwner {
        uint256 tokenId = _tokenId++;
        _safeMint(to, tokenId);
        setRandomValue(tokenId);
    }

    function setRandomValue(uint256 tokenId) internal {
        uint256 randomValue = uint256(keccak256(abi.encodePacked(tokenId, block.timestamp, msg.sender)));
        randomValues[tokenId] = randomValue;
    }

    function getRandomValue(uint256 tokenId) public view returns (uint256) {
        return randomValues[tokenId];
    }
}
