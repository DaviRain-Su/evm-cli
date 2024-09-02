// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.24;

contract CallerContract {
    // 声明目标合约的 ABI
    string private constant targetABI = "[{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_value\",\"type\":\"uint256\"}],\"name\":\"increment\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"number\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"reset\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"}]";

    // 声明目标合约地址
    address private targetAddress;

    // 构造函数，接受目标合约地址作为参数
    constructor(address _targetAddress) {
        targetAddress = _targetAddress;
    }

    // 调用目标合约函数的示例函数
    function callTargetFunction() public view returns (uint) {
        bytes memory abiBytes = bytes(targetABI); // 将 ABI 字符串转换为字节数组
        (bool success, bytes memory data) = targetAddress.staticcall(abi.encodeWithSignature(abi.decode(abiBytes, (bytes)), "number()")); // 使用函数的签名
        require(success, "Call failed");
        uint256 result;
        assembly {
            result := mload(add(data, 32))
        }
        return result;
    }
}
