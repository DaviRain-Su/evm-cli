import { ethers } from 'ethers'
import { Console, time } from 'console'
let wbera_abi = require("./wbera.json");
let honey_abi = require("./honey.json");
let Router_abi = require("./beradexapi.json");

async function main()  {
const provider = new ethers.providers.JsonRpcProvider('https://artio.rpc.berachain.com/') //kalblaer666
const { chainId } = await provider.getNetwork()
console.log(chainId)

const poolAddress = '0xa88572f08f79d28b8f864350f122c1cc0abb0d96'
const routerAddress = "0x0d5862FDbdd12490f9b4De54c236cff63B038074";
const wberaAddress = "0x5806E416dA447b267cEA759358cF22Cc41FAE80F"
const honeyAddress = '0x7EeCA4205fF31f947EdBd49195a7A88E6A91161B'
const bankAddress = '0x0000000000000000000000000000000000696969'
const router_Contract = new ethers.Contract(routerAddress, Router_abi, provider)
const wbera_Contract = new ethers.Contract(wberaAddress, wbera_abi, provider)
const honey_Contract = new ethers.Contract(honeyAddress, honey_abi, provider)

const privateKey = '你的私钥'
const wallet = new ethers.Wallet(privateKey)
const  account_address = '你的地址'
const connectedWallet = wallet.connect(provider)


const wbera_balance_dex = await wbera_Contract.balanceOf(account_address.toLocaleLowerCase())/ 10**18;
const honey__balance_dex = await honey_Contract.balanceOf(account_address.toLocaleLowerCase())/ 10**18;

console.log(wbera_balance_dex, honey__balance_dex)
const preview = await router_Contract.getLiquidity(poolAddress)
console.log("honey: ", Number(preview[1][0])/ 10**18, "wbera:", Number(preview[1][1]/ 10**18))
const HoneyPerBera =   (Number(preview[1][0])/ 10**18) / ( Number(preview[1][1]/ 10**18  ))//1 bera 买多少honey
const BeraPerHoney = (Number(preview[1][1])/ 10**18) / ( Number(preview[1][0]/ 10**18  )) //1 honey 买多少bera
console.log(HoneyPerBera , BeraPerHoney)
var wei = ethers.utils.parseEther('1');
const getPreview = await router_Contract.getPreviewSwapExact(0, poolAddress, honeyAddress,  wei,  wberaAddress )
console.log(Number(getPreview[1])/10**18)
const GWEI = 10 ** 9
const feeData = await provider.getFeeData();
let gasPrice = feeData.gasPrice;
var nonce = await provider.getTransactionCount(account_address, 'latest')
const params = {
    kind:  0,
    poolId: poolAddress,
    assetIn: honeyAddress,
    amountIn: wei,
    assetOut: wberaAddress,
    amountOut: getPreview[1],
    deadline:  Math.floor(Date.now() / 1000) + (60 * 10),
  }
  var tx1 : any= null
    const tx = await router_Contract.connect(connectedWallet).swap(
        0,
        poolAddress,
        honeyAddress,
        wei,
        wberaAddress,
        getPreview[1],
        Math.floor(Date.now() / 1000) + (60 * 10),
        {
            'nonce': nonce,
            'gasPrice': gasPrice,
            'from': account_address.toLowerCase()}
    )
      console.log(tx)



}

main()
