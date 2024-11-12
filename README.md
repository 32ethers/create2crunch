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
$ cargo run --release 0x0000000000ffe8b47b3e2130213b802212439497 0x48E516B34A1274f49457b9C6182097796D0498Cb 0x94d114296a5af85c1fd2dc039cdaa32f1ed4b0fe0868f02d888bfc91feb645d9 1 0 1
```

There 6 parameters:

1. Factory address: the CREATE2 factory will have a view method for checking what address you'll get for submitting a particular salt.
2. The deployer address for Uniswap v4: 0x48E516B34A1274f49457b9C6182097796D0498Cb
3. The hash of the initcode for Uniswap v4: 0x94d114296a5af85c1fd2dc039cdaa32f1ed4b0fe0868f02d888bfc91feb645d9
4. Platform id of your GPU, you can find it with ```clinfo -l```
5. Device id under platform. 
6. Number of leading zeros, the actual count will be n\*2 or n\*2+1, e.g., if you set 2, you will get address with 4 or 5 leading zeros.

For the example above, NVIDIA GeForce RTX 4070 is at platform 1 and device 0, so we set "1 0" as parameter. And if you want to mining with CPU, you can choose platform 0 and device 0.

If you mine some address successfully, you will got output like this 

```
total runtime: 0:00:20.908724308013916 (557 cycles)                     work size per cycle: 67,108,864
rate: 1784.85 million attempts per second                       total found this run: 552
current search space: d9127281xxxxxxxxc24c811f          threshold: 1 leading.
0x48e516b34a1274f49457b9c6182097796d0498cb8ae28d439f235003ec94cc46 => 0x0044444F9c3a23C669bCaDEe38b6fC20AcED1E24 (2 / 6) 66
0x48e516b34a1274f49457b9c6182097796d0498cbee1b43ae4d7b7203fd69ecee => 0x004444d75A25b93C2ed117c831D820b350F864C5 (2 / 5) 85
0x48e516b34a1274f49457b9c6182097796d0498cbb9e6b048a5bbcb03f0416a47 => 0x0004444494aE9D72E694bC7D7b9e5a61d60C5B3b (3 / 7) 77
0x48e516b34a1274f49457b9c6182097796d0498cb516dc31a84a6ed033baa96a8 => 0x004444A4CcA0DA4A01141eeB21cf1E2BdD432384 (2 / 9) 89
0x48e516b34a1274f49457b9c6182097796d0498cb8a2eb045701f9503de9af11a => 0x004444c1A9458dbF3A81bC9cCaB796400F2dd35a (2 / 6) 86
0x48e516b34a1274f49457b9c6182097796d0498cb07356a62c735a303a9a50be8 => 0x0044441AcaaAc23d780e0d37D5FcB806B62CE208 (2 / 4) 84
0x48e516b34a1274f49457b9c6182097796d0498cbefc460be07446903c190873b => 0x004444b8084166B67947B64Ba0dee3aa6b64374F (2 / 9) 89
0x48e516b34a1274f49457b9c6182097796d0498cbf9fbb8d0613409026733d5f7 => 0x0044446BDD640A96DDFe1a469c4FBA27bF7C43bf (2 / 8) 88
0x48e516b34a1274f49457b9c6182097796d0498cb82ca74f6b40f56022d4371c2 => 0x004444EBBe4554Cc037e8b38ea1003b1AFF4aBe6 (2 / 7) 87
```

Take the first address for example, first section(0x48e516b34a1274f49457b9c6182097796d0498cb8ae28d439f235003ec94cc46) is salt. then followed by address(0x0044444F9c3a23C669bCaDEe38b6fC20AcED1E24). the number ```(2/6)``` means this address has 2 leading zeros and 6 number 4. The last number 66 is the score of this address. the rule of scoring is: 

> * 10 points for each leading 0 nibble
> * 40 points if the address starts with four consecutive 4s
> * 20 points if the first nibble after the four 4s is not a 4
> * 20 points if the last four nibbles are all 4s
> * 1 point for each 4 elsewhere in the address

So, that is, good luck for the mining!