// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.26;

// This is the enter point
contract ExampleCoin {
    address public minter;
    mapping(address => uint) public balances;

    event Sent(address from, address to, uint amount);

    // The constructor only can called once
    constructor() {
        minter = msg.sender;
    }

    // With the requiere, only the minter just to call it
    function mint(address receiver, uint amount) public {
        require(msg.sender == minter);
        balances[receiver] += amount;
    }

    // This is a custom error
    error InsufficientBalance(uint requested, uint available);


    // Finally, this function check the balances and transfer de funds and emit a event
    function send(address receiver, uint amount) public {
        if (amount > balances[msg.sender]) {
            revert InsufficientBalance({requested: amount, available: balances[msg.sender]});
        } else {
            balances[msg.sender] -= amount;
            balances[receiver] += amount;
            emit Sent(msg.sender, receiver, amount);
        }
    }
}