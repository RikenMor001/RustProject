// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract SimpleStorage {
    // State variable to store a number
    uint256 private storedNumber;
    
    // Event to emit when number is stored
    event NumberStored(uint256 number);
    
    // Function to store a number
    function storeNumber(uint256 _number) public {
        storedNumber = _number;
        emit NumberStored(_number);
    }
    
    // Function to retrieve the stored number
    function retrieveNumber() public view returns (uint256) {
        return storedNumber;
    }
    
    // Function to add to the stored number
    function addToNumber(uint256 _addend) public {
        storedNumber += _addend;
        emit NumberStored(storedNumber);
    }
}
