// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

contract MyToken is ERC721, Ownable {
    uint256 private _nextTokenId;
    mapping(uint256 => uint256) private _randomValues;

    constructor(address initialOwner) ERC721("MyToken", "MTK") Ownable(initialOwner) {}

    function safeMint(address to) public onlyOwner {
        uint256 tokenId = _nextTokenId++;
        uint256 randomValue = uint256(keccak256(abi.encodePacked(tokenId, block.timestamp, msg.sender)));
        _randomValues[tokenId] = randomValue;
        _safeMint(to, tokenId);
    }

    function getRandomValue(uint256 tokenId) public view returns (uint256) {
        return _randomValues[tokenId];
    }
}
