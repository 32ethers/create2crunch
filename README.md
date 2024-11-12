# create2crunch-for uniswap v4 address

A fork of create2crunch, for [uniswap v4 address mining](https://v4-address.uniswap.org/)

Will follow the rule [here](https://blog.uniswap.org/uniswap-v4-address-mining-challenge) and mine address like 0x00000044449bD64171D095726Eeb6b768Bbeb184

This app will use opencl device only, such as your CPU or GPU. 

Usage:

You have to ensure you have opencl installed first. So install clinfo to check:

```sh
$ sudo apt install clinfo
$ clinfo -l
```

If you can get output like this, then you already have opencl installed. 

```
Platform #0: Intel(R) OpenCL
 `-- Device #0: Intel(R) Xeon(R) CPU E5-2686 v4 @ 2.30GHz
Platform #1: NVIDIA CUDA
 `-- Device #0: NVIDIA GeForce RTX 4070
Platform #2: Intel(R) FPGA Emulation Platform for OpenCL(TM)
 `-- Device #0: Intel(R) FPGA Emulation Device
```

If you got nothing, you must configure opencl and GPU driver.

When opencl is ready, check your rust

```sh
$ rustc --version  
```

If rust is not installed. Install rust with this

```sh
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then you can start mining

```sh
$ git clone https://github.com/32ethers/create2crunch
$ cd create2crunch
$ cargo run --release 0x48E516B34A1274f49457b9C6182097796D0498Cb 0x88888414c16cA5793D8D239aBbab2A0e6b358568 0x94d114296a5af85c1fd2dc039cdaa32f1ed4b0fe0868f02d888bfc91feb645d9 1 0 2
```

There 6 parameters:

1. The deployer address for Uniswap v4: 0x48E516B34A1274f49457b9C6182097796D0498Cb
2. Your address, which will submit the result
3. The hash of the initcode for Uniswap v4: 0x94d114296a5af85c1fd2dc039cdaa32f1ed4b0fe0868f02d888bfc91feb645d9
4. Platform id of your GPU, you can find it with ```clinfo -l```
5. Device id under platform. 
6. Number of leading zeros, the actual count will be n\*2 or n\*2+1, e.g., if you set 2, you will get address with 4 or 5 leading zeros.

For the example above, NVIDIA GeForce RTX 4070 is at platform 1 and device 0, so we set "1 0" as parameter. And if you want to mining with CPU, you can choose platform 0 and device 0.

If you mine some address successfully, you will got output like this 

```
Setting up experimental OpenCL miner using device platform 1, device 0...
total runtime: 0:00:7.069246768951416 (190 cycles)                      work size per cycle: 67,108,864
rate: 1800.76 million attempts per second                       total found this run: 5
current search space: 5f4b7ebcxxxxxxxx2f1f5d0e          threshold: 2 leading.
0x88888414c16ca5793d8d239abbab2a0e6b358568493bc5670cdc2802810c8ae0 => 0x000044442a626dC782a8A9038d4c024703DEd20A (4 / 6) 106
0x88888414c16ca5793d8d239abbab2a0e6b358568ba7560035c942e014753d4b6 => 0x00004444bd1648b7792009cD66D33D846AecFB30 (4 / 6) 106
0x88888414c16ca5793d8d239abbab2a0e6b3585689fb0bb1ee3a3320198ea4807 => 0x00004444ED681E1a812b9eF0805e57fd78Fc50A8 (4 / 4) 104
0x88888414c16ca5793d8d239abbab2a0e6b35856865981340dbb8af03a7003c83 => 0x00004444D0852FEcCDa4D0552c0909e92801EE6e (4 / 5) 105
0x88888414c16ca5793d8d239abbab2a0e6b3585686f7109c40f4d2903b8b754aa => 0x00004444bD5DD1a99f749773Bf75128ad79858c8 (4 / 5) 105
```

Take the first address for example, first section(0x88888414c16ca5793d8d239abbab2a0e6b358568493bc5670cdc2802810c8ae0) is salt. then followed by address(0x000044442a626dC782a8A9038d4c024703DEd20A). the number ```(4/6)``` means this address has 4 leading zeros and 6 number 4. The last number 106 is the score of this address. the rule of scoring is: 

> * 10 points for each leading 0 nibble
> * 40 points if the address starts with four consecutive 4s
> * 20 points if the first nibble after the four 4s is not a 4
> * 20 points if the last four nibbles are all 4s
> * 1 point for each 4 elsewhere in the address

The mined address will also be kept in efficient_addresses.txt. so you don't have worry it would missing. 

So, that is all, good luck for the mining!