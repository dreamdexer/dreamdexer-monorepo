use serde_json::{from_str, json, Value};

pub fn get_avalanche_tokens_map() -> Value {
	from_str::<Value>(
		r#"{
			"0xaeb044650278731ef3dc244692ab9f64c78ffaea": {
				"ticker": "BUSD",
				"address": "0xaEb044650278731Ef3DC244692AB9F64C78FfaEA",
				"name": "Binance USD",
				"decimals": 18
			},
			"0xb97ef9ef8734c71904d8002f8b6bc66dd9c48a6e": {
				"ticker": "USDC",
				"address": "0xB97EF9Ef8734C71904D8002F8b6Bc66Dd9c48a6E",
				"name": "USD Coin",
				"decimals": 6
			},
			"0x9702230a8ea53601f5cd2dc00fdbc13d4df4a8c7": {
				"ticker": "USDt",
				"address": "0x9702230A8Ea53601f5cD2dc00fDBc13d4dF4A8c7",
				"name": "TetherToken",
				"decimals": 6
			},
			"0xa7d7079b0fead91f3e65f86e8915cb59c1a4c664": {
				"ticker": "USDC.e",
				"address": "0xA7D7079b0FEaD91F3e65f86E8915Cb59c1a4C664",
				"name": "USD Coin",
				"decimals": 6
			},
			"0xd586e7f844cea2f87f50152665bcbc2c279d8d70": {
				"ticker": "DAI.e",
				"address": "0xd586E7F844cEa2F87f50152665BCbc2C279D8d70",
				"name": "Dai Stablecoin",
				"decimals": 18
			},
			"0x50b7545627a5162f82a992c33b87adc75187b218": {
				"ticker": "WBTC.e",
				"address": "0x50b7545627a5162F82A992c33b87aDc75187B218",
				"name": "Wrapped BTC",
				"decimals": 8
			},
			"0x49d5c2bdffac6ce2bfdb6640f4f80f226bc10bab": {
				"ticker": "WETH.e",
				"address": "0x49D5c2BdFfac6CE2BFdB6640F4F80f226bc10bAB",
				"name": "Wrapped Ether",
				"decimals": 18
			},
			"0xb31f66aa3c1e785363f0875a1b74e27b85fd66c7": {
				"ticker": "WAVAX",
				"address": "0xB31f66AA3C1e785363F0875A1B74E27b85FD66c7",
				"name": "Wrapped AVAX",
				"decimals": 18
			},
			"0x6d80113e533a2c0fe82eabd35f1875dcea89ea97": {
				"ticker": "aAvaWAVAX",
				"address": "0x6d80113e533a2C0fe82EaBD35f1875DcEA89Ea97",
				"name": "Aave Avalanche WAVAX",
				"decimals": 18
			},
			"0xc7198437980c041c805a1edcba50c1ce5db95118": {
				"ticker": "USDT.e",
				"address": "0xc7198437980c041c805A1EDcbA50c1Ce5db95118",
				"name": "Tether USD",
				"decimals": 6
			},
			"0x152b9d0fdc40c096757f570a51e494bd4b943e50": {
				"ticker": "BTC.b",
				"address": "0x152b9d0FdC40C096757F570A51E494bd4b943E50",
				"name": "Bitcoin",
				"decimals": 8
			},
			"0x5947bb275c521040051d82396192181b413227a3": {
				"ticker": "LINK.e",
				"address": "0x5947BB275c521040051D82396192181b413227A3",
				"name": "Chainlink Token",
				"decimals": 18
			},
			"0xa32608e873f9ddef944b24798db69d80bbb4d1ed": {
				"ticker": "CRA",
				"address": "0xA32608e873F9DdEF944B24798db69d80Bbb4d1ed",
				"name": "CRA",
				"decimals": 18
			},
			"0xdc42728b0ea910349ed3c6e1c9dc06b5fb591f98": {
				"ticker": "FRAX",
				"address": "0xDC42728B0eA910349ed3c6e1c9Dc06b5FB591f98",
				"name": "Frax",
				"decimals": 18
			},
			"0xb27c8941a7df8958a1778c0259f76d1f8b711c35": {
				"ticker": "KLO",
				"address": "0xb27c8941a7Df8958A1778c0259f76D1F8B711C35",
				"name": "Kalao Token",
				"decimals": 18
			},
			"0xabc9547b534519ff73921b1fba6e672b5f58d083": {
				"ticker": "WOO.e",
				"address": "0xaBC9547B534519fF73921b1FBA6E672b5f58D083",
				"name": "Wootrade Network",
				"decimals": 18
			},
			"0x69a61f38df59cbb51962e69c54d39184e21c27ec": {
				"ticker": "PARTY",
				"address": "0x69A61f38Df59CBB51962E69C54D39184E21C27Ec",
				"name": "PARTY",
				"decimals": 18
			},
			"0x191c10aa4af7c30e871e70c95db0e4eb77237530": {
				"ticker": "aAvaLINK",
				"address": "0x191c10Aa4AF7C30e871E70C95dB0E4eb77237530",
				"name": "Aave Avalanche LINK",
				"decimals": 18
			},
			"0x63a72806098bd3d9520cc43356dd78afe5d386d9": {
				"ticker": "AAVE.e",
				"address": "0x63a72806098Bd3D9520cC43356dD78afe5D386D9",
				"name": "Aave Token",
				"decimals": 18
			},
			"0x59414b3089ce2af0010e7523dea7e2b35d776ec7": {
				"ticker": "YAK",
				"address": "0x59414b3089ce2AF0010e7523Dea7E2b35d776ec7",
				"name": "Yak Token",
				"decimals": 18
			},
			"0x2147efff675e4a4ee1c2f918d181cdbd7a8e208f": {
				"ticker": "ALPHA.e",
				"address": "0x2147EFFF675e4A4eE1C2f918d181cDBd7a8E208f",
				"name": "AlphaToken",
				"decimals": 18
			},
			"0x544c42fbb96b39b21df61cf322b5edc285ee7429": {
				"ticker": "INSUR",
				"address": "0x544c42fBB96B39B21DF61cf322b5EDC285EE7429",
				"name": "Avalanche INSUR Token",
				"decimals": 18
			},
			"0x65378b697853568da9ff8eab60c13e1ee9f4a654": {
				"ticker": "HUSKY",
				"address": "0x65378b697853568dA9ff8EaB60C13E1Ee9f4a654",
				"name": "Husky",
				"decimals": 18
			},
			"0xde3a24028580884448a5397872046a019649b084": {
				"ticker": "USDT",
				"address": "0xde3A24028580884448a5397872046a019649b084",
				"name": "Tether USD",
				"decimals": 6
			},
			"0xf20d962a6c8f70c731bd838a3a388d7d48fa6e15": {
				"ticker": "ETH",
				"address": "0xf20d962a6c8f70c731bd838a3a388D7d48fA6e15",
				"name": "Ether",
				"decimals": 18
			},
			"0xe1c110e1b1b4a1ded0caf3e42bfbdbb7b5d7ce1c": {
				"ticker": "ELK",
				"address": "0xE1C110E1B1b4A1deD0cAf3E42BfBdbB7b5d7cE1C",
				"name": "Elk",
				"decimals": 18
			},
			"0xf693248f96fe03422fea95ac0afbbbc4a8fdd172": {
				"ticker": "TUS",
				"address": "0xf693248F96Fe03422FEa95aC0aFbBBc4a8FdD172",
				"name": "Treasure Under Sea",
				"decimals": 18
			},
			"0x4f60a160d8c2dddaafe16fcc57566db84d674bd6": {
				"ticker": "JEWEL",
				"address": "0x4f60a160D8C2DDdaAfe16FCC57566dB84D674BD6",
				"name": "Jewels",
				"decimals": 18
			},
			"0xba7deebbfc5fa1100fb055a87773e1e99cd3507a": {
				"ticker": "DAI",
				"address": "0xbA7dEebBFC5fA1100Fb055a87773e1E99Cd3507a",
				"name": "Dai Stablecoin",
				"decimals": 18
			},
			"0x19860ccb0a68fd4213ab9d8266f7bbf05a8dde98": {
				"ticker": "BUSD.e",
				"address": "0x19860CCB0A68fd4213aB9D8266F7bBf05A8dDe98",
				"name": "Binance USD",
				"decimals": 18
			},
			"0xc7b5d72c836e718cda8888eaf03707faef675079": {
				"ticker": "SWAP.e",
				"address": "0xc7B5D72C836e718cDA8888eaf03707fAef675079",
				"name": "TrustSwap Token",
				"decimals": 18
			},
			"0x026e91e4c3d35eb31a90fcdbf50313d0290af3cb": {
				"ticker": "XBN",
				"address": "0x026e91e4C3d35EB31a90FcdBF50313d0290Af3cb",
				"name": "Elastic BNB",
				"decimals": 18
			},
			"0x37b608519f91f70f2eeb0e5ed9af4061722e4f76": {
				"ticker": "SUSHI.e",
				"address": "0x37B608519F91f70F2EeB0e5Ed9AF4061722e4F76",
				"name": "SushiToken",
				"decimals": 18
			},
			"0x39cf1bd5f15fb22ec3d9ff86b0727afc203427cc": {
				"ticker": "SUSHI",
				"address": "0x39cf1BD5f15fb22eC3D9Ff86b0727aFc203427cc",
				"name": "SushiToken",
				"decimals": 18
			},
			"0x8ebaf22b6f053dffeaf46f4dd9efa95d89ba8580": {
				"ticker": "UNI.e",
				"address": "0x8eBAf22B6F053dFFeaf46f4Dd9eFA95D89ba8580",
				"name": "Uniswap",
				"decimals": 18
			},
			"0x90842eb834cfd2a1db0b1512b254a18e4d396215": {
				"ticker": "GB",
				"address": "0x90842eb834cFD2A1DB0b1512B254a18E4D396215",
				"name": "Good Bridging",
				"decimals": 9
			},
			"0x8ce2dee54bb9921a2ae0a63dbb2df8ed88b91dd9": {
				"ticker": "AAVE",
				"address": "0x8cE2Dee54bB9921a2AE0A63dBb2DF8eD88B91dD9",
				"name": "Aave Token",
				"decimals": 18
			},
			"0x340fe1d898eccaad394e2ba0fc1f93d27c7b717a": {
				"ticker": "ORBS",
				"address": "0x340fE1D898ECCAad394e2ba0fC1F93d27c7b717A",
				"name": "Orbs",
				"decimals": 18
			},
			"0x9eaac1b23d935365bd7b542fe22ceee2922f52dc": {
				"ticker": "YFI.e",
				"address": "0x9eAaC1B23d935365bD7b542Fe22cEEe2922f52dc",
				"name": "yearn.finance",
				"decimals": 18
			},
			"0x5fc17416925789e0852fbfcd81c490ca4abc51f9": {
				"ticker": "SURE",
				"address": "0x5fC17416925789E0852FBFcd81c490ca4abc51F9",
				"name": "inSure",
				"decimals": 18
			},
			"0x02d980a0d7af3fb7cf7df8cb35d9edbcf355f665": {
				"ticker": "SHIB.e",
				"address": "0x02D980A0D7AF3fb7Cf7Df8cB35d9eDBCF355f665",
				"name": "SHIBA INU",
				"decimals": 18
			},
			"0x408d4cd0adb7cebd1f1a1c33a0ba2098e1295bab": {
				"ticker": "WBTC",
				"address": "0x408D4cD0ADb7ceBd1F1A1C33A0Ba2098E1295bAB",
				"name": "Wrapped BTC",
				"decimals": 8
			},
			"0x88128fd4b259552a9a1d457f435a6527aab72d42": {
				"ticker": "MKR.e",
				"address": "0x88128fd4b259552A9A1D457f435a6527AAb72d42",
				"name": "Maker",
				"decimals": 18
			},
			"0xd501281565bf7789224523144fe5d98e8b28f267": {
				"ticker": "1INCH.e",
				"address": "0xd501281565bf7789224523144Fe5D98e8B28f267",
				"name": "1INCH Token",
				"decimals": 18
			},
			"0xa384bc7cdc0a93e686da9e7b8c0807cd040f4e0b": {
				"ticker": "WOW",
				"address": "0xA384Bc7Cdc0A93e686da9E7B8C0807cD040F4E0b",
				"name": "WOWswap",
				"decimals": 18
			},
			"0xb3fe5374f67d7a22886a0ee082b2e2f9d2651651": {
				"ticker": "LINK",
				"address": "0xB3fe5374F67D7a22886A0eE082b2E2f9d2651651",
				"name": "ChainLink Token",
				"decimals": 18
			},
			"0x249848beca43ac405b8102ec90dd5f22ca513c06": {
				"ticker": "CRV.e",
				"address": "0x249848BeCA43aC405b8102Ec90Dd5F22CA513c06",
				"name": "Curve DAO Token",
				"decimals": 18
			},
			"0xf39f9671906d8630812f9d9863bbef5d523c84ab": {
				"ticker": "UNI",
				"address": "0xf39f9671906d8630812f9d9863bBEf5D523c84Ab",
				"name": "Uniswap",
				"decimals": 18
			},
			"0xbec243c995409e6520d7c41e404da5deba4b209b": {
				"ticker": "SNX.e",
				"address": "0xBeC243C995409E6520D7C41E404da5dEba4b209B",
				"name": "Synthetix Network Token",
				"decimals": 18
			},
			"0x596fa47043f99a4e0f122243b841e55375cde0d2": {
				"ticker": "ZRX.e",
				"address": "0x596fA47043f99A4e0F122243B841E55375cdE0d2",
				"name": "ZRX",
				"decimals": 18
			},
			"0x98443b96ea4b0858fdf3219cd13e98c7a4690588": {
				"ticker": "BAT.e",
				"address": "0x98443B96EA4b0858FDF3219Cd13e98C7A4690588",
				"name": "Basic Attention Token",
				"decimals": 18
			},
			"0x8a0cac13c7da965a312f08ea4229c37869e85cb9": {
				"ticker": "GRT.e",
				"address": "0x8a0cAc13c7da965a312f08ea4229c37869e85cB9",
				"name": "Graph Token",
				"decimals": 18
			},
			"0x99519acb025a0e0d44c3875a4bbf03af65933627": {
				"ticker": "YFI",
				"address": "0x99519AcB025a0e0d44c3875A4BbF03af65933627",
				"name": "yearn.finance",
				"decimals": 18
			},
			"0xc3048e19e76cb9a3aa9d77d8c03c29fc906e2437": {
				"ticker": "COMP.e",
				"address": "0xc3048E19E76CB9a3Aa9d77D8C03c29Fc906e2437",
				"name": "Compound",
				"decimals": 18
			},
			"0xe54eb2c3009fa411bf24fb017f9725b973ce36f0": {
				"ticker": "1INCH",
				"address": "0xE54EB2C3009Fa411BF24fB017F9725b973CE36F0",
				"name": "1INCH Token",
				"decimals": 18
			},
			"0x3bd2b1c7ed8d396dbb98ded3aebb41350a5b2339": {
				"ticker": "UMA.e",
				"address": "0x3Bd2B1c7ED8D396dbb98DED3aEbb41350a5b2339",
				"name": "UMA Voting Token v1",
				"decimals": 18
			},
			"0xe80761ea617f66f96274ea5e8c37f03960ecc679": {
				"ticker": "variableDebtAvaAAVE",
				"address": "0xE80761Ea617F66F96274eA5e8c37f03960ecC679",
				"name": "Aave Avalanche Variable Debt AAVE",
				"decimals": 18
			},
			"0xf15f26710c827dde8acba678682f3ce24f2fb56e": {
				"ticker": "stableDebtAvaWAVAX",
				"address": "0xF15F26710c827DDe8ACBA678682F3Ce24f2Fb56E",
				"name": "Aave Avalanche Stable Debt WAVAX",
				"decimals": 18
			},
			"0xf329e36c7bf6e5e86ce2150875a84ce77f477375": {
				"ticker": "aAvaAAVE",
				"address": "0xf329e36C7bF6E5E86ce2150875a84Ce77f477375",
				"name": "Aave Avalanche AAVE",
				"decimals": 18
			},
			"0xfaef6a702d15428e588d4c0614aefb4348d83d48": {
				"ticker": "stableDebtAvaAAVE",
				"address": "0xfAeF6A702D15428E588d4C0614AEFb4348D83D48",
				"name": "Aave Avalanche Stable Debt AAVE",
				"decimals": 18
			},
			"0x4a1c3ad6ed28a636ee1751c69071f6be75deb8b8": {
				"ticker": "variableDebtAvaWAVAX",
				"address": "0x4a1c3aD6Ed28a636ee1751C69071f6be75DEb8B8",
				"name": "Aave Avalanche Variable Debt WAVAX",
				"decimals": 18
			},
			"0x078f358208685046a11c85e8ad32895ded33a249": {
				"ticker": "aAvaWBTC",
				"address": "0x078f358208685046a11C85e8ad32895DED33A249",
				"name": "Aave Avalanche WBTC",
				"decimals": 8
			},
			"0x0c84331e39d6658cd6e6b9ba04736cc4c4734351": {
				"ticker": "variableDebtAvaWETH",
				"address": "0x0c84331e39d6658Cd6e6b9ba04736cC4c4734351",
				"name": "Aave Avalanche Variable Debt WETH",
				"decimals": 18
			},
			"0xfb00ac187a8eb5afae4eace434f493eb62672df7": {
				"ticker": "variableDebtAvaUSDT",
				"address": "0xfb00AC187a8Eb5AFAE4eACE434F493Eb62672df7",
				"name": "Aave Avalanche Variable Debt USDT",
				"decimals": 6
			},
			"0xe50fa9b3c56ffb159cb0fca61f5c9d750e8128c8": {
				"ticker": "aAvaWETH",
				"address": "0xe50fA9b3c56FfB159cB0FCA61F5c9D750e8128c8",
				"name": "Aave Avalanche WETH",
				"decimals": 18
			},
			"0x6ab707aca953edaefbc4fd23ba73294241490620": {
				"ticker": "aAvaUSDT",
				"address": "0x6ab707Aca953eDAeFBc4fD23bA73294241490620",
				"name": "Aave Avalanche USDT",
				"decimals": 6
			},
			"0x70effc565db6eef7b927610155602d31b670e802": {
				"ticker": "stableDebtAvaUSDT",
				"address": "0x70eFfc565DB6EEf7B927610155602d31b670e802",
				"name": "Aave Avalanche Stable Debt USDT",
				"decimals": 6
			},
			"0x92b42c66840c7ad907b4bf74879ff3ef7c529473": {
				"ticker": "variableDebtAvaWBTC",
				"address": "0x92b42c66840C7AD907b4BF74879FF3eF7c529473",
				"name": "Aave Avalanche Variable Debt WBTC",
				"decimals": 8
			},
			"0xd8ad37849950903571df17049516a5cd4cbe55f6": {
				"ticker": "stableDebtAvaWETH",
				"address": "0xD8Ad37849950903571df17049516a5CD4cbE55F6",
				"name": "Aave Avalanche Stable Debt WETH",
				"decimals": 18
			},
			"0x953a573793604af8d41f306feb8274190db4ae0e": {
				"ticker": "variableDebtAvaLINK",
				"address": "0x953A573793604aF8d41F306FEb8274190dB4aE0e",
				"name": "Aave Avalanche Variable Debt LINK",
				"decimals": 18
			},
			"0x307ffe186f84a3bc2613d1ea417a5737d69a7007": {
				"ticker": "stableDebtAvaUSDC",
				"address": "0x307ffe186F84a3bc2613D1eA417A5737D69A7007",
				"name": "Aave Avalanche Stable Debt USDC",
				"decimals": 6
			},
			"0x89d976629b7055ff1ca02b927ba3e020f22a44e4": {
				"ticker": "stableDebtAvaLINK",
				"address": "0x89D976629b7055ff1ca02b927BA3e020F22A44e4",
				"name": "Aave Avalanche Stable Debt LINK",
				"decimals": 18
			},
			"0xfccf3cabbe80101232d343252614b6a3ee81c989": {
				"ticker": "variableDebtAvaUSDC",
				"address": "0xFCCf3cAbbe80101232d343252614b6A3eE81C989",
				"name": "Aave Avalanche Variable Debt USDC",
				"decimals": 6
			},
			"0xd94112b5b62d53c9402e7a60289c6810def1dc9b": {
				"ticker": "stableDebtAvaDAI",
				"address": "0xd94112B5B62d53C9402e7A60289c6810dEF1dC9B",
				"name": "Aave Avalanche Stable Debt DAI",
				"decimals": 18
			},
			"0x625e7708f30ca75bfd92586e17077590c60eb4cd": {
				"ticker": "aAvaUSDC",
				"address": "0x625E7708f30cA75bfd92586e17077590C60eb4cD",
				"name": "Aave Avalanche USDC",
				"decimals": 6
			},
			"0x82e64f49ed5ec1bc6e43dad4fc8af9bb3a2312ee": {
				"ticker": "aAvaDAI",
				"address": "0x82E64f49Ed5EC1bC6e43DAD4FC8Af9bb3A2312EE",
				"name": "Aave Avalanche DAI",
				"decimals": 18
			},
			"0x8619d80fb0141ba7f184cbf22fd724116d9f7ffc": {
				"ticker": "variableDebtAvaDAI",
				"address": "0x8619d80FB0141ba7F184CbF22fd724116D9f7ffC",
				"name": "Aave Avalanche Variable Debt DAI",
				"decimals": 18
			},
			"0xd2cd7a59aa8f8fdc68d01b1e8a95747730b927d3": {
				"ticker": "CRAM",
				"address": "0xD2cd7a59Aa8f8FDc68d01b1e8A95747730b927d3",
				"name": "Crabada Amulet",
				"decimals": 18
			},
			"0x431de0736f523c2d974b5698dbce2707871d04b6": {
				"ticker": "CRA-X",
				"address": "0x431De0736f523c2D974B5698dbcE2707871D04B6",
				"name": "CRA-X",
				"decimals": 18
			},
			"0x3c1bb39bb696b443a1d80bb2b3a3d950ba9dee87": {
				"ticker": "BiFi",
				"address": "0x3C1BB39bb696B443a1D80BB2b3a3d950Ba9DEE87",
				"name": "BiFi",
				"decimals": 18
			},
			"0x3bd2dfd03bc7c3011ed7fb8c4d0949b382726cee": {
				"ticker": "STACK",
				"address": "0x3BD2dFd03BC7c3011ed7fb8c4d0949B382726cee",
				"name": "StackOS",
				"decimals": 18
			},
			"0x2920cd5b8a160b2addb00ec5d5f4112255d4ae75": {
				"ticker": "stableDebtvWAVAX",
				"address": "0x2920CD5b8A160b2Addb00Ec5d5f4112255d4ae75",
				"name": "Aave Avalanche Market stable debt WAVAX",
				"decimals": 18
			},
			"0x66a0fe52fb629a6cb4d10b8580afdffe888f5fd4": {
				"ticker": "variableDebtvWAVAX",
				"address": "0x66A0FE52Fb629a6cB4D10B8580AFDffE888F5Fd4",
				"name": "Aave Avalanche Market variable debt vWAVAX",
				"decimals": 18
			},
			"0xdfe521292ece2a4f44242efbcd66bc594ca9714b": {
				"ticker": "avWAVAX",
				"address": "0xDFE521292EcE2A4f44242efBcD66Bc594CA9714B",
				"name": "Aave Avalanche Market WAVAX",
				"decimals": 18
			},
			"0x3484408989985d68c9700dc1cfdfeae6d2f658cf": {
				"ticker": "stableDebtvWBTC",
				"address": "0x3484408989985d68C9700dc1CFDFeAe6d2f658CF",
				"name": "Aave Avalanche Market stable debt WBTC",
				"decimals": 8
			},
			"0x2dc0e35ec3ab070b8a175c829e23650ee604a9eb": {
				"ticker": "variableDebtvWBTC",
				"address": "0x2dc0E35eC3Ab070B8a175C829e23650Ee604a9eB",
				"name": "Aave Avalanche Market variable debt vWBTC",
				"decimals": 8
			},
			"0x686bef2417b6dc32c50a3cbfbcc3bb60e1e9a15d": {
				"ticker": "avWBTC",
				"address": "0x686bEF2417b6Dc32C50a3cBfbCC3bb60E1e9a15D",
				"name": "Aave Avalanche Market WBTC",
				"decimals": 8
			},
			"0x8352e3fd18b8d84d3c8a1b538d788899073c7a8e": {
				"ticker": "variableDebtvAAVE",
				"address": "0x8352E3fd18B8d84D3c8a1b538d788899073c7A8E",
				"name": "Aave Avalanche Market variable debt vAAVE",
				"decimals": 18
			},
			"0xd45b7c061016102f9fa220502908f2c0f1add1d7": {
				"ticker": "avAAVE",
				"address": "0xD45B7c061016102f9FA220502908f2c0f1add1D7",
				"name": "Aave Avalanche Market AAVE",
				"decimals": 18
			},
			"0x66904e4f3f44e3925d22ceca401b6f2da085c98f": {
				"ticker": "stableDebtvAAVE",
				"address": "0x66904E4F3f44e3925D22ceca401b6F2DA085c98f",
				"name": "Aave Avalanche Market stable debt AAVE",
				"decimals": 18
			},
			"0x848c080d2700cbe1b894a3374ad5e887e5ccb89c": {
				"ticker": "variableDebtvUSDC",
				"address": "0x848c080d2700CBE1B894a3374AD5E887E5cCb89c",
				"name": "Aave Avalanche Market variable debt vUSDC",
				"decimals": 6
			},
			"0x5b14679135dbe8b02015ec3ca4924a12e4c6c85a": {
				"ticker": "stableDebtvUSDC",
				"address": "0x5B14679135dbE8B02015ec3Ca4924a12E4C6C85a",
				"name": "Aave Avalanche Market stable debt USDC",
				"decimals": 6
			},
			"0x46a51127c3ce23fb7ab1de06226147f446e4a857": {
				"ticker": "avUSDC",
				"address": "0x46A51127C3ce23fb7AB1DE06226147F446e4a857",
				"name": "Aave Avalanche Market USDC",
				"decimals": 6
			},
			"0x3676e4ee689d527ddb89812b63fad0b7501772b3": {
				"ticker": "stableDebtvDAI",
				"address": "0x3676E4EE689D527dDb89812B63fAD0B7501772B3",
				"name": "Aave Avalanche Market stable debt DAI",
				"decimals": 18
			},
			"0x47afa96cdc9fab46904a55a6ad4bf6660b53c38a": {
				"ticker": "avDAI",
				"address": "0x47AFa96Cdc9fAb46904A55a6ad4bf6660B53c38a",
				"name": "Aave Avalanche Market DAI",
				"decimals": 18
			},
			"0x1852dc24d1a8956a0b356aa18ede954c7a0ca5ae": {
				"ticker": "variableDebtvDAI",
				"address": "0x1852DC24d1a8956a0B356AA18eDe954c7a0Ca5ae",
				"name": "Aave Avalanche Market variable debt vDAI",
				"decimals": 18
			},
			"0x60f6a45006323b97d97cb0a42ac39e2b757ada63": {
				"ticker": "stableDebtvWETH",
				"address": "0x60F6A45006323B97d97cB0a42ac39e2b757ADA63",
				"name": "Aave Avalanche Market stable debt WETH",
				"decimals": 18
			},
			"0x53f7c5869a859f0aec3d334ee8b4cf01e3492f21": {
				"ticker": "avWETH",
				"address": "0x53f7c5869a859F0AeC3D334ee8B4Cf01E3492f21",
				"name": "Aave Avalanche Market WETH",
				"decimals": 18
			},
			"0x4e575cacb37bc1b5afec68a0462c4165a5268983": {
				"ticker": "variableDebtvWETH",
				"address": "0x4e575CacB37bc1b5afEc68a0462c4165A5268983",
				"name": "Aave Avalanche Market variable debt vWETH",
				"decimals": 18
			},
			"0xc1a49c0b9c10f35850bd8e15eaef0346be63e002": {
				"ticker": "DUEL",
				"address": "0xc1a49c0B9C10F35850bd8E15EaeF0346BE63E002",
				"name": "Duel Network",
				"decimals": 18
			},
			"0x3d51a9fb5dcc87f7b237b04975559b920a9a56ff": {
				"ticker": "WNAV",
				"address": "0x3D51a9fB5dCc87F7B237B04975559b920a9a56Ff",
				"name": "Wrapped Navcoin",
				"decimals": 8
			},
			"0xd185c562306cb257a53c6b9d7287ebed9b1bb410": {
				"ticker": "ELP",
				"address": "0xd185C562306CB257A53C6b9D7287EBED9B1BB410",
				"name": "Elk Liquidity",
				"decimals": 18
			},
			"0x490c69b3a746a10b163f1e9a5674f2057d3d956f": {
				"ticker": "ELP",
				"address": "0x490C69b3A746A10B163f1E9A5674F2057d3D956F",
				"name": "Elk Liquidity",
				"decimals": 18
			},
			"0x563d2d28ea10691bae85838d1ee8f1397217b252": {
				"ticker": "ELP",
				"address": "0x563d2D28ea10691bae85838d1eE8F1397217b252",
				"name": "Elk Liquidity",
				"decimals": 18
			},
			"0xdfff750529a2eaba8b13e1b81f054ede83ca52a2": {
				"ticker": "ELP",
				"address": "0xDfFF750529A2eABA8B13E1b81F054ede83cA52a2",
				"name": "Elk Liquidity",
				"decimals": 18
			},
			"0x2930b7cffd8f458330f4f66e3fb336ccfd125176": {
				"ticker": "ELP",
				"address": "0x2930B7CFfd8F458330f4F66E3FB336cCFd125176",
				"name": "Elk Liquidity",
				"decimals": 18
			},
			"0xf21887c326c7bc061504b1fe3a2d15d4d19ffd51": {
				"ticker": "ELP",
				"address": "0xf21887C326C7BC061504b1fe3A2D15D4D19ffd51",
				"name": "Elk Liquidity",
				"decimals": 18
			},
			"0x2b0aedac7f88c5787b05dd09f96b9819c6b92c2c": {
				"ticker": "ELP",
				"address": "0x2B0AEdac7f88C5787b05dd09F96b9819C6b92c2C",
				"name": "Elk Liquidity",
				"decimals": 18
			},
			"0xca94033d7a0b14bc6377fb0bf1c56218d24eb68d": {
				"ticker": "ELP",
				"address": "0xca94033d7a0B14bc6377fb0Bf1c56218d24Eb68D",
				"name": "Elk Liquidity",
				"decimals": 18
			},
			"0x957c17a32a88028b3e0cfa995fca0c016b6b7ed9": {
				"ticker": "ELP",
				"address": "0x957C17a32A88028B3E0CFa995fCa0C016b6b7ED9",
				"name": "Elk Liquidity",
				"decimals": 18
			},
			"0xb11b720d574ea4888dc8b3cd3e7149774bd977c9": {
				"ticker": "ELP",
				"address": "0xb11B720D574Ea4888DC8b3cd3e7149774bd977c9",
				"name": "Elk Liquidity",
				"decimals": 18
			},
			"0xaf91700c900a1c348b277be5adcd9458c3007254": {
				"ticker": "ELP",
				"address": "0xaf91700c900A1c348b277Be5AdCD9458C3007254",
				"name": "Elk Liquidity",
				"decimals": 18
			},
			"0x1aeb1def5b064df8e4470e57af17df72961a9ef8": {
				"ticker": "ELP",
				"address": "0x1AEb1DeF5B064Df8e4470E57Af17dF72961A9eF8",
				"name": "Elk Liquidity",
				"decimals": 18
			},
			"0xf5e04586ad7d9cf947b65a81c8e4bd7517480afa": {
				"ticker": "ELP",
				"address": "0xF5e04586AD7d9cf947B65A81C8e4Bd7517480aFA",
				"name": "Elk Liquidity",
				"decimals": 18
			},
			"0x29f0293c64c02a997f123309431272834bf3bb29": {
				"ticker": "ELP",
				"address": "0x29f0293C64c02a997F123309431272834BF3bB29",
				"name": "Elk Liquidity",
				"decimals": 18
			},
			"0xa8de0766a02535c2b2749088b8c9dd029afbc233": {
				"ticker": "ELP",
				"address": "0xA8De0766A02535c2b2749088B8C9Dd029afBC233",
				"name": "Elk Liquidity",
				"decimals": 18
			},
			"0x27f382534916520f178bf9e201db4cd8172cc7d2": {
				"ticker": "ELP",
				"address": "0x27f382534916520f178bf9e201db4CD8172CC7D2",
				"name": "Elk Liquidity",
				"decimals": 18
			},
			"0x6715adb3f8b8001fbcd049b683db05cd406a9f7f": {
				"ticker": "ELP",
				"address": "0x6715adB3f8B8001fbcd049b683DB05CD406a9f7F",
				"name": "Elk Liquidity",
				"decimals": 18
			},
			"0xdc4000acbe8f9d6aa81696657927588cca2823ec": {
				"ticker": "ELP",
				"address": "0xDc4000aCBE8f9D6aA81696657927588ccA2823Ec",
				"name": "Elk Liquidity",
				"decimals": 18
			},
			"0x84cf8ef74974399b4473bcf474507fe9557250ab": {
				"ticker": "ELP",
				"address": "0x84Cf8Ef74974399b4473bCF474507FE9557250ab",
				"name": "Elk Liquidity",
				"decimals": 18
			},
			"0x024b440d5eea2d95f90425186e9c6d83e0ea67ef": {
				"ticker": "ELP",
				"address": "0x024B440D5eeA2D95f90425186E9c6d83e0Ea67eF",
				"name": "Elk Liquidity",
				"decimals": 18
			},
			"0xed8c2af5c9f7a0fa5aaa0d8f8e78ddca28eef3d6": {
				"ticker": "YRT",
				"address": "0xeD8c2af5c9F7A0fa5AAa0D8F8E78DDCa28eef3D6",
				"name": "Yield Yak: JLP JOE-AVAX",
				"decimals": 18
			},
			"0x0e6ad18b2558ae80279d051c76ba783ffe17ad9f": {
				"ticker": "ELP",
				"address": "0x0e6AD18B2558aE80279D051c76bA783FFE17AD9f",
				"name": "Elk Liquidity",
				"decimals": 18
			},
			"0x0c7457b05d4cb71e401982091e4a61e52c632134": {
				"ticker": "ELP",
				"address": "0x0c7457b05d4Cb71e401982091E4A61e52c632134",
				"name": "Elk Liquidity",
				"decimals": 18
			},
			"0xd6e8a9784ad740d449209f4c684685672fa37e0d": {
				"ticker": "ELP",
				"address": "0xD6e8A9784ad740d449209F4C684685672FA37e0d",
				"name": "Elk Liquidity",
				"decimals": 18
			},
			"0x70ac8e9f324800e7f20b31ecea9210e0d2b4b4b9": {
				"ticker": "ELP",
				"address": "0x70aC8E9F324800e7f20B31EceA9210E0D2B4B4b9",
				"name": "Elk Liquidity",
				"decimals": 18
			},
			"0xc73db3e26c4a4deabcddefa77144136b260d08cc": {
				"ticker": "ELP",
				"address": "0xC73db3E26c4A4DeabcdDEFA77144136b260d08Cc",
				"name": "Elk Liquidity",
				"decimals": 18
			},
			"0x5e4f035f22098fd7480d97bba26c88746a9f7595": {
				"ticker": "ELP",
				"address": "0x5e4f035F22098fD7480d97Bba26c88746a9f7595",
				"name": "Elk Liquidity",
				"decimals": 18
			},
			"0x237690b5df9406f5e13c541d614c5a150d832c37": {
				"ticker": "ELP",
				"address": "0x237690b5dF9406F5e13c541d614C5a150d832c37",
				"name": "Elk Liquidity",
				"decimals": 18
			},
			"0xb650eb6fd574f3c8b7174ed5a4d426a7e38d9147": {
				"ticker": "ELP",
				"address": "0xb650eB6FD574f3c8b7174Ed5A4d426a7e38d9147",
				"name": "Elk Liquidity",
				"decimals": 18
			},
			"0x84ec9ed16e6c2bc09380866a44b29b2d447bce5e": {
				"ticker": "ELP",
				"address": "0x84EC9eD16E6C2bC09380866a44b29b2d447BCe5e",
				"name": "Elk Liquidity",
				"decimals": 18
			},
			"0xf70aaac7892d70d8473d902e47763af8d8be5607": {
				"ticker": "ELP",
				"address": "0xF70aaaC7892D70D8473D902e47763Af8D8be5607",
				"name": "Elk Liquidity",
				"decimals": 18
			},
			"0xbf07613489ab0fb9a9282329594678e1d0587c06": {
				"ticker": "ELP",
				"address": "0xbF07613489Ab0Fb9a9282329594678E1d0587c06",
				"name": "Elk Liquidity",
				"decimals": 18
			},
			"0x172fc4f87ab39864420cc8e167dab16bc79d07ef": {
				"ticker": "ELP",
				"address": "0x172fC4f87Ab39864420CC8e167Dab16BC79d07eF",
				"name": "Elk Liquidity",
				"decimals": 18
			},
			"0x8ea0328052f094e8136ca91fab08d07ddbb1366f": {
				"ticker": "ELP",
				"address": "0x8ea0328052F094E8136cA91faB08d07DDbb1366f",
				"name": "Elk Liquidity",
				"decimals": 18
			},
			"0x178d62fb9b23777fce2c02ed465396fe233172f6": {
				"ticker": "ELP",
				"address": "0x178D62Fb9B23777Fce2c02eD465396FE233172F6",
				"name": "Elk Liquidity",
				"decimals": 18
			},
			"0xa7ab0dbd8a01d392bf2d5aface3b37419508c601": {
				"ticker": "ELP",
				"address": "0xA7Ab0DBd8a01D392Bf2D5AFAcE3b37419508c601",
				"name": "Elk Liquidity",
				"decimals": 18
			},
			"0x8a83e2e3a18dace06f628a6048eea76e63468909": {
				"ticker": "ELP",
				"address": "0x8a83e2E3a18DacE06f628A6048EEA76E63468909",
				"name": "Elk Liquidity",
				"decimals": 18
			},
			"0x144082d87a10ab909c368abe017600cfef27a1f6": {
				"ticker": "ELP",
				"address": "0x144082d87a10AB909c368ABe017600cfef27A1f6",
				"name": "Elk Liquidity",
				"decimals": 18
			},
			"0xf02ae7c1ec564ec7e12e3fbb8673fc5699b0dbda": {
				"ticker": "ELP",
				"address": "0xf02aE7c1ec564ec7E12e3FBB8673Fc5699b0DbdA",
				"name": "Elk Liquidity",
				"decimals": 18
			},
			"0x2612da8fc26efbca3cc3f8fd543bcba72b10ab59": {
				"ticker": "ELP",
				"address": "0x2612dA8fc26Efbca3cC3F8fD543BCBa72b10aB59",
				"name": "Elk Liquidity",
				"decimals": 18
			},
			"0x7832928758afd045ce857afe5b6320466b5c05ed": {
				"ticker": "YRT",
				"address": "0x7832928758afD045cE857aFe5b6320466b5c05ed",
				"name": "Yield Yak: PGL PNG-SPORE",
				"decimals": 18
			},
			"0xe285a5cb85297a10fb65eb8fedbf3900ec242db8": {
				"ticker": "YRT",
				"address": "0xE285a5Cb85297a10fB65EB8FEdbf3900eC242dB8",
				"name": "Yield Yak: PGL CYCLE-AVAX",
				"decimals": 18
			},
			"0xa295bd4bbb49959e9ca1c85f0f1200b85d0a5fa5": {
				"ticker": "ELP",
				"address": "0xa295bd4BbB49959e9Ca1C85F0F1200b85d0A5FA5",
				"name": "Elk Liquidity",
				"decimals": 18
			},
			"0xb34fe8a87dfebd5ab0a03db73f2d49b903e63db6": {
				"ticker": "YRT",
				"address": "0xb34fE8A87DFEbD5Ab0a03DB73F2d49b903E63DB6",
				"name": "Yield Yak: PGL AVME-AVAX",
				"decimals": 18
			},
			"0x8c325c11e06619ba0825010fed6d565b4fddc887": {
				"ticker": "YRT",
				"address": "0x8c325C11e06619Ba0825010feD6D565b4FddC887",
				"name": "Yield Yak: PGL PNG-DAI",
				"decimals": 18
			},
			"0xf1854dce74e2e354ce6acb5daaa77a93690820d4": {
				"ticker": "YRT",
				"address": "0xf1854dCe74e2e354ce6Acb5dAAa77A93690820D4",
				"name": "Yield Yak: PGL PNG-WBTC",
				"decimals": 18
			},
			"0xb9c5ae872e946bb2b39f1cfbeb2749830ccf20fa": {
				"ticker": "YRT",
				"address": "0xB9C5AE872E946bB2b39f1CFBEb2749830cCf20fa",
				"name": "Yield Yak: PGL PNG-USDT",
				"decimals": 18
			},
			"0xe867ec0fa1969380960fe544f2a8d5758a26e254": {
				"ticker": "YRT",
				"address": "0xe867eC0fa1969380960fE544F2A8d5758A26E254",
				"name": "Yield Yak: PGL PNG-ETH",
				"decimals": 18
			},
			"0xbd8b2bc5caab6843365084b529905964d0626248": {
				"ticker": "cCNR",
				"address": "0xbd8B2bC5CaAb6843365084b529905964d0626248",
				"name": "Compounding CNR",
				"decimals": 18
			},
			"0x562acea3c03dbddc25e2f24bb2685d17bdb4e62f": {
				"ticker": "YRT",
				"address": "0x562ACEA3c03dBDDc25e2F24bb2685D17Bdb4e62f",
				"name": "Compounding XAVA",
				"decimals": 18
			},
			"0xb667121b4d4b6ea5de4bb61bd3a02e53529bfcca": {
				"ticker": "YRT",
				"address": "0xb667121B4D4b6ea5DE4bb61bd3a02E53529BfcCA",
				"name": "Yield Yak: BGL BAG-XAVA",
				"decimals": 18
			},
			"0x39f7fcb3af11b0a274514c581d468739e75f64ec": {
				"ticker": "YRT",
				"address": "0x39F7fCB3aF11B0a274514C581d468739e75f64EC",
				"name": "Yield Yak: BGL AVAX-XAVA",
				"decimals": 18
			},
			"0xb4026833ead174668b78f612a971817382623379": {
				"ticker": "ELP",
				"address": "0xB4026833eAd174668b78F612a971817382623379",
				"name": "Elk Liquidity",
				"decimals": 18
			},
			"0x9b3365cf1927c29fa6c1d9bbd214a6bf7b893a06": {
				"ticker": "YRT",
				"address": "0x9B3365CF1927c29Fa6C1d9bbD214A6bf7b893a06",
				"name": "Yield Yak: PGL XAVA-AVAX",
				"decimals": 18
			},
			"0x9c6d01f5f13cab51af4167c4350acce4483c5a11": {
				"ticker": "ELP",
				"address": "0x9C6d01F5f13CAB51AF4167c4350aCCE4483C5A11",
				"name": "Elk Liquidity",
				"decimals": 18
			},
			"0x2cfb60bd96b5215437c3e9d2cab218fce29f096c": {
				"ticker": "YRT",
				"address": "0x2cFB60bD96B5215437c3e9D2cAB218fcE29F096C",
				"name": "Yield Yak: PGL PNG-VSO",
				"decimals": 18
			},
			"0xbfbbeb1bde34c53533a38cfbb07c37ffa06502a5": {
				"ticker": "YRT",
				"address": "0xbFBbEB1Bde34C53533A38cFBb07c37fFA06502a5",
				"name": "Yield Yak: PGL AVAX-VSO",
				"decimals": 18
			},
			"0xe97e85a8cc54980e0370faa196dae25e43b2aef0": {
				"ticker": "YRT",
				"address": "0xe97e85a8cc54980e0370faA196DAE25E43b2aef0",
				"name": "Yield Yak: PGL PEFI-LINK",
				"decimals": 18
			},
			"0x8df0790e9b9f06add676b53136bd9734e4916d59": {
				"ticker": "YRT",
				"address": "0x8DF0790E9b9f06AdD676b53136Bd9734e4916D59",
				"name": "Yield Yak: Gondola WBTC-zBTC",
				"decimals": 18
			},
			"0x86d919752822fcf060307201808e8f206ad10324": {
				"ticker": "YRT",
				"address": "0x86D919752822Fcf060307201808e8f206Ad10324",
				"name": "Yield Yak: ZERO-LP ZERO-ZUSDT",
				"decimals": 18
			},
			"0xb2da66e3245d47b67ea0d98b8789c614e2e3a76d": {
				"ticker": "YRT",
				"address": "0xb2dA66e3245d47b67Ea0d98B8789C614e2e3A76D",
				"name": "Yield Yak: ZERO-LP AVAX-ZERO",
				"decimals": 18
			},
			"0x7924612c49084b71eaa383c3aeb8a7e7eeb1a63e": {
				"ticker": "YRT",
				"address": "0x7924612c49084b71eaA383C3aeB8A7e7EEb1A63e",
				"name": "Yield Yak: ZERO-LP CHART-zUSDC",
				"decimals": 18
			},
			"0xc92fac369925f6f6e88384c2a6f3678977dc6213": {
				"ticker": "YRT",
				"address": "0xC92fAc369925f6f6e88384c2A6f3678977dC6213",
				"name": "Yield Yak: ZERO-LP AVAX-CHART",
				"decimals": 18
			},
			"0x81dbdb246fa1bb98a85c59eaf9face97403b6c19": {
				"ticker": "YRT",
				"address": "0x81dBdb246Fa1bB98a85C59eAF9face97403b6C19",
				"name": "Yield Yak: ZERO-LP ZERO-CHART",
				"decimals": 18
			},
			"0x4e42c97efae0777643938262f43c247b702ad7c6": {
				"ticker": "YRT",
				"address": "0x4e42c97EFae0777643938262f43c247b702Ad7c6",
				"name": "Yield Yak: PGL AVAX-USDT",
				"decimals": 18
			},
			"0x43c1d15c34d73eb6dfd2bf39bc53c0a0b5724b17": {
				"ticker": "YRT",
				"address": "0x43C1d15C34D73eb6dFD2bf39BC53C0a0b5724b17",
				"name": "Yield Yak: PGL BIRD-USDT",
				"decimals": 18
			},
			"0xf0c2d1d6a3d7fb3d1ddc2468fcf1d3b1ab648dac": {
				"ticker": "YRT",
				"address": "0xF0c2d1D6A3D7Fb3d1Ddc2468fCF1d3B1AB648DaC",
				"name": "Yield Yak: PGL BIRD-AVAX",
				"decimals": 18
			},
			"0xb42cd0bea11ad9ec610f2bd5826463bb93396538": {
				"ticker": "YRT",
				"address": "0xB42cD0bea11ad9eC610F2bd5826463Bb93396538",
				"name": "Yield Yak: Compounding Gondola",
				"decimals": 18
			},
			"0x9968f0c68b9d812c226f3d6eb3cd3c958979e1af": {
				"ticker": "YRT",
				"address": "0x9968f0c68B9D812C226F3d6eb3cd3c958979E1af",
				"name": "Yield Yak: Gondola USDT-zUSDT",
				"decimals": 18
			},
			"0x0caaa919feeb50029f99b288e43a73c66178c976": {
				"ticker": "YRT",
				"address": "0x0cAAA919FEeb50029f99b288e43a73c66178C976",
				"name": "Yield Yak: Gondola ETH-zETH",
				"decimals": 18
			},
			"0x57a6e0cef1e32bee5407bdcab57de10eeb383aa6": {
				"ticker": "YRT",
				"address": "0x57A6E0CEF1E32BeE5407BDcaB57dE10eEb383aA6",
				"name": "Yield Yak: Gondola DAI-zDAI",
				"decimals": 18
			},
			"0x0bd7cddb6b76eeaa8aaaec124e7ffe0d26496d3e": {
				"ticker": "YRT",
				"address": "0x0Bd7cdDB6B76eeAA8AAaEC124e7Ffe0d26496D3e",
				"name": "Yield Yak: PGL PNG-LINK",
				"decimals": 18
			},
			"0xc474e6cae8b7c7e43b3c69125a7226a4a08a4229": {
				"ticker": "YRT",
				"address": "0xc474e6cAE8b7c7E43b3c69125A7226A4A08a4229",
				"name": "Yield Yak: PGL PEFI-DAI",
				"decimals": 18
			},
			"0x3b23e8a535b2a9e4c73f617c1c5a36299b3c57b2": {
				"ticker": "YRT",
				"address": "0x3b23E8a535B2a9e4C73f617c1c5A36299b3C57B2",
				"name": "Yield Yak: PGL AVAX-SNOB",
				"decimals": 18
			},
			"0x9550593ea20359a4efeb914c1fff2d123ec9c091": {
				"ticker": "YRT",
				"address": "0x9550593eA20359A4EfeB914C1FFf2d123EC9C091",
				"name": "Yield Yak: PGL PNG-SNOB",
				"decimals": 18
			},
			"0xf947758148e056cd8096efd07f536b59f3247206": {
				"ticker": "YRT",
				"address": "0xF947758148e056Cd8096eFD07f536B59f3247206",
				"name": "Yield Yak: Lydia-LP LYD-SNOB",
				"decimals": 18
			},
			"0x73a5e6752205688889c684349270e430bd36af8c": {
				"ticker": "ELP",
				"address": "0x73A5e6752205688889C684349270E430bd36AF8c",
				"name": "Elk Liquidity",
				"decimals": 18
			},
			"0xdf98af02258e7f32a4feb77c159a42bd64d204d3": {
				"ticker": "YRT",
				"address": "0xDf98Af02258E7F32A4fEb77c159A42Bd64D204d3",
				"name": "Yield Yak: ZERO-LP GDL-ZERO",
				"decimals": 18
			},
			"0xfdd8b1444b3a929979d84e2255ab52cb9f7b5a4c": {
				"ticker": "YRT",
				"address": "0xfdd8B1444B3A929979d84E2255Ab52cb9F7B5a4C",
				"name": "Yield Yak: PGL GDL-AVAX",
				"decimals": 18
			},
			"0x90e24a2dfd80f02d01c7b630e8e3199c8a0388d3": {
				"ticker": "YRT",
				"address": "0x90E24A2dfd80F02D01c7B630E8e3199C8A0388D3",
				"name": "Yield Yak: BGL BAG-LINK",
				"decimals": 18
			},
			"0x165fa1023429e266cd767845e8de419ce3abd379": {
				"ticker": "YRT",
				"address": "0x165FA1023429E266Cd767845E8dE419Ce3abd379",
				"name": "Yield Yak: BGL BAG-USDT",
				"decimals": 18
			},
			"0xbd9f16eee869808bf22823427d1f4a1e7a440e8d": {
				"ticker": "YRT",
				"address": "0xBD9f16EeE869808bF22823427D1f4a1E7A440E8D",
				"name": "Yield Yak: BGL BAG-ETH",
				"decimals": 18
			},
			"0x8f871d05d7afb9daffa5df13a91c74e870e6c31e": {
				"ticker": "YRT",
				"address": "0x8F871D05d7AfB9dAffA5Df13A91c74e870e6c31E",
				"name": "Yield Yak: BGL BAG-WBTC",
				"decimals": 18
			},
			"0x8bf6402afcfe11519947829af44770fa44a01949": {
				"ticker": "YRT",
				"address": "0x8Bf6402AfcfE11519947829Af44770Fa44A01949",
				"name": "Yield Yak: BGL BAG-DAI",
				"decimals": 18
			},
			"0xfd1f86448b56942c32b954092f2fdbce91e37bf6": {
				"ticker": "YRT",
				"address": "0xFD1F86448b56942C32B954092F2fDBCE91E37Bf6",
				"name": "Yield Yak: BGL AVAX-USDT",
				"decimals": 18
			},
			"0xa004785b6a53dd1f523d5519b5efdc619b6b92c5": {
				"ticker": "YRT",
				"address": "0xA004785b6a53dd1f523d5519b5EFDC619B6b92c5",
				"name": "Yield Yak: BGL AVAX-DAI",
				"decimals": 18
			},
			"0xfb5aa7660fde5013996fd72a193accf00212af32": {
				"ticker": "YRT",
				"address": "0xfb5Aa7660fDe5013996FD72a193ACCF00212Af32",
				"name": "Yield Yak: BGL AVAX-LINK",
				"decimals": 18
			},
			"0x8c3c86bea8ed5acbce4944def6731291eb193c26": {
				"ticker": "YRT",
				"address": "0x8C3C86bEA8eD5ACbCE4944deF6731291Eb193C26",
				"name": "Yield Yak: BGL AVAX-ETH",
				"decimals": 18
			},
			"0xfc47515433ee291e692958a2d15f99896fafc0bc": {
				"ticker": "YRT",
				"address": "0xFC47515433eE291E692958a2D15F99896FAFC0BC",
				"name": "Yield Yak: BGL AVAX-WBTC",
				"decimals": 18
			},
			"0x908698b561ea14f153ddd1ee02f99ebe0a4cea0f": {
				"ticker": "YRT",
				"address": "0x908698B561eA14f153dDD1Ee02f99EBE0A4cea0f",
				"name": "Yield Yak: BGL BAG-AVAX",
				"decimals": 18
			},
			"0x8d325d788140c391fcb55a45e19bb4c66ef87529": {
				"ticker": "YRT",
				"address": "0x8D325d788140c391Fcb55A45E19bB4c66Ef87529",
				"name": "Yield Yak: PGL ETH-AVAX",
				"decimals": 18
			},
			"0x976eeea238514fd13a4ab3e4cb45efa17f9426a7": {
				"ticker": "YRT",
				"address": "0x976EEea238514fd13A4aB3E4cB45EfA17F9426A7",
				"name": "Yield Yak: PGL SUSHI-PEFI",
				"decimals": 18
			},
			"0x7a43d565837274ea2f6e6d95f4020871669c13c9": {
				"ticker": "YRT",
				"address": "0x7A43D565837274eA2f6E6d95F4020871669C13C9",
				"name": "Yield Yak: Lydia-LP LYD-ETH",
				"decimals": 18
			},
			"0xf752325d33f748115660b8661ab7de10cfd1997c": {
				"ticker": "YRT",
				"address": "0xf752325D33f748115660B8661ab7DE10cfd1997C",
				"name": "Yield Yak: Lydia-LP AVAX-SUSHI",
				"decimals": 18
			},
			"0x2ee33e53ebd75222c7c62f1e9feb0bc9766136ba": {
				"ticker": "YRT",
				"address": "0x2eE33e53eBD75222c7c62F1e9FEB0bc9766136ba",
				"name": "Yield Yak: Lydia-LP AVAX-ETH",
				"decimals": 18
			},
			"0x104a9f8c3a1f3bf0814105e1fd457cd7775979ce": {
				"ticker": "YRT",
				"address": "0x104a9f8C3a1f3bF0814105E1fD457CD7775979cE",
				"name": "Yield Yak: Lydia-LP LYD-PNG",
				"decimals": 18
			},
			"0x069e889d729d4d96dae10c86a4d1f629ad81adef": {
				"ticker": "YRT",
				"address": "0x069E889D729D4D96Dae10C86a4D1F629ad81adEf",
				"name": "Yield Yak: Lydia-LP LYD-USDT",
				"decimals": 18
			},
			"0x1ad3be171a6be9f6b9a60c30c5373185bd9c0b6e": {
				"ticker": "cBAMBOO",
				"address": "0x1Ad3bE171A6be9f6b9a60C30c5373185bD9c0B6e",
				"name": "Compounding Bamboo",
				"decimals": 18
			},
			"0x6b128f920813df43ac91d83ed8a79b08c99a41db": {
				"ticker": "YRT",
				"address": "0x6b128f920813dF43Ac91D83ed8A79b08C99a41db",
				"name": "Yield Yak: PGL SNOB-PEFI",
				"decimals": 18
			},
			"0xbd42169e094cd8b95513cb3640e9d37418258e17": {
				"ticker": "YRT",
				"address": "0xbD42169E094CD8b95513cB3640e9D37418258E17",
				"name": "Yield Yak: PGL PNG-PEFI",
				"decimals": 18
			},
			"0x579cf87201c82b21ba9ae29678b812e07fccf14c": {
				"ticker": "YRT",
				"address": "0x579cF87201C82B21ba9ae29678B812E07fccf14C",
				"name": "Yield Yak: PGL AVAX-PEFI",
				"decimals": 18
			},
			"0x4cacece41645ae3a78655cdc33320bed00f72437": {
				"ticker": "YRT",
				"address": "0x4CAceCe41645Ae3A78655cdC33320beD00f72437",
				"name": "Yield Yak: Olive-LP WAVAX-SNOB",
				"decimals": 18
			},
			"0xded4e24c158b973d4cd309b19295954dd0ad5a23": {
				"ticker": "YRT",
				"address": "0xded4e24C158b973D4cD309b19295954DD0ad5A23",
				"name": "Yield Yak: Olive-LP WAVAX-ELK",
				"decimals": 18
			},
			"0xba78660bca43b5a692c632c2a02ff329701dcbf1": {
				"ticker": "YRT",
				"address": "0xBA78660BCa43b5a692c632c2A02ff329701DcbF1",
				"name": "Yield Yak: ELP ELK-LINK",
				"decimals": 18
			},
			"0xc2387b6ab5b75aacd6a4a94e44f8402f1e8afdc2": {
				"ticker": "YRT",
				"address": "0xC2387b6AB5B75aacD6A4A94e44f8402f1E8AFdc2",
				"name": "Yield Yak: ELP ELK-USDT",
				"decimals": 18
			},
			"0xaa21128578043c10c5c50bff1011879c5188ddb2": {
				"ticker": "YRT",
				"address": "0xaa21128578043C10c5C50BFF1011879c5188dDB2",
				"name": "Yield Yak: ELP ELK-ETH",
				"decimals": 18
			},
			"0x20afdef84d1e87d9bfd8086f7269183e12c712e2": {
				"ticker": "YRT",
				"address": "0x20afdEf84D1E87D9BfD8086F7269183E12C712E2",
				"name": "Yield Yak: ELP ELK-AVAX",
				"decimals": 18
			},
			"0xf2a857ee56f568989da138653c044379c08f0657": {
				"ticker": "YRT",
				"address": "0xF2a857Ee56f568989Da138653c044379c08f0657",
				"name": "Yield Yak: PGL AVAX-YFI",
				"decimals": 18
			},
			"0xbab74ca880e918440ad5a8b9ccece56cce5cda31": {
				"ticker": "YRT",
				"address": "0xbaB74Ca880E918440AD5A8b9CCECe56cCE5CDA31",
				"name": "Yield Yak: PGL AVAX-AAVE",
				"decimals": 18
			},
			"0xb373a54f7f773a99febe49f2c5db11756574f0ce": {
				"ticker": "YRT",
				"address": "0xb373a54f7F773a99feBe49F2C5dB11756574f0CE",
				"name": "Yield Yak: PGL AVAX-DAI",
				"decimals": 18
			},
			"0xe51d794d7822e5a78ba681a69294a31bf2117b94": {
				"ticker": "YRT",
				"address": "0xe51d794d7822e5A78bA681A69294a31BF2117b94",
				"name": "Yield Yak: PGL AVAX-USDT",
				"decimals": 18
			},
			"0x1817fe376740b53cae73224b7f0a57f23dd4c9b5": {
				"ticker": "YRT",
				"address": "0x1817fE376740b53CAe73224B7F0a57F23DD4C9b5",
				"name": "Yield Yak: PGL AVAX-WBTC",
				"decimals": 18
			},
			"0x0bc2fb2d9ba486cbf9e6192f982023f72e80f96f": {
				"ticker": "YRT",
				"address": "0x0bc2Fb2d9Ba486cBF9e6192f982023f72E80f96F",
				"name": "Yield Yak: PGL AVAX-LINK",
				"decimals": 18
			},
			"0x206d15cb7da413c0e662f284e00f907f51999a40": {
				"ticker": "YRT",
				"address": "0x206D15CB7DA413C0E662f284e00f907f51999a40",
				"name": "Yield Yak: PGL AVAX-SUSHI",
				"decimals": 18
			},
			"0x2521597a67dba56932358f0a67f196ba0cd33053": {
				"ticker": "YRT",
				"address": "0x2521597A67DbA56932358F0a67f196BA0cD33053",
				"name": "Yield Yak: PGL AVAX-UNI",
				"decimals": 18
			},
			"0xa544b965c2a05b97c44f3126cec916332afb3175": {
				"ticker": "YRT",
				"address": "0xA544b965C2a05b97C44f3126cec916332aFb3175",
				"name": "Yield Yak: PGL AVAX-PNG",
				"decimals": 18
			},
			"0x06404fc9c69f8333dc24d4c856e2c8db7983eb8a": {
				"ticker": "YRT",
				"address": "0x06404FC9C69F8333DC24D4C856E2c8Db7983EB8a",
				"name": "Yield Yak: PGL AVAX-ETH",
				"decimals": 18
			},
			"0x1ec356f7ea5a1cd8b479cf7a27c76a24b323d409": {
				"ticker": "YRT",
				"address": "0x1EC356F7EA5a1cD8B479Cf7a27c76A24b323D409",
				"name": "Yield Yak: PGL AVAX-YFI",
				"decimals": 18
			},
			"0x09ad8bafb4da3951ff8d4eebd91c654bceab053c": {
				"ticker": "YRT",
				"address": "0x09Ad8BafB4Da3951FF8d4EEbD91C654bCEAb053c",
				"name": "Yield Yak: PGL AVAX-AAVE",
				"decimals": 18
			},
			"0xebbe895fa78d8cebcc44a68dc95689f53b66c9a7": {
				"ticker": "YRT",
				"address": "0xEbbe895fA78D8CEbcc44a68dc95689F53B66c9a7",
				"name": "Yield Yak: PGL AVAX-DAI",
				"decimals": 18
			},
			"0x9099a81ddb2c6ee1fd3993e021e6c9179f84ccd1": {
				"ticker": "YRT",
				"address": "0x9099A81DDb2c6Ee1Fd3993e021E6c9179f84CcD1",
				"name": "Yield Yak: PGL AVAX-USDT",
				"decimals": 18
			},
			"0x5d2e25ff380ae1e872bc23184af14ded67e3a29b": {
				"ticker": "YRT",
				"address": "0x5D2E25ff380AE1e872Bc23184Af14dED67E3a29b",
				"name": "Yield Yak: PGL AVAX-WBTC",
				"decimals": 18
			},
			"0x02fe99babb22da03b19d4696169b7e92f35445b9": {
				"ticker": "YRT",
				"address": "0x02Fe99baBb22DA03B19D4696169b7E92f35445b9",
				"name": "Yield Yak: PGL AVAX-LINK",
				"decimals": 18
			},
			"0xcdc094b37015569178fa6920492228af811c4db9": {
				"ticker": "YRT",
				"address": "0xCdC094B37015569178fA6920492228aF811C4Db9",
				"name": "Yield Yak: PGL AVAX-SUSHI",
				"decimals": 18
			},
			"0xb88f392e8992d1fc4b54ffed5164bd548c238d95": {
				"ticker": "YRT",
				"address": "0xb88F392e8992d1fc4b54FFED5164BD548C238d95",
				"name": "Yield Yak: PGL AVAX-UNI",
				"decimals": 18
			},
			"0xdf8cd9449babbf8c7687bbdb69d09eb413b78018": {
				"ticker": "YRT",
				"address": "0xDf8CD9449babBf8C7687BbdB69d09EB413B78018",
				"name": "Yield Yak: PGL AVAX-PNG",
				"decimals": 18
			},
			"0xa7d7cdc5671858aef49827a7a06a0e0880f8eebf": {
				"ticker": "YRT",
				"address": "0xA7d7CDc5671858AEF49827A7a06a0E0880f8eEbf",
				"name": "Yield Yak: PGL AVAX-ETH",
				"decimals": 18
			},
			"0xce453c94f63990f2597cca15960a048fd44ceba1": {
				"ticker": "ELP",
				"address": "0xCE453C94f63990F2597ccA15960A048fD44cEba1",
				"name": "Elk Liquidity",
				"decimals": 18
			},
			"0xdab4513a18ab3ce23470c1a75e3da7dca2643c35": {
				"ticker": "ELP",
				"address": "0xDaB4513A18ab3cE23470c1A75e3DA7dcA2643c35",
				"name": "Elk Liquidity",
				"decimals": 18
			},
			"0xaa22703af4a6575feb316f078bdc584a94fde108": {
				"ticker": "YRT",
				"address": "0xaa22703AF4a6575fEB316f078bdc584A94FdE108",
				"name": "Yield Yak: ELP AVAX-WBTC",
				"decimals": 18
			},
			"0x6d76d0a8ad4f2325c6f4286dd3a40c1cd3b817de": {
				"ticker": "YRT",
				"address": "0x6d76d0a8AD4F2325C6f4286DD3a40c1cd3B817dE",
				"name": "Yield Yak: ELP AVAX-LINK",
				"decimals": 18
			},
			"0x18df163393c6da3285c2888461b8fd736272e245": {
				"ticker": "YRT",
				"address": "0x18DF163393c6DA3285C2888461b8fD736272E245",
				"name": "Yield Yak: ELP AVAX-ETH",
				"decimals": 18
			},
			"0x411b1f257e84394a60c7fb8fbda9c53a0c441057": {
				"ticker": "YRT",
				"address": "0x411b1F257E84394A60C7Fb8fbdA9C53A0C441057",
				"name": "Yield Yak: ZERO-LP ZERO-zSUSHI",
				"decimals": 18
			},
			"0xd8399d3db7da1e1db132de4e5d3ccf8620dfa2e6": {
				"ticker": "YRT",
				"address": "0xD8399D3db7da1E1Db132DE4E5D3cCF8620dfa2e6",
				"name": "Yield Yak: ZERO-LP ZERO-zUNI",
				"decimals": 18
			},
			"0x9800f6189ee6849361d8bf4c0cd7a75a3074550d": {
				"ticker": "YRT",
				"address": "0x9800F6189EE6849361d8bF4c0cD7a75a3074550d",
				"name": "Yield Yak: ZERO-LP ZERO-zUSDT",
				"decimals": 18
			},
			"0xca0803b898f64e3a990998a79ad874c1880ba346": {
				"ticker": "YRT",
				"address": "0xca0803B898F64E3a990998a79Ad874c1880bA346",
				"name": "Yield Yak: ZERO-LP ZERO-zETH",
				"decimals": 18
			},
			"0xc53dc280dc14f24b21370919d757cf7b296bfd31": {
				"ticker": "YRT",
				"address": "0xC53DC280dc14F24B21370919d757cF7b296BFd31",
				"name": "Yield Yak: YSL YTS-PNG",
				"decimals": 18
			},
			"0xaf985c13836b753445487c99448da38bd7774a17": {
				"ticker": "YRT",
				"address": "0xaf985c13836b753445487c99448dA38Bd7774A17",
				"name": "Yield Yak: YSL YTS-USDT",
				"decimals": 18
			},
			"0x2409e04f96e71fc931e18196509f130dfe0959a6": {
				"ticker": "YRT",
				"address": "0x2409E04F96E71Fc931E18196509F130dFE0959a6",
				"name": "Yield Yak: YSL YTS-ETH",
				"decimals": 18
			},
			"0x72e332cc6bf00ec48861c51df104770edf7890b6": {
				"ticker": "YRT",
				"address": "0x72E332cc6bf00EC48861C51df104770EDf7890B6",
				"name": "Yield Yak: YSL AVAX-USDT",
				"decimals": 18
			},
			"0x1ad92f75c2740efd143ed666f8de3ed963b22ad3": {
				"ticker": "YRT",
				"address": "0x1AD92f75C2740efD143ED666f8dE3eD963b22AD3",
				"name": "Yield Yak: YSL AVAX-ETH",
				"decimals": 18
			},
			"0x0345f2b85d238a963965ef8163ccb93eff81ff5e": {
				"ticker": "YRT",
				"address": "0x0345f2b85d238a963965Ef8163ccB93efF81Ff5E",
				"name": "Yield Yak: YSL AVAX-YTS",
				"decimals": 18
			},
			"0x1245819ff9b947ffe352ed6fab40e6ace7299677": {
				"ticker": "ELP",
				"address": "0x1245819ff9b947fFE352ed6fAb40e6aCe7299677",
				"name": "Elk Liquidity",
				"decimals": 18
			},
			"0x9f125fe210b27199229221be5f8e0ebb9094af37": {
				"ticker": "ELP",
				"address": "0x9f125fe210b27199229221BE5F8E0ebB9094Af37",
				"name": "Elk Liquidity",
				"decimals": 18
			},
			"0x4dd36b27e038e5c479d8bec440c8f4ee89b6df5d": {
				"ticker": "YRT",
				"address": "0x4DD36b27e038E5c479d8BEC440c8f4eE89B6dF5D",
				"name": "Yield Yak: OLIVE",
				"decimals": 18
			},
			"0x882b1f5c8f5438e1c47ef464a6199c70f20cceda": {
				"ticker": "YRT",
				"address": "0x882b1F5C8f5438e1C47Ef464A6199C70f20cCeDa",
				"name": "Yield Yak: ELP LINK-ELK",
				"decimals": 18
			},
			"0xc82ad2eab7ee0b9226ae884255ca9c1cb33d9e2c": {
				"ticker": "YRT",
				"address": "0xC82ad2Eab7EE0B9226aE884255ca9c1cB33D9E2C",
				"name": "Yield Yak: Olive-LP DAI-USDT",
				"decimals": 18
			},
			"0xec2258cc4b75ad0a013c7346c07f7470aea7f0e4": {
				"ticker": "YRT",
				"address": "0xec2258cC4B75aD0a013C7346c07f7470AEa7F0e4",
				"name": "Yield Yak: Olive-LP PNG-WAVAX",
				"decimals": 18
			},
			"0x9c36eca9309f0ceb5818da889e47d3c3e2ba9f32": {
				"ticker": "YRT",
				"address": "0x9C36eca9309f0CeB5818da889E47D3c3E2ba9F32",
				"name": "Yield Yak: Olive-LP WBTC-WAVAX",
				"decimals": 18
			},
			"0xa019f49464fcd206d080060cbbcb1a089441a732": {
				"ticker": "YRT",
				"address": "0xa019F49464FCD206d080060CBbCB1A089441a732",
				"name": "Yield Yak: Olive-LP WAVAX-ETH",
				"decimals": 18
			},
			"0x3a8d3fbab8b87473b9b2d78393200b099fa98497": {
				"ticker": "YRT",
				"address": "0x3a8D3FBaB8b87473b9B2D78393200B099fA98497",
				"name": "Yield Yak: Olive-LP SUSHI-WAVAX",
				"decimals": 18
			},
			"0x212df67bf1243ee57686883c9222637136bb65e4": {
				"ticker": "YRT",
				"address": "0x212dF67Bf1243eE57686883C9222637136bb65E4",
				"name": "Yield Yak: Olive-LP WAVAX-USDT",
				"decimals": 18
			},
			"0x89a806347b0814a265dc17afc343866b2214dd0f": {
				"ticker": "YRT",
				"address": "0x89a806347b0814a265Dc17aFc343866b2214dD0F",
				"name": "Yield Yak: Olive-LP OLIVE-USDT",
				"decimals": 18
			},
			"0xfdffdf6dc4fb30bede8af0f78d42c5468f37324b": {
				"ticker": "YRT",
				"address": "0xfDffdf6Dc4FB30BeDE8af0f78D42c5468F37324B",
				"name": "Yield Yak: Olive-LP OLIVE-WAVAX",
				"decimals": 18
			},
			"0xd335117acc6bf8b829fa0687776fe99b3888fbfd": {
				"ticker": "YRT",
				"address": "0xd335117acC6BF8B829Fa0687776FE99B3888fbFD",
				"name": "Yield Yak: PLP AVAX-ETH",
				"decimals": 18
			},
			"0x4a79206b5e1af8ff4e62183f2f2ff2b4efb0a9b9": {
				"ticker": "YRT",
				"address": "0x4a79206B5e1aF8ff4e62183f2f2ff2B4Efb0A9B9",
				"name": "Yield Yak: PLP BAMBOO-V2-LINK",
				"decimals": 18
			},
			"0xdefc4b949ea970533fe6f5a75248d4da534f8873": {
				"ticker": "YRT",
				"address": "0xDEfC4b949EA970533FE6F5A75248d4Da534F8873",
				"name": "Yield Yak: PLP WBTC-WAVAX",
				"decimals": 18
			},
			"0x0b4a63943e96463b432ec79e7287e63d830bec51": {
				"ticker": "YRT",
				"address": "0x0b4a63943e96463b432ec79E7287E63d830BEC51",
				"name": "Yield Yak: PLP WAVAX-ETH",
				"decimals": 18
			},
			"0x69293f1f9f4ad45ef9295f94ccaed0c045466214": {
				"ticker": "YRT",
				"address": "0x69293F1f9F4ad45ef9295f94ccaED0C045466214",
				"name": "Yield Yak: PLP WBTC-ETH",
				"decimals": 18
			},
			"0x00508fd84e2aca19b8af38c2c59171971d97495b": {
				"ticker": "YRT",
				"address": "0x00508fD84E2aCA19B8aF38C2c59171971D97495b",
				"name": "Yield Yak: PLP WAVAX-USDT",
				"decimals": 18
			},
			"0x43801e030afbdfd9a30da3e5321dd6b609083bcb": {
				"ticker": "YRT",
				"address": "0x43801E030AFbdFD9A30dA3E5321DD6B609083BCB",
				"name": "Yield Yak: PLP DAI-USDT",
				"decimals": 18
			},
			"0x06fd3ecee685ca5717c814dc7a790641e98ad9c7": {
				"ticker": "YRT",
				"address": "0x06Fd3eceE685cA5717c814dc7a790641E98aD9c7",
				"name": "Yield Yak: PLP WBTC-WAVAX",
				"decimals": 18
			},
			"0xc0787fd12249451763be576cf73621f3af61ebbd": {
				"ticker": "YRT",
				"address": "0xC0787Fd12249451763be576cf73621f3aF61ebBD",
				"name": "Yield Yak: PLP WAVAX-ETH",
				"decimals": 18
			},
			"0xaa9e134d3f9c802f099c53a0c0195373e34d92f7": {
				"ticker": "YRT",
				"address": "0xaa9e134D3f9C802f099c53A0C0195373e34d92F7",
				"name": "Yield Yak: PLP WBTC-ETH",
				"decimals": 18
			},
			"0xf0bde208a05cc0a056b5fae1a78212adf7a4affe": {
				"ticker": "YRT",
				"address": "0xF0BDE208A05CC0a056B5FAE1a78212Adf7A4AFfe",
				"name": "Yield Yak: PLP WAVAX-BAMBOO",
				"decimals": 18
			},
			"0x4a4421c3315319ee9ac251aa851d780672e1544d": {
				"ticker": "ELP",
				"address": "0x4a4421C3315319EE9aC251AA851D780672e1544d",
				"name": "Elk Liquidity",
				"decimals": 18
			},
			"0xb44a9b6905af7c801311e8f4e76932ee959c663c": {
				"ticker": "ANY",
				"address": "0xB44a9B6905aF7c801311e8F4E76932ee959c663C",
				"name": "Anyswap",
				"decimals": 18
			},
			"0x8054faba5fc1b8523a0c2fb54845c2ec3326b347": {
				"ticker": "SNOW",
				"address": "0x8054fABa5fC1b8523A0c2fb54845C2Ec3326B347",
				"name": "Snowball: YSL WAVAX-USDT",
				"decimals": 18
			},
			"0x007a0f7c4cae14415dbcac83da92d4c01f7601ed": {
				"ticker": "SNOW",
				"address": "0x007A0f7C4CAe14415dbCAC83da92d4c01f7601ED",
				"name": "Snowball: YSL WAVAX-ETH",
				"decimals": 18
			},
			"0x79521b914ec1b55e1bb4d52978fb585918a827db": {
				"ticker": "ELP",
				"address": "0x79521b914ec1b55e1BB4d52978Fb585918a827db",
				"name": "Elk Liquidity",
				"decimals": 18
			},
			"0x50f91f231f8eeca4ff6e0d4f2dabe7d22228d34d": {
				"ticker": "SNOW",
				"address": "0x50F91f231F8eEca4ff6e0D4f2dABe7d22228D34d",
				"name": "Snowball: ELP DAI-ELK",
				"decimals": 18
			},
			"0xfe0521f000d20f0bb9bb2cba51dc9066468671f9": {
				"ticker": "SNOW",
				"address": "0xfE0521F000d20f0bb9bB2cBa51dC9066468671f9",
				"name": "Snowball: ZERO-LP ZERO-zDAI",
				"decimals": 18
			},
			"0x6bbea2dba0ea15f17c92f84765ae1c3ab651df58": {
				"ticker": "SNOW",
				"address": "0x6BbeA2dbA0Ea15F17C92F84765ae1c3Ab651dF58",
				"name": "Snowball: ZERO-LP ZERO-WAVAX",
				"decimals": 18
			},
			"0x303dff9550a1e959a7e6731809a587acc195fd21": {
				"ticker": "SNOW",
				"address": "0x303DfF9550a1E959A7E6731809A587acc195fD21",
				"name": "Snowball: YSL YTS-WAVAX",
				"decimals": 18
			},
			"0x485240fd6e73ec6afd32a7c64707f7aac11ee151": {
				"ticker": "SNOW",
				"address": "0x485240Fd6e73EC6afd32A7C64707F7AAc11EE151",
				"name": "Snowball: ZERO-LP ZERO-zBTC",
				"decimals": 18
			},
			"0x1a7d36d12e12abfa91e8da437bda4c86657163e5": {
				"ticker": "SNOW",
				"address": "0x1a7d36D12e12ABfA91e8da437bDa4c86657163e5",
				"name": "Snowball: ELP USDT-ELK",
				"decimals": 18
			},
			"0xd661a38708e6544f2d32e302e6e893a97be7314c": {
				"ticker": "SNOW",
				"address": "0xd661a38708E6544F2D32e302e6E893A97Be7314c",
				"name": "Snowball: ELP ELK-ETH",
				"decimals": 18
			},
			"0x141be6e009a0d9a5c0d4e557c0636e97b3de2a7b": {
				"ticker": "SNOW",
				"address": "0x141bE6e009a0D9a5c0d4e557c0636e97B3de2a7B",
				"name": "Snowball: ZERO-LP ZERO-zUSDC",
				"decimals": 18
			},
			"0x7e449382cba7560a159301806f63550c4e05009c": {
				"ticker": "ELP",
				"address": "0x7e449382CBA7560A159301806F63550C4E05009c",
				"name": "Elk Liquidity",
				"decimals": 18
			},
			"0x449ee12de33e45c5a5bacbbe0f9ddd276e0ef417": {
				"ticker": "SNOW",
				"address": "0x449Ee12de33E45c5A5BaCBbE0f9DDd276e0ef417",
				"name": "Snowball: ELP WAVAX-ELK",
				"decimals": 18
			},
			"0xd514419729a55d2615f87b074e15cd28eca3fc9b": {
				"ticker": "ELP",
				"address": "0xD514419729A55d2615F87b074e15cD28ECa3FC9B",
				"name": "Elk Liquidity",
				"decimals": 18
			},
			"0x269fafdc9f362414b9c08adbb8a3106af5a2dc01": {
				"ticker": "ELP",
				"address": "0x269Fafdc9F362414B9c08ADBb8A3106AF5a2dc01",
				"name": "Elk Liquidity",
				"decimals": 18
			},
			"0x288ffb87fc69dc652f9b564faf05db7468013544": {
				"ticker": "ELP",
				"address": "0x288Ffb87fc69dC652f9b564fAF05dB7468013544",
				"name": "Elk Liquidity",
				"decimals": 18
			},
			"0xe4b269b61471d81898be034dab496910a5796154": {
				"ticker": "ELP",
				"address": "0xe4b269b61471D81898Be034DAB496910A5796154",
				"name": "Elk Liquidity",
				"decimals": 18
			},
			"0x0826e1a55ebef25d725bb944555f714db84d95bb": {
				"ticker": "ELP",
				"address": "0x0826e1A55ebeF25d725Bb944555F714Db84d95Bb",
				"name": "Elk Liquidity",
				"decimals": 18
			},
			"0x266ed42866b3d2d418c87ebf8b58f22add0e6f8e": {
				"ticker": "ELP",
				"address": "0x266ed42866b3d2d418C87EbF8b58f22add0E6F8e",
				"name": "Elk Liquidity",
				"decimals": 18
			},
			"0xf31a76d123dd6b10008f2f096ab0aafbd508a2de": {
				"ticker": "ELP",
				"address": "0xF31a76d123dd6b10008F2f096AB0AafBd508A2de",
				"name": "Elk Liquidity",
				"decimals": 18
			},
			"0x3e58308a97af12ad4affe6f688a8b7e8a8e459b1": {
				"ticker": "ELP",
				"address": "0x3e58308A97AF12AD4aFfE6f688a8b7e8a8E459B1",
				"name": "Elk Liquidity",
				"decimals": 18
			},
			"0x5bae3110066890d0bdb9837c14c2461168a3407e": {
				"ticker": "ELP",
				"address": "0x5bae3110066890D0BDB9837c14C2461168a3407E",
				"name": "Elk Liquidity",
				"decimals": 18
			},
			"0x3fbd7dd99ca32f98a56ddee57b6b96e1152668f5": {
				"ticker": "ELP",
				"address": "0x3Fbd7DD99Ca32f98a56DDEE57B6b96e1152668F5",
				"name": "Elk Liquidity",
				"decimals": 18
			},
			"0x53e3d3e77b3ed38fdc5156f6ef761d85012850c2": {
				"ticker": "ELP",
				"address": "0x53e3D3e77B3ed38FDc5156F6EF761D85012850c2",
				"name": "Elk Liquidity",
				"decimals": 18
			},
			"0xfebf47cf89f766e6c24317b17f862ba5d4d82f8c": {
				"ticker": "ELP",
				"address": "0xfEbf47CF89F766E6c24317b17F862bA5d4d82f8c",
				"name": "Elk Liquidity",
				"decimals": 18
			},
			"0xe1c8f3d529bea8e3fa1fac5b416335a2f998ee1c": {
				"ticker": "ELK",
				"address": "0xE1C8f3d529BEa8E3fA1FAC5B416335a2f998EE1C",
				"name": "Elk",
				"decimals": 18
			},
			"0x8d35beb9e8750959c3520d202b589ab78c366d59": {
				"ticker": "SNOW",
				"address": "0x8D35BEB9e8750959c3520d202b589aB78C366D59",
				"name": "Snowball: ZERO-LP ZERO-zUSDC",
				"decimals": 18
			},
			"0x456a68eec203a35a8fa1d9c7bf5797909d1cee04": {
				"ticker": "SNOW",
				"address": "0x456A68EeC203a35A8fa1D9c7Bf5797909D1cee04",
				"name": "Snowball: ZERO-LP WAVAX-zETH",
				"decimals": 18
			},
			"0x6ea142f55ae733eeaeef5fea34d4312478c762e5": {
				"ticker": "SNOW",
				"address": "0x6ea142f55aE733eeaEef5fEA34d4312478c762e5",
				"name": "Snowball: ZERO-LP zUSDC-WAVAX",
				"decimals": 18
			},
			"0x7f3bb8db336ff50120e290e5c8ec78b20f619d01": {
				"ticker": "SNOW",
				"address": "0x7f3BB8dB336ff50120E290e5C8eC78B20f619D01",
				"name": "Snowball: COM-LP COM-WAVAX",
				"decimals": 18
			},
			"0xfd410034f88b99e4bee8bd7b51fa323b6678bf73": {
				"ticker": "SNOW",
				"address": "0xfd410034f88B99E4BeE8Bd7B51Fa323B6678Bf73",
				"name": "Snowball: COM-LP xCOM-WAVAX",
				"decimals": 18
			},
			"0x49e01ade31690d286c5e820a8daa4412125c7e7a": {
				"ticker": "SNOW",
				"address": "0x49e01Ade31690D286C5E820a8DAA4412125c7E7a",
				"name": "Snowball: ZERO-LP ZERO-zETH",
				"decimals": 18
			},
			"0xc47c5050e1f8e45a6741613e29a303d7cc106aff": {
				"ticker": "SNOW",
				"address": "0xC47c5050E1f8e45a6741613E29a303D7cc106afF",
				"name": "Snowball: ZERO-LP ZERO-WAVAX",
				"decimals": 18
			},
			"0x5777e014b585a5f05db9902ef944df9c45f2054c": {
				"ticker": "YFFI",
				"address": "0x5777E014b585A5F05dB9902ef944Df9C45F2054C",
				"name": "yffi.finance",
				"decimals": 18
			},
			"0xc84d7bff2555955b44bdf6a307180810412d751b": {
				"ticker": "UMA",
				"address": "0xC84d7bfF2555955b44BDF6A307180810412D751B",
				"name": "UMA Voting Token v1",
				"decimals": 18
			},
			"0x390ba0fb0bd3aa2a5484001606329701148074e6": {
				"ticker": "RUNE",
				"address": "0x390ba0fb0Bd3Aa2a5484001606329701148074e6",
				"name": "THORChain ETH.RUNE",
				"decimals": 18
			},
			"0x53ceedb4f6f277edfddedb91373b044fe6ab5958": {
				"ticker": "COMP",
				"address": "0x53CEedB4f6f277edfDDEdB91373B044FE6AB5958",
				"name": "Compound",
				"decimals": 18
			},
			"0x6b329326e0f6b95b93b52229b213334278d6f277": {
				"ticker": "BAT",
				"address": "0x6b329326E0F6b95B93b52229b213334278D6f277",
				"name": "Basic Attention Token",
				"decimals": 18
			},
			"0x58fd406656bcc7e0c8891bee8d84fbbd72db2e5c": {
				"ticker": "ASTRA",
				"address": "0x58Fd406656bCc7e0c8891bee8D84fbBd72DB2E5c",
				"name": "ASTRAL",
				"decimals": 18
			},
			"0xe7821e8a908047fb15a260920e3878cacbedf464": {
				"ticker": "DOMPE",
				"address": "0xe7821e8a908047FB15a260920E3878CacBedF464",
				"name": "DOMPE",
				"decimals": 18
			},
			"0x370b039577aab45bba72a05bcf92411df5059f03": {
				"ticker": "OLYMPIC",
				"address": "0x370B039577Aab45bBa72A05bcF92411df5059F03",
				"name": "OLYMPIC",
				"decimals": 18
			},
			"0x0512b4ac706004aaa4cdd8fe8c94f57065aad6b9": {
				"ticker": "TEST",
				"address": "0x0512b4Ac706004aaA4CdD8fe8c94F57065Aad6b9",
				"name": "Test",
				"decimals": 18
			},
			"0xb72ab6f7177bbb41efcc17d817778d77460259f1": {
				"ticker": "VERSE",
				"address": "0xB72ab6f7177bBb41eFcC17D817778d77460259F1",
				"name": "Traverse",
				"decimals": 9
			},
			"0xbbd9678aeaf31b24520587adbaff9ce45c8674ea": {
				"ticker": "JEWEL",
				"address": "0xBbD9678AEAF31b24520587aDbAff9Ce45c8674Ea",
				"name": "DeFi Kingdoms",
				"decimals": 18
			},
			"0xb7b009189ccf0de1d0627cc7cd5b3269e1398887": {
				"ticker": "FLYHIGH",
				"address": "0xB7b009189CCf0De1D0627cC7Cd5b3269e1398887",
				"name": "AvaxRocket",
				"decimals": 9
			},
			"0xd2a307f1178359fb00a781529bb513f26647aed6": {
				"ticker": "UDAO",
				"address": "0xd2a307f1178359FB00A781529bb513F26647aEd6",
				"name": "UnitedDao",
				"decimals": 18
			},
			"0x454e67025631c065d3cfad6d71e6892f74487a15": {
				"ticker": "JLP",
				"address": "0x454E67025631C065d3cFAD6d71E6892f74487a15",
				"name": "Joe LP Token",
				"decimals": 18
			},
			"0x4508d73a8234ffc3bbe53a9688afede1cde84a45": {
				"ticker": "TESTTH",
				"address": "0x4508d73a8234FFc3bBE53A9688afEDE1cdE84a45",
				"name": "TESTTH",
				"decimals": 9
			},
			"0x93115d0d3980a0f0b953ccf76727c401d33699e4": {
				"ticker": "PIX",
				"address": "0x93115d0D3980a0F0b953cCF76727c401D33699e4",
				"name": "Pixel Nodes",
				"decimals": 18
			},
			"0xbade91c6ca85e56fa7bc72b25ac2e2d7f0e960af": {
				"ticker": "MTEST",
				"address": "0xbAde91C6ca85e56FA7bc72B25ac2E2D7f0e960AF",
				"name": "MTEST",
				"decimals": 18
			},
			"0x44d1f1b80d442c303430857c8c53a40b200406c0": {
				"ticker": "NOBEL",
				"address": "0x44d1F1B80D442c303430857C8c53a40b200406c0",
				"name": "Nobelium DAO",
				"decimals": 18
			},
			"0xf10b5a72f0abeb27a5ddba950082e4edff4c825d": {
				"ticker": "PARR",
				"address": "0xf10b5a72f0abeB27A5DdBa950082e4eDFF4c825d",
				"name": "ParrotDao",
				"decimals": 9
			},
			"0x5c4590267620b8031fa76d983d119a2422320640": {
				"ticker": "CORKSCREW",
				"address": "0x5C4590267620B8031fA76d983D119a2422320640",
				"name": "CORK",
				"decimals": 18
			},
			"0x95189f25b4609120f72783e883640216e92732da": {
				"ticker": "JLP",
				"address": "0x95189f25b4609120F72783E883640216E92732DA",
				"name": "Joe LP Token",
				"decimals": 18
			},
			"0x3be37fc936026337eeb7e71bfafa30b09305863e": {
				"ticker": "EVE",
				"address": "0x3bE37FC936026337eeb7e71bfAfA30B09305863E",
				"name": "EVE",
				"decimals": 18
			},
			"0xe07b162b6bb72cb7c91f89e80029b30d400ed0ff": {
				"ticker": "OXY",
				"address": "0xE07B162b6bb72CB7c91f89e80029b30d400ED0Ff",
				"name": "Oxy-Fi",
				"decimals": 18
			},
			"0xed46443c18e38064523180fc364c6180b35803d3": {
				"ticker": "CROWN",
				"address": "0xed46443C18E38064523180Fc364C6180b35803d3",
				"name": "MidasDAO",
				"decimals": 9
			},
			"0xf59b2fa5d1c56a379dda6f79586666aca935450f": {
				"ticker": "VTP",
				"address": "0xf59B2FA5d1C56a379DDa6F79586666aCA935450F",
				"name": "Vanguard token prout",
				"decimals": 18
			},
			"0x4c7c111379a08e089fcbc558deee92e97d795f1f": {
				"ticker": "GROW",
				"address": "0x4c7c111379a08E089fCBc558DeeE92e97d795f1F",
				"name": "Growth Capital",
				"decimals": 18
			},
			"0xcb221ff1798119898d9776505ef6b099b062a149": {
				"ticker": "XOMB",
				"address": "0xcB221FF1798119898D9776505ef6B099B062a149",
				"name": "XOMB Token",
				"decimals": 18
			},
			"0xa55fba7c47cc203245e056756621b94787bac882": {
				"ticker": "BLCKTST",
				"address": "0xA55FBa7c47Cc203245E056756621b94787BAc882",
				"name": "Blocktest",
				"decimals": 18
			},
			"0x2d77fad50fbeb266489ce402d3e83d4d0216f486": {
				"ticker": "TEST",
				"address": "0x2D77FAd50FbEb266489ce402D3E83d4d0216f486",
				"name": "TestToken",
				"decimals": 18
			},
			"0x0c471271ac5d28f824f5c8fd7f8aee2652ad4f67": {
				"ticker": "$Lucky",
				"address": "0x0c471271aC5d28F824f5c8fD7f8AEE2652Ad4f67",
				"name": "LuckyDao",
				"decimals": 18
			},
			"0x27f14266833bc685ccc4619aed4ed520273e441d": {
				"ticker": "WTF",
				"address": "0x27F14266833Bc685ccC4619aed4ED520273e441d",
				"name": "WTF",
				"decimals": 18
			},
			"0x0c56938dc4be6fc6b5641445be21644797ce251a": {
				"ticker": "SNR",
				"address": "0x0C56938DC4bE6fc6B5641445be21644797cE251A",
				"name": "SNR.e",
				"decimals": 18
			},
			"0xd430e4d8f23409403a4e74155bba381e5c83c3b1": {
				"ticker": "ThorPrinter",
				"address": "0xD430E4d8f23409403a4E74155BBA381e5c83C3B1",
				"name": "Thor Printer",
				"decimals": 18
			},
			"0xe4437cdefa392bd9c3b077f49d3a229de7809879": {
				"ticker": "ILV",
				"address": "0xE4437cDEFa392BD9C3b077F49d3A229de7809879",
				"name": "Illivium",
				"decimals": 18
			},
			"0x9065c2ac793ffbc5010fac8ad949eb4c64f0c414": {
				"ticker": "GOLD",
				"address": "0x9065c2aC793FfBc5010Fac8Ad949EB4C64F0C414",
				"name": "GOLD",
				"decimals": 18
			},
			"0xdc955d61397ae513dcf2ee600b7cc43ce0542522": {
				"ticker": "OHBABY",
				"address": "0xdc955D61397aE513Dcf2EE600B7cc43cE0542522",
				"name": "OHBABY",
				"decimals": 18
			},
			"0xbec561b549916508e76a88b8c1439c933c9bf055": {
				"ticker": "TST",
				"address": "0xBEc561b549916508e76A88B8C1439c933c9bF055",
				"name": "Test",
				"decimals": 18
			},
			"0x8a82312f284ef973092aea55c50717ef9751197d": {
				"ticker": "FSHARE",
				"address": "0x8a82312f284eF973092aea55C50717EF9751197d",
				"name": "FSHARE",
				"decimals": 18
			},
			"0xe530dc2095ef5653205cf5ea79f8979a7028065c": {
				"ticker": "PGL",
				"address": "0xE530dC2095Ef5653205CF5ea79F8979a7028065c",
				"name": "Pangolin Liquidity",
				"decimals": 18
			},
			"0x55c907e86997a9cb87563ac6c9b632e937713259": {
				"ticker": "T1",
				"address": "0x55C907E86997a9CB87563aC6c9b632E937713259",
				"name": "T1",
				"decimals": 9
			},
			"0x3b760b2527a4c2bb5e0f2b9e7bc77744ade2484e": {
				"ticker": "DB",
				"address": "0x3B760B2527a4C2bb5E0F2B9e7BC77744adE2484E",
				"name": "DiamondBank",
				"decimals": 9
			},
			"0xc46491e8d451b4ce0d34cbdd7062b4869b6a88ae": {
				"ticker": "IME",
				"address": "0xc46491e8d451b4CE0D34CBDd7062b4869b6A88AE",
				"name": "Imperium Empires",
				"decimals": 18
			},
			"0xba09679ab223c6bdaf44d45ba2d7279959289ab0": {
				"ticker": "PGL",
				"address": "0xbA09679Ab223C6bdaf44D45Ba2d7279959289AB0",
				"name": "Pangolin Liquidity",
				"decimals": 18
			},
			"0xbe217b06d3233d56938aa2a8cd1ac728c9dc848a": {
				"ticker": "GTPS",
				"address": "0xBe217b06d3233D56938aA2a8cd1Ac728c9DC848A",
				"name": "Global Transaction Payment Solution",
				"decimals": 18
			},
			"0x6ae6fde2178d7479332a48209f7268109b37e40b": {
				"ticker": "PLAGUE",
				"address": "0x6ae6fdE2178D7479332a48209F7268109B37E40b",
				"name": "PlagueDao.com",
				"decimals": 9
			},
			"0x13bbbcd75366c12185f6d792d0b13c693cf9b161": {
				"ticker": "SLN",
				"address": "0x13bBBCD75366c12185F6d792D0B13C693cF9B161",
				"name": "Silicon Capital",
				"decimals": 9
			},
			"0xe2eeebaf210502aa815008618c89ca9d98d97924": {
				"ticker": "888play.cc",
				"address": "0xE2eeEBAf210502aA815008618C89CA9d98d97924",
				"name": "888play.cc",
				"decimals": 8
			},
			"0xc792fff305914106eba41e3c418fa4f0387c881e": {
				"ticker": "KING",
				"address": "0xC792FfF305914106EBa41e3c418Fa4F0387C881e",
				"name": "KingdomDAO",
				"decimals": 18
			},
			"0xcb67f9a6130c7b26196d76570f44e5dfa2ecd385": {
				"ticker": "ISHARE",
				"address": "0xcB67f9A6130c7b26196d76570F44E5dFa2Ecd385",
				"name": "ISHARE",
				"decimals": 18
			},
			"0xac128ab37883b811de4c2c1ca1f51aed561e2475": {
				"ticker": "HYBRID",
				"address": "0xac128AB37883b811dE4c2C1ca1f51AeD561e2475",
				"name": "Hybrid",
				"decimals": 18
			},
			"0x9e3f774b1442c0846448f85c566ae14036137b60": {
				"ticker": "GAMER",
				"address": "0x9e3F774B1442c0846448f85c566aE14036137B60",
				"name": "GAMER",
				"decimals": 18
			},
			"0xa886c6eeceeebbe5e467e133d0c88db686947519": {
				"ticker": "SPEAR",
				"address": "0xA886c6eEcEeEBBe5e467E133D0C88dB686947519",
				"name": "SpearFinance",
				"decimals": 18
			},
			"0x72957d48dcb25ae59a0b8521ad00cf7de345c971": {
				"ticker": "SEC",
				"address": "0x72957d48dCb25Ae59a0B8521aD00cF7De345c971",
				"name": "Secret DAO",
				"decimals": 9
			},
			"0xa0e16cbeae137f26f6413a3d8c665e2f467a1878": {
				"ticker": "LILBABY2",
				"address": "0xa0E16CbeaE137f26f6413a3D8c665E2f467A1878",
				"name": "LILBABY2",
				"decimals": 6
			},
			"0x7ce3a8c3f36447d6eeb04b9e7a551a921f570f36": {
				"ticker": "META",
				"address": "0x7CE3A8c3F36447d6EEb04b9e7A551A921f570F36",
				"name": "MetaFund",
				"decimals": 18
			},
			"0x6a84d6b63e616ed43fdaee459a42b5ccc6cf50e1": {
				"ticker": "CORK",
				"address": "0x6A84D6b63e616ed43FdAee459a42b5cCC6cF50e1",
				"name": "Corkscrew",
				"decimals": 18
			},
			"0xb58ce46b73ce9901489987df08aa418d3119dd57": {
				"ticker": "BDK",
				"address": "0xb58cE46B73Ce9901489987DF08AA418D3119DD57",
				"name": "Bandung Koda",
				"decimals": 18
			},
			"0x40c2537bc94838c86ed31c6b7c01d27448a0b34c": {
				"ticker": "NEON",
				"address": "0x40c2537Bc94838c86ed31c6b7C01D27448A0b34c",
				"name": "Nodeon",
				"decimals": 18
			},
			"0xe3afe6dcffac1a833a35a908ea502f770fd31bdf": {
				"ticker": "MAGIC",
				"address": "0xE3AfE6DCFfaC1a833A35a908Ea502f770fD31BdF",
				"name": "MAGIC DAO",
				"decimals": 18
			},
			"0x316dc89a2e9f55598bd8b7b6d590415bc6cdf212": {
				"ticker": "LOTUS",
				"address": "0x316Dc89a2E9F55598bd8b7b6D590415BC6CDF212",
				"name": "LOTUS DAO",
				"decimals": 18
			},
			"0x9d914f3110d59a0a0e3960962112c8fabc6215a4": {
				"ticker": "ISHARE",
				"address": "0x9d914f3110d59A0A0E3960962112c8FabC6215a4",
				"name": "ISHARE",
				"decimals": 18
			},
			"0x25969d015efc39656ea99ee6047d6390e1cdf73d": {
				"ticker": "TXT",
				"address": "0x25969D015EFC39656EA99Ee6047D6390e1CDF73D",
				"name": "Test TestTest",
				"decimals": 18
			},
			"0x66c00e4b114ce1302e31a8e97c31214148351bbd": {
				"ticker": "HRW",
				"address": "0x66c00E4B114cE1302E31a8e97C31214148351bbd",
				"name": "Horse Racing Winner",
				"decimals": 18
			},
			"0xa567ad465bb7fcb8c65d5411af41658eed02adc2": {
				"ticker": "GREEN",
				"address": "0xa567AD465bB7FcB8c65D5411Af41658eEd02adC2",
				"name": "GreenDAO",
				"decimals": 18
			},
			"0x7c05d54fc5cb6e4ad87c6f5db3b807c94bb89c52": {
				"ticker": "PGL",
				"address": "0x7c05d54fc5CB6e4Ad87c6f5db3b807C94bB89c52",
				"name": "Pangolin Liquidity",
				"decimals": 18
			},
			"0xd9e8ae66782529b3fc9157af2b6fa137bea46e5d": {
				"ticker": "LiPOGE",
				"address": "0xD9E8Ae66782529b3fc9157AF2B6fA137beA46E5D",
				"name": "iLiPogeCoin",
				"decimals": 9
			},
			"0xd55f778ec0ef242f035fd6e406028910956c073e": {
				"ticker": "PVAX",
				"address": "0xd55f778eC0Ef242f035Fd6E406028910956C073e",
				"name": "PONZAVAX",
				"decimals": 6
			},
			"0x168c14b83c959778b6b8c4e4fa0c20af8df23e61": {
				"ticker": "LiPOGE",
				"address": "0x168C14B83C959778b6b8C4e4FA0C20af8df23e61",
				"name": "iLiPoge Coin",
				"decimals": 9
			},
			"0x2e2c76b29d39bc802a170c7b755ff5116bb9d706": {
				"ticker": "FOX",
				"address": "0x2E2c76b29d39BC802A170c7b755fF5116BB9d706",
				"name": "LittleFox",
				"decimals": 18
			},
			"0xa5b0f5ef809fd04c9d4320211c711cb34ef812dd": {
				"ticker": "CLAVIS",
				"address": "0xA5b0f5ef809FD04C9D4320211C711Cb34ef812dD",
				"name": "Clavis",
				"decimals": 9
			},
			"0x136acd46c134e8269052c62a67042d6bdedde3c9": {
				"ticker": "MEMO",
				"address": "0x136Acd46C134E8269052c62A67042D6bDeDde3C9",
				"name": "MEMOries",
				"decimals": 9
			},
			"0xe84bd7e8d14f5795178ea2741c3f680813343ad5": {
				"ticker": "Splash",
				"address": "0xe84bD7E8d14f5795178Ea2741C3F680813343ad5",
				"name": "Splash Token",
				"decimals": 18
			},
			"0x04d80d453033450703e3dc2d0c1e0c0281c42d81": {
				"ticker": "PGL",
				"address": "0x04D80d453033450703E3DC2d0C1e0C0281c42D81",
				"name": "Pangolin Liquidity",
				"decimals": 18
			},
			"0x297d38e7f18c4f8f503ba8e37704026819ed7f7a": {
				"ticker": "LOOP",
				"address": "0x297D38e7F18C4f8F503ba8E37704026819eD7F7A",
				"name": "LOOT Printer",
				"decimals": 9
			},
			"0x4f23e24bf57b35d453fe332468fff9759904b40c": {
				"ticker": "FYN",
				"address": "0x4f23e24Bf57b35d453FE332468fff9759904b40c",
				"name": "Affyn",
				"decimals": 18
			},
			"0x7b21dadd67b3b272534519cec15491120f170307": {
				"ticker": "Rabbit",
				"address": "0x7b21daDD67b3b272534519Cec15491120F170307",
				"name": "WhiteRabbit",
				"decimals": 18
			},
			"0x2482ec3928adf1718caa5956608684f5a54e14d0": {
				"ticker": "sGG",
				"address": "0x2482ec3928aDf1718caA5956608684F5A54e14D0",
				"name": "Staked GalaxyGoggle",
				"decimals": 9
			},
			"0xad4715e16abe7fdb93750788957c061fedc4850c": {
				"ticker": "NEON",
				"address": "0xAD4715E16aBe7FdB93750788957C061FEDc4850C",
				"name": "Nodeon",
				"decimals": 18
			},
			"0xb11197d4c5ac495274588708191b99aa6e642d88": {
				"ticker": "FGD",
				"address": "0xB11197D4c5Ac495274588708191b99AA6e642d88",
				"name": "FREEDOM GOD DAO",
				"decimals": 18
			},
			"0xf03e3983b0393116b8ec41112f6de4995430bd04": {
				"ticker": "MSHARE",
				"address": "0xf03E3983B0393116b8eC41112f6dE4995430bd04",
				"name": "MSHARE",
				"decimals": 18
			},
			"0x58a1795197a199cb1f32a5e1f0d84f931ec470ff": {
				"ticker": "AS",
				"address": "0x58A1795197A199CB1f32A5e1f0D84f931EC470ff",
				"name": "Adaptive SANTRAST Network",
				"decimals": 12
			},
			"0xe592b24ac5762991b4e985800e905696dbf5095a": {
				"ticker": "STOMB",
				"address": "0xe592B24ac5762991B4E985800E905696dBf5095a",
				"name": "Snowtomb Token",
				"decimals": 18
			},
			"0x411a68f5fac3f0f840c0ceebfacae1d94e882eba": {
				"ticker": "KANDY",
				"address": "0x411a68f5FAC3f0F840c0cEeBfAcaE1D94E882ebA",
				"name": "Kandy Token",
				"decimals": 18
			},
			"0xfe555376a88cc296e22e56f32002a1b6a6e05de9": {
				"ticker": "THIRST",
				"address": "0xfE555376A88Cc296E22E56f32002a1B6a6e05DE9",
				"name": "Water Game",
				"decimals": 18
			},
			"0xb59131cc9649cb9cec7bfb89ade1337bb4c30ff3": {
				"ticker": "SNOWY",
				"address": "0xb59131Cc9649CB9ceC7bfB89adE1337Bb4c30fF3",
				"name": "Fantastic Protocol SNOWY Token",
				"decimals": 18
			},
			"0xb3eb251b0b32b719c447c0018f574272c3f27add": {
				"ticker": "PTP",
				"address": "0xB3eB251b0B32B719C447C0018f574272c3f27AdD",
				"name": "Platypus",
				"decimals": 18
			},
			"0x7a530f61240c010b550b925a9f23a4ad1448aa4c": {
				"ticker": "HALL",
				"address": "0x7a530f61240c010b550b925A9F23A4AD1448Aa4C",
				"name": "HALL Token",
				"decimals": 18
			},
			"0x30a95e40650a436ae71c2fa4b6ecd35f16f208d6": {
				"ticker": "DBD",
				"address": "0x30A95E40650a436ae71C2Fa4B6eCD35F16F208d6",
				"name": "Diamond Bank DAO",
				"decimals": 12
			},
			"0xb656bf6337c0f6e1146ff1e734956032a41694c4": {
				"ticker": "CKT",
				"address": "0xb656BF6337c0F6e1146Ff1E734956032A41694C4",
				"name": "C Knowledge Transfer",
				"decimals": 18
			},
			"0x94c2dff7485ab05cc412d00cbdbff7298172a91a": {
				"ticker": "THC$",
				"address": "0x94c2DfF7485AB05cC412d00CBDBfF7298172a91A",
				"name": "THC$",
				"decimals": 18
			},
			"0xbd4d90e56432ca3e5610b5126cbf8710c56dc32e": {
				"ticker": "XRING",
				"address": "0xBD4d90E56432cA3e5610b5126cBf8710c56dc32E",
				"name": "XRING",
				"decimals": 18
			},
			"0x2e1a7a2889154cded22f4ce8c4d1cc6dc4a58452": {
				"ticker": "CMSN",
				"address": "0x2E1a7A2889154Cded22f4CE8c4D1cC6Dc4A58452",
				"name": "The Commission",
				"decimals": 18
			},
			"0x841c10d7dcd9474007e68b023351f9cf22ff4b25": {
				"ticker": "MetaAvax",
				"address": "0x841C10D7dcD9474007e68B023351F9CF22fF4b25",
				"name": "MetaAvax",
				"decimals": 9
			},
			"0x96a11b7beabb1a1cc6780aa1a79105b976096702": {
				"ticker": "BULL",
				"address": "0x96A11B7BeAbB1A1cc6780aA1a79105b976096702",
				"name": "Bull Nodes",
				"decimals": 9
			},
			"0xa389f9430876455c36478deea9769b7ca4e3ddb1": {
				"ticker": "JLP",
				"address": "0xA389f9430876455C36478DeEa9769B7Ca4E3DDB1",
				"name": "Joe LP Token",
				"decimals": 18
			},
			"0x95e812c4f88a1e9ff350aa42a0b8fce75e1e3b8c": {
				"ticker": "PPeC",
				"address": "0x95e812C4f88a1e9Ff350aa42a0B8FcE75e1E3B8C",
				"name": "Paid Per Click",
				"decimals": 18
			},
			"0x30915fb3eb2a5cd1c03205ad39efed9c2628daa5": {
				"ticker": "testtest",
				"address": "0x30915Fb3eB2A5cD1C03205AD39efED9c2628Daa5",
				"name": "testtest",
				"decimals": 18
			},
			"0xb2f7c49e90fe365e95214041f916f454d7062607": {
				"ticker": "DO-NOT-BUY",
				"address": "0xB2F7c49E90Fe365E95214041F916F454d7062607",
				"name": "DO-NOT-BUY",
				"decimals": 18
			},
			"0x5b537940f0f1a0845ea68db1d00b81298e4a1020": {
				"ticker": "ROSH",
				"address": "0x5b537940f0f1a0845ea68db1D00b81298e4a1020",
				"name": "RoyalShiba",
				"decimals": 18
			},
			"0x99a828fe0c1d68d9aebbb8651cdbdbac65dc6207": {
				"ticker": "0FP0EXP",
				"address": "0x99a828fe0C1D68D9aeBBB8651CDBDbac65dc6207",
				"name": "Fajar Purnama Experiment",
				"decimals": 18
			},
			"0xed8a46ffc1835ff196b17487d8e9383a1e7c4753": {
				"ticker": "WAVAX",
				"address": "0xed8A46Ffc1835fF196b17487d8E9383A1e7c4753",
				"name": "Wrapped AVAX",
				"decimals": 18
			},
			"0x99f17613c0b38c9d38cd0b8db1702a00fdba3f39": {
				"ticker": "MEMA",
				"address": "0x99F17613c0B38c9d38Cd0b8dB1702a00FdbA3f39",
				"name": "Meme Manatee",
				"decimals": 18
			},
			"0x7c2dd5aa3068a0a9f62f90f6269d175aac247657": {
				"ticker": "MAG",
				"address": "0x7c2dD5Aa3068A0A9f62F90F6269d175AaC247657",
				"name": "Magnet DAO",
				"decimals": 18
			},
			"0x9c2079b6dfaeb0bdee979cfc27aa1c3e09e67b87": {
				"ticker": "PSHARE",
				"address": "0x9C2079b6dFAEB0bdEe979cfC27Aa1c3e09e67b87",
				"name": "PSHARE",
				"decimals": 18
			},
			"0x2f65c8ab2d99fdbbdf8ef09739b4dd800bda002c": {
				"ticker": "DOUB",
				"address": "0x2f65c8aB2d99FDBbdf8ef09739B4DD800BDa002C",
				"name": "Doubloon",
				"decimals": 9
			},
			"0x9b908fc861cee312255cb91a5f3d1ecb8f2d1772": {
				"ticker": "WAVAX",
				"address": "0x9B908Fc861cee312255CB91a5f3d1eCB8F2D1772",
				"name": "Wrapped AVAX",
				"decimals": 18
			},
			"0x37bfd22f8bc7758638dcc75d3d8a10cff677d896": {
				"ticker": "YETI",
				"address": "0x37bfd22F8BC7758638dCc75d3d8A10CFf677d896",
				"name": "Yeti Finance",
				"decimals": 6
			},
			"0x1fdf160e97a4d3937dc253e044e950fbce7cd8bc": {
				"ticker": "ICE",
				"address": "0x1fDF160e97a4d3937dC253E044e950fbCE7Cd8Bc",
				"name": "ICE DAO",
				"decimals": 9
			},
			"0x97bb36f8df689e0ca3b58fddc316b8514e86c5a7": {
				"ticker": "COOKIE",
				"address": "0x97bB36F8dF689E0cA3b58FddC316b8514E86C5A7",
				"name": "Cookie",
				"decimals": 18
			},
			"0x0fbaee98db86abaf42b545939cab0e7c70700125": {
				"ticker": "MAG",
				"address": "0x0fBAeE98dB86ABaf42b545939Cab0e7C70700125",
				"name": "MagnetDAO",
				"decimals": 18
			},
			"0x80585b615a15dfacf40127b1d2322670edddab2e": {
				"ticker": "POWL",
				"address": "0x80585b615A15dfacf40127b1D2322670eDdDaB2e",
				"name": "Powell Printer",
				"decimals": 6
			},
			"0x307f5b13092f0ee833c8247b393bea75a764df37": {
				"ticker": "Zbm",
				"address": "0x307f5B13092f0eE833c8247B393bEa75A764df37",
				"name": "ZeubCoin",
				"decimals": 18
			},
			"0x583a030be86f986d54dfa73e2d55b75a9ff1500c": {
				"ticker": "Avav",
				"address": "0x583a030bE86f986d54dFa73E2d55b75A9ff1500c",
				"name": "Avav",
				"decimals": 18
			},
			"0x26fa8026fb02ec7d2966c3e1843a79a594917b12": {
				"ticker": "TEST",
				"address": "0x26FA8026Fb02eC7d2966C3E1843a79A594917b12",
				"name": "Test",
				"decimals": 18
			},
			"0x0c996a1a73794a9bf00c0766de53f323c2194637": {
				"ticker": "SMN",
				"address": "0x0c996A1A73794a9Bf00C0766DE53F323c2194637",
				"name": "SmartNodes",
				"decimals": 18
			},
			"0xac4a04bc6d9750871d3ecc5143fe0b4868a32a7b": {
				"ticker": "Avast",
				"address": "0xAc4a04Bc6D9750871d3eCC5143FE0b4868A32A7B",
				"name": "Avast",
				"decimals": 9
			},
			"0xf5c09ee197d1e393c6cf37f4852b562fac0566da": {
				"ticker": "WAVAX",
				"address": "0xF5C09Ee197d1E393c6Cf37F4852B562Fac0566Da",
				"name": "Wrapped AVAX",
				"decimals": 18
			},
			"0x8e07269b6426f3d331c32d4b2ff609cfb4214ae1": {
				"ticker": "WAVAX",
				"address": "0x8e07269B6426F3D331c32d4b2fF609cFB4214aE1",
				"name": "Wrapped AVAX",
				"decimals": 18
			},
			"0x3fced31f003b403403e07a9d0dcfcf750a8ff53f": {
				"ticker": "WAXAV",
				"address": "0x3FCeD31f003B403403E07a9d0DcFCF750a8FF53F",
				"name": "Wrapped AVAX",
				"decimals": 18
			},
			"0x94b256af82f82bb878977e7f312ab34d15704b29": {
				"ticker": "sJoe",
				"address": "0x94b256aF82f82Bb878977e7F312AB34D15704b29",
				"name": "supplyJoe",
				"decimals": 18
			},
			"0xdc9e71ff5f80e8905e3020df1967fdef5fca9127": {
				"ticker": "WAVAX",
				"address": "0xdC9E71Ff5F80e8905E3020dF1967fDEf5FCa9127",
				"name": "Wrapped AVAX",
				"decimals": 18
			},
			"0x2c791261b19241265241d0f80c9dd06d16566037": {
				"ticker": "IDM",
				"address": "0x2C791261B19241265241D0F80c9DD06D16566037",
				"name": "Idemand ",
				"decimals": 18
			},
			"0x86124f2a9fddd6badf5b2e8c5f8e101b78a16cfb": {
				"ticker": "WAVAX",
				"address": "0x86124f2A9fDDD6BadF5B2e8C5f8e101B78a16CFB",
				"name": "Wrapped AVAX",
				"decimals": 18
			},
			"0x3197838de2df096f5c5af3273f466d568051680f": {
				"ticker": "RIU",
				"address": "0x3197838De2df096F5C5Af3273f466D568051680f",
				"name": "Roy Inu",
				"decimals": 18
			},
			"0x9a38568ac6acc415f520d40e487e2ecb2ca81121": {
				"ticker": "WAVAX",
				"address": "0x9A38568aC6aCc415f520D40E487e2EcB2CA81121",
				"name": "Wrapped AVAX",
				"decimals": 18
			},
			"0x349ef135d64f1ecf66954be0c6cd2d4020be2e10": {
				"ticker": "TSNAKE",
				"address": "0x349Ef135d64F1eCF66954be0c6cd2d4020be2E10",
				"name": "Snake Finance Shares",
				"decimals": 18
			},
			"0x9d871bd3d730f416d5d06ff8bc17be4119070552": {
				"ticker": "TERRA",
				"address": "0x9d871BD3D730F416D5d06Ff8BC17bE4119070552",
				"name": "AVATerra Finance",
				"decimals": 18
			},
			"0x4ec58f9d205f9c919920313932cc71ec68d123c7": {
				"ticker": "Splash",
				"address": "0x4ec58f9D205F9c919920313932cc71EC68d123C7",
				"name": "Splash Token",
				"decimals": 18
			},
			"0x4388c9780dfecadb19df944024f5c7c7cd8c8516": {
				"ticker": "SNOWY",
				"address": "0x4388c9780dFECaDb19df944024F5c7c7cd8C8516",
				"name": "FAKESNOWY",
				"decimals": 18
			},
			"0xbd918ed441767fe7924e99f6a0e0b568ac1970d9": {
				"ticker": "PGL",
				"address": "0xbd918Ed441767fe7924e99F6a0E0B568ac1970D9",
				"name": "Pangolin Liquidity",
				"decimals": 18
			},
			"0xe531974fb74be94ce6311c667dd69de1a52c0966": {
				"ticker": "SPEAR",
				"address": "0xE531974fB74be94Ce6311c667dd69de1a52C0966",
				"name": "Spear Finance",
				"decimals": 6
			},
			"0xc376878e12274a44fe360af5ab4e9abd739cff28": {
				"ticker": "PIGGY",
				"address": "0xc376878E12274A44fE360Af5AB4e9abD739Cff28",
				"name": "PIGGY",
				"decimals": 18
			},
			"0xb54f16fb19478766a268f172c9480f8da1a7c9c3": {
				"ticker": "TIME",
				"address": "0xb54f16fB19478766A268F172C9480f8da1a7c9C3",
				"name": "Time",
				"decimals": 9
			},
			"0x92d3152d89bd55111e9b8acac34513b94aa9f269": {
				"ticker": "SMN",
				"address": "0x92d3152D89BD55111e9b8AcAC34513b94Aa9f269",
				"name": "SmartNodes",
				"decimals": 18
			},
			"0xb002542f5d3acaba29257749350d117fa73a0333": {
				"ticker": "SECRET",
				"address": "0xb002542f5d3aCaBa29257749350d117FA73A0333",
				"name": "Secret DAO",
				"decimals": 9
			},
			"0x877ade8c5f4f9cccede050beb1f1a52554c707ce": {
				"ticker": "SRA",
				"address": "0x877ADe8c5F4F9CccEDE050BeB1F1a52554C707ce",
				"name": "Sierra",
				"decimals": 9
			},
			"0xa76f6c51b2cf38d1f0c9df89d322ea111fc6c1b2": {
				"ticker": "WZC",
				"address": "0xa76f6c51B2Cf38d1f0C9dF89d322eA111fc6c1B2",
				"name": "WizCoin",
				"decimals": 18
			},
			"0x75ea33f2bef079ce352474acf3b790bb77d8e7c7": {
				"ticker": "BDAO",
				"address": "0x75Ea33f2bEF079ce352474ACf3B790bb77D8E7C7",
				"name": "Bamboo DAO Shares",
				"decimals": 18
			},
			"0x714f020c54cc9d104b6f4f6998c63ce2a31d1888": {
				"ticker": "FITFI",
				"address": "0x714f020C54cc9D104B6F4f6998C63ce2a31D1888",
				"name": "STEP.APP",
				"decimals": 18
			},
			"0x7b51214dd60dec837102f28d0ece3d42b0c55a7c": {
				"ticker": "ZSHARE",
				"address": "0x7b51214dD60DEc837102f28d0eCE3d42B0C55a7c",
				"name": "Zilla Shares",
				"decimals": 18
			},
			"0x5c63077b6d86a5273383b702e1687ee16495e810": {
				"ticker": "AVAXAPE",
				"address": "0x5c63077B6d86A5273383B702E1687ee16495E810",
				"name": "AvaxApecoin",
				"decimals": 18
			},
			"0x3ae5e35848089fa0a714b165a8e5ff1646f0fc61": {
				"ticker": "Mashedpotatoes",
				"address": "0x3AE5e35848089fA0a714B165A8E5FF1646F0Fc61",
				"name": "Mashed potatoes",
				"decimals": 18
			},
			"0x98cc5567fd027f41bbd0401569f54e0f11032aa2": {
				"ticker": "ELN",
				"address": "0x98Cc5567FD027F41BbD0401569F54E0f11032AA2",
				"name": "Elon AI",
				"decimals": 9
			},
			"0x872a436059406ef61456d57442bab9f92e73a4c7": {
				"ticker": "SMOKA",
				"address": "0x872a436059406EF61456D57442BAb9f92E73A4C7",
				"name": "SMOKA",
				"decimals": 18
			},
			"0x130966628846bfd36ff31a822705796e8cb8c18d": {
				"ticker": "MIM",
				"address": "0x130966628846BFd36ff31a822705796e8cb8C18D",
				"name": "Magic Internet Money",
				"decimals": 18
			},
			"0x88d4656ce12613938d97d05f12d6f0072ad2799b": {
				"ticker": "LVL",
				"address": "0x88d4656cE12613938d97D05f12D6f0072Ad2799B",
				"name": "Levels DAO",
				"decimals": 9
			},
			"0x06f1f7021f83b07a1417d7c2e8694152950225a6": {
				"ticker": "SueDaniel",
				"address": "0x06f1f7021F83B07a1417d7C2e8694152950225a6",
				"name": "SueDaniel",
				"decimals": 18
			},
			"0xe0666a726ef6cba4baed0569664d4cf6a261aa44": {
				"ticker": "DCA",
				"address": "0xE0666A726ef6Cba4BAed0569664d4cF6a261Aa44",
				"name": "Diega",
				"decimals": 18
			},
			"0x8cd4d8e7e3ac80ce51c46fdb9380b13afe7368cd": {
				"ticker": "APE",
				"address": "0x8cd4D8e7E3ac80CE51C46FdB9380b13aFE7368CD",
				"name": "APE Nodes",
				"decimals": 9
			},
			"0x66d3a5344cfffc6017b53e03285bca04d3c73a4f": {
				"ticker": "CHAD",
				"address": "0x66D3A5344CfFFC6017b53E03285bCA04D3c73a4F",
				"name": "GigaChad",
				"decimals": 18
			},
			"0x563ca8141849cdbc70201554b6cf20a2d469ea81": {
				"ticker": "FVAX",
				"address": "0x563CA8141849CdBC70201554b6CF20A2D469EA81",
				"name": "FUCVAX",
				"decimals": 8
			},
			"0x623893ca6e8963b637e9e69453192105cc83aff1": {
				"ticker": "SNAKE",
				"address": "0x623893ca6E8963b637e9E69453192105cc83AFF1",
				"name": "Snake Finance Token",
				"decimals": 18
			},
			"0x1b4372761970c3b4d22da33a3a47e15c42a82901": {
				"ticker": "TEST",
				"address": "0x1B4372761970c3b4D22DA33A3a47E15C42A82901",
				"name": "TEST",
				"decimals": 18
			},
			"0xc87a233bd0047a3e4921d806f27b51b838d4c102": {
				"ticker": "$UNO",
				"address": "0xc87a233BD0047A3E4921D806f27b51B838D4c102",
				"name": "Unknown",
				"decimals": 18
			},
			"0x0da67235dd5787d67955420c84ca1cecd4e5bb3b": {
				"ticker": "wMEMO",
				"address": "0x0da67235dD5787D67955420C84ca1cEcd4E5Bb3b",
				"name": "Wrapped MEMO",
				"decimals": 18
			},
			"0x82267c56b05aff3742b997412b3aae32f9a087ae": {
				"ticker": "RED",
				"address": "0x82267C56B05Aff3742B997412B3aae32F9a087Ae",
				"name": "RedTriangleDAO",
				"decimals": 18
			},
			"0xd3bfbe18c8536100e8a93def0ae30f58b860336f": {
				"ticker": "SHINA",
				"address": "0xD3bFbe18C8536100e8A93dEf0ae30f58b860336f",
				"name": "Shinavax",
				"decimals": 18
			},
			"0x579dad9d765e5a3bd1acf6aa5078fd667b76dc64": {
				"ticker": "RUBY",
				"address": "0x579DAd9D765E5a3bD1AcF6aa5078fd667b76dc64",
				"name": "DeDragon Ruby Token",
				"decimals": 18
			},
			"0xea86bea511792653edcc219e9cc4c09b5df433b0": {
				"ticker": "TOKEN1",
				"address": "0xEA86bEa511792653eDCc219e9CC4c09b5Df433b0",
				"name": "Token1",
				"decimals": 9
			},
			"0xbce9ee1f533807152e47e4726fa88290cd3e6c8b": {
				"ticker": "HIVE",
				"address": "0xbCE9ee1f533807152E47e4726fA88290cd3E6C8B",
				"name": "Hive Nodes",
				"decimals": 9
			},
			"0x3e14a07fedb0d1c10b8d52b70ae99179d888d089": {
				"ticker": "SMN",
				"address": "0x3e14a07FEdB0D1C10B8D52B70ae99179D888d089",
				"name": "SmartNodes",
				"decimals": 18
			},
			"0x2243a1a75330a9f10d72df9b7862a4705bef9620": {
				"ticker": "TSNAKE",
				"address": "0x2243a1a75330a9F10D72DF9b7862A4705bEF9620",
				"name": "Snake Finance Shares",
				"decimals": 18
			},
			"0xa03afdfbca422b03210a598b3386338ad9c5f422": {
				"ticker": "LEMONADE",
				"address": "0xa03AfDfBCA422B03210A598b3386338AD9c5F422",
				"name": "Lemonade Shares",
				"decimals": 18
			},
			"0x7a574a74feb6eb5d660b37867e3ec56943696d8d": {
				"ticker": "SMART",
				"address": "0x7a574A74fEb6eb5d660b37867E3Ec56943696D8d",
				"name": "Smart Nodes",
				"decimals": 9
			},
			"0xa3cfa54ac4dded16deab21f253e17b3c0224cded": {
				"ticker": "SSOD",
				"address": "0xA3cfA54aC4dDeD16DEaB21F253e17b3C0224cDEd",
				"name": "SantaShibaOlympusDAO",
				"decimals": 9
			},
			"0xa27097b1b56e01ab888029a60ae2cd7f010948a7": {
				"ticker": "LGS",
				"address": "0xA27097B1b56e01AB888029a60aE2cD7f010948a7",
				"name": "Lunar Galaxy Shiba",
				"decimals": 18
			},
			"0xb4305c8cfe3ef9eaaa9bb5766980f128fe37ac4c": {
				"ticker": "OP",
				"address": "0xb4305C8CFe3Ef9eAAa9bb5766980f128FE37ac4c",
				"name": "Optimism",
				"decimals": 18
			},
			"0xc0cd51030c03b8c5cbb76cb5efad72f126aa471f": {
				"ticker": "NEXT",
				"address": "0xc0Cd51030c03B8c5cbb76cb5efAD72f126aa471f",
				"name": "NEXT NODE",
				"decimals": 18
			},
			"0x4dd81c44b35f5b748025474021389d7c2613d610": {
				"ticker": "FTN",
				"address": "0x4dD81C44B35f5B748025474021389D7c2613D610",
				"name": "Fortune DAO",
				"decimals": 9
			},
			"0xc4cad276a2ef981d81fadbb9597885854077252d": {
				"ticker": "HPT",
				"address": "0xC4CAd276a2Ef981d81fADbb9597885854077252d",
				"name": "HighPoint Finance",
				"decimals": 18
			},
			"0x7b536aa4deaa421f0cc42ae26c9362001cdd2c92": {
				"ticker": "DBY",
				"address": "0x7B536aa4DEaa421F0CC42ae26C9362001CdD2C92",
				"name": "MetaDerby",
				"decimals": 6
			},
			"0x78df43745206757719d4ec6e07940585d44e960d": {
				"ticker": "LAVA",
				"address": "0x78DF43745206757719d4EC6E07940585D44e960d",
				"name": "Lava Financial",
				"decimals": 18
			},
			"0xe8ac210cd57d5672c24d7a88e8940897ecfa493c": {
				"ticker": "TEST",
				"address": "0xe8ac210Cd57D5672c24D7a88E8940897ecfA493C",
				"name": "TEST",
				"decimals": 8
			},
			"0x72428830370141376a6312511c5fcfaa592f3f82": {
				"ticker": "XSHARES",
				"address": "0x72428830370141376a6312511c5fCFaA592F3F82",
				"name": "XSHARE Token",
				"decimals": 18
			},
			"0x4e9c808d7e94012b450f054a91d24076fbd75455": {
				"ticker": "FIREP",
				"address": "0x4e9C808D7E94012B450F054a91D24076fBD75455",
				"name": "FirePheonix",
				"decimals": 18
			},
			"0x96d54cca5b017117bea9de39943e707045913dec": {
				"ticker": "PINATA",
				"address": "0x96d54cCA5b017117Bea9DE39943E707045913deC",
				"name": "NFT Management",
				"decimals": 9
			},
			"0x90bfe464e6290617cbc7bbd411eeb04ed09829f9": {
				"ticker": "WLH",
				"address": "0x90BFe464E6290617cbC7Bbd411eeb04ed09829f9",
				"name": "WLH",
				"decimals": 18
			},
			"0x244433fc967732d63376acd68eabc10b343568b9": {
				"ticker": "XACA",
				"address": "0x244433FC967732D63376ACD68EAbc10B343568B9",
				"name": "XACA",
				"decimals": 18
			},
			"0x5d34bf45218e371f30ab0eedc621de7d5a69a07b": {
				"ticker": "BANANA",
				"address": "0x5d34bF45218e371F30ab0eEdC621DE7d5A69A07b",
				"name": "Gorilla Nodes",
				"decimals": 18
			},
			"0xd04f2aa1edee96512b22feca678fa78638e907a7": {
				"ticker": "MOB",
				"address": "0xD04F2aA1edee96512b22FecA678FA78638E907A7",
				"name": "MOB DAO",
				"decimals": 9
			},
			"0x506f9344f9633e06c5561e538b17c28fe3ffc1fa": {
				"ticker": "WOLF",
				"address": "0x506f9344f9633E06c5561E538b17C28fe3Ffc1fA",
				"name": "Wolf Inu",
				"decimals": 9
			},
			"0x1b97861e216550eb9b09dfa4ff32dc3a83971892": {
				"ticker": "DGEC",
				"address": "0x1b97861E216550eb9B09dfa4FF32Dc3A83971892",
				"name": "Doge Cash",
				"decimals": 9
			},
			"0xb8f4e74aadafffd715d510b5f36e7eca23c96aef": {
				"ticker": "JRNY",
				"address": "0xB8f4e74AadAfFfD715D510b5F36e7Eca23C96AEf",
				"name": "JRNY Crypto",
				"decimals": 18
			},
			"0xed8cbd9f0ce3c6986b22002f03c6475ceb7a6256": {
				"ticker": "JLP",
				"address": "0xeD8CBD9F0cE3C6986b22002F03c6475CEb7a6256",
				"name": "Joe LP Token",
				"decimals": 18
			},
			"0x0284fa9ba7a180cdbf45decf852ea66abafcfe9a": {
				"ticker": "KEVIN",
				"address": "0x0284FA9Ba7a180cDBf45DECf852eA66abAfCFe9A",
				"name": "Kevin Shares",
				"decimals": 18
			},
			"0xc0058fcd9e833a5bb23885f6ea5aea778b1b0fa1": {
				"ticker": "AVIBAINU",
				"address": "0xc0058fcD9E833A5bB23885f6ea5aEa778b1b0Fa1",
				"name": "Aviba Inu",
				"decimals": 18
			},
			"0x1e150ef5c7e2e5560459bbe421d44f8dd625753b": {
				"ticker": "YUL",
				"address": "0x1E150EF5C7E2e5560459bBe421d44f8dD625753b",
				"name": "Yuli",
				"decimals": 18
			},
			"0xde6e8c3bf7068db1a76981f73e27a1bdd6b6c1f0": {
				"ticker": "SELN",
				"address": "0xDE6E8c3Bf7068DB1A76981f73E27a1BDD6b6C1f0",
				"name": "Safe Elon",
				"decimals": 9
			},
			"0x23324e22068de19d789ea3e46fc0ebabb987c905": {
				"ticker": "NEBULA",
				"address": "0x23324e22068dE19D789EA3E46Fc0eBABB987c905",
				"name": "Nebula Nodes",
				"decimals": 18
			},
			"0x13909fe20a5f656512437dbb59e8a422551af17a": {
				"ticker": "AUTO",
				"address": "0x13909fe20A5F656512437dBB59E8a422551aF17A",
				"name": "Auto Nodes Airdrops Every 8Hrs",
				"decimals": 9
			},
			"0xb2ac04b71888e17aa2c5102cf3d0215467d74100": {
				"ticker": "bAVAX",
				"address": "0xB2AC04b71888E17Aa2c5102cf3d0215467D74100",
				"name": "Blizz AVAX",
				"decimals": 18
			},
			"0x4bfefe904aa2654cd3da3448de4a571bb63bcece": {
				"ticker": "LAVA",
				"address": "0x4bfEfE904AA2654Cd3dA3448de4A571bB63bcEcE",
				"name": "Lava Financial",
				"decimals": 9
			},
			"0x6c68b34bfe8a0d453b1b1b47b34ee255d2565f23": {
				"ticker": "TDAO",
				"address": "0x6c68b34bFE8a0D453B1B1B47B34eE255d2565f23",
				"name": "THE DAO",
				"decimals": 18
			},
			"0xd8f701ca4312009f6cdf295eea7e18336d40e29b": {
				"ticker": "ARSW",
				"address": "0xD8F701CA4312009f6cDF295eEa7E18336d40e29b",
				"name": "ArthSwap",
				"decimals": 18
			},
			"0x92876c3a3e2b0788c841587a14989392a555bffe": {
				"ticker": "MIND+",
				"address": "0x92876C3A3E2B0788C841587a14989392A555BffE",
				"name": "MIND+",
				"decimals": 18
			},
			"0x4e63fc43fc88b2b8d0d4ae89038847e1aeccdb34": {
				"ticker": "HCHR",
				"address": "0x4E63Fc43fC88B2b8d0D4ae89038847E1aecCdB34",
				"name": "Hachi Robot",
				"decimals": 9
			},
			"0x8729438eb15e2c8b576fcc6aecda6a148776c0f5": {
				"ticker": "QI",
				"address": "0x8729438EB15e2C8B576fCc6AeCdA6A148776C0F5",
				"name": "BENQI",
				"decimals": 18
			},
			"0x71682f67e8a8de95499901d1041d81e25251ae49": {
				"ticker": "JET",
				"address": "0x71682f67E8A8de95499901D1041D81e25251Ae49",
				"name": "JET DAO",
				"decimals": 9
			},
			"0x10720a029a5e5281391be3181477e48d47d0ff91": {
				"ticker": "sSDOG",
				"address": "0x10720A029A5e5281391Be3181477e48d47D0fF91",
				"name": "Staked Snowdog",
				"decimals": 9
			},
			"0xe360ba74411196b949a90db52fc79192a7b22b06": {
				"ticker": "HP",
				"address": "0xe360bA74411196B949A90dB52fc79192A7b22b06",
				"name": "Hypothermia DAO (3,3)",
				"decimals": 18
			},
			"0xab250dbd5df742da488fa69f7b7851caf26bbc0b": {
				"ticker": "XSHARE",
				"address": "0xaB250DbD5Df742dA488fA69f7B7851CaF26bBc0B",
				"name": "XSHARE",
				"decimals": 18
			},
			"0x6e84a6216ea6dacc71ee8e6b0a5b7322eebc0fdd": {
				"ticker": "JOE",
				"address": "0x6e84a6216eA6dACC71eE8E6b0a5B7322EEbC0fDd",
				"name": "JoeToken",
				"decimals": 18
			},
			"0xe6b45fc9f3c6c6958bb10acda571b4979a98842f": {
				"ticker": "DGC",
				"address": "0xe6B45Fc9F3C6c6958BB10AcdA571B4979a98842f",
				"name": "Digit Coin",
				"decimals": 10
			},
			"0x0592bf44a8376ce0004acf439bc5dd2332273608": {
				"ticker": "SKY",
				"address": "0x0592BF44a8376Ce0004Acf439bc5DD2332273608",
				"name": "SKY DAO",
				"decimals": 9
			},
			"0x21ca236b0f7610bec588c4e5ea5a5f5faced569e": {
				"ticker": "SGEM",
				"address": "0x21cA236b0f7610BEc588c4E5ea5a5f5faCED569E",
				"name": "SGEM",
				"decimals": 18
			},
			"0xa9d4afe22b54758d8d4d96b12030b1c6166df0ae": {
				"ticker": "TEST",
				"address": "0xA9d4AFE22b54758d8D4d96b12030B1C6166Df0ae",
				"name": "TEST",
				"decimals": 18
			},
			"0x620aa0146a6e28a7003c83b93ada0e9df4beccc9": {
				"ticker": "$UNO",
				"address": "0x620aA0146A6E28A7003c83b93AdA0e9dF4BeCcc9",
				"name": "Unknown Nodes",
				"decimals": 18
			},
			"0xa6c022ac29864f321a6874333dca805fddf94275": {
				"ticker": "MSNPP",
				"address": "0xa6C022AC29864F321A6874333DCa805fddF94275",
				"name": "MSNPP",
				"decimals": 6
			},
			"0x1e5b9912c987e3ce5af39e53f0f9dc9a3ce1da3e": {
				"ticker": "BPNG",
				"address": "0x1e5b9912c987e3cE5Af39e53f0f9Dc9a3CE1DA3E",
				"name": "Baby Pangolin",
				"decimals": 9
			},
			"0xf04d0ac92ac1467ee723cd965a0173de143bf31f": {
				"ticker": "Andromeda",
				"address": "0xF04D0Ac92Ac1467EE723CD965a0173De143bF31f",
				"name": "Andromeda Node",
				"decimals": 18
			},
			"0x63b30332bcec99df7e828b2190a896d241393dcb": {
				"ticker": "SATOSHI",
				"address": "0x63b30332BCec99DF7E828b2190a896d241393dCB",
				"name": "Satoshi Nodes",
				"decimals": 9
			},
			"0x845a17b279b5ed4aaf7959c01ca31c20219eb102": {
				"ticker": "GHST",
				"address": "0x845a17b279b5eD4AaF7959C01ca31C20219eb102",
				"name": "Safe Ghost",
				"decimals": 12
			},
			"0x296865984b9a240d3fc50077a1bf28deeec164cf": {
				"ticker": "STBL",
				"address": "0x296865984B9A240d3fC50077a1bF28DEEEc164cf",
				"name": "Stable DAO",
				"decimals": 9
			},
			"0xebe5f11de6a3e64a8c2743704acebe24b8c55741": {
				"ticker": "TRFC",
				"address": "0xEbE5F11DE6A3E64a8c2743704aceBE24B8C55741",
				"name": "Terrific Finance",
				"decimals": 18
			},
			"0x06fb02557cc34046ea13dca0b93ae3b3757ce0a3": {
				"ticker": "RABBIT",
				"address": "0x06FB02557CC34046ea13dca0B93ae3B3757Ce0A3",
				"name": "White Rabbit DAO",
				"decimals": 18
			},
			"0x8ace91b1c72e4f54d2144a4083ef97f8bc97b690": {
				"ticker": "STR",
				"address": "0x8Ace91b1c72E4f54d2144A4083ef97F8bc97b690",
				"name": "Wrapped Star",
				"decimals": 9
			},
			"0xd8330119693ed135411bafc9c1b2a95da0f1ae5b": {
				"ticker": "ALOT",
				"address": "0xd8330119693ED135411BAfc9C1b2a95dA0f1aE5B",
				"name": "Dexalot Token",
				"decimals": 18
			},
			"0x70ed511631bca9b14e640a57433a497f181fe521": {
				"ticker": "LOST",
				"address": "0x70eD511631Bca9B14E640a57433A497f181FE521",
				"name": "LOST WORLDS",
				"decimals": 6
			},
			"0xcfe4e074e8829855f3c2ba2c454344520fd1b00e": {
				"ticker": "MAX",
				"address": "0xcfe4E074E8829855f3c2bA2c454344520FD1b00E",
				"name": "Maximum",
				"decimals": 9
			},
			"0x80c46da5634ffd6addc8452b44fb4d886bcd8025": {
				"ticker": "SUKI",
				"address": "0x80C46Da5634ffd6AdDc8452b44FB4d886BCD8025",
				"name": "Sushi King",
				"decimals": 9
			},
			"0xa029698ff926592a46bccbd32012ca158ccdd6ef": {
				"ticker": "ELON",
				"address": "0xA029698fF926592a46bCcbd32012Ca158cCDd6eF",
				"name": "Musk Money",
				"decimals": 9
			},
			"0xdd8bcf5ff620d74cb8b7d201a78d7e77b7e1026a": {
				"ticker": "MSHARE",
				"address": "0xDD8bcF5Ff620D74cB8b7D201A78D7E77B7E1026A",
				"name": "MoonShine Share",
				"decimals": 18
			},
			"0x393ee3148478db3e4414c21e58da4f27e0c262bd": {
				"ticker": "DIVD",
				"address": "0x393Ee3148478dB3E4414C21e58Da4f27e0c262bd",
				"name": "ReflectionUSD",
				"decimals": 9
			},
			"0xfcde4a87b8b6fa58326bb462882f1778158b02f1": {
				"ticker": "WXT",
				"address": "0xfcDe4A87b8b6FA58326BB462882f1778158B02F1",
				"name": "Wirex Token",
				"decimals": 18
			},
			"0x26f76fe8be2817eaff22d6fbe5a8a20d0a359cde": {
				"ticker": "ELON",
				"address": "0x26F76fe8be2817eAFF22d6FBe5A8A20d0A359CDE",
				"name": "Zen Elon",
				"decimals": 9
			},
			"0xc1c5ad9ccc2e1717fca015b6656510c49f80a419": {
				"ticker": "WINGS",
				"address": "0xC1C5ad9CcC2e1717fCA015b6656510C49f80a419",
				"name": "Angel Fund",
				"decimals": 10
			},
			"0xe4576777c529e2b7eadde1ce65eb60415c362968": {
				"ticker": "ANKR",
				"address": "0xE4576777C529E2B7eaddE1cE65EB60415c362968",
				"name": "ANKR Nodes",
				"decimals": 9
			},
			"0x1f4c82b1aa78fb825a9eaec8a674b77f5b99739d": {
				"ticker": "TNODES",
				"address": "0x1f4C82B1Aa78FB825a9eaec8a674B77f5B99739d",
				"name": "TNODES",
				"decimals": 18
			},
			"0xb9d5ae23e5bf54e9731f1f2224109bb6aa557d86": {
				"ticker": "MAX",
				"address": "0xb9D5AE23e5bF54e9731F1F2224109bb6aa557D86",
				"name": "AVA MAX",
				"decimals": 9
			},
			"0xb952df7188ac2f81eee0c4c9aaade6ec4a5eea06": {
				"ticker": "STORMZ",
				"address": "0xb952Df7188AC2F81eee0C4C9aAAde6Ec4a5eeA06",
				"name": "StormNodes",
				"decimals": 18
			},
			"0x4d81911f0e30d2e12dcc954091701b39dc59e34a": {
				"ticker": "MILK",
				"address": "0x4D81911F0E30D2E12dcc954091701B39dC59e34a",
				"name": "Dairy.Money MILK",
				"decimals": 18
			},
			"0x17be13583192151c181a6f6ec9d11a4697f4e839": {
				"ticker": "RISE",
				"address": "0x17be13583192151c181a6F6ec9D11a4697f4E839",
				"name": "EverRise",
				"decimals": 18
			},
			"0xef236318fb9f88df7f77aa6354d035e434cef8d3": {
				"ticker": "MAG",
				"address": "0xEF236318FB9f88DF7f77AA6354d035e434ceF8d3",
				"name": "Magic DAO",
				"decimals": 9
			},
			"0xc3a8d300333bffe3ddf6166f2bc84e6d38351bed": {
				"ticker": "RISE",
				"address": "0xC3A8d300333BFfE3ddF6166F2Bc84E6d38351BED",
				"name": "EverRise",
				"decimals": 18
			},
			"0x813539bb86e08948421afd7d7311b6138503a85f": {
				"ticker": "CLAW",
				"address": "0x813539Bb86e08948421Afd7D7311b6138503a85F",
				"name": "PantherNodes",
				"decimals": 18
			},
			"0x97c98876198064db718a4a9c2d46ef38a4ce8885": {
				"ticker": "AWOOL",
				"address": "0x97c98876198064DB718a4A9c2D46ef38A4ce8885",
				"name": "AVAX WOLF",
				"decimals": 18
			},
			"0x60c0b0c3b97c4c1b5c74995cd3e6deebc2c55c78": {
				"ticker": "BDF",
				"address": "0x60C0B0C3B97c4C1B5c74995Cd3E6deebC2c55c78",
				"name": "Bandung Financial",
				"decimals": 18
			},
			"0xf52d08815b74e3cdd733735c79d3d0bb9e0b7a28": {
				"ticker": "OGHS",
				"address": "0xF52D08815b74e3cdD733735C79d3d0BB9E0B7A28",
				"name": "OMG Ghost",
				"decimals": 10
			},
			"0x371985c647a59b387d5223c60303e51d446159e5": {
				"ticker": "GHST",
				"address": "0x371985C647a59B387d5223c60303e51D446159E5",
				"name": "Ghost Markets",
				"decimals": 11
			},
			"0x712b05710bcfe9c9b1df08d0f846f42c3d457f3e": {
				"ticker": "SMRT",
				"address": "0x712b05710bcFe9C9B1DF08D0f846f42c3D457F3E",
				"name": "SmartNodes",
				"decimals": 18
			},
			"0x71f78f6884acf32129b728157615540aadb579c6": {
				"ticker": "Milk",
				"address": "0x71F78f6884Acf32129b728157615540aAdB579C6",
				"name": "Milk",
				"decimals": 6
			},
			"0x2b2c81e08f1af8835a78bb2a90ae924ace0ea4be": {
				"ticker": "sAVAX",
				"address": "0x2b2C81e08f1Af8835a78Bb2A90AE924ACE0eA4bE",
				"name": "Staked AVAX",
				"decimals": 18
			},
			"0x252f42147acc1733c295fa24ba31b81c8c48dab7": {
				"ticker": "ASTRO",
				"address": "0x252F42147aCC1733c295FA24bA31b81C8C48dAb7",
				"name": "100 Days",
				"decimals": 18
			},
			"0x2b36193806eb1569a3163cf5d4371038e0495179": {
				"ticker": "METAG",
				"address": "0x2b36193806EB1569A3163cf5D4371038e0495179",
				"name": "Metagamz",
				"decimals": 18
			},
			"0x3653d958809dbf01413a7f1e5eb3abd01f198093": {
				"ticker": "TED",
				"address": "0x3653d958809DbF01413a7f1e5eB3aBd01f198093",
				"name": "Teddy Cash (https://teddy.cash)",
				"decimals": 9
			},
			"0x2059cb12f806524de2cf256a784fc84b3788006f": {
				"ticker": "BAT",
				"address": "0x2059CB12F806524De2Cf256a784fc84B3788006f",
				"name": "Battle Nodes",
				"decimals": 9
			},
			"0x99b5605b2fdf669736f9f7bc51fcca73c4eba1c9": {
				"ticker": "SUP",
				"address": "0x99b5605b2Fdf669736F9F7BC51fccA73c4eBA1C9",
				"name": "SuperBowl DAO",
				"decimals": 9
			},
			"0x36e901721b372d8d68ef5706c24b2fa5ad4ecaa8": {
				"ticker": "NEIBR",
				"address": "0x36E901721B372D8D68ef5706c24B2fA5aD4ECaA8",
				"name": "Neighbours",
				"decimals": 18
			},
			"0x7ffa6d2944694886f16c2672dbdb603cf2645f4a": {
				"ticker": "SMOKA",
				"address": "0x7fFA6d2944694886F16C2672DBdB603cF2645F4A",
				"name": "SMOKA",
				"decimals": 18
			},
			"0x15ddba24b479f82f1c8d1376d787f7e9af3b28e8": {
				"ticker": "Vegetables",
				"address": "0x15dDba24b479F82f1c8d1376D787F7e9AF3b28e8",
				"name": "Vegetables",
				"decimals": 18
			},
			"0x6fd8cb76ab6bd4c12a5b27606606039a622b9024": {
				"ticker": "GLD",
				"address": "0x6Fd8cb76Ab6BD4C12a5b27606606039a622B9024",
				"name": "Gold Project",
				"decimals": 11
			},
			"0x869c84885304187a383ee2d26218c0fbf6a79c87": {
				"ticker": "APAY",
				"address": "0x869c84885304187A383ee2d26218C0FBF6A79C87",
				"name": "AVA PAY",
				"decimals": 9
			},
			"0x111111111111ed1d73f860f57b2798b683f2d325": {
				"ticker": "YUSD",
				"address": "0x111111111111ed1D73f860F57b2798b683f2d325",
				"name": "YUSD Stablecoin",
				"decimals": 18
			},
			"0xaf8db404b50b7f0bbcdf4661a4430846c63d72c1": {
				"ticker": "SSHARE",
				"address": "0xaF8Db404B50B7f0bbcdF4661a4430846C63D72C1",
				"name": "SSHARE",
				"decimals": 18
			},
			"0x914c5cf10a4bd9c0889a30109a442e3ae3bc154a": {
				"ticker": "APL",
				"address": "0x914C5cF10A4Bd9c0889a30109a442E3aE3Bc154A",
				"name": "Apple SV",
				"decimals": 12
			},
			"0xc26dcc91b69e30c96ad22e5dda81781cf592f4be": {
				"ticker": "FKT",
				"address": "0xC26Dcc91B69e30c96aD22E5DDA81781cf592f4be",
				"name": "Flake Token",
				"decimals": 18
			},
			"0xaf8c30781136089cf61da48597e128d4fed0f063": {
				"ticker": "NEON",
				"address": "0xaf8c30781136089CF61dA48597E128D4fED0f063",
				"name": "NODEON (discord.com/invite/nodeon)",
				"decimals": 8
			},
			"0x7b12946de863209cc6b15c0a6723a43e866ddf85": {
				"ticker": "ASND",
				"address": "0x7b12946De863209CC6b15C0a6723A43e866DDf85",
				"name": "ASND",
				"decimals": 6
			},
			"0xcdd50f6a3b3da9372b34d01c2aa10c1e9df78a5e": {
				"ticker": "ELXR",
				"address": "0xCDD50f6a3b3DA9372b34d01C2aA10c1E9dF78A5E",
				"name": "Elexir",
				"decimals": 18
			},
			"0x3f2b579810e3fd4aed4aa8dc22220f504f298a5f": {
				"ticker": "AVAXS",
				"address": "0x3F2B579810E3FD4Aed4aA8DC22220F504F298A5f",
				"name": "Avalanches",
				"decimals": 18
			},
			"0x23eeeef953b505949b1aefe1989e6eb005d2accf": {
				"ticker": "DBY",
				"address": "0x23eeeEf953b505949b1AEfE1989e6Eb005D2ACCf",
				"name": "MetaDerby",
				"decimals": 6
			},
			"0x22d4002028f537599be9f666d1c4fa138522f9c8": {
				"ticker": "PTP",
				"address": "0x22d4002028f537599bE9f666d1c4Fa138522f9c8",
				"name": "Platypus",
				"decimals": 18
			},
			"0x98df877f469e80a3e18196ea2e0edbeac51bb3bb": {
				"ticker": "WSBA",
				"address": "0x98DF877f469e80a3E18196EA2e0eDbEaC51bB3Bb",
				"name": "Wrapped Shiba",
				"decimals": 9
			},
			"0x32000cd723b7ce0c62842cab132b41cb7921c36d": {
				"ticker": "PXT",
				"address": "0x32000Cd723b7ce0C62842cAB132b41cB7921c36D",
				"name": "ProjectXNodes",
				"decimals": 9
			},
			"0xd1374e0e1bb1ef2af651ae617378fcbfa4090878": {
				"ticker": "PHANTOM",
				"address": "0xd1374E0e1bB1ef2AF651ae617378fCBFa4090878",
				"name": "Phantom DAO (https://t.me/phantomdaoftm)",
				"decimals": 9
			},
			"0x3996f14d907c431d72bf9aaa75eb5b431d7c9073": {
				"ticker": "HERMES",
				"address": "0x3996F14d907c431D72bf9aaa75Eb5B431D7C9073",
				"name": "HERMESDAO",
				"decimals": 9
			},
			"0xb0181b82fc8ba4d250ed3374da0b1d8d151e84d5": {
				"ticker": "APLG",
				"address": "0xB0181B82fC8BA4d250Ed3374DA0B1d8D151E84D5",
				"name": "Apple Gaming",
				"decimals": 9
			},
			"0xa9563136f4db528e98360fac70347a3d73b7e2b7": {
				"ticker": "BUZZ",
				"address": "0xA9563136F4db528e98360fAc70347a3d73B7E2B7",
				"name": "BuzzToken",
				"decimals": 18
			},
			"0xd8516362fd6c824b991764a316b3c508adc8c67a": {
				"ticker": "XSHARES",
				"address": "0xD8516362fd6C824b991764A316B3C508AdC8c67A",
				"name": "XSHARE Token",
				"decimals": 18
			},
			"0x859b0921b783874175701fe06393f736535d5074": {
				"ticker": "WINE",
				"address": "0x859b0921b783874175701FE06393F736535d5074",
				"name": "Wine Shares",
				"decimals": 18
			},
			"0x0d21ee4333a7f190de88cbfd68c772b36097e251": {
				"ticker": "JADE",
				"address": "0x0d21Ee4333a7f190DE88CBfd68C772B36097E251",
				"name": "Jade Protocol",
				"decimals": 9
			},
			"0x27b0a85db6fe072ecf3b1f67ec24496a7c39bc93": {
				"ticker": "BRAIN",
				"address": "0x27b0A85DB6FE072ecf3B1f67eC24496a7C39bc93",
				"name": "Brain Shares",
				"decimals": 18
			},
			"0x9977e352c338112262ed1cba9f99203b29bd64ac": {
				"ticker": "ASND",
				"address": "0x9977E352c338112262ed1Cba9f99203B29bd64aC",
				"name": "ASND",
				"decimals": 6
			},
			"0x1cc9cb8d0cc686e326893112ea19aa54dea00e2f": {
				"ticker": "Genesis",
				"address": "0x1cC9cB8d0CC686e326893112EA19aa54dEA00E2f",
				"name": "Genesis Node",
				"decimals": 18
			},
			"0xd1c3f94de7e5b45fa4edbba472491a9f4b166fc4": {
				"ticker": "XAVA",
				"address": "0xd1c3f94DE7e5B45fa4eDBBA472491a9f4B166FC4",
				"name": "Avalaunch",
				"decimals": 18
			},
			"0x093783055f9047c2bff99c4e414501f8a147bc69": {
				"ticker": "ALOT",
				"address": "0x093783055F9047C2BfF99c4e414501F8A147bC69",
				"name": "Dexalot Token",
				"decimals": 18
			},
			"0x08a390242eadfd49c62cd2c4db0761c99bf0f503": {
				"ticker": "ALEPH",
				"address": "0x08a390242eAdFD49C62cd2C4DB0761C99Bf0F503",
				"name": "Aleph Finance",
				"decimals": 18
			},
			"0xf62ea66d20de6ddba4098f9045b4be650d8cced7": {
				"ticker": "BSHARE",
				"address": "0xf62eA66d20de6Ddba4098f9045b4be650d8Cced7",
				"name": "BSHARE",
				"decimals": 18
			},
			"0x37cd0b44a6cfdbb58d61a4c1f8343ae602a21f50": {
				"ticker": "TND",
				"address": "0x37CD0b44A6CfDBb58D61a4C1f8343Ae602A21F50",
				"name": "Super Thunder",
				"decimals": 11
			},
			"0xe19a1684873fab5fb694cfd06607100a632ff21c": {
				"ticker": "BAVA",
				"address": "0xe19A1684873faB5Fb694CfD06607100A632fF21c",
				"name": "BavaToken",
				"decimals": 18
			},
			"0xf59c6babd2f1fd0b7ba9b2d11991394e66f7713b": {
				"ticker": "XSHARES",
				"address": "0xf59C6babD2F1Fd0b7Ba9b2D11991394e66f7713B",
				"name": "XSHARE Token",
				"decimals": 18
			},
			"0xb7a4ca0c9b58a33b244c44a8bf9833b0e7918429": {
				"ticker": "JLP",
				"address": "0xB7a4Ca0c9B58a33B244C44a8Bf9833b0E7918429",
				"name": "Joe LP Token",
				"decimals": 18
			},
			"0x6bd3db958ffb54f3a9c4bf2ce87a481a0196985b": {
				"ticker": "DBY",
				"address": "0x6bd3dB958ffB54F3A9C4bF2CE87a481a0196985B",
				"name": "MetaDerby",
				"decimals": 6
			},
			"0x00013574315c6837f0afcb720ca201e1fbc33d55": {
				"ticker": "PAE",
				"address": "0x00013574315C6837f0Afcb720Ca201E1FbC33d55",
				"name": "Ripae Finance",
				"decimals": 18
			},
			"0xc17c30e98541188614df99239cabd40280810ca3": {
				"ticker": "RISE",
				"address": "0xC17c30e98541188614dF99239cABD40280810cA3",
				"name": "EverRise",
				"decimals": 18
			},
			"0x69be72c20ea651d53f4d7b69685055e1bf55d370": {
				"ticker": "STAR",
				"address": "0x69be72c20ea651D53f4d7B69685055E1BF55D370",
				"name": "Star Swap",
				"decimals": 10
			},
			"0xce983dc04135107ca9e59fe8102e07da5d133937": {
				"ticker": "SAC",
				"address": "0xcE983dC04135107CA9E59fE8102E07DA5d133937",
				"name": "Search Neuron",
				"decimals": 18
			},
			"0x1db2eb5f4dc18df2f9c99d5d284819d1a04fcc56": {
				"ticker": "CAKE",
				"address": "0x1DB2EB5F4dc18df2f9C99d5D284819D1A04FCC56",
				"name": "Cake Chain",
				"decimals": 9
			},
			"0x6b2ab1c6d77b4ce319ec1cfd80caadafc70e9429": {
				"ticker": "SIMIAN",
				"address": "0x6b2Ab1C6d77b4CE319eC1CFD80CAADaFc70e9429",
				"name": "Simian Nodes",
				"decimals": 18
			},
			"0xd68091d5456c8e06fc9ffd0dbb4f87a35bbe227e": {
				"ticker": "FMC",
				"address": "0xD68091d5456c8e06FC9fFd0dBB4f87A35BBE227E",
				"name": "FORCE MATIC",
				"decimals": 18
			},
			"0x7697a9d69801ffb9927621322ca34cfe3dcd149c": {
				"ticker": "Island",
				"address": "0x7697A9D69801fFb9927621322cA34CFE3DCD149c",
				"name": "TheIsland",
				"decimals": 9
			},
			"0x150f6087492b3d8a0e293f953f1a4875d001e341": {
				"ticker": "ELON",
				"address": "0x150f6087492b3d8a0E293f953F1A4875D001e341",
				"name": "Elon Chain",
				"decimals": 10
			},
			"0x951abbf5f760432557c6573f8ca5a935dc189a01": {
				"ticker": "WHITERABBIT",
				"address": "0x951aBBf5f760432557C6573F8cA5a935Dc189A01",
				"name": "WhiteRabbit",
				"decimals": 18
			},
			"0xb1ee5bd4e1cbb65f3fe32ca566fb07cb4343ac3c": {
				"ticker": "SEC",
				"address": "0xb1ee5Bd4e1cbB65f3fe32ca566FB07Cb4343ac3c",
				"name": "Secret Project",
				"decimals": 9
			},
			"0x562a15e213ec5ad651539f2f28be65965a5de65c": {
				"ticker": "CRG",
				"address": "0x562A15e213EC5aD651539f2f28be65965a5dE65C",
				"name": "Zen Corgi",
				"decimals": 11
			},
			"0x5a267d320fa4f8a8b30b44b4ee9e1d4adb00db43": {
				"ticker": "FOREST",
				"address": "0x5a267D320fA4F8a8B30b44b4Ee9E1D4adb00DB43",
				"name": "Forest Token",
				"decimals": 18
			},
			"0x20f1830a62fcb67551981174df922eca5ae4d18d": {
				"ticker": "RShare",
				"address": "0x20F1830A62fcB67551981174dF922Eca5ae4D18D",
				"name": "RShare",
				"decimals": 18
			},
			"0x5eb949e58c620b95dbb26be9d8f58be5142daf2e": {
				"ticker": "APEN",
				"address": "0x5eb949E58c620B95DBB26Be9D8f58be5142dAF2e",
				"name": "APENodes.Finance",
				"decimals": 18
			},
			"0x7d1232b90d3f809a54eeaeebc639c62df8a8942f": {
				"ticker": "SB",
				"address": "0x7d1232B90D3F809A54eeaeeBC639C62dF8a8942f",
				"name": "Snowbank",
				"decimals": 9
			},
			"0x5853c51ed3bd8818f234a0783a82fde7d65a34bd": {
				"ticker": "GSHARE",
				"address": "0x5853C51ED3Bd8818F234A0783A82fDe7D65A34bd",
				"name": "GSHARE",
				"decimals": 18
			},
			"0xc6faea21aaa1b3082eac4bf7760e4e83b877c804": {
				"ticker": "SIMIAN",
				"address": "0xc6Faea21aaa1B3082eAC4Bf7760E4e83b877C804",
				"name": "Simian Nodes",
				"decimals": 18
			},
			"0x7e91ee5b9bd11050a190c170ec69c9dc42a0d64b": {
				"ticker": "BHCI",
				"address": "0x7E91ee5B9bd11050a190C170EC69c9Dc42a0D64B",
				"name": "Baby Hachi",
				"decimals": 9
			},
			"0x23f07a1c03e7c6d0c88e0e05e79b6e3511073fd5": {
				"ticker": "CDS",
				"address": "0x23f07a1C03e7C6D0C88e0E05E79B6E3511073fD5",
				"name": "Crypto Development Services",
				"decimals": 8
			},
			"0x2201f53a9441cb8d6705cc8f4524f2f85a55aac5": {
				"ticker": "HSHARE",
				"address": "0x2201f53a9441cB8d6705cC8F4524f2f85a55aAc5",
				"name": "HSHARE",
				"decimals": 18
			},
			"0x12f7d7b768e4086d8ebe1c8c3ff6bb81b0915951": {
				"ticker": "FORK",
				"address": "0x12f7d7B768e4086d8EbE1C8c3fF6bb81B0915951",
				"name": "FORKGANG",
				"decimals": 18
			},
			"0x74f407b24e01ceb69b74e771bd909950eaf0833e": {
				"ticker": "DOGE",
				"address": "0x74F407b24e01CeB69b74e771bD909950EAF0833e",
				"name": "Doge Monster",
				"decimals": 12
			},
			"0x665865c14d153793e0916bc824ef05343cedb591": {
				"ticker": "BANANA",
				"address": "0x665865C14d153793e0916bC824Ef05343CEdb591",
				"name": "Gorilla Nodes",
				"decimals": 18
			},
			"0x557d656a2f8456307c74c6d2237966ccb239477d": {
				"ticker": "RAINBOW",
				"address": "0x557d656A2f8456307C74C6D2237966CCb239477D",
				"name": "Rainbow",
				"decimals": 18
			},
			"0x92515901ceb031d771f4f5487f36c01fb978dba9": {
				"ticker": "CRAFT",
				"address": "0x92515901cEb031D771F4F5487f36C01FB978dBA9",
				"name": "CRAFT",
				"decimals": 18
			},
			"0x75e80319b1d9285eeeb7ebe82121cf8f8fde5a28": {
				"ticker": "SUPREME",
				"address": "0x75e80319B1D9285eEEB7ebE82121cf8f8fDe5a28",
				"name": "Supreme Token",
				"decimals": 18
			},
			"0x9366d30feba284e62900f6295bc28c9906f33172": {
				"ticker": "BioFi",
				"address": "0x9366d30FeBA284E62900f6295BC28c9906f33172",
				"name": "BioFi",
				"decimals": 6
			},
			"0x5e3e643a0e6ce7d1fab39c5ea41a27514d27f363": {
				"ticker": "MLN",
				"address": "0x5E3e643a0E6cE7d1fAB39C5eA41A27514d27F363",
				"name": "MerlinFinance",
				"decimals": 6
			},
			"0x25b3cf25874b9842fa349794be23b3e153ca7654": {
				"ticker": "ASTRO",
				"address": "0x25b3CF25874B9842Fa349794Be23b3e153Ca7654",
				"name": "100 Days Ventures",
				"decimals": 18
			},
			"0x92a4f622054fce00c16ea69fb4a9b1c795465dc3": {
				"ticker": "MALAMUTE",
				"address": "0x92A4F622054FCe00C16Ea69Fb4a9b1c795465dc3",
				"name": "Malamute Inu",
				"decimals": 18
			},
			"0x4b7ab9db5ffb73ca4887d159e5e89f8bad05be7c": {
				"ticker": "xPay",
				"address": "0x4B7ab9Db5fFB73Ca4887d159e5E89f8baD05be7C",
				"name": "AVAX Payment Processor",
				"decimals": 9
			},
			"0x92191f34dde7b46e657edec4bc0f33622c6424ee": {
				"ticker": "EXC",
				"address": "0x92191F34Dde7b46e657EDEC4BC0f33622C6424ee",
				"name": "Pendragon Protocol Excalibur Token",
				"decimals": 18
			},
			"0x4571e17a39608514d0dda347e5f423aec2761c86": {
				"ticker": "PLUTUS",
				"address": "0x4571E17a39608514D0DDa347e5f423aec2761c86",
				"name": "Plutus Node",
				"decimals": 18
			},
			"0x8c1374c85e2e21a5b8970ad0b50602fd03c01c1f": {
				"ticker": "SPRT",
				"address": "0x8C1374c85e2e21A5B8970aD0B50602Fd03c01C1F",
				"name": "Spiritdao",
				"decimals": 6
			},
			"0x8baace2d176036cd767a6cc005ab202224e1ce04": {
				"ticker": "PONZI",
				"address": "0x8baace2D176036cd767A6CC005ab202224e1CE04",
				"name": "PonziNodes",
				"decimals": 18
			},
			"0xd06bc91bc6de5b23baf22fada2a302d07f1c1f77": {
				"ticker": "MAG",
				"address": "0xD06BC91Bc6De5B23bAF22FadA2a302D07f1c1F77",
				"name": "Magnet Dao",
				"decimals": 18
			},
			"0x3e66b5a8313f13d04b580ae9325c9da79dacdd92": {
				"ticker": "SMRS",
				"address": "0x3e66B5A8313f13D04B580aE9325C9Da79DaCDD92",
				"name": "Safe Mars",
				"decimals": 11
			},
			"0xf955725a1497d18b4b1a66f4d764eb7afe54559a": {
				"ticker": "ICE",
				"address": "0xf955725A1497d18B4B1A66F4d764EB7AfE54559a",
				"name": "ICE",
				"decimals": 18
			},
			"0x57617c8c8e0a65775e984f365b764ec4bbbe255e": {
				"ticker": "GCH",
				"address": "0x57617c8c8e0a65775E984f365b764Ec4Bbbe255E",
				"name": "Glitch Inu",
				"decimals": 9
			},
			"0x2a901d91e7a852bb4d2c3ab75e2c90f050f0744b": {
				"ticker": "ELNB",
				"address": "0x2a901D91e7A852Bb4d2c3ab75e2C90F050F0744b",
				"name": "Elon Beast",
				"decimals": 12
			},
			"0x6f6c7d83c96096606d984c88a766590e386d8738": {
				"ticker": "StarNode",
				"address": "0x6F6c7d83C96096606D984c88a766590E386D8738",
				"name": "Star Node",
				"decimals": 18
			},
			"0x30b9dcd146e6f10c42817ff69e674f5675cc24c9": {
				"ticker": "GST",
				"address": "0x30b9DCd146E6F10C42817Ff69E674f5675CC24C9",
				"name": "GreenSatoshiToken",
				"decimals": 18
			},
			"0x26481371ab73238f866d024d59f7f5bb0848d040": {
				"ticker": "MARS",
				"address": "0x26481371ab73238f866D024d59F7F5BB0848D040",
				"name": "Mini Mars",
				"decimals": 12
			},
			"0x78eaba26fadffa42b264633ad75f8ec867b6ace4": {
				"ticker": "tost",
				"address": "0x78Eaba26fAdFfA42b264633aD75f8Ec867b6AcE4",
				"name": "Noumar",
				"decimals": 18
			},
			"0x7bf54666b20430d82255a2122d7fee5710e610bf": {
				"ticker": "SPACEX",
				"address": "0x7Bf54666b20430D82255a2122d7fEE5710e610bf",
				"name": "SpaceX",
				"decimals": 9
			},
			"0xefcac2829ba230f20328582966d378745c583580": {
				"ticker": "IAPE",
				"address": "0xEFCaC2829BA230f20328582966D378745C583580",
				"name": "Island Apes",
				"decimals": 9
			},
			"0xc8ffefeb25f16b6ae5bbacf7c564cd9866e8f92a": {
				"ticker": "MVA",
				"address": "0xC8ffEfeB25F16B6Ae5BBaCF7C564Cd9866E8F92A",
				"name": "MinervaDAO",
				"decimals": 18
			},
			"0x0342993a0a3bcff7668578082c3ad17b5277d97d": {
				"ticker": "HLTH",
				"address": "0x0342993A0A3Bcff7668578082c3Ad17b5277D97D",
				"name": "HLTH Token",
				"decimals": 18
			},
			"0xfc3a200a526c84a86b8aed290f8123f40dd290c0": {
				"ticker": "DLQ",
				"address": "0xfC3a200A526C84a86b8aED290F8123F40DD290C0",
				"name": "Deliq",
				"decimals": 18
			},
			"0x4ea285e729181b672c8ba0ba13949d1e586f8967": {
				"ticker": "CRG",
				"address": "0x4EA285e729181B672C8Ba0bA13949d1E586f8967",
				"name": "Treasure Corgi",
				"decimals": 11
			},
			"0xf7ed17f0fb2b7c9d3ddbc9f0679b2e1098993e81": {
				"ticker": "MUG",
				"address": "0xF7ed17f0Fb2B7C9D3DDBc9F0679b2e1098993e81",
				"name": "Mu Gold",
				"decimals": 18
			},
			"0xc118d77baf86a93ec41d867675c48c98b19953fd": {
				"ticker": "Pollen",
				"address": "0xc118D77bAf86a93EC41d867675c48C98B19953Fd",
				"name": "Pollen",
				"decimals": 18
			},
			"0x7b6cd1aee60b956e6593ce404555ea5d535e14ee": {
				"ticker": "APLB",
				"address": "0x7b6Cd1aee60B956e6593CE404555EA5D535E14eE",
				"name": "Apple Block",
				"decimals": 11
			},
			"0x240dd0252a81bb9100370861f95f9d79a394b7ae": {
				"ticker": "POOLZ",
				"address": "0x240DD0252a81BB9100370861f95f9d79a394b7AE",
				"name": "Poolz.Finance",
				"decimals": 9
			},
			"0x2f0d1606776efbd1a0d7acbade76b24a4c7c7376": {
				"ticker": "SHB",
				"address": "0x2F0d1606776EfBD1a0d7acbADE76b24a4C7C7376",
				"name": "Shiba Network",
				"decimals": 11
			},
			"0x3bb9ab23483bdb9f78979d720cced54235df104c": {
				"ticker": "ELN",
				"address": "0x3bb9aB23483BDb9f78979d720CCED54235df104c",
				"name": "Elon Swap",
				"decimals": 12
			},
			"0xaffe7586639d3fdf9fec94101485d142f00e8421": {
				"ticker": "APE",
				"address": "0xAfFe7586639d3fdf9fEc94101485d142F00E8421",
				"name": "Ape AVAX",
				"decimals": 10
			},
			"0x60781c2586d68229fde47564546784ab3faca982": {
				"ticker": "PNG",
				"address": "0x60781C2586D68229fde47564546784ab3fACA982",
				"name": "Pangolin",
				"decimals": 18
			},
			"0x312ee43df66d1fd1ea28e5b28f355da84dca13c2": {
				"ticker": "SWAPXI",
				"address": "0x312eE43Df66d1Fd1EA28e5b28F355Da84dCA13C2",
				"name": "SwapXI Token",
				"decimals": 12
			},
			"0xa1ca41dbfdb4c98f27b8dfd8f09b1a8fbc353a43": {
				"ticker": "WAR",
				"address": "0xa1CA41DBfdb4c98f27B8dfd8F09B1A8fBC353a43",
				"name": "War Nodes",
				"decimals": 9
			},
			"0x45dbb2b935ed1e7025826287ff102be8f265f7d0": {
				"ticker": "KANDY",
				"address": "0x45dBB2B935eD1E7025826287FF102BE8F265f7D0",
				"name": "KandylandDAO",
				"decimals": 18
			},
			"0x860b313267ddc879c53f2e080e5b661a8f2584b6": {
				"ticker": "ANGX",
				"address": "0x860b313267DdC879c53F2e080E5b661a8f2584b6",
				"name": "Angel X",
				"decimals": 9
			},
			"0xb648fa7a5f5ed3b3c743140346e3dc3fe94a9125": {
				"ticker": "AFINS",
				"address": "0xB648fA7A5f5ED3b3c743140346E3Dc3Fe94a9125",
				"name": "altFINS",
				"decimals": 18
			},
			"0xab5b7ce88372a55eece8d0802867a354275dc7b8": {
				"ticker": "bloq",
				"address": "0xAB5b7ce88372A55EEcE8D0802867A354275dC7b8",
				"name": "bloq.com",
				"decimals": 9
			},
			"0x2ac4ccbc98a6680bbd9dd0a2eafbb13becf09644": {
				"ticker": "sCROWN",
				"address": "0x2Ac4CCBc98A6680Bbd9Dd0A2eAFBB13BECf09644",
				"name": "Staked MidasDAO",
				"decimals": 9
			},
			"0x994a34d7fb34d8b5f90c317d5e68b34f39520128": {
				"ticker": "AKA",
				"address": "0x994A34d7fB34D8B5F90C317d5e68B34f39520128",
				"name": "Zen Akita",
				"decimals": 10
			},
			"0x28d42719510eeebb55be18dd32a276b5f66cbd9f": {
				"ticker": "MARS",
				"address": "0x28D42719510eEEBB55Be18Dd32a276B5F66CbD9f",
				"name": "Treasure Mars",
				"decimals": 11
			},
			"0x75117c32ad3f8234a2f13cc5afd262fc791263ee": {
				"ticker": "WAR",
				"address": "0x75117C32aD3f8234a2F13cC5afd262fC791263ee",
				"name": "War Games",
				"decimals": 9
			},
			"0x1dc473fcf830b2315d0350b355d13b6011b3a4c3": {
				"ticker": "SIMIAN",
				"address": "0x1dC473fCf830b2315D0350B355D13B6011B3a4C3",
				"name": "Simian Nodes",
				"decimals": 18
			},
			"0xf6c5662f2fa152f733c75db9f43cd77eb649a3f3": {
				"ticker": "PSI",
				"address": "0xF6C5662F2Fa152f733c75db9F43cd77eb649a3F3",
				"name": "PSINODES",
				"decimals": 18
			},
			"0x66821342eb56be36b19e7c3f5776ebd316327593": {
				"ticker": "Rain",
				"address": "0x66821342Eb56be36b19e7C3F5776eBd316327593",
				"name": "Rain DAO",
				"decimals": 18
			},
			"0x8d41fd04fff0b87563d7631d1a0614b2acebafad": {
				"ticker": "RAINBOW",
				"address": "0x8d41fd04Fff0B87563d7631d1A0614b2ACeBaFAd",
				"name": "Unicorn Nodes",
				"decimals": 18
			},
			"0x5e9c5b4ceb8a149b4fe6c8e5dd797ae0ff663959": {
				"ticker": "ELN",
				"address": "0x5E9C5b4ceb8a149b4fe6c8E5Dd797Ae0ff663959",
				"name": "Elon Starter",
				"decimals": 12
			},
			"0xd4000aac6de096b9d9379df862aa99af2013333f": {
				"ticker": "REVO",
				"address": "0xD4000aaC6De096b9d9379DF862AA99af2013333f",
				"name": "Revomon",
				"decimals": 18
			},
			"0x2c6194f046d89eab731b2d94e8223d38c0b20c56": {
				"ticker": "STRI",
				"address": "0x2C6194F046D89eAB731b2D94E8223D38c0B20c56",
				"name": "Star Infinity",
				"decimals": 9
			},
			"0x72c5c7190f9186cd9d2c7ba9f201711e15ac6d94": {
				"ticker": "Agate",
				"address": "0x72C5c7190f9186cD9D2c7ba9f201711e15aC6D94",
				"name": "Agate Nodes",
				"decimals": 18
			},
			"0xc8ad16df0708811064b055fd491fb3eabe1e9c22": {
				"ticker": "BTL",
				"address": "0xc8ad16DF0708811064B055fD491fb3eAbe1E9c22",
				"name": "Battle Nodes",
				"decimals": 9
			},
			"0x537cbb595a94a2703d997d3071abaac12dd7884f": {
				"ticker": "MARS",
				"address": "0x537cBb595a94A2703d997D3071aBAAc12Dd7884f",
				"name": "Mars Infinity",
				"decimals": 10
			},
			"0x060556209e507d30f2167a101bfc6d256ed2f3e1": {
				"ticker": "xPTP",
				"address": "0x060556209E507d30f2167a101bFC6D256Ed2f3e1",
				"name": "xPTP",
				"decimals": 18
			},
			"0x9a1f1a351e2ad46f4e01588108fde7981dc75e1e": {
				"ticker": "RBX",
				"address": "0x9a1f1a351e2Ad46f4e01588108fde7981Dc75e1E",
				"name": "RBX",
				"decimals": 18
			},
			"0x9d0109eaa18d6751e5e2e81b0b9e28028857e081": {
				"ticker": "SMN",
				"address": "0x9d0109eAA18d6751E5E2E81B0b9E28028857e081",
				"name": "SmartNodes",
				"decimals": 18
			},
			"0x4cac16ceadc1a2c5200d8e6db79da8e36d941e2c": {
				"ticker": "ELON",
				"address": "0x4CAc16CEADc1a2c5200D8E6Db79DA8e36D941e2C",
				"name": "OMG Elon",
				"decimals": 12
			},
			"0x8cefbe0fe81839284285cd3609b3a8c0f64ebad8": {
				"ticker": "MARS",
				"address": "0x8cEFBE0fE81839284285Cd3609b3A8C0F64ebad8",
				"name": "Mars BTC",
				"decimals": 11
			},
			"0x2e99b86306afceed40c4a2883afa09f83309e8e2": {
				"ticker": "SF",
				"address": "0x2e99B86306AfCEED40c4a2883AfA09f83309e8e2",
				"name": "Snow Fake",
				"decimals": 18
			},
			"0x9fe5df10021c08c3b7256f96e58d03d665b45c6c": {
				"ticker": "GOL",
				"address": "0x9fe5dF10021C08c3B7256F96E58D03D665b45C6c",
				"name": "Gold Block",
				"decimals": 9
			},
			"0x39dfd54ecf83c9bb2586faaf7d0ffc2864935c79": {
				"ticker": "ELON",
				"address": "0x39DfD54ECF83c9bB2586faaF7d0fFC2864935c79",
				"name": "Zen Elon",
				"decimals": 10
			},
			"0x7bb4f4e80abcaa1179b2d4c84f25abd9ba506baf": {
				"ticker": "WINGS",
				"address": "0x7Bb4F4e80abCAA1179b2D4c84f25ABd9BA506bAF",
				"name": "Wrapped Angel",
				"decimals": 12
			},
			"0x1491e702b02b246a7de1bc49f7380100ba34ca72": {
				"ticker": "APED",
				"address": "0x1491e702B02b246A7de1bC49F7380100BA34cA72",
				"name": "Ape DAO",
				"decimals": 9
			},
			"0xabddf128c6f79d3c1c8a6ea5d70a7a9005e771d1": {
				"ticker": "MVA",
				"address": "0xABDDF128C6f79D3c1C8A6Ea5d70A7a9005e771d1",
				"name": "MinervaDAO",
				"decimals": 18
			},
			"0xa358d5eac1f869296fb6626b59e6f6e0b5430d08": {
				"ticker": "MARS",
				"address": "0xa358D5eaC1F869296FB6626b59e6f6E0b5430d08",
				"name": "Mars Block",
				"decimals": 9
			},
			"0xee06825e18bd400b476dd0f82309f41e198042b4": {
				"ticker": "APEX",
				"address": "0xeE06825e18BD400B476Dd0F82309F41E198042B4",
				"name": "Apex Nodes",
				"decimals": 18
			},
			"0x337d3a7ab61422118aa21e9a0dde4b9ec3190ce9": {
				"ticker": "MOON",
				"address": "0x337D3A7ab61422118Aa21E9A0ddE4b9eC3190cE9",
				"name": "Moon Markets",
				"decimals": 11
			},
			"0x6460a2a3c9d451d0c198b7668343d80ed58522e6": {
				"ticker": "OHMP",
				"address": "0x6460A2a3c9d451D0C198B7668343d80ed58522E6",
				"name": "Olymplus",
				"decimals": 9
			},
			"0xed0adc760b311affd4b6d13defc18303ce2fddb8": {
				"ticker": "SNOB",
				"address": "0xeD0Adc760b311aFfd4b6d13DEfc18303cE2FDDB8",
				"name": "Snowball DAO",
				"decimals": 9
			},
			"0x62edc0692bd897d2295872a9ffcac5425011c661": {
				"ticker": "GMX",
				"address": "0x62edc0692BD897D2295872a9FFCac5425011c661",
				"name": "GMX",
				"decimals": 18
			},
			"0x7d37c2077abff1973952d6cd8880016efa3e4c45": {
				"ticker": "STK",
				"address": "0x7d37C2077AbFf1973952d6cd8880016EFa3E4C45",
				"name": "Servertech",
				"decimals": 18
			},
			"0x8430146cfd6f29c2b580c1004787b7d3c9f9f3b8": {
				"ticker": "VPND",
				"address": "0x8430146cFd6F29c2B580c1004787b7d3c9F9F3b8",
				"name": "VaporNodes",
				"decimals": 18
			},
			"0x392727ab885900f4f107384ad9b8f4494abcc65d": {
				"ticker": "TAPL",
				"address": "0x392727AB885900f4f107384AD9b8f4494ABCC65D",
				"name": "Treasure Apple",
				"decimals": 10
			},
			"0xb1ae4455951a26515685ac1ede64ee11ec715066": {
				"ticker": "SGEM",
				"address": "0xB1ae4455951A26515685aC1Ede64eE11ec715066",
				"name": "SGEM",
				"decimals": 18
			},
			"0xc0503f6eb1922c594a445f1952f9f719b40ff69f": {
				"ticker": "EM",
				"address": "0xc0503f6Eb1922c594a445F1952F9f719B40fF69f",
				"name": "EMERALD",
				"decimals": 18
			},
			"0x510c4d8ecb31d2f7f258728b65bb388023b2001c": {
				"ticker": "FOXSHARE",
				"address": "0x510c4d8ECB31d2f7f258728B65bB388023b2001c",
				"name": "FOXSHARES Token",
				"decimals": 18
			},
			"0x4221a979705edce8373b3fa18a405ee067022479": {
				"ticker": "WPAN",
				"address": "0x4221a979705edCE8373B3fa18a405eE067022479",
				"name": "Wrapped Panda",
				"decimals": 12
			},
			"0x246a08a58549e14d0fcb24d7e9227661ede807a8": {
				"ticker": "AVANET",
				"address": "0x246A08a58549E14D0Fcb24D7E9227661EdE807A8",
				"name": "AvangerNews",
				"decimals": 18
			},
			"0x6423d9067e835113b43edd5b9f038303b01b8e9b": {
				"ticker": "Plutu",
				"address": "0x6423d9067e835113B43eDD5B9F038303b01b8e9b",
				"name": "PLUTUS",
				"decimals": 9
			},
			"0x3061a551dbf7d621ae21070b6c52e19a33533d58": {
				"ticker": "BERRY",
				"address": "0x3061a551DBf7D621AE21070B6C52e19a33533d58",
				"name": "BerryFund",
				"decimals": 18
			},
			"0xd4f559abf5d8be067c404a04a17034867853a1b7": {
				"ticker": "AKA",
				"address": "0xd4F559aBf5D8bE067c404a04a17034867853A1b7",
				"name": "Akita Network",
				"decimals": 12
			},
			"0xa43d937fbc112f659b1491368d75b54682446189": {
				"ticker": "WINGS",
				"address": "0xa43d937fBC112F659b1491368d75b54682446189",
				"name": "Angel AI",
				"decimals": 9
			},
			"0x236061c4018e28988840b2c6ff56451e40f6e3bc": {
				"ticker": "ULTA",
				"address": "0x236061C4018e28988840B2c6fF56451e40f6e3bc",
				"name": "ULTA",
				"decimals": 9
			},
			"0x76c6a5b6062ce17618d6cd0ac1088a1aef5e8c7b": {
				"ticker": "CYBER",
				"address": "0x76c6a5B6062CE17618d6Cd0aC1088a1AEF5E8C7B",
				"name": "Cyber",
				"decimals": 9
			},
			"0x3b7fdb11b1d2433415659dc370f3b8d3d644bc5a": {
				"ticker": "VTX",
				"address": "0x3B7fDb11B1d2433415659Dc370f3B8d3d644bC5A",
				"name": "Vector Finance",
				"decimals": 6
			},
			"0x4c5cc4eb816fc341b0f8caec17ebe6a05fdfbb0b": {
				"ticker": "CHI",
				"address": "0x4C5cC4eb816FC341B0f8CAeC17EbE6a05FdfBb0b",
				"name": "Chimp Chain",
				"decimals": 12
			},
			"0x6a45ef641e73f141f4816b64977395044800f224": {
				"ticker": "TANK",
				"address": "0x6a45EF641E73F141F4816B64977395044800F224",
				"name": "Tank Shares",
				"decimals": 18
			},
			"0x812c96218e50ada0bececedad149051766c2958a": {
				"ticker": "PKT",
				"address": "0x812c96218E50aDA0BECecEDaD149051766C2958a",
				"name": "PKT",
				"decimals": 18
			},
			"0x5085434227ab73151fad2de546210cbc8663df96": {
				"ticker": "DBY",
				"address": "0x5085434227aB73151fAd2DE546210Cbc8663dF96",
				"name": "Metaderby token",
				"decimals": 18
			},
			"0xc8600b3fbf406a44f97e72367dfc2f5bb63fbe7b": {
				"ticker": "MSHARE",
				"address": "0xc8600b3FBF406a44f97E72367dFC2f5bb63FBE7b",
				"name": "Magic Money Shares Token",
				"decimals": 18
			},
			"0x7cdb5da45854f88c5ca93861a3b4ac10fbbee5f2": {
				"ticker": "GHT",
				"address": "0x7CdB5DA45854f88c5Ca93861A3b4aC10fBbEe5f2",
				"name": "Super Ghost",
				"decimals": 10
			},
			"0x8ebd36afa15dd29f4bc9be49843ee7903146c18d": {
				"ticker": "CMTD",
				"address": "0x8Ebd36AfA15dD29F4bc9be49843ee7903146C18D",
				"name": "Chromatid Token",
				"decimals": 18
			},
			"0xae8e6a1e28cd357af6f69473638bb5d3200511b2": {
				"ticker": "APEX",
				"address": "0xaE8E6a1E28Cd357AF6f69473638bB5D3200511b2",
				"name": "Ape X",
				"decimals": 11
			},
			"0xe6d1afea0b76c8f51024683dd27fa446ddaf34b6": {
				"ticker": "WSHARE",
				"address": "0xe6d1aFea0B76C8f51024683DD27FA446dDAF34B6",
				"name": "Frozen Walrus Share",
				"decimals": 18
			},
			"0xf75a8f03c3c754b2539bce2872f09b040fc06173": {
				"ticker": "LAVA",
				"address": "0xF75A8F03C3c754B2539bCe2872f09B040fC06173",
				"name": "Lava Financial",
				"decimals": 18
			},
			"0xb4473ec5ac2909a90cfec869d741e7080670d71c": {
				"ticker": "SBAS",
				"address": "0xb4473eC5ac2909a90cFeC869D741E7080670d71c",
				"name": "Shiba Swap",
				"decimals": 10
			},
			"0x83f4299252fb8678444f59d6728a01de70063187": {
				"ticker": "GLTCH",
				"address": "0x83f4299252fb8678444f59D6728A01De70063187",
				"name": "Glitch AVA",
				"decimals": 9
			},
			"0x77777777777d4554c39223c354a05825b2e8faa3": {
				"ticker": "YETI",
				"address": "0x77777777777d4554c39223C354A05825b2E8Faa3",
				"name": "Yeti Finance",
				"decimals": 18
			},
			"0x07f95b4f8067a234245ca5b8d5794b9d65be620e": {
				"ticker": "SECRET",
				"address": "0x07f95B4f8067A234245Ca5B8d5794b9d65bE620E",
				"name": "Secret Protocol",
				"decimals": 9
			},
			"0xbb0721596fb3528c3ec08a2ab1f5c1e00503a8f6": {
				"ticker": "FLK",
				"address": "0xBb0721596FB3528C3Ec08a2AB1F5C1E00503a8f6",
				"name": "Mini Floki",
				"decimals": 12
			},
			"0x51b38bca0e284da70979c170eeaeb75d87d6a3ce": {
				"ticker": "SS",
				"address": "0x51b38bCA0e284da70979c170eEAEb75D87d6a3Ce",
				"name": "SnowSafe",
				"decimals": 18
			},
			"0x2a94e82a742d9d5a7259e7319d65fd3df924155d": {
				"ticker": "SUP",
				"address": "0x2A94e82A742D9d5a7259E7319d65fD3DF924155d",
				"name": "SUPREME NODES",
				"decimals": 9
			},
			"0xaee7458d9f547cd1db97505e9b34732839399650": {
				"ticker": "MOON",
				"address": "0xaEe7458d9F547CD1DB97505E9b34732839399650",
				"name": "Moon Monster",
				"decimals": 12
			},
			"0x16cccff79acb6c54c1c6973b382d00c038e0e331": {
				"ticker": "EMERALD",
				"address": "0x16ccCFF79aCB6c54c1C6973B382d00c038e0e331",
				"name": "Emerald Token",
				"decimals": 18
			},
			"0xc715395f96fe01b267a9657b44c0e099f305fe3d": {
				"ticker": "FTM",
				"address": "0xc715395f96FE01B267a9657b44C0e099f305fe3d",
				"name": "Fantom INC",
				"decimals": 12
			},
			"0x1a4994c0e2323e309062d800f7f23a3f30774184": {
				"ticker": "STLX",
				"address": "0x1a4994C0e2323e309062D800f7F23a3F30774184",
				"name": "StealthEX.io",
				"decimals": 9
			},
			"0x3cc4cbf490a8ac5dd4c9ea8fc57e29407d56a7ce": {
				"ticker": "FINGER",
				"address": "0x3CC4cbF490a8Ac5Dd4c9eA8fC57e29407d56a7CE",
				"name": "FingerprintsDAO",
				"decimals": 18
			},
			"0xdc684e04b7bf659e6485eb263df4fe5698af3ea9": {
				"ticker": "COME",
				"address": "0xdc684e04b7Bf659E6485EB263Df4fE5698Af3EA9",
				"name": "Comet",
				"decimals": 18
			},
			"0x8550fbe573a4efeae3d79a5cf97ba214213eba77": {
				"ticker": "ZDOG",
				"address": "0x8550fbE573a4efeAe3D79A5cF97ba214213EBA77",
				"name": "Zen Doge",
				"decimals": 10
			},
			"0xd143684698f933e3561a6e03200826e479ebb633": {
				"ticker": "FLOKI",
				"address": "0xd143684698F933e3561A6E03200826E479Ebb633",
				"name": "Floki Infinity",
				"decimals": 11
			},
			"0xf92a3a93adac0c010c896c6735a64653e1fd3615": {
				"ticker": "CORGI",
				"address": "0xF92A3A93AdAC0C010C896c6735A64653e1FD3615",
				"name": "King Corgi",
				"decimals": 10
			},
			"0xbafe4e1edd39f1a3b6364f84c38a2895a21cebf3": {
				"ticker": "TAIWAN",
				"address": "0xbafE4E1Edd39F1a3b6364f84C38a2895a21CebF3",
				"name": "TaiwanToken",
				"decimals": 18
			},
			"0x822b906e74d493d07223cf6858620ccda66b2154": {
				"ticker": "rloop",
				"address": "0x822b906E74D493D07223CF6858620ccDa66b2154",
				"name": "rLoop",
				"decimals": 18
			},
			"0xb1d7896a5d2fccae347c1f0cd6b33f8919b5c852": {
				"ticker": "ZSHARE",
				"address": "0xb1D7896a5D2fCCaE347C1f0Cd6b33f8919B5c852",
				"name": "Zilla Shares",
				"decimals": 18
			},
			"0xec59f418cb7122d56df28eb1627ad217c43416ba": {
				"ticker": "STAR",
				"address": "0xec59F418CB7122D56df28EB1627AD217C43416BA",
				"name": "Star AI",
				"decimals": 11
			},
			"0xffd1947879dc7870bf37500cd6a822827350c1dd": {
				"ticker": "GHS",
				"address": "0xffD1947879dc7870BF37500Cd6A822827350c1dd",
				"name": "King Ghost",
				"decimals": 10
			},
			"0xfd450043d7ed935ac897482fb30e9d65fe87c473": {
				"ticker": "CGI",
				"address": "0xFD450043D7ED935Ac897482fb30E9d65Fe87c473",
				"name": "Corgi Infinity",
				"decimals": 10
			},
			"0xe4dd99b60324d960deaf77a57a0c7a0a2ac60bdc": {
				"ticker": "DEGEN",
				"address": "0xe4dD99B60324d960DeAF77a57a0C7A0a2ac60bDc",
				"name": "DegenFi",
				"decimals": 18
			},
			"0xad04d37d474c5ae694378d912bdef2c818633e5c": {
				"ticker": "MHUB",
				"address": "0xAd04d37D474C5Ae694378D912BdEF2C818633E5c",
				"name": "MetaHub",
				"decimals": 9
			},
			"0xf6f315e3c25b7c0fe9bb8c6bbfdc568e32109db6": {
				"ticker": "CHIMP",
				"address": "0xF6f315E3C25B7c0Fe9Bb8C6BBfdc568E32109db6",
				"name": "Chimp King",
				"decimals": 12
			},
			"0xba5dc7e77d150816b758e9826fcad2d74820e379": {
				"ticker": "WOWX",
				"address": "0xbA5dC7e77d150816b758E9826fCad2D74820e379",
				"name": "WowneroX",
				"decimals": 18
			},
			"0x906ad9d440bf6da6d606926a0678c2ccf3e8261d": {
				"ticker": "STAR",
				"address": "0x906aD9d440bf6Da6d606926A0678c2cCF3e8261d",
				"name": "Star Infinity",
				"decimals": 11
			},
			"0x1cf9d1053d8add7f20ffce7c5f67c5769444822e": {
				"ticker": "GHT",
				"address": "0x1Cf9D1053D8add7F20FFce7c5F67c5769444822e",
				"name": "Treasure Ghost",
				"decimals": 12
			},
			"0x49928d4bf4d07d05d08b79fb67ce92ee0b808d95": {
				"ticker": "MARS",
				"address": "0x49928d4bF4D07D05D08B79FB67CE92ee0b808D95",
				"name": "Mars Fund",
				"decimals": 9
			},
			"0xb6c08e6934d264d7c4dc3e34030d00df8e308148": {
				"ticker": "DUMB",
				"address": "0xB6c08e6934D264D7C4Dc3E34030D00dF8e308148",
				"name": "DumbCoin",
				"decimals": 18
			},
			"0x1b679fc7e0e6d1b80115df0d185da631f3a954cb": {
				"ticker": "GHST",
				"address": "0x1B679Fc7E0E6d1b80115dF0D185Da631f3A954CB",
				"name": "Zen Ghost",
				"decimals": 12
			},
			"0x026187bdbc6b751003517bcb30ac7817d5b766f8": {
				"ticker": "H2O",
				"address": "0x026187BdbC6b751003517bcb30Ac7817D5B766f8",
				"name": "Defrost Finance H2O",
				"decimals": 18
			},
			"0xc55036b5348cfb45a932481744645985010d3a44": {
				"ticker": "WINE",
				"address": "0xC55036B5348CfB45a932481744645985010d3A44",
				"name": "Wine Shares",
				"decimals": 18
			},
			"0x095a2c90e2aa40db16c9693faa4cb5e7b39faf9c": {
				"ticker": "ARMOR",
				"address": "0x095a2c90E2aA40dB16C9693faa4CB5e7B39fAf9c",
				"name": "Armor Protocol",
				"decimals": 9
			},
			"0x971d71be5fd86350075f1dc96d9e5a1184fe23df": {
				"ticker": "OWHL",
				"address": "0x971D71BE5Fd86350075f1dc96D9E5A1184fe23Df",
				"name": "OMG Whale",
				"decimals": 10
			},
			"0x1755d2332714b2ea908a9080f0ea69d5a3bc9580": {
				"ticker": "WHLR",
				"address": "0x1755D2332714B2Ea908A9080f0eA69D5a3bC9580",
				"name": "Whale Robot",
				"decimals": 9
			},
			"0x10e674a6644c05ed3abbeb986325c14bd22bcad9": {
				"ticker": "GHST",
				"address": "0x10e674a6644C05ed3aBbEB986325C14bD22BcAd9",
				"name": "Ghost Coin",
				"decimals": 12
			},
			"0xa157d7aeacf26937971b63b609aed0adc7b60ad0": {
				"ticker": "FLOKI",
				"address": "0xA157d7AeAcF26937971b63b609aed0ADC7B60ad0",
				"name": "Mini Floki",
				"decimals": 12
			},
			"0x008d42c239e5a0da1f70c6b5324cae2c7f277e7e": {
				"ticker": "SCMP",
				"address": "0x008D42c239e5A0Da1F70c6B5324cAe2C7f277E7e",
				"name": "Super Chimp",
				"decimals": 11
			},
			"0x654ccd745813aae42c5c71aa3ac3f6e665f4663a": {
				"ticker": "DEGEN",
				"address": "0x654ccD745813AaE42C5c71aA3aC3f6E665f4663A",
				"name": "DegenFi",
				"decimals": 18
			},
			"0xf7f9be9e718e3edb13c089c2818459641e80a7db": {
				"ticker": "DON",
				"address": "0xF7F9BE9e718e3edB13c089c2818459641e80a7db",
				"name": "Dogeon",
				"decimals": 18
			},
			"0x48ecc65cc969585440b261d6601a582e0210ba94": {
				"ticker": "rJOE",
				"address": "0x48eCc65CC969585440b261D6601a582E0210BA94",
				"name": "ROCKET JOE",
				"decimals": 8
			},
			"0x1157950c64f3bf1bf62a7840c148cf608e724ffc": {
				"ticker": "Hatter",
				"address": "0x1157950c64f3bF1bf62A7840C148cf608e724FFc",
				"name": "Hatter",
				"decimals": 18
			},
			"0xb7000052c7581187cd0c0c9392345f8adef057e6": {
				"ticker": "ASTRO",
				"address": "0xb7000052c7581187Cd0c0C9392345F8ADef057E6",
				"name": "100 Days",
				"decimals": 18
			},
			"0x3af6e2619140f356b04507b1a47e00091649244a": {
				"ticker": "EZE",
				"address": "0x3AF6e2619140f356B04507b1A47e00091649244a",
				"name": "EZE",
				"decimals": 18
			},
			"0xf425e3f8d3a9338cba472f2ea76a010c0bfc55c1": {
				"ticker": "AKA",
				"address": "0xF425e3f8D3a9338Cba472F2Ea76a010c0BfC55C1",
				"name": "Exo Akita",
				"decimals": 11
			},
			"0xdcf8c2f3026781b9b432b029df25b5079321f5ee": {
				"ticker": "VOLT",
				"address": "0xDCf8c2F3026781b9B432b029DF25b5079321F5EE",
				"name": "VOLT",
				"decimals": 5
			},
			"0x4476e30ff8708ef0f79e7bbf71a76b82d8646e59": {
				"ticker": "Test",
				"address": "0x4476e30Ff8708eF0F79E7Bbf71a76b82D8646E59",
				"name": "TestToken",
				"decimals": 18
			},
			"0x85437d1a6b53ea1bf65fedb02819ff86bcbbf319": {
				"ticker": "PNPL",
				"address": "0x85437D1a6b53Ea1BF65FedB02819FF86bCbbF319",
				"name": "Baby Pineapple",
				"decimals": 10
			},
			"0xf891214fdcf9cdaa5fdc42369ee4f27f226adad6": {
				"ticker": "IME",
				"address": "0xF891214fdcF9cDaa5fdC42369eE4F27F226AdaD6",
				"name": "Imperium Empires Token",
				"decimals": 18
			},
			"0xf536bed7b7cec459f04968ddc9fd33d5ccdac0d1": {
				"ticker": "ALEPH",
				"address": "0xF536BED7b7CeC459f04968dDC9fD33D5CCDaC0d1",
				"name": "Aleph Finance",
				"decimals": 18
			},
			"0xe6f43e43317b55bc00f17ff2943707a2c50f3286": {
				"ticker": "MARS",
				"address": "0xE6f43e43317b55bc00F17Ff2943707A2C50F3286",
				"name": "Exo Mars",
				"decimals": 12
			},
			"0x867960b1c37573b0ddfbb1af10c399e6e68d3b7d": {
				"ticker": "FKI",
				"address": "0x867960B1c37573B0ddFBB1aF10c399e6e68D3B7D",
				"name": "Floki NET",
				"decimals": 10
			},
			"0x7185c7750f0abde50e0abf3d4fb5f2edf23b1015": {
				"ticker": "SHB",
				"address": "0x7185C7750F0ABDE50E0abF3d4Fb5f2Edf23B1015",
				"name": "Queen Shiba",
				"decimals": 9
			},
			"0x5b501bd7f4c30de97e1666097074755cc826764c": {
				"ticker": "EASY",
				"address": "0x5B501bD7F4c30DE97E1666097074755CC826764c",
				"name": "EasyNodes",
				"decimals": 18
			},
			"0x58531deee173b01955b58303e37bcf1db5ac12ff": {
				"ticker": "APE",
				"address": "0x58531DeeE173B01955B58303E37BcF1dB5aC12Ff",
				"name": "Ape Fund",
				"decimals": 10
			},
			"0xdffc52fdbed4cd4f95d8cee88c7d8b2c6a5a66bd": {
				"ticker": "HAKU",
				"address": "0xDfFc52FdBeD4CD4f95d8CEe88c7d8b2c6A5A66Bd",
				"name": "HakuSwap Token",
				"decimals": 18
			},
			"0x43b59ccc39f45ab94b4a8fd6e6da5457f8cab9c7": {
				"ticker": "BATMAN",
				"address": "0x43B59CCc39f45AB94B4A8fD6e6da5457F8cab9C7",
				"name": "BATMAN",
				"decimals": 18
			},
			"0x46cd130eada79874757295ed724781a6685197dc": {
				"ticker": "SHBC",
				"address": "0x46CD130EADa79874757295ed724781A6685197DC",
				"name": "Shiba Coin",
				"decimals": 10
			},
			"0x532bb48581996c7f54c966ec6214fcbb85527ff6": {
				"ticker": "NEON",
				"address": "0x532bb48581996c7F54c966EC6214FCBb85527FF6",
				"name": "Nodeon",
				"decimals": 18
			},
			"0x38b00fdf5f57831ca168676a71d860d1f084c84c": {
				"ticker": "MSAA",
				"address": "0x38B00FDf5f57831Ca168676A71D860D1f084C84c",
				"name": "MetaSwap Avalanche",
				"decimals": 18
			},
			"0x7d66444e33ab4308fd9329f51cb4740fcbfb2df6": {
				"ticker": "WANG",
				"address": "0x7d66444E33Ab4308fd9329F51CB4740FcBFB2Df6",
				"name": "Wrapped Angel",
				"decimals": 11
			},
			"0x9a73de03c15a87ca2a225ad3d050db59a3f42d45": {
				"ticker": "TOM",
				"address": "0x9a73dE03c15A87cA2A225aD3d050dB59a3F42d45",
				"name": "Fantom Coin",
				"decimals": 11
			},
			"0xd7538cabbf8605bde1f4901b47b8d42c61de0367": {
				"ticker": "PGL",
				"address": "0xd7538cABBf8605BdE1f4901B47B8D42c61DE0367",
				"name": "Pangolin Liquidity",
				"decimals": 18
			},
			"0x49f7b688c65bc78489998ba287f8b7da07d6fd22": {
				"ticker": "CHI",
				"address": "0x49f7b688c65bc78489998bA287F8B7Da07d6Fd22",
				"name": "Chimp AVA",
				"decimals": 10
			},
			"0xe5fb232a63bbf3f2cd6e0b2037b94dfdd2a8e86f": {
				"ticker": "YETI",
				"address": "0xe5fb232a63bbf3f2cD6e0b2037B94dfDd2a8e86f",
				"name": "Yeti Finance",
				"decimals": 18
			},
			"0xfa137b8ee130fc1ddcf314fbd06dbd1f00da80c2": {
				"ticker": "ANG",
				"address": "0xFA137B8eE130FC1DDcF314fBd06DBD1f00Da80c2",
				"name": "Sushi Angel",
				"decimals": 11
			},
			"0xc3c761f8d9ce992431e49298e699aa19936d1bd4": {
				"ticker": "ASTRO",
				"address": "0xC3C761F8d9cE992431e49298e699aa19936D1BD4",
				"name": "100 Days",
				"decimals": 18
			},
			"0xba629bf6834845c8489648420ed306fd3fc67075": {
				"ticker": "CAKE",
				"address": "0xBa629bF6834845c8489648420ED306fD3fC67075",
				"name": "Cake Classic",
				"decimals": 12
			},
			"0x9977f40464e90776ad4903f0d82a74ebd1e6906a": {
				"ticker": "DS",
				"address": "0x9977f40464E90776Ad4903f0d82a74Ebd1e6906a",
				"name": "DAO SOCIETY",
				"decimals": 18
			},
			"0xa0894b162306f9ae045f7e67487a6d3114d04caa": {
				"ticker": "CFKI",
				"address": "0xa0894b162306F9ae045F7e67487A6D3114D04cAA",
				"name": "Crypto Floki",
				"decimals": 11
			},
			"0x525e06048d446963405f61d323210fc1d7d62325": {
				"ticker": "SAPL",
				"address": "0x525E06048D446963405f61D323210FC1D7d62325",
				"name": "Safe Apple",
				"decimals": 9
			},
			"0xba26bf4a4d94ce2849da96c28795a5bd45ea824d": {
				"ticker": "MOON",
				"address": "0xba26bf4A4d94cE2849Da96C28795a5Bd45Ea824d",
				"name": "Moon Ledger",
				"decimals": 12
			},
			"0x10e9f8b7c3d3be2fc1f0d5e293ab10d7463553d7": {
				"ticker": "WHALE",
				"address": "0x10E9F8b7C3d3bE2fc1F0d5E293Ab10d7463553d7",
				"name": "Exo Whale",
				"decimals": 12
			},
			"0x0901834d1baedc6fae12f8cfea6f2e23ce8b043d": {
				"ticker": "ZAPC",
				"address": "0x0901834d1bAeDC6fAE12f8CFEA6f2e23cE8b043D",
				"name": "Thunder Classic",
				"decimals": 9
			},
			"0xb2a85c5ecea99187a977ac34303b80acbddfa208": {
				"ticker": "ROCO",
				"address": "0xb2a85C5ECea99187A977aC34303b80AcbDdFa208",
				"name": "ROCO",
				"decimals": 18
			},
			"0xfa25560aeff7f3f53f1524a30a1c98714951cb5f": {
				"ticker": "nALEPH",
				"address": "0xfa25560AefF7F3f53f1524A30a1c98714951Cb5F",
				"name": "Aleph",
				"decimals": 18
			},
			"0xa42b568f1fcf6d18d670c8d761360577fbe9f5e2": {
				"ticker": "GShares",
				"address": "0xA42B568f1FcF6d18D670C8d761360577FBE9f5e2",
				"name": " Glamour Shares",
				"decimals": 18
			},
			"0xac55d9ba6335ffd04c7030c5d98996dfeb711756": {
				"ticker": "WINGS",
				"address": "0xAc55d9bA6335ffd04C7030c5D98996dfeb711756",
				"name": "Treasure Angel",
				"decimals": 11
			},
			"0xd24c2ad096400b6fbcd2ad8b24e7acbc21a1da64": {
				"ticker": "FRAX",
				"address": "0xD24C2Ad096400B6FBcd2ad8B24E7acBc21A1da64",
				"name": "Frax",
				"decimals": 18
			},
			"0x86aacf5bed5cebb4cf26d19f61296ce3e0c9df56": {
				"ticker": "CGI",
				"address": "0x86AAcf5BED5ceBB4CF26d19F61296Ce3E0c9DF56",
				"name": "Corgi Queen",
				"decimals": 12
			},
			"0x4683167217e2ba7c60a9235ace3707697ca0c20c": {
				"ticker": "PLYMTES",
				"address": "0x4683167217E2BA7C60A9235ACE3707697CA0C20C",
				"name": "Playmate's",
				"decimals": 18
			},
			"0x2f41c8335bd7da267cab94d87bb81e661b44cfbe": {
				"ticker": "STAR",
				"address": "0x2f41C8335BD7dA267CAb94d87Bb81E661b44cFbe",
				"name": "Star Starter",
				"decimals": 10
			},
			"0x04ac85b2666dce9a782e4e61b07b157ef1224923": {
				"ticker": "D1",
				"address": "0x04AC85b2666dcE9A782e4e61b07B157Ef1224923",
				"name": "D1Token",
				"decimals": 18
			},
			"0xb7d126c982a49276b50b3e8e9c1b620209acf0ab": {
				"ticker": "BITPIX",
				"address": "0xb7d126C982A49276b50b3E8E9c1b620209aCf0aB",
				"name": "BitPixels NFT Billboard",
				"decimals": 9
			},
			"0x84795947f0a85a42d5fb70f3b83d1b22a4f3f60b": {
				"ticker": "TOMD",
				"address": "0x84795947F0A85a42D5fB70f3B83d1b22a4f3f60b",
				"name": "Fantom DAO",
				"decimals": 11
			},
			"0xb7ce6ca78da82417ccf1975cb0960104e7e2e843": {
				"ticker": "FLOKI",
				"address": "0xb7cE6cA78dA82417CCF1975cB0960104E7E2e843",
				"name": "Floki Starter",
				"decimals": 11
			},
			"0x1f329abe0187b3857692fa289458fac892fbf7d3": {
				"ticker": "BASH",
				"address": "0x1F329AbE0187b3857692FA289458FaC892fBf7D3",
				"name": "Bash Nodes",
				"decimals": 9
			},
			"0x78a3285000cc981e7ec9fc8481b5598ed6a1d8f4": {
				"ticker": "SHB",
				"address": "0x78a3285000cc981e7Ec9fC8481b5598ED6a1d8f4",
				"name": "Shiba Chain",
				"decimals": 11
			},
			"0x2d190649f20fdf72676f007c2b3d299dbc44258a": {
				"ticker": "ZWHL",
				"address": "0x2d190649F20fDF72676f007c2B3d299Dbc44258A",
				"name": "Zen Whale",
				"decimals": 11
			},
			"0x26284a7477cb2862561de518ca0c945c993e0537": {
				"ticker": "ZAPK",
				"address": "0x26284A7477cB2862561dE518ca0C945C993E0537",
				"name": "Thunder King",
				"decimals": 9
			},
			"0xe96d3e01a650a8f18921c11bd917d3bf14eeb873": {
				"ticker": "CRG",
				"address": "0xe96D3e01A650A8F18921c11bD917D3bf14Eeb873",
				"name": "Corgi Infinity",
				"decimals": 10
			},
			"0xec3492a2508ddf4fdc0cd76f31f340b30d1793e6": {
				"ticker": "CLY",
				"address": "0xec3492a2508DDf4FDc0cD76F31f340b30d1793e6",
				"name": "Colony Token",
				"decimals": 18
			},
			"0xe3b594ad06da6fd89c03a0b2ad51ff2256b84eb6": {
				"ticker": "QAPL",
				"address": "0xe3b594Ad06da6Fd89c03A0b2ad51FF2256B84eb6",
				"name": "Queen Apple",
				"decimals": 12
			},
			"0x4ef491a54b7d16265574586849f2566ce27e48a2": {
				"ticker": "PDAB",
				"address": "0x4ef491A54B7D16265574586849f2566cE27E48a2",
				"name": "Panda Block",
				"decimals": 9
			},
			"0x32c064ae9ca34d9fc8ee09d40ea80c7bb5dbb11d": {
				"ticker": "APL",
				"address": "0x32C064aE9CA34d9Fc8Ee09d40eA80c7BB5dBB11d",
				"name": "Super Apple",
				"decimals": 12
			},
			"0x50bc34d76fac257484989142a8f24dc9cbc41db2": {
				"ticker": "KCMP",
				"address": "0x50BC34d76Fac257484989142a8F24dC9cbc41db2",
				"name": "King Chimp",
				"decimals": 11
			},
			"0x89b0f716b0829c735697516908e3bb200b9ade2d": {
				"ticker": "MRS",
				"address": "0x89b0f716B0829C735697516908e3Bb200B9aDE2d",
				"name": "Mars Network",
				"decimals": 9
			},
			"0x7dba74ea4e9f8e9c361dbd8008f8bf4e06161d0c": {
				"ticker": "LEMONADE",
				"address": "0x7dBa74EA4E9f8e9c361DbD8008f8BF4e06161D0c",
				"name": "LEMONADE SHARES",
				"decimals": 18
			},
			"0xfaf4e910fd48dc90b587d2ea31fce72e79a9d3db": {
				"ticker": "WHLG",
				"address": "0xFAF4E910FD48dc90b587D2ea31fcE72e79A9d3DB",
				"name": "Whale Gaming",
				"decimals": 10
			},
			"0x3d26b9f1fe7dfa279522f13850d322414424e61a": {
				"ticker": "GSHARE",
				"address": "0x3D26b9F1fe7dFA279522F13850D322414424e61A",
				"name": "GSHARE",
				"decimals": 18
			},
			"0x19ea54730fe381044aa23d523a7216086441b9cf": {
				"ticker": "FIRE",
				"address": "0x19Ea54730fE381044aa23d523a7216086441b9CF",
				"name": "Firebank",
				"decimals": 9
			},
			"0x1ed178576f6489c9ceaa2c198365090ceccbb54d": {
				"ticker": "RING",
				"address": "0x1ed178576f6489c9cEaa2C198365090cECcbB54d",
				"name": "RING",
				"decimals": 18
			},
			"0x2905d6d6957983d9ed73bc019ff2782c39dd7a49": {
				"ticker": "SNCT",
				"address": "0x2905d6D6957983d9ED73BC019fF2782c39dd7a49",
				"name": "Snake City Token",
				"decimals": 18
			},
			"0x0448ce764182bd668f1e7bb8ecadb8add667e1ae": {
				"ticker": "Spirit",
				"address": "0x0448Ce764182BD668F1E7BB8EcADb8Add667e1aE",
				"name": "Spiritdao",
				"decimals": 6
			},
			"0x70e2fb9b066dfc39aaeeb9d195cb35ad2c71c6a5": {
				"ticker": "SWHL",
				"address": "0x70E2fb9B066dFC39aaEEB9D195Cb35aD2c71c6A5",
				"name": "Safe Whale",
				"decimals": 10
			},
			"0x50d3929f9b68ad832e850231d3ecf9839873d742": {
				"ticker": "MRS",
				"address": "0x50D3929F9B68ad832e850231D3EcF9839873D742",
				"name": "Exo Mars",
				"decimals": 9
			},
			"0xc05d6e7efa25d0094adc147dd18c16a9bf939bcf": {
				"ticker": "GEM",
				"address": "0xc05d6E7EFA25D0094aDC147dD18C16A9Bf939bcf",
				"name": "Gem Finance",
				"decimals": 9
			},
			"0x5be69ae3bc056ca793d72041043d909a886cb574": {
				"ticker": "MOON",
				"address": "0x5be69AE3bc056Ca793d72041043D909a886cb574",
				"name": "Moon Swap",
				"decimals": 10
			},
			"0xf9c5c7b95329c19a7fc17c28fa7d5df56e8fd989": {
				"ticker": "CGI",
				"address": "0xf9C5c7B95329c19a7FC17c28fA7d5df56E8fd989",
				"name": "Corgi X",
				"decimals": 9
			},
			"0x539cb40d3670fe03dbe67857c4d8da307a70b305": {
				"ticker": "SNOW",
				"address": "0x539cB40D3670fE03Dbe67857C4d8da307a70B305",
				"name": "Snowflake",
				"decimals": 18
			},
			"0xe3685d6aa9ba3c1bbffb045982f43f0f54de04d3": {
				"ticker": "MOON",
				"address": "0xe3685d6Aa9bA3C1bbffb045982F43f0f54DE04d3",
				"name": "Crypto Moon",
				"decimals": 12
			},
			"0x9002522b2cdbe3ba393960a9d57cc39d6304f23c": {
				"ticker": "WINGS",
				"address": "0x9002522B2cDBe3bA393960A9D57cC39D6304f23C",
				"name": "Angel Diamond",
				"decimals": 12
			},
			"0x8360b5de152c616f95aa5278cee7211e99728804": {
				"ticker": "YUM",
				"address": "0x8360b5dE152C616f95AA5278cEe7211e99728804",
				"name": "Wrapped Cake",
				"decimals": 10
			},
			"0xf1550937207c770688cddb539c1f628b8012e6b3": {
				"ticker": "GHS",
				"address": "0xf1550937207C770688CDdb539c1F628B8012E6b3",
				"name": "Mini Ghost",
				"decimals": 10
			},
			"0xa9e14d0bdc5a9ba5d17793a125e18024b208b09e": {
				"ticker": "ELNI",
				"address": "0xA9e14d0BDC5a9Ba5D17793a125E18024B208b09E",
				"name": "Elon Inu",
				"decimals": 11
			},
			"0x600e5d7ecac4a0c5b2711ae4c6c97a5be3f21f9f": {
				"ticker": "DEGEN",
				"address": "0x600E5d7EcAc4a0C5b2711ae4C6C97A5BE3F21F9f",
				"name": "Degen Nodes",
				"decimals": 9
			},
			"0x648f14292e51e0861c27c4ebfc3ac3e7dc2a4eff": {
				"ticker": "MARS",
				"address": "0x648F14292E51e0861C27C4EBfC3Ac3E7dC2a4EFf",
				"name": "Treasure Mars",
				"decimals": 11
			},
			"0x982cdde153e6b006aa057779829789d8375b1a1f": {
				"ticker": "PPLE",
				"address": "0x982cdDe153e6b006aA057779829789D8375b1A1f",
				"name": "Pineapple AI",
				"decimals": 11
			},
			"0x9a542e3dfb16b65f954df8feefb37f4e8ff833cc": {
				"ticker": "ASTRO",
				"address": "0x9a542e3Dfb16B65F954dF8FeEFB37F4e8ff833cC",
				"name": "100 Days Ventures",
				"decimals": 18
			},
			"0x99b3a14507c7c6eb1fb7010956f893241d4824c0": {
				"ticker": "SFAN",
				"address": "0x99B3A14507c7C6EB1fB7010956F893241D4824c0",
				"name": "Super Fantom",
				"decimals": 11
			},
			"0x15c96c20a5d769ab400b69e53a28c00a2556d1c6": {
				"ticker": "CAKE",
				"address": "0x15c96C20a5d769AB400B69E53A28C00a2556D1C6",
				"name": "Crypto Cake",
				"decimals": 12
			},
			"0xa2dc72507b561ce9fdbb99f588bbbbca4b138edb": {
				"ticker": "ELN",
				"address": "0xA2dc72507b561Ce9Fdbb99F588BbbbCa4b138eDb",
				"name": "Elon Gaming",
				"decimals": 11
			},
			"0x9e9d3e99230c3bb4fa7efaa8d92be807a03cf2b4": {
				"ticker": "FLIX",
				"address": "0x9e9d3E99230c3BB4FA7eFAa8D92BE807a03CF2B4",
				"name": "FLIX Token",
				"decimals": 18
			},
			"0x434d475ec7637455f67beb90007223ecbadce293": {
				"ticker": "PAN",
				"address": "0x434d475Ec7637455f67beB90007223eCbADCE293",
				"name": "Panda Diamond",
				"decimals": 10
			},
			"0x06e12990682a6f5995eb28648fb9df0df4422d0f": {
				"ticker": "STR",
				"address": "0x06E12990682A6f5995Eb28648fb9df0DF4422D0F",
				"name": "Treasure Star",
				"decimals": 9
			},
			"0x3e13c75bf7ad3d1a38a9f8a5df216b5d322f4801": {
				"ticker": "CAKE",
				"address": "0x3E13c75bf7AD3d1A38a9F8A5DF216B5d322f4801",
				"name": "Wrapped Cake",
				"decimals": 10
			},
			"0x773838dfd90d8f1de6d5e5080f2279eeca669327": {
				"ticker": "DOG",
				"address": "0x773838dFD90d8f1DE6d5E5080F2279EEcA669327",
				"name": "Crypto Doge",
				"decimals": 9
			},
			"0x8b5484823e34601496b4b9ed4c4451e5cf1ca6c1": {
				"ticker": "CMP",
				"address": "0x8b5484823e34601496B4b9Ed4c4451e5Cf1ca6C1",
				"name": "Chimp Robot",
				"decimals": 11
			},
			"0x6254ac979f6b7f1a4a62262799799208deb7a457": {
				"ticker": "POLAR",
				"address": "0x6254ac979f6B7F1A4A62262799799208DeB7A457",
				"name": "Polar Nodes",
				"decimals": 18
			},
			"0x32eff3b1956353ae0ca18cbdc313147a1697473d": {
				"ticker": "PIN",
				"address": "0x32Eff3b1956353aE0CA18cBDC313147A1697473D",
				"name": "Pineapple Markets",
				"decimals": 12
			},
			"0x742da202b233fffe16359843df38bbc769f8f40e": {
				"ticker": "STAR",
				"address": "0x742Da202B233fFFe16359843Df38bbC769f8F40e",
				"name": "Super Star",
				"decimals": 12
			},
			"0xd4ae3b66f178e05e00c8e68ce96e2ccd07791ce4": {
				"ticker": "ELON",
				"address": "0xd4Ae3B66f178e05E00C8e68CE96e2CCd07791ce4",
				"name": "Elon Ledger",
				"decimals": 11
			},
			"0xd028437771f39e0a232afdbf48fb1a554af171f3": {
				"ticker": "AKT",
				"address": "0xD028437771f39e0A232aFDBf48fB1a554AF171F3",
				"name": "Mini Akita",
				"decimals": 9
			},
			"0x784da4927156e349104298b5f306fa646ec3c336": {
				"ticker": "TERT",
				"address": "0x784da4927156E349104298b5F306Fa646Ec3C336",
				"name": "TERT",
				"decimals": 6
			},
			"0xb22438fba9e98a263a031f45ea06e4cc8cd8e5f0": {
				"ticker": "CMRS",
				"address": "0xb22438fbA9E98A263A031f45Ea06E4cc8Cd8E5F0",
				"name": "Crypto Mars",
				"decimals": 10
			},
			"0xd9db9dcd2bda3431c5c6a250a4155615257f4dd1": {
				"ticker": "FLY",
				"address": "0xD9dB9dCd2BDa3431c5C6A250a4155615257F4Dd1",
				"name": "Hoppers Game",
				"decimals": 18
			},
			"0x4b993a3544b01946c474bcb82fd483a5cff32a46": {
				"ticker": "GLA",
				"address": "0x4B993a3544B01946C474bcB82FD483A5cfF32a46",
				"name": "GLAcier",
				"decimals": 9
			},
			"0x87eef758abaf0e54030f94a383b9c29c7febf9af": {
				"ticker": "PARCL",
				"address": "0x87eeF758abaf0E54030f94A383B9C29C7fEbf9Af",
				"name": "Homeowners Association",
				"decimals": 18
			},
			"0xa54c34396098752e3fd42617f6b13d6ec67ecf08": {
				"ticker": "CORGI",
				"address": "0xa54c34396098752E3FD42617F6b13D6ec67Ecf08",
				"name": "Exo Corgi",
				"decimals": 10
			},
			"0x234cbc482460f795052d297ec2e03dca7b9070ae": {
				"ticker": "MarsNode",
				"address": "0x234cBC482460F795052d297EC2e03dCa7b9070Ae",
				"name": "Mars Node",
				"decimals": 18
			},
			"0x1ea97e75af9bbc0215354188e8dac2aeb3d5c720": {
				"ticker": "AVASTRO",
				"address": "0x1ea97E75Af9BBc0215354188e8dac2aEb3d5c720",
				"name": "PLANAVASTRO",
				"decimals": 18
			},
			"0x39215c723c1ae39884721bc659c5beacd85e8006": {
				"ticker": "YUM",
				"address": "0x39215c723C1ae39884721bc659C5BEACd85e8006",
				"name": "Cake X",
				"decimals": 11
			},
			"0x070092b3a985f9e5424351d68730c9a318ad96eb": {
				"ticker": "THRONE",
				"address": "0x070092b3A985f9E5424351D68730c9A318ad96eb",
				"name": "Wrapped sCROWN",
				"decimals": 18
			},
			"0x64b783d7a5e7e4190b73881bfac1a70516cbaa92": {
				"ticker": "FTMS",
				"address": "0x64B783d7a5e7e4190B73881bfac1a70516cbaA92",
				"name": "Fantom SV",
				"decimals": 11
			},
			"0x37d87e316cb4e35163881fdb6c6bc0cdba91dc0a": {
				"ticker": "SET",
				"address": "0x37d87e316CB4e35163881fDb6c6Bc0CdBa91dc0A",
				"name": "Stabilize Token",
				"decimals": 18
			},
			"0xea6bfc4a12f1e25a0a60dc6f26948229fd120ce5": {
				"ticker": "LDOG",
				"address": "0xeA6bFC4a12f1e25a0a60dc6f26948229Fd120Ce5",
				"name": "Little Doge",
				"decimals": 9
			},
			"0x3c0ecf5f430bbe6b16a8911cb25d898ef20805cf": {
				"ticker": "PGL",
				"address": "0x3c0ECf5F430bbE6B16A8911CB25d898Ef20805cF",
				"name": "Pangolin Liquidity",
				"decimals": 18
			},
			"0xb06b0c8d7d33e60f9a2966fbe52d15fc9bc9f7f0": {
				"ticker": "MOON",
				"address": "0xB06B0C8D7D33E60F9A2966Fbe52D15FC9bc9F7F0",
				"name": "Moon Rise",
				"decimals": 12
			},
			"0xc25f856c089daee92c1489f050f66002e078c022": {
				"ticker": "MOON",
				"address": "0xc25f856c089DAeE92c1489F050f66002e078C022",
				"name": "MoonShot",
				"decimals": 9
			},
			"0x9bb7068fef9ee416a1593b7f80f5cc4e21c3ea33": {
				"ticker": "SHIB",
				"address": "0x9BB7068fEf9EE416a1593b7F80f5Cc4E21C3Ea33",
				"name": "Shiba Treasure",
				"decimals": 9
			},
			"0xc557274c429f02ab757846b791fc3ffe4d7e6890": {
				"ticker": "SUGAR",
				"address": "0xC557274C429F02aB757846b791fC3fFe4d7E6890",
				"name": "Sugar",
				"decimals": 18
			},
			"0x16a89a66705d1d24538cbb550d28c3a2a683733a": {
				"ticker": "WHALE",
				"address": "0x16a89A66705D1d24538CBB550d28c3A2A683733a",
				"name": "Whale Cash",
				"decimals": 11
			},
			"0xd09638905f4803566b3975c82c7609a00f537be8": {
				"ticker": "GLAD",
				"address": "0xd09638905F4803566B3975c82C7609A00f537Be8",
				"name": "Glad Token",
				"decimals": 18
			},
			"0xfda229547d2d325b4b34a6bebf8faebf9c612750": {
				"ticker": "MRS",
				"address": "0xFDa229547d2D325b4b34A6bEBF8Faebf9c612750",
				"name": "Mars Ledger",
				"decimals": 11
			},
			"0x10bf0a67842ff7dd2ef6a4a25907ea82c52f5674": {
				"ticker": "FAN",
				"address": "0x10bF0A67842FF7dD2ef6a4a25907eA82c52F5674",
				"name": "King Fantom",
				"decimals": 10
			},
			"0x3b767737d642bf005413abbc0a7ea3eac919ac70": {
				"ticker": "CHIMP",
				"address": "0x3B767737D642Bf005413abbc0A7eA3eaC919Ac70",
				"name": "Chimp SV",
				"decimals": 12
			},
			"0xaaf8afff15fe4fcc58c2de748ff98f3177fbf5f5": {
				"ticker": "TTOM",
				"address": "0xaAf8AffF15FE4fcc58C2De748Ff98f3177FbF5f5",
				"name": "Treasure Fantom",
				"decimals": 11
			},
			"0x3927b3577734fc7d32f239021b15b54550a12e92": {
				"ticker": "DEGEN",
				"address": "0x3927B3577734Fc7d32f239021b15B54550A12E92",
				"name": "DegenFi",
				"decimals": 18
			},
			"0xc7f4debc8072e23fe9259a5c0398326d8efb7f5c": {
				"ticker": "HeC",
				"address": "0xC7f4debC8072e23fe9259A5C0398326d8EfB7f5c",
				"name": "HeroesChained",
				"decimals": 18
			},
			"0xc53391a21d6b3d5649f8d2fda0a67d718a9b0af1": {
				"ticker": "BGOL",
				"address": "0xc53391a21d6B3d5649f8D2FDa0a67D718A9b0Af1",
				"name": "Baby Gold",
				"decimals": 12
			},
			"0x0c7fe9a166942346cf75c7b751f877fddc08a67e": {
				"ticker": "TI",
				"address": "0x0c7Fe9A166942346CF75c7b751f877FddC08a67E",
				"name": "The Island",
				"decimals": 9
			},
			"0xcb937ae2233931b0f62353d68a93a39f2ca5d4a3": {
				"ticker": "BAKA",
				"address": "0xCB937aE2233931b0F62353D68A93A39F2CA5d4A3",
				"name": "Baby Akita",
				"decimals": 9
			},
			"0xc80decece83b7477f2657c74b3a90ac506e588b9": {
				"ticker": "GOL",
				"address": "0xC80DecECE83b7477F2657c74b3a90aC506e588b9",
				"name": "Zen Gold",
				"decimals": 11
			},
			"0x161924cefa3a581e9fc90977fad85b0a73101312": {
				"ticker": "PIN",
				"address": "0x161924cefA3A581E9Fc90977FAd85B0A73101312",
				"name": "Pineapple Beast",
				"decimals": 10
			},
			"0xdefd3f1f18a9f4da42389c9b0b8e319fe1392d9d": {
				"ticker": "MARS",
				"address": "0xdefd3F1f18a9F4dA42389c9B0b8e319Fe1392D9D",
				"name": "Mars Robot",
				"decimals": 10
			},
			"0xdd37355a019e393d5736bfb581ce243e1f6561fc": {
				"ticker": "LVL",
				"address": "0xdd37355A019E393D5736bfB581cE243e1f6561Fc",
				"name": "Levels DAO",
				"decimals": 9
			},
			"0x8b6abe0491f4c3af33d49a36852620a655d17920": {
				"ticker": "HeC",
				"address": "0x8B6ABe0491f4c3Af33D49a36852620a655D17920",
				"name": "HeroesChained",
				"decimals": 1
			},
			"0x56fa4e28039bb78ac76850f9761275b8d0cd2ef2": {
				"ticker": "eSHARE",
				"address": "0x56Fa4e28039BB78AC76850F9761275b8D0Cd2ef2",
				"name": "eDollar Shares",
				"decimals": 18
			},
			"0xade84b50a27ff3c4abe84887e97b789fd5ca117b": {
				"ticker": "PNPL",
				"address": "0xade84B50a27ff3C4ABe84887e97B789FD5ca117b",
				"name": "Meta Pineapple",
				"decimals": 11
			},
			"0x043833991fe5191ed6e3899085294a65b1dda262": {
				"ticker": "MARS",
				"address": "0x043833991FE5191ed6E3899085294A65b1dDA262",
				"name": "Mars Finance",
				"decimals": 11
			},
			"0x78e1728a38b6a90a7af25474c80d28a9f5447bfa": {
				"ticker": "PIN",
				"address": "0x78E1728A38B6A90A7AF25474c80D28a9f5447bfA",
				"name": "Baby Pineapple",
				"decimals": 11
			},
			"0xbfe064775cbda768e9faf84162fa98d8a39107b3": {
				"ticker": "SDOGGY",
				"address": "0xbfE064775cbdA768E9fAf84162FA98D8A39107b3",
				"name": "Snow Doggy",
				"decimals": 18
			},
			"0x4bc7bab5c022bfb18f4cf8f1c94810dcbb292f10": {
				"ticker": "ELN",
				"address": "0x4bC7BaB5C022bFb18f4Cf8f1c94810DCBB292F10",
				"name": "Zen Elon",
				"decimals": 10
			},
			"0x40064ce057fb99a5c8e34f61365cc5996e59ab57": {
				"ticker": "PXT",
				"address": "0x40064CE057Fb99a5c8e34F61365cC5996E59aB57",
				"name": "ProjectX",
				"decimals": 18
			},
			"0x6011580fd7363f99e8853675e3846980cf22ec94": {
				"ticker": "PPLE",
				"address": "0x6011580fD7363f99E8853675e3846980CF22EC94",
				"name": "Mini Pineapple",
				"decimals": 10
			},
			"0x63682bdc5f875e9bf69e201550658492c9763f89": {
				"ticker": "BSGG",
				"address": "0x63682bDC5f875e9bF69E201550658492C9763F89",
				"name": "Betswap.gg",
				"decimals": 18
			},
			"0x574216a694cf7ae4097a8b204687a301de755476": {
				"ticker": "SAS",
				"address": "0x574216A694Cf7ae4097a8B204687a301dE755476",
				"name": "Sashimi DAO",
				"decimals": 6
			},
			"0xbef904955f86ab2a7941483d5ace3aae0658e6c3": {
				"ticker": "PAN",
				"address": "0xbeF904955F86ab2A7941483d5aCe3AAE0658E6c3",
				"name": "Panda Queen",
				"decimals": 12
			},
			"0xf644f8a9fb95cc9bd8db1a07931f23aca52c7c2e": {
				"ticker": "SHIB",
				"address": "0xF644F8A9FB95cc9BD8dB1a07931f23ACa52c7C2E",
				"name": "Shiba Farm",
				"decimals": 9
			},
			"0xe1ddef90032d42e7189a3451638417d33be1b7ca": {
				"ticker": "$SIMIAN",
				"address": "0xE1dDEf90032D42e7189A3451638417d33be1b7Ca",
				"name": "SIMIAN NODES",
				"decimals": 18
			},
			"0xbc61c7ecef56e40404fc359ef4dfd6e7528f2b09": {
				"ticker": "JLP",
				"address": "0xbC61C7eCEf56E40404fC359ef4dfd6E7528f2B09",
				"name": "Joe LP Token",
				"decimals": 18
			},
			"0xc72313136b1120a5017401719df2fece18ff26d6": {
				"ticker": "ESTR",
				"address": "0xc72313136B1120A5017401719Df2fEce18fF26D6",
				"name": "Exo Star",
				"decimals": 10
			},
			"0x2f6f07cdcf3588944bf4c42ac74ff24bf56e7590": {
				"ticker": "STG",
				"address": "0x2F6F07CDcf3588944Bf4C42aC74ff24bF56e7590",
				"name": "StargateToken",
				"decimals": 18
			},
			"0x19105d3eb1d22e6689d9f11fd22b974ec23579ca": {
				"ticker": "OELN",
				"address": "0x19105d3eB1D22E6689d9F11Fd22B974eC23579cA",
				"name": "OMG Elon",
				"decimals": 12
			},
			"0xaa2439dbad718c9329a5893a51a708c015f76346": {
				"ticker": "OTWO",
				"address": "0xAA2439DBAd718c9329a5893A51a708C015F76346",
				"name": "O2 Token",
				"decimals": 9
			},
			"0x7eadbd6f5dbb6d02f3adf52640f1c6ab3a0dd931": {
				"ticker": "SAPL",
				"address": "0x7eADBd6f5dBb6d02f3adf52640f1c6aB3A0Dd931",
				"name": "Sushi Apple",
				"decimals": 9
			},
			"0xac9bf1d34593f1b130d99c02b8b35e117eb43a95": {
				"ticker": "PLAYMATE",
				"address": "0xAc9bf1D34593F1b130d99c02B8b35E117Eb43a95",
				"name": "Playmate",
				"decimals": 9
			},
			"0xa6ce3dc45de8c814a020adf66d2983e967189c7a": {
				"ticker": "APL",
				"address": "0xA6ce3DC45dE8c814a020aDf66d2983e967189C7a",
				"name": "Apple Queen",
				"decimals": 9
			},
			"0x7dac911fa6adae1efb8f4cfcde0d108edc5b862a": {
				"ticker": "ELN",
				"address": "0x7dAc911fA6ADae1eFB8F4CfcdE0D108Edc5b862a",
				"name": "Elon SV",
				"decimals": 10
			},
			"0xb0472f429e0410f3e0142a0746a1d63aa2a7e6d1": {
				"ticker": "WHALE",
				"address": "0xb0472f429E0410f3E0142a0746a1D63aa2A7E6d1",
				"name": "Whale INC",
				"decimals": 10
			},
			"0xdffdf9b1a0444a88b05584be7f61bc44cdfa2453": {
				"ticker": "GHST",
				"address": "0xdfFDf9b1A0444A88b05584be7F61bC44cDfa2453",
				"name": "Queen Ghost",
				"decimals": 9
			},
			"0xfbf9eb38f98473c44ec7663ada250d0a32de0c56": {
				"ticker": "APEC",
				"address": "0xFBF9eb38f98473c44ec7663Ada250D0A32de0c56",
				"name": "Ape Chain",
				"decimals": 9
			},
			"0xe381aecb67460780e5952b2aba83c5ea9fcfba8f": {
				"ticker": "CORES",
				"address": "0xe381aecB67460780e5952b2abA83c5eA9fcFba8F",
				"name": "CORES",
				"decimals": 18
			},
			"0xd0c8d6f5c964389f1252457146eec9852e32b888": {
				"ticker": "MARS",
				"address": "0xD0C8D6F5C964389F1252457146eeC9852e32B888",
				"name": "Mars DAO",
				"decimals": 9
			},
			"0xc6651db02c9be30306b5259e2751d6539bf22f90": {
				"ticker": "GTH",
				"address": "0xC6651dB02c9be30306B5259E2751D6539bf22f90",
				"name": "Fantasy Glitch",
				"decimals": 11
			},
			"0x6e2c53be1931de62ceacfd54e0773b071acd8e85": {
				"ticker": "MRST",
				"address": "0x6E2C53BE1931dE62cEacFd54e0773B071aCd8e85",
				"name": "Mars Treasure",
				"decimals": 12
			},
			"0xe9de33a00e835b110c60b66f242b47f77c0c9713": {
				"ticker": "GTH",
				"address": "0xe9de33A00E835b110C60b66F242B47F77C0c9713",
				"name": "Glitch Protocol",
				"decimals": 11
			},
			"0xb6b892954ca344803f229226b1b0efc0856fdf0e": {
				"ticker": "BSHARE",
				"address": "0xb6b892954ca344803f229226B1B0Efc0856FdF0e",
				"name": "BSHARE",
				"decimals": 18
			},
			"0x5dd32e3e7d18de9c7cea10a7f8a108c98a40c926": {
				"ticker": "CMPB",
				"address": "0x5dD32e3e7D18de9C7Cea10A7F8a108c98A40c926",
				"name": "Chimp Beast",
				"decimals": 12
			},
			"0x6083ea455cef04b27152e616aae8580f536d084a": {
				"ticker": "RAINBOW",
				"address": "0x6083eA455CEf04b27152E616AAE8580F536D084A",
				"name": "Rainbow",
				"decimals": 18
			},
			"0xea068fba19ce95f12d252ad8cb2939225c4ea02d": {
				"ticker": "FIEF",
				"address": "0xeA068Fba19CE95f12d252aD8Cb2939225C4Ea02D",
				"name": "Fief",
				"decimals": 18
			},
			"0x86b4096e45be4420b3e129a131dc5127aee33b60": {
				"ticker": "FTM",
				"address": "0x86b4096E45bE4420b3E129a131dC5127aeE33B60",
				"name": "Fantom Queen",
				"decimals": 10
			},
			"0x88e0b40148881fc7c572e62620f3e036ec2a9d23": {
				"ticker": "MFER",
				"address": "0x88e0b40148881Fc7c572e62620F3e036EC2A9d23",
				"name": "MferNodes",
				"decimals": 18
			},
			"0x5ae0e283330a1b41a49a3927905f6efc7e2ed384": {
				"ticker": "ZEUS",
				"address": "0x5aE0e283330a1B41a49a3927905F6eFc7e2eD384",
				"name": "Zeus Nodes",
				"decimals": 18
			},
			"0x61fb8d136839824933ce732cd263f1c3870b90c3": {
				"ticker": "CMP",
				"address": "0x61fb8D136839824933Ce732CD263F1c3870b90C3",
				"name": "Chimp Ledger",
				"decimals": 11
			},
			"0x7cb4f9a1214bac54181596d3fb862dc0dd0f8f19": {
				"ticker": "JewelNode",
				"address": "0x7cb4f9A1214BaC54181596D3fb862dC0dd0F8F19",
				"name": "Jewel Node",
				"decimals": 18
			},
			"0x43fe163e24610f098217e37425071abfd9c5aa9d": {
				"ticker": "CHIMP",
				"address": "0x43fE163E24610F098217E37425071abFD9c5aa9D",
				"name": "Mini Chimp",
				"decimals": 11
			},
			"0x46583656a2a5b67ae54e3e63d365bb7f48623807": {
				"ticker": "MOON",
				"address": "0x46583656a2A5B67aE54e3E63D365Bb7F48623807",
				"name": "Zen Moon",
				"decimals": 9
			},
			"0x55a9251252c7e56a884d5ddb514f2ff225660961": {
				"ticker": "PI",
				"address": "0x55a9251252C7e56A884d5DdB514F2ff225660961",
				"name": "PI DAO",
				"decimals": 9
			},
			"0x59c01691b0a87748f2402fa30012e4f397029b30": {
				"ticker": "STAR",
				"address": "0x59C01691B0A87748F2402fa30012e4f397029B30",
				"name": "Star SV",
				"decimals": 10
			},
			"0x0ceab70494ca083c356337ae5b72cc0c2cb42c7a": {
				"ticker": "AGMT",
				"address": "0x0ceAB70494CA083c356337aE5B72Cc0C2CB42c7A",
				"name": "Agamotto Token",
				"decimals": 18
			},
			"0x321e7092a180bb43555132ec53aaa65a5bf84251": {
				"ticker": "gOHM",
				"address": "0x321E7092a180BB43555132ec53AaA65a5bF84251",
				"name": "Governance OHM",
				"decimals": 18
			},
			"0x675906a1860f429e423ab70597f795b233f4fe09": {
				"ticker": "LZAP",
				"address": "0x675906a1860f429E423Ab70597F795b233F4FE09",
				"name": "Little Thunder",
				"decimals": 10
			},
			"0x78b5711b3e880c670f9f18d2db7051cc22839a4c": {
				"ticker": "GHS",
				"address": "0x78b5711B3E880c670F9F18D2Db7051cc22839A4c",
				"name": "Ghost Token",
				"decimals": 10
			},
			"0x24cb0bcd6a415edb68302db969479782cbb223fa": {
				"ticker": "STAR",
				"address": "0x24Cb0bCd6A415EDB68302dB969479782cBb223fA",
				"name": "Star Markets",
				"decimals": 9
			},
			"0x00ee200df31b869a321b10400da10b561f3ee60d": {
				"ticker": "ACRE",
				"address": "0x00EE200Df31b869a321B10400Da10b561F3ee60d",
				"name": "Arable Protocol",
				"decimals": 18
			},
			"0x699242f62ab97585ecd4e5906ade2d6b44c07ff1": {
				"ticker": "Arb",
				"address": "0x699242F62aB97585EcD4e5906AdE2d6B44C07fF1",
				"name": "Arbitrum",
				"decimals": 18
			},
			"0xbcfb46babc31d38a87e4f2fa27170b245678c872": {
				"ticker": "KONG",
				"address": "0xBcfb46bAbC31D38a87E4F2fA27170B245678C872",
				"name": "King Kong",
				"decimals": 9
			},
			"0x6555cd6241049e5854396603398d56b10f7fffa8": {
				"ticker": "THNI",
				"address": "0x6555cd6241049E5854396603398D56b10F7ffFA8",
				"name": "Thunder INC",
				"decimals": 12
			},
			"0x88821e22983cdb79c93114ae76536fc7ed1ac042": {
				"ticker": "GOL",
				"address": "0x88821E22983Cdb79C93114aE76536fc7eD1ac042",
				"name": "Treasure Gold",
				"decimals": 9
			},
			"0x7a98502c91e05fb4bfd32072e6bfc825ff6d1808": {
				"ticker": "CORGI",
				"address": "0x7a98502c91E05fb4BFD32072e6BfC825ff6d1808",
				"name": "Wrapped Corgi",
				"decimals": 9
			},
			"0x5888e15958f16276c8d52fd2af6a0d19ae166aa5": {
				"ticker": "CRG",
				"address": "0x5888E15958F16276C8d52fd2af6a0D19aE166AA5",
				"name": "Queen Corgi",
				"decimals": 11
			},
			"0x2d2ed197d22100335e049720124add4ec40f1f12": {
				"ticker": "CMP",
				"address": "0x2d2ed197d22100335E049720124adD4eC40F1F12",
				"name": "Chimp Coin",
				"decimals": 12
			},
			"0x961c8c0b1aad0c0b10a51fef6a867e3091bcef17": {
				"ticker": "DYP",
				"address": "0x961C8c0B1aaD0c0b10a51FeF6a867E3091BCef17",
				"name": "DeFiYieldProtocol",
				"decimals": 18
			},
			"0x130bbd4731474eae7b4fab2b93a9d6c20a57649d": {
				"ticker": "FLOKI",
				"address": "0x130bbD4731474EAE7B4Fab2B93A9d6C20a57649D",
				"name": "Wrapped Floki",
				"decimals": 10
			},
			"0x5b4debc01b7ec5dfcaa5b5a8ad0d28d2b74afa5f": {
				"ticker": "ZAP",
				"address": "0x5B4dEbc01B7eC5DFCaa5B5a8Ad0D28d2b74aFA5F",
				"name": "Zen Thunder",
				"decimals": 9
			},
			"0x94dfa479d45d24f6aa0fa7d53d51290455814442": {
				"ticker": "KNG",
				"address": "0x94DFa479d45D24F6aA0Fa7D53D51290455814442",
				"name": "Crypto Kings",
				"decimals": 9
			},
			"0x94960952876e3ed6a7760b198354fcc5319a406a": {
				"ticker": "RBX",
				"address": "0x94960952876e3ED6A7760B198354fCc5319A406a",
				"name": "RBX",
				"decimals": 18
			},
			"0x84c996bc1d2bfca3de9214487bd11d2293a4a68e": {
				"ticker": "PIZZA",
				"address": "0x84C996Bc1D2BFcA3DE9214487bd11D2293a4a68E",
				"name": "Pizza Game",
				"decimals": 18
			},
			"0x1a4ba81b2c937a7ac2c2ed13ac12c135e7040bb2": {
				"ticker": "GOL",
				"address": "0x1a4bA81B2c937A7aC2c2Ed13Ac12c135e7040BB2",
				"name": "Gold X",
				"decimals": 12
			},
			"0x42006ab57701251b580bdfc24778c43c9ff589a1": {
				"ticker": "EVO",
				"address": "0x42006Ab57701251B580bDFc24778C43c9ff589A1",
				"name": "EVO",
				"decimals": 18
			},
			"0xacc301a753a1123ac3093d2bef78a9f27ff01d9e": {
				"ticker": "GLD",
				"address": "0xacc301A753A1123aC3093D2bEF78a9f27ff01D9e",
				"name": "Queen Gold",
				"decimals": 10
			},
			"0x59c97c98279723d8c3de0282c311c5c3dea076d6": {
				"ticker": "Crystal",
				"address": "0x59c97c98279723D8c3de0282c311c5c3dEa076d6",
				"name": "Crystal",
				"decimals": 18
			},
			"0x9fc95c8b2619e2f945e7eea79dd5c7474af9e7e2": {
				"ticker": "MRSP",
				"address": "0x9FC95C8B2619E2F945e7eeA79DD5c7474aF9e7E2",
				"name": "Mars Project",
				"decimals": 9
			},
			"0x79e5436d0b35588c65e0cfaecc9ffc43a8dc424b": {
				"ticker": "FTM",
				"address": "0x79E5436D0b35588c65E0cfAECC9ffC43A8dC424B",
				"name": "Fantom Finance",
				"decimals": 10
			},
			"0xb6ae195f42fec57ac8527e90b833bb547e7778c4": {
				"ticker": "STAR",
				"address": "0xb6AE195f42Fec57Ac8527E90B833bb547e7778c4",
				"name": "Star Fund",
				"decimals": 12
			},
			"0x5cf8638bf8ebe32b78b92532899770ac5243e31f": {
				"ticker": "GLDS",
				"address": "0x5cF8638BF8ebe32B78b92532899770Ac5243E31F",
				"name": "Gold Starter",
				"decimals": 10
			},
			"0xdb4f2785b30143e4aee78a6c59276af1ee971044": {
				"ticker": "VRS",
				"address": "0xDB4F2785B30143e4aEe78A6C59276AF1EE971044",
				"name": "$Verse",
				"decimals": 18
			},
			"0x03a09f57ebb273457d745f30849585b34a2891f8": {
				"ticker": "DOGE",
				"address": "0x03a09f57EbB273457d745f30849585b34a2891F8",
				"name": "Doge King",
				"decimals": 9
			},
			"0xb1b339b255e0c6269c0eeee1cdee2cfd639d0c7a": {
				"ticker": "EANG",
				"address": "0xb1B339B255E0c6269C0eeeE1cDee2cfd639D0c7A",
				"name": "Exo Angel",
				"decimals": 12
			},
			"0xf30820ac30e3c41f4cbfd9d70f215ff327944ac4": {
				"ticker": "MGLD",
				"address": "0xF30820AC30E3c41F4Cbfd9d70f215fF327944Ac4",
				"name": "Mini Gold",
				"decimals": 12
			},
			"0x7d16944a9025836ff5db77e01ad972e08dd9d549": {
				"ticker": "APL",
				"address": "0x7d16944a9025836fF5db77e01AD972e08DD9d549",
				"name": "Fantasy Apple",
				"decimals": 11
			},
			"0xadfb5189452be241288a576fd69d974f96259001": {
				"ticker": "HCHP",
				"address": "0xAdFB5189452Be241288a576fD69D974f96259001",
				"name": "Hachi Protocol",
				"decimals": 11
			},
			"0x4d30c2cc3d2454be6873c97c20ff2b23b08bd1ee": {
				"ticker": "FLK",
				"address": "0x4d30c2CC3D2454Be6873C97C20FF2b23b08Bd1eE",
				"name": "Super Floki",
				"decimals": 11
			},
			"0x62b8091ea9442d3ec140293b7cefca7528581534": {
				"ticker": "SCAP",
				"address": "0x62b8091Ea9442D3Ec140293B7cefCa7528581534",
				"name": "Space Capital",
				"decimals": 9
			},
			"0x642423fbd4d8567816f1a8e8767411da1e4dd4cc": {
				"ticker": "PNPL",
				"address": "0x642423FBD4D8567816F1a8e8767411DA1e4dd4Cc",
				"name": "Zen Pineapple",
				"decimals": 9
			},
			"0x08b87307c97e6edff2a9b9a0414ac7ed218c1ee1": {
				"ticker": "INDEX",
				"address": "0x08B87307C97E6EDfF2a9B9A0414Ac7ed218C1Ee1",
				"name": "Index DAO",
				"decimals": 18
			},
			"0xe5f9b653f6064699edc872e418a02d7c6ce904a9": {
				"ticker": "GHS",
				"address": "0xe5F9b653f6064699Edc872e418A02d7c6Ce904a9",
				"name": "Ghost Cash",
				"decimals": 12
			},
			"0x95852154e57ca99311e7805fd3bf6857558ff8d5": {
				"ticker": "CORGI",
				"address": "0x95852154E57ca99311E7805FD3Bf6857558ff8d5",
				"name": "Baby Corgi",
				"decimals": 10
			},
			"0x769ae917e840451b09c0ec6ff08e5283caef8250": {
				"ticker": "Fly",
				"address": "0x769Ae917E840451B09c0EC6fF08E5283caEF8250",
				"name": "Fly",
				"decimals": 6
			},
			"0x18f3db5173c91f54fdbed1c19b2e489b655ea578": {
				"ticker": "ELON",
				"address": "0x18F3Db5173C91f54FdBeD1c19B2e489b655ea578",
				"name": "Zen Elon",
				"decimals": 12
			},
			"0x283854269366633d434dbc611bfed9f016bee066": {
				"ticker": "ETHS",
				"address": "0x283854269366633d434dBc611bfeD9F016BeE066",
				"name": "Etherstones",
				"decimals": 18
			},
			"0xf7d0cef3635268db34b98d4ea9776ec4d94d2794": {
				"ticker": "PNDC",
				"address": "0xF7d0cEF3635268Db34B98D4eA9776ec4D94D2794",
				"name": "Panda Classic",
				"decimals": 9
			},
			"0x204c2564b857f8e51cdf2d6fee2a43860dbeb642": {
				"ticker": "MOON",
				"address": "0x204C2564b857f8e51CDf2d6fEE2A43860dbeB642",
				"name": "Crypto Moon",
				"decimals": 11
			},
			"0xaa6a5d4c373c0139eb1cd7d47f4f642e07be59d2": {
				"ticker": "SPIN",
				"address": "0xAa6A5D4C373c0139EB1cD7D47f4f642E07Be59d2",
				"name": "Sushi Pineapple",
				"decimals": 10
			},
			"0xe7f4e9e7664070e41fa4acdd6ca77c7b2447383f": {
				"ticker": "PANGCI",
				"address": "0xe7F4e9e7664070e41fA4aCdd6ca77C7b2447383f",
				"name": "PANG CI",
				"decimals": 18
			},
			"0x40ac681fc8e357182e6a28e8a4a1d80fa53ee264": {
				"ticker": "DGE",
				"address": "0x40AC681FC8E357182e6A28e8a4a1d80fa53EE264",
				"name": "Project Doge",
				"decimals": 11
			},
			"0xc00e0d3c2029bf2385baaa975e7290880c92ed0e": {
				"ticker": "GLD",
				"address": "0xC00e0d3C2029BF2385BAAA975e7290880C92eD0E",
				"name": "Gold Dapp",
				"decimals": 11
			},
			"0xe6e7e03b60c0f8daae5db98b03831610a60ffe1b": {
				"ticker": "VTX",
				"address": "0xe6E7e03b60c0F8DaAE5Db98B03831610A60FfE1B",
				"name": "Vector Token",
				"decimals": 18
			},
			"0x97e98ac3c2cc7f9ec4e7251fae08b1b40a525e55": {
				"ticker": "STAR",
				"address": "0x97e98aC3c2CC7F9EC4E7251fAE08B1b40a525E55",
				"name": "Star BTC",
				"decimals": 11
			},
			"0x369df582777f0d0d642ec65e7fb93e292c92b2c3": {
				"ticker": "QAKA",
				"address": "0x369dF582777F0d0d642ec65E7FB93e292C92b2c3",
				"name": "Queen Akita",
				"decimals": 10
			},
			"0xe79ba630dfe4de93882b81da028d810c0a0c885b": {
				"ticker": "GHST",
				"address": "0xE79BA630dFe4DE93882B81da028D810c0a0c885B",
				"name": "Ghost SV",
				"decimals": 9
			},
			"0x2bab4fe8fc32942bf1104bbfac9e7cf5582e624f": {
				"ticker": "HCI",
				"address": "0x2bAb4Fe8FC32942bf1104BbfAc9e7cF5582E624F",
				"name": "Hachi AVAX",
				"decimals": 10
			},
			"0xafe927f13677a6b4b5e911d51ccb2af935d29b9c": {
				"ticker": "GHST",
				"address": "0xAFe927F13677a6b4B5e911D51ccb2aF935D29b9C",
				"name": "Ghost Markets",
				"decimals": 12
			},
			"0x1b9187da7e95e1eb7acc63f58a28de149f469b96": {
				"ticker": "WHALE",
				"address": "0x1b9187dA7E95E1eb7acc63f58A28De149f469B96",
				"name": "Whale Block",
				"decimals": 9
			},
			"0xbb632ffe8241f10682d225d964cfba19fec0e233": {
				"ticker": "TOM",
				"address": "0xBB632FfE8241f10682D225d964cFba19FEc0E233",
				"name": "Meta Fantom",
				"decimals": 9
			},
			"0x4bd2b743aa03113d13f9d645eb0aec129d5eadc9": {
				"ticker": "MGCH",
				"address": "0x4Bd2b743AA03113d13f9d645Eb0AEC129d5eaDc9",
				"name": "Meta Glitch",
				"decimals": 11
			},
			"0xce7bf3daab45716b643de75262690b6e494ea5e8": {
				"ticker": "LPDA",
				"address": "0xCe7BF3DaAb45716B643De75262690b6e494eA5E8",
				"name": "Little Panda",
				"decimals": 12
			},
			"0x67df8a6da41611d3100d4a911e9119c5ee344457": {
				"ticker": "TNDR",
				"address": "0x67dF8A6da41611D3100D4a911E9119C5eE344457",
				"name": "Thunder Rocket",
				"decimals": 12
			},
			"0x1f2f2f652330b576f5e0c793359038b50097d516": {
				"ticker": "DOGE",
				"address": "0x1F2F2F652330b576F5E0C793359038b50097d516",
				"name": "Doge Markets",
				"decimals": 9
			},
			"0xa8627ae08b7ac0f6c23a06351509ec5edc655273": {
				"ticker": "GLD",
				"address": "0xA8627AE08b7AC0f6C23A06351509ec5EdC655273",
				"name": "Gold NET",
				"decimals": 9
			},
			"0xd206bfe127e1f77af88213fa7f2484a2dee5d665": {
				"ticker": "ANG",
				"address": "0xd206Bfe127e1f77af88213fA7f2484A2DEe5d665",
				"name": "Angel Token",
				"decimals": 11
			},
			"0x074c3a55e4c8d8fc0489f5026664ac452809e870": {
				"ticker": "STAR",
				"address": "0x074C3A55E4C8D8Fc0489f5026664Ac452809E870",
				"name": "Star Coin",
				"decimals": 10
			},
			"0x4529d4dde9c608935ad9826b4f2ada15f5c27243": {
				"ticker": "OF",
				"address": "0x4529d4Dde9c608935ad9826B4f2AdA15f5C27243",
				"name": "OnlyFans",
				"decimals": 18
			},
			"0x0b47d67a059a22fc741faadf2d2bcf216d3cdd7a": {
				"ticker": "Dice",
				"address": "0x0b47D67a059A22FC741fAaDF2d2bCf216d3cDd7A",
				"name": "Dice DAO",
				"decimals": 18
			},
			"0xb8e932950907943c7249ea317bf87a14af4e0255": {
				"ticker": "GCH",
				"address": "0xB8E932950907943c7249Ea317BF87A14Af4e0255",
				"name": "Little Glitch",
				"decimals": 10
			},
			"0x695fa794d59106cebd40ab5f5ca19f458c723829": {
				"ticker": "HAKU",
				"address": "0x695Fa794d59106cEbd40ab5f5cA19F458c723829",
				"name": "HakuSwap Token",
				"decimals": 18
			},
			"0x7bb242ee22f74a57671035fddad2fde35cdc9995": {
				"ticker": "PNDA",
				"address": "0x7BB242eE22F74a57671035fDDaD2Fde35cdc9995",
				"name": "Panda AI",
				"decimals": 9
			},
			"0xd8d62fd54a5e7d01c039694b31701937414df3bc": {
				"ticker": "GHS",
				"address": "0xd8d62fd54A5e7d01C039694B31701937414dF3bc",
				"name": "Crypto Ghost",
				"decimals": 11
			},
			"0xeff502356ad9c22aa05f804bd64dbc717df09803": {
				"ticker": "MARS",
				"address": "0xeff502356aD9c22aa05F804bd64DBc717dF09803",
				"name": "Mars Beast",
				"decimals": 10
			},
			"0xdc4e4e37373d04a1384f73080e32d23aacc7a6a1": {
				"ticker": "SHB",
				"address": "0xDC4e4e37373d04A1384F73080E32D23AAcc7a6A1",
				"name": "Shiba Network",
				"decimals": 10
			},
			"0x2f8cd6d4b5604a322fbf0f0a8eb4a33c7fc30d03": {
				"ticker": "CHI",
				"address": "0x2f8cd6D4b5604A322FbF0f0A8Eb4a33c7Fc30d03",
				"name": "Chimp DAO",
				"decimals": 11
			},
			"0xb4411b6c04de597802680f3ea82119faee20a920": {
				"ticker": "GUNS",
				"address": "0xb4411B6C04DE597802680f3ea82119FAEe20a920",
				"name": "Gun Games",
				"decimals": 9
			},
			"0xfc826dda66088af3d699b1476f48be76ee3b3278": {
				"ticker": "PIN",
				"address": "0xfC826dda66088AF3d699b1476F48Be76EE3b3278",
				"name": "Sushi Pineapple",
				"decimals": 12
			},
			"0x6a4933420cf689712acd1ac6596716c8a10182e3": {
				"ticker": "GHST",
				"address": "0x6A4933420cf689712ACd1aC6596716c8a10182e3",
				"name": "Ghost Markets",
				"decimals": 12
			},
			"0x306a3488ab5d3bfff7d8fe816e468e26b0e60620": {
				"ticker": "ELN",
				"address": "0x306A3488AB5d3bfFF7d8FE816E468E26B0E60620",
				"name": "Elon DAO",
				"decimals": 9
			},
			"0x7872ed1e57113bc6011ac72aec72a0a0efeba12a": {
				"ticker": "YUMD",
				"address": "0x7872Ed1e57113BC6011Ac72Aec72a0a0efeBa12a",
				"name": "Cake Dapp",
				"decimals": 11
			},
			"0x1e38057894906400ba34ed4daafb717f5414caca": {
				"ticker": "FTMB",
				"address": "0x1E38057894906400bA34ED4dAAfB717f5414CaCa",
				"name": "Fantom Beast",
				"decimals": 12
			},
			"0x77a2deed889cd3eb75bc6fa237d2daf403e9390e": {
				"ticker": "SUBZERO",
				"address": "0x77A2DeEd889cd3EB75Bc6FA237d2Daf403E9390E",
				"name": "SUBZERO",
				"decimals": 18
			},
			"0x1c0608c7e4f5e6973af86eb2e185d4a7bf0cdf15": {
				"ticker": "MOON",
				"address": "0x1C0608C7E4F5E6973af86eB2e185d4a7BF0CDF15",
				"name": "Crypto Moon",
				"decimals": 10
			},
			"0x0592af5414f2f8d90a5ae3c25e937804d3965c87": {
				"ticker": "HOOP",
				"address": "0x0592af5414F2f8d90a5ae3C25E937804D3965C87",
				"name": "Hoopoe",
				"decimals": 18
			},
			"0x963dd0e9cb2e22dc98665ff04dc3a8b4368e1cf9": {
				"ticker": "PDA",
				"address": "0x963Dd0e9CB2E22dc98665FF04Dc3A8b4368e1cf9",
				"name": "Project Panda",
				"decimals": 12
			},
			"0xacdc4f2a3f412f745cd1b1afe2363f25836378d3": {
				"ticker": "MARS",
				"address": "0xACDc4f2a3f412f745cd1B1Afe2363F25836378D3",
				"name": "Crypto Mars",
				"decimals": 12
			},
			"0xa1a64f57bc64bad64a9655d191ad8397df9b5dcc": {
				"ticker": "MCHAIN",
				"address": "0xA1a64F57bC64baD64a9655d191Ad8397df9b5dcC",
				"name": "MultiChain Bridge",
				"decimals": 9
			},
			"0x0d16ecae23dcecaf3df5eedb5e0ee207f42af813": {
				"ticker": "THNG",
				"address": "0x0D16eCAE23dceCAf3dF5eEdb5e0eE207f42af813",
				"name": "Thunder Gaming",
				"decimals": 10
			},
			"0x8a52ed4c19c5a3e65bf75fdc56e63785d08b6d45": {
				"ticker": "APL",
				"address": "0x8A52eD4c19c5a3E65bF75FdC56e63785d08B6D45",
				"name": "Apple Swap",
				"decimals": 10
			},
			"0x6b38e8a635271b9a4772973f7c41808a1a2d599d": {
				"ticker": "MGOL",
				"address": "0x6B38e8A635271B9A4772973F7c41808a1a2D599d",
				"name": "Mini Gold",
				"decimals": 12
			},
			"0xf5ccdeaacd1c0f33af0fab58316f2ff8c52c9bd3": {
				"ticker": "SAPE",
				"address": "0xf5CcDeaACd1C0F33Af0faB58316F2FF8C52C9bD3",
				"name": "Sushi Ape",
				"decimals": 9
			},
			"0xde53d15d635313c7ae30585148c07b2f557a82da": {
				"ticker": "HCIM",
				"address": "0xde53d15D635313c7AE30585148c07B2f557A82DA",
				"name": "Hachi Monster",
				"decimals": 10
			},
			"0x420adc7083fb04287c81ae5ec763e599e4900112": {
				"ticker": "CHI",
				"address": "0x420ADC7083Fb04287c81ae5Ec763E599e4900112",
				"name": "Sushi Chimp",
				"decimals": 9
			},
			"0xb29da44318de7affdf844bb7fbd09790fd6e72b4": {
				"ticker": "LUNA",
				"address": "0xB29Da44318DE7AfFdf844BB7FBD09790FD6E72B4",
				"name": "LUNA",
				"decimals": 9
			},
			"0xab928c139ca62c3060852a629bff505915509c7e": {
				"ticker": "PHOENIX",
				"address": "0xAb928c139cA62C3060852A629bFf505915509c7E",
				"name": "phoenix.finance",
				"decimals": 18
			},
			"0x17bccdde22c05aa0af06c5f8d61536dfe6e3c887": {
				"ticker": "ANGS",
				"address": "0x17bCCDDe22C05Aa0af06C5F8D61536dfE6E3c887",
				"name": "Angel Starter",
				"decimals": 9
			},
			"0x6d4b2e045b4072b43269859d7a27f68175153d84": {
				"ticker": "ISIS",
				"address": "0x6d4b2E045b4072B43269859d7a27F68175153d84",
				"name": "ISIS MONEY",
				"decimals": 8
			},
			"0xeaf45d6e9315fefc393eed3d55e4a7df35ab9a6b": {
				"ticker": "SBA",
				"address": "0xeAF45d6E9315fEfC393Eed3d55E4A7DF35AB9A6B",
				"name": "Shiba Queen",
				"decimals": 11
			},
			"0xdf2bddd5b3f33e03fbac4f935a0cc5009fdbaf3b": {
				"ticker": "CORK",
				"address": "0xdf2bdDd5b3f33e03FBAC4f935A0cc5009FDbAF3b",
				"name": "Corkscrew",
				"decimals": 18
			},
			"0x3d43cbdc14f65d7c4dd0b85acc5c98fee92bd811": {
				"ticker": "FLKI",
				"address": "0x3d43CbDC14F65d7C4dD0B85aCc5c98Fee92BD811",
				"name": "Floki Infinity",
				"decimals": 12
			},
			"0x7f679864137662a9ba484fbb3456d91a9b89bfbd": {
				"ticker": "ECHI",
				"address": "0x7f679864137662A9BA484Fbb3456d91A9b89bfBd",
				"name": "Exo Chimp",
				"decimals": 12
			},
			"0x8eef32663124ca49fd06d5b50258c1af9db17534": {
				"ticker": "MEAD",
				"address": "0x8eEf32663124Ca49fd06D5B50258C1Af9DB17534",
				"name": "Mead",
				"decimals": 18
			},
			"0xdf024daf83ad2cee2cba56f87130ac631d6b1b0b": {
				"ticker": "APE",
				"address": "0xdF024daf83ad2ceE2cbA56f87130Ac631d6B1b0B",
				"name": "Fantasy Ape",
				"decimals": 11
			},
			"0x8fd66af00e9f07f3312f5a2e2967ba41d6152f9c": {
				"ticker": "AKA",
				"address": "0x8FD66Af00E9F07F3312F5a2E2967BA41D6152F9c",
				"name": "Project Akita",
				"decimals": 11
			},
			"0xd61e364bf088b080bfc1ca1a8cd1889aaa8957ab": {
				"ticker": "SHIB",
				"address": "0xD61E364bF088B080bfc1cA1a8Cd1889Aaa8957aB",
				"name": "Mini Shiba",
				"decimals": 9
			},
			"0xb60c86ecacb0e7fd42ad305434747094b479d5c5": {
				"ticker": "TCHI",
				"address": "0xb60C86ECacB0E7Fd42Ad305434747094b479D5c5",
				"name": "Treasure Chimp",
				"decimals": 11
			},
			"0x151a3db26c9c95019deb9da9bdff01ce0d4c1cc6": {
				"ticker": "AKAF",
				"address": "0x151A3DB26C9C95019deB9Da9BDFF01CE0d4C1Cc6",
				"name": "Akita Fund",
				"decimals": 10
			},
			"0x31f25502e88a151e245c6fee0dbd2f13b9ce0974": {
				"ticker": "WINGS",
				"address": "0x31F25502E88a151e245C6feE0dbd2F13b9CE0974",
				"name": "Wrapped Angel",
				"decimals": 10
			},
			"0xaa3e536257cdb71f4a216874588efb550e29ebc7": {
				"ticker": "DANK",
				"address": "0xaa3E536257CdB71f4A216874588EFb550e29EBc7",
				"name": "DANK DAO",
				"decimals": 9
			},
			"0x42839a0a382ffc41dd4bdc970103fec227396a18": {
				"ticker": "preTEST",
				"address": "0x42839A0A382ffC41dd4bDC970103FEc227396a18",
				"name": "100 Tests vFinal",
				"decimals": 18
			},
			"0xe8b07076c8fae6d5610d687b6c6d3ab4ab4bacf2": {
				"ticker": "GLT",
				"address": "0xe8b07076C8fae6d5610d687b6c6D3AB4AB4BAcf2",
				"name": "Glitch Chain",
				"decimals": 9
			},
			"0xef3cc3b3c47af472829a80447ab7747342e9bb8e": {
				"ticker": "MOON",
				"address": "0xEf3cc3B3C47aF472829A80447AB7747342E9bb8E",
				"name": "Moon Ledger",
				"decimals": 10
			},
			"0x7afee2dfd2d5ec660cdef1e1407f6161e77bc169": {
				"ticker": "BAT",
				"address": "0x7aFEe2DFd2D5Ec660CDeF1E1407F6161e77BC169",
				"name": "Battle Nodes",
				"decimals": 9
			},
			"0x5817d4f0b62a59b17f75207da1848c2ce75e7af4": {
				"ticker": "VTX",
				"address": "0x5817D4F0b62A59b17f75207DA1848C2cE75e7AF4",
				"name": "Vector",
				"decimals": 18
			},
			"0xbc0d1fe06b2f7b05eee4035a704026da28a87d18": {
				"ticker": "MGOL",
				"address": "0xbC0d1fE06b2F7B05eee4035A704026dA28a87d18",
				"name": "Mini Gold",
				"decimals": 11
			},
			"0x8d3e008aae2650f66bd3adeb20f27460ba4e0d63": {
				"ticker": "QAPL",
				"address": "0x8D3E008AAe2650f66bd3aDEb20f27460Ba4E0d63",
				"name": "Queen Pineapple",
				"decimals": 11
			},
			"0xc70d5c53314ac8ef79ec85294e239b246d3543f8": {
				"ticker": "SIMIAN",
				"address": "0xC70d5c53314Ac8Ef79ec85294e239B246d3543F8",
				"name": "Simian Nodes",
				"decimals": 18
			},
			"0x5541d83efad1f281571b343977648b75d95cdac2": {
				"ticker": "GRAPE",
				"address": "0x5541D83EFaD1f281571B343977648B75d95cdAC2",
				"name": "Grape Finance",
				"decimals": 18
			},
			"0x409eda4a37d3b4e4dd419ba6cbbfe9225fd61b9a": {
				"ticker": "AKITA",
				"address": "0x409Eda4A37D3b4e4DD419BA6CbBFE9225fD61B9a",
				"name": "Queen Akita",
				"decimals": 10
			},
			"0xfc4a2a15d6574f0b5d5b723cd4dea750a5612bf3": {
				"ticker": "GDSY",
				"address": "0xFC4A2a15d6574f0b5d5B723cD4DEa750a5612bF3",
				"name": "Golden Society Token",
				"decimals": 18
			},
			"0x0a7993d1935136ebcb8ada661fff7e0f00b848dc": {
				"ticker": "FAPL",
				"address": "0x0a7993d1935136ebcB8ADa661FFF7E0f00b848Dc",
				"name": "Fantasy Apple",
				"decimals": 10
			},
			"0x3e2dafe992153706020387747c5cc68630c66f10": {
				"ticker": "MOON",
				"address": "0x3E2dAfe992153706020387747c5Cc68630c66f10",
				"name": "Moon X",
				"decimals": 9
			},
			"0x5b8df2855b2f3672b4c0401a06d4c6c2f6de6952": {
				"ticker": "CON",
				"address": "0x5B8DF2855b2F3672b4c0401a06d4c6c2F6dE6952",
				"name": "Contexia",
				"decimals": 18
			},
			"0xb445c0afbf40085253d27cec274778636465d36a": {
				"ticker": "HCID",
				"address": "0xb445c0AfbF40085253D27CeC274778636465d36A",
				"name": "Hachi DAO",
				"decimals": 11
			},
			"0x6def411fcf4dbf53934e66092e191716589829d9": {
				"ticker": "PLAYMATES",
				"address": "0x6dEf411FCF4DBF53934E66092E191716589829D9",
				"name": "Redlight Node District",
				"decimals": 18
			},
			"0x7c8e49c609f05bb7d574e56bdf6c2a4b7d3e0640": {
				"ticker": "GLTP",
				"address": "0x7c8E49c609f05bB7d574e56BdF6C2a4B7D3e0640",
				"name": "Glitch Protocol",
				"decimals": 10
			},
			"0x8b6663725d7e7368b01eefddc16be085e492498e": {
				"ticker": "KISHIMOTO",
				"address": "0x8B6663725D7e7368B01EeFdDc16Be085e492498E",
				"name": "Kishimoto Inu",
				"decimals": 9
			},
			"0x7c2f4ed6cdba093d315d5dfc85f5121ef3dadf0c": {
				"ticker": "BAPL",
				"address": "0x7C2f4ed6Cdba093d315D5dfc85f5121ef3DAdF0C",
				"name": "Baby Pineapple",
				"decimals": 12
			},
			"0x93ab3f17d655e46882664b1260b90ecb4a36b727": {
				"ticker": "ANG",
				"address": "0x93AB3f17D655e46882664b1260B90eCB4A36B727",
				"name": "Angel DAO",
				"decimals": 9
			},
			"0x6a20f04c7ddd7468cf2b611d3a3915fab80baf10": {
				"ticker": "MRST",
				"address": "0x6a20f04c7DDD7468CF2b611D3A3915faB80BAF10",
				"name": "Mars Token",
				"decimals": 12
			},
			"0xb0a8e082e5f8d2a04e74372c1be47737d85a0e73": {
				"ticker": "USV",
				"address": "0xb0a8E082E5f8d2a04e74372c1bE47737d85A0E73",
				"name": "Universal Store of Value",
				"decimals": 9
			},
			"0x4c47b8d711c0d36db9c719f731abd964cf406acc": {
				"ticker": "ETH",
				"address": "0x4C47B8D711c0d36dB9C719F731ABD964cF406aCc",
				"name": "ETH Nodes",
				"decimals": 9
			},
			"0xe95b7cc8761a6dd2bf57dfb3c6a7d217162e4058": {
				"ticker": "GOSR",
				"address": "0xe95b7cc8761A6dd2bf57DfB3C6a7D217162E4058",
				"name": "Ghost Rise",
				"decimals": 11
			},
			"0xac1869c3458e5d9d20d858de71a279ff16038470": {
				"ticker": "GOL",
				"address": "0xAc1869c3458e5d9d20D858DE71a279FF16038470",
				"name": "Gold Markets",
				"decimals": 12
			},
			"0xf3317f75eea7511e496e7b7e770a5724b6d6c2f5": {
				"ticker": "WHL",
				"address": "0xf3317f75EeA7511E496e7b7E770A5724B6d6c2F5",
				"name": "Whale Treasure",
				"decimals": 9
			},
			"0x9021d9db3aade3643f5c7dd263e23cba7ec8d894": {
				"ticker": "MOON",
				"address": "0x9021D9db3aAdE3643F5c7DD263E23CbA7EC8D894",
				"name": "Moon X",
				"decimals": 9
			},
			"0x20c8aa8b1ddbc0013b50bce06867f76c61f590f5": {
				"ticker": "THN",
				"address": "0x20C8aa8B1dDBC0013b50BcE06867f76C61F590F5",
				"name": "Thunder King",
				"decimals": 10
			},
			"0x3e61b50fc6d68f7513fa8807a1111d05fbc6c4cb": {
				"ticker": "OCHI",
				"address": "0x3E61B50Fc6D68f7513Fa8807a1111d05FBC6c4Cb",
				"name": "OMG Hachi",
				"decimals": 11
			},
			"0x1c20e891bab6b1727d14da358fae2984ed9b59eb": {
				"ticker": "TUSD",
				"address": "0x1C20E891Bab6b1727d14Da358FAe2984Ed9B59EB",
				"name": "TrueUSD",
				"decimals": 18
			},
			"0x6e033b3152300996fb4150d3c10e6313674b0a00": {
				"ticker": "BAD",
				"address": "0x6E033B3152300996fb4150D3c10e6313674B0A00",
				"name": "BadBoys DAO",
				"decimals": 9
			},
			"0xafd70dfd3a50fb86683bb8e05f352e251c14418a": {
				"ticker": "STAR",
				"address": "0xAFd70dfD3a50FB86683BB8E05f352e251c14418A",
				"name": "Fantasy Star",
				"decimals": 12
			},
			"0xf77480e45512e1febb01b88d86b50a7bf8dca8db": {
				"ticker": "WCMP",
				"address": "0xF77480e45512E1FEbB01B88D86B50a7BF8DCa8Db",
				"name": "Wrapped Chimp",
				"decimals": 10
			},
			"0xe4318b636d04368dd67b005d26ccedce865ecbe6": {
				"ticker": "TND",
				"address": "0xe4318b636d04368dD67b005D26CCeDCe865eCBe6",
				"name": "Wrapped Thunder",
				"decimals": 12
			},
			"0xf4fab82dc1b4a019b2efa27f5ac7f93d15572a7b": {
				"ticker": "MDGE",
				"address": "0xf4fAB82dC1b4A019b2EFa27f5aC7F93D15572a7b",
				"name": "Mini Doge",
				"decimals": 10
			},
			"0x181712db5946f1bb3f79477f6244232d51446f0d": {
				"ticker": "TOM",
				"address": "0x181712db5946F1bB3F79477F6244232D51446F0D",
				"name": "Fantom King",
				"decimals": 12
			},
			"0xf874eebd75f4d63e88bdfdf3aeaca02df4a86c56": {
				"ticker": "MKALM",
				"address": "0xF874EEbd75F4D63E88bDFdf3aEacA02df4a86C56",
				"name": "Mock Kalmar Token",
				"decimals": 18
			},
			"0x0e4566b0f2bdc1106aa371fd5f4569a34e2a66b7": {
				"ticker": "ELIG",
				"address": "0x0e4566B0f2BDc1106aa371fd5f4569A34e2A66b7",
				"name": "Elite Gamers",
				"decimals": 9
			},
			"0x6e12128873da723ceb20f073fa8afb9e617af9e4": {
				"ticker": "PNPL",
				"address": "0x6E12128873DA723ceB20f073fa8AFb9E617AF9e4",
				"name": "Fantasy Pineapple",
				"decimals": 9
			},
			"0x73e3636dbc3df211dffaa9023e735cae3f70aeab": {
				"ticker": "ZAP",
				"address": "0x73E3636dbC3DF211DFfaa9023E735cae3F70AEAb",
				"name": "Project Thunder",
				"decimals": 9
			},
			"0x672ed6c6ea15f2ca893c6e444f68e424ade10b51": {
				"ticker": "SHIB",
				"address": "0x672eD6C6EA15F2ca893C6e444F68E424ADE10B51",
				"name": "Shiba Block",
				"decimals": 11
			},
			"0x4e9a38f05c38106c1cf5c145df24959ec50ff70d": {
				"ticker": "PGL",
				"address": "0x4E9A38F05c38106C1cf5c145Df24959ec50ff70D",
				"name": "Pangolin Liquidity",
				"decimals": 18
			},
			"0xd2fce0417cd7c749eb9733ffecfc4c4be126911a": {
				"ticker": "LEO",
				"address": "0xD2FcE0417cD7C749eB9733ffeCFc4C4BE126911a",
				"name": "Leonidas",
				"decimals": 18
			},
			"0x10252c92ea58b608cc05dce51c22605e8e33f523": {
				"ticker": "CPAY",
				"address": "0x10252c92eA58b608Cc05dcE51c22605E8e33f523",
				"name": "Crypto Payment Processor",
				"decimals": 9
			},
			"0xccfa7a07fc8d7be80b7cc83439d51d5903a974fb": {
				"ticker": "CHI",
				"address": "0xcCfA7A07fc8d7bE80B7CC83439D51d5903a974Fb",
				"name": "Chimp Queen",
				"decimals": 9
			},
			"0xd86bb3de0756a71ff21c84f19224f7b83ffe81ed": {
				"ticker": "IME",
				"address": "0xd86bB3de0756a71ff21c84F19224f7B83fFe81eD",
				"name": "Imperium Empires",
				"decimals": 18
			},
			"0x7de817e72c65d3b2ae45affb8aaf80945dfb98ef": {
				"ticker": "FLKR",
				"address": "0x7dE817e72C65D3b2Ae45afFb8Aaf80945DFb98ef",
				"name": "Floki Robot",
				"decimals": 11
			},
			"0xa20afad6b478e170671e98be8eadfd7218f08687": {
				"ticker": "CGI",
				"address": "0xA20afAd6B478e170671e98bE8EadfD7218F08687",
				"name": "Corgi Rise",
				"decimals": 9
			},
			"0xf12f3bcfe7efc37f058e1b5936e02723a96d987f": {
				"ticker": "PDOG",
				"address": "0xf12F3BcfE7Efc37f058e1B5936e02723a96D987F",
				"name": "Project Doge",
				"decimals": 10
			},
			"0x0f86995a3a793aca94c43f12e171ae98062e6185": {
				"ticker": "ZAP",
				"address": "0x0f86995a3a793Aca94c43F12E171aE98062e6185",
				"name": "Thunder Finance",
				"decimals": 11
			},
			"0xb4a19b9e0bfb4464be3dd4df54ef932e315a958f": {
				"ticker": "FLY",
				"address": "0xb4a19B9E0BFb4464be3dD4Df54Ef932e315a958f",
				"name": "Fly Bridge",
				"decimals": 9
			},
			"0xeb6126b58d317489a695c27b88b1ddf1eb1c32f0": {
				"ticker": "MRS",
				"address": "0xeB6126B58D317489A695C27B88B1DDF1Eb1C32F0",
				"name": "Crypto Mars",
				"decimals": 12
			},
			"0x7c08413cbf02202a1c13643db173f2694e0f73f0": {
				"ticker": "MAXI",
				"address": "0x7C08413cbf02202a1c13643dB173f2694e0F73f0",
				"name": "Maximizer",
				"decimals": 9
			},
			"0x1a305118722fcbfba1b8fe690348c55503863906": {
				"ticker": "STAR",
				"address": "0x1A305118722fcbfBA1B8Fe690348C55503863906",
				"name": "Star Chain",
				"decimals": 9
			},
			"0x26c3ee127f5cee38863ab348bfab70f233726f52": {
				"ticker": "GLT",
				"address": "0x26c3EE127F5cEE38863ab348BfAb70f233726F52",
				"name": "OMG Glitch",
				"decimals": 9
			},
			"0x093f8124a841d1452b262ad1587ab7c07a2a35b1": {
				"ticker": "SHBI",
				"address": "0x093F8124a841D1452b262Ad1587ab7C07A2A35B1",
				"name": "Shiba INC",
				"decimals": 12
			},
			"0xa45951515caa0bcae22bcb1f6bb0b23b59863ec6": {
				"ticker": "ZAP",
				"address": "0xA45951515cAA0BCAe22BcB1F6bb0B23b59863ec6",
				"name": "Thunder AVA",
				"decimals": 12
			},
			"0xda045390e531fefa22ac83eae6c465407d7381e9": {
				"ticker": "FKI",
				"address": "0xDA045390e531FEfa22AC83Eae6c465407D7381E9",
				"name": "Floki Infinity",
				"decimals": 10
			},
			"0xcd3c19882b2e0c0b00e4f0b28f4e3c247c48d62b": {
				"ticker": "STRA",
				"address": "0xcD3c19882b2e0c0B00e4F0b28f4E3c247C48D62b",
				"name": "Star AVA",
				"decimals": 10
			},
			"0x8d9e14ec0d5cbbad55c4f3aae66427b8b213a08d": {
				"ticker": "HCH",
				"address": "0x8d9E14ec0D5cBBaD55c4F3aae66427b8B213A08d",
				"name": "Hachi Diamond",
				"decimals": 12
			},
			"0x37a79ab164d8da62b73026f35463d3a095324f9a": {
				"ticker": "KGLD",
				"address": "0x37a79aB164d8dA62B73026F35463D3A095324F9A",
				"name": "King Gold",
				"decimals": 9
			},
			"0x21cbd8508a195271620d194d8d543cb6c07957e4": {
				"ticker": "INFI",
				"address": "0x21cBD8508A195271620d194D8D543cB6c07957e4",
				"name": "InfiniteNodes",
				"decimals": 18
			},
			"0x0741a4472339ccad0d34d486c70992c69a9407c2": {
				"ticker": "WINGS",
				"address": "0x0741a4472339cCaD0D34D486c70992C69a9407c2",
				"name": "Safe Angel",
				"decimals": 12
			},
			"0x7dca8a0f81868dafdbd03409cd328e976bb3947d": {
				"ticker": "MOON",
				"address": "0x7DCa8A0f81868dafDbd03409cD328E976Bb3947D",
				"name": "Little Moon",
				"decimals": 10
			},
			"0xade4a7467a1e570552f8b328024fb5bcc70ab0e9": {
				"ticker": "STR",
				"address": "0xaDe4A7467A1e570552F8b328024fb5bCc70aB0e9",
				"name": "King Star",
				"decimals": 10
			},
			"0x8e47dbe33573c31bfa47ec5e73bdeffc27ed3229": {
				"ticker": "DOGE",
				"address": "0x8E47DBe33573c31bfa47eC5E73BDEfFC27ed3229",
				"name": "Doge INC",
				"decimals": 12
			},
			"0x3a6647f3a0493c38ceff242e4f32f418a931df54": {
				"ticker": "APL",
				"address": "0x3a6647f3A0493C38CefF242E4f32f418a931df54",
				"name": "Mini Apple",
				"decimals": 12
			},
			"0x188697ea8ee1d091dcfd96db8949f970204f9800": {
				"ticker": "OMRS",
				"address": "0x188697eA8eE1d091DCFd96dB8949F970204f9800",
				"name": "OMG Mars",
				"decimals": 11
			},
			"0x613f8694dda37db9caf214495347f10ecd27b534": {
				"ticker": "CGIC",
				"address": "0x613F8694ddA37db9Caf214495347F10ecD27b534",
				"name": "Corgi Coin",
				"decimals": 9
			},
			"0x391eee85ead4227671b2fa4e5fd4a509b4418b17": {
				"ticker": "WINGS",
				"address": "0x391Eee85ead4227671B2Fa4E5FD4A509b4418b17",
				"name": "Sushi Angel",
				"decimals": 9
			},
			"0x034e349c1c80e782339432d9a23bcacf50a22c2d": {
				"ticker": "BLIZZARD",
				"address": "0x034e349c1c80e782339432D9A23BcacF50A22c2D",
				"name": "Blizzard Finance",
				"decimals": 18
			},
			"0x856614f61b21ceddd29ade77b266e52fb50f5fca": {
				"ticker": "MOON",
				"address": "0x856614F61b21CeddD29ade77B266e52fb50f5Fca",
				"name": "Moon SV",
				"decimals": 10
			},
			"0x82a92b89a4d13e91083262f9d0890b7587164db4": {
				"ticker": "QYUM",
				"address": "0x82a92b89a4D13E91083262f9D0890B7587164dB4",
				"name": "Queen Cake",
				"decimals": 10
			},
			"0x637afeff75ca669ff92e4570b14d6399a658902f": {
				"ticker": "COOK",
				"address": "0x637afeff75ca669fF92e4570B14D6399A658902f",
				"name": "Poly-Peg COOK",
				"decimals": 18
			},
			"0xd1674acc98b4ed4b30b6328742b753bf959ff737": {
				"ticker": "CHI",
				"address": "0xD1674aCc98B4Ed4b30B6328742B753bf959FF737",
				"name": "Chimp Inu",
				"decimals": 12
			},
			"0x0f4d5fa48202b3a154afdeed2eb8d9a48c7eae26": {
				"ticker": "AKITA",
				"address": "0x0F4D5fa48202B3a154afdeED2Eb8d9a48C7Eae26",
				"name": "Project Akita",
				"decimals": 9
			},
			"0xa98483926e965948c615379ff685fdb31cebd885": {
				"ticker": "MOON",
				"address": "0xA98483926e965948C615379ff685fdb31CEBD885",
				"name": "Mini Moon",
				"decimals": 12
			},
			"0x7a29d243952daa847e9c5ab2e603208c89e61430": {
				"ticker": "MAPL",
				"address": "0x7A29d243952dAA847e9c5aB2E603208C89e61430",
				"name": "Meta Apple",
				"decimals": 10
			},
			"0x9863a5022b3844f92fa2315844b60a4b5e9a82e2": {
				"ticker": "STR",
				"address": "0x9863a5022B3844f92FA2315844b60A4B5E9a82e2",
				"name": "Safe Star",
				"decimals": 11
			},
			"0xe4294b225b3cd07a85eaf075bc550a11ed16b548": {
				"ticker": "CHIMP",
				"address": "0xE4294b225B3CD07A85Eaf075bc550a11ED16b548",
				"name": "Chimp Cash",
				"decimals": 9
			},
			"0x6389ab2e4c6486b9581d7071a47671307565721d": {
				"ticker": "ELND",
				"address": "0x6389AB2e4C6486B9581d7071A47671307565721d",
				"name": "Elon Diamond",
				"decimals": 11
			},
			"0x16237e89ed8fbeb5bff65f4ae90d5d2d99be12ea": {
				"ticker": "AngryFrog",
				"address": "0x16237E89Ed8FbEB5bFf65f4AE90D5d2D99BE12eA",
				"name": "Daniele and Sifu lied to Frog Nation. Every Wonderland holders deserve to be repaid by Treasury. Sifu  is the Co-founder of QuadrigaCX, Michael Patryn. Support to raise awareness and legal expenses.",
				"decimals": 18
			},
			"0x885e14eb0634971fcf5d6a6ca9983482b27d88a2": {
				"ticker": "GOS",
				"address": "0x885E14EB0634971fCf5d6A6cA9983482B27d88A2",
				"name": "Ghost NET",
				"decimals": 11
			},
			"0xd19a6412c4dd29c8a14be5e91358432fe913f827": {
				"ticker": "APE",
				"address": "0xd19A6412C4Dd29C8a14BE5e91358432fE913f827",
				"name": "Ape Inu",
				"decimals": 10
			},
			"0xf3821e7a06f78ec1dfd88473cf3c4b41badb1a49": {
				"ticker": "GHS",
				"address": "0xf3821e7A06f78eC1dFd88473Cf3C4B41baDb1a49",
				"name": "Ghost Rocket",
				"decimals": 10
			},
			"0xdd8d16360d8ce2b7e89532ba6f9efd428babb3cb": {
				"ticker": "CORK",
				"address": "0xDD8d16360d8ce2b7E89532bA6F9Efd428bAbb3CB",
				"name": "Corkscrew",
				"decimals": 18
			},
			"0xcf22eb296b478a831cc7cd9552b91766d81b61c1": {
				"ticker": "APLB",
				"address": "0xcF22eB296B478a831Cc7Cd9552B91766D81B61C1",
				"name": "Apple BTC",
				"decimals": 10
			},
			"0x5306377ba6178d304975364d964ea952a062e065": {
				"ticker": "GTH",
				"address": "0x5306377BA6178D304975364D964Ea952A062E065",
				"name": "King Glitch",
				"decimals": 11
			},
			"0x449674b82f05d498e126dd6615a1057a9c088f2c": {
				"ticker": "LOST",
				"address": "0x449674B82F05d498E126Dd6615a1057A9c088f2C",
				"name": "LostToken",
				"decimals": 18
			},
			"0x387a2d7147a9ac9c5dc7cf4dd1791878983887a0": {
				"ticker": "GHSG",
				"address": "0x387a2D7147a9AC9c5Dc7cF4dD1791878983887A0",
				"name": "Ghost Gaming",
				"decimals": 12
			},
			"0x2f77e2c5ebd3b4d38cea7e28345b4cda4ae2cdb7": {
				"ticker": "GOL",
				"address": "0x2F77E2c5ebD3b4d38cea7e28345B4cda4ae2Cdb7",
				"name": "Crypto Gold",
				"decimals": 9
			},
			"0x4643ac9346f33fefb5725612219f577a175559bd": {
				"ticker": "MCHI",
				"address": "0x4643ac9346F33fEFB5725612219F577a175559bD",
				"name": "Meta Hachi",
				"decimals": 9
			},
			"0xbbff614d2f78053935f1608a5a2fd6564e4d376d": {
				"ticker": "PARR",
				"address": "0xbbFf614D2F78053935f1608a5a2fD6564E4D376D",
				"name": "ParrotDao",
				"decimals": 18
			},
			"0xb8ba82525e69860b266b13ec33108ad84c330186": {
				"ticker": "SKY",
				"address": "0xB8ba82525e69860B266B13EC33108AD84c330186",
				"name": "SKY Nodes",
				"decimals": 18
			},
			"0x40d360c4a517ed7206a1f732b203fc10fe53a9e4": {
				"ticker": "SAPL",
				"address": "0x40D360c4A517Ed7206a1f732B203FC10FE53A9E4",
				"name": "Safe Apple",
				"decimals": 9
			},
			"0x5bd1535df7d78bfea087e858b4faeb2822affeb6": {
				"ticker": "CORGI",
				"address": "0x5bd1535df7d78bFEa087E858B4faeb2822affeb6",
				"name": "Zen Corgi",
				"decimals": 10
			},
			"0x41f1f453162f4d382b1cc454d874b95e61711f75": {
				"ticker": "Crabapple",
				"address": "0x41f1f453162F4d382b1cc454d874b95E61711f75",
				"name": "Crabapple",
				"decimals": 18
			},
			"0x19071ac897e6d94670252eaf1253239877016e8a": {
				"ticker": "CGI",
				"address": "0x19071ac897e6d94670252EaF1253239877016e8A",
				"name": "Project Corgi",
				"decimals": 10
			},
			"0x3fe2169f11f9ebab2f0ef9b3c5d42ed04a51946f": {
				"ticker": "ELNQ",
				"address": "0x3FE2169F11F9EbAb2F0Ef9b3C5d42Ed04a51946f",
				"name": "Elon Queen",
				"decimals": 11
			},
			"0x6ffeed47fd31ca6c54f45f58904a941974fda213": {
				"ticker": "WHLB",
				"address": "0x6fFeed47fd31Ca6c54F45f58904a941974Fda213",
				"name": "Whale BTC",
				"decimals": 11
			},
			"0x8f63050417a15b798798a510c1a1b163f59c421e": {
				"ticker": "PHOENIX",
				"address": "0x8F63050417a15B798798A510C1A1b163f59C421e",
				"name": "phoenix.finance",
				"decimals": 18
			},
			"0x065dab4c66ccf97b55ce50c9791dd220f26d0ce8": {
				"ticker": "LCMP",
				"address": "0x065daB4C66cCF97b55CE50C9791Dd220F26D0CE8",
				"name": "Little Chimp",
				"decimals": 10
			},
			"0x2b2144a529a6685efa1b89a7abb1ec7b15548800": {
				"ticker": "AKTF",
				"address": "0x2b2144a529A6685EfA1B89a7abb1ec7B15548800",
				"name": "Akita Finance",
				"decimals": 11
			},
			"0xba51a6e1866c8427f6b6737e158427859d521068": {
				"ticker": "WHALE",
				"address": "0xBa51a6e1866c8427F6b6737E158427859d521068",
				"name": "Super Whale",
				"decimals": 11
			},
			"0xeac5bea65a424d26af03c334bcfe4a486bb3608d": {
				"ticker": "test22",
				"address": "0xeac5BeA65A424d26Af03C334bCFe4a486bB3608D",
				"name": "test22",
				"decimals": 18
			},
			"0x6d6056159f597efc32b0d8ba81fc17564e554afa": {
				"ticker": "MAPE",
				"address": "0x6d6056159f597efc32b0d8Ba81Fc17564e554AFa",
				"name": "Meta Ape",
				"decimals": 10
			},
			"0xd2088823b533957d9ad34a9adbb5af2330236743": {
				"ticker": "STAR",
				"address": "0xD2088823b533957D9aD34a9adBB5Af2330236743",
				"name": "OMG Star",
				"decimals": 10
			},
			"0x16c0f7e3411c76597c5f57fefebefb38af8bd9e1": {
				"ticker": "APL",
				"address": "0x16c0f7E3411c76597C5F57FefEbEfb38af8BD9E1",
				"name": "Apple Infinity",
				"decimals": 12
			},
			"0x77502ea5489e7e681c931fa8fdf49253b55f3fe1": {
				"ticker": "DOGE",
				"address": "0x77502Ea5489e7e681c931fA8FDf49253b55F3fe1",
				"name": "Super Doge",
				"decimals": 9
			},
			"0xcb284d5d0f1a36cca42d3e6a6b93fdf02e671b42": {
				"ticker": "CORK",
				"address": "0xCB284d5D0F1A36CcA42D3E6a6B93fDf02E671B42",
				"name": "Corkscrew",
				"decimals": 18
			},
			"0x4bf0b1d59a7d418379adaed6effd420fa1044406": {
				"ticker": "APL",
				"address": "0x4BF0b1d59a7d418379AdAED6efFd420fa1044406",
				"name": "Safe Apple",
				"decimals": 10
			},
			"0x47e452deff9eb7db6033b995778fc4cb8c47d626": {
				"ticker": "CMPD",
				"address": "0x47E452DEFf9EB7dB6033B995778fc4CB8C47d626",
				"name": "Chimp Diamond",
				"decimals": 12
			},
			"0xc0d2da43f479b7aab4d816cd533731599be97c16": {
				"ticker": "THN",
				"address": "0xc0d2dA43f479B7aAB4D816Cd533731599Be97c16",
				"name": "Crypto Thunder",
				"decimals": 9
			},
			"0xa71325e8980c450cb3e211e7a84e7a7de3f5b7ba": {
				"ticker": "FIRN",
				"address": "0xa71325E8980c450Cb3E211E7a84E7A7de3F5B7BA",
				"name": "Firn Token",
				"decimals": 18
			},
			"0x01f3b72283a33ba2d8a4fbe0038d7a4d27acd07a": {
				"ticker": "CAKE",
				"address": "0x01f3B72283A33ba2d8a4FBE0038D7A4d27AcD07a",
				"name": "Wrapped Cake",
				"decimals": 11
			},
			"0xf3592df8f676980db7e5f97ae9b1d04c1773ef65": {
				"ticker": "MOON",
				"address": "0xF3592df8f676980dB7E5f97AE9B1d04C1773eF65",
				"name": "Moon Fund",
				"decimals": 9
			},
			"0x8a1703b7058e585e28cf217d6a74aaf966218333": {
				"ticker": "APLC",
				"address": "0x8a1703B7058e585e28CF217D6A74aaf966218333",
				"name": "Pineapple Chain",
				"decimals": 9
			},
			"0x78598d02d5e4d35fd014d361ab62603651e44d47": {
				"ticker": "SSHARE",
				"address": "0x78598d02d5e4D35fd014d361AB62603651E44D47",
				"name": "Super Share",
				"decimals": 18
			},
			"0x6ca7429fe4b01c81120e629254feb825631fdab9": {
				"ticker": "YUM",
				"address": "0x6cA7429fe4b01C81120e629254feB825631FDab9",
				"name": "Little Cake",
				"decimals": 12
			},
			"0x4dde340541c8681cf7dcfcf7ab40ee21f2717201": {
				"ticker": "FTM",
				"address": "0x4DDe340541c8681cf7dCFCf7AB40eE21F2717201",
				"name": "Fantom Inu",
				"decimals": 12
			},
			"0x098b353ac814cfd7b8396aa73e40eac990400d79": {
				"ticker": "SANG",
				"address": "0x098B353Ac814CFD7b8396aA73e40EAc990400d79",
				"name": "Super Angel",
				"decimals": 12
			},
			"0x9504ecf93be905be62834d8ae3fd8357a1146745": {
				"ticker": "DOUB",
				"address": "0x9504ecf93BE905BE62834d8ae3fD8357a1146745",
				"name": "Doubloon",
				"decimals": 18
			},
			"0x06f7d42e6783d9fcfd7b2bb971072873b50eddf6": {
				"ticker": "PPLE",
				"address": "0x06f7d42E6783d9FcFd7b2bB971072873b50EDDF6",
				"name": "Pineapple Ledger",
				"decimals": 12
			},
			"0xb08208d175e47dc483adfdacbfb352a46c2a5e7f": {
				"ticker": "CMP",
				"address": "0xB08208d175E47Dc483AdFdacbFb352a46c2A5e7F",
				"name": "Zen Chimp",
				"decimals": 10
			},
			"0x3d182577a34d1400b88544ea7d4b293340cb2386": {
				"ticker": "FKI",
				"address": "0x3D182577A34D1400b88544ea7D4b293340Cb2386",
				"name": "Mini Floki",
				"decimals": 9
			},
			"0x1a8d61092c154f3d73866acc93b4092d8a7f7029": {
				"ticker": "APLI",
				"address": "0x1a8d61092c154F3d73866acC93b4092D8A7F7029",
				"name": "Pineapple Inu",
				"decimals": 12
			},
			"0xb516fb832f1bbdabd1f8ee5fe35ce2c3277ae5ea": {
				"ticker": "WINGS",
				"address": "0xb516FB832F1BbDABD1f8Ee5Fe35Ce2C3277AE5ea",
				"name": "Sushi Angel",
				"decimals": 10
			},
			"0xee9801669c6138e84bd50deb500827b776777d28": {
				"ticker": "O3",
				"address": "0xEe9801669C6138E84bD50dEB500827b776777d28",
				"name": "O3 Swap Token",
				"decimals": 18
			},
			"0xed6cbea16d30d6a6ccc098ea66c905a66eb5916a": {
				"ticker": "APE",
				"address": "0xED6CbEA16d30D6A6CCc098EA66C905a66Eb5916A",
				"name": "Ape Storm",
				"decimals": 9
			},
			"0xadb75387821eca5a14903ef471c1705e9c90dc76": {
				"ticker": "ZSHARE",
				"address": "0xadB75387821EcA5a14903ef471C1705e9C90dC76",
				"name": "Zilla Shares",
				"decimals": 18
			},
			"0x861f748621ea676a27dc920c817df8e8f80c3667": {
				"ticker": "YUM",
				"address": "0x861F748621ea676a27DC920C817Df8E8F80c3667",
				"name": "Cake Robot",
				"decimals": 9
			},
			"0x0b4ae6432762def5d8ac4787d60005787365d630": {
				"ticker": "AKT",
				"address": "0x0B4aE6432762dEF5d8AC4787D60005787365d630",
				"name": "Project Akita",
				"decimals": 10
			},
			"0x74f457aa55f81c7b896a9555a35b22606e64dd85": {
				"ticker": "DGVI",
				"address": "0x74f457Aa55f81C7b896a9555a35B22606E64dd85",
				"name": "DoggiVerse",
				"decimals": 18
			},
			"0xf7a651be62ec908a204ab13fa1e011e0ce848136": {
				"ticker": "RAC",
				"address": "0xF7A651BE62Ec908a204aB13FA1e011E0Ce848136",
				"name": "Rich Apes Club",
				"decimals": 9
			},
			"0xb8efdd45f6fd658af8c17cb441e867033acd8d1d": {
				"ticker": "PNPL",
				"address": "0xB8eFdD45f6FD658AF8C17cb441e867033acD8D1D",
				"name": "Pineapple Farm",
				"decimals": 9
			},
			"0x9e20af05ab5fed467dfdd5bb5752f7d5410c832e": {
				"ticker": "PXT2",
				"address": "0x9e20Af05AB5FED467dFDd5bb5752F7d5410C832e",
				"name": "ProjectX2",
				"decimals": 18
			},
			"0x216fba5dcabc2ad82150b33df7dc12b05160020d": {
				"ticker": "CMP",
				"address": "0x216fba5dcAbc2ad82150b33df7dC12B05160020d",
				"name": "Chimp Cash",
				"decimals": 12
			},
			"0xc166c2f17907938ea09bb1f989e9325ed7b90da1": {
				"ticker": "MF",
				"address": "0xC166C2f17907938Ea09bB1f989E9325ed7b90dA1",
				"name": "MountainFinances",
				"decimals": 18
			},
			"0x702590d885339c98487b6df3fc998d5c021adc8d": {
				"ticker": "STR",
				"address": "0x702590d885339c98487B6Df3fC998d5c021ADC8d",
				"name": "Star Chain",
				"decimals": 9
			},
			"0x6240cb2a58a0e151fe953bcc7cfa89aab2779476": {
				"ticker": "GLDT",
				"address": "0x6240cb2a58a0e151FE953BcC7CfA89aAB2779476",
				"name": "Gold Treasure",
				"decimals": 9
			},
			"0xc2ee64e90b1ab2a58fa724bd8eb71bc733003789": {
				"ticker": "KGCH",
				"address": "0xC2Ee64e90B1ab2a58FA724bd8Eb71Bc733003789",
				"name": "King Glitch",
				"decimals": 10
			},
			"0x07a031d41ee5655fe1b109e3caf8516807c58d98": {
				"ticker": "SHBP",
				"address": "0x07a031d41Ee5655fe1b109E3caF8516807c58d98",
				"name": "Shiba Project",
				"decimals": 11
			},
			"0x5875c368cddd5fb9bf2f410666ca5aad236dabd4": {
				"ticker": "PGL",
				"address": "0x5875c368Cddd5FB9Bf2f410666ca5aad236DAbD4",
				"name": "Pangolin Liquidity",
				"decimals": 18
			},
			"0xadbaf61faed8fe7ee473de686835e33cfbb15bd8": {
				"ticker": "PPLE",
				"address": "0xadbaF61faeD8FE7EE473de686835E33Cfbb15Bd8",
				"name": "Pineapple Rocket",
				"decimals": 12
			},
			"0x6d787b92a8e7c0475a57d6fd878a80c0992e34ce": {
				"ticker": "PKV",
				"address": "0x6d787B92A8E7c0475A57d6Fd878A80c0992e34CE",
				"name": "PKV",
				"decimals": 18
			},
			"0x66e25ce0c99f5fc3cacad3a1b88479f4984f91eb": {
				"ticker": "CRND",
				"address": "0x66e25Ce0C99f5fC3CACad3A1b88479f4984F91eB",
				"name": "Crypto Nodes",
				"decimals": 18
			},
			"0x4a4f77d74cf5fd4ea4ab71ba79988c055a5c27b2": {
				"ticker": "LESS",
				"address": "0x4a4f77d74cf5Fd4ea4AB71Ba79988c055A5C27b2",
				"name": "LessToken",
				"decimals": 18
			},
			"0x20ac72c00da1f6670e79cc729d896afae280797a": {
				"ticker": "PDAD",
				"address": "0x20Ac72c00DA1F6670e79CC729D896aFAe280797a",
				"name": "Panda Diamond",
				"decimals": 10
			},
			"0xe895962c2d94e32ec9c5219d37ee63243e34c5dc": {
				"ticker": "APE",
				"address": "0xE895962C2d94e32EC9c5219d37ee63243E34C5Dc",
				"name": "Crypto Ape",
				"decimals": 12
			},
			"0x6d2f5dbf3a7396fce32cfe406aef7a8aff812fbb": {
				"ticker": "ASTRO",
				"address": "0x6d2f5dBf3a7396FCe32CfE406Aef7a8AFF812Fbb",
				"name": "100 Days Ventures - v1.1",
				"decimals": 18
			},
			"0x3ef7e5a8798a4e2b6bd61e673653eede58cd4bc0": {
				"ticker": "STAR",
				"address": "0x3eF7E5A8798a4e2b6BD61E673653EeDe58CD4bc0",
				"name": "Star DAO",
				"decimals": 12
			},
			"0x9032a61ef38be9d30870bccf22c6e2ced3d00f51": {
				"ticker": "TAPL",
				"address": "0x9032a61Ef38bE9d30870BCcf22C6e2ceD3d00F51",
				"name": "Treasure Pineapple",
				"decimals": 11
			},
			"0xf06f9d4aac556bdc731f77a1fd9c330de017b4da": {
				"ticker": "SBAR",
				"address": "0xF06F9d4AAc556bdC731F77a1Fd9C330de017B4Da",
				"name": "Shiba Rise",
				"decimals": 9
			},
			"0x7b2b702706d9b361dfe3f00bd138c0cfda7fb2cf": {
				"ticker": "PLN",
				"address": "0x7b2B702706D9b361dfE3f00bD138C0CFDA7FB2Cf",
				"name": "Pollen",
				"decimals": 18
			},
			"0x47c67f281fec22fa04057540f25c5fde36bd6a0e": {
				"ticker": "GOS",
				"address": "0x47C67f281feC22fA04057540F25C5FdE36bd6A0E",
				"name": "Queen Ghost",
				"decimals": 9
			},
			"0x1e09d3e0957bb4d5ae52107954e2a21325405303": {
				"ticker": "CORGI",
				"address": "0x1e09d3e0957bb4D5ae52107954e2a21325405303",
				"name": "Sushi Corgi",
				"decimals": 11
			},
			"0x9b1b534dfa49369fc92d7c720d05fccd6210678d": {
				"ticker": "SBA",
				"address": "0x9b1b534DfA49369FC92d7C720D05fccD6210678d",
				"name": "Little Shiba",
				"decimals": 11
			},
			"0x45938cec6e206db88bc530a341b594a80bdde6b3": {
				"ticker": "ORBIT",
				"address": "0x45938cEc6e206Db88bC530a341B594a80bdDE6B3",
				"name": "Europa Token",
				"decimals": 18
			},
			"0x4412f46e4a7f6c6433026b86f607242062a1b943": {
				"ticker": "WDOG",
				"address": "0x4412f46E4a7f6c6433026B86f607242062a1b943",
				"name": "Wrapped Doge",
				"decimals": 11
			},
			"0x72d251d62beb31bfcbdfd31a021bcbeeb321b4c0": {
				"ticker": "APL",
				"address": "0x72D251D62beb31bFcBDfD31a021bCbEEb321B4C0",
				"name": "Apple Cash",
				"decimals": 11
			},
			"0x018f79403a33bd633d677f89d09982b34c231ee6": {
				"ticker": "SHIB",
				"address": "0x018f79403a33bD633D677f89D09982B34c231Ee6",
				"name": "Shiba Queen",
				"decimals": 10
			},
			"0xcaa1560ea4b91540bfdb94d4a04e98bccaa44bf8": {
				"ticker": "BSN",
				"address": "0xCaA1560EA4b91540bfdB94d4a04E98BCCaA44bF8",
				"name": "Black Shark Nodes",
				"decimals": 18
			},
			"0x1b1b21a0f767e198d61ee80418f98d5d822093f1": {
				"ticker": "CAKE",
				"address": "0x1B1b21a0F767e198D61EE80418f98d5d822093F1",
				"name": "Crypto Cake",
				"decimals": 12
			},
			"0x88675cc83dcfa58da4c32681f5485e0fbd8dad87": {
				"ticker": "AKITA",
				"address": "0x88675cC83dCfa58dA4C32681F5485E0fBd8DAD87",
				"name": "Akita Cash",
				"decimals": 12
			},
			"0xc1171b333f6fd774805c8c0e93a4655166628964": {
				"ticker": "ELN",
				"address": "0xC1171b333f6fd774805C8c0E93a4655166628964",
				"name": "Elon Protocol",
				"decimals": 11
			},
			"0x57d5029a27880e3693e29551558f7ba516f3ac86": {
				"ticker": "APL",
				"address": "0x57D5029A27880e3693e29551558f7ba516F3Ac86",
				"name": "Apple Inu",
				"decimals": 9
			},
			"0xf58d335858cdc567b69a0b39f5037065a0628bbc": {
				"ticker": "BCS",
				"address": "0xF58d335858cDc567B69a0B39F5037065a0628bBC",
				"name": "Black Cat Sheriff",
				"decimals": 18
			},
			"0x77d67c02b10ebf4505e3ef1eb200640f27bd47f4": {
				"ticker": "APE",
				"address": "0x77D67C02B10EBF4505E3Ef1eB200640F27bD47F4",
				"name": "Baby Ape",
				"decimals": 12
			},
			"0x93c8df8ac620961e40da962654bfc28cd7972a78": {
				"ticker": "CHI",
				"address": "0x93c8dF8Ac620961E40dA962654BFC28cd7972A78",
				"name": "Chimp Markets",
				"decimals": 9
			},
			"0xbb4422cba1d4246b6fd23e967be2aff0c2992525": {
				"ticker": "WHALE",
				"address": "0xBB4422cBa1d4246B6fD23E967Be2aff0c2992525",
				"name": "Whale AI",
				"decimals": 10
			},
			"0x25c395adf9ded703189ad6f49704dba6062b4049": {
				"ticker": "SPAN",
				"address": "0x25C395AdF9DeD703189AD6F49704DBa6062B4049",
				"name": "Super Panda",
				"decimals": 9
			},
			"0xfea82a0054c395af338d8df8fe6e0e387b4eec2e": {
				"ticker": "SWHL",
				"address": "0xFEA82a0054C395AF338D8DF8FE6e0e387b4eec2e",
				"name": "Safe Whale",
				"decimals": 11
			},
			"0xdf17d39a677eb08447044250a16aad009910b228": {
				"ticker": "DOGE",
				"address": "0xdf17d39A677EB08447044250A16aaD009910B228",
				"name": "Doge AVA",
				"decimals": 11
			},
			"0xf53cc73cf2638b330e62b094efedf0a7d9ee1b53": {
				"ticker": "LAVA",
				"address": "0xf53cC73cF2638B330e62B094efeDf0a7d9eE1b53",
				"name": "LavaToken",
				"decimals": 18
			},
			"0x725167ea39d84a24bcc897add486601253c5a0de": {
				"ticker": "QHCI",
				"address": "0x725167EA39d84A24BcC897aDd486601253c5A0DE",
				"name": "Queen Hachi",
				"decimals": 11
			},
			"0xb00015eaa4602a7c42394cf4b143073d1daba5f6": {
				"ticker": "SHIB",
				"address": "0xb00015eAa4602A7C42394CF4B143073D1dAbA5F6",
				"name": "Mini Shiba",
				"decimals": 12
			},
			"0x9f285507ea5b4f33822ca7abb5ec8953ce37a645": {
				"ticker": "DEG",
				"address": "0x9f285507Ea5B4F33822CA7aBb5EC8953ce37A645",
				"name": "DegisToken",
				"decimals": 18
			},
			"0x4e18167697a263900956b85b3d1e90308ba27b1c": {
				"ticker": "APE",
				"address": "0x4E18167697a263900956B85B3D1E90308Ba27B1c",
				"name": "Project Ape",
				"decimals": 12
			},
			"0x87ac26925a3f3f7f9f0f709495df3b724e33954a": {
				"ticker": "CGID",
				"address": "0x87AC26925a3F3f7F9F0f709495dF3B724E33954a",
				"name": "Corgi Dapp",
				"decimals": 9
			},
			"0x5fa462b937a3b5fbe553f69f24f9b481553d328f": {
				"ticker": "MRS",
				"address": "0x5fA462B937a3b5Fbe553F69f24f9B481553d328f",
				"name": "Little Mars",
				"decimals": 9
			},
			"0x755ce29b78d80f0fe1077d44c3cf68b21f9244c0": {
				"ticker": "SHIB",
				"address": "0x755Ce29b78d80f0FE1077d44c3Cf68b21F9244C0",
				"name": "Shiba Rocket",
				"decimals": 10
			},
			"0x9df2d477369f7f98d6b31ae0986fff5133c5afea": {
				"ticker": "GLT",
				"address": "0x9df2d477369f7f98D6B31ae0986ffF5133c5afEa",
				"name": "Meta Glitch",
				"decimals": 9
			},
			"0x9a86cd1082b799ef5eaa81d46ac1cf43f274452b": {
				"ticker": "HWATERr1",
				"address": "0x9A86cD1082b799eF5eAA81d46aC1Cf43F274452B",
				"name": "HashWater(r1)",
				"decimals": 18
			},
			"0xe5e2a02bb8e98a5079777ef572c4504a54c3cfa7": {
				"ticker": "BALLIN",
				"address": "0xe5E2a02Bb8e98a5079777eF572C4504a54c3cFA7",
				"name": "Ballin Project",
				"decimals": 9
			},
			"0x75287a47bd8b2cc357128800d0415acde9aaefa8": {
				"ticker": "$WHISKEY",
				"address": "0x75287a47bd8b2cC357128800D0415acDE9AaeFA8",
				"name": "HennyDAO",
				"decimals": 18
			},
			"0xc609364582d4e9ad09d240489b2a5a57486a51fe": {
				"ticker": "MOON",
				"address": "0xc609364582D4E9ad09D240489b2A5A57486a51fe",
				"name": "Moon Ledger",
				"decimals": 9
			},
			"0x126612bd66e9074545b6256da518375d7f7f981d": {
				"ticker": "CAKE",
				"address": "0x126612bD66E9074545B6256DA518375D7F7F981d",
				"name": "Cake Farm",
				"decimals": 9
			},
			"0xb16abb36335fc3a73a712a62d5485528ee88c86f": {
				"ticker": "ELON",
				"address": "0xb16Abb36335fc3a73a712a62D5485528Ee88c86f",
				"name": "Elon AVA",
				"decimals": 12
			},
			"0xe06fc0f559b9a8e61943898b6099220c8210725e": {
				"ticker": "TRFC",
				"address": "0xE06Fc0f559B9a8E61943898b6099220C8210725e",
				"name": "Terrific Protocol TRFC Token",
				"decimals": 18
			},
			"0x97d367a5f900f5c9db4370d0d801fc52332244c7": {
				"ticker": "STATIK",
				"address": "0x97d367A5f900F5c9dB4370D0D801Fc52332244C7",
				"name": "STATIK",
				"decimals": 18
			},
			"0x4d35a5bf9fe429d735dd6a4a7081b31939a3cb09": {
				"ticker": "SLOT",
				"address": "0x4d35A5bf9fe429D735DD6a4a7081B31939a3cB09",
				"name": "Snowtomb Lot",
				"decimals": 9
			},
			"0xab2f92d6044c4703954f7ed3ec3d2e76a6940c32": {
				"ticker": "GLD",
				"address": "0xAb2F92D6044c4703954f7ed3eC3d2e76A6940C32",
				"name": "Gold AI",
				"decimals": 11
			},
			"0x31531405f805051ba7a8f86f69306cf622130b0e": {
				"ticker": "ANG",
				"address": "0x31531405F805051ba7A8f86f69306Cf622130B0e",
				"name": "Angel Block",
				"decimals": 12
			},
			"0x0478b9ba9a8d50fb9b846cf2eab171a0c4bc992a": {
				"ticker": "APL",
				"address": "0x0478B9ba9A8D50FB9b846CF2EAb171a0c4bc992a",
				"name": "Baby Apple",
				"decimals": 11
			},
			"0xf9f2002bab95ca5ee7c33ba11bc70c9f2a3409a2": {
				"ticker": "CITRUS",
				"address": "0xf9F2002bab95cA5ee7c33Ba11Bc70c9F2a3409A2",
				"name": "Citrus Finance",
				"decimals": 18
			},
			"0x32591ed516aa6f67e338cb9f88989f3fb91a860b": {
				"ticker": "NIFTYX",
				"address": "0x32591ED516aA6F67E338Cb9f88989f3fb91A860B",
				"name": "Niftyx",
				"decimals": 18
			},
			"0x5fb5619b0417361a3eea45f6511114e3ca3fb2b7": {
				"ticker": "GMOON",
				"address": "0x5FB5619b0417361a3eEA45f6511114e3Ca3fb2B7",
				"name": " Ghost Moon Coin",
				"decimals": 6
			},
			"0xd783076c56ae529f4e189e740ba7aa3c4b4b6bf6": {
				"ticker": "APLA",
				"address": "0xd783076C56aE529F4e189e740bA7AA3c4B4b6bF6",
				"name": "Pineapple AVAX",
				"decimals": 11
			},
			"0xe38991276ff1e592485db268eea3f6b809292e6b": {
				"ticker": "PAN",
				"address": "0xe38991276FF1E592485Db268eeA3f6b809292e6b",
				"name": "Panda Robot",
				"decimals": 9
			},
			"0x2f72bac338d8f79c0866974059114f2c6380fb49": {
				"ticker": "SPG",
				"address": "0x2F72BAc338d8F79c0866974059114f2c6380fB49",
				"name": "SPRING",
				"decimals": 6
			},
			"0x57dd62dc89ae8577f3b50272920de76550875147": {
				"ticker": "MTHN",
				"address": "0x57dD62DC89aE8577F3b50272920De76550875147",
				"name": "Mini Thunder",
				"decimals": 12
			},
			"0xb520b4cf0cd863c7952266903e48271e56b87bd1": {
				"ticker": "AVAXMINER",
				"address": "0xB520B4Cf0cD863C7952266903E48271e56B87bD1",
				"name": "AvaxMiner",
				"decimals": 18
			},
			"0xa8bb4eba50356ec118d061d7a936b146f8c04799": {
				"ticker": "PIN",
				"address": "0xa8bb4ebA50356eC118d061d7a936b146F8C04799",
				"name": "Pineapple Swap",
				"decimals": 12
			},
			"0xd63e149ace569fc44465f213e222b3d3405cc3b8": {
				"ticker": "COOKIES",
				"address": "0xd63E149aCE569FC44465f213e222b3D3405cC3b8",
				"name": "Cookies",
				"decimals": 9
			},
			"0xd5ff9c46e52616117e430c8bb176abaccbbae463": {
				"ticker": "FLOI",
				"address": "0xd5Ff9c46e52616117e430C8BB176abacCBbAE463",
				"name": "Floki Inu",
				"decimals": 9
			},
			"0xa08bad72df1d1ade11c80e10f744a572457e7cd7": {
				"ticker": "HCH",
				"address": "0xA08BAd72Df1d1aDe11c80E10F744A572457E7CD7",
				"name": "Treasure Hachi",
				"decimals": 11
			},
			"0x8ad9ddb32aa7c184368756a4fda035aabc40b302": {
				"ticker": "FLOKI",
				"address": "0x8Ad9Ddb32Aa7C184368756A4FDa035AAbc40b302",
				"name": "Meta Floki",
				"decimals": 9
			},
			"0x5676a255ec5ad888772f6abbcd391c1e1ebef26b": {
				"ticker": "CHI",
				"address": "0x5676A255ec5ad888772F6ABBCd391C1e1EBEf26B",
				"name": "Exo Hachi",
				"decimals": 11
			},
			"0x5b2cf2d63299a473293577cf22a5241fb0e8e1b2": {
				"ticker": "BEARSHARE",
				"address": "0x5B2Cf2d63299a473293577CF22a5241Fb0e8e1b2",
				"name": "BEARSHARES Token",
				"decimals": 18
			},
			"0xdbb8558b640e10a05416d9e4b221531de94a604c": {
				"ticker": "GLTCH",
				"address": "0xDbB8558b640E10A05416d9E4B221531de94A604C",
				"name": "Glitch Treasure",
				"decimals": 10
			},
			"0xce1bffbd5374dac86a2893119683f4911a2f7814": {
				"ticker": "SPELL",
				"address": "0xCE1bFFBD5374Dac86a2893119683F4911a2F7814",
				"name": "Spell Token",
				"decimals": 18
			},
			"0xf4e0b2dbfac42672a0e87f086710c2649aee80b6": {
				"ticker": "BRIBE",
				"address": "0xf4e0B2dBfAC42672A0e87f086710c2649aeE80B6",
				"name": "BRIBE",
				"decimals": 18
			},
			"0x042113ff59d50521374162b555b44f2b68d51554": {
				"ticker": "RND",
				"address": "0x042113fF59d50521374162B555B44f2b68D51554",
				"name": "Redlight Node District",
				"decimals": 18
			},
			"0xfe47d7e01d6709f4cbd00b6ab29b6cd55c9a995a": {
				"ticker": "wraither",
				"address": "0xFE47d7E01d6709f4cBD00B6AB29b6Cd55c9a995a",
				"name": "wraither",
				"decimals": 18
			},
			"0xb4695c404a5c2fde8a33a02a67bac0f622fb763d": {
				"ticker": "FLOKI",
				"address": "0xB4695C404A5C2fDE8a33a02a67bac0F622FB763d",
				"name": "Wrapped Floki",
				"decimals": 11
			},
			"0xb7bcab940c1cadb117987483e1e171fe73abe02e": {
				"ticker": "GLTCH",
				"address": "0xb7BcAb940C1CADb117987483e1E171fe73ABE02E",
				"name": "Project Glitch",
				"decimals": 10
			},
			"0x478867a976ec0cbad24efd5b8ab20313c9628c28": {
				"ticker": "ELN",
				"address": "0x478867a976ec0cbaD24efd5B8aB20313C9628C28",
				"name": "Elon Rise",
				"decimals": 11
			},
			"0x62d209e5eb7b749d18ee0b4af893b54123129c2d": {
				"ticker": "PNPL",
				"address": "0x62D209E5EB7b749d18Ee0b4af893B54123129c2d",
				"name": "Zen Pineapple",
				"decimals": 10
			},
			"0x14374c8e94eaf196bef6869674ece4a313f19d38": {
				"ticker": "STRT",
				"address": "0x14374C8e94eaF196bEf6869674eCe4a313f19d38",
				"name": "Star Treasure",
				"decimals": 10
			},
			"0x339c601e45783e88edb302ad2fa1ff51172d26c9": {
				"ticker": "WINGS",
				"address": "0x339c601E45783e88eDB302Ad2FA1ff51172D26C9",
				"name": "Exo Angel",
				"decimals": 12
			},
			"0x5472e93626803ff5c8821d98e8608c39ca33a11c": {
				"ticker": "RGK",
				"address": "0x5472E93626803fF5c8821d98E8608C39cA33A11C",
				"name": "Ragnarok",
				"decimals": 9
			},
			"0xb22af66de7f1a045cf24347ba52cad89b8f6efb2": {
				"ticker": "SBA",
				"address": "0xB22AF66dE7F1A045Cf24347ba52CaD89b8F6efb2",
				"name": "Shiba Coin",
				"decimals": 10
			},
			"0x19efbd41d5e748a0c3926c3353a8225d34b028a6": {
				"ticker": "MEAD",
				"address": "0x19eFbD41d5e748A0C3926C3353A8225D34B028A6",
				"name": "Mead",
				"decimals": 18
			},
			"0xeebbd50e9eba4f91b13e5a73bf2972969602106e": {
				"ticker": "$LABOR",
				"address": "0xeeBBD50E9eBa4f91B13e5a73bF2972969602106E",
				"name": "Labor Union Trade",
				"decimals": 18
			},
			"0x04aa0e14045bbe20a3d1d367448d58701c9e5e35": {
				"ticker": "MEGA",
				"address": "0x04aa0E14045bbe20a3D1d367448D58701c9e5E35",
				"name": "Mega DAO",
				"decimals": 9
			},
			"0x3709e8615e02c15b096f8a9b460ccb8ca8194e86": {
				"ticker": "VEE",
				"address": "0x3709E8615E02C15B096f8a9B460ccb8cA8194e86",
				"name": "Vee",
				"decimals": 18
			},
			"0x95302b4800cb33386cd1b6d7152ea428273be4ae": {
				"ticker": "RGK",
				"address": "0x95302b4800cB33386Cd1b6d7152EA428273BE4AE",
				"name": "RNODE",
				"decimals": 9
			},
			"0x8851ecb143758f3759979085badc1f6c53dda503": {
				"ticker": "ECD",
				"address": "0x8851eCb143758f3759979085badc1F6C53DDa503",
				"name": "Echidna Finance",
				"decimals": 6
			},
			"0xf9cd15364bff29d52e5657f3556b82b21e871a8d": {
				"ticker": "SNAKE",
				"address": "0xf9Cd15364Bff29d52e5657f3556B82b21e871a8d",
				"name": "SnakeFinance Token",
				"decimals": 18
			},
			"0x12d45cb5d8420ac1fc1934752fe114897032c363": {
				"ticker": "PANA",
				"address": "0x12d45cB5d8420Ac1FC1934752fe114897032C363",
				"name": "Panda AVAX",
				"decimals": 12
			},
			"0xe75fb3db715e21f9f3526787ce7105d859d79418": {
				"ticker": "THN",
				"address": "0xe75fb3DB715E21f9f3526787cE7105D859d79418",
				"name": "Super Thunder",
				"decimals": 9
			},
			"0x2b9f33d3d768dda42df58c69c3c74e373687116f": {
				"ticker": "ZCHI",
				"address": "0x2b9F33D3d768Dda42df58c69C3c74E373687116F",
				"name": "Zen Hachi",
				"decimals": 12
			},
			"0xb86597ade81ec60756d1dc0d085dde69589b1986": {
				"ticker": "BAPE",
				"address": "0xB86597adE81EC60756d1Dc0d085DDe69589b1986",
				"name": "Baby Ape",
				"decimals": 10
			},
			"0x772b0494b856c392f570506ae94e3684887c35ae": {
				"ticker": "KPAN",
				"address": "0x772B0494b856c392F570506AE94e3684887c35Ae",
				"name": "King Panda",
				"decimals": 10
			},
			"0xaaef3f184c939165014323837ba72b988e2ee87a": {
				"ticker": "HCH",
				"address": "0xaaef3F184c939165014323837Ba72b988e2Ee87a",
				"name": "King Hachi",
				"decimals": 9
			},
			"0xa1263b7d87546ac61c8a07f8934297f4bd01bb0c": {
				"ticker": "DOUB",
				"address": "0xa1263B7D87546AC61C8A07f8934297F4bd01BB0C",
				"name": "PIRATE DOUBLOON",
				"decimals": 18
			},
			"0x1d60109178c48e4a937d8ab71699d8ebb6f7c5de": {
				"ticker": "MAG",
				"address": "0x1d60109178C48E4A937D8AB71699D8eBb6F7c5dE",
				"name": "Magnet",
				"decimals": 9
			},
			"0x37666b8f51b1d3e30beaa0b788b721548edf66a0": {
				"ticker": "MGOL",
				"address": "0x37666B8F51b1D3e30beAA0b788b721548EdF66A0",
				"name": "Mini Gold",
				"decimals": 12
			},
			"0x44a2d52ea15375f472f440fb02ed0ebcd766e608": {
				"ticker": "BFAN",
				"address": "0x44a2D52EA15375f472f440fB02Ed0EBCD766e608",
				"name": "Baby Fantom",
				"decimals": 11
			},
			"0xe6ef3bfb234f93243d8e78fbcb08d9d565d3e432": {
				"ticker": "MRS",
				"address": "0xE6Ef3Bfb234f93243d8e78FBcb08d9D565d3e432",
				"name": "Mars Chain",
				"decimals": 12
			},
			"0xd962e6e7d945fbfecf35184da5774f653f5cd5fb": {
				"ticker": "Avakita",
				"address": "0xd962e6E7d945fBfecf35184dA5774f653f5cD5FB",
				"name": "@Avakita",
				"decimals": 9
			},
			"0x6c1080ce3f91dd775e1a70199aff2b8f521cf749": {
				"ticker": "DOGE",
				"address": "0x6C1080cE3f91DD775e1a70199aFF2b8F521Cf749",
				"name": "Doge BTC",
				"decimals": 10
			},
			"0x57319d41f71e81f3c65f2a47ca4e001ebafd4f33": {
				"ticker": "xJOE",
				"address": "0x57319d41F71E81F3c65F2a47CA4e001EbAFd4F33",
				"name": "JoeBar",
				"decimals": 18
			},
			"0x47eb6f7525c1aa999fbc9ee92715f5231eb1241d": {
				"ticker": "MELT",
				"address": "0x47EB6F7525C1aA999FBC9ee92715F5231eB1241D",
				"name": "Defrost Finance Token",
				"decimals": 18
			},
			"0x549b0e7b3dc114eff1d7a8b3e724fd1147ef7e8d": {
				"ticker": "MAPL",
				"address": "0x549b0E7B3DC114EFF1d7a8b3e724Fd1147Ef7E8d",
				"name": "Mini Apple",
				"decimals": 10
			},
			"0xfe502de4c66dfb3377194405c288e287ec5b5091": {
				"ticker": "SRA",
				"address": "0xFE502dE4C66dFB3377194405c288E287ec5B5091",
				"name": "Sierra",
				"decimals": 18
			},
			"0x47ee4f4e7e4d3ded443a6fc40dba5841ce5374c0": {
				"ticker": "ZAPL",
				"address": "0x47ee4F4E7e4D3deD443A6FC40dba5841cE5374C0",
				"name": "Zen Apple",
				"decimals": 9
			},
			"0x2b0b320b47d8e0dd0e4477cf90c307c7ed984ad2": {
				"ticker": "METAG",
				"address": "0x2b0B320B47D8e0DD0E4477cf90c307c7ed984Ad2",
				"name": "METAG",
				"decimals": 18
			},
			"0x0d7d9b1d58a1f9458e4e1aad519afedc04177ce8": {
				"ticker": "PAPL",
				"address": "0x0d7D9B1D58a1f9458E4E1aAd519afEdc04177Ce8",
				"name": "Project Apple",
				"decimals": 12
			},
			"0xacabc9f5893f05c5baee5c44916f52b2e2bff0ad": {
				"ticker": "HCI",
				"address": "0xAcaBc9f5893f05c5BAEE5c44916F52b2e2BfF0ad",
				"name": "Fantasy Hachi",
				"decimals": 9
			},
			"0x1b7dfcff0b002e8f344376b0c0ef2eb7759f9207": {
				"ticker": "PINI",
				"address": "0x1b7DFCFF0b002E8F344376B0C0ef2eb7759F9207",
				"name": "Pineapple Inu",
				"decimals": 12
			},
			"0xd9e8b903b56d6c13a6e6cfcec146ee5178bd541d": {
				"ticker": "MARS",
				"address": "0xD9e8B903B56D6C13A6e6CFCEC146EE5178BD541d",
				"name": "King Mars",
				"decimals": 11
			},
			"0x76e711243c8163f6e2baf5479daa8f43f8a167dc": {
				"ticker": "APLQ",
				"address": "0x76E711243c8163F6e2bAf5479DaA8F43F8A167dC",
				"name": "Pineapple Queen",
				"decimals": 10
			},
			"0xe6a800945310606de25ffef83da5c6e271963189": {
				"ticker": "FLY",
				"address": "0xe6a800945310606dE25fFEf83dA5C6e271963189",
				"name": "NO FLY ZONE",
				"decimals": 9
			},
			"0x99a855f23f53753e4003cdf0e0da44719e974040": {
				"ticker": "PAN",
				"address": "0x99a855f23f53753E4003cDf0e0DA44719E974040",
				"name": "Panda Protocol",
				"decimals": 10
			},
			"0x730f7a07c7d1b88efe96ad4f722da60b2fff3c02": {
				"ticker": "FFKI",
				"address": "0x730F7A07c7d1b88eFe96AD4f722dA60b2ffF3c02",
				"name": "Fantasy Floki",
				"decimals": 12
			},
			"0x1565c7746236434b9b37b8c5f4a8f7387869685a": {
				"ticker": "AKT",
				"address": "0x1565C7746236434B9b37B8C5F4A8f7387869685A",
				"name": "Akita Inu",
				"decimals": 11
			},
			"0xbd928e48544de6331616da6596442b1744757f9b": {
				"ticker": "CRG",
				"address": "0xbD928E48544DE6331616da6596442b1744757F9b",
				"name": "Corgi Cash",
				"decimals": 12
			},
			"0xa1344abad6cbd7943dec5d12c2bfba22878426ba": {
				"ticker": "preASTRO",
				"address": "0xA1344abad6CBd7943deC5d12c2BFbA22878426bA",
				"name": "100 Days Ventures",
				"decimals": 18
			},
			"0xb1df48cce4f52450076257008a2a7b5447257b76": {
				"ticker": "GAL",
				"address": "0xb1dF48cCE4f52450076257008A2a7b5447257b76",
				"name": "Project Galaxy",
				"decimals": 18
			},
			"0xba0b793d035febcd4e613144654a8f9bbdf56f4b": {
				"ticker": "TOMB",
				"address": "0xBA0B793d035FEBCd4e613144654a8F9bBDf56f4b",
				"name": "Fantom Block",
				"decimals": 12
			},
			"0xb30811b2abb6b86e253cf38dc91e6efa5705185b": {
				"ticker": "REKT",
				"address": "0xB30811b2ABb6B86e253Cf38Dc91E6Efa5705185b",
				"name": "Rekt",
				"decimals": 9
			},
			"0x3ee97d514bbef95a2f110e6b9b73824719030f7a": {
				"ticker": "sSPELL",
				"address": "0x3Ee97d514BBef95a2f110e6B9b73824719030f7a",
				"name": "Staked Spell Tokens",
				"decimals": 18
			},
			"0x4f2cee2ce977cef2a7bbac87ecd63f9c1540b371": {
				"ticker": "WWHL",
				"address": "0x4f2Cee2ce977cEF2a7Bbac87ECD63f9C1540b371",
				"name": "Wrapped Whale",
				"decimals": 10
			},
			"0x445afa7ad5a54e1dd0ab9a78262a5e0f2d082c90": {
				"ticker": "PPLE",
				"address": "0x445afA7AD5a54e1dD0AB9a78262A5E0F2d082C90",
				"name": "Meta Pineapple",
				"decimals": 10
			},
			"0x15c482ad3e86aac59f791270302e1c029f6e6a26": {
				"ticker": "WINGS",
				"address": "0x15C482AD3E86Aac59f791270302E1C029f6e6a26",
				"name": "Angel Cash",
				"decimals": 11
			},
			"0xab577dfdc3245afe6d10ecc302ccea01b96422f3": {
				"ticker": "THN",
				"address": "0xAb577dFdc3245aFe6D10ECc302ccEa01B96422f3",
				"name": "Thunder Classic",
				"decimals": 12
			},
			"0x75fac073492f50798cc56bf5ff6a179e7338d789": {
				"ticker": "SYUM",
				"address": "0x75fac073492F50798cc56BF5ff6A179E7338d789",
				"name": "Safe Cake",
				"decimals": 11
			},
			"0x50ac6c600f8452639404b573cb3352958cf8ffdc": {
				"ticker": "APE",
				"address": "0x50AC6C600F8452639404b573cb3352958cf8FfDC",
				"name": "Exo Ape",
				"decimals": 12
			},
			"0xb2917b4617c99a84a13c205d3c2dc66576888609": {
				"ticker": "ELN",
				"address": "0xB2917B4617C99A84a13c205d3C2dC66576888609",
				"name": "Project Elon",
				"decimals": 12
			},
			"0x9b9d566813a9893c76b93adb7e613405595c7c2d": {
				"ticker": "SHIB",
				"address": "0x9b9D566813a9893C76B93aDB7E613405595c7C2d",
				"name": "Shiba BTC",
				"decimals": 9
			},
			"0x25f4c783e84091169e4eca412592c64c8ad0518c": {
				"ticker": "CMP",
				"address": "0x25f4c783e84091169E4Eca412592c64C8AD0518C",
				"name": "Mini Chimp",
				"decimals": 12
			},
			"0x9ad8ffd3bd96e312b4cbb84865b1891ddd76d79c": {
				"ticker": "ZAPD",
				"address": "0x9aD8ffD3BD96e312B4cBb84865B1891dDD76d79c",
				"name": "Thunder Dapp",
				"decimals": 12
			},
			"0xf0b0af504151f432ace24616b3ca7f1ae885e273": {
				"ticker": "FPND",
				"address": "0xf0b0Af504151f432ACE24616b3Ca7f1ae885e273",
				"name": "Fantasy Panda",
				"decimals": 10
			},
			"0xc6792762ffb9f2cae12dc9735bd49795c2fa58f6": {
				"ticker": "SGHS",
				"address": "0xc6792762FFB9f2CAe12DC9735Bd49795c2fa58f6",
				"name": "Sushi Ghost",
				"decimals": 9
			},
			"0x729865b11376c126630930f7d073e53815fe9571": {
				"ticker": "TOM",
				"address": "0x729865B11376C126630930F7D073E53815fe9571",
				"name": "Fantom Beast",
				"decimals": 9
			},
			"0xf692d92f36f68e5be9f829805ca9242c98a8498c": {
				"ticker": "CAKE",
				"address": "0xf692d92f36f68E5Be9f829805CA9242C98a8498c",
				"name": "Project Cake",
				"decimals": 11
			},
			"0x55b9411acb619382d82d83f602e9b90dd1dfa247": {
				"ticker": "LDOG",
				"address": "0x55b9411acb619382D82D83F602E9b90dd1DFa247",
				"name": "Little Doge",
				"decimals": 11
			},
			"0xf7909d47daaf3379e195269b8ea9e53064bc672d": {
				"ticker": "CAMG",
				"address": "0xf7909D47DAaF3379E195269b8EA9e53064bC672D",
				"name": "Cat and mouse game",
				"decimals": 18
			},
			"0xadcb4d16207e8ac056d7886c75a9148eb51cf593": {
				"ticker": "GHT",
				"address": "0xaDcb4D16207E8aC056D7886C75A9148eB51Cf593",
				"name": "Safe Ghost",
				"decimals": 9
			},
			"0xcbe8df04e34e5eebe982c675c858d826b3c27297": {
				"ticker": "MEAD",
				"address": "0xcbE8Df04e34e5eEbE982c675c858D826b3c27297",
				"name": "Mead",
				"decimals": 18
			},
			"0xd6070ae98b8069de6b494332d1a1a81b6179d960": {
				"ticker": "BIFI",
				"address": "0xd6070ae98b8069de6B494332d1A1a81B6179D960",
				"name": "beefy.finance",
				"decimals": 18
			},
			"0x78ba51a81c2391360af01cab7e328375987eab99": {
				"ticker": "PAN",
				"address": "0x78ba51a81c2391360AF01Cab7E328375987eAB99",
				"name": "Meta Panda",
				"decimals": 10
			},
			"0xa64cbe690fde0ca177f3273de9b8a39a91a5a140": {
				"ticker": "GTU",
				"address": "0xA64CbE690fde0Ca177f3273dE9b8a39a91A5a140",
				"name": "GTU",
				"decimals": 18
			},
			"0x1e7e63a164155e2b0eb68b418ee58afdcaf8f374": {
				"ticker": "GOLD",
				"address": "0x1e7E63a164155e2b0Eb68B418Ee58aFDcAf8f374",
				"name": "Avax Gold",
				"decimals": 9
			},
			"0x3df307e8e9a897da488211682430776cdf0f17cc": {
				"ticker": "gREKT",
				"address": "0x3Df307e8E9a897Da488211682430776CDF0f17cC",
				"name": "Governance REKT",
				"decimals": 18
			},
			"0xc00c2326ce5cb9c3948b4b44bd64013fee6bf25d": {
				"ticker": "MOON",
				"address": "0xC00C2326cE5Cb9C3948B4b44bD64013fEe6Bf25D",
				"name": "Queen Moon",
				"decimals": 12
			},
			"0xaa6f82a9397faee844c4851e25fb0d64c2f90f58": {
				"ticker": "MSHARE",
				"address": "0xAA6f82A9397fAEE844C4851e25FB0D64C2f90F58",
				"name": "MSHARE",
				"decimals": 18
			},
			"0x1d985e71936877714b7e4818c0b72c565f3d54de": {
				"ticker": "TOM",
				"address": "0x1D985e71936877714b7e4818C0b72C565f3D54De",
				"name": "Fantom Rocket",
				"decimals": 10
			},
			"0xad198c20e43e1b60988812971ede728a7617cced": {
				"ticker": "STAR",
				"address": "0xAD198C20e43E1B60988812971ede728A7617CCed",
				"name": "Treasure Star",
				"decimals": 10
			},
			"0xe29909a29b5dcaacac7007ad2fe1321558ac2951": {
				"ticker": "DOGE",
				"address": "0xE29909A29b5DcaacAC7007ad2fE1321558Ac2951",
				"name": "Doge BTC",
				"decimals": 9
			},
			"0xd9d90f882cddd6063959a9d837b05cb748718a05": {
				"ticker": "MORE",
				"address": "0xd9D90f882CDdD6063959A9d837B05Cb748718A05",
				"name": "More Token",
				"decimals": 18
			},
			"0x861da474fa3e4ead2e117f300fd902925adad9a3": {
				"ticker": "GOLD",
				"address": "0x861Da474fa3E4eaD2e117f300fD902925AdAD9A3",
				"name": "GOLD DAO",
				"decimals": 9
			},
			"0x8d9882988f02318d1c801183b209ca3e8427a2e6": {
				"ticker": "CASH",
				"address": "0x8d9882988F02318D1C801183B209CA3e8427A2e6",
				"name": "Avax Cash",
				"decimals": 9
			},
			"0x0600bbd8e6204cc270a1c1dadb4b530cfb57279e": {
				"ticker": "MRS",
				"address": "0x0600BBd8e6204CC270A1c1DaDb4B530CFB57279e",
				"name": "Mars Block",
				"decimals": 9
			},
			"0x84be1ea318948761e33c5d26c2cde72b6c8fc909": {
				"ticker": "CMP",
				"address": "0x84Be1EA318948761E33c5d26C2CDe72B6C8FC909",
				"name": "Exo Chimp",
				"decimals": 11
			},
			"0x492175a3893da14c4973cba06df80122ff2dd9ca": {
				"ticker": "WHLC",
				"address": "0x492175A3893Da14C4973CbA06DF80122FF2dD9ca",
				"name": "Whale Cash",
				"decimals": 9
			},
			"0x30325ea1780f98b9562e0917cb5ae800734f099c": {
				"ticker": "GLD",
				"address": "0x30325ea1780F98b9562E0917cB5Ae800734F099c",
				"name": "Gold Markets",
				"decimals": 11
			},
			"0x081a99648fa7e9f0a8839715f8068a5bdc106312": {
				"ticker": "preASTRO",
				"address": "0x081A99648fA7e9F0A8839715f8068a5BDc106312",
				"name": "100 Days Ventures",
				"decimals": 18
			},
			"0x7388acaa7165cad9a1aa8b09694ea0d4a003c02f": {
				"ticker": "MegaBurn",
				"address": "0x7388acAa7165caD9A1aA8B09694Ea0d4A003c02F",
				"name": "https://twitter.com/MegaBurnAVAX",
				"decimals": 9
			},
			"0xe4e0edbfc5e13a2f0f3dc79469ec97004e917463": {
				"ticker": "AKITA",
				"address": "0xE4E0EdbFc5E13a2F0f3Dc79469Ec97004e917463",
				"name": "Super Akita",
				"decimals": 10
			},
			"0xda584c8a6490ac338d98146ada799726add2e859": {
				"ticker": "ELON",
				"address": "0xDa584C8a6490Ac338D98146aDA799726adD2e859",
				"name": "Queen Elon",
				"decimals": 9
			},
			"0xf5dc4d1f0e3ca0237a7199dd1a09c295da0541d9": {
				"ticker": "BAKT",
				"address": "0xf5Dc4D1F0E3CA0237A7199dd1A09C295dA0541d9",
				"name": "Baby Akita",
				"decimals": 10
			},
			"0x0416881bbdc8a6a030dca6ec4a20f7ca7251780d": {
				"ticker": "NXS",
				"address": "0x0416881bBdc8A6a030Dca6eC4A20F7ca7251780d",
				"name": "Nexus",
				"decimals": 18
			},
			"0x04dd8b92ab6b332d7ab180c701c8ebc35be1428d": {
				"ticker": "GHT",
				"address": "0x04Dd8B92aB6b332d7AB180C701c8eBC35Be1428d",
				"name": "Baby Ghost",
				"decimals": 9
			},
			"0x9e8dd0e991c085a54efa731c7ac8b2aa9676ebf9": {
				"ticker": "PPND",
				"address": "0x9E8Dd0E991C085a54Efa731c7ac8b2aA9676EbF9",
				"name": "Project Panda",
				"decimals": 12
			},
			"0x7b6824838888c2a5b68fd2775021e9b9e3ffe8fe": {
				"ticker": "ELON",
				"address": "0x7B6824838888C2A5b68Fd2775021e9b9E3fFE8FE",
				"name": "OMG Elon",
				"decimals": 9
			},
			"0xa3be1b566668fe0c17e1062cd3fc896759001efb": {
				"ticker": "GRAIN",
				"address": "0xA3bE1b566668fE0c17e1062Cd3fC896759001EFb",
				"name": "GRAINTOKEN",
				"decimals": 18
			},
			"0x700e050ceefd145646e0f365d972c716c910d6a6": {
				"ticker": "WHLD",
				"address": "0x700e050CeEFd145646E0F365d972c716c910D6A6",
				"name": "Whale DAO",
				"decimals": 11
			},
			"0x3bcecdc7d66674268666a3b1554b9c4e6a7b88c7": {
				"ticker": "ZAPR",
				"address": "0x3bCEcDC7D66674268666a3b1554B9C4E6a7B88C7",
				"name": "Thunder Rise",
				"decimals": 9
			},
			"0xfa4d89cdaeae1b9dec43c39f086debcd0075556c": {
				"ticker": "KANDY",
				"address": "0xfa4d89cdAEae1B9DEc43c39F086DeBCD0075556c",
				"name": "KandyLand Dao",
				"decimals": 9
			},
			"0xad0a1d1bb032634259b7af66ab75b1a9a7e33532": {
				"ticker": "ELON",
				"address": "0xAd0a1d1bB032634259B7af66ab75b1A9A7E33532",
				"name": "Exo Elon",
				"decimals": 10
			},
			"0x0e177e73d56c1775ffa161ba1ef64307f5a17742": {
				"ticker": "FUTFI",
				"address": "0x0e177e73d56c1775Ffa161ba1EF64307f5a17742",
				"name": "Future Finance",
				"decimals": 9
			},
			"0xf48b04a715c843f1b18f5bf198e276dcddd0b506": {
				"ticker": "APE",
				"address": "0xf48B04a715C843f1b18F5bf198e276DcDdd0b506",
				"name": "Ape Finance",
				"decimals": 10
			},
			"0x4106089e38513974362415a3203c25e8afe52bd5": {
				"ticker": "CAKE",
				"address": "0x4106089E38513974362415a3203C25E8afE52Bd5",
				"name": "OMG Cake",
				"decimals": 11
			},
			"0x4d596290827c24ba6bc2fd4c058ea34faddbe94a": {
				"ticker": "FLOKI",
				"address": "0x4d596290827C24bA6BC2fd4c058EA34FadDbe94a",
				"name": "Floki AVAX",
				"decimals": 12
			},
			"0xd609465f27eafd7080c35e3e7cc13d4234541415": {
				"ticker": "CTND",
				"address": "0xd609465f27EAfd7080C35E3e7cc13d4234541415",
				"name": "Crypto Thunder",
				"decimals": 11
			},
			"0x7fcb23279d4f70e0228a5fe5ab04bda82a96d06f": {
				"ticker": "GLD",
				"address": "0x7fCB23279D4f70e0228a5FE5AB04bda82a96d06f",
				"name": "Gold BTC",
				"decimals": 9
			},
			"0xe796d70dd3bc702b346a71b83287b2be11901ab5": {
				"ticker": "SAKT",
				"address": "0xe796D70dd3bc702B346a71B83287b2Be11901Ab5",
				"name": "Safe Akita",
				"decimals": 12
			},
			"0xf280a5136e7cc6749352e24e09dade104c7f3661": {
				"ticker": "FLK",
				"address": "0xf280A5136e7Cc6749352e24E09dADE104c7F3661",
				"name": "Floki Dapp",
				"decimals": 9
			},
			"0xcfe7dabd1d5deac8f1ccf87527ebac18748b6ca7": {
				"ticker": "THN",
				"address": "0xcfE7DAbd1d5deAC8f1Ccf87527eBAC18748B6CA7",
				"name": "Thunder NET",
				"decimals": 10
			},
			"0x640d01a4c67141f4eabacafc9cd3361070266d44": {
				"ticker": "PPLE",
				"address": "0x640d01a4C67141F4eabaCAFc9cd3361070266d44",
				"name": "Treasure Pineapple",
				"decimals": 11
			},
			"0x5f1312d37a607465b96dfecb83c1e41e508b27cf": {
				"ticker": "MOON",
				"address": "0x5F1312D37a607465B96dFECb83C1E41e508B27Cf",
				"name": "Moon AVAX",
				"decimals": 9
			},
			"0x0894fa0923ff808056a3cf24aa145d02c6491229": {
				"ticker": "CAESARS",
				"address": "0x0894fa0923ff808056A3cf24aa145d02C6491229",
				"name": "Caesars DAO",
				"decimals": 9
			},
			"0xa91d0c1a24d492bf606665a0cb01d65ce3068803": {
				"ticker": "SHB",
				"address": "0xA91d0C1a24D492bF606665A0CB01d65CE3068803",
				"name": "Queen Shiba",
				"decimals": 9
			},
			"0xe8e917012985b1bd3677cb7c61953b095f5d63af": {
				"ticker": "GLTD",
				"address": "0xe8E917012985b1bd3677cB7c61953b095F5d63af",
				"name": "Glitch Diamond",
				"decimals": 12
			},
			"0x9de05b095d492082bfb67069f47cdc3fcbd71527": {
				"ticker": "HCH",
				"address": "0x9de05B095D492082bFB67069f47CDc3fCBd71527",
				"name": "Hachi Classic",
				"decimals": 10
			},
			"0xc2e750c48bb7f798ba0804b94ba20f69a9ac3b76": {
				"ticker": "GOL",
				"address": "0xc2e750c48BB7F798ba0804b94BA20f69A9Ac3B76",
				"name": "Gold Treasure",
				"decimals": 11
			},
			"0x2265a8ff7b091a0e6707bbae19d428f1a0811f27": {
				"ticker": "nRYU",
				"address": "0x2265a8ff7b091a0E6707BbAE19D428f1a0811F27",
				"name": "nRYU Token",
				"decimals": 18
			},
			"0xc0261b8e957497350fd36af369408265326a1290": {
				"ticker": "GSHARE",
				"address": "0xC0261b8e957497350fD36aF369408265326a1290",
				"name": "GSHARE",
				"decimals": 18
			},
			"0x6dfd7a7215b6dfd2aaa7828ef9d809d908740ad3": {
				"ticker": "ANGI",
				"address": "0x6dfD7a7215b6Dfd2AAa7828ef9D809D908740ad3",
				"name": "Angel INC",
				"decimals": 10
			},
			"0xe337a99fe0ac0f3e01a97167d21082a2bb96326e": {
				"ticker": "NEIBR",
				"address": "0xE337a99fE0AC0F3E01A97167D21082A2Bb96326E",
				"name": "The Neighbours",
				"decimals": 18
			},
			"0x77eed4ef52fe684cc8c7edfd066e5e6cad56f0c0": {
				"ticker": "PSTR",
				"address": "0x77EeD4EF52Fe684cC8C7edfd066e5e6CaD56F0c0",
				"name": "Project Star",
				"decimals": 12
			},
			"0x944d10449438e54d295f0408265791d0c47a310d": {
				"ticker": "FLUX",
				"address": "0x944D10449438e54d295F0408265791D0c47a310D",
				"name": "Flux Avax Token",
				"decimals": 9
			},
			"0x5252b195a1aec27bb2243e8f9ed26c2b358976b1": {
				"ticker": "WFTM",
				"address": "0x5252B195a1aec27Bb2243e8F9eD26C2b358976B1",
				"name": "Wrapped Fantom",
				"decimals": 11
			},
			"0x1b53e0bc02904ee8fdaf09ed624a3e7a88d367dd": {
				"ticker": "XPANZ",
				"address": "0x1B53e0bc02904eE8FdAF09Ed624a3E7A88D367Dd",
				"name": "eXpanz",
				"decimals": 18
			},
			"0x0c0e6b31fb3a583fdef2508cdabe2d17f82683b9": {
				"ticker": "APE",
				"address": "0x0c0e6b31fb3A583fdEf2508cdaBe2D17f82683B9",
				"name": "Safe Ape",
				"decimals": 10
			},
			"0x28a47993d73d229356840445053483d55120ae0d": {
				"ticker": "ANG",
				"address": "0x28A47993D73D229356840445053483D55120Ae0D",
				"name": "Mini Angel",
				"decimals": 12
			},
			"0x24dfd8ae82a6604f28ed8ce198017591f582aa36": {
				"ticker": "WHALE",
				"address": "0x24DfD8ae82a6604F28eD8ce198017591f582aa36",
				"name": "Queen Whale",
				"decimals": 10
			},
			"0x76fef9f486b191931d3415689f1e3561398cc7ab": {
				"ticker": "APLB",
				"address": "0x76fEF9f486B191931d3415689f1E3561398cc7aB",
				"name": "Apple BTC",
				"decimals": 11
			},
			"0xd67de0e0a0fd7b15dc8348bb9be742f3c5850454": {
				"ticker": "FXS",
				"address": "0xD67de0e0a0Fd7b15dC8348Bb9BE742F3c5850454",
				"name": "Frax Share",
				"decimals": 18
			},
			"0x33095254eddab33ad1c18a1b155a608469941f8d": {
				"ticker": "INFI",
				"address": "0x33095254eDDab33AD1C18a1B155a608469941F8d",
				"name": "InfiniteNodes",
				"decimals": 18
			},
			"0x2ce4730a579c3c5510246232e8f8d409cbc0c1c5": {
				"ticker": "META",
				"address": "0x2CE4730A579C3c5510246232e8F8d409Cbc0c1C5",
				"name": "MetaVerse DAO",
				"decimals": 9
			},
			"0x9bf81f004aa6f22bdf73b55eac8016790918f653": {
				"ticker": "SBAA",
				"address": "0x9bF81F004aA6f22BDF73b55eac8016790918f653",
				"name": "Shiba AI",
				"decimals": 12
			},
			"0xfcc6ce74f4cd7edef0c5429bb99d38a3608043a5": {
				"ticker": "FIRE",
				"address": "0xfcc6CE74f4cd7eDEF0C5429bB99d38A3608043a5",
				"name": "FIRE",
				"decimals": 18
			},
			"0x3c93868df6aa8098d0703b221de9124f2f2f3626": {
				"ticker": "Fantasy",
				"address": "0x3C93868df6AA8098D0703B221dE9124f2f2F3626",
				"name": "Fantasy Node",
				"decimals": 18
			},
			"0xf3aa1e52dac92b56fb534c296043269215b99fa3": {
				"ticker": "OYUM",
				"address": "0xF3aA1e52DAc92b56fb534C296043269215B99fa3",
				"name": "OMG Cake",
				"decimals": 12
			},
			"0xbf096702e7e18194c1dd9cb8f8b1068dc07d46dc": {
				"ticker": "EGJP",
				"address": "0xbF096702E7E18194c1dD9CB8f8B1068dc07d46DC",
				"name": "EGJP",
				"decimals": 18
			},
			"0x275004591ef3623b4ef7e571ffb559db1e8b361e": {
				"ticker": "APLD",
				"address": "0x275004591eF3623b4ef7e571FFb559DB1E8b361e",
				"name": "Apple Diamond",
				"decimals": 12
			},
			"0xb8fdfdcaaf23dc0a907cb500feae210c2809a9cb": {
				"ticker": "CHI",
				"address": "0xB8fDfDCAaf23dC0a907cb500FEaE210C2809a9CB",
				"name": "Wrapped Chimp",
				"decimals": 12
			},
			"0x83a283641c6b4df383bcddf807193284c84c5342": {
				"ticker": "VPND",
				"address": "0x83a283641C6B4DF383BCDDf807193284C84c5342",
				"name": "VaporNodes",
				"decimals": 18
			},
			"0xc6b8a1e0a362e6765f6f33a4b246ad7978a2e1f5": {
				"ticker": "PAND",
				"address": "0xc6b8a1e0A362e6765f6f33a4b246Ad7978A2e1f5",
				"name": "Panda DAO",
				"decimals": 10
			},
			"0x5adb8a7a8f59da6925c18acf8731b472e6f852ee": {
				"ticker": "APE",
				"address": "0x5Adb8a7a8F59dA6925c18ACF8731B472E6F852EE",
				"name": "Little Ape",
				"decimals": 11
			},
			"0x56c092923845add9ad92dd8bc015d4dd8699dfa9": {
				"ticker": "STR",
				"address": "0x56C092923845AdD9Ad92DD8BC015D4dd8699dfa9",
				"name": "Star Classic",
				"decimals": 12
			},
			"0x2337a2704e5a6b45c3563abdff2455d23cc0cf5c": {
				"ticker": "APLI",
				"address": "0x2337a2704e5a6b45C3563AbdFF2455d23Cc0CF5C",
				"name": "Apple Infinity",
				"decimals": 9
			},
			"0xb28934178ca17a004c8b1655fc8dd5dad09f7639": {
				"ticker": "WHCI",
				"address": "0xb28934178ca17a004c8b1655FC8Dd5DAd09F7639",
				"name": "Wrapped Hachi",
				"decimals": 10
			},
			"0xdc248e404defbfefdf573f897b87114b4d0c9641": {
				"ticker": "RND",
				"address": "0xDc248E404defbfEfdF573f897b87114b4D0c9641",
				"name": "PlayMate",
				"decimals": 18
			},
			"0x1aec19619d59ea2a3fbee666314ddd9ddae13c20": {
				"ticker": "SRA",
				"address": "0x1AEC19619d59Ea2a3FbEE666314dDd9DdAE13C20",
				"name": "SierraDAO",
				"decimals": 18
			},
			"0x1cc58f047a102a6443572b071f503276800f9893": {
				"ticker": "HAKU",
				"address": "0x1Cc58F047a102a6443572B071F503276800F9893",
				"name": "Haku Swap",
				"decimals": 18
			},
			"0x42cfa3da3148bf9a40fd22641fc153dce00e28ac": {
				"ticker": "BLISS",
				"address": "0x42CFa3Da3148Bf9A40fd22641fC153DCe00e28ac",
				"name": "BLISS",
				"decimals": 18
			},
			"0x3e62c71745e2d54b101840515b3eb08121455561": {
				"ticker": "FTM",
				"address": "0x3E62c71745E2D54b101840515B3eB08121455561",
				"name": "OMG Fantom",
				"decimals": 11
			},
			"0x6ab00c3d05fbcbbdff4c0e51f039a2ee7d6ec083": {
				"ticker": "PDA",
				"address": "0x6aB00c3D05fbcBBDff4C0E51F039a2ee7D6ec083",
				"name": "Panda Swap",
				"decimals": 12
			},
			"0x1aae03b17fcd675f38f3e854f56e6c0b070dcf32": {
				"ticker": "PPL",
				"address": "0x1aAE03B17FcD675F38F3e854F56e6C0b070Dcf32",
				"name": "PplCoin",
				"decimals": 18
			},
			"0x5d3a57db46b96277543c4143d123d10160ed672d": {
				"ticker": "ZAP",
				"address": "0x5D3a57Db46B96277543C4143D123D10160eD672D",
				"name": "Thunder Farm",
				"decimals": 11
			},
			"0x3117734e4d0a28ed1e9f67779727c162a034d75c": {
				"ticker": "CGI",
				"address": "0x3117734E4D0a28ED1E9F67779727C162a034D75c",
				"name": "Super Corgi",
				"decimals": 9
			},
			"0x1c84864035b55b23200e115c54ff950c823be5aa": {
				"ticker": "ACE",
				"address": "0x1c84864035b55b23200E115c54FF950C823BE5AA",
				"name": "Avax Ace",
				"decimals": 9
			},
			"0xbf2c5788c028765880421e8468240b5050cf45f9": {
				"ticker": "GHST",
				"address": "0xBf2c5788C028765880421e8468240b5050cf45f9",
				"name": "Little Ghost",
				"decimals": 9
			},
			"0x1f305316fddc1ef9c2e8314501e90ee40fa025ff": {
				"ticker": "LTHN",
				"address": "0x1f305316FDDC1ef9C2E8314501e90eE40fA025FF",
				"name": "Little Thunder",
				"decimals": 11
			},
			"0xf804f75ab0925457e1f8021530682fc58b3dad14": {
				"ticker": "MFTM",
				"address": "0xf804f75ab0925457E1F8021530682fC58B3dad14",
				"name": "Meta Fantom",
				"decimals": 10
			},
			"0x9b49ab717ac6bf23efdd3418a31556b24b280ded": {
				"ticker": "MOON",
				"address": "0x9B49aB717ac6Bf23eFDd3418a31556B24b280dEd",
				"name": "Project Moon",
				"decimals": 11
			},
			"0xef9e4bebb2a69cccef1c803298727f082952a9c5": {
				"ticker": "ZTND",
				"address": "0xEf9e4beBB2A69CcCEf1c803298727f082952A9C5",
				"name": "Zen Thunder",
				"decimals": 9
			},
			"0xe9cdb1690f081377c4671618107e8265207c8032": {
				"ticker": "APL",
				"address": "0xE9cDb1690f081377c4671618107e8265207C8032",
				"name": "Queen Apple",
				"decimals": 12
			},
			"0xb46417527cce9200816b7a5ef32e8bd0347b20b5": {
				"ticker": "MARS",
				"address": "0xb46417527cCe9200816b7a5ef32e8bD0347b20b5",
				"name": "Zen Mars",
				"decimals": 10
			},
			"0x326ae41ec730995f2e5e95927ce80b11851aee77": {
				"ticker": "GLD",
				"address": "0x326AE41ec730995f2E5e95927Ce80B11851aEE77",
				"name": "Gold Coin",
				"decimals": 10
			},
			"0xe7824f4192e2f8e1dee657d3d77375936e0d6ee0": {
				"ticker": "Phantom",
				"address": "0xe7824F4192e2f8e1DeE657d3D77375936e0d6Ee0",
				"name": "Phantom Node",
				"decimals": 18
			},
			"0x3ee2baa8e8775a3b1710707728e997da6d51aa4d": {
				"ticker": "HCI",
				"address": "0x3ee2baA8E8775a3b1710707728e997DA6D51aA4d",
				"name": "Hachi Block",
				"decimals": 12
			},
			"0x36bbe1ece917703e222538e493ffc2d23668be24": {
				"ticker": "SPLASH",
				"address": "0x36bbE1Ece917703e222538e493FFC2D23668BE24",
				"name": "Splash Games",
				"decimals": 9
			},
			"0x57f099df1b75fed00357e5c5238d8170f6b2fc0e": {
				"ticker": "DOGE",
				"address": "0x57F099DF1B75fed00357e5C5238D8170F6B2fC0e",
				"name": "Super Doge",
				"decimals": 9
			},
			"0xbb4c0a5967ac38e2972e4b9130177b1af6ddfcef": {
				"ticker": "APE",
				"address": "0xbb4C0a5967AC38E2972E4B9130177b1aF6dDfcef",
				"name": "Treasure Ape",
				"decimals": 11
			},
			"0x249c0860fa3da0b452a2e49b91621f0f0c7f4835": {
				"ticker": "GLDI",
				"address": "0x249c0860Fa3da0B452A2E49b91621f0f0C7f4835",
				"name": "Gold Infinity",
				"decimals": 9
			},
			"0x264c1383ea520f73dd837f915ef3a732e204a493": {
				"ticker": "BNB",
				"address": "0x264c1383EA520f73dd837F915ef3a732e204a493",
				"name": "Binance",
				"decimals": 18
			},
			"0x63468133ed352e602beb61dd254d6060ad2fe419": {
				"ticker": "sTHO",
				"address": "0x63468133ed352E602bEB61DD254D6060Ad2fe419",
				"name": "StakedThorus",
				"decimals": 18
			},
			"0xe74d38f5ab632aebf93af48c5b9beff2e61bf554": {
				"ticker": "GTHM",
				"address": "0xe74d38F5aB632aEbF93Af48C5b9BEFf2e61bF554",
				"name": "Glitch Markets",
				"decimals": 11
			},
			"0xa1b0684884c4f24ea5e2f11afa0db1368e24e9d1": {
				"ticker": "FAND",
				"address": "0xA1B0684884C4f24eA5E2f11AfA0db1368e24e9d1",
				"name": "Fantom Diamond",
				"decimals": 11
			},
			"0x9e3e0e6b777eed9da3efd76b868743bad6e6dcab": {
				"ticker": "TANK",
				"address": "0x9E3E0E6b777eED9DA3efd76B868743bAD6e6DcAb",
				"name": "Tank Shares",
				"decimals": 18
			},
			"0xc8914ded0d8484f9bcd562797c8f6c99d4d224e3": {
				"ticker": "GLTCH",
				"address": "0xc8914DEd0D8484F9BCd562797C8F6c99d4D224E3",
				"name": "Glitch Diamond",
				"decimals": 12
			},
			"0xeda04ee2712ca9961847f0db7bc26eb8c22c7d84": {
				"ticker": "TGLD",
				"address": "0xeDa04Ee2712CA9961847F0dB7Bc26EB8C22c7D84",
				"name": "Treasure Gold",
				"decimals": 10
			},
			"0xba2bcf16dfd139712ea8a6f896fd3e33fb76e580": {
				"ticker": "SUN",
				"address": "0xBA2Bcf16dFD139712ea8A6f896fD3E33fb76e580",
				"name": "ECLIPSE",
				"decimals": 9
			},
			"0x6cc8ef6cfc173fc55a920c29e05222dead33b573": {
				"ticker": "DLC",
				"address": "0x6Cc8EF6CfC173Fc55A920C29E05222DEaD33B573",
				"name": "DashLeague Crystals",
				"decimals": 18
			},
			"0x6750a85ea02d7740698d587e2f63c0969ff027bf": {
				"ticker": "DOGE",
				"address": "0x6750a85Ea02D7740698D587E2F63C0969FF027Bf",
				"name": "Baby Doge",
				"decimals": 11
			},
			"0xae4aa155d2987b454c29450ef4f862cf00907b61": {
				"ticker": "THO",
				"address": "0xAE4AA155D2987B454C29450ef4f862CF00907B61",
				"name": "Thorus",
				"decimals": 18
			},
			"0x5a15bdcf9a3a8e799fa4381e666466a516f2d9c8": {
				"ticker": "SLIME",
				"address": "0x5a15Bdcf9a3A8e799fa4381E666466a516F2d9C8",
				"name": "Snail Trail",
				"decimals": 18
			},
			"0x3cb6cd27b72b3b1a6eda56699a4f1edc018e98a1": {
				"ticker": "MILK",
				"address": "0x3Cb6cD27b72B3b1A6eDA56699A4f1EDC018E98a1",
				"name": "Milk",
				"decimals": 18
			},
			"0x69feb5389247d68f3a9d7f3d87b987efe7d976cb": {
				"ticker": "PSTR",
				"address": "0x69FeB5389247D68f3a9D7f3d87b987EFE7d976Cb",
				"name": "Project Star",
				"decimals": 9
			},
			"0xe433ac3508c45926c1bc84da5fd87df886fa2bf3": {
				"ticker": "PEPE",
				"address": "0xE433Ac3508C45926c1BC84Da5fd87dF886fA2BF3",
				"name": "PepeBank",
				"decimals": 9
			},
			"0x8546733d85282656b753e1cb3c8cca1ac47abf1e": {
				"ticker": "STAR",
				"address": "0x8546733D85282656B753e1cb3c8CCA1AC47abf1E",
				"name": "OMG Star",
				"decimals": 11
			},
			"0x62cc0a1bd902bf97cfcb68bf70ce917b540f06f6": {
				"ticker": "HCI",
				"address": "0x62CC0A1Bd902Bf97CFCB68Bf70CE917B540F06F6",
				"name": "Hachi Dapp",
				"decimals": 9
			},
			"0x423a7a12a554b207822bb3a2faf4e9437b9f45a7": {
				"ticker": "TND",
				"address": "0x423A7A12a554B207822bB3A2fAF4e9437B9f45A7",
				"name": "Thunder Starter",
				"decimals": 9
			},
			"0x743a8b97dcd64f9a04589ac7edd9a19f2c86d4cf": {
				"ticker": "ICE",
				"address": "0x743A8B97dCD64f9A04589AC7EDd9A19f2C86d4CF",
				"name": "ICE",
				"decimals": 18
			},
			"0x73c66508439e5223865d55870406317c9b00b10e": {
				"ticker": "AKT",
				"address": "0x73C66508439e5223865d55870406317C9B00B10E",
				"name": "Akita X",
				"decimals": 9
			},
			"0x64409633d750301c3089a53a7c3267fc7376d649": {
				"ticker": "INFI",
				"address": "0x64409633D750301c3089a53a7c3267Fc7376d649",
				"name": "InfiniteNodes",
				"decimals": 18
			},
			"0xf3d9fff6364570202c52f945ba9da687d325276c": {
				"ticker": "CORGI",
				"address": "0xf3d9fff6364570202c52f945BA9dA687d325276C",
				"name": "Corgi NET",
				"decimals": 12
			},
			"0x176a88b3dab171688193ba56e41b0bd85a12258e": {
				"ticker": "DOGE",
				"address": "0x176a88B3DaB171688193Ba56e41b0bD85A12258e",
				"name": "Doge AI",
				"decimals": 12
			},
			"0x3c3dc86069588dd6aaf93a05412a87a3fc7615da": {
				"ticker": "SIMIAN",
				"address": "0x3C3Dc86069588dD6AAF93a05412A87a3FC7615DA",
				"name": "Simian Nodes",
				"decimals": 9
			},
			"0xc1ce798f1afb8d19562e58254d4e67e35292daf9": {
				"ticker": "FLY",
				"address": "0xC1ce798f1AFb8D19562E58254d4E67e35292dAf9",
				"name": "Hoppers Game",
				"decimals": 18
			},
			"0xe8a13e298a560d6ce5c42c7505b82752bd3d10ac": {
				"ticker": "CAKE",
				"address": "0xe8a13E298A560D6CE5C42c7505b82752bD3D10AC",
				"name": "Cake X",
				"decimals": 9
			},
			"0xfe1939770418f700bffab05c4e9a9da0427b048a": {
				"ticker": "APE",
				"address": "0xFe1939770418f700bffAb05c4E9A9DA0427B048a",
				"name": "OMG Ape",
				"decimals": 9
			},
			"0x1657c04c310461f8ad181b2bdbbeb86e72b7cad0": {
				"ticker": "CGIB",
				"address": "0x1657C04c310461F8Ad181B2BdbbeB86E72b7CAD0",
				"name": "Corgi Beast",
				"decimals": 11
			},
			"0x58f211a45670851e387e0b344bbd3f01f5e9a438": {
				"ticker": "FKI",
				"address": "0x58f211a45670851E387E0B344bBd3f01f5e9a438",
				"name": "Floki Markets",
				"decimals": 11
			},
			"0x824896f7f77980c984d498f0c4aaac29bf06c58b": {
				"ticker": "DOGE",
				"address": "0x824896F7f77980C984D498F0c4aAaC29bf06C58B",
				"name": "Doge INC",
				"decimals": 9
			},
			"0x467345ed6d045ac2db63e1fba9a9b261b9d96fd6": {
				"ticker": "TOM",
				"address": "0x467345ED6D045ac2DB63E1fba9A9B261b9d96fD6",
				"name": "Fantom Monster",
				"decimals": 9
			},
			"0xb11877e59c3b59870e8a6f771a0c83d1ee3f60f8": {
				"ticker": "DOUB",
				"address": "0xB11877E59C3b59870E8A6f771A0c83D1ee3F60f8",
				"name": "PIRATE DOUBLOON",
				"decimals": 18
			},
			"0x388089e67b864dc76091bde638bca9e639f9cece": {
				"ticker": "RNBW",
				"address": "0x388089e67B864DC76091bdE638bca9e639f9ceCE",
				"name": "Rainbow",
				"decimals": 18
			},
			"0x217a70148f5f88b34d9efa410627f1e887fbf077": {
				"ticker": "CGI",
				"address": "0x217a70148f5f88B34d9Efa410627f1E887fBf077",
				"name": "Corgi Dapp",
				"decimals": 12
			},
			"0x277299b5884e1f0c576f2a37743381340d35dc68": {
				"ticker": "ELN",
				"address": "0x277299B5884e1f0c576F2a37743381340d35DC68",
				"name": "Mini Elon",
				"decimals": 12
			},
			"0x4bf8668ea93fc6357a696279fe32a350ce77459d": {
				"ticker": "MOON",
				"address": "0x4bf8668ea93FC6357a696279fe32a350Ce77459d",
				"name": "Moon X",
				"decimals": 11
			},
			"0xcd0683f9e30042f2a17f46183377ac12e563cea9": {
				"ticker": "ASHARE",
				"address": "0xCd0683f9e30042F2a17f46183377Ac12E563CEa9",
				"name": "aShare",
				"decimals": 18
			},
			"0x71fdbb77bef7e07593570f31c5f271858095231f": {
				"ticker": "QSBA",
				"address": "0x71fDbB77BeF7e07593570f31C5f271858095231f",
				"name": "Queen Shiba",
				"decimals": 9
			},
			"0x3f78571f38bc307f1087f0bdd792bbc5d49f2c7e": {
				"ticker": "XSHARE",
				"address": "0x3f78571F38bC307f1087F0bDD792Bbc5d49F2c7e",
				"name": "XSHARE",
				"decimals": 18
			},
			"0x24b83228fcdb6d32ab33546788f361f9931a1093": {
				"ticker": "AKITA",
				"address": "0x24b83228fcdb6D32aB33546788f361f9931A1093",
				"name": "Akita AI",
				"decimals": 11
			},
			"0xf5901c084d6fc69750b15ea1e7d4b0119122ed3d": {
				"ticker": "STR",
				"address": "0xf5901C084D6Fc69750b15Ea1e7D4b0119122ed3d",
				"name": "Super Star",
				"decimals": 9
			},
			"0xe5b2a282198ff9b4383766492a284ab5d6ee8e73": {
				"ticker": "BTCH",
				"address": "0xE5B2a282198Ff9B4383766492A284AB5D6ee8E73",
				"name": "BEETCHDOTFARM",
				"decimals": 18
			},
			"0x2d24f79f7259bf259f7b076733398e739fdf7dfc": {
				"ticker": "APLR",
				"address": "0x2d24F79F7259bf259f7b076733398e739fDF7Dfc",
				"name": "Apple Robot",
				"decimals": 10
			},
			"0x1f1e7c893855525b303f99bdf5c3c05be09ca251": {
				"ticker": "SYN",
				"address": "0x1f1E7c893855525b303f99bDF5c3c05Be09ca251",
				"name": "Synapse",
				"decimals": 18
			},
			"0x7a219e17e3c5081cf4f69d5de8a8d51f97fa1242": {
				"ticker": "TRUMP",
				"address": "0x7A219e17e3c5081cF4F69D5DE8A8d51F97fA1242",
				"name": "TrumpNodes",
				"decimals": 18
			},
			"0x72699ba15cc734f8db874fa9652c8de12093f187": {
				"ticker": "GRO",
				"address": "0x72699ba15CC734F8db874fa9652c8DE12093F187",
				"name": "Growth-Peg Token",
				"decimals": 18
			},
			"0x0eb476cc2e6b62003b6f497f1434adbe48a2069f": {
				"ticker": "APE",
				"address": "0x0eb476CC2E6b62003B6f497F1434aDbe48A2069f",
				"name": "Little Ape",
				"decimals": 10
			},
			"0x446d3366bcad8b722ae1d46960f4e2459f9a649f": {
				"ticker": "BANANA",
				"address": "0x446D3366bCAd8b722Ae1d46960F4e2459F9A649f",
				"name": "BANANAVAS",
				"decimals": 18
			},
			"0xeeffba5821e3fd475334e93f946d0b11a3bb579d": {
				"ticker": "APL",
				"address": "0xeEFFba5821E3FD475334E93f946D0b11a3BB579d",
				"name": "Pineapple Project",
				"decimals": 12
			},
			"0x936692f4e20fcb4562af4ed3178a90b362c00a6c": {
				"ticker": "CMPC",
				"address": "0x936692F4e20fCb4562af4ED3178A90b362C00A6c",
				"name": "Chimp Classic",
				"decimals": 12
			},
			"0xef229be156bac37c9fbdd23bc30d33f2d7250fa1": {
				"ticker": "GCH",
				"address": "0xeF229bE156Bac37c9fbDd23bC30D33F2D7250fA1",
				"name": "Treasure Glitch",
				"decimals": 11
			},
			"0xf7713040561ad903560a1f766006955079a5416f": {
				"ticker": "BATTLE",
				"address": "0xF7713040561aD903560A1f766006955079a5416f",
				"name": "Battle Nodes",
				"decimals": 9
			},
			"0xd086e2b72235c1cf9f05d8c5accccfc06f6bc1be": {
				"ticker": "FKI",
				"address": "0xd086E2B72235c1cf9F05D8C5ACccCfC06f6bC1Be",
				"name": "Floki King",
				"decimals": 10
			},
			"0xdb9438f5e3afa5ced760be9692604c3c5ab816d1": {
				"ticker": "$BURN",
				"address": "0xdB9438f5E3Afa5cED760BE9692604c3c5Ab816d1",
				"name": "Snow Burn",
				"decimals": 18
			},
			"0x5c4b6bcbfbda4c6870a16cba6635c9844934bb4e": {
				"ticker": "MOON",
				"address": "0x5C4b6BCbFbda4c6870A16Cba6635c9844934bb4E",
				"name": "Exo Moon",
				"decimals": 10
			},
			"0x32d03c74db8eeb7c204e5b24811cef83911d64af": {
				"ticker": "STAR",
				"address": "0x32d03C74dB8Eeb7C204e5b24811CEF83911D64Af",
				"name": "Baby Star",
				"decimals": 11
			},
			"0xe6c431a35753b6792abb93ce1f09b2df2ede0ec4": {
				"ticker": "ZAP",
				"address": "0xE6C431a35753B6792aBb93Ce1F09b2dF2EDe0ec4",
				"name": "Thunder Monster",
				"decimals": 12
			},
			"0x3013ca82f226f73e14de75e30212a02b46e7e894": {
				"ticker": "WINGS",
				"address": "0x3013Ca82f226f73e14de75e30212A02b46e7E894",
				"name": "Baby Angel",
				"decimals": 12
			},
			"0xd1fe8a04110ce0ec0492dd58d8e8289e27e27799": {
				"ticker": "Dice",
				"address": "0xd1fE8A04110Ce0ec0492dD58D8e8289E27E27799",
				"name": "Dice DAO",
				"decimals": 18
			},
			"0xd74214293476ab802518f0c406c60d0db4aeb543": {
				"ticker": "MRS",
				"address": "0xD74214293476ab802518f0c406C60d0dB4Aeb543",
				"name": "Mars Finance",
				"decimals": 9
			},
			"0xf372ae40c3777697e3ff7bb01b05e1c625f10c34": {
				"ticker": "FANL",
				"address": "0xF372ae40c3777697E3fF7bB01B05E1c625f10c34",
				"name": "Fantom Ledger",
				"decimals": 12
			},
			"0x8177dd60255479a70b02a310be672c941aa94f40": {
				"ticker": "EVERSHARES",
				"address": "0x8177Dd60255479A70B02a310bE672C941Aa94f40",
				"name": "EVEREST Shares",
				"decimals": 18
			},
			"0xdf7edaefff07fbdaaace1f7781b230b5c793213a": {
				"ticker": "MOON",
				"address": "0xDF7EDAEfFF07FbdaAACe1f7781B230b5c793213A",
				"name": "MoonDAO t.me/MoonFinancial",
				"decimals": 18
			},
			"0xb3356c885c433b1ce85c947b764b0b34b3b9bc02": {
				"ticker": "SBAA",
				"address": "0xb3356C885C433b1cE85c947B764B0B34b3b9Bc02",
				"name": "Shiba AI",
				"decimals": 10
			},
			"0x95923f63da09b4f7520f7c65a31f318d8228b744": {
				"ticker": "FOOK",
				"address": "0x95923F63dA09B4f7520f7C65a31F318D8228B744",
				"name": "FOOK Token",
				"decimals": 18
			},
			"0xfc32516c6df1d346b56c4e477125a5d4a0ca56a2": {
				"ticker": "AAA",
				"address": "0xfc32516C6Df1d346b56c4E477125A5d4A0cA56A2",
				"name": "AAA",
				"decimals": 18
			},
			"0x5684a087c739a2e845f4aaaabf4fbd261edc2be8": {
				"ticker": "LF",
				"address": "0x5684a087C739A2e845F4AaAaBf4FBd261edc2bE8",
				"name": "Life",
				"decimals": 9
			},
			"0x27b7ec611981e60196a07ee2c753ee75cd4dea2f": {
				"ticker": "JPEG",
				"address": "0x27B7Ec611981E60196a07eE2C753Ee75Cd4deA2F",
				"name": "JPEG",
				"decimals": 18
			},
			"0x424b3058ac9a6ec8c0466aa1c51efcc659f46a8f": {
				"ticker": "MARS",
				"address": "0x424B3058aC9a6ec8C0466AA1c51EfCC659F46A8f",
				"name": "Sushi Mars",
				"decimals": 11
			},
			"0x48fa48be6deb2ded6a707f977efba5870d98ebda": {
				"ticker": "PIN",
				"address": "0x48fa48be6DeB2DeD6a707f977Efba5870D98eBdA",
				"name": "Pineapple Finance",
				"decimals": 9
			},
			"0x8f22c985b1ed65f3ebff9cd9980e04d53f8f4a3e": {
				"ticker": "CMP",
				"address": "0x8F22c985B1ED65F3EbFF9cD9980e04D53F8F4A3E",
				"name": "Chimp Project",
				"decimals": 12
			},
			"0xc5b8f5c6a7ee9aa5d2e4d83af84bc2b6e73f10b2": {
				"ticker": "MGLD",
				"address": "0xC5B8F5c6A7eE9aa5d2E4d83AF84Bc2B6E73F10b2",
				"name": "Meta Gold",
				"decimals": 11
			},
			"0x1398f82099c386c9ecd3abe9d332c1043e554355": {
				"ticker": "MRS",
				"address": "0x1398f82099C386C9ecd3AbE9d332C1043E554355",
				"name": "Crypto Mars",
				"decimals": 12
			},
			"0x483416eb3afa601b9c6385f63cec0c82b6abf1fb": {
				"ticker": "SKILL",
				"address": "0x483416eB3aFA601B9C6385f63CeC0C82B6aBf1fb",
				"name": "Skill Token",
				"decimals": 18
			},
			"0x06db7528dc7ed2865ad433c5f11b8d9397a71722": {
				"ticker": "GOLS",
				"address": "0x06dB7528DC7ed2865ad433C5F11b8D9397A71722",
				"name": "Gold SV",
				"decimals": 11
			},
			"0xb83db30192e06541b844790430342c0078fceecf": {
				"ticker": "GOS",
				"address": "0xB83db30192E06541b844790430342C0078fCeECF",
				"name": "Ghost Monster",
				"decimals": 9
			},
			"0xc34c4e49b90afa4cc9be7f0c15f3dd46f52c7cae": {
				"ticker": "PANB",
				"address": "0xc34C4E49b90aFa4CC9Be7f0C15F3DD46f52C7cae",
				"name": "Panda Block",
				"decimals": 10
			},
			"0x67316b4e8842bf5089ee3909e27b17f7f2abfd73": {
				"ticker": "CORK",
				"address": "0x67316b4e8842BF5089ee3909E27b17f7F2ABfD73",
				"name": "Corkscrew",
				"decimals": 18
			},
			"0xb1030c8ec88ba38ea31c3c63564cc3dba70ad122": {
				"ticker": "WHALE",
				"address": "0xb1030c8ec88bA38EA31C3c63564cC3dba70ad122",
				"name": "OMG Whale",
				"decimals": 12
			},
			"0xe16003347f9793299dddcb6dddfe6a6f5156f6b8": {
				"ticker": "DAR",
				"address": "0xe16003347F9793299DdDcb6DDdFE6A6f5156F6b8",
				"name": "Dalarnia",
				"decimals": 18
			},
			"0xbf217ff020b463b81af354caa8c405836d168688": {
				"ticker": "STAR",
				"address": "0xbF217Ff020b463B81AF354caA8C405836D168688",
				"name": "Star Finance",
				"decimals": 9
			},
			"0xc7965cb57377e7e89b01a565010a3be0cb3e79e1": {
				"ticker": "ANG",
				"address": "0xc7965cb57377E7E89B01A565010A3Be0Cb3E79e1",
				"name": "Treasure Angel",
				"decimals": 12
			},
			"0x50c72103940d419fb64448f258f7eabba784f84b": {
				"ticker": "DLQ",
				"address": "0x50c72103940D419FB64448F258f7EAbbA784F84B",
				"name": "Deliq",
				"decimals": 18
			},
			"0x5e83b78b2f0239f8eb3dbf5d245965d0d6268422": {
				"ticker": "Emerald",
				"address": "0x5E83B78B2F0239f8Eb3DBF5d245965D0D6268422",
				"name": "EmeraldNodes",
				"decimals": 18
			},
			"0x1ca3b9875898a12df3e0cc2dfe005f29244b2a44": {
				"ticker": "CHIF",
				"address": "0x1ca3B9875898A12df3E0CC2dfE005F29244b2A44",
				"name": "Hachi Fund",
				"decimals": 11
			},
			"0x2b99f07dde6f9c32383a9bf25ead245c036e8a66": {
				"ticker": "DOG",
				"address": "0x2B99f07DdE6f9C32383A9BF25eAd245C036e8A66",
				"name": "Doge Swap",
				"decimals": 12
			},
			"0x271feea623d258c2c9e5b546cceda3c697fc1c21": {
				"ticker": "APL",
				"address": "0x271feeA623D258c2C9E5B546cceDA3c697fc1c21",
				"name": "Treasure Apple",
				"decimals": 12
			},
			"0x966bfcd58e029033721b544e62e7703b199b179f": {
				"ticker": "MARS",
				"address": "0x966BFCd58e029033721b544e62E7703B199B179f",
				"name": "Mars Protocol",
				"decimals": 12
			},
			"0xf0bf6b50c12b7558953074ccc84e960041d61280": {
				"ticker": "MSBA",
				"address": "0xF0Bf6B50c12b7558953074CCc84E960041d61280",
				"name": "Mini Shiba",
				"decimals": 11
			},
			"0x986c9b4d317bc7251d211025219a6ee4bd80ec23": {
				"ticker": "EXPO",
				"address": "0x986C9B4d317bC7251d211025219A6Ee4bd80Ec23",
				"name": "Exponential Capital",
				"decimals": 9
			},
			"0xd837d67d4d54170e6832cb2d4843c08b7610946e": {
				"ticker": "INFI",
				"address": "0xd837d67D4d54170e6832cB2D4843C08b7610946E",
				"name": "InfiniteNodes",
				"decimals": 18
			},
			"0xa6a44752c72e527b83d218a6b8986f9e3d2adab8": {
				"ticker": "XOMB",
				"address": "0xA6a44752c72E527B83D218a6b8986f9e3D2ADab8",
				"name": "XOMB Token",
				"decimals": 18
			},
			"0xc98588ffbf80c0c8717d8e328cc9363ece9e6c82": {
				"ticker": "SHBB",
				"address": "0xc98588FFbf80C0C8717D8E328cc9363ece9E6C82",
				"name": "Shiba Block",
				"decimals": 9
			},
			"0x5a7f9e087a13b922881c5e46fea622369c8f69d3": {
				"ticker": "DFIAT",
				"address": "0x5a7f9e087a13B922881C5e46fEa622369c8f69D3",
				"name": "DeFiato Native Token",
				"decimals": 18
			},
			"0x6e08fc704d097c153ef67d17ecd3251a2cc23931": {
				"ticker": "16888",
				"address": "0x6E08Fc704D097c153EF67d17eCD3251a2Cc23931",
				"name": "16888",
				"decimals": 18
			},
			"0x490bf3abcab1fb5c88533d850f2a8d6d38298465": {
				"ticker": "PLAYMATES",
				"address": "0x490bf3ABcAb1fB5c88533d850F2a8d6D38298465",
				"name": "Playmates",
				"decimals": 18
			},
			"0x9bdaed91078019ce1381527607ac8ded1039f1de": {
				"ticker": "MCMP",
				"address": "0x9BdAED91078019cE1381527607ac8DEd1039F1dE",
				"name": "Meta Chimp",
				"decimals": 9
			},
			"0xd0d6f95740e3bc585c4628a25f6255a0dea79fc1": {
				"ticker": "FKI",
				"address": "0xd0d6F95740E3bc585C4628a25F6255a0dEa79Fc1",
				"name": "Wrapped Floki",
				"decimals": 11
			},
			"0x8a95a8328328182997e1000f27c9ae8bc8aa41b8": {
				"ticker": "ZAPN",
				"address": "0x8a95A8328328182997E1000F27c9Ae8Bc8aA41B8",
				"name": "Thunder NET",
				"decimals": 10
			},
			"0xa13ade307c8782929b3b039be93cdf8b2e8de54f": {
				"ticker": "SPND",
				"address": "0xA13Ade307c8782929B3b039be93cdf8B2E8de54f",
				"name": "Safe Panda",
				"decimals": 12
			},
			"0x4690700b5b1590f48a87f98ce1bbba6e0b2315a7": {
				"ticker": "CRG",
				"address": "0x4690700B5b1590F48A87f98ce1bBBa6e0B2315A7",
				"name": "Corgi Ledger",
				"decimals": 9
			},
			"0xfe5e5c4223030222c3c57490949627a2bc2f7fe1": {
				"ticker": "GEM",
				"address": "0xFE5e5c4223030222c3c57490949627A2bC2F7fe1",
				"name": "GemNodesV2",
				"decimals": 18
			},
			"0x100cc3a819dd3e8573fd2e46d1e66ee866068f30": {
				"ticker": "DCAU",
				"address": "0x100Cc3a819Dd3e8573fD2E46D1E66ee866068f30",
				"name": "Dragon Crypto Aurum",
				"decimals": 18
			},
			"0xbed2eb51431529fc7afe314be484fe874e0c1c25": {
				"ticker": "INFI",
				"address": "0xBed2eB51431529Fc7afe314bE484fE874e0C1C25",
				"name": "InfiniteNodes",
				"decimals": 18
			},
			"0xe0474c15bc7f8213ee5bfb42f9e68b2d6be2e136": {
				"ticker": "TOKEN",
				"address": "0xE0474c15BC7f8213eE5bfB42f9E68B2d6BE2e136",
				"name": "Token",
				"decimals": 9
			},
			"0xbc3bff85f09bf5fb045921ae8361ac2dd919a774": {
				"ticker": "CHIN",
				"address": "0xBC3bFF85F09bF5fB045921AE8361ac2dd919a774",
				"name": "Hachi NET",
				"decimals": 9
			},
			"0xeefc07ba44225cde700580ce7221c792da0b72f7": {
				"ticker": "OFKI",
				"address": "0xeEFc07BA44225CdE700580Ce7221c792Da0b72F7",
				"name": "OMG Floki",
				"decimals": 10
			},
			"0xb55d32f314fe08bc3dbba9b6af9cef969833a13f": {
				"ticker": "DOGE",
				"address": "0xb55D32F314Fe08BC3Dbba9b6AF9cef969833A13f",
				"name": "Project Doge",
				"decimals": 10
			},
			"0x564e88de3f506d665828c2b164ac6b9ea8b92f9c": {
				"ticker": "UNIDAO",
				"address": "0x564e88dE3F506D665828C2B164Ac6b9ea8B92f9c",
				"name": "UnicornDAO",
				"decimals": 9
			},
			"0x3e86a5728689afc03c7aeb4d8c8e2853c450a445": {
				"ticker": "QGOL",
				"address": "0x3E86A5728689aFc03C7aEb4d8C8E2853C450a445",
				"name": "Queen Gold",
				"decimals": 12
			},
			"0xeb9e93e71c515ec2095e5b10ae0636843d39a409": {
				"ticker": "SPECTA",
				"address": "0xEB9E93E71C515Ec2095e5B10aE0636843d39a409",
				"name": "SpectreFi",
				"decimals": 18
			},
			"0x6deec9094a5856cba9c6b75f35a6a25ed7c28e76": {
				"ticker": "GLTCH",
				"address": "0x6dEeC9094A5856Cba9c6b75f35a6a25ed7c28E76",
				"name": "Queen Glitch",
				"decimals": 10
			},
			"0x5325c51ed80c7f26c8137f1723ada12a1eca5ee1": {
				"ticker": "APL",
				"address": "0x5325C51ED80c7F26c8137F1723Ada12A1eca5eE1",
				"name": "Apple Farm",
				"decimals": 11
			},
			"0xbd100d061e120b2c67a24453cf6368e63f1be056": {
				"ticker": "iDYP",
				"address": "0xBD100d061E120b2c67A24453CF6368E63f1Be056",
				"name": "iDeFiYieldProtocol",
				"decimals": 18
			},
			"0xe677bdc4f5ac7ef7701f4199f25abad34d41fd7f": {
				"ticker": "SNRW",
				"address": "0xe677bdc4f5AC7EF7701F4199f25aBAD34d41fd7F",
				"name": "SNRW.e",
				"decimals": 18
			},
			"0x7f45ddb2daa41e781da537f2481bbfef1361f18e": {
				"ticker": "BMSTR",
				"address": "0x7F45DDB2dAa41E781DA537F2481bbFEF1361f18E",
				"name": "Bit Master",
				"decimals": 9
			},
			"0x42a76fabb96c5265c96cafc1c067da0ac355a984": {
				"ticker": "DGEI",
				"address": "0x42A76fABb96C5265C96caFC1C067DA0aC355A984",
				"name": "Doge Inu",
				"decimals": 10
			},
			"0xb90d7a1f521bb17241a29e7d4ecdbd0bb39a1e41": {
				"ticker": "SDGE",
				"address": "0xb90D7A1F521bb17241a29E7d4ECDbD0bB39a1e41",
				"name": "Safe Doge",
				"decimals": 11
			},
			"0x5344d1b55ddc853525fd3cf4e897ee403643014c": {
				"ticker": "CORGI",
				"address": "0x5344d1b55dDc853525FD3CF4E897Ee403643014C",
				"name": "Queen Corgi",
				"decimals": 11
			},
			"0xb189ad52dad0b179a13713206064c69e6b26bea7": {
				"ticker": "MARS",
				"address": "0xB189Ad52dAD0b179A13713206064c69E6b26bea7",
				"name": "Wrapped Mars",
				"decimals": 9
			},
			"0x9f88201c24889760356fe213935ead6bcc8109e6": {
				"ticker": "DOGE",
				"address": "0x9f88201c24889760356FE213935eAD6bcc8109e6",
				"name": "Baby Doge",
				"decimals": 11
			},
			"0x3aa5e6c7cf7284f9e1c0a0eb9074a06e9486fd1f": {
				"ticker": "CORGI",
				"address": "0x3aa5e6C7Cf7284F9E1C0A0eb9074a06e9486fD1F",
				"name": "Zen Corgi",
				"decimals": 11
			},
			"0x6ec4157a7b83859c7429339ad1a2c09e59c761f7": {
				"ticker": "uatASTRO",
				"address": "0x6Ec4157a7B83859C7429339aD1a2C09e59c761f7",
				"name": "100 Days Ventures",
				"decimals": 18
			},
			"0xe74167123d0693750978691fa2d123a873d5dcb2": {
				"ticker": "APL",
				"address": "0xE74167123D0693750978691FA2D123A873D5DCb2",
				"name": "Safe Apple",
				"decimals": 11
			},
			"0x1da4ccfe0a21cfea0380c6b4b1e45c60c92989a2": {
				"ticker": "WINGS",
				"address": "0x1DA4CcFE0A21cfeA0380c6B4B1E45c60c92989a2",
				"name": "Angel Inu",
				"decimals": 12
			},
			"0xc0a99af0eaab4443f62301067e26e9d76c4fbb03": {
				"ticker": "GCH",
				"address": "0xc0a99aF0eaab4443F62301067E26e9d76C4fbB03",
				"name": "Baby Glitch",
				"decimals": 12
			},
			"0xe89aea80846e8747b765dea1bf948d7f5e8bf130": {
				"ticker": "STAR",
				"address": "0xe89Aea80846e8747B765DEa1Bf948d7f5e8BF130",
				"name": "Zen Star",
				"decimals": 12
			},
			"0xa9485b2a4942c196d20ad8b29e14889f1713b915": {
				"ticker": "SPND",
				"address": "0xa9485B2A4942c196D20aD8B29e14889f1713b915",
				"name": "Sushi Panda",
				"decimals": 10
			},
			"0x9e05dd0aaa1d119f0d37256bcede0fd8e894b474": {
				"ticker": "MAPL",
				"address": "0x9e05dd0AaA1d119F0d37256bCEde0FD8E894b474",
				"name": "Meta Pineapple",
				"decimals": 10
			},
			"0x77c61ab75a45987a12d3807840f207012d470828": {
				"ticker": "DGEI",
				"address": "0x77c61aB75a45987a12D3807840f207012d470828",
				"name": "Doge Inu",
				"decimals": 10
			},
			"0xac4a3af5c2ee5a71ab79980f84a7709a29d2f032": {
				"ticker": "STAR",
				"address": "0xAc4A3af5c2EE5A71Ab79980F84a7709a29D2F032",
				"name": "Wrapped Star",
				"decimals": 9
			},
			"0xcc752ad59f9b7dcdacd7357fc1d380a2a4988452": {
				"ticker": "GLT",
				"address": "0xcc752aD59f9B7DcdacD7357Fc1d380A2a4988452",
				"name": "Little Glitch",
				"decimals": 9
			},
			"0xb1a7eb6ce5a6963cd4102c27fe6e122f5e9fcca0": {
				"ticker": "SHIB",
				"address": "0xB1A7Eb6Ce5a6963cd4102C27FE6e122F5e9FCCa0",
				"name": "Safe Shiba",
				"decimals": 9
			},
			"0x2ef579b2fd108f2bbf1cd360ad40d0075af35095": {
				"ticker": "CHIMP",
				"address": "0x2Ef579B2Fd108f2bBF1CD360aD40d0075Af35095",
				"name": "Chimp Markets",
				"decimals": 12
			},
			"0x4519b12c48335ccddf7522a235051916e25c5ef9": {
				"ticker": "APL",
				"address": "0x4519b12C48335cCDDF7522a235051916E25c5EF9",
				"name": "Apple Project",
				"decimals": 9
			},
			"0xbe9a3da3c3d0f75562cac0280770319f68fe1138": {
				"ticker": "WHALE",
				"address": "0xBe9a3dA3c3D0F75562caC0280770319F68FE1138",
				"name": "Whale Block",
				"decimals": 12
			},
			"0x3eefb18003d033661f84e48360ebecd181a84709": {
				"ticker": "ISA",
				"address": "0x3EeFb18003D033661f84e48360eBeCD181A84709",
				"name": "Islander",
				"decimals": 18
			},
			"0x3e7ad25bbda1fe2a3b7e82f2ee490bc5e5a7166c": {
				"ticker": "MOON",
				"address": "0x3E7ad25BBdA1fE2A3b7e82F2EE490bC5e5A7166c",
				"name": "Moon Inu",
				"decimals": 10
			},
			"0x4359f2f244cb5dafd765a881d3116f6b79652712": {
				"ticker": "INFI",
				"address": "0x4359F2f244CB5DaFd765A881d3116f6B79652712",
				"name": "InfiniteNodes",
				"decimals": 18
			},
			"0x84ae4affb9814a2f7d37771def45c98a34100f8f": {
				"ticker": "WANG",
				"address": "0x84aE4AFfb9814a2F7d37771Def45C98A34100F8F",
				"name": "Wrapped Angel",
				"decimals": 9
			},
			"0x57d2eadb0a2eb66458fa02fae747853223eeb00c": {
				"ticker": "GHST",
				"address": "0x57D2EAdb0A2Eb66458FA02Fae747853223eEb00c",
				"name": "Ghost AVAX",
				"decimals": 10
			},
			"0xeb8343d5284caec921f035207ca94db6baaacbcd": {
				"ticker": "ECD",
				"address": "0xeb8343D5284CaEc921F035207ca94DB6BAaaCBcd",
				"name": "Echidna Token",
				"decimals": 18
			},
			"0x2964b1342998d6b4194c42a92d4d79c967dadbbb": {
				"ticker": "MOON",
				"address": "0x2964b1342998d6B4194c42a92D4D79C967DADbbB",
				"name": "Zen Moon",
				"decimals": 10
			},
			"0xa131a1f5d2bdb1721b8798aeac1b48b9f24d08a3": {
				"ticker": "SBA",
				"address": "0xa131a1f5D2Bdb1721b8798aEaC1b48B9f24d08A3",
				"name": "Exo Shiba",
				"decimals": 9
			},
			"0x941af7ff7c13d696185e99a1a551a451c072b070": {
				"ticker": "BM",
				"address": "0x941aF7fF7c13D696185E99A1a551a451C072b070",
				"name": "Big Moon",
				"decimals": 18
			},
			"0x9e15f045e44ea5a80e7fbc193a35287712cc5569": {
				"ticker": "3ULL",
				"address": "0x9e15f045e44ea5a80e7Fbc193A35287712Cc5569",
				"name": "3ULL v2",
				"decimals": 18
			},
			"0x8c2ada5973ed2b3052b1c4dc6b7486d8dd635925": {
				"ticker": "HCI",
				"address": "0x8C2aDA5973Ed2b3052b1C4dC6B7486d8Dd635925",
				"name": "Hachi NET",
				"decimals": 12
			},
			"0x28c4b5d26eeb1bfe3b2d1ab648c220ba5cb68b51": {
				"ticker": "GEM",
				"address": "0x28c4b5D26eeB1BfE3b2D1aB648C220BA5CB68B51",
				"name": "Gem Finance",
				"decimals": 9
			},
			"0x554373ab855a039b2f0ea5e1875e0bc8520714b7": {
				"ticker": "AKITA",
				"address": "0x554373aB855A039B2F0Ea5e1875e0Bc8520714b7",
				"name": "Akita X",
				"decimals": 10
			},
			"0x09f46a1e4d50cc3b3427ecbee53a5589d14b1c31": {
				"ticker": "LPND",
				"address": "0x09F46a1E4D50cC3B3427EcbeE53A5589d14B1C31",
				"name": "Little Panda",
				"decimals": 9
			},
			"0xdc3f0a90305a48ce62401a0fe590015bc06692b3": {
				"ticker": "PAPE",
				"address": "0xdC3f0a90305A48ce62401A0FE590015bc06692b3",
				"name": "Project Ape",
				"decimals": 12
			},
			"0x6e986bdb19feba049a1be609d08e1e8bbb601b40": {
				"ticker": "MRSD",
				"address": "0x6E986Bdb19fEBA049A1be609d08E1E8BBb601B40",
				"name": "Mars Dapp",
				"decimals": 12
			},
			"0xaf15f5d7577990d67f44e67e654273ae9d20255d": {
				"ticker": "ZPAN",
				"address": "0xAf15f5D7577990d67F44e67e654273aE9D20255D",
				"name": "Zen Panda",
				"decimals": 9
			},
			"0x88eccf2f72fce414baa07869e26b9fc47ca7be82": {
				"ticker": "CHIMP",
				"address": "0x88EccF2f72fCe414BAa07869e26b9fC47CA7BE82",
				"name": "Wrapped Chimp",
				"decimals": 10
			},
			"0x8dd3c396ab269c7c4c9fd8ff96210228d090ec06": {
				"ticker": "STR",
				"address": "0x8DD3C396ab269c7c4c9FD8ff96210228d090eC06",
				"name": "Star Treasure",
				"decimals": 10
			},
			"0x7fbfd0007c0c27df419ff2363b01ebb90da96ba2": {
				"ticker": "FLOKI",
				"address": "0x7FbFd0007c0C27Df419fF2363b01EBb90dA96BA2",
				"name": "Floki Queen",
				"decimals": 10
			},
			"0xc4bca582a4016b68174877e0adf09dfe07b68946": {
				"ticker": "DEK",
				"address": "0xC4BcA582a4016B68174877E0aDf09dfE07B68946",
				"name": "DEK Nodes",
				"decimals": 18
			},
			"0xc87d897edd059e99bddbb240e575a2401b0b993e": {
				"ticker": "WHALE",
				"address": "0xC87D897EDd059e99BdDbb240E575a2401B0b993e",
				"name": "Whale Chain",
				"decimals": 12
			},
			"0x8dff732af9508205c5aca7439c1e328a5cfe20b5": {
				"ticker": "DFIAT",
				"address": "0x8dFF732AF9508205C5aCA7439c1E328a5Cfe20b5",
				"name": "DeFiato",
				"decimals": 18
			},
			"0x5052ad88fe5a8862927d724d1c8fc27f0f42be2f": {
				"ticker": "EGGS",
				"address": "0x5052AD88Fe5a8862927d724d1C8fc27f0F42be2f",
				"name": "$EGGS",
				"decimals": 18
			},
			"0x34180c1bf3f824bc16347c013ff6701c2f6fc82d": {
				"ticker": "MOKA",
				"address": "0x34180c1bF3F824BC16347c013fF6701c2f6fC82D",
				"name": "MOKA Finance",
				"decimals": 18
			},
			"0x28324b9f79996e48f4db20aa5a2bea65ec7c2ce4": {
				"ticker": "QGLT",
				"address": "0x28324b9f79996E48F4Db20aa5a2BEA65ec7c2Ce4",
				"name": "Queen Glitch",
				"decimals": 12
			},
			"0x8e80fa7bfc0f46da60d074b6ab90b0638600afc5": {
				"ticker": "FKI",
				"address": "0x8E80fa7BFc0F46Da60d074B6ab90B0638600aFc5",
				"name": "Exo Floki",
				"decimals": 11
			},
			"0xd49bcd3f8206b57c2cc4c93da6a9322007586e58": {
				"ticker": "AKA",
				"address": "0xd49bcd3F8206B57c2cc4c93dA6A9322007586E58",
				"name": "Akita Ledger",
				"decimals": 9
			},
			"0x3eb8eb2a9d740bce41dd7d237d06383df333cb30": {
				"ticker": "WHALE",
				"address": "0x3eb8Eb2A9D740bCe41Dd7D237d06383df333Cb30",
				"name": "Whale AI",
				"decimals": 12
			},
			"0x11cdfcdf07cd1cfd54dbf7a90aa42e2cf50ce412": {
				"ticker": "APEM",
				"address": "0x11cDfCdF07Cd1cfD54dbF7A90aA42e2CF50Ce412",
				"name": "Ape Monster",
				"decimals": 10
			},
			"0x0382a2df923c5a784fb44189ab521b7c3b6350b7": {
				"ticker": "OELN",
				"address": "0x0382a2df923c5A784FB44189aB521B7C3B6350B7",
				"name": "OMG Elon",
				"decimals": 10
			},
			"0xafc4d521df3c0566d61931f81f02f1a525bad04d": {
				"ticker": "WSDQ",
				"address": "0xafc4d521DF3C0566d61931F81f02f1A525Bad04D",
				"name": "Wasdaq",
				"decimals": 9
			},
			"0x79817307e7d3ced2a53ffc0c8e6eaeadfbdab3f4": {
				"ticker": "CMP",
				"address": "0x79817307E7d3ced2A53FfC0C8E6eAEAdFBDaB3F4",
				"name": "Treasure Chimp",
				"decimals": 12
			},
			"0x494bae744db45f81dcb1a66caf8f111e5f8d0f87": {
				"ticker": "TOMI",
				"address": "0x494Bae744dB45f81Dcb1A66CaF8F111E5F8D0f87",
				"name": "Fantom Inu",
				"decimals": 12
			},
			"0x3e95540c9e64fcac38c1b10a6045e3e8c55ac139": {
				"ticker": "PIP",
				"address": "0x3e95540C9e64FCaC38C1B10a6045e3E8c55ac139",
				"name": "Sandra",
				"decimals": 18
			},
			"0x7315d580555e18e855f2e8d616c7753c588794a4": {
				"ticker": "GTHB",
				"address": "0x7315D580555e18e855F2e8D616C7753C588794a4",
				"name": "Glitch Beast",
				"decimals": 12
			},
			"0xe73200ffb8ba740a40e208ea842512e17427bfcc": {
				"ticker": "HYPE",
				"address": "0xe73200FfB8ba740A40E208Ea842512E17427bFcc",
				"name": "Hype Club",
				"decimals": 18
			},
			"0xf6dbd40c56c9ee9d34f862d30257189b8ce31697": {
				"ticker": "PNPL",
				"address": "0xf6DbD40c56c9ee9d34F862D30257189B8cE31697",
				"name": "Treasure Pineapple",
				"decimals": 11
			},
			"0x69e329e273cd2ca1f94441cd3f34cf98114ffc9c": {
				"ticker": "ZAPC",
				"address": "0x69E329e273cD2Ca1F94441Cd3F34cF98114fFC9c",
				"name": "Thunder Coin",
				"decimals": 9
			},
			"0xee61874594e174c2db2a8df6edac591d32b75570": {
				"ticker": "APE",
				"address": "0xeE61874594e174c2Db2a8dF6EDAc591d32B75570",
				"name": "Project Ape",
				"decimals": 11
			},
			"0xcb856090430b53fe67a56d5a52ccd608fb30688c": {
				"ticker": "STR",
				"address": "0xCB856090430B53fE67A56d5A52CcD608fb30688c",
				"name": "Crypto Star",
				"decimals": 11
			},
			"0x60964c6c540e64177971becc7d4af771ec3c34ac": {
				"ticker": "GLT",
				"address": "0x60964C6C540E64177971bECc7D4Af771EC3c34Ac",
				"name": "Glitch Token",
				"decimals": 10
			},
			"0xb599c3590f42f8f995ecfa0f85d2980b76862fc1": {
				"ticker": "UST",
				"address": "0xb599c3590F42f8F995ECfa0f85D2980B76862fc1",
				"name": "UST (Wormhole)",
				"decimals": 6
			},
			"0xdbf31df14b66535af65aac99c32e9ea844e14501": {
				"ticker": "renBTC",
				"address": "0xDBf31dF14B66535aF65AaC99C32e9eA844e14501",
				"name": "renBTC",
				"decimals": 8
			},
			"0xe8dc7b37e8e5fadaebcefb85b80aa42de4e86a09": {
				"ticker": "CHI",
				"address": "0xE8DC7b37e8e5FADAeBceFB85b80AA42dE4e86a09",
				"name": "Chimp Protocol",
				"decimals": 11
			},
			"0x78cb4d5fe759bbb559b4f478f1fe2e6ce14cabdc": {
				"ticker": "ESBA",
				"address": "0x78Cb4D5fe759BBB559B4F478f1Fe2e6CE14cABdc",
				"name": "Exo Shiba",
				"decimals": 10
			},
			"0xb0e50deeb6119107b76bf9c4c6ca7dd898972fb0": {
				"ticker": "PANS",
				"address": "0xb0E50deEb6119107B76bf9c4c6cA7DD898972fB0",
				"name": "Panda Starter",
				"decimals": 11
			},
			"0x302abf007c2045f1bc0867a4b7abaae2152e0eb3": {
				"ticker": "RIBT",
				"address": "0x302Abf007C2045F1bC0867a4b7abaaE2152e0EB3",
				"name": "Ribbit",
				"decimals": 18
			},
			"0x39db7b91237396ac694304589f1282ce026a4776": {
				"ticker": "QFLK",
				"address": "0x39dB7b91237396AC694304589f1282ce026a4776",
				"name": "Queen Floki",
				"decimals": 11
			},
			"0xbc011b7ad5b89aeac1384ea18f5ccedd3ce8eaa2": {
				"ticker": "TNDR",
				"address": "0xBC011B7aD5b89aeAc1384ea18F5cceDd3ce8eaa2",
				"name": "Thunder Rocket",
				"decimals": 12
			},
			"0x865640db386ba28f01f801d4a1067092540f507f": {
				"ticker": "PNDS",
				"address": "0x865640db386BA28f01f801d4A1067092540F507F",
				"name": "Panda Starter",
				"decimals": 10
			},
			"0xb2558eb2f546e7da9d2a7efc4b1437db595f5a45": {
				"ticker": "FLOKI",
				"address": "0xB2558EB2F546e7Da9D2a7efc4b1437Db595f5a45",
				"name": "Floki Starter",
				"decimals": 11
			},
			"0x0e73ee704f6783d6e6520af7a6b85209992bf3e4": {
				"ticker": "ANGM",
				"address": "0x0e73EE704F6783d6E6520AF7a6b85209992bF3E4",
				"name": "Angel Monster",
				"decimals": 10
			},
			"0x457708ae5ed2f17025bfdf7edb786cdcd50edc28": {
				"ticker": "SFT",
				"address": "0x457708Ae5ED2f17025BfDF7EDB786cdcd50EdC28",
				"name": "Snow Factory Token",
				"decimals": 18
			},
			"0x9b62172abc8d3edfc40bf93cf48fb03c134b1321": {
				"ticker": "CAKE",
				"address": "0x9b62172ABC8D3EDfc40bf93CF48Fb03c134B1321",
				"name": "Safe Cake",
				"decimals": 10
			},
			"0x9ddcd0d3cff71de6213f9d2a710919beaa2401c0": {
				"ticker": "WHL",
				"address": "0x9ddcD0D3CfF71dE6213f9D2a710919bEaa2401C0",
				"name": "Whale King",
				"decimals": 9
			},
			"0xa0cae7c24b8b47277b76a7555c5db941fc460619": {
				"ticker": "DOGE",
				"address": "0xa0cae7c24b8B47277b76a7555c5Db941Fc460619",
				"name": "Doge Project",
				"decimals": 10
			},
			"0xa699dda0873ea0f8c2c4787656c8a9e60f362793": {
				"ticker": "APEI",
				"address": "0xa699DDA0873eA0f8C2c4787656C8A9E60F362793",
				"name": "APE Island",
				"decimals": 9
			},
			"0xd39688390bf5ce225e23fc549932cda7002451c5": {
				"ticker": "SAPL",
				"address": "0xD39688390BF5CE225E23Fc549932cDA7002451c5",
				"name": "Super Apple",
				"decimals": 10
			},
			"0x19cfd1a6b1cd236b542b561ed26b32d697e5a77f": {
				"ticker": "GMA",
				"address": "0x19cfd1A6b1cD236b542B561eD26b32d697E5a77f",
				"name": "ENIGM",
				"decimals": 9
			},
			"0x007a0b1d92c3697fba5592814110fa8e34b4d769": {
				"ticker": "ANTG",
				"address": "0x007A0B1D92C3697FBa5592814110Fa8E34b4D769",
				"name": "ANTG",
				"decimals": 6
			},
			"0x8ae8be25c23833e0a01aa200403e826f611f9cd2": {
				"ticker": "CRAFT",
				"address": "0x8aE8be25C23833e0A01Aa200403e826F611f9CD2",
				"name": "CRAFT",
				"decimals": 18
			},
			"0x8172f970e660635877291cdb8ca45cbd63b75df4": {
				"ticker": "DOG",
				"address": "0x8172f970E660635877291CDB8Ca45CbD63B75dF4",
				"name": "Doge AVA",
				"decimals": 11
			},
			"0xa7e211da69eec0fee5bf1482f08ff3f663a01922": {
				"ticker": "ELON",
				"address": "0xA7e211dA69eEc0Fee5Bf1482F08FF3f663a01922",
				"name": "Zen Elon",
				"decimals": 9
			},
			"0xa9d15f89af194f40e2f64fad5d6eebf490988456": {
				"ticker": "YUM",
				"address": "0xa9d15f89AF194F40E2f64FAd5D6Eebf490988456",
				"name": "Cake Coin",
				"decimals": 9
			},
			"0xe4ea8aef5b7151c29a2291883d1bac08a50bb8e9": {
				"ticker": "SHBN",
				"address": "0xe4eA8aEf5B7151c29a2291883d1BaC08A50bb8e9",
				"name": "Shiba NET",
				"decimals": 12
			},
			"0x0a2b9a7da534d1c8523895a4e64c54cdf07000e8": {
				"ticker": "AVAXination",
				"address": "0x0A2b9a7dA534d1C8523895A4E64C54cDF07000e8",
				"name": "AVAXination",
				"decimals": 9
			},
			"0xa33476e3c985a92e7cd7d1e0a4d7c55969845036": {
				"ticker": "FKI",
				"address": "0xA33476E3C985a92E7cD7D1e0a4D7c55969845036",
				"name": "Mini Floki",
				"decimals": 11
			},
			"0x5a8357ab48fa2f963582d4fe2190bd698567c638": {
				"ticker": "AKAM",
				"address": "0x5A8357AB48FA2f963582D4fE2190Bd698567c638",
				"name": "Akita Monster",
				"decimals": 12
			},
			"0x7b3ab7e460e60aff2d8c40dee8a2f0226c95e612": {
				"ticker": "KGHS",
				"address": "0x7B3AB7e460e60afF2d8C40DEE8a2F0226c95e612",
				"name": "King Ghost",
				"decimals": 10
			},
			"0x1339b75ad89ee5cb5463c313e568b2d3301e28d7": {
				"ticker": "CGIS",
				"address": "0x1339b75Ad89EE5Cb5463c313e568b2d3301E28d7",
				"name": "Corgi SV",
				"decimals": 10
			},
			"0x395908aeb53d33a9b8ac35e148e9805d34a555d3": {
				"ticker": "WLRS",
				"address": "0x395908aeb53d33A9B8ac35e148E9805D34A555D3",
				"name": "frozenwalrus.finance",
				"decimals": 18
			},
			"0xdcd2d9b2eb27276c72e9c7a14ceedc8c032bc7d4": {
				"ticker": "STAR",
				"address": "0xDCd2d9b2eB27276C72e9C7a14ceEDc8c032bC7d4",
				"name": "Wrapped Star",
				"decimals": 11
			},
			"0x420a05276c151caa875888eb1acdbc2ab948f57e": {
				"ticker": "THNP",
				"address": "0x420a05276c151CaA875888eb1acDbC2AB948f57e",
				"name": "Thunder Project",
				"decimals": 11
			},
			"0xfd6aa3a644456e3d356f5e7964c061a959b975b3": {
				"ticker": "SZAP",
				"address": "0xFd6aA3a644456E3D356f5e7964c061a959b975B3",
				"name": "Safe Thunder",
				"decimals": 11
			},
			"0xed2b42d3c9c6e97e11755bb37df29b6375ede3eb": {
				"ticker": "HON",
				"address": "0xEd2b42D3C9c6E97e11755BB37df29B6375ede3EB",
				"name": "HonToken",
				"decimals": 18
			},
			"0x8f47416cae600bccf9530e9f3aeaa06bdd1caa79": {
				"ticker": "THOR",
				"address": "0x8F47416CaE600bccF9530E9F3aeaA06bdD1Caa79",
				"name": "THOR v2",
				"decimals": 18
			},
			"0x325a98f258a5732c7b06555603f6af5bc1c17f0a": {
				"ticker": "$ALPHA",
				"address": "0x325a98F258a5732c7b06555603F6aF5BC1C17F0a",
				"name": "$ALPHA",
				"decimals": 9
			},
			"0x30b77348ccce87b3113cf21d876fef9ab28c5526": {
				"ticker": "CTHN",
				"address": "0x30B77348CcCE87B3113Cf21D876FEf9Ab28c5526",
				"name": "Crypto Thunder",
				"decimals": 11
			},
			"0x6db44f4c8b7b03cc2fabebf6b50cfabc447a2476": {
				"ticker": "OTGECE",
				"address": "0x6dB44f4C8B7B03CC2faBEBf6b50CfAbC447A2476",
				"name": "Otgece Node",
				"decimals": 18
			},
			"0x6b1fdc9eef6ab5c4482273f25ea4d2b1e991ab0f": {
				"ticker": "LAP",
				"address": "0x6b1Fdc9eeF6AB5C4482273f25Ea4D2B1e991aB0F",
				"name": "Love and Peace",
				"decimals": 18
			},
			"0x61751a60bc0a6dbf0a56354e87067a3ca5977298": {
				"ticker": "FKII",
				"address": "0x61751A60Bc0A6Dbf0a56354E87067a3cA5977298",
				"name": "Floki INC",
				"decimals": 10
			},
			"0xd2230fe9b914d6546783e8c4df65e2a54407f090": {
				"ticker": "CHI",
				"address": "0xd2230fE9b914d6546783e8C4dF65E2a54407f090",
				"name": "Chimp X",
				"decimals": 12
			},
			"0x77e847345af47e4ae356cba4ff624f0a82ab010e": {
				"ticker": "GCHC",
				"address": "0x77E847345AF47e4AE356CBa4ff624F0A82aB010E",
				"name": "Glitch Cash",
				"decimals": 10
			},
			"0x04c3131a0cb8f8c00e35f6e5ff0e76c13a6b858c": {
				"ticker": "STR",
				"address": "0x04C3131a0cB8f8C00E35f6E5FF0E76C13a6b858C",
				"name": "Super Star",
				"decimals": 9
			},
			"0x6e9342a862411bfc5ddea382c6569f450a81c400": {
				"ticker": "MARS",
				"address": "0x6E9342A862411Bfc5ddeA382c6569f450a81C400",
				"name": "Mars Token",
				"decimals": 12
			},
			"0x087666b0497de39a71eecdcde22b93b0e80924b7": {
				"ticker": "POS",
				"address": "0x087666B0497dE39A71EEcDcDE22b93B0e80924b7",
				"name": "POSEIDON DAO",
				"decimals": 18
			},
			"0xf59f9068633f866b2d62da1b1e931e4cb5d9f5d0": {
				"ticker": "INDEX",
				"address": "0xF59f9068633F866B2D62Da1b1e931E4cB5d9f5D0",
				"name": "Index DAO",
				"decimals": 18
			},
			"0x9d92bdf3f87ef1452023ab049e70bf175a7d58a3": {
				"ticker": "GHSF",
				"address": "0x9D92BdF3F87Ef1452023Ab049E70Bf175a7D58a3",
				"name": "Ghost Farm",
				"decimals": 11
			},
			"0x256ec8ab7f09fbaeea09172f10bea633b5d6a4c5": {
				"ticker": "CORGI",
				"address": "0x256eC8aB7F09FbaeEA09172f10BeA633b5D6a4C5",
				"name": "Corgi Coin",
				"decimals": 11
			},
			"0xe730128ffbd46a11f7036e0040c0988d2411feba": {
				"ticker": "MARS",
				"address": "0xE730128FFBD46A11f7036E0040c0988D2411feBA",
				"name": "Mars Ledger",
				"decimals": 10
			},
			"0xc32bb3f9ef533b0248780cc92997e9643166f395": {
				"ticker": "GLTCH",
				"address": "0xc32bb3f9EF533B0248780cC92997e9643166f395",
				"name": "Glitch Dapp",
				"decimals": 10
			},
			"0x321060d9fe9b20df375a3bc5f14df6c280ad44d8": {
				"ticker": "APL",
				"address": "0x321060D9FE9B20DF375a3Bc5F14df6c280ad44D8",
				"name": "Little Apple",
				"decimals": 9
			},
			"0xf1b6029d893dde0b3a506f032381d6ed55cc9bdf": {
				"ticker": "SCI",
				"address": "0xF1B6029D893ddE0B3A506F032381D6ed55Cc9Bdf",
				"name": "Simply Compound Interest",
				"decimals": 18
			},
			"0xfd9e4b85e2eb2a2df40aa61feed3123fb23355ab": {
				"ticker": "BTCp",
				"address": "0xFd9E4B85E2EB2A2Df40aA61FeED3123Fb23355AB",
				"name": "BITCOIN PRINT",
				"decimals": 4
			},
			"0xb489ecf9da6547843c17413e68f27f6678529683": {
				"ticker": "THN",
				"address": "0xb489ECf9dA6547843c17413e68F27F6678529683",
				"name": "Baby Thunder",
				"decimals": 10
			},
			"0x4886d1a924f3841ff94d9241e4fb619c1713eed2": {
				"ticker": "MDGEN",
				"address": "0x4886d1A924f3841fF94D9241e4Fb619C1713Eed2",
				"name": "MilliDEGEN",
				"decimals": 18
			},
			"0xbf27cc69a094dc5a0ae543c2d280fbe97eea1e30": {
				"ticker": "CGIP",
				"address": "0xbF27CC69a094Dc5A0aE543C2d280FBE97eEA1e30",
				"name": "Corgi Project",
				"decimals": 12
			},
			"0x68fb902fbafdd2a94667ed8a48fcb6a0fc9299fa": {
				"ticker": "SSHB",
				"address": "0x68fB902fbAfDD2A94667eD8A48FCB6A0Fc9299FA",
				"name": "Sushi Shiba",
				"decimals": 12
			},
			"0x831f98544dca243cd2bd8c21231c73d541111e5d": {
				"ticker": "WINGS",
				"address": "0x831f98544dCA243CD2bd8c21231C73d541111E5d",
				"name": "Angel Token",
				"decimals": 10
			},
			"0x2e92428d9ad084c21a6bd6735baf3297e5be51ab": {
				"ticker": "nALEPH",
				"address": "0x2e92428D9AD084c21A6Bd6735Baf3297e5bE51ab",
				"name": "Aleph",
				"decimals": 18
			},
			"0x82dca2c798f42f2b6cfc18cf24bfe63adc132ac9": {
				"ticker": "RISE",
				"address": "0x82DcA2C798f42F2b6cfc18Cf24bFe63adC132aC9",
				"name": "Up Rising Nodes",
				"decimals": 18
			},
			"0x57c6eac84c88445d1823101077e6ef5bf3649a71": {
				"ticker": "WHL",
				"address": "0x57C6eac84C88445d1823101077E6eF5bF3649a71",
				"name": "Queen Whale",
				"decimals": 10
			},
			"0x6c233efe3ec911e656cbce5b376e0b8866e3e778": {
				"ticker": "GTHA",
				"address": "0x6C233EFE3ec911E656CbCe5B376e0B8866e3e778",
				"name": "Glitch AVAX",
				"decimals": 12
			},
			"0xff856a8fc55815ff7a7090a9ebcf1f0656ea7ef8": {
				"ticker": "SSTR",
				"address": "0xff856a8fC55815ff7A7090A9ebcf1f0656EA7eF8",
				"name": "Super Star",
				"decimals": 10
			},
			"0x1abf19e14a81023f74d18a1cd4541c73266617d8": {
				"ticker": "PND",
				"address": "0x1ABf19E14a81023F74d18a1CD4541c73266617D8",
				"name": "Exo Panda",
				"decimals": 11
			},
			"0x8a0977071a261ca88facc5d16164dea023f4fe95": {
				"ticker": "APL",
				"address": "0x8a0977071a261CA88faCC5d16164dea023F4fE95",
				"name": "Pineapple Block",
				"decimals": 12
			},
			"0x2f3c326484efa757e0bf9a46dad6626d63ff3cb8": {
				"ticker": "VOLT",
				"address": "0x2f3C326484eFa757E0bF9a46DAd6626D63Ff3CB8",
				"name": "VOLT",
				"decimals": 5
			},
			"0x30c1f0cbce5f1046d686ba3d012425bcacf34147": {
				"ticker": "FTMD",
				"address": "0x30C1f0CBcE5f1046d686ba3d012425BCacF34147",
				"name": "Fantom DAO",
				"decimals": 12
			},
			"0xa176bd81e623ca5c8c12e45e9e5a73083543de84": {
				"ticker": "PAN",
				"address": "0xa176BD81e623Ca5C8C12e45e9E5a73083543dE84",
				"name": "Panda Monster",
				"decimals": 10
			},
			"0x3b55e45fd6bd7d4724f5c47e0d1bcaedd059263e": {
				"ticker": "miMatic",
				"address": "0x3B55E45fD6bd7d4724F5c47E0d1bCaEdd059263e",
				"name": "MAI",
				"decimals": 18
			},
			"0x670922672533beb0304cafd2196d6af49d3d1e26": {
				"ticker": "MARS",
				"address": "0x670922672533Beb0304CaFD2196D6af49D3d1E26",
				"name": "Fantasy Mars",
				"decimals": 12
			},
			"0x1a47a41b1d817ee56213e8e003560a5c65dbcb30": {
				"ticker": "ANG",
				"address": "0x1a47A41B1D817ee56213E8e003560A5C65Dbcb30",
				"name": "Angel BTC",
				"decimals": 11
			},
			"0x4d8952a5320d4382d419f23864b03330bb4b31a5": {
				"ticker": "SHB",
				"address": "0x4d8952A5320D4382d419F23864b03330BB4B31a5",
				"name": "Shiba INC",
				"decimals": 12
			},
			"0x3c238230e5b89de69390d62ba1777dd1c1f3801b": {
				"ticker": "WINGS",
				"address": "0x3C238230E5b89DE69390D62BA1777Dd1c1F3801B",
				"name": "King Angel",
				"decimals": 12
			},
			"0x89b54661dbddbcd459b6942dccc692019da32ab6": {
				"ticker": "MOON",
				"address": "0x89b54661DbdDbcD459B6942dCCc692019Da32Ab6",
				"name": "Zen Moon",
				"decimals": 10
			},
			"0x44d6021d6d58718bcd07ebed92ec6eaa11b06694": {
				"ticker": "STND",
				"address": "0x44d6021d6D58718BCD07EBed92EC6EAA11B06694",
				"name": "Super Thunder",
				"decimals": 9
			},
			"0xced15b0420e4299e89ef630ed19f1de994eca154": {
				"ticker": "HCI",
				"address": "0xCED15b0420e4299E89EF630ed19F1DE994EcA154",
				"name": "Safe Hachi",
				"decimals": 11
			},
			"0x06161be42fa37844388952d87970c54a230e8c5a": {
				"ticker": "GLTCH",
				"address": "0x06161bE42FA37844388952D87970C54A230E8C5A",
				"name": "Safe Glitch",
				"decimals": 9
			},
			"0x48ae8da7ae015e55e18d8b0d34f9bbe2cb6ece0d": {
				"ticker": "WCS",
				"address": "0x48aE8da7ae015e55E18d8b0d34f9bbE2cB6ECE0D",
				"name": "Whatsup Complementary System",
				"decimals": 9
			},
			"0xb8417c1cd5954c4efbfc6e740440e6dddfbeb203": {
				"ticker": "GOLD",
				"address": "0xB8417c1Cd5954c4eFbfC6e740440e6DDDFBeB203",
				"name": "Gold DAO",
				"decimals": 10
			},
			"0x8a596436cf4e2e6e325c4bc29effcd89b9ae8568": {
				"ticker": "ZTND",
				"address": "0x8a596436CF4e2e6e325C4bC29EFfCD89b9aE8568",
				"name": "Zen Thunder",
				"decimals": 9
			},
			"0xc03c3ab3d3ceb9fb773f0680a0459d6c59c256a4": {
				"ticker": "MOON",
				"address": "0xc03C3aB3D3Ceb9FB773f0680A0459D6C59c256A4",
				"name": "Moon Beast",
				"decimals": 11
			},
			"0xd314ecf1f6d5d5d1737ecf0afee14259e0542b51": {
				"ticker": "CFLK",
				"address": "0xD314Ecf1F6D5d5D1737eCf0afeE14259e0542b51",
				"name": "Crypto Floki",
				"decimals": 12
			},
			"0x221ad26689c4e6a00b060a394d842a2bea5f2d90": {
				"ticker": "YUM",
				"address": "0x221AD26689c4e6a00b060A394d842A2beA5F2D90",
				"name": "Cake Starter",
				"decimals": 11
			},
			"0x9f7fd87cb68a489e57d84318f865ba425868750f": {
				"ticker": "MPND",
				"address": "0x9f7Fd87cb68a489e57D84318F865BA425868750F",
				"name": "Meta Panda",
				"decimals": 9
			},
			"0x2273e03f080a5562f04048d8eaa4c8d70250d5e5": {
				"ticker": "FLOKI",
				"address": "0x2273e03F080a5562F04048d8EAa4c8D70250D5e5",
				"name": "Floki Classic",
				"decimals": 12
			},
			"0x11a42f52cb551d07eba57cea14c35659314b14df": {
				"ticker": "DROP",
				"address": "0x11A42f52cb551D07EBa57CEA14C35659314b14DF",
				"name": "SOSAVAX",
				"decimals": 9
			},
			"0x91289fee141efb902e2e7d303e52bad2550eaa84": {
				"ticker": "HERMES",
				"address": "0x91289FEe141efb902e2e7D303e52Bad2550Eaa84",
				"name": "HERMESDAO",
				"decimals": 9
			},
			"0xecfd4e38de3171f3dc5eb24934db6d353c04b519": {
				"ticker": "DOGE",
				"address": "0xEcfd4E38dE3171f3dc5eb24934db6d353c04b519",
				"name": "Wrapped Doge",
				"decimals": 9
			},
			"0x730f8a5d8fe59526d9fa416caa1a987155fd9d5d": {
				"ticker": "FANI",
				"address": "0x730f8a5D8fe59526d9Fa416cAa1A987155fD9D5d",
				"name": "Fantom Inu",
				"decimals": 10
			},
			"0xa1d1b3ff116e4845ec02b7c4d9ff19f23b9736d7": {
				"ticker": "GLDD",
				"address": "0xA1d1b3ff116E4845ec02B7C4D9Ff19F23B9736d7",
				"name": "Gold Dapp",
				"decimals": 10
			},
			"0xac7c663b29c7c9014e9496ffc695f9e6ae5fd9cb": {
				"ticker": "PAN",
				"address": "0xAC7C663b29c7C9014e9496ffC695F9e6aE5Fd9Cb",
				"name": "Panda Starter",
				"decimals": 12
			},
			"0xdf65c40a57ba263afdf52efd5db3c9ed0a4e11e4": {
				"ticker": "TIDDY",
				"address": "0xDF65C40A57bA263AFDf52efD5db3c9ED0a4E11e4",
				"name": "TiddyGuppy",
				"decimals": 18
			},
			"0x0f079e2d70cd024ba029b77c3e34fa27cdfae7e4": {
				"ticker": "CATE",
				"address": "0x0f079e2D70cd024Ba029B77C3E34fA27CDfAe7E4",
				"name": "CATE",
				"decimals": 18
			},
			"0x9aa76ae9f804e7a70ba3fb8395d0042079238e9c": {
				"ticker": "PGL",
				"address": "0x9AA76aE9f804E7a70bA3Fb8395D0042079238E9C",
				"name": "Pangolin Liquidity",
				"decimals": 18
			},
			"0xf6d46849db378ae01d93732585bec2c4480d1fd5": {
				"ticker": "FORT",
				"address": "0xf6d46849DB378AE01D93732585BEc2C4480D1fD5",
				"name": "Fort",
				"decimals": 9
			},
			"0xa5c98d737069e34ac97cf0eba9f11051a1197459": {
				"ticker": "EXCTN",
				"address": "0xA5c98d737069E34Ac97Cf0EBa9F11051a1197459",
				"name": "Execution Nodes",
				"decimals": 1
			},
			"0x3a6debddd5c9ee94c46b6c7efe498ed112db9a58": {
				"ticker": "APED",
				"address": "0x3a6deBDDd5c9eE94c46b6c7eFE498ED112DB9a58",
				"name": "Ape Diamond",
				"decimals": 12
			},
			"0x657ee6cfe8934de217d63d7c625fdcef93cfe316": {
				"ticker": "CORGI",
				"address": "0x657EE6cFE8934DE217D63d7C625fDCEf93CFE316",
				"name": "Corgi Fund",
				"decimals": 10
			},
			"0x88c48d4397274b795191b38bf96a69114e466fcf": {
				"ticker": "TGOS",
				"address": "0x88C48d4397274B795191B38BF96A69114e466FCf",
				"name": "Treasure Ghost",
				"decimals": 12
			},
			"0x460d3bd8e68290d8a1b0bface0ebb6b7a65cfa5d": {
				"ticker": "CHIQ",
				"address": "0x460D3BD8e68290d8A1B0bFAce0ebb6b7A65cFa5d",
				"name": "Chimp Queen",
				"decimals": 12
			},
			"0xcce70060824091a243a9767363f4d1dda7f0c885": {
				"ticker": "DGEC",
				"address": "0xcCe70060824091A243A9767363F4d1dDA7F0c885",
				"name": "Doge Coin",
				"decimals": 12
			},
			"0x3be748cb166d98d83049fa10a119b6ee624c168a": {
				"ticker": "STAR",
				"address": "0x3BE748CB166d98d83049Fa10A119B6ee624C168a",
				"name": "Crypto Star",
				"decimals": 12
			},
			"0x163b8eee1c216d13c8181249bf7dc4b4bbe6d2a2": {
				"ticker": "ELON",
				"address": "0x163b8EEe1c216D13C8181249Bf7dc4b4BBE6D2A2",
				"name": "Elon Gaming",
				"decimals": 12
			},
			"0x94a7f270cd12545a277e656266aef5e27df3eb28": {
				"ticker": "AvSTRM",
				"address": "0x94a7f270cd12545A277E656266Aef5e27dF3Eb28",
				"name": "Avalanche-Peg Stream",
				"decimals": 18
			},
			"0xb1c865d94166c3c87b39e1924edf43d4b564ae89": {
				"ticker": "DOOM",
				"address": "0xB1c865D94166C3c87B39E1924eDf43D4b564Ae89",
				"name": "DOOM",
				"decimals": 6
			},
			"0xa172e276164f1a10b6463ded9626247907579034": {
				"ticker": "QOM",
				"address": "0xA172e276164f1A10B6463DeD9626247907579034",
				"name": "AVAX QOM",
				"decimals": 18
			},
			"0x4dce34de5936c89a50d068c3fd62f958fe863116": {
				"ticker": "APLA",
				"address": "0x4dcE34dE5936c89a50d068C3fD62f958fE863116",
				"name": "Pineapple AVA",
				"decimals": 11
			},
			"0xea6ae603bae3cc49140ff8cbdfc5436c8d04a5e4": {
				"ticker": "GHT",
				"address": "0xEa6ae603Bae3cC49140FF8CbdFc5436C8D04A5E4",
				"name": "Meta Ghost",
				"decimals": 11
			},
			"0x9d9ae5b7eadd2b5f78c2f67a6e5a6ccee92bebbd": {
				"ticker": "GUN",
				"address": "0x9d9ae5b7EAdD2B5F78C2F67a6e5a6CCEe92bebBD",
				"name": "Gun Games",
				"decimals": 9
			},
			"0xb0b47bb170ac5e39d146dffe2f6f53e107a4a174": {
				"ticker": "CAKE",
				"address": "0xB0B47BB170ac5E39d146DFFe2F6F53e107a4a174",
				"name": "Crypto Cake",
				"decimals": 12
			},
			"0x4bf5cd1ac6fff12e88aedd3c70eb4148f90f8894": {
				"ticker": "ORBIT",
				"address": "0x4bf5cd1AC6FfF12E88AEDD3c70EB4148F90F8894",
				"name": "Orbit",
				"decimals": 18
			},
			"0x936ed206cdc41e4392ef588540545e0934a1dd18": {
				"ticker": "STAR",
				"address": "0x936ED206cdc41e4392EF588540545E0934a1dD18",
				"name": "Fantasy Star",
				"decimals": 12
			},
			"0x20bb31fb92170535286710cf9283f136055cbbf5": {
				"ticker": "ZANG",
				"address": "0x20bB31FB92170535286710cf9283f136055cBBF5",
				"name": "Zen Angel",
				"decimals": 10
			},
			"0x45daf5b975f808b8d51b8f6babe62033fd62c457": {
				"ticker": "WGLD",
				"address": "0x45daf5B975F808b8d51B8f6bABe62033Fd62C457",
				"name": "Wrapped Gold",
				"decimals": 11
			},
			"0x309d6f267572c3bc359166fe08a9ef926d5601c3": {
				"ticker": "MGOL",
				"address": "0x309D6f267572C3BC359166FE08A9EF926D5601C3",
				"name": "Mini Gold",
				"decimals": 10
			},
			"0xa94a13addfe1cc3b8be5a1b2d6133d914e15274d": {
				"ticker": "STAR",
				"address": "0xa94A13aDDFE1CC3B8Be5A1b2d6133D914e15274d",
				"name": "Star Inu",
				"decimals": 9
			},
			"0x3314347c6cea964f85c87aed621666a67109eaad": {
				"ticker": "NXTGEN",
				"address": "0x3314347c6cEA964F85C87AEd621666A67109Eaad",
				"name": "Next Gen Nodes",
				"decimals": 9
			},
			"0x3d5533b93a59653ba2ad397ccf16ffa7c933382f": {
				"ticker": "FirstNode",
				"address": "0x3d5533B93A59653ba2ad397ccF16ffa7C933382f",
				"name": "First Node",
				"decimals": 18
			},
			"0x2f81ae5128e8ff22e1820f9dee39638363106743": {
				"ticker": "SSBA",
				"address": "0x2F81ae5128E8FF22e1820F9DEe39638363106743",
				"name": "Sushi Shiba",
				"decimals": 11
			},
			"0x0b2857399eb473ce8bd62a5006f6c61bbd3bb861": {
				"ticker": "WHALE",
				"address": "0x0b2857399Eb473ce8bD62A5006f6c61BBd3BB861",
				"name": "Whale INC",
				"decimals": 10
			},
			"0x2fa069e1aacb7560914ea87cd4d80ab8bc518e2f": {
				"ticker": "ZEUS",
				"address": "0x2fa069E1Aacb7560914Ea87CD4d80aB8bc518E2f",
				"name": "Zeus Node Finance",
				"decimals": 18
			},
			"0xa660f845515645aa2c581236034bb5dff3a1ff99": {
				"ticker": "GOLD",
				"address": "0xa660F845515645AA2C581236034BB5dFf3A1FF99",
				"name": "Gold DAO",
				"decimals": 9
			},
			"0x9badac7bcc7ff8daab49ea4a22038053cb9d975c": {
				"ticker": "BFLK",
				"address": "0x9BaDAc7BCc7fF8dAab49eA4A22038053Cb9d975c",
				"name": "Baby Floki",
				"decimals": 9
			},
			"0x8463e7450ed25379296d7b7f1748e2685d2474d2": {
				"ticker": "MRS",
				"address": "0x8463E7450eD25379296d7B7F1748e2685d2474D2",
				"name": "Exo Mars",
				"decimals": 11
			},
			"0x0f0fc7223eb8dd09c7980d7c084ed166c50de7ae": {
				"ticker": "SAKA",
				"address": "0x0F0fc7223EB8dD09C7980d7C084ED166c50de7ae",
				"name": "Super Akita",
				"decimals": 10
			},
			"0xddc69ba8063fe1a2f53f38bb5c598b1ecb85d9ca": {
				"ticker": "BDS",
				"address": "0xDDc69BA8063fE1A2f53F38Bb5c598b1eCB85d9ca",
				"name": "Bandung Saito",
				"decimals": 18
			},
			"0xf5b1a0d66856cbf5627b0105714a7e8a89977349": {
				"ticker": "ZSHARE",
				"address": "0xF5b1A0d66856CBF5627b0105714a7E8a89977349",
				"name": "ZShare",
				"decimals": 18
			},
			"0xebb5e1fb32e53a8930e01871ac67604294771ee4": {
				"ticker": "FLOKI",
				"address": "0xEbB5e1fB32e53a8930E01871aC67604294771eE4",
				"name": "Floki BTC",
				"decimals": 9
			},
			"0x488f73cddda1de3664775ffd91623637383d6404": {
				"ticker": "YTS",
				"address": "0x488F73cddDA1DE3664775fFd91623637383D6404",
				"name": "YetiSwap",
				"decimals": 18
			},
			"0x87b33d9e393e4fa46558d833a3cfc0fb147d72a6": {
				"ticker": "AGMT",
				"address": "0x87B33d9e393e4fa46558D833A3cfc0FB147d72A6",
				"name": "Agamotto Token",
				"decimals": 18
			},
			"0x31e77cd95f9799d8fbef505ba7acc84fdb8b219e": {
				"ticker": "KR",
				"address": "0x31e77Cd95F9799d8fBeF505Ba7aCc84FDb8B219e",
				"name": "Krypton",
				"decimals": 18
			},
			"0x1e45b0e62523a774a3f9dc714c692fc951da33a6": {
				"ticker": "ZGLD",
				"address": "0x1e45B0e62523a774a3F9dc714C692fc951DA33a6",
				"name": "Zen Gold",
				"decimals": 9
			},
			"0x0f42d71a176f5d0448567f6d7b9426daf057bd72": {
				"ticker": "LFKI",
				"address": "0x0f42d71a176F5d0448567F6d7B9426daf057BD72",
				"name": "Little Floki",
				"decimals": 10
			},
			"0x61e8e10ad6f1df360775067d83ace2bc2d04259d": {
				"ticker": "FTM",
				"address": "0x61E8e10Ad6f1df360775067D83ACe2BC2D04259d",
				"name": "Fantom Ledger",
				"decimals": 10
			},
			"0x359c3bec46bae5ba621791c1bd00804715da9f4a": {
				"ticker": "APL",
				"address": "0x359c3Bec46BAE5ba621791C1Bd00804715Da9F4A",
				"name": "Crypto Apple",
				"decimals": 10
			},
			"0x9305119f2a79ea3adbda22f3f501d4a97b9ef1a1": {
				"ticker": "CGIA",
				"address": "0x9305119f2a79eA3adbda22f3f501d4A97b9EF1A1",
				"name": "Corgi AI",
				"decimals": 9
			},
			"0x6aead7f47afa9e355b852ded1942651970342db6": {
				"ticker": "AKITA",
				"address": "0x6aEAD7f47afa9E355B852Ded1942651970342DB6",
				"name": "Treasure Akita",
				"decimals": 9
			},
			"0xb221091b5a6534f5cfa544011a44d71ea35692ea": {
				"ticker": "ELON",
				"address": "0xB221091B5A6534f5cfA544011a44d71Ea35692ea",
				"name": "Sushi Elon",
				"decimals": 10
			},
			"0x239766272c74f320fca497f2b50c6e211386c44f": {
				"ticker": "SEA",
				"address": "0x239766272c74F320fCA497F2B50C6E211386c44F",
				"name": "DeepSea DAO",
				"decimals": 18
			},
			"0xe48c74833ce6f18a8e54f73f1d02b8e9f9ff8caa": {
				"ticker": "COSMIC",
				"address": "0xe48C74833ce6f18A8e54f73f1D02B8E9f9Ff8Caa",
				"name": "Cosmic",
				"decimals": 18
			},
			"0xc4621e0d86d3c61045a58715968419165b51eddf": {
				"ticker": "155-AVAX",
				"address": "0xc4621e0D86d3C61045A58715968419165B51edDF",
				"name": "155-AVAX Strike Token",
				"decimals": 18
			},
			"0x0795b0da5aba851e8d1f0c59771d602cadb40d61": {
				"ticker": "MPL",
				"address": "0x0795b0DA5AbA851E8D1f0C59771d602cAdB40D61",
				"name": "Maple",
				"decimals": 18
			},
			"0xcc064324afab4501e31de890744d7acf6f5e04d1": {
				"ticker": "MRS",
				"address": "0xcc064324AFaB4501E31De890744D7ACf6f5E04D1",
				"name": "Mars AVA",
				"decimals": 11
			},
			"0x4aa9262d9aaff77f9261bdb4582c27aa463f65aa": {
				"ticker": "ZIR",
				"address": "0x4aA9262d9AAFF77F9261Bdb4582c27aa463F65aA",
				"name": "Zionist Resistance",
				"decimals": 18
			},
			"0x4ce444de27a13c7319d204016a6e3b8e3af3f2b9": {
				"ticker": "INC",
				"address": "0x4Ce444DE27A13c7319D204016a6E3B8E3AF3F2B9",
				"name": "Inu Capital",
				"decimals": 9
			},
			"0x1154619505e01312d12e178163c69296519abd7c": {
				"ticker": "DON",
				"address": "0x1154619505e01312D12e178163c69296519Abd7c",
				"name": "Dogeon Token",
				"decimals": 18
			},
			"0xeb842f1bad956f37e8187c813dd3b2e013d79a9f": {
				"ticker": "WHIP",
				"address": "0xeb842f1bAD956f37e8187C813DD3b2E013D79A9F",
				"name": "Pyrex Whiper",
				"decimals": 18
			},
			"0x9ed1cfeb31988d6beb3440eeaeb5494efa504753": {
				"ticker": "ZGLD",
				"address": "0x9Ed1CfEB31988D6beB3440EeaeB5494efa504753",
				"name": "Zen Gold",
				"decimals": 11
			},
			"0x65eb66934a93b25799b0500d9339efc6adb94f96": {
				"ticker": "APLK",
				"address": "0x65eb66934A93b25799b0500d9339efC6aDb94f96",
				"name": "Pineapple King",
				"decimals": 9
			},
			"0x1ed9609ac81abfd829b3c8ac734cd0fa7fcbd21c": {
				"ticker": "MOON",
				"address": "0x1Ed9609ac81AbFd829b3C8aC734cD0fa7FCBD21c",
				"name": "Treasure Moon",
				"decimals": 11
			},
			"0x50cc825fadd731bacf4f5f961f7267cf12d91a15": {
				"ticker": "ELN",
				"address": "0x50Cc825faDD731BacF4f5F961f7267CF12d91a15",
				"name": "Elon Markets",
				"decimals": 10
			},
			"0x505421748269463fb51b320d12bc68831bbed959": {
				"ticker": "FST",
				"address": "0x505421748269463Fb51b320d12BC68831bbED959",
				"name": "FutureSwap token",
				"decimals": 18
			},
			"0xe6f1ec811b3d38f7cb6821d9faf35875b6f30836": {
				"ticker": "Chill",
				"address": "0xe6f1eC811B3d38f7cb6821D9FAF35875b6F30836",
				"name": "Chill",
				"decimals": 9
			},
			"0x30d5ae2bab2bd248f1b90b248f8e4143df70ebe1": {
				"ticker": "FROST",
				"address": "0x30d5Ae2BAb2bd248f1B90B248F8E4143Df70Ebe1",
				"name": "Frozen Capital",
				"decimals": 18
			},
			"0x6d923f688c7ff287dc3a5943caeefc994f97b290": {
				"ticker": "SMRTr",
				"address": "0x6D923f688C7FF287dc3A5943CAeefc994F97b290",
				"name": "SmarterCoin",
				"decimals": 18
			},
			"0x05fd4a8fa66c54dc048cd13f67b57f04efdacd12": {
				"ticker": "SBA",
				"address": "0x05Fd4a8fA66c54dc048CD13F67b57f04EfdAcD12",
				"name": "Shiba AI",
				"decimals": 9
			},
			"0x1e603abd869d632e31216ec6267b0d5c76598bb8": {
				"ticker": "PINP",
				"address": "0x1E603abD869d632E31216eC6267B0d5C76598BB8",
				"name": "Pineapple Project",
				"decimals": 11
			},
			"0x8ab082b3de0318d24a606e78eccad63cb6060a79": {
				"ticker": "ZAP",
				"address": "0x8ab082b3DE0318d24A606e78ECCaD63cb6060A79",
				"name": "Mini Thunder",
				"decimals": 12
			},
			"0x8efc8fc72026ac4afc6ef23ed824fb0a29b56f2f": {
				"ticker": "GOL",
				"address": "0x8eFc8fc72026ac4AFC6ef23ED824Fb0a29B56f2f",
				"name": "OMG Gold",
				"decimals": 11
			},
			"0x17757dce604899699b79462a63dad925b82fe221": {
				"ticker": "GRAPE",
				"address": "0x17757dcE604899699b79462a63dAd925B82FE221",
				"name": "Grape Token",
				"decimals": 18
			},
			"0xb78c5d64e1e2bb74407e34291110425030554511": {
				"ticker": "ELXR",
				"address": "0xB78C5d64e1e2bB74407e34291110425030554511",
				"name": "ELXR",
				"decimals": 6
			},
			"0x489990205763d6a4289345d51735490bd038b087": {
				"ticker": "ZEUS",
				"address": "0x489990205763D6a4289345d51735490Bd038b087",
				"name": "Zeus Node Finance",
				"decimals": 18
			},
			"0x65108e46f03ab307b242fa947cd4fd6e3bf02c98": {
				"ticker": "PDA",
				"address": "0x65108e46f03AB307B242FA947CD4FD6E3bf02C98",
				"name": "Panda Cash",
				"decimals": 11
			},
			"0xc3608859b6e97a83ee9af3e6a83db809fbd4b756": {
				"ticker": "HAUNT",
				"address": "0xc3608859B6e97a83EE9AF3e6a83Db809fbD4B756",
				"name": "Cemetery Finance",
				"decimals": 9
			},
			"0xa4e1d94fcbfc4e0dea8e3724056fcd8d7a41f362": {
				"ticker": "HCI",
				"address": "0xa4e1d94fCBfc4e0DEa8e3724056FCD8D7a41f362",
				"name": "Hachi Beast",
				"decimals": 10
			},
			"0xd97ff0f7be2e2a0c7b4b3a2f41b74d2b636aee78": {
				"ticker": "AXL",
				"address": "0xD97fF0F7Be2E2a0c7B4b3a2f41b74d2B636aEe78",
				"name": "Axxeleum Finance",
				"decimals": 18
			},
			"0x08e7c9d7b1f978af915c6e4e01cbdee3d705552b": {
				"ticker": "PNPL",
				"address": "0x08e7C9D7B1f978Af915C6E4E01CBDEe3d705552B",
				"name": "Crypto Pineapple",
				"decimals": 9
			},
			"0xfe390d712f09bdd5086b53ee9ea0cf6ae2acebc1": {
				"ticker": "WHL",
				"address": "0xfe390D712F09Bdd5086B53eE9ea0CF6aE2ACebc1",
				"name": "Exo Whale",
				"decimals": 12
			},
			"0xa2f09126961778791776675192d69272fbcad0f3": {
				"ticker": "DOGE",
				"address": "0xa2F09126961778791776675192d69272fbCAD0f3",
				"name": "Zen Doge",
				"decimals": 12
			},
			"0x5bd32feb657766a25db1d7bd5b33958276f65ede": {
				"ticker": "MOON",
				"address": "0x5bD32Feb657766A25Db1D7Bd5b33958276f65EDe",
				"name": "Moon INC",
				"decimals": 9
			},
			"0xc81ceccaf1cd0264f7095095ac68a45fed0659a5": {
				"ticker": "ANGS",
				"address": "0xc81CeCcaf1Cd0264F7095095AC68A45FED0659a5",
				"name": "Angel Starter",
				"decimals": 10
			},
			"0x810e737323917d31a109376c3acbdcc9d24f1ece": {
				"ticker": "CRG",
				"address": "0x810E737323917D31A109376c3AcBDCc9D24F1ece",
				"name": "Corgi AVAX",
				"decimals": 11
			},
			"0x26fa89d404983cf5c6fec218ff91f72e9c2311a2": {
				"ticker": "WHL",
				"address": "0x26fA89D404983CF5C6FEc218Ff91f72E9C2311a2",
				"name": "Little Whale",
				"decimals": 9
			},
			"0x76b3ff7f04e69f4b9d293401889064958cd2d455": {
				"ticker": "CMPF",
				"address": "0x76b3FF7F04E69F4b9D293401889064958CD2D455",
				"name": "Chimp Finance",
				"decimals": 9
			},
			"0xff31e0bc17c99f081a1263c62cad8bedd8afa5c3": {
				"ticker": "GHS",
				"address": "0xFf31e0Bc17C99f081a1263C62Cad8bedd8aFa5c3",
				"name": "Exo Ghost",
				"decimals": 11
			},
			"0x7761e2338b35bceb6bda6ce477ef012bde7ae611": {
				"ticker": "EGG",
				"address": "0x7761E2338B35bCEB6BdA6ce477EF012bde7aE611",
				"name": "chikn egg",
				"decimals": 18
			},
			"0xf12e34a927b995971abc6cfdf4ad941137ab47e4": {
				"ticker": "DOG",
				"address": "0xf12e34A927B995971aBc6cFdF4AD941137AB47e4",
				"name": "Crypto Doge",
				"decimals": 11
			},
			"0x6d9d27e51664c9ad19c08155963036e19a3a148e": {
				"ticker": "GOL",
				"address": "0x6D9D27E51664c9Ad19C08155963036e19A3a148e",
				"name": "Gold Token",
				"decimals": 10
			},
			"0x6c6f910a79639dcc94b4feef59ff507c2e843929": {
				"ticker": "aAVAXb",
				"address": "0x6C6f910A79639dcC94b4feEF59Ff507c2E843929",
				"name": "Ankr Avalanche Reward Earning Bond",
				"decimals": 18
			},
			"0x4c443911fe1215fcbb2ccff34d53f6fd69a631e6": {
				"ticker": "MILK",
				"address": "0x4c443911FE1215FcBb2Ccff34d53f6fd69a631e6",
				"name": "Dairy Money",
				"decimals": 9
			},
			"0xf142c46854ad088e80f03f9ff9f73156bd44f6f6": {
				"ticker": "OAKA",
				"address": "0xF142C46854Ad088e80f03F9ff9f73156Bd44f6F6",
				"name": "OMG Akita",
				"decimals": 12
			},
			"0xc89e21d0eef4ad6ee3c7d751ce2d8ae4875be0a5": {
				"ticker": "Artand",
				"address": "0xC89e21d0EEf4aD6ee3C7D751Ce2d8Ae4875be0A5",
				"name": "Artand",
				"decimals": 18
			},
			"0xb778356da16c3f2518ca4c2029446b9e5b16b4bb": {
				"ticker": "FAN",
				"address": "0xB778356Da16c3F2518ca4C2029446b9e5B16B4Bb",
				"name": "Fantom X",
				"decimals": 10
			},
			"0x6911c3a705998d7e8f5e3d233af9484bb4a2207f": {
				"ticker": "CEL",
				"address": "0x6911C3a705998d7E8F5e3D233aF9484bb4A2207f",
				"name": "Celsius.Network",
				"decimals": 9
			},
			"0x8a75c2ce70b4f3a27f570b21c8e87101caa15745": {
				"ticker": "HCI",
				"address": "0x8A75C2Ce70b4F3a27f570B21C8e87101caA15745",
				"name": "Hachi Token",
				"decimals": 11
			},
			"0xfc575c0fb9f1d1bd8888a3f2976d7a6189ab6b83": {
				"ticker": "SHBA",
				"address": "0xfC575c0fB9f1d1bD8888a3F2976d7A6189ab6B83",
				"name": "Shiba AVAX",
				"decimals": 11
			},
			"0x7fbc431e8a779dab7dd92c3993ef58fbba4bf4f3": {
				"ticker": "SHIB",
				"address": "0x7fBc431E8a779Dab7dD92C3993Ef58fBba4bf4f3",
				"name": "Shiba Fund",
				"decimals": 12
			},
			"0x6487776192eff6c25e51f5209a0a6c2d3d6d4cbb": {
				"ticker": "ELN",
				"address": "0x6487776192eff6C25e51f5209A0a6C2D3d6D4CbB",
				"name": "Elon AVAX",
				"decimals": 12
			},
			"0xe71856307d88b166eeb7250d017c34c9c370c19c": {
				"ticker": "MRS",
				"address": "0xe71856307d88B166eeb7250D017C34C9C370c19c",
				"name": "Mars Project",
				"decimals": 12
			},
			"0x373b5f54a40a45682a5627553adfecadebba5c63": {
				"ticker": "TSUN",
				"address": "0x373B5F54a40a45682A5627553aDfeCADEBBa5c63",
				"name": "ApocalypticaDAO",
				"decimals": 9
			},
			"0x123cb3c332b7015913f1ea71be03a1d634a178c4": {
				"ticker": "FAN",
				"address": "0x123cb3c332B7015913f1eA71be03A1D634A178C4",
				"name": "OMG Fantom",
				"decimals": 12
			},
			"0x9a9c08e3f73fc28ad617b0fc9141e13dcf63f921": {
				"ticker": "SHB",
				"address": "0x9a9C08e3f73FC28Ad617B0fc9141E13DCf63f921",
				"name": "Wrapped Shiba",
				"decimals": 12
			},
			"0xfffe5fc3e511ce11df20684aec435a3e2b7d8136": {
				"ticker": "OT-qiUSDC-28DEC2023",
				"address": "0xfffe5fC3E511cE11dF20684AEC435A3E2b7D8136",
				"name": "OT Benqi USDC 28DEC2023",
				"decimals": 6
			},
			"0x1111111111182587795ef1098ac7da81a108c97a": {
				"ticker": "BPT",
				"address": "0x1111111111182587795eF1098ac7da81a108C97a",
				"name": "Bold Point Token",
				"decimals": 18
			},
			"0x54a14d74a37fa3332a521d67fcf76757a83a3833": {
				"ticker": "CAPL",
				"address": "0x54A14D74a37fA3332A521D67fcf76757a83a3833",
				"name": "Crypto Apple",
				"decimals": 11
			},
			"0xd039c9079ca7f2a87d632a9c0d7cea0137bacfb5": {
				"ticker": "APE-X",
				"address": "0xd039C9079ca7F2a87D632A9C0d7cEa0137bAcFB5",
				"name": "Ape-X",
				"decimals": 9
			},
			"0xe0a7f41e31a333d0acbc32afd4d998e4177daab3": {
				"ticker": "FLASH",
				"address": "0xe0A7F41E31a333D0AcbC32AFd4D998e4177DaaB3",
				"name": "Flash Nodes",
				"decimals": 9
			},
			"0xaed188a41ddc32e3e05e4b645c9e2a08945449ee": {
				"ticker": "HCI",
				"address": "0xAed188A41ddC32E3E05e4B645c9E2A08945449ee",
				"name": "Hachi Token",
				"decimals": 12
			},
			"0x51cb6c7d43be70e77d07fd2b8b9150fd243b38c9": {
				"ticker": "STR",
				"address": "0x51Cb6c7D43be70E77D07Fd2b8B9150FD243B38c9",
				"name": "Star King",
				"decimals": 12
			},
			"0x1f72fed09b5475e69d99e7a4529e74868ae88e4f": {
				"ticker": "CHI",
				"address": "0x1F72fEd09b5475e69D99e7a4529E74868AE88E4F",
				"name": "Chimp AVAX",
				"decimals": 12
			},
			"0x14ba23f0b431cc1fe3413faac73c549e984f3027": {
				"ticker": "MAGIA",
				"address": "0x14bA23F0b431cc1FE3413Faac73c549e984F3027",
				"name": "Magia Nodes",
				"decimals": 18
			},
			"0x0f399d96a1ae3fefb5c8f7af43dade9ac8bf4925": {
				"ticker": "PONZINODES",
				"address": "0x0f399d96a1ae3feFB5C8f7Af43DADE9Ac8Bf4925",
				"name": "PONZINODES",
				"decimals": 18
			},
			"0x814409abbc142fa5824c034d564d8d738b20cd51": {
				"ticker": "ELECTRUM",
				"address": "0x814409AbbC142fa5824C034d564D8D738b20cD51",
				"name": "ElectrumBar Token",
				"decimals": 18
			},
			"0xdd3ef3f867fce839e259e2f3d244137c4b584a24": {
				"ticker": "AKITA",
				"address": "0xdd3EF3F867FCE839E259e2f3d244137c4b584A24",
				"name": "Akita Network",
				"decimals": 12
			},
			"0x0a1baa90e4802c6cff230ab7e38fba7592d2066b": {
				"ticker": "ASTOR",
				"address": "0x0A1bAA90e4802C6cFf230AB7e38fbA7592d2066B",
				"name": "AstroNode",
				"decimals": 18
			},
			"0x40a74f556725063dca2ba0cb5a4372bc925070b0": {
				"ticker": "ELNP",
				"address": "0x40a74F556725063DCA2bA0cB5A4372bc925070B0",
				"name": "Elon Project",
				"decimals": 11
			},
			"0xb95775333c7af5b7527ba4442462d6f70d37df1c": {
				"ticker": "FUEL",
				"address": "0xb95775333C7Af5B7527bA4442462d6f70D37Df1c",
				"name": "Fuel Nodes",
				"decimals": 18
			},
			"0x0fec6d8a84a85b79a1ffe0e28c1902e08b653efe": {
				"ticker": "HOOP",
				"address": "0x0fEc6d8A84A85b79A1FFE0E28c1902E08b653EFE",
				"name": "Hoopoe Ventures",
				"decimals": 18
			},
			"0x390080708adbea88fc66f26f25f93ac8ae043bab": {
				"ticker": "GCH",
				"address": "0x390080708adBEA88fC66F26F25F93Ac8ae043bab",
				"name": "Glitch BTC",
				"decimals": 12
			},
			"0xda19149450c6adc242f70466036b31642fa65cfd": {
				"ticker": "YUMF",
				"address": "0xda19149450c6AdC242F70466036b31642FA65CfD",
				"name": "Cake Fund",
				"decimals": 9
			},
			"0x9ab42a84c9a412e2c7ea6cd08f5da1ac4aececdd": {
				"ticker": "LEAF",
				"address": "0x9ab42A84c9A412e2C7EA6CD08f5dA1AC4aECECdd",
				"name": "LEAF",
				"decimals": 18
			},
			"0x027dbca046ca156de9622cd1e2d907d375e53aa7": {
				"ticker": "AMPL",
				"address": "0x027dbcA046ca156De9622cD1e2D907d375e53aa7",
				"name": "Ampleforth secured by Meter Passport",
				"decimals": 9
			},
			"0x63fb6ddd707007aa87aede99339708bc335de931": {
				"ticker": "APE",
				"address": "0x63fb6ddD707007AA87aEDE99339708BC335de931",
				"name": "Ape Swap",
				"decimals": 11
			},
			"0x340990358ae38008181b6db6156d9021a2d425da": {
				"ticker": "ORBIT",
				"address": "0x340990358AE38008181b6dB6156D9021A2D425dA",
				"name": "Europa Token",
				"decimals": 18
			},
			"0xca7730d239f9fca6d9683b0a62ed1a6c660d7cba": {
				"ticker": "FREE",
				"address": "0xCa7730D239f9fCA6d9683B0a62ed1A6c660D7CbA",
				"name": "Freebitcoins",
				"decimals": 18
			},
			"0xae14490e6ab94f1bf177b883eb28b8d8898041d2": {
				"ticker": "LVI",
				"address": "0xAE14490e6aB94f1bf177B883EB28b8d8898041d2",
				"name": "SuperBowl Nodes",
				"decimals": 9
			},
			"0xd296bbd25f6355d332f8cc32a51dbab81039508d": {
				"ticker": "MULTI",
				"address": "0xD296bBd25F6355D332F8cC32A51dbab81039508D",
				"name": "MultiChain.org Bridge",
				"decimals": 9
			},
			"0x894b4225673ee9976be71d6a52d04372da55936e": {
				"ticker": "HCI",
				"address": "0x894B4225673Ee9976bE71D6A52D04372DA55936E",
				"name": "Treasure Hachi",
				"decimals": 11
			},
			"0x4cdc121aff564414ddc11e7226138c9211cce8a1": {
				"ticker": "preASTRO",
				"address": "0x4CDC121afF564414ddc11E7226138c9211cCE8A1",
				"name": "100 Days Ventures",
				"decimals": 18
			},
			"0xb0170a7589a552fd99bcfd4ae240bcae5fc8dd88": {
				"ticker": "CHIMP",
				"address": "0xB0170A7589A552Fd99bCfd4ae240bCaE5fc8dd88",
				"name": "Chimp Network",
				"decimals": 11
			},
			"0x14cb0db8bf106d7ffa765c90e5ebdc418805d914": {
				"ticker": "CAKE",
				"address": "0x14cb0db8bf106d7fFA765c90E5EBDc418805d914",
				"name": "Cake SV",
				"decimals": 9
			},
			"0xd7f4b21ae88a5e41fcc952f313c9b9ecf37104a6": {
				"ticker": "SHIB",
				"address": "0xd7f4B21Ae88A5E41fcC952f313C9B9EcF37104a6",
				"name": "Mini Shiba",
				"decimals": 10
			},
			"0x5079d431bee5d5ef365c4ce03c2f517adc647be3": {
				"ticker": "Yuzu",
				"address": "0x5079D431bEE5d5eF365c4Ce03C2f517adC647Be3",
				"name": "Yuzu",
				"decimals": 10
			},
			"0xea554209ffb58c7483b9342afde8a2acdbb4945b": {
				"ticker": "HCI",
				"address": "0xeA554209ffB58c7483b9342AfdE8a2ACDbB4945b",
				"name": "Hachi Dapp",
				"decimals": 11
			},
			"0x3e531f5277f13da1020be467abd4842e1b615b3d": {
				"ticker": "DUELS",
				"address": "0x3e531F5277F13dA1020be467abd4842e1B615b3D",
				"name": "Avax Duels",
				"decimals": 9
			},
			"0xb2dafee44eac58c3a5ad8db77728597ff34ec70e": {
				"ticker": "XmasNodes",
				"address": "0xb2daFee44EaC58c3a5Ad8dB77728597Ff34Ec70E",
				"name": "ChristmasNodes",
				"decimals": 18
			},
			"0x162c0efc46a116a524b56a26044278423ea71b96": {
				"ticker": "GCH",
				"address": "0x162c0Efc46A116A524b56a26044278423EA71B96",
				"name": "Sushi Glitch",
				"decimals": 10
			},
			"0x2d49e254ed5898d7ced13ae3d76f324e914099c6": {
				"ticker": "GUNS",
				"address": "0x2d49E254Ed5898D7CEd13Ae3d76f324e914099C6",
				"name": "Gun Games",
				"decimals": 9
			},
			"0x1386a9713cc8df0284c5735ac935e4c0e94ab2d4": {
				"ticker": "STAR",
				"address": "0x1386a9713cc8Df0284C5735Ac935E4C0e94ab2D4",
				"name": "Fantasy Star",
				"decimals": 12
			},
			"0x46e5c7afe3ca0cf3af5e03c81d3d3200777d2769": {
				"ticker": "PLAYMATES",
				"address": "0x46E5c7AFe3Ca0CF3AF5e03C81d3d3200777d2769",
				"name": "Playmates",
				"decimals": 18
			},
			"0x7a428a630312e00c1b17601ec1b5fb12bd919b51": {
				"ticker": "ANG",
				"address": "0x7a428A630312e00c1b17601Ec1B5FB12bD919B51",
				"name": "King Angel",
				"decimals": 12
			},
			"0x1c1e662dc00ac95d51cd4bb069565acedbf8ece9": {
				"ticker": "ZGOL",
				"address": "0x1c1E662dC00AC95d51cD4BB069565ACEdbF8eCE9",
				"name": "Zen Gold",
				"decimals": 11
			},
			"0x137bbfafc54c8dbe19e10b2f670959ae226aefd2": {
				"ticker": "FAILD",
				"address": "0x137BbFAFC54C8Dbe19E10B2F670959aE226aeFD2",
				"name": "FAILD",
				"decimals": 9
			},
			"0xf5ee578505f4d876fef288dfd9fd5e15e9ea1318": {
				"ticker": "VOLT",
				"address": "0xf5ee578505f4D876FeF288DfD9fD5e15e9EA1318",
				"name": "Asgardian Aereus",
				"decimals": 9
			},
			"0xf03b40ee75eba56a09a096b813a00665c8924491": {
				"ticker": "APE",
				"address": "0xf03B40eE75EbA56a09A096B813a00665c8924491",
				"name": "Queen Ape",
				"decimals": 9
			},
			"0x1db749847c4abb991d8b6032102383e6bfd9b1c7": {
				"ticker": "DON",
				"address": "0x1DB749847C4abB991d8B6032102383e6BfD9B1c7",
				"name": "Dogeon Token",
				"decimals": 18
			},
			"0x659552a3039dc080276ca5d2fe3c45326ab0455b": {
				"ticker": "AKITA",
				"address": "0x659552A3039DC080276cA5D2fe3c45326Ab0455b",
				"name": "Akita AVA",
				"decimals": 12
			},
			"0xd8dc150e2282f203930c4f973058fd88a6797a99": {
				"ticker": "GHT",
				"address": "0xd8Dc150e2282F203930C4F973058FD88A6797A99",
				"name": "OMG Ghost",
				"decimals": 9
			},
			"0x4860e0f7264b433d54bd2cc2bd4a1b2bfedfaff4": {
				"ticker": "TT",
				"address": "0x4860e0f7264b433d54bd2cc2Bd4A1B2bfEDfAfF4",
				"name": "Tsttokens",
				"decimals": 18
			},
			"0x9574ffc529c5c7bfe452f95350dcbbc6e1f10813": {
				"ticker": "ETHN",
				"address": "0x9574FFC529c5c7Bfe452F95350dCBbC6E1f10813",
				"name": "Exo Thunder",
				"decimals": 9
			},
			"0x008e26068b3eb40b443d3ea88c1ff99b789c10f7": {
				"ticker": "ZERO",
				"address": "0x008E26068B3EB40B443d3Ea88c1fF99B789c10F7",
				"name": "Zero.Exchange Token",
				"decimals": 18
			},
			"0xea5144ed6859f26f3cc94a0ddd86ed32101518b2": {
				"ticker": "E8",
				"address": "0xea5144eD6859f26f3cC94a0Ddd86ed32101518B2",
				"name": "Energy8",
				"decimals": 9
			},
			"0x942187243f8f9feb1a170c85e2722db6fe5e4150": {
				"ticker": "MOON",
				"address": "0x942187243f8F9FEb1A170C85e2722db6fE5e4150",
				"name": "Meta Moon",
				"decimals": 9
			},
			"0xa2da280a5c43b59d200885a74fd6f0f103b0f635": {
				"ticker": "AVATAR",
				"address": "0xa2Da280a5c43b59d200885a74FD6F0F103B0F635",
				"name": "AvatarDAO",
				"decimals": 18
			},
			"0xed53d6a40ce4deed6c1d669dc310ffc5f6f62462": {
				"ticker": "CORK",
				"address": "0xed53D6A40Ce4deED6C1D669Dc310FFC5F6F62462",
				"name": "Corkscrew",
				"decimals": 18
			},
			"0x0278a1328da2394af64eac693a34614756358254": {
				"ticker": "DVERSE",
				"address": "0x0278a1328da2394af64eAC693a34614756358254",
				"name": "DraVersr",
				"decimals": 9
			},
			"0x73bbd041a7caa550d40e9ac905dc82ed953ed96b": {
				"ticker": "LMRS",
				"address": "0x73bBd041A7CAa550d40e9AC905DC82ED953eD96B",
				"name": "Little Mars",
				"decimals": 12
			},
			"0xbfbc251f860952f00099e75e053a58fd3ebcef91": {
				"ticker": "WHALE",
				"address": "0xbfBC251f860952f00099E75E053A58Fd3EBCeF91",
				"name": "Treasure Whale",
				"decimals": 10
			},
			"0x2d7fda52287acc43441b6dd7439e0f1243da5556": {
				"ticker": "AGC",
				"address": "0x2D7fDA52287ACc43441b6dD7439E0f1243DA5556",
				"name": "Ascenders",
				"decimals": 18
			},
			"0x4821bc8a4e6b70d7b3b1a8afdc125a4afb92b462": {
				"ticker": "DB",
				"address": "0x4821BC8A4e6b70d7B3B1a8aFDC125A4afb92b462",
				"name": "DiamondBank DAO",
				"decimals": 18
			},
			"0x2a19916de0179b5002d24df9aee97c08f9a151bc": {
				"ticker": "THN",
				"address": "0x2A19916DE0179b5002d24df9AEE97c08F9a151bc",
				"name": "Crypto Thunder",
				"decimals": 11
			},
			"0x7956a0561827795ed2e3e67cf65fab2fd045dc48": {
				"ticker": "LHCH",
				"address": "0x7956A0561827795ED2E3E67Cf65fAb2fD045dc48",
				"name": "Little Hachi",
				"decimals": 10
			},
			"0xe0cb3a92d0a2c008674382dbdb085e3750d1707f": {
				"ticker": "WHALE",
				"address": "0xE0Cb3a92d0A2c008674382DbDB085e3750D1707f",
				"name": "Whale Cash",
				"decimals": 9
			},
			"0x1dfed468d319ea11623acb942fea418b0a639dfd": {
				"ticker": "MRS",
				"address": "0x1DfEd468d319Ea11623aCb942FEa418b0A639dfd",
				"name": "Mars Markets",
				"decimals": 10
			},
			"0x30e8e477049487695a8b86338348b6b5ca9faca9": {
				"ticker": "ELN",
				"address": "0x30E8e477049487695a8b86338348b6B5cA9FACa9",
				"name": "Safe Elon",
				"decimals": 9
			},
			"0x1a2e32bb5efb0afe3b3e875f988df700d66aceb5": {
				"ticker": "MONA",
				"address": "0x1a2e32BB5EFb0AfE3b3E875F988df700D66aceb5",
				"name": "MonacoNFT.io",
				"decimals": 18
			},
			"0xcfd8db6c61a3b9dde3936c9c3aeb01b99991bfb6": {
				"ticker": "STR",
				"address": "0xcfd8Db6C61A3b9dDE3936C9C3AeB01B99991Bfb6",
				"name": "Star Classic",
				"decimals": 12
			},
			"0x888365cef16baf8354bf1a9a88e548555b7348bf": {
				"ticker": "PRINT",
				"address": "0x888365Cef16Baf8354Bf1A9A88e548555b7348bF",
				"name": "Printy",
				"decimals": 18
			},
			"0xe1e7f7a8fe70bd636b4dfa08ca154e339f77b236": {
				"ticker": "JEW",
				"address": "0xE1E7f7a8Fe70bd636b4dfA08Ca154e339F77b236",
				"name": "Jew Money",
				"decimals": 9
			},
			"0xef916461af3dd4682b4bde481b9274d2f0ed5987": {
				"ticker": "ICLD",
				"address": "0xeF916461af3Dd4682b4BDe481b9274D2F0ed5987",
				"name": "Icelandic shares",
				"decimals": 18
			},
			"0x637b83dd044935899681b0aa37f359cc35a69b43": {
				"ticker": "ARMOR",
				"address": "0x637B83Dd044935899681B0AA37F359cC35A69B43",
				"name": "SpartanDAO",
				"decimals": 9
			},
			"0x80cf56b4d34670b10867c540066f9979b610ad44": {
				"ticker": "STUART",
				"address": "0x80cF56b4D34670B10867c540066f9979b610Ad44",
				"name": "Stuart Finance",
				"decimals": 18
			},
			"0xd5785b7bafaae7cdad603ef5f40bccd130449fcc": {
				"ticker": "MOON",
				"address": "0xD5785b7BafaAe7CDAD603Ef5f40BcCD130449fcc",
				"name": "OMG Moon",
				"decimals": 10
			},
			"0xbdbaded021a173c2839643e929e9c1ebb900687c": {
				"ticker": "GTC",
				"address": "0xbDbaDeD021a173C2839643e929E9c1eBB900687C",
				"name": "GT Capital Holding",
				"decimals": 9
			},
			"0x888a3fcc6b035272c1ad088e31222af4266cd974": {
				"ticker": "SHB",
				"address": "0x888a3fCC6B035272c1ad088e31222AF4266cd974",
				"name": "Fantasy Shiba",
				"decimals": 9
			},
			"0xfb1acaba073663dbd96e2119e616751669004592": {
				"ticker": "SHIB",
				"address": "0xFB1AcabA073663Dbd96e2119e616751669004592",
				"name": "Fantasy Shiba",
				"decimals": 10
			},
			"0x512298db58be4acbbfddfdbaf7e79b4ffddd2e81": {
				"ticker": "GTHF",
				"address": "0x512298Db58bE4AcbbFDDFDbAf7E79B4fFDdD2E81",
				"name": "Glitch Farm",
				"decimals": 12
			},
			"0x1ed959d43696a045f7cf59748f04ce272922917f": {
				"ticker": "CON",
				"address": "0x1eD959d43696a045f7cF59748f04CE272922917F",
				"name": "Contexia",
				"decimals": 18
			},
			"0x71e854e92602cd4cbfc0d3fb5b3761453a4211b8": {
				"ticker": "GLD",
				"address": "0x71e854E92602cD4CbFC0D3Fb5B3761453A4211b8",
				"name": "Crypto Gold",
				"decimals": 12
			},
			"0xe155ac1bb1b7b5b50ab6a574f6768d709341cbc9": {
				"ticker": "QTM",
				"address": "0xE155aC1BB1b7B5B50AB6a574F6768D709341CBc9",
				"name": "Quantum Verse",
				"decimals": 8
			},
			"0xe380248c15f3816fac1eaf9defafd52f579f0293": {
				"ticker": "CHEESE",
				"address": "0xe380248C15F3816faC1Eaf9deFafD52f579F0293",
				"name": "MOONRATa",
				"decimals": 9
			},
			"0xf8aa047530668d26a58bf3cf25e48ca190b38532": {
				"ticker": "DOOM",
				"address": "0xF8aa047530668D26A58Bf3cf25e48Ca190B38532",
				"name": "DOOM",
				"decimals": 6
			},
			"0x53632621c2c268d409a54e9fb78ac9279d471a3f": {
				"ticker": "ZEUS",
				"address": "0x53632621C2C268d409a54E9fb78aC9279D471a3f",
				"name": "ZEUS v1",
				"decimals": 18
			},
			"0x92a5f9b1304eb836e7c7818d2c5ee538f0c34d1c": {
				"ticker": "HCH",
				"address": "0x92a5f9B1304Eb836e7c7818D2c5EE538f0C34d1c",
				"name": "Hachi Coin",
				"decimals": 11
			},
			"0x01d0d7eb72e996b62651c60fe70b88ca07bb8e92": {
				"ticker": "SHARK",
				"address": "0x01d0D7Eb72E996B62651C60Fe70b88ca07Bb8e92",
				"name": "SharkCoin",
				"decimals": 18
			},
			"0x563ff52a1eb952036f5a39e74bb3081d81e97f12": {
				"ticker": "FAN",
				"address": "0x563fF52a1Eb952036F5A39e74BB3081D81e97F12",
				"name": "Fantom Treasure",
				"decimals": 10
			},
			"0x5d9593d5a600ae428b397036e52ada788f42a231": {
				"ticker": "GLT",
				"address": "0x5d9593D5A600Ae428b397036e52adA788f42A231",
				"name": "Glitch Farm",
				"decimals": 10
			},
			"0x56037dd89b29e594530615f3e1ba97e4ae631a8c": {
				"ticker": "AR",
				"address": "0x56037DD89b29E594530615f3e1ba97E4Ae631a8C",
				"name": "Arsenic Finance",
				"decimals": 18
			},
			"0x641686ee9092484cce91b334cf0681f51c3bdadf": {
				"ticker": "Honey",
				"address": "0x641686ee9092484CCe91b334cf0681f51c3bDaDF",
				"name": "Honey",
				"decimals": 18
			},
			"0xe1bebbd9f13f3ce10c54194bb46bea9997c37405": {
				"ticker": "BCMP",
				"address": "0xE1BEBBd9f13F3CE10C54194bB46beA9997c37405",
				"name": "Baby Chimp",
				"decimals": 11
			},
			"0xe6b430e688b76ac89588c587cf8d35e96df88b15": {
				"ticker": "FB",
				"address": "0xe6B430e688b76Ac89588c587CF8d35E96Df88B15",
				"name": "Frostbite DAO (3,3)",
				"decimals": 18
			},
			"0x5f21940f976d1c1450bd58ec64b7ad49a7fbc6eb": {
				"ticker": "GOL",
				"address": "0x5F21940f976D1C1450bD58EC64B7aD49a7Fbc6eB",
				"name": "Gold Fund",
				"decimals": 12
			},
			"0xdbe685613bcd91de29ed8358dbdb378a0b152341": {
				"ticker": "SGOS",
				"address": "0xDbE685613BCD91De29Ed8358dBDB378a0B152341",
				"name": "Sushi Ghost",
				"decimals": 9
			},
			"0xdb33aa59103f2a4249ea8b9f855a76fc3da7d432": {
				"ticker": "ZANG",
				"address": "0xdb33aA59103F2a4249Ea8B9F855a76fC3dA7d432",
				"name": "Zen Angel",
				"decimals": 10
			},
			"0x25f1e150f034b6c6af6247196434720eb8942af6": {
				"ticker": "PLAYMATE",
				"address": "0x25f1e150f034B6c6AF6247196434720eb8942Af6",
				"name": "Playmate",
				"decimals": 9
			},
			"0x5dbff54f5c0cc80a14a928410489c6dbdbd54e82": {
				"ticker": "ACMEX",
				"address": "0x5DbfF54f5c0cc80A14A928410489C6DBDBd54e82",
				"name": "ACME Avalanche https://t.me/acmeavalanche",
				"decimals": 18
			},
			"0xed20e0cef0a1636c2176e537134f069b1c82aafc": {
				"ticker": "GRILLZ",
				"address": "0xeD20E0CEF0A1636c2176e537134f069B1c82AAFc",
				"name": "GRILLZ",
				"decimals": 18
			},
			"0x19eb556a403f9bc954d6c2667dc0e62e2793e810": {
				"ticker": "ANG",
				"address": "0x19eb556a403F9bc954d6c2667Dc0E62e2793e810",
				"name": "Angel Network",
				"decimals": 9
			},
			"0xf5f54ec4ec6fac257eb95c06301f3b267c612df6": {
				"ticker": "SHIB",
				"address": "0xf5f54eC4Ec6fAC257Eb95c06301f3B267c612df6",
				"name": "Shiba Queen",
				"decimals": 9
			},
			"0x278e9e4490f6259725a2544c98073f5acf5211ea": {
				"ticker": "TOM",
				"address": "0x278e9e4490F6259725a2544C98073F5AcF5211EA",
				"name": "Wrapped Fantom",
				"decimals": 12
			},
			"0x56dadc08e987b156964718b3a8dc8e352dc17410": {
				"ticker": "CAPL",
				"address": "0x56dAdc08E987b156964718b3a8DC8E352Dc17410",
				"name": "Crypto Apple",
				"decimals": 9
			},
			"0xddcb014bca1a5e7ad62315167d2fff1cc7f800d5": {
				"ticker": "CHI",
				"address": "0xddCB014bCA1A5e7AD62315167D2fFf1cc7f800d5",
				"name": "Chimp Gaming",
				"decimals": 11
			},
			"0xf53ee01b8460525ff5cc3a1be08828b8f5d53947": {
				"ticker": "PLE",
				"address": "0xf53EE01B8460525FF5cc3A1be08828b8F5D53947",
				"name": "Plethori",
				"decimals": 9
			},
			"0x9e919eef80383e82a3b9ee12263c0f2c7e71f61c": {
				"ticker": "$REACT",
				"address": "0x9e919eeF80383E82a3b9ee12263c0F2C7e71F61c",
				"name": "Rebase Aggregator Capital",
				"decimals": 18
			},
			"0x77617acfb1d328cd4361bc6a5ed57c2ad0133862": {
				"ticker": "MRS",
				"address": "0x77617aCfb1d328CD4361Bc6a5eD57C2aD0133862",
				"name": "Wrapped Mars",
				"decimals": 12
			},
			"0x0ca1bb5cc5cd4dbb1708c426541f4a2e39b26676": {
				"ticker": "ICE",
				"address": "0x0ca1Bb5cc5cD4DBb1708c426541f4a2E39B26676",
				"name": "Project ICE Storm",
				"decimals": 9
			},
			"0xba28b56509019a6afaa464d7db26bad9e65cd473": {
				"ticker": "ASAFTI",
				"address": "0xba28b56509019A6AFaa464d7DB26baD9E65cd473",
				"name": "SafuTitano",
				"decimals": 18
			},
			"0xffde5147f7e3181c20455e71caefff9bbe6c794e": {
				"ticker": "ROCK",
				"address": "0xfFdE5147f7E3181c20455E71caeFff9bBE6c794e",
				"name": "Rock Finance",
				"decimals": 9
			},
			"0x2794e1102e0f48ecd7df36bf0c8e5d3fe3270265": {
				"ticker": "TELN",
				"address": "0x2794E1102E0F48ECD7df36Bf0c8E5D3fE3270265",
				"name": "Treasure Elon",
				"decimals": 12
			},
			"0x38365be61c7075f56666ddbee8264135edffe4d2": {
				"ticker": "HCH",
				"address": "0x38365bE61c7075F56666dDBEE8264135edFFE4d2",
				"name": "Project Hachi",
				"decimals": 9
			},
			"0xbfe4dd6c1dfa51787217041b17851a07b0c47ec5": {
				"ticker": "QAPL",
				"address": "0xBfE4dd6C1dFa51787217041B17851a07b0C47EC5",
				"name": "Queen Apple",
				"decimals": 10
			},
			"0xdf436d68c20f5bfbbc708d0514d6444833c2a609": {
				"ticker": "PT",
				"address": "0xdf436D68C20F5bFbbC708D0514D6444833C2A609",
				"name": "Platypus NFT",
				"decimals": 18
			},
			"0xc7d752d540db59aa28924f0c79580a738f5c0b11": {
				"ticker": "CHCI",
				"address": "0xC7D752D540db59Aa28924f0C79580A738f5C0b11",
				"name": "Crypto Hachi",
				"decimals": 11
			},
			"0xe37c1f88ae5aaaa190f744e5e31e63ef5e7f3c03": {
				"ticker": "CAKE",
				"address": "0xe37c1F88AE5Aaaa190F744e5e31e63EF5e7f3C03",
				"name": "Treasure Cake",
				"decimals": 9
			},
			"0xb7ea2ebb7b4b36ca8a61f07b845bca42e9c5893b": {
				"ticker": "WHL",
				"address": "0xB7eA2ebB7b4b36CA8A61F07b845BcA42e9C5893B",
				"name": "Exo Whale",
				"decimals": 9
			},
			"0xbd514c4b1f6f6296164af5249b0653afa38f5ae7": {
				"ticker": "STK",
				"address": "0xbD514c4B1f6F6296164Af5249B0653afa38F5AE7",
				"name": "ServerTech",
				"decimals": 18
			},
			"0xb723783e0f9015c8e20b87f6cf7ae24df6479e62": {
				"ticker": "CCY",
				"address": "0xb723783e0f9015c8E20b87F6CF7ae24dF6479e62",
				"name": "ChoccyCoin",
				"decimals": 18
			},
			"0x8e607bba5219736038dd6fa7d08a0d2f784d7527": {
				"ticker": "SBA",
				"address": "0x8e607bba5219736038Dd6fa7D08A0d2F784D7527",
				"name": "Shiba Queen",
				"decimals": 12
			},
			"0xe06fba763c2104db5027f57f6a5be0a0d86308af": {
				"ticker": "AKITAX",
				"address": "0xE06fba763C2104dB5027F57f6A5Be0a0D86308af",
				"name": "AKITAVAX",
				"decimals": 18
			},
			"0xf32398dae246c5f672b52a54e9b413dffcae1a44": {
				"ticker": "KACY",
				"address": "0xf32398dae246C5f672B52A54e9B413dFFcAe1A44",
				"name": "Kassandra",
				"decimals": 18
			},
			"0x707ebea96efc0bd84d9497048072037dcbba2bc0": {
				"ticker": "YUM",
				"address": "0x707ebea96efc0Bd84D9497048072037dCBBA2Bc0",
				"name": "King Cake",
				"decimals": 10
			},
			"0xf70aa9c275bebd0e667a9fdb656c6ea3d0252952": {
				"ticker": "STAR",
				"address": "0xf70Aa9c275beBD0E667a9Fdb656C6EA3D0252952",
				"name": "OMG Star",
				"decimals": 10
			},
			"0xab8b846eb66920a7458186ea7fdcb9f127e89f79": {
				"ticker": "TAPL",
				"address": "0xAb8b846Eb66920a7458186ea7fDcb9f127e89F79",
				"name": "Treasure Apple",
				"decimals": 12
			},
			"0x221f6954cb0c55afda458ccc6b0183efb9ac3dfe": {
				"ticker": "T102",
				"address": "0x221f6954CB0c55Afda458cCc6B0183eFB9aC3dfE",
				"name": "Testing102",
				"decimals": 18
			},
			"0x6353e4f938e7c78dae0e2630ab3e84342803a2d9": {
				"ticker": "SSD",
				"address": "0x6353e4f938E7C78daE0e2630ab3E84342803A2D9",
				"name": "ShibaSnowDAO",
				"decimals": 9
			},
			"0x8bd9e29346c40e655cbce446fd4c62504046ac83": {
				"ticker": "TIGER",
				"address": "0x8bD9e29346c40E655CBce446fD4C62504046Ac83",
				"name": "TIGER ISLAND",
				"decimals": 8
			},
			"0xee2e2868272fd6d51d78412d849500753f9dd579": {
				"ticker": "ASTRO",
				"address": "0xee2e2868272FD6D51D78412d849500753f9DD579",
				"name": "ASTRO",
				"decimals": 18
			},
			"0x2b1302bcb6305ef6220028b28b165e5a358f0b3c": {
				"ticker": "yball",
				"address": "0x2b1302bcB6305eF6220028B28b165e5a358F0B3C",
				"name": "Yellow Ball",
				"decimals": 18
			},
			"0xfb1367f38da2e452e862f50952795e1d6f0fdd95": {
				"ticker": "SHITAVAX",
				"address": "0xFB1367F38da2e452e862f50952795E1D6F0FDd95",
				"name": "BUYTHISSHIT",
				"decimals": 6
			},
			"0xb24d39c4f65b32678e708ca35c04655f2ccf0b9a": {
				"ticker": "ASTRO",
				"address": "0xB24d39c4F65B32678E708Ca35c04655F2Ccf0b9a",
				"name": "Astro Token",
				"decimals": 18
			},
			"0x3af0eb8bcbd4c4c6e26e309c4e47af59bad5fc2f": {
				"ticker": "PINT",
				"address": "0x3Af0eB8BcBd4C4C6E26e309c4E47Af59Bad5FC2f",
				"name": "pub.finance",
				"decimals": 18
			},
			"0x33333ee26a7d02e41c33828b42fb1e0889143477": {
				"ticker": "LIQR",
				"address": "0x33333ee26a7d02e41c33828B42Fb1E0889143477",
				"name": "LIQR",
				"decimals": 18
			},
			"0xf198b91637ba185b74af1e55bd54cb74dc5dd5a3": {
				"ticker": "APLB",
				"address": "0xf198B91637Ba185B74Af1e55Bd54cB74dC5dd5A3",
				"name": "Apple Beast",
				"decimals": 9
			},
			"0x9acc9aff73de48c58d36edc2eb4e38d6778a378f": {
				"ticker": "CAKE",
				"address": "0x9ACc9aFF73De48c58d36EdC2eb4e38D6778A378f",
				"name": "Cake Cash",
				"decimals": 11
			},
			"0x6e7f5c0b9f4432716bdd0a77a3601291b9d9e985": {
				"ticker": "SPORE",
				"address": "0x6e7f5C0b9f4432716bDd0a77a3601291b9D9e985",
				"name": "Spore.Finance",
				"decimals": 9
			},
			"0xddaaad7366b455aff8e7c82940c43ceb5829b604": {
				"ticker": "mYAK",
				"address": "0xdDAaAD7366B455AfF8E7c82940C43CEB5829B604",
				"name": "MiniYAK",
				"decimals": 12
			},
			"0x336b653d809f027b6764420166ff5d3887b2b217": {
				"ticker": "CHI",
				"address": "0x336B653D809F027B6764420166fF5d3887b2b217",
				"name": "Hachi Chain",
				"decimals": 10
			},
			"0x79d36fcf95c5e1875416f3ba319ddd481801f4c0": {
				"ticker": "AKITA",
				"address": "0x79d36FCf95c5e1875416F3BA319Ddd481801F4c0",
				"name": "Akita AVA",
				"decimals": 12
			},
			"0x1c778bcd11bd6cba834c7b6a2aaf51ae05eeffb8": {
				"ticker": "CAKE",
				"address": "0x1C778bcd11bd6Cba834C7B6A2aaF51AE05eEffB8",
				"name": "Cake Starter",
				"decimals": 9
			},
			"0x98fb95237014cfe011b115c921202daa39b9a872": {
				"ticker": "PGOL",
				"address": "0x98fb95237014cFE011B115C921202daA39b9a872",
				"name": "Project Gold",
				"decimals": 9
			},
			"0xdf35af8ff6dba70220710229d25fdf457e30ca5f": {
				"ticker": "TORTUGA",
				"address": "0xDf35af8Ff6dBa70220710229d25FdF457e30ca5f",
				"name": "Tortuga Capital",
				"decimals": 9
			},
			"0xe5c987f00ebdd4e9414de30c2a0ff61e34d7d860": {
				"ticker": "MILK",
				"address": "0xe5C987F00EBdd4E9414dE30C2A0fF61e34D7d860",
				"name": "MILK",
				"decimals": 18
			},
			"0xf64668031242824a498fe0c2086e85678ec0f39b": {
				"ticker": "WHALE",
				"address": "0xF64668031242824a498FE0C2086e85678EC0F39B",
				"name": "Sushi Whale",
				"decimals": 10
			},
			"0x382c591021ddebdfcb4c6f674d573512fe6a6711": {
				"ticker": "IMPR",
				"address": "0x382C591021dDebdfCb4C6f674d573512fe6a6711",
				"name": "Imperio DAO",
				"decimals": 18
			},
			"0xef4988cbe89316fa12650dcc036be2b242895306": {
				"ticker": "BAMBOO",
				"address": "0xef4988cbe89316fa12650DcC036bE2B242895306",
				"name": "BambooToken",
				"decimals": 18
			},
			"0xa4898cc2229ad6cf88f813a01c9fb2c7244164db": {
				"ticker": "MOON",
				"address": "0xA4898cc2229Ad6CF88f813A01C9fB2C7244164db",
				"name": "Safe Moon",
				"decimals": 10
			},
			"0xc319d7228e26d47aa1d1004b41054120c02560e5": {
				"ticker": "APL",
				"address": "0xC319D7228e26d47aa1D1004b41054120C02560E5",
				"name": "Mini Apple",
				"decimals": 9
			},
			"0x90018d8c20b39038c4f465cc09fa61f496fb24e0": {
				"ticker": "MARS",
				"address": "0x90018d8C20B39038c4f465CC09FA61F496Fb24E0",
				"name": "Mars Project",
				"decimals": 11
			},
			"0xfb98b335551a418cd0737375a2ea0ded62ea213b": {
				"ticker": "PENDLE",
				"address": "0xfB98B335551a418cD0737375a2ea0ded62Ea213b",
				"name": "Pendle",
				"decimals": 18
			},
			"0xeb37f4f6dbdc76683f7fb7d0b6006a0c06e78bbf": {
				"ticker": "CORGI",
				"address": "0xeb37f4F6dBdc76683f7fB7d0B6006A0c06E78bbf",
				"name": "Corgi Finance",
				"decimals": 12
			},
			"0x345e29bcedf39dc2f98f642309c3cde2b53d164e": {
				"ticker": "NODA",
				"address": "0x345E29bCeDF39dC2f98f642309C3cde2b53D164e",
				"name": "NODA",
				"decimals": 9
			},
			"0xaf5e9f43bf307c4abf701b4e4dfd2cb8cfa35cb1": {
				"ticker": "ARA",
				"address": "0xaf5e9f43bF307c4aBF701b4E4DfD2cb8cfa35Cb1",
				"name": "Ara finance",
				"decimals": 18
			},
			"0x1b5c6efac72745d822fd03285c07b91dfd59bb57": {
				"ticker": "GLD",
				"address": "0x1b5C6efAc72745d822FD03285c07b91Dfd59BB57",
				"name": "Wrapped Gold",
				"decimals": 10
			},
			"0x214db107654ff987ad859f34125307783fc8e387": {
				"ticker": "FXS",
				"address": "0x214DB107654fF987AD859F34125307783fC8e387",
				"name": "Frax Share",
				"decimals": 18
			},
			"0xfeaef5e1baf48779e4958f8bc2c9748b113413a3": {
				"ticker": "MAG",
				"address": "0xfeaef5E1baf48779e4958f8bc2c9748b113413a3",
				"name": "Magnet",
				"decimals": 9
			},
			"0x815e86591b067f2ee67c0cc34d8cf1ba891b1283": {
				"ticker": "FUCKS",
				"address": "0x815E86591b067F2eE67C0CC34d8cf1BA891B1283",
				"name": "ZER0 FUCKS",
				"decimals": 9
			},
			"0x7bb00f26500d5a973143dac2c4afe54c57922d8a": {
				"ticker": "APLA",
				"address": "0x7bb00f26500d5a973143dAc2C4afE54c57922D8a",
				"name": "Pineapple AI",
				"decimals": 10
			},
			"0xd27a437350b3894750e4032171b06f03c62c9ebf": {
				"ticker": "KSBA",
				"address": "0xd27a437350b3894750E4032171b06F03C62c9eBf",
				"name": "King Shiba",
				"decimals": 11
			},
			"0xda8c7bb197207863cb0f2fb45cdc2c38d216ac24": {
				"ticker": "VIPER",
				"address": "0xda8C7Bb197207863cB0F2Fb45cDc2C38d216Ac24",
				"name": "Viper Nodes",
				"decimals": 9
			},
			"0x9466ab927611725b9af76b9f31b2f879ff14233d": {
				"ticker": "PAE",
				"address": "0x9466Ab927611725B9AF76b9F31B2F879Ff14233d",
				"name": "Ripae",
				"decimals": 18
			},
			"0x98716351b2660f8ebbe4bafb34a07f9c4aa35e14": {
				"ticker": "SYMBIOT",
				"address": "0x98716351b2660F8Ebbe4bAfB34A07f9c4aA35E14",
				"name": "SYMBIOT",
				"decimals": 18
			},
			"0x7637c2d7a6ad3846e448f02b283fb4eba67d4f39": {
				"ticker": "GEMS",
				"address": "0x7637C2d7a6AD3846e448F02B283Fb4EBa67d4f39",
				"name": "Avax Gems",
				"decimals": 9
			},
			"0xa56f9a54880afbc30cf29bb66d2d9adcdcaeadd6": {
				"ticker": "QI",
				"address": "0xA56F9A54880afBc30CF29bB66d2D9ADCdcaEaDD6",
				"name": "Qi Dao Protocol",
				"decimals": 18
			},
			"0x4123f55048889fa021dfb6de4b4071c6f90112ed": {
				"ticker": "SIFU",
				"address": "0x4123F55048889fa021dfb6dE4B4071c6F90112ed",
				"name": "Sifu Inu",
				"decimals": 18
			},
			"0x1b88d7ad51626044ec62ef9803ea264da4442f32": {
				"ticker": "ZOO",
				"address": "0x1B88D7aD51626044Ec62eF9803EA264DA4442F32",
				"name": "ZooToken",
				"decimals": 18
			},
			"0xc43e85277e76112d5e4007dd4ebf521abc4694fe": {
				"ticker": "SHBM",
				"address": "0xC43E85277e76112D5e4007Dd4ebf521abc4694fe",
				"name": "Shiba Monster",
				"decimals": 10
			},
			"0x3d673e759f1971d305ddc79849d1bc8820763281": {
				"ticker": "SCRG",
				"address": "0x3D673E759F1971d305ddC79849d1Bc8820763281",
				"name": "Sushi Corgi",
				"decimals": 9
			},
			"0x2ed4e7b6d259181b5061ffdc663ff152e6df70ad": {
				"ticker": "SHIB",
				"address": "0x2eD4E7b6d259181b5061FFdc663FF152E6dF70ad",
				"name": "Shiba Network",
				"decimals": 12
			},
			"0x156a29e17d9773bc6a934ca864ff3a3639108148": {
				"ticker": "TNODES",
				"address": "0x156a29E17D9773Bc6A934Ca864fF3a3639108148",
				"name": "TNODES",
				"decimals": 18
			},
			"0x23243a73cf380eb7e39d235c3a3ff31a9a5acf91": {
				"ticker": "HENTAI",
				"address": "0x23243A73cf380Eb7e39d235c3A3ff31A9a5ACF91",
				"name": "HentaiDAO",
				"decimals": 9
			},
			"0xf9dac5b385ebf0a1d2d3f706069b9b8f1848c1ad": {
				"ticker": "MOON",
				"address": "0xf9dAc5B385EbF0A1D2d3F706069b9b8f1848c1AD",
				"name": "Project Moon",
				"decimals": 12
			},
			"0xa1144a6a1304bd9cbb16c800f7a867508726566e": {
				"ticker": "BAG",
				"address": "0xa1144a6A1304bd9cbb16c800F7a867508726566E",
				"name": "Baguette",
				"decimals": 18
			},
			"0x2394682078783e5c130f98a90a2715b0c84a4d41": {
				"ticker": "CrownNode",
				"address": "0x2394682078783E5C130F98a90A2715B0c84A4D41",
				"name": "Crown Node",
				"decimals": 18
			},
			"0xe120e27d92b280453fa29d71d808354087bf5dc1": {
				"ticker": "PNPL",
				"address": "0xe120E27D92B280453fa29d71D808354087bf5dC1",
				"name": "Pineapple Diamond",
				"decimals": 12
			},
			"0x9235b893a3e61a14b2d02a91ec1394fbf411689e": {
				"ticker": "ICE",
				"address": "0x9235B893a3e61a14b2D02a91EC1394fbF411689E",
				"name": "ICE",
				"decimals": 9
			},
			"0xd7861675597e19dcc0d7dce4770853d8b444bb82": {
				"ticker": "SHIB",
				"address": "0xd7861675597E19Dcc0d7dcE4770853D8B444bB82",
				"name": "Exo Shiba",
				"decimals": 12
			},
			"0xf5ab9500bf37a1676d3a571a1f284cee1c4c7483": {
				"ticker": "MOON",
				"address": "0xF5AB9500Bf37a1676d3A571A1f284Cee1c4C7483",
				"name": "Moon Treasure",
				"decimals": 11
			},
			"0x242f0df62038deddfec962d776462170cab57eea": {
				"ticker": "THN",
				"address": "0x242f0dF62038dedDFEC962d776462170cAb57eEa",
				"name": "Wrapped Thunder",
				"decimals": 9
			},
			"0x438416afa3d5a10afe1cda4e563ed378f21fc84b": {
				"ticker": "T101",
				"address": "0x438416AFa3D5a10afE1cDA4e563Ed378F21fC84B",
				"name": "Testing101",
				"decimals": 18
			},
			"0x431d5dff03120afa4bdf332c61a6e1766ef37bdb": {
				"ticker": "JPYC",
				"address": "0x431D5dfF03120AFA4bDf332c61A6e1766eF37BDB",
				"name": "JPY Coin",
				"decimals": 18
			},
			"0x61ce8e4a56dad2d235379a6976487efc8edc7de9": {
				"ticker": "STR",
				"address": "0x61ce8e4a56Dad2D235379A6976487efC8edC7De9",
				"name": "Star Diamond",
				"decimals": 11
			},
			"0xc9d40995e911fcd6f643d64aabba130eb1e45649": {
				"ticker": "GTH",
				"address": "0xc9d40995E911fcd6F643D64aAbbA130Eb1E45649",
				"name": "Glitch DAO",
				"decimals": 9
			},
			"0xc38f41a296a4493ff429f1238e030924a1542e50": {
				"ticker": "SNOB",
				"address": "0xC38f41A296A4493Ff429F1238e030924A1542e50",
				"name": "Snowball",
				"decimals": 18
			},
			"0x591ace9dad1191d9e782ce70d2ac8a0a228d21c8": {
				"ticker": "FLY",
				"address": "0x591AcE9DaD1191D9E782cE70D2AC8a0A228D21c8",
				"name": "FKY",
				"decimals": 18
			},
			"0x45c13620b55c35a5f539d26e88247011eb10fdbd": {
				"ticker": "HCT",
				"address": "0x45C13620B55C35A5f539d26E88247011Eb10fDbd",
				"name": "Hurricane Token",
				"decimals": 18
			},
			"0x8f2b8bbecefebedb391327df5f724591e62fefea": {
				"ticker": "ELXR",
				"address": "0x8F2B8bbEcefebEdb391327DF5f724591e62FEFEa",
				"name": "ELXR",
				"decimals": 6
			},
			"0xff9c36cc631395d1b01806ddc4e205979da64a2f": {
				"ticker": "TOMS",
				"address": "0xFf9c36CC631395D1b01806dDC4E205979Da64a2f",
				"name": "Fantom Swap",
				"decimals": 12
			},
			"0xbda4f33add1ec68c052f6873ee427020213f9e2c": {
				"ticker": "AKITA",
				"address": "0xBDA4f33ADD1ec68C052f6873ee427020213F9E2c",
				"name": "Sushi Akita",
				"decimals": 9
			},
			"0x9d90340c443fe0d4c071f7cd9e4b346495ea9a9a": {
				"ticker": "DOGE",
				"address": "0x9d90340C443Fe0d4c071F7cd9e4B346495Ea9a9a",
				"name": "Project Doge",
				"decimals": 10
			},
			"0x91bfd67c9fc7c489ed51839e560c5a3bfbf21dc0": {
				"ticker": "GRAVE",
				"address": "0x91Bfd67C9Fc7C489Ed51839e560c5A3bfbf21Dc0",
				"name": "grave.finance",
				"decimals": 18
			},
			"0xf04cc4ada5205cb1d5a5c9645741449bf1e136af": {
				"ticker": "STND",
				"address": "0xF04cc4ADa5205Cb1d5A5C9645741449bF1E136AF",
				"name": "Sushi Thunder",
				"decimals": 10
			},
			"0xf55b4c91f58b7e5f4ed07322e50f6ed8eaf9ef96": {
				"ticker": "LAPE",
				"address": "0xF55B4c91f58b7E5F4eD07322e50F6ED8eaf9Ef96",
				"name": "Little Ape",
				"decimals": 10
			},
			"0x8e4c85088383d24a12931404a871b48aeba2bf41": {
				"ticker": "ELN",
				"address": "0x8E4c85088383d24a12931404a871b48aEba2bF41",
				"name": "Wrapped Elon",
				"decimals": 10
			},
			"0xca11539d20fcaa07dcfb33ed0f3d4b051b08c9dc": {
				"ticker": "MRS",
				"address": "0xcA11539D20FcAa07dCfB33Ed0f3d4B051B08C9DC",
				"name": "Wrapped Mars",
				"decimals": 10
			},
			"0xafe52bac66e9fb8efe72f80a5eb97e65020d15d1": {
				"ticker": "SUMATRAN",
				"address": "0xAFE52BAc66E9Fb8eFe72F80A5EB97E65020D15d1",
				"name": "Sumatran Inu",
				"decimals": 18
			},
			"0xb094ffef135e867e24b6bf75dc2f9d1baae34ea0": {
				"ticker": "SODA",
				"address": "0xb094fFeF135E867e24b6bf75dc2f9D1baAe34eA0",
				"name": "Soda",
				"decimals": 18
			},
			"0xe7db05e6c5f6ec055a276bdb77043c1d0b599abb": {
				"ticker": "FLOKI",
				"address": "0xe7dB05e6c5F6ec055a276bDb77043C1d0B599Abb",
				"name": "Project Floki",
				"decimals": 9
			},
			"0xa272f8e4c8284e1011080f49684783775f694483": {
				"ticker": "WHALE",
				"address": "0xA272F8e4c8284E1011080F49684783775f694483",
				"name": "Whale Block",
				"decimals": 11
			},
			"0x9bb18df32ed450844e7040d5261ed32e20721e63": {
				"ticker": "PNPL",
				"address": "0x9Bb18DF32Ed450844e7040d5261ed32E20721e63",
				"name": "King Pineapple",
				"decimals": 9
			},
			"0x9e760c8ef37133be130e66f73fb6f45341973a8d": {
				"ticker": "CRG",
				"address": "0x9E760c8Ef37133bE130e66f73fb6F45341973a8d",
				"name": "Corgi King",
				"decimals": 12
			},
			"0x3ee0487079ed267f0b553dfc816aedbca7b5a487": {
				"ticker": "WHL",
				"address": "0x3EE0487079ED267f0b553DFC816AEDBCa7B5a487",
				"name": "Whale Markets",
				"decimals": 10
			},
			"0x5418a3cc0fd5f8a202d655bb8e49c918de490db7": {
				"ticker": "SAPL",
				"address": "0x5418A3cC0FD5F8a202D655bB8e49C918De490Db7",
				"name": "Safe Apple",
				"decimals": 12
			},
			"0xe28e578a5fbe4a2fbe6051ddb414e830e27bf42b": {
				"ticker": "RIG",
				"address": "0xe28e578a5fbE4a2fBE6051DDb414e830E27bf42b",
				"name": "RigToken",
				"decimals": 18
			},
			"0xea369476568f6f74b8cd0eca5550430931d19ef2": {
				"ticker": "CCHI",
				"address": "0xea369476568F6f74b8Cd0Eca5550430931d19EF2",
				"name": "Crypto Chimp",
				"decimals": 10
			},
			"0x765a0bb2835ffe7c1462878db7c649e5a57a93ee": {
				"ticker": "APL",
				"address": "0x765A0bB2835ffe7C1462878Db7c649E5A57a93EE",
				"name": "Baby Apple",
				"decimals": 12
			},
			"0x7e9b4376ef26325bf83772c7e509e65909a7b398": {
				"ticker": "GOS",
				"address": "0x7e9b4376Ef26325bf83772c7e509e65909a7b398",
				"name": "King Ghost",
				"decimals": 9
			},
			"0x547b2719e9c284c95749b0dd72bc5955a4ac2f60": {
				"ticker": "APL",
				"address": "0x547B2719E9c284c95749b0dD72BC5955a4Ac2f60",
				"name": "King Apple",
				"decimals": 11
			},
			"0x0a153c8e6db60ee047c3df827b57761db29fd9e6": {
				"ticker": "MARS",
				"address": "0x0a153C8e6dB60EE047c3dF827b57761dB29fd9E6",
				"name": "Meta Mars",
				"decimals": 9
			},
			"0xb7187b8aefa4a504e8a078604ac3eb2094c2f4bb": {
				"ticker": "PCHI",
				"address": "0xB7187b8AEfA4a504e8a078604aC3EB2094c2f4bB",
				"name": "Project Chimp",
				"decimals": 10
			},
			"0x4123c9a65d4b7b415e472b1ed64635472a344ba8": {
				"ticker": "DB",
				"address": "0x4123C9A65D4B7B415E472b1eD64635472A344BA8",
				"name": "DiamondBank",
				"decimals": 9
			},
			"0x2e64d0251d497ed3afaef19be77ddc220085e859": {
				"ticker": "SHB",
				"address": "0x2E64D0251D497ED3afAEf19be77DDc220085E859",
				"name": "Shiba AVA",
				"decimals": 12
			},
			"0x9565da78e5dd251c4d65b760de823787eec64e0d": {
				"ticker": "TND",
				"address": "0x9565Da78E5Dd251C4d65b760DE823787eeC64e0D",
				"name": "Baby Thunder",
				"decimals": 12
			},
			"0x9636eaab54aca30803500764adfb305bbe163b0f": {
				"ticker": "SS",
				"address": "0x9636EAab54aca30803500764ADfB305bbE163b0f",
				"name": "Space Ship",
				"decimals": 9
			},
			"0x85a3d29c69c5bcdf4ca0234719c769605ce8ae41": {
				"ticker": "SNIPER",
				"address": "0x85A3d29C69C5bcdf4CA0234719C769605cE8Ae41",
				"name": "Sniper Nodes",
				"decimals": 9
			},
			"0x47536f17f4ff30e64a96a7555826b8f9e66ec468": {
				"ticker": "CRV",
				"address": "0x47536F17F4fF30e64A96a7555826b8f9e66ec468",
				"name": "Curve DAO Token",
				"decimals": 18
			},
			"0xf932adba162c9a82a3b46696b7baefd8e44eaa12": {
				"ticker": "ECRG",
				"address": "0xF932AdBa162C9A82a3b46696b7BAeFD8E44EAA12",
				"name": "Exo Corgi",
				"decimals": 12
			},
			"0x35d8106ed31d3a94a410dfaa98d00345a32478f9": {
				"ticker": "ORBIT",
				"address": "0x35d8106eD31d3a94A410dfAa98D00345a32478F9",
				"name": "Orbitlaunch DAO",
				"decimals": 9
			},
			"0x4424387c5d8a203b7c668715ac6cce0cd0a12d72": {
				"ticker": "FLK",
				"address": "0x4424387C5d8A203B7C668715Ac6CCE0CD0A12D72",
				"name": "Floki Classic",
				"decimals": 10
			},
			"0x670a23ef9d4ec473eb97c361f283da0b7ce0727d": {
				"ticker": "APLD",
				"address": "0x670a23ef9D4EC473eB97C361F283dA0b7Ce0727D",
				"name": "Pineapple Dapp",
				"decimals": 9
			},
			"0xc8a2436386707bf7cf68cbff81490a0a6f83a982": {
				"ticker": "CHIMP",
				"address": "0xc8A2436386707BF7cF68cBFf81490a0A6F83a982",
				"name": "OMG Chimp",
				"decimals": 11
			},
			"0x8e05b136c32dd2f2ecd3eda6ae0e104dc3d2f2bf": {
				"ticker": "NY",
				"address": "0x8E05B136c32dd2F2eCD3EDa6aE0E104Dc3D2F2bf",
				"name": "nyanheroes.com",
				"decimals": 18
			},
			"0x8d689389c7afe53ddc4bfd39b6a670c13ea76931": {
				"ticker": "SSHARE",
				"address": "0x8d689389c7AfE53dDc4BFd39B6a670c13ea76931",
				"name": "SSHARE",
				"decimals": 18
			},
			"0xb3f4a6be2387b1053391f427e3e773d2bdb42e03": {
				"ticker": "HYBRID",
				"address": "0xB3F4A6bE2387b1053391f427e3E773D2bdb42E03",
				"name": "Hybrid Finance",
				"decimals": 18
			},
			"0x09940e5a8b63bb531b85472eaa234c7f4223d36c": {
				"ticker": "TND",
				"address": "0x09940E5A8B63bb531b85472eAa234C7F4223d36c",
				"name": "Little Thunder",
				"decimals": 10
			},
			"0x6a557ed6275f6f421de2b6a9c8a840ea6085612a": {
				"ticker": "THN",
				"address": "0x6A557ED6275f6f421de2B6A9c8A840eA6085612A",
				"name": "Thunder AI",
				"decimals": 12
			},
			"0x61ea6480c79a90a17ee5b41bdf79f7859148a066": {
				"ticker": "FLK",
				"address": "0x61ea6480c79a90A17EE5b41BdF79F7859148A066",
				"name": "Floki Coin",
				"decimals": 10
			},
			"0x469bed926d2e4fa0a438c3c2ed2588a104602948": {
				"ticker": "APL",
				"address": "0x469BEd926D2e4FA0a438c3c2eD2588a104602948",
				"name": "Crypto Apple",
				"decimals": 11
			},
			"0xe96c6843d8fd94ec76e127a8a01fc90cd905d66b": {
				"ticker": "Comet",
				"address": "0xE96C6843D8Fd94ec76e127A8A01fc90CD905D66b",
				"name": "Comet Node",
				"decimals": 6
			},
			"0x957c11a2d89a76bc82d4dd179d5d53a82739f310": {
				"ticker": "AKAC",
				"address": "0x957c11a2D89A76bc82d4Dd179D5D53A82739f310",
				"name": "Akita Cash",
				"decimals": 10
			},
			"0x3aac8dc01cb43695951b702ef6313f6a2d79c5bf": {
				"ticker": "WHALE",
				"address": "0x3AaC8DC01CB43695951B702eF6313f6A2d79C5BF",
				"name": "Whale BTC",
				"decimals": 11
			},
			"0xd6be5d4eebb8a7ecc4cca3b2682d0bd63c2ecc6a": {
				"ticker": "BITE",
				"address": "0xD6bE5d4EEbB8a7ecc4ccA3B2682D0Bd63C2ecC6A",
				"name": "Snake Protocol",
				"decimals": 9
			},
			"0x570bc763051ef41d53ce83d54a389516c2391bb5": {
				"ticker": "BZAP",
				"address": "0x570bc763051eF41D53Ce83d54A389516c2391Bb5",
				"name": "Baby Thunder",
				"decimals": 12
			},
			"0xc0616edc952985319709ef2170aeaaf12860288f": {
				"ticker": "AKTN",
				"address": "0xc0616edc952985319709EF2170AeAAf12860288F",
				"name": "Akita NET",
				"decimals": 9
			},
			"0xc3d498ee9c6078b962bb7b4522e449bc6ae7bc88": {
				"ticker": "AKT",
				"address": "0xC3D498ee9c6078b962BB7B4522e449bC6aE7BC88",
				"name": "Akita Coin",
				"decimals": 9
			},
			"0xefc27f2af38ff8de83766af661fd02129c23508f": {
				"ticker": "CMP",
				"address": "0xefc27F2AF38FF8dE83766Af661Fd02129C23508F",
				"name": "Crypto Chimp",
				"decimals": 12
			},
			"0x1ca4b92f89a71ce493553e7ce1e21ef04c787da7": {
				"ticker": "FLK",
				"address": "0x1CA4B92F89A71Ce493553E7Ce1e21Ef04C787Da7",
				"name": "Treasure Floki",
				"decimals": 10
			},
			"0xc4784acc90bfb98f5ba59969ebefa9e6f3f571d8": {
				"ticker": "WHALE",
				"address": "0xC4784acc90Bfb98f5Ba59969ebEFA9e6f3F571D8",
				"name": "Whale AVA",
				"decimals": 12
			},
			"0xe0dffd31fb7aa064d982e2ccf1bef70703f74a1e": {
				"ticker": "TCHI",
				"address": "0xe0dfFd31fB7aA064d982e2CCf1BEf70703F74a1E",
				"name": "Treasure Chimp",
				"decimals": 9
			},
			"0xfdeb73a638e3c55e7806195a8d2869c0244ef145": {
				"ticker": "ELON",
				"address": "0xfdEB73a638E3c55E7806195a8d2869C0244Ef145",
				"name": "Elon Diamond",
				"decimals": 12
			},
			"0xb59882d8aabc7940b4baced0cb14d3b1b8201dd0": {
				"ticker": "FegAVAX",
				"address": "0xB59882d8aaBC7940b4BacED0cb14d3b1B8201dd0",
				"name": "FegAVAX",
				"decimals": 9
			},
			"0xb940cd96e95d8d016e6fcdf59f3114d081536dda": {
				"ticker": "N2",
				"address": "0xB940Cd96E95D8D016E6fcdF59F3114d081536ddA",
				"name": "Node Squared",
				"decimals": 9
			},
			"0xe53846be459f219eb2dcfbe9190af15512585c25": {
				"ticker": "TOMI",
				"address": "0xe53846BE459f219eB2DcFBe9190af15512585c25",
				"name": "Fantom INC",
				"decimals": 12
			},
			"0x99e3278e8dbd25dcd14f19942bcf14b5593e612c": {
				"ticker": "DGE",
				"address": "0x99e3278E8DbD25dCD14f19942Bcf14b5593E612c",
				"name": "Doge Swap",
				"decimals": 10
			},
			"0xd2b9631c4128f4fc3c2a36bb1ce975da0bbf0f68": {
				"ticker": "GMI",
				"address": "0xD2B9631c4128F4Fc3c2A36bB1cE975dA0Bbf0f68",
				"name": "ThousandXcoin",
				"decimals": 9
			},
			"0xa7b47575a9b4c2ebf3e6f9ad1d84932bd7af380f": {
				"ticker": "DEGEN",
				"address": "0xA7b47575A9b4c2EbF3e6f9ad1d84932bD7AF380F",
				"name": "DegenFi Token",
				"decimals": 18
			},
			"0xaefb2b7a5d73989f318517828d83c64c15d39e67": {
				"ticker": "MARS",
				"address": "0xAEfb2b7a5D73989f318517828d83C64c15D39E67",
				"name": "Super Mars",
				"decimals": 9
			},
			"0x7b564b5d85be0eb9760e929d935a98d914ce061e": {
				"ticker": "SnowBurn",
				"address": "0x7b564B5d85bE0Eb9760e929d935A98D914CE061e",
				"name": "SnowBurn",
				"decimals": 18
			},
			"0x53dafc36aec50b460f5aad8f3155fc02b2edc4c4": {
				"ticker": "DOGE",
				"address": "0x53Dafc36AEc50B460F5AaD8F3155fc02b2edc4C4",
				"name": "Super Doge",
				"decimals": 12
			},
			"0x88c7c5a64bb86876935272c1755b25b1118a9eef": {
				"ticker": "TAPL",
				"address": "0x88c7C5a64BB86876935272c1755b25B1118a9EeF",
				"name": "Treasure Apple",
				"decimals": 10
			},
			"0xfee44bb04bc969e1dff7cb8f895d02ce9f8253b1": {
				"ticker": "GLD",
				"address": "0xfEE44BB04bC969e1dFf7cb8F895d02CE9F8253B1",
				"name": "Gold Protocol",
				"decimals": 12
			},
			"0x26b6b6e4d856aef34ef33b880c63d2b66fb4ba9f": {
				"ticker": "2HeDao",
				"address": "0x26b6B6e4d856Aef34EF33B880C63D2B66FB4Ba9f",
				"name": "2-Helium DAO",
				"decimals": 8
			},
			"0x802d484ea6e722deb19ef5f66a90cb983a321637": {
				"ticker": "CMPG",
				"address": "0x802d484Ea6E722dEb19ef5f66a90CB983a321637",
				"name": "Chimp Gaming",
				"decimals": 12
			},
			"0x45324950c6ba08112ebf72754004a66a0a2b7721": {
				"ticker": "PGL",
				"address": "0x45324950c6ba08112EbF72754004a66a0a2b7721",
				"name": "Pangolin Liquidity",
				"decimals": 18
			},
			"0x8ea1127f346c2beabc15ded9b462e2ba6d4ebed4": {
				"ticker": "GLT",
				"address": "0x8eA1127f346C2bEABC15DEd9B462e2bA6d4ebED4",
				"name": "Sushi Glitch",
				"decimals": 11
			},
			"0x9a8e0217cd870783c3f2317985c57bf570969153": {
				"ticker": "MAGIC",
				"address": "0x9A8E0217cD870783c3f2317985C57Bf570969153",
				"name": "Magic",
				"decimals": 18
			},
			"0xc77996a57529ec553be0ea19cc8c92669488f892": {
				"ticker": "MOON",
				"address": "0xc77996a57529eC553be0EA19cC8c92669488f892",
				"name": "Moon DAO",
				"decimals": 9
			},
			"0x42ca81bf5d2d8c0df69ec10bb8e6915b18612b5c": {
				"ticker": "APET",
				"address": "0x42ca81bF5d2D8C0dF69EC10bb8E6915b18612b5C",
				"name": "Ape Token",
				"decimals": 12
			},
			"0x9381d5e65b953517f99cc87ab0a38646b6577ae0": {
				"ticker": "AKITA",
				"address": "0x9381d5e65B953517f99Cc87AB0a38646B6577ae0",
				"name": "Akita INC",
				"decimals": 11
			},
			"0xe896cdeaac9615145c0ca09c8cd5c25bced6384c": {
				"ticker": "PEFI",
				"address": "0xe896CDeaAC9615145c0cA09C8Cd5C25bced6384c",
				"name": "PenguinToken",
				"decimals": 18
			},
			"0x2ca4df5b0a45cee475a2104b5336d388489fac11": {
				"ticker": "TYUM",
				"address": "0x2cA4df5B0a45cee475a2104B5336d388489fAc11",
				"name": "Treasure Cake",
				"decimals": 12
			},
			"0x5106f787e8778a86d1928ed5ad0b0215dbfa00b8": {
				"ticker": "DAXE",
				"address": "0x5106f787E8778a86D1928ed5ad0B0215dBFA00b8",
				"name": "DAXE",
				"decimals": 18
			},
			"0x186e3307238a809048c97e5710a8b422d41883c0": {
				"ticker": "MSHB",
				"address": "0x186e3307238a809048C97e5710a8b422d41883c0",
				"name": "Meta Shiba",
				"decimals": 9
			},
			"0xf5503a5fd60e997562859306e89a6ebbe9a2c38c": {
				"ticker": "PSBA",
				"address": "0xF5503a5fD60e997562859306e89a6EBbe9a2c38c",
				"name": "Project Shiba",
				"decimals": 11
			},
			"0xfe6b19286885a4f7f55adad09c3cd1f906d2478f": {
				"ticker": "SOL",
				"address": "0xFE6B19286885a4F7F55AdAD09C3Cd1f906D2478F",
				"name": "Wrapped SOL (Wormhole)",
				"decimals": 9
			},
			"0x8dc4d5f5d7caa96b93f7095fa0c7a2aa8264816d": {
				"ticker": "OPSY",
				"address": "0x8DC4D5F5d7caA96B93f7095FA0c7a2Aa8264816d",
				"name": "OPSY Token",
				"decimals": 18
			},
			"0x921f99719eb6c01b4b8f0ba7973a7c24891e740a": {
				"ticker": "MAGE",
				"address": "0x921f99719Eb6C01b4B8f0BA7973A7C24891e740A",
				"name": "MetaBrands",
				"decimals": 18
			},
			"0xc2ff0f3fb84e71227ead7f8e1ef1a0ccd9be09ae": {
				"ticker": "MARS",
				"address": "0xc2Ff0F3fB84E71227eAd7F8E1Ef1A0CcD9BE09ae",
				"name": "Mars Finance",
				"decimals": 11
			},
			"0xfcc780a394ebcf46d844790c7e307ba3fa5b741b": {
				"ticker": "CGLD",
				"address": "0xFcc780A394EbCF46d844790c7e307Ba3fA5B741B",
				"name": "Crypto Gold",
				"decimals": 9
			},
			"0x881e314a9f97a295f2cf3dc27f22d382cf0bfa4b": {
				"ticker": "PINATA",
				"address": "0x881e314a9F97A295f2cf3dc27f22D382cf0bFA4B",
				"name": "NFT Management",
				"decimals": 9
			},
			"0x9f3bb52e7c0dffb22a1c7de57eba474b55ccb220": {
				"ticker": "EDBIT",
				"address": "0x9f3Bb52E7C0dFFb22A1C7dE57EBA474B55ccB220",
				"name": "EDBIT",
				"decimals": 18
			},
			"0xf90d99f13e8a90c32699153a0c5c8dca87e83584": {
				"ticker": "SNY",
				"address": "0xF90D99F13e8A90C32699153A0C5c8Dca87E83584",
				"name": "SquidNewYear",
				"decimals": 18
			},
			"0xe349b2dc89e0bcd9ccd1722488379429e889e982": {
				"ticker": "MOON",
				"address": "0xe349b2Dc89E0bCd9CCd1722488379429E889E982",
				"name": "Moon Token",
				"decimals": 10
			},
			"0xe3b5ba3f8477b4fc697291c9e159d41570be70c3": {
				"ticker": "PINS",
				"address": "0xe3B5Ba3f8477b4Fc697291c9e159D41570bE70c3",
				"name": "Pineapple SV",
				"decimals": 10
			},
			"0x1097495778d957cbbe01a8952e883b76715d5f21": {
				"ticker": "SGOS",
				"address": "0x1097495778d957cBBe01a8952e883b76715d5f21",
				"name": "Safe Ghost",
				"decimals": 10
			},
			"0x5023283c29ceedc4b69fcaf905ad0cbab15ea390": {
				"ticker": "FANS",
				"address": "0x5023283C29ceedC4B69fcAF905aD0CBab15ea390",
				"name": "Fantom Swap",
				"decimals": 12
			},
			"0x39912d83acb4a373321387300f4fbe88aa5d6f14": {
				"ticker": "CROWN",
				"address": "0x39912D83acb4A373321387300f4FBE88Aa5d6F14",
				"name": "MidasDAO",
				"decimals": 9
			},
			"0xda1cc0abbc65101a9d1d6fe2dc9e6b1a81947534": {
				"ticker": "PLAYMATES",
				"address": "0xDa1CC0AbBc65101a9d1D6fe2DC9e6B1a81947534",
				"name": "Playmates",
				"decimals": 18
			},
			"0xa89f989738b3d05380e68dd7b1c78efaab6edfaa": {
				"ticker": "PNPL",
				"address": "0xA89F989738B3d05380e68dd7B1c78eFaab6edFAA",
				"name": "Pineapple Swap",
				"decimals": 11
			},
			"0x4c8272f873938c5e216ca4c69f22758fbf78e85c": {
				"ticker": "FTM",
				"address": "0x4C8272f873938c5e216CA4c69F22758FBf78E85c",
				"name": "Fantom Diamond",
				"decimals": 9
			},
			"0xa70fe4b93bbbfea0813f62e83e43a4756d015335": {
				"ticker": "GHST",
				"address": "0xa70fE4B93BbBFea0813f62E83E43a4756D015335",
				"name": "Queen Ghost",
				"decimals": 10
			},
			"0xc9d6a82524cb68678e6a97f5af2789bbe9f2e02c": {
				"ticker": "GLTCH",
				"address": "0xC9d6a82524Cb68678E6a97F5af2789BBE9F2E02c",
				"name": "Glitch Rocket",
				"decimals": 11
			},
			"0xa78c5992cd24ffa409c7ba5cdda9378a018771d8": {
				"ticker": "CAKE",
				"address": "0xA78C5992CD24FFa409c7ba5cdDa9378A018771d8",
				"name": "Cake Classic",
				"decimals": 12
			},
			"0x82514f2d3ec0f6c23ec8fe879034751edffbc8d4": {
				"ticker": "PLAYMATE",
				"address": "0x82514f2D3Ec0f6c23ec8Fe879034751EDFFbC8d4",
				"name": "Red Node District",
				"decimals": 9
			},
			"0xc3321bffe46ec7294304b53b43fdc7b4ae908945": {
				"ticker": "AV",
				"address": "0xC3321bfFe46ec7294304B53B43FDc7b4ae908945",
				"name": "Avocado",
				"decimals": 18
			},
			"0xabe3427bc7440a087e6ed7ce05d041aa29d68369": {
				"ticker": "DOGE",
				"address": "0xABE3427bC7440A087e6Ed7Ce05d041Aa29d68369",
				"name": "Doge Fund",
				"decimals": 9
			},
			"0xe3e5314a08ea74cc3e18920ab3b7da084f7b28d5": {
				"ticker": "AKITA",
				"address": "0xe3e5314a08ea74CC3e18920ab3b7Da084f7b28D5",
				"name": "Akita Diamond",
				"decimals": 10
			},
			"0x9948ea800cd9ea28e19cd64930946235be5d3092": {
				"ticker": "STAR",
				"address": "0x9948ea800CD9Ea28E19cD64930946235Be5D3092",
				"name": "Wrapped Star",
				"decimals": 10
			},
			"0xd876b7f1e2f1d1bf282386755017d78e9cd4d916": {
				"ticker": "PXT",
				"address": "0xd876B7F1e2F1d1bF282386755017D78e9cD4d916",
				"name": "ProjectXNodes",
				"decimals": 9
			},
			"0x32407d1fd7395fd7497c5ec556a0a7ced6140732": {
				"ticker": "FLUX",
				"address": "0x32407D1fd7395fd7497c5EC556a0A7ced6140732",
				"name": "Flux Nodes",
				"decimals": 18
			},
			"0xb73a673a5d9643975e9580e756ad1017188eddbf": {
				"ticker": "APL",
				"address": "0xb73A673A5D9643975e9580e756AD1017188eDDBf",
				"name": "Apple Classic",
				"decimals": 12
			},
			"0x7c4556156d6a4b7293d639548471e03ccf836bb3": {
				"ticker": "AKITA",
				"address": "0x7C4556156d6a4B7293D639548471E03cCf836bb3",
				"name": "Akita Monster",
				"decimals": 12
			},
			"0xdf87eb5217d04b5787ae7f993b4e91c883a44043": {
				"ticker": "Playmates",
				"address": "0xdF87EB5217d04b5787ae7f993B4E91C883a44043",
				"name": "Redlight Node District",
				"decimals": 18
			},
			"0xfaa9c23bb4826201bf048c10b4b8a759b866cd61": {
				"ticker": "TCC",
				"address": "0xFAA9c23BB4826201BF048C10B4B8A759B866cD61",
				"name": "Triple Confirmation Community Token",
				"decimals": 9
			},
			"0xb4e1ba64dafacdae5ce5e8eeb3ca0e42ca13e02f": {
				"ticker": "LUCKY",
				"address": "0xb4e1ba64DAFAcdaE5cE5e8EEb3ca0E42ca13e02f",
				"name": "Lucky DAO",
				"decimals": 18
			},
			"0xb347aafd42fb0175fc7131ee3842e53030f8ca72": {
				"ticker": "PLAYMATES",
				"address": "0xB347AaFD42fb0175fC7131EE3842e53030F8CA72",
				"name": "Playmates",
				"decimals": 18
			},
			"0x51fab256ad2f5e7cbb96dd578c0e942b1f7ec96a": {
				"ticker": "PFTM",
				"address": "0x51FAb256Ad2F5e7CBB96Dd578c0e942B1f7Ec96a",
				"name": "Project Fantom",
				"decimals": 12
			},
			"0x037c416916a05fd371d054a8c28b6d70d41c5efb": {
				"ticker": "PPLE",
				"address": "0x037c416916a05FD371d054A8C28B6d70D41C5eFB",
				"name": "Pineapple Beast",
				"decimals": 12
			},
			"0x363be8eb78952057fecb4aab682eb01b1888b4cf": {
				"ticker": "SSTR",
				"address": "0x363Be8EB78952057fEcB4aaB682eB01b1888B4Cf",
				"name": "Sushi Star",
				"decimals": 12
			},
			"0x6ac61d231113e9b112ce366679007a5ee076d082": {
				"ticker": "PDAD",
				"address": "0x6Ac61D231113E9B112Ce366679007A5ee076d082",
				"name": "Panda Diamond",
				"decimals": 9
			},
			"0x970c9a02099d3de5545591cbdf23fd591607bd3d": {
				"ticker": "ANTG",
				"address": "0x970c9A02099d3dE5545591CBdf23fd591607BD3d",
				"name": "ANTG",
				"decimals": 6
			},
			"0x7b371c1ca607929d9ca17753e141fecc7cc2651f": {
				"ticker": "CAKE",
				"address": "0x7B371c1cA607929d9Ca17753e141feCC7Cc2651f",
				"name": "Fantasy Cake",
				"decimals": 12
			},
			"0xe074c0ee4ac16044f53ef60c3bfe097a006d4c3b": {
				"ticker": "RND",
				"address": "0xe074C0eE4AC16044F53ef60c3bFe097A006D4C3b",
				"name": "Redlight Node District",
				"decimals": 9
			},
			"0x20547edc7ce44be30fa9bd3de82ae909380fd85f": {
				"ticker": "APL",
				"address": "0x20547EdC7Ce44BE30fa9BD3De82Ae909380fd85F",
				"name": "King Pineapple",
				"decimals": 11
			},
			"0xdad847727cd1a7064e6c10cbd21f72ea7a61d5fa": {
				"ticker": "PI",
				"address": "0xDaD847727cd1A7064E6C10CBD21F72Ea7a61D5Fa",
				"name": "Pirate Island",
				"decimals": 9
			},
			"0x2473e5a33400c88ba0338eb3d4ebdf8be54c7169": {
				"ticker": "WHL",
				"address": "0x2473E5A33400C88BA0338Eb3d4EBdF8bE54c7169",
				"name": "Queen Whale",
				"decimals": 9
			},
			"0xae61061d9d952c4abe421109205f9e03be50ef16": {
				"ticker": "APE",
				"address": "0xAE61061D9D952c4ABe421109205F9e03bE50EF16",
				"name": "Fantasy Ape",
				"decimals": 9
			},
			"0x18c8db0ada99a2b0fac6bdb7e9a5439978ff2f59": {
				"ticker": "TND",
				"address": "0x18C8DB0aDA99A2B0Fac6bDB7e9a5439978ff2f59",
				"name": "OMG Thunder",
				"decimals": 12
			},
			"0x089699b995a76a33dea73bb33067f58aba6529cb": {
				"ticker": "WHLR",
				"address": "0x089699b995a76A33deA73bb33067f58aBa6529Cb",
				"name": "Whale Rise",
				"decimals": 11
			},
			"0xb32ac3c79a94ac1eb258f3c830bbdbc676483c93": {
				"ticker": "OSWAP",
				"address": "0xb32aC3C79A94aC1eb258f3C830bBDbc676483c93",
				"name": "OpenSwap",
				"decimals": 18
			},
			"0xfda34e6264deb0913893b00389968ae4cb299a00": {
				"ticker": "GREEN",
				"address": "0xFdA34e6264DEb0913893b00389968AE4CB299A00",
				"name": "Green Candle DAO",
				"decimals": 18
			},
			"0x878930185f84f21bc7688d8bf67570411c2e5b7e": {
				"ticker": "AKA",
				"address": "0x878930185F84f21BC7688D8Bf67570411C2E5B7e",
				"name": "Akita Diamond",
				"decimals": 12
			},
			"0x056d114ff1e01de3bca30f0efa3655df42880e5b": {
				"ticker": "KTE",
				"address": "0x056D114fF1e01dE3BCA30F0Efa3655DF42880e5B",
				"name": "KyteOne",
				"decimals": 18
			},
			"0x306545367a329a6c8119d5c982ffc3c9970b6241": {
				"ticker": "GMONEY",
				"address": "0x306545367A329a6C8119D5c982FfC3c9970b6241",
				"name": "Money Game",
				"decimals": 9
			},
			"0x99323fcee3134c97312c5f6412c129240dcb6ec7": {
				"ticker": "KGOL",
				"address": "0x99323FcEE3134c97312c5f6412c129240dcB6Ec7",
				"name": "King Gold",
				"decimals": 9
			},
			"0x507205439edb285d3ee8c077fe4ed427b90d3aed": {
				"ticker": "SKOL",
				"address": "0x507205439EdB285d3EE8C077Fe4ED427b90d3Aed",
				"name": "Magic Token SKOL",
				"decimals": 18
			},
			"0x8832d30286907612952aafc3d249e79e39732dcc": {
				"ticker": "ASAFTI",
				"address": "0x8832D30286907612952AaFC3D249E79e39732dCC",
				"name": "SafuTitano",
				"decimals": 5
			},
			"0xd81d45e7635400ddd9c028839e9a9ef479006b28": {
				"ticker": "EMBR",
				"address": "0xD81D45E7635400dDD9c028839e9a9eF479006B28",
				"name": "EmbrToken",
				"decimals": 18
			},
			"0x9ccaaa040b8a43874fd1f82f8a1fd17286ea098c": {
				"ticker": "LEO",
				"address": "0x9CcAAa040B8A43874fd1F82F8A1FD17286EA098c",
				"name": "Leonidas",
				"decimals": 5
			},
			"0xb8ef3a190b68175000b74b4160d325fd5024760e": {
				"ticker": "RUG",
				"address": "0xb8EF3a190b68175000B74B4160d325FD5024760e",
				"name": "R U Generous",
				"decimals": 9
			},
			"0x511643d9bce61c13da4e00b3562ac9f724023d58": {
				"ticker": "ELON",
				"address": "0x511643D9bce61C13da4e00b3562ac9f724023D58",
				"name": "Baby Elon",
				"decimals": 12
			},
			"0xe8627e5a6e50d7ed6098cb3dea42b0a2d9bf2604": {
				"ticker": "TND",
				"address": "0xe8627e5a6E50d7eD6098cB3DEA42B0a2D9BF2604",
				"name": "Thunder Treasure",
				"decimals": 10
			},
			"0x064848b4bf7fc53b5199fb769338d40d535487a7": {
				"ticker": "MARS",
				"address": "0x064848b4bF7fC53B5199FB769338D40D535487A7",
				"name": "Crypto Mars",
				"decimals": 9
			},
			"0x711f8778c83eaabdc31105c47c51dc155dd7552c": {
				"ticker": "LDGE",
				"address": "0x711F8778c83eAABDC31105c47C51DC155DD7552c",
				"name": "Little Doge",
				"decimals": 11
			},
			"0x127bf2d080da1fb749542f4274d76951f287ae84": {
				"ticker": "MAGIA",
				"address": "0x127bF2D080da1fB749542f4274d76951F287aE84",
				"name": "Magia Nodes",
				"decimals": 18
			},
			"0x76c2b3f9acd4ad8f259b8f87d7e11df7aeee176d": {
				"ticker": "APE",
				"address": "0x76c2B3f9acd4aD8f259b8F87D7e11dF7aEEE176D",
				"name": "Ape Treasure",
				"decimals": 12
			},
			"0x0a866a33a28e784b7c3db6a0e1ba9b027ed71e8f": {
				"ticker": "ELNF",
				"address": "0x0a866a33a28E784b7c3Db6A0E1Ba9B027ed71e8F",
				"name": "Elon Finance",
				"decimals": 10
			},
			"0x8b82a291f83ca07af22120aba21632088fc92931": {
				"ticker": "WETH",
				"address": "0x8b82A291F83ca07Af22120ABa21632088fC92931",
				"name": "Wrapped Ether (Wormhole)",
				"decimals": 18
			},
			"0x4b150f58022653df16e4542c266a88bd1b18921e": {
				"ticker": "PANR",
				"address": "0x4B150F58022653dF16E4542c266A88bd1B18921E",
				"name": "Panda Robot",
				"decimals": 10
			},
			"0x8cd309e14575203535ef120b5b0ab4dded0c2073": {
				"ticker": "wsOHM",
				"address": "0x8CD309e14575203535EF120b5b0Ab4DDeD0C2073",
				"name": "Wrapped sOHM",
				"decimals": 18
			},
			"0x2522a71a9570f83d8ab2abe61245360413a58e68": {
				"ticker": "APLM",
				"address": "0x2522a71a9570f83d8AB2AbE61245360413a58e68",
				"name": "Apple Markets",
				"decimals": 10
			},
			"0xc6636e205460be6073b65094422910161d767ef2": {
				"ticker": "FUSD",
				"address": "0xc6636e205460bE6073b65094422910161d767ef2",
				"name": "Fortress USD",
				"decimals": 18
			},
			"0x73717af58280443b2dcb4481cddad27a3df63b00": {
				"ticker": "STND",
				"address": "0x73717aF58280443b2dCb4481cddAD27a3dF63b00",
				"name": "Super Thunder",
				"decimals": 12
			},
			"0xe27f68a0258cdab9cad20d1a4a46f2a5373dbdab": {
				"ticker": "MOON",
				"address": "0xe27f68A0258cDaB9cAd20d1a4a46F2a5373DBdAB",
				"name": "Mini Moon",
				"decimals": 11
			},
			"0xcc8160e5752eb608df5db4a276e16bec0a1a066a": {
				"ticker": "HOST",
				"address": "0xCc8160e5752EB608df5dB4A276E16beC0a1A066A",
				"name": "HOST",
				"decimals": 18
			},
			"0xabd54ae2ace5ff99a8b97119ae405f0859953a63": {
				"ticker": "HGOLDr1",
				"address": "0xaBD54ae2ace5Ff99a8b97119Ae405F0859953A63",
				"name": "HashGold(r1)",
				"decimals": 18
			},
			"0x6cdfc38b35eb8bfb0b147e8fa362ca2d590332d4": {
				"ticker": "ANGS",
				"address": "0x6cDfC38b35Eb8bFB0B147e8Fa362Ca2d590332D4",
				"name": "Angel SV",
				"decimals": 11
			},
			"0x12c48230729b4c6d2c34e6c8264ef8e484435b74": {
				"ticker": "ZSHARE",
				"address": "0x12C48230729B4C6D2C34e6c8264Ef8e484435b74",
				"name": "Zilla Shares",
				"decimals": 18
			},
			"0xf230d4cf12eec20112035494f14755a978e29afd": {
				"ticker": "MOON",
				"address": "0xf230D4Cf12Eec20112035494F14755a978e29AFD",
				"name": "Moon Markets",
				"decimals": 12
			},
			"0x164abec9013d3fdc934fba9bdb022b077d68163c": {
				"ticker": "YUMK",
				"address": "0x164aBeC9013D3FdC934FBa9BDb022B077d68163c",
				"name": "Cake King",
				"decimals": 11
			},
			"0x0d84e4a8d602b85d0ba8173b46acf538cde6fa0c": {
				"ticker": "QAPL",
				"address": "0x0d84E4a8D602B85D0BA8173b46ACF538cDe6Fa0c",
				"name": "Queen Pineapple",
				"decimals": 9
			},
			"0x215d37580e07fc38a07a658062b24a06bd900d23": {
				"ticker": "ELNQ",
				"address": "0x215d37580e07fC38a07a658062B24a06Bd900D23",
				"name": "Elon Queen",
				"decimals": 9
			},
			"0x8f4193f7121de7f7d8d970044cba14bbcbd84092": {
				"ticker": "FKI",
				"address": "0x8F4193f7121De7F7d8D970044CBA14bBcbD84092",
				"name": "Safe Floki",
				"decimals": 9
			},
			"0xe0ecffed8d5f2a099d3c132b1cb0bc3025082fa5": {
				"ticker": "FKI",
				"address": "0xE0eCfFed8d5f2A099d3c132B1cB0bc3025082FA5",
				"name": "Treasure Floki",
				"decimals": 11
			},
			"0x97a56c8a4d797544c187f6c9dd8dab0e2e4e1990": {
				"ticker": "CHIMP",
				"address": "0x97a56c8A4d797544c187F6C9dd8DAb0e2e4E1990",
				"name": "Project Chimp",
				"decimals": 9
			},
			"0x3c7a83a4bacd5721adbfce585cd82e337c5e63fb": {
				"ticker": "ANG",
				"address": "0x3c7A83A4bACd5721ADBFcE585Cd82E337C5e63Fb",
				"name": "Baby Angel",
				"decimals": 12
			},
			"0xa1675542dcdf10e89275f21f7c01651fd0b1e85b": {
				"ticker": "PRMD",
				"address": "0xa1675542dCDF10E89275F21f7C01651Fd0b1E85b",
				"name": "Pyramid",
				"decimals": 18
			},
			"0x884a7e5d8ced58de729bd7ddaff8da3a49dc5efa": {
				"ticker": "PPLE",
				"address": "0x884a7e5D8cEd58DE729Bd7dDaFF8da3A49dC5efA",
				"name": "Pineapple Token",
				"decimals": 10
			},
			"0x1ca02afb3e64362eddbcf7948225e70e4af82986": {
				"ticker": "CAKE",
				"address": "0x1Ca02afb3E64362EddBCF7948225e70E4AF82986",
				"name": "Baby Cake",
				"decimals": 12
			},
			"0xc957cf2b5c84d518baefc7b43f8b31729f33123b": {
				"ticker": "APE",
				"address": "0xC957cF2b5c84D518bAEFc7b43F8b31729f33123B",
				"name": "Ape Beast",
				"decimals": 9
			},
			"0xc27d543e68f65629def4516c89927af9c520ba3f": {
				"ticker": "PXT",
				"address": "0xc27D543e68F65629DEF4516C89927AF9C520ba3F",
				"name": "ProjectXNodes",
				"decimals": 9
			},
			"0xb279f8dd152b99ec1d84a489d32c35bc0c7f5674": {
				"ticker": "STEAK",
				"address": "0xb279f8DD152B99Ec1D84A489D32c35bC0C7F5674",
				"name": "STEAK",
				"decimals": 18
			},
			"0x5cee076e886124aecc884d65f5355fc2aeafc4d5": {
				"ticker": "EM",
				"address": "0x5cee076E886124AECc884d65f5355Fc2AeAFC4D5",
				"name": "EMERALD",
				"decimals": 4
			},
			"0x8986f6a37a81491afcccad6c1ff9686698ed28af": {
				"ticker": "PRMD",
				"address": "0x8986f6A37A81491afCCCAD6c1Ff9686698ED28Af",
				"name": "Pyramid",
				"decimals": 18
			},
			"0x6e61a39c5bde7d9c5b78bc680fca53092e822196": {
				"ticker": "SBA",
				"address": "0x6E61A39C5BDE7d9c5b78bc680fCa53092e822196",
				"name": "Shiba Network",
				"decimals": 9
			},
			"0x486eb78a6858072fbc7150f7e8adf088e2ea6090": {
				"ticker": "SFAN",
				"address": "0x486eB78A6858072FbC7150f7e8aDF088e2ea6090",
				"name": "Sushi Fantom",
				"decimals": 10
			},
			"0xb7ec13c6c59321d3f7059de9546020895a1073e8": {
				"ticker": "THN",
				"address": "0xb7Ec13C6C59321D3F7059DE9546020895a1073E8",
				"name": "Treasure Thunder",
				"decimals": 9
			},
			"0xd54e41a0be7ad7a59384d5972d47cb003134b380": {
				"ticker": "ENIAC",
				"address": "0xd54E41a0bE7ad7A59384d5972D47cb003134b380",
				"name": "Eniac Token",
				"decimals": 18
			},
			"0x346c6ff91197732ac631dfcf90c223bced6b489b": {
				"ticker": "FCRG",
				"address": "0x346C6fF91197732aC631DFCF90C223bCED6B489B",
				"name": "Fantasy Corgi",
				"decimals": 9
			},
			"0x45d1170294c123dca4334239f510be7f549c9aaa": {
				"ticker": "CGI",
				"address": "0x45D1170294C123dCA4334239F510bE7F549C9aaA",
				"name": "Corgi Infinity",
				"decimals": 12
			},
			"0xcdf152ee8a9116630ec543e9a837f166b6098477": {
				"ticker": "APEC",
				"address": "0xCdF152eE8a9116630Ec543e9a837f166B6098477",
				"name": "Ape Classic",
				"decimals": 9
			},
			"0x36c9af98c328e2af2e5ac64da9388e951b030954": {
				"ticker": "KAIRU",
				"address": "0x36C9Af98C328e2aF2e5aC64da9388e951b030954",
				"name": "Kairu Token",
				"decimals": 8
			},
			"0x0f45760e513da4a48b04df95254fda16709347d2": {
				"ticker": "WINGS",
				"address": "0x0F45760E513da4A48b04DF95254FDa16709347D2",
				"name": "Zen Angel",
				"decimals": 11
			},
			"0x11fe33f3b61fae98fcc255a69a390faf289a68ba": {
				"ticker": "MOON",
				"address": "0x11FE33F3B61FAe98fcc255A69a390faf289a68bA",
				"name": "Moon AI",
				"decimals": 11
			},
			"0xadb41aeff102f3d734f4da1a34140608877f838d": {
				"ticker": "CHIMP",
				"address": "0xaDb41aefF102f3D734f4DA1A34140608877F838D",
				"name": "Chimp INC",
				"decimals": 10
			},
			"0x8d7e2a88b9ad1cb0d2ae4dd75937fc10a27c148b": {
				"ticker": "BTH",
				"address": "0x8d7e2a88b9Ad1Cb0D2ae4dd75937fc10a27C148b",
				"name": "BitHotel",
				"decimals": 9
			},
			"0x9b828c36aa01a21366684dbeb848c8686e402ee6": {
				"ticker": "CGLD",
				"address": "0x9B828c36Aa01A21366684DbEb848C8686E402EE6",
				"name": "Crypto Gold",
				"decimals": 10
			},
			"0xbeb36481589a0acb1b8740024f754002dc744c4a": {
				"ticker": "APLC",
				"address": "0xBEb36481589A0acb1B8740024F754002Dc744C4a",
				"name": "Apple Chain",
				"decimals": 9
			},
			"0xc7306629002dd429361efcbcc31618bff42a23b6": {
				"ticker": "CHIMP",
				"address": "0xC7306629002Dd429361EFCbcc31618BFf42A23b6",
				"name": "Chimp Starter",
				"decimals": 12
			},
			"0x6ca558bd3eab53da1b25ab97916dd14bf6cfee4e": {
				"ticker": "pAVAX",
				"address": "0x6ca558bd3eaB53DA1B25aB97916dd14bf6CFEe4E",
				"name": "pAVAX",
				"decimals": 18
			},
			"0x2f787678e7200143a9324fa830b9f104ffd56dc4": {
				"ticker": "SHBR",
				"address": "0x2F787678e7200143a9324Fa830b9F104fFd56dc4",
				"name": "Shiba Rocket",
				"decimals": 11
			},
			"0x48fd132ef0e1d6cc6c32562f89e997ca4e8fe8a6": {
				"ticker": "DGE",
				"address": "0x48fD132ef0e1d6CC6c32562F89e997CA4E8Fe8a6",
				"name": "Project Doge",
				"decimals": 12
			},
			"0x838842f827e134fed888e58e4f24a28eef974062": {
				"ticker": "FSDOG",
				"address": "0x838842F827e134Fed888e58E4F24A28EEF974062",
				"name": "FUCKSDOG",
				"decimals": 18
			},
			"0x91f688cbf524245f06756b60b5209dd931da1bc1": {
				"ticker": "IDLE",
				"address": "0x91f688CBF524245f06756B60b5209DD931Da1BC1",
				"name": "Idle",
				"decimals": 9
			},
			"0xa3f01f136910c4330fa2f509b77c41159359a109": {
				"ticker": "AKITAking",
				"address": "0xA3F01f136910C4330fa2F509b77C41159359A109",
				"name": "AKITAking",
				"decimals": 9
			},
			"0x345e59f5c3fcc241fbbfff5fd65e9abc80e0e376": {
				"ticker": "CNR",
				"address": "0x345E59F5C3FCc241FbbFfF5Fd65e9abC80E0e376",
				"name": "CumNodeRocket",
				"decimals": 9
			},
			"0xc69eba65e87889f0805db717af06797055a0ba07": {
				"ticker": "ncash",
				"address": "0xc69Eba65e87889f0805dB717Af06797055A0BA07",
				"name": "NitroNetwork",
				"decimals": 18
			},
			"0x40c623f1602a3039b9e4d389f4363f366d79e1df": {
				"ticker": "MARS",
				"address": "0x40C623F1602A3039B9e4D389F4363F366D79e1Df",
				"name": "Mars Infinity",
				"decimals": 12
			},
			"0x9630732fe0511ce97c930a8f3d86aa2735075a8e": {
				"ticker": "MOONEY",
				"address": "0x9630732FE0511ce97c930a8f3d86aa2735075A8E",
				"name": "MOONEY",
				"decimals": 18
			},
			"0xa5e59761ebd4436fa4d20e1a27cba29fb2471fc6": {
				"ticker": "SHERPA",
				"address": "0xa5E59761eBD4436fa4d20E1A27cBa29FB2471Fc6",
				"name": "Sherpa",
				"decimals": 18
			},
			"0xfcea43e7a10a779e09f27814060de7155acdc4d1": {
				"ticker": "PXT",
				"address": "0xFcEa43E7a10a779e09F27814060DE7155aCdC4d1",
				"name": "ProjectX v2",
				"decimals": 1
			},
			"0xd0300d1acf3f2920d5669d0bdf8fd12105621555": {
				"ticker": "OTO",
				"address": "0xD0300d1acF3f2920d5669D0BdF8fD12105621555",
				"name": "OTO Protocol",
				"decimals": 18
			},
			"0xbccd0a73d94147044dbbe5c9c3096a29f358faef": {
				"ticker": "SPE",
				"address": "0xBccd0a73d94147044DbBE5C9c3096a29F358faEf",
				"name": "SPE",
				"decimals": 18
			},
			"0xa83858f58680772063a81eee9f69d2b27350f6ca": {
				"ticker": "WHLM",
				"address": "0xa83858f58680772063A81eeE9F69D2B27350F6ca",
				"name": "Whale Markets",
				"decimals": 12
			},
			"0xb15d0c534b8a4638db3e33b6f9beaae504d114d0": {
				"ticker": "PPLE",
				"address": "0xB15d0C534B8A4638dB3E33b6F9beAaE504d114d0",
				"name": "Pineapple DAO",
				"decimals": 9
			},
			"0x12ec7f7a345e3b3b20ce9e2936ab5efd5b63fc02": {
				"ticker": "GRAPE",
				"address": "0x12eC7F7A345e3b3B20ce9e2936AB5eFd5B63fC02",
				"name": "GRAPE",
				"decimals": 18
			},
			"0x4970588a70209d18c0e07da2f3e97a27fb6765ec": {
				"ticker": "MOON",
				"address": "0x4970588A70209D18C0E07Da2f3e97a27FB6765eC",
				"name": "Baby Moon",
				"decimals": 9
			},
			"0x0356ed6caec19d3f353821480ca44bf7db6d713d": {
				"ticker": "ANYP",
				"address": "0x0356ED6caec19D3F353821480CA44Bf7dB6D713d",
				"name": "AnyPrinter V2",
				"decimals": 9
			},
			"0x2ed2f87e2de90d18246a041b3108438754f0b352": {
				"ticker": "PAN",
				"address": "0x2ed2F87E2dE90d18246a041B3108438754f0B352",
				"name": "Crypto Panda",
				"decimals": 9
			},
			"0x8d5342062d784d4cc1ffbdf56ab82015e590c5f7": {
				"ticker": "WHL",
				"address": "0x8d5342062d784D4cc1fFbdF56AB82015E590c5F7",
				"name": "Queen Whale",
				"decimals": 10
			},
			"0x8c27bd9ced392ade48b6d126454341c29519ac5f": {
				"ticker": "VTK",
				"address": "0x8C27Bd9ceD392ADe48B6d126454341c29519ac5F",
				"name": "VanillaToken",
				"decimals": 18
			},
			"0x2d87845b299cf619ecb122d06b7c90ea7ebbc218": {
				"ticker": "EFTM",
				"address": "0x2d87845B299cF619ecb122D06b7c90eA7eBbC218",
				"name": "Exo Fantom",
				"decimals": 11
			},
			"0x9de6d0fa563b4669b585b38c9eba0ef62da4b8f1": {
				"ticker": "HAAX",
				"address": "0x9De6d0Fa563b4669B585b38c9ebA0EF62dA4B8F1",
				"name": "Happy Avax",
				"decimals": 18
			},
			"0x1d81ee9d696e180b421859154b122920972797ca": {
				"ticker": "APLR",
				"address": "0x1d81ee9d696e180B421859154b122920972797ca",
				"name": "Apple Robot",
				"decimals": 9
			},
			"0x73f3f7200872458474a8c457d7c0adf8e96b5f10": {
				"ticker": "SHIB",
				"address": "0x73F3f7200872458474A8c457d7C0Adf8e96B5f10",
				"name": "Shiba Swap",
				"decimals": 11
			},
			"0x237917e8a998b37759c8ee2faa529d60c66c2927": {
				"ticker": "sifu",
				"address": "0x237917E8a998b37759c8EE2fAa529D60c66c2927",
				"name": "Sifu",
				"decimals": 18
			},
			"0x6c88f1e8fb4c1f073ac960d9234adc3bbfe07e5e": {
				"ticker": "APAY",
				"address": "0x6C88F1e8Fb4C1F073ac960D9234aDC3BbFE07E5E",
				"name": "Avax Payments",
				"decimals": 9
			},
			"0x351116480614d80fc00ada7cdbc40e72e6636cdf": {
				"ticker": "WHL",
				"address": "0x351116480614d80fc00ADA7cdbC40E72E6636cDf",
				"name": "Treasure Whale",
				"decimals": 9
			},
			"0xf2f13f0b7008ab2fa4a2418f4ccc3684e49d20eb": {
				"ticker": "WMATIC",
				"address": "0xf2f13f0B7008ab2FA4A2418F4ccC3684E49D20Eb",
				"name": "Wrapped Matic (Wormhole)",
				"decimals": 18
			},
			"0x846d50248baf8b7ceaa9d9b53bfd12d7d7fbb25a": {
				"ticker": "VSO",
				"address": "0x846D50248BAf8b7ceAA9d9B53BFd12d7D7FBB25a",
				"name": "VersoToken",
				"decimals": 18
			},
			"0x06dfb6ae537f5d53efb3788a8c28bbe249fbe1cc": {
				"ticker": "ChikuInu",
				"address": "0x06DfB6ae537F5D53efb3788a8c28Bbe249fBe1CC",
				"name": "ChikuInu",
				"decimals": 18
			},
			"0x31b36f1feab96a86adf2ea4c4310a38ce6314cb9": {
				"ticker": "MRS",
				"address": "0x31b36f1FeAb96a86ADf2Ea4c4310A38ce6314Cb9",
				"name": "Fantasy Mars",
				"decimals": 9
			},
			"0xea6c4e7ef5258d0f52024af7f9bd53be8a1dab39": {
				"ticker": "PIZZA",
				"address": "0xea6c4e7Ef5258d0F52024af7f9bd53be8A1daB39",
				"name": "PIZZA GAME",
				"decimals": 9
			},
			"0x995e066ad9b66498d8f20968568135181c30442f": {
				"ticker": "THN",
				"address": "0x995e066aD9B66498D8f20968568135181c30442f",
				"name": "Fantasy Thunder",
				"decimals": 12
			},
			"0x4edc54f6269c7f669d5b1ede19665fc36909b77b": {
				"ticker": "GLD",
				"address": "0x4EDC54f6269c7F669d5b1eDE19665Fc36909B77b",
				"name": "Gold Project",
				"decimals": 9
			},
			"0x2f5e0396847180de6e53632f703d1c69957859b7": {
				"ticker": "WINGS",
				"address": "0x2F5e0396847180dE6E53632f703D1c69957859b7",
				"name": "Angel Monster",
				"decimals": 10
			},
			"0x2d9124836b8ba9b56cae6480a409f39805b24766": {
				"ticker": "AKITA",
				"address": "0x2d9124836B8bA9b56caE6480a409F39805B24766",
				"name": "Akita AVAX",
				"decimals": 12
			},
			"0xd13ec98738cc75693ba98f6d211f8995e5c43030": {
				"ticker": "MILK",
				"address": "0xd13Ec98738cc75693Ba98f6D211f8995e5C43030",
				"name": "Dairy Money",
				"decimals": 18
			},
			"0x3ea586975a802d9f9d803c59c7872f182be42281": {
				"ticker": "FLOKI",
				"address": "0x3ea586975a802D9F9D803C59C7872F182be42281",
				"name": "Sushi Floki",
				"decimals": 12
			},
			"0xcb4d96a80831552f2024a2d59c7a95eeac4325b6": {
				"ticker": "ELN",
				"address": "0xCb4d96A80831552f2024A2d59C7a95EeaC4325B6",
				"name": "Project Elon",
				"decimals": 11
			},
			"0xbfc154dfe7069c9a160940ea23aba448197c3031": {
				"ticker": "ZLN",
				"address": "0xBfC154Dfe7069c9A160940eA23aba448197C3031",
				"name": "Zalinium",
				"decimals": 18
			},
			"0x909b4faee7846482cc193e8a74a529ff7b83798d": {
				"ticker": "PND",
				"address": "0x909b4faEE7846482cc193e8a74a529fF7b83798D",
				"name": "Crypto Panda",
				"decimals": 10
			},
			"0xa48507f280c1ebd613af653ec442adadbd66261f": {
				"ticker": "AKT",
				"address": "0xA48507F280c1EBd613af653EC442adAdbd66261F",
				"name": "Exo Akita",
				"decimals": 12
			},
			"0xd21cdca47fa45a0a51eec030e27af390ab3aa489": {
				"ticker": "MEAD",
				"address": "0xD21CdCA47Fa45A0A51eec030E27AF390ab3aa489",
				"name": "Mead",
				"decimals": 18
			},
			"0xf3155aaa8e4109575b66370500c8e3d54349f37b": {
				"ticker": "ZSHARE",
				"address": "0xf3155aaA8e4109575b66370500c8e3d54349F37B",
				"name": "zShare",
				"decimals": 18
			},
			"0x5cc5f64fdc4161e64bfc8e944a6453b684ea7ce9": {
				"ticker": "MOON",
				"address": "0x5cc5f64fDc4161e64Bfc8e944A6453b684EA7CE9",
				"name": "Moon AVAX",
				"decimals": 12
			},
			"0x9a9770529b96dd044ab08106dc9ddb877e55d65e": {
				"ticker": "MOON",
				"address": "0x9a9770529b96DD044ab08106dC9dDb877E55D65e",
				"name": "Moon SV",
				"decimals": 12
			},
			"0xe173b05e3f008ea4aea64b7592425217bf5d258a": {
				"ticker": "SLOT",
				"address": "0xE173B05e3f008ea4aEA64B7592425217bf5D258A",
				"name": "Snowtomb Lot",
				"decimals": 1
			},
			"0x9ad5817b49b5b223008ea4553ac8ec960b50f486": {
				"ticker": "MOON",
				"address": "0x9aD5817b49b5b223008Ea4553ac8ec960B50f486",
				"name": "Moon Ledger",
				"decimals": 12
			},
			"0xd4fd8233f8009d77532ea209f8d7f303fe2f3304": {
				"ticker": "HCH",
				"address": "0xD4fd8233F8009D77532EA209f8D7F303fe2f3304",
				"name": "OMG Hachi",
				"decimals": 10
			},
			"0x8a3e7a557a70d1fec4db61547ee217dcfb41c8bc": {
				"ticker": "AVAL",
				"address": "0x8A3e7a557a70D1FeC4Db61547eE217dcFB41C8bC",
				"name": "Avalend Finance",
				"decimals": 18
			},
			"0xd036414fa2bcbb802691491e323bff1348c5f4ba": {
				"ticker": "MU",
				"address": "0xD036414fa2BCBb802691491E323BFf1348C5F4Ba",
				"name": "Mu Coin",
				"decimals": 18
			},
			"0x6619ee604439a57cd3997a8a4042851783871db4": {
				"ticker": "MILF",
				"address": "0x6619ee604439A57CD3997A8A4042851783871db4",
				"name": "Metaverse Investment Legacy Fund",
				"decimals": 9
			},
			"0x63de64b8b56b64d08891d9b0dd599022739e0103": {
				"ticker": "FKI",
				"address": "0x63De64B8b56b64D08891D9B0dD599022739E0103",
				"name": "Floki Chain",
				"decimals": 9
			},
			"0xc46e6220de8c7c98eb7a0c6548126d2772a6a5ff": {
				"ticker": "SBA",
				"address": "0xC46E6220dE8c7C98Eb7A0C6548126D2772A6A5ff",
				"name": "Sushi Shiba",
				"decimals": 12
			},
			"0x049b4a500696b6e1a37b9b018f2b4b218d56be22": {
				"ticker": "WHALE",
				"address": "0x049b4a500696B6E1a37B9b018F2B4B218d56BE22",
				"name": "Whale Swap",
				"decimals": 11
			},
			"0xc695474cec43f0da8ae9e042c13901d94bd5114e": {
				"ticker": "LFKI",
				"address": "0xc695474cec43f0DA8ae9e042c13901d94bd5114e",
				"name": "Little Floki",
				"decimals": 11
			},
			"0x49ae74396c644a9cda3c45ed1d6bbba90fcc157b": {
				"ticker": "ANGF",
				"address": "0x49aE74396C644A9CdA3C45ED1d6Bbba90fCc157B",
				"name": "Angel Farm",
				"decimals": 9
			},
			"0xb9da28c3a12a0aaf6591fcb15e8a026c38fce7a8": {
				"ticker": "Flok",
				"address": "0xb9DA28C3a12A0aaF6591FcB15e8A026C38FCe7a8",
				"name": "Floki Santa",
				"decimals": 18
			},
			"0xf0a71e8e0ab533c9d5c1f1d4d16f9f7dccf277fe": {
				"ticker": "PDAC",
				"address": "0xF0A71E8e0ab533C9D5C1f1d4d16F9f7dccF277FE",
				"name": "Panda Classic",
				"decimals": 9
			},
			"0x49a282effaca994f883ffcd3a8e6b15335e3a6d1": {
				"ticker": "GHSS",
				"address": "0x49A282EFFACa994F883FfCD3A8e6B15335e3a6d1",
				"name": "Ghost Starter",
				"decimals": 9
			},
			"0xd0755413bfe2e08db6be72761cdd56d77d4b60f1": {
				"ticker": "NORTH",
				"address": "0xD0755413bfE2e08dB6bE72761cdD56d77d4B60f1",
				"name": "North Token",
				"decimals": 18
			},
			"0x4156f18bf7c1ef04248632c66aa119de152d8f2e": {
				"ticker": "ZEUS",
				"address": "0x4156F18bF7C1ef04248632C66Aa119De152D8f2E",
				"name": "Zeus Node Finance",
				"decimals": 18
			},
			"0x0f577433bf59560ef2a79c124e9ff99fca258948": {
				"ticker": "MONEY",
				"address": "0x0f577433Bf59560Ef2a79c124E9Ff99fCa258948",
				"name": "Moremoney USD",
				"decimals": 18
			},
			"0xd8273b6f64788e5a6279df3345f5551e4c47b891": {
				"ticker": "NOVAVAX",
				"address": "0xd8273B6F64788E5A6279df3345f5551e4c47B891",
				"name": "Novavax",
				"decimals": 18
			},
			"0xd01ec8f031edd517889d0673a864654f56ee0077": {
				"ticker": "GRAVESHARE",
				"address": "0xd01ec8F031EDd517889d0673A864654F56eE0077",
				"name": "GRAVESHARE",
				"decimals": 18
			},
			"0x1b4e273d7ca6749872797b3670e6343b4e6ab071": {
				"ticker": "AKITA",
				"address": "0x1B4E273D7CA6749872797b3670E6343b4E6Ab071",
				"name": "Meta Akita",
				"decimals": 12
			},
			"0xb3de0757e3984a9774c53098f4ad8a4ced6d0ff6": {
				"ticker": "DOGE",
				"address": "0xB3De0757e3984A9774c53098f4ad8A4Ced6d0FF6",
				"name": "Zen Doge",
				"decimals": 11
			},
			"0x9e037de681cafa6e661e6108ed9c2bd1aa567ecd": {
				"ticker": "WALBT",
				"address": "0x9E037dE681CaFA6E661e6108eD9c2bd1AA567Ecd",
				"name": "Wrapped AllianceBlock Token",
				"decimals": 18
			},
			"0x535b6ec409c0d080b139ffbf683d3e5a79bb3a0f": {
				"ticker": "ELON",
				"address": "0x535b6Ec409C0D080b139fFBf683D3e5A79BB3A0f",
				"name": "Elon Beast",
				"decimals": 11
			},
			"0xdeaddab88b16d11ab36fdb444cd32d75cbfc3aac": {
				"ticker": "CORGI",
				"address": "0xdeADdaB88B16D11ab36FDb444cd32D75CBfc3AAc",
				"name": "Crypto Corgi",
				"decimals": 10
			},
			"0x6e146de6d5472287ae29906a0a44e95b8ee39b5e": {
				"ticker": "OGLT",
				"address": "0x6E146De6D5472287Ae29906a0a44E95b8Ee39b5E",
				"name": "OMG Glitch",
				"decimals": 11
			},
			"0xc9e8c581d67fe5cc6217b2fa1b2d4d9cecb60fb2": {
				"ticker": "SGEM",
				"address": "0xc9e8c581d67Fe5cc6217B2Fa1B2d4D9Cecb60fB2",
				"name": "SGEM",
				"decimals": 18
			},
			"0xfd0c58f03c83d6960bb9dbfd45076d78df2f095d": {
				"ticker": "ASND",
				"address": "0xFd0c58F03c83d6960BB9dbFd45076d78Df2F095D",
				"name": "Ascend",
				"decimals": 18
			},
			"0xfc5cdfdb17613d9cb097474a4fb0d8d319f979aa": {
				"ticker": "FANA",
				"address": "0xFC5cdfDb17613d9cB097474A4fB0d8d319f979aA",
				"name": " Project X Financial ",
				"decimals": 9
			},
			"0x6f5ad4b6d90598463c3208cbfa0c713e6f65cfec": {
				"ticker": "INFI",
				"address": "0x6f5aD4B6d90598463C3208cbFA0C713e6F65cfec",
				"name": "Fantom AI",
				"decimals": 18
			},
			"0x8b6d6461547eca870af5ea748f29e7bb0f9d305a": {
				"ticker": "ELON",
				"address": "0x8b6D6461547eCa870AF5eA748f29e7bb0F9D305A",
				"name": "InfiniteNodes",
				"decimals": 9
			},
			"0x01ea6706712a32eb520a051dce3fa98acc321c8c": {
				"ticker": "BMRS",
				"address": "0x01eA6706712a32EB520a051dcE3fa98aCC321c8C",
				"name": "Elon Project",
				"decimals": 9
			},
			"0x96fe76638cdb14596ceae97ad9e6811197314194": {
				"ticker": "GRAY",
				"address": "0x96FE76638cdB14596CeaE97ad9E6811197314194",
				"name": "Baby Mars",
				"decimals": 18
			},
			"0xfda866cfece71f4c17b4a5e5f9a00ac08c72eadc": {
				"ticker": "PERA",
				"address": "0xfDA866CFEcE71F4C17b4a5e5f9A00ac08C72Eadc",
				"name": "Gray",
				"decimals": 18
			},
			"0x13a5a6569c5329f11679d75b1f7017c301ff015f": {
				"ticker": "CRGB",
				"address": "0x13a5a6569c5329f11679d75B1F7017c301ff015F",
				"name": "Picaso DAO",
				"decimals": 12
			},
			"0xb1466d4cf0dcfc0bcddcf3500f473cdacb88b56d": {
				"ticker": "WET",
				"address": "0xB1466d4cf0DCfC0bCdDcf3500F473cdACb88b56D",
				"name": "Corgi Beast",
				"decimals": 18
			},
			"0x7c5dadaf63113cc40454119d0e5400630b25dd97": {
				"ticker": "GLTI",
				"address": "0x7C5dADaF63113cC40454119d0e5400630b25dd97",
				"name": "Weble Ecosystem Token",
				"decimals": 11
			},
			"0x70ca047970d10e9edc60151e9a8131fb5a51bf50": {
				"ticker": "OSHARE",
				"address": "0x70cA047970d10E9EDc60151E9a8131fb5A51Bf50",
				"name": "Glitch Infinity",
				"decimals": 18
			},
			"0x99b4590f686578bdea779292e814a0b1b79424d9": {
				"ticker": "OSP",
				"address": "0x99b4590F686578BdEa779292e814a0B1b79424d9",
				"name": "Olympic Shares",
				"decimals": 9
			},
			"0x81069d237bdbb092af1256b53a379c69adb4c17a": {
				"ticker": "PLAYMATES",
				"address": "0x81069d237BdbB092Af1256b53a379c69ADb4c17a",
				"name": "OpenSea Protocol",
				"decimals": 18
			},
			"0xd2068151eb8e1337984386c1c00a135f2e6be2f6": {
				"ticker": "MOON",
				"address": "0xd2068151Eb8E1337984386C1C00A135F2e6bE2f6",
				"name": "Redlight Node District",
				"decimals": 12
			},
			"0xe7daf29889ff9323f69314431c94b10b79f765b0": {
				"ticker": "ELON",
				"address": "0xE7daF29889FF9323F69314431c94b10B79f765B0",
				"name": "Moon SV",
				"decimals": 12
			},
			"0x4bfc90322dd638f81f034517359bd447f8e0235a": {
				"ticker": "NEWO",
				"address": "0x4Bfc90322dD638F81F034517359BD447f8E0235a",
				"name": "Sushi Elon",
				"decimals": 18
			},
			"0x32958a28d1641af1d4bdd790550fc0d8049dbd3a": {
				"ticker": "DAMO",
				"address": "0x32958A28D1641AF1d4bdd790550fC0D8049dbd3A",
				"name": "New Order",
				"decimals": 18
			},
			"0xbdf4f4d40844fb12f5af0b323435caf4f10d5191": {
				"ticker": "LGD",
				"address": "0xbdf4f4d40844fB12f5aF0b323435cAF4F10d5191",
				"name": "HAI DAMO",
				"decimals": 18
			},
			"0x495c164d7b33a1fccd4e943a5eed1240f9623931": {
				"ticker": "MOON",
				"address": "0x495C164d7B33a1FCCD4E943a5Eed1240f9623931",
				"name": "LegendsDao",
				"decimals": 11
			},
			"0xb311dcba56cd48f2c228973f47d58c3048eebf8c": {
				"ticker": "APE",
				"address": "0xB311dCba56CD48F2c228973F47D58c3048eeBf8C",
				"name": "Queen Moon",
				"decimals": 9
			},
			"0x8533e9e5a34c99b28d10be7c7d727ef0ce8bdf49": {
				"ticker": "ELNF",
				"address": "0x8533E9E5A34c99B28d10be7C7D727Ef0CE8bdf49",
				"name": "K2 Protocol",
				"decimals": 9
			},
			"0x18683099a95177d14739155736d47e8c9d5558e2": {
				"ticker": "CMP",
				"address": "0x18683099a95177d14739155736d47e8C9D5558e2",
				"name": "Elon Fund",
				"decimals": 11
			},
			"0x50d1b694744ada9797febd3c19c080bcddc3d233": {
				"ticker": "PXT",
				"address": "0x50d1B694744ADa9797FEbd3C19C080BCddc3D233",
				"name": "MasterMiM",
				"decimals": 9
			},
			"0x8d9e05731fcab15ae8934db8087a47612bbbd9c3": {
				"ticker": "KITSU",
				"address": "0x8d9E05731fcAB15aE8934dB8087a47612bBbd9C3",
				"name": "Kitsune Inu",
				"decimals": 18
			},
			"0x613fad8aae153cca215f3f455d505120e476c060": {
				"ticker": "CAKE",
				"address": "0x613fad8aAE153cca215f3F455d505120e476c060",
				"name": "Little Cake",
				"decimals": 9
			},
			"0x8522110b891f8cce57d5dc3e1ba784c646bf4b85": {
				"ticker": "MOON",
				"address": "0x8522110B891F8cce57d5dc3E1ba784c646BF4B85",
				"name": "Moon Robot",
				"decimals": 11
			},
			"0x0d208af7f8d236047f160dc683613a4941b30aec": {
				"ticker": "PNPL",
				"address": "0x0D208af7F8D236047F160DC683613A4941b30aEc",
				"name": "Wrapped Pineapple",
				"decimals": 11
			},
			"0x1e07f347585c1fe2ae003a0bbe2ca14e48725b54": {
				"ticker": "PNPL",
				"address": "0x1E07f347585C1FE2Ae003a0bBE2cA14e48725b54",
				"name": "Meta Pineapple",
				"decimals": 12
			},
			"0x100d8e04784b81f0f9f8076f90d79ad8872a40c1": {
				"ticker": "CHI",
				"address": "0x100D8E04784b81f0F9F8076f90d79AD8872A40C1",
				"name": "Chimp Farm",
				"decimals": 12
			},
			"0x5fbeae40dcf302368d2a89f5edf88eff4886ddd5": {
				"ticker": "STAR",
				"address": "0x5FBEAE40DcF302368d2A89F5eDF88eff4886DdD5",
				"name": "Safe Star",
				"decimals": 11
			},
			"0x875c28a5490922912342d63407bb73af5a8df6e0": {
				"ticker": "RKT",
				"address": "0x875C28A5490922912342D63407bB73aF5A8DF6e0",
				"name": "Rocket Finance",
				"decimals": 9
			},
			"0xb08b65f44ee8060122612a0a901887328c3d8b25": {
				"ticker": "SHIBN",
				"address": "0xb08b65f44EE8060122612A0a901887328c3d8b25",
				"name": "ShibaNaut",
				"decimals": 9
			},
			"0x5a52b45553c72f99edd272ac9d221d8e3dc7c245": {
				"ticker": "FLOKI",
				"address": "0x5a52B45553c72f99Edd272ac9D221d8E3Dc7c245",
				"name": "Mini Floki",
				"decimals": 11
			},
			"0xacb594f507d50fa22c1dbe95f1751524e00b5761": {
				"ticker": "SCHI",
				"address": "0xacb594f507D50FA22C1DBe95F1751524e00b5761",
				"name": "Super Hachi",
				"decimals": 11
			},
			"0x4a46c761081b5a3ba9ddb5a4fba4f5d00cba75d0": {
				"ticker": "WIREX",
				"address": "0x4A46C761081b5A3BA9DDB5a4fBa4f5D00cBa75D0",
				"name": "WireX B",
				"decimals": 9
			},
			"0xc8474521b598cafa9480629c2cbcf7af0e027fbc": {
				"ticker": "EWHL",
				"address": "0xC8474521B598caFA9480629C2cbCf7Af0E027FBC",
				"name": "Exo Whale",
				"decimals": 12
			},
			"0x23e3b6aa5b0e3d8c3232a6cde3e117cf7166fa09": {
				"ticker": "SANTA",
				"address": "0x23e3b6aa5b0e3D8c3232A6cdE3e117Cf7166fa09",
				"name": "Santa Joe",
				"decimals": 9
			},
			"0xe16253892f126d068e711c2fdde6db56969dbcf6": {
				"ticker": "Splash",
				"address": "0xE16253892F126D068E711C2fdde6DB56969dBCf6",
				"name": "Splash Token",
				"decimals": 18
			},
			"0xf10287a312f3d07e5ef774bd415de342c56b6654": {
				"ticker": "WGOL",
				"address": "0xF10287a312F3d07E5Ef774bD415DE342C56b6654",
				"name": "Wrapped Gold",
				"decimals": 9
			},
			"0xdd33ef5704ada19d254cb636b5a0a1bad3936050": {
				"ticker": "MMAC",
				"address": "0xdD33ef5704ADA19d254cb636B5A0A1bAd3936050",
				"name": "MMAC",
				"decimals": 9
			},
			"0x4dde89859dcdf2c9c73e1363fef9a66474d4351f": {
				"ticker": "Yuzu",
				"address": "0x4DDe89859dcdF2c9C73e1363fef9a66474D4351f",
				"name": "Yuzu Financial",
				"decimals": 18
			},
			"0xe1d70994be12b73e76889412b284a8f19b0de56d": {
				"ticker": "EEUR",
				"address": "0xE1d70994Be12b73E76889412b284A8F19b0DE56d",
				"name": "e-Money EUR stablecoin",
				"decimals": 6
			},
			"0xcb1c85ba30d61725b500762a90e5d01603dda38c": {
				"ticker": "FKI",
				"address": "0xCB1c85ba30d61725B500762A90E5D01603dda38C",
				"name": "Exo Floki",
				"decimals": 10
			},
			"0xb5f13f32fbda67ea40a616095a31c5cb5cfd5f18": {
				"ticker": "GOL",
				"address": "0xb5F13F32FBDA67EA40a616095A31c5cB5cFd5f18",
				"name": "Sushi Gold",
				"decimals": 9
			},
			"0xe30aa9ab659b5e50e0954cb5005820d06f20cb29": {
				"ticker": "ANTG",
				"address": "0xE30Aa9Ab659b5e50E0954Cb5005820d06f20CB29",
				"name": "ANTG",
				"decimals": 18
			},
			"0xaf8d981b35af4080f2d6b7233d3b27e457854e5f": {
				"ticker": "PXT",
				"address": "0xAF8D981B35aF4080F2d6B7233d3B27E457854e5f",
				"name": "ProjectXNodes",
				"decimals": 9
			},
			"0xd83d1959173287cf5f932da8c5ab9b3822ef6793": {
				"ticker": "BETS",
				"address": "0xd83d1959173287cF5F932DA8C5ab9B3822Ef6793",
				"name": "Casino & Sports Book",
				"decimals": 9
			},
			"0x9b350cbd3133c2b3202a0a89ef1245612cb5142f": {
				"ticker": "CPTINU",
				"address": "0x9B350Cbd3133c2B3202A0a89Ef1245612cb5142F",
				"name": "CAPTAIN INU",
				"decimals": 9
			},
			"0xce8b208554450a0fb7bb1e57d9f318ea6ffe22a9": {
				"ticker": "FTM",
				"address": "0xCE8b208554450A0fb7BB1E57d9F318Ea6ffE22a9",
				"name": "Fantom Swap",
				"decimals": 9
			},
			"0x92bc73afed857e200bcaafbfd0683d6a3cbb36d1": {
				"ticker": "PP",
				"address": "0x92bC73afED857e200BcAaFbFd0683d6a3CbB36D1",
				"name": "Platypus Token",
				"decimals": 18
			},
			"0x3030c32a11d53e9bdb31ed55e572a52bdab65972": {
				"ticker": "SHBS",
				"address": "0x3030c32A11D53e9bdb31ed55e572a52BDaB65972",
				"name": "Shiba Swap",
				"decimals": 10
			},
			"0xd35ad78fbd44fe580646582a2845f5b033493490": {
				"ticker": "DIFF",
				"address": "0xD35aD78fBD44Fe580646582A2845F5B033493490",
				"name": "Diffusion",
				"decimals": 18
			},
			"0x320a0e5b1681f31410e19cb6d58fe65165e5c664": {
				"ticker": "PXT",
				"address": "0x320A0E5B1681F31410e19cB6d58Fe65165E5c664",
				"name": "ProjectXNodes",
				"decimals": 18
			},
			"0x34ae9b5dc496497c87277168a3b5a2d07a05c133": {
				"ticker": "FTM",
				"address": "0x34aE9B5dC496497C87277168a3b5A2D07A05c133",
				"name": "Crypto Fantom",
				"decimals": 11
			},
			"0x28a413c2c379d91800b2a28b1ff5d10882df1798": {
				"ticker": "PXT",
				"address": "0x28A413c2c379d91800B2A28B1FF5d10882Df1798",
				"name": "ProjectXNodes",
				"decimals": 18
			},
			"0x4a087f6f213af1bb8c945a1ed764957790e8d211": {
				"ticker": "ETHS",
				"address": "0x4A087F6F213AF1BB8c945A1ED764957790e8D211",
				"name": "EtherStones",
				"decimals": 18
			},
			"0xdb905f2ef8c8695b19db6da944e88275296b9f6b": {
				"ticker": "PAN",
				"address": "0xDb905f2ef8C8695b19db6DA944E88275296b9F6b",
				"name": "Safe Panda",
				"decimals": 10
			},
			"0x9236d1191a3c60d5600a9f942c16f045bff84488": {
				"ticker": "DOGE",
				"address": "0x9236d1191a3c60D5600a9F942c16f045bFF84488",
				"name": "Doge Token",
				"decimals": 11
			},
			"0xd2ad73ce020911a4c04c284bfd2d451b4a777bdb": {
				"ticker": "0xB",
				"address": "0xD2ad73Ce020911A4C04c284bfd2d451b4A777BDB",
				"name": "0xBlock",
				"decimals": 18
			},
			"0xfcffdaf2c10f81d91ff9b8ac0639981417c75fd6": {
				"ticker": "MOON",
				"address": "0xfcFFDAf2C10f81d91FF9b8aC0639981417C75fd6",
				"name": "Moon Block",
				"decimals": 9
			},
			"0xab592d197acc575d16c3346f4eb70c703f308d1e": {
				"ticker": "FEED",
				"address": "0xab592d197ACc575D16C3346f4EB70C703F308D1E",
				"name": "chikn feed",
				"decimals": 18
			},
			"0xfc6da929c031162841370af240dec19099861d3b": {
				"ticker": "DOMI",
				"address": "0xFc6Da929c031162841370af240dEc19099861d3B",
				"name": "Domi",
				"decimals": 18
			},
			"0x9185da6187dfd7d591c55c28e2881f4e214235d3": {
				"ticker": "SYUM",
				"address": "0x9185dA6187DFD7D591C55c28E2881f4e214235d3",
				"name": "Super Cake",
				"decimals": 11
			},
			"0xb5fa74903538da6f29095d7c9eb06840e130be77": {
				"ticker": "FUT",
				"address": "0xb5Fa74903538Da6F29095D7C9Eb06840e130BE77",
				"name": "Future Nodes",
				"decimals": 9
			},
			"0xfed3d9ea35711a251688a4672e45b9fd8ef5d9a7": {
				"ticker": "AC",
				"address": "0xfED3D9eA35711a251688A4672E45B9FD8Ef5d9a7",
				"name": "AvaClifford Inu",
				"decimals": 18
			},
			"0xdfdbd03c1cb9b17c0710039b2f9dda9cce6d7a95": {
				"ticker": "ZYUM",
				"address": "0xDFDbd03C1Cb9b17c0710039B2f9DDa9cce6D7a95",
				"name": "Zen Cake",
				"decimals": 11
			},
			"0xefc3115c8218d9886b3def6a3d8fa1986c9d431d": {
				"ticker": "WANA",
				"address": "0xefc3115C8218d9886B3DeF6a3D8fa1986C9D431d",
				"name": "Wana",
				"decimals": 9
			},
			"0x109711c76e11a953ebae7301ecfa9cf737fec8b1": {
				"ticker": "MSTIM",
				"address": "0x109711c76e11a953EbAe7301ecFA9CF737fec8b1",
				"name": "MasterMiM",
				"decimals": 18
			},
			"0x98465d106553bcb33215f7de7c9e467f1669a6cb": {
				"ticker": "GLD",
				"address": "0x98465d106553BCb33215F7de7C9e467f1669A6cb",
				"name": "Gold Robot",
				"decimals": 12
			},
			"0x5870a34ee9abf322fb209eb0856773caedb2c8e3": {
				"ticker": "CAKE",
				"address": "0x5870a34ee9AbF322FB209eB0856773CAeDb2C8E3",
				"name": "Cake Farm",
				"decimals": 11
			},
			"0xfb602b66e3a6fb1de008fbf7117938f222dca87e": {
				"ticker": "Op3n",
				"address": "0xFB602b66E3A6fB1dE008Fbf7117938F222DCa87E",
				"name": "Op3n",
				"decimals": 18
			},
			"0xed54483af4d8e059a41476fee25c3afb487b32aa": {
				"ticker": "SHBG",
				"address": "0xED54483af4d8e059a41476fEE25c3afB487B32aA",
				"name": "Shiba Gaming",
				"decimals": 9
			},
			"0x49aeacf6c2d2c10432a3261df8b68c46b4145ae6": {
				"ticker": "LOLIQ",
				"address": "0x49aEacF6C2d2C10432A3261dF8B68C46b4145aE6",
				"name": "Low Liq",
				"decimals": 18
			},
			"0x4e3642603a75528489c2d94f86e9507260d3c5a1": {
				"ticker": "JGN",
				"address": "0x4e3642603a75528489C2D94f86e9507260d3c5a1",
				"name": "Juggernaut DeFi",
				"decimals": 18
			},
			"0xa3b90e378ebd25bf4ad78cb61aea0ccd7bc4a683": {
				"ticker": "APE",
				"address": "0xa3B90e378eBD25BF4AD78Cb61AEA0ccd7bc4a683",
				"name": "Fantasy Ape",
				"decimals": 12
			},
			"0x6241af3817db48a7f9e19fd9446d78e50936d275": {
				"ticker": "JPEG",
				"address": "0x6241af3817Db48a7F9E19FD9446d78E50936d275",
				"name": "JPEG",
				"decimals": 18
			},
			"0x11267a998ba1c54e6acc882d34395aaab86d2f3a": {
				"ticker": "SCGI",
				"address": "0x11267A998ba1c54E6acC882D34395aAAb86d2f3A",
				"name": "Super Corgi",
				"decimals": 10
			},
			"0xf29aab43f53e10a5dc51c7a91971107db0b72115": {
				"ticker": "ALUNA",
				"address": "0xF29aAb43F53E10A5dC51c7a91971107DB0b72115",
				"name": "A brand new Luna",
				"decimals": 18
			},
			"0x649b0a88039958eeeb934b4c1ae45ff7318f4bd9": {
				"ticker": "WHALE",
				"address": "0x649b0A88039958eEeb934B4C1AE45ff7318F4BD9",
				"name": "Queen Whale",
				"decimals": 12
			},
			"0x63831d53fff1d3f9d0ce189f226d51f0c042628f": {
				"ticker": "AKAI",
				"address": "0x63831D53fFF1D3f9d0CE189F226d51f0c042628f",
				"name": "Akita INC",
				"decimals": 12
			},
			"0x09d16578d634064790f5919c75a7d4ea1fab3984": {
				"ticker": "CATS",
				"address": "0x09d16578D634064790f5919c75A7D4ea1FAB3984",
				"name": "CATOSHI",
				"decimals": 18
			},
			"0xb40b244a6f766eedc2b34bb610d2cf40b257d6dc": {
				"ticker": "PEDESTAL",
				"address": "0xB40B244A6f766eeDc2b34bb610d2cF40B257D6DC",
				"name": "Pedestal Node Finance",
				"decimals": 18
			},
			"0x112e61601e5acd0bc9fc7fd8d0482f4b44d5fe4b": {
				"ticker": "ANGC",
				"address": "0x112e61601e5acd0BC9fc7fd8d0482f4b44D5fe4b",
				"name": "Angel Chain",
				"decimals": 12
			},
			"0xf3a28779eb827d75ae4125b08c7f997f29a0dae1": {
				"ticker": "DGE",
				"address": "0xF3A28779Eb827d75ae4125b08c7F997F29a0Dae1",
				"name": "Mini Doge",
				"decimals": 11
			},
			"0x19fdc13bcb0f272e98e59c36d2348baa96929f95": {
				"ticker": "LAPE",
				"address": "0x19FDC13bcB0F272e98e59C36d2348bAa96929f95",
				"name": "Little Ape",
				"decimals": 12
			},
			"0x0f66e896a1595c5cac86094bb018e21e82b83719": {
				"ticker": "CORGI",
				"address": "0x0F66e896A1595c5cAc86094bb018E21E82b83719",
				"name": "Exo Corgi",
				"decimals": 9
			},
			"0x16d8bf67c8429209f85173cca67ac3b882746fb4": {
				"ticker": "GLTCH",
				"address": "0x16D8bf67C8429209F85173ccA67aC3b882746fb4",
				"name": "Little Glitch",
				"decimals": 10
			},
			"0x19a9bbb7fbefa0b10ab1d1407d7e4cbeb1feff96": {
				"ticker": "SNIPER",
				"address": "0x19A9BbB7fBEfa0B10aB1D1407D7e4Cbeb1feFf96",
				"name": "Sniper Nodes",
				"decimals": 9
			},
			"0xcad9e5c3c2c5d0390253c1a8d81d0715dceb90b2": {
				"ticker": "CHI",
				"address": "0xcAd9E5C3c2c5d0390253C1A8D81D0715dCeB90b2",
				"name": "Mini Hachi",
				"decimals": 9
			},
			"0xbd46c7b381f40e7872f1e039aab144a096b1d194": {
				"ticker": "PDA",
				"address": "0xBD46C7B381f40E7872F1e039aaB144a096B1d194",
				"name": "Panda Rocket",
				"decimals": 11
			},
			"0x5104a66907af201384342f22793a03d880c75573": {
				"ticker": "BCRG",
				"address": "0x5104a66907af201384342F22793A03d880c75573",
				"name": "Baby Corgi",
				"decimals": 9
			},
			"0x71fc560c87b62cca62c4e1b2fb09b806d9d48a95": {
				"ticker": "GUN",
				"address": "0x71FC560C87B62ccA62C4e1b2FB09b806D9D48A95",
				"name": "Gun Nodes",
				"decimals": 9
			},
			"0xf27f88fa2bbe916b8e117d39a0c623c8003d8de9": {
				"ticker": "TNDS",
				"address": "0xF27F88fA2bBE916b8E117d39A0c623c8003D8dE9",
				"name": "Thunder Swap",
				"decimals": 12
			},
			"0x8e6b7c875c210d465a41df007a398050db8bd00f": {
				"ticker": "$MILK",
				"address": "0x8e6B7c875C210D465A41df007a398050DB8BD00f",
				"name": "MILK TOKEN",
				"decimals": 18
			},
			"0x67170cfe1c5f4e27bd65164ad038656c76a058bf": {
				"ticker": "MRS",
				"address": "0x67170cFE1C5F4e27bD65164ad038656C76a058bf",
				"name": "Mars Coin",
				"decimals": 12
			},
			"0x1ab0bb215bbc43008c8bf446fd891f79c390c96b": {
				"ticker": "ANG",
				"address": "0x1AB0bb215bbC43008c8Bf446fD891f79C390c96B",
				"name": "Treasure Angel",
				"decimals": 12
			},
			"0x2e37e02035d2f2c9980c4ff6631c34de23642e39": {
				"ticker": "TOM",
				"address": "0x2E37E02035D2F2C9980C4FF6631c34De23642E39",
				"name": "Fantom SV",
				"decimals": 11
			},
			"0xc530f4142d74abb2fa71c941d498eb6e878df66e": {
				"ticker": "GShares",
				"address": "0xC530F4142D74abB2FA71c941D498Eb6E878dF66E",
				"name": " Glamour Shares",
				"decimals": 18
			},
			"0x084e6aaffbf5b40f22270b2f0b4a160a38fc51d6": {
				"ticker": "BRAIN",
				"address": "0x084E6aAffBf5B40f22270B2f0B4A160A38Fc51d6",
				"name": "Brain Shares",
				"decimals": 18
			},
			"0x42db6d4c3dbea58574afdd141d6967fab83bf0e0": {
				"ticker": "CHIMP",
				"address": "0x42dB6d4C3dbeA58574afDD141d6967FaB83bF0E0",
				"name": "Chimp Fund",
				"decimals": 11
			},
			"0xd9de1bfb3bb846c9441ecbac076d0e9bdfcc4373": {
				"ticker": "CRAFT",
				"address": "0xd9de1bFB3bB846C9441ecbaC076d0e9bDFcC4373",
				"name": "CRAFT",
				"decimals": 18
			},
			"0xd377e4e270809223280d01144ac93cdcb40a87d3": {
				"ticker": "WMV",
				"address": "0xd377e4E270809223280D01144AC93cDcB40a87D3",
				"name": "Woke_Mind_Virus",
				"decimals": 9
			},
			"0x9a9870a79a1025720e27d51a5830068c15e04f99": {
				"ticker": "MAG2",
				"address": "0x9A9870A79a1025720E27D51a5830068C15e04F99",
				"name": "MAGNETIZER",
				"decimals": 9
			},
			"0xf5d38b90c7612105d8f576a0b7e62aaae3490275": {
				"ticker": "CHI",
				"address": "0xf5D38b90C7612105D8F576A0B7e62AaAe3490275",
				"name": "King Hachi",
				"decimals": 12
			},
			"0xd3d6bef780f0e7a3f6fa3444262e1ab1c4ff6dc9": {
				"ticker": "HUG",
				"address": "0xd3d6bEF780f0E7a3f6FA3444262E1AB1C4ff6dC9",
				"name": "Hug DAO",
				"decimals": 18
			},
			"0x254aebd959934bed1388b32c56c3ed47fb4cb0b2": {
				"ticker": "C98",
				"address": "0x254aEbd959934bEd1388B32C56C3ED47fB4cb0b2",
				"name": "Coin98 Wallet",
				"decimals": 9
			},
			"0x9e3ca00f2d4a9e5d4f0add0900de5f15050812cf": {
				"ticker": "NFTD",
				"address": "0x9E3Ca00f2d4A9E5d4f0add0900de5f15050812cF",
				"name": "NFTrade Token [via ChainPort.io]",
				"decimals": 18
			},
			"0x39b161934178a5bf1cddfe3f47ca616d45a6e9ed": {
				"ticker": "DGE",
				"address": "0x39b161934178a5BF1cDDfe3f47cA616D45a6e9Ed",
				"name": "Doge Rise",
				"decimals": 11
			},
			"0xaabc158e981de006cf5edcb462b15cab20d756f7": {
				"ticker": "SHOP",
				"address": "0xAabC158E981De006cf5edCB462B15CAB20D756f7",
				"name": "Shopify MetaMall",
				"decimals": 9
			},
			"0xe4b7f6fc95ff8c59b01be4cee89da20901629ec0": {
				"ticker": "TEST",
				"address": "0xe4B7F6FC95fF8c59B01Be4cEE89DA20901629EC0",
				"name": "TEST",
				"decimals": 18
			},
			"0x1c4b61d0be6a061ea844cc0619d6887ba0f1dc5b": {
				"ticker": "Nostrum",
				"address": "0x1C4B61d0Be6A061eA844cc0619D6887ba0f1Dc5b",
				"name": "Nostrum",
				"decimals": 18
			},
			"0x69b31d508f755d77d5d1dc063f6c4f800a0bf8de": {
				"ticker": "RTFY",
				"address": "0x69b31D508F755D77d5d1dC063f6C4F800A0BF8de",
				"name": "Ratify",
				"decimals": 18
			},
			"0x0b71e5357867d89be80f5c331e310e3f4e431248": {
				"ticker": "FAN",
				"address": "0x0b71E5357867D89be80F5C331e310E3f4E431248",
				"name": "Sushi Fantom",
				"decimals": 10
			},
			"0x564a341df6c126f90cf3ecb92120fd7190acb401": {
				"ticker": "TRYB",
				"address": "0x564A341Df6C126f90cf3ECB92120FD7190ACb401",
				"name": "BiLira",
				"decimals": 6
			},
			"0xedef80e8346f1a43d1f64a6a9d7e0eb84217b83b": {
				"ticker": "GTH",
				"address": "0xeDeF80E8346f1A43d1f64a6a9d7e0Eb84217B83b",
				"name": "Glitch AVAX",
				"decimals": 9
			},
			"0x5f737f7c4d5c2ee7977974fd7885e91a5a2a8768": {
				"ticker": "SAPL",
				"address": "0x5f737f7C4d5c2ee7977974Fd7885E91a5A2A8768",
				"name": "Super Pineapple",
				"decimals": 10
			},
			"0xa23d539030fa8e8aa4d667634e28d1b22b4ef9a1": {
				"ticker": "ELNS",
				"address": "0xA23D539030FA8E8aA4D667634E28D1B22B4eF9A1",
				"name": "Elon Swap",
				"decimals": 11
			},
			"0xcba91bb2c31f38525fe5b3a666ef443a0a9483ca": {
				"ticker": "JET",
				"address": "0xcBA91bB2c31f38525fE5b3A666ef443a0a9483Ca",
				"name": "F15 Jets",
				"decimals": 9
			},
			"0xf6de72c35aec9fd2ea239c77c125971bedfa5c07": {
				"ticker": "RoboTime",
				"address": "0xF6De72C35AEC9FD2eA239C77C125971BedFa5C07",
				"name": "RoboTime",
				"decimals": 9
			},
			"0x58a1b8a64062d18de51ae68404cce9d85ec503f8": {
				"ticker": "Arak",
				"address": "0x58A1B8a64062D18DE51AE68404cCE9D85eC503F8",
				"name": "ArakNode",
				"decimals": 6
			},
			"0x045867dc8afb8678e3398064875943c380c6a48d": {
				"ticker": "CHIMP",
				"address": "0x045867dc8AfB8678e3398064875943C380C6A48d",
				"name": "Super Chimp",
				"decimals": 11
			},
			"0xf72d7709e526f541716a738e9ede9510b0ac1309": {
				"ticker": "TND",
				"address": "0xF72d7709E526f541716a738e9EDE9510b0aC1309",
				"name": "Thunder SV",
				"decimals": 10
			},
			"0x89cd490d623989502e09b0cbfa3e268419f58518": {
				"ticker": "HCH",
				"address": "0x89cD490d623989502E09B0CbFa3e268419f58518",
				"name": "Hachi AVAX",
				"decimals": 10
			},
			"0x58e84123026aa13ddc1faf12efce4d9ccb7b5a62": {
				"ticker": "CRG",
				"address": "0x58E84123026aa13DDC1fAf12eFCE4D9cCB7B5A62",
				"name": "Safe Corgi",
				"decimals": 12
			},
			"0x4e6184c7e052476a33261bf7494bfdea0dfabee0": {
				"ticker": "MGOL",
				"address": "0x4E6184c7E052476A33261bF7494BFdEA0DFaBee0",
				"name": "Mini Gold",
				"decimals": 12
			},
			"0x429fb142d22da3fbb0ff6304d22492472f770408": {
				"ticker": "CRG",
				"address": "0x429fb142D22da3FBb0Ff6304D22492472f770408",
				"name": "Zen Corgi",
				"decimals": 10
			},
			"0x005281ded69b9425f775155d9f6bf12713a5e181": {
				"ticker": "AVAX-60.5",
				"address": "0x005281ded69b9425f775155d9F6Bf12713A5e181",
				"name": "AVAX-60.5 Strike Token",
				"decimals": 18
			},
			"0x739c5c570b85e54ed7236b6a5bdb11058b0a9f56": {
				"ticker": "REALMS",
				"address": "0x739C5c570B85E54eD7236B6a5BDb11058b0a9f56",
				"name": " Avax Realms",
				"decimals": 9
			},
			"0x609fd06934921d8a93d5858f9c10cdcb699f6fb9": {
				"ticker": "$KANDY",
				"address": "0x609fd06934921D8a93D5858f9c10cDCb699f6FB9",
				"name": "Kandy",
				"decimals": 1
			},
			"0x925a8ba267512a9f241c916eb2796c00a8b05ef8": {
				"ticker": "DOG",
				"address": "0x925a8BA267512A9F241C916eb2796c00a8b05EF8",
				"name": "Safe Doge",
				"decimals": 12
			},
			"0x62beaf4fdcbd3d51e5b4a9065dde499f3b87de80": {
				"ticker": "THN",
				"address": "0x62bEAf4FDcbD3d51E5b4a9065Dde499F3B87De80",
				"name": "Thunder AVA",
				"decimals": 12
			},
			"0x4fbf0429599460d327bd5f55625e30e4fc066095": {
				"ticker": "TSD",
				"address": "0x4fbf0429599460D327BD5F55625E30E4fC066095",
				"name": "TSD Stablecoin",
				"decimals": 18
			},
			"0xfe15c2695f1f920da45c30aae47d11de51007af9": {
				"ticker": "JLP",
				"address": "0xFE15c2695F1F920da45C30AAE47d11dE51007AF9",
				"name": "Joe LP Token",
				"decimals": 18
			},
			"0x6c383579d73e79dd04746b3882a99ffd70efe966": {
				"ticker": "GHST",
				"address": "0x6c383579D73e79DD04746B3882A99FfD70efE966",
				"name": "Ghost Protocol",
				"decimals": 10
			},
			"0x0034af59bd61a3d40e6e5789cabddbfda6efa642": {
				"ticker": "MAFIN",
				"address": "0x0034aF59Bd61a3d40e6e5789cABDdbFDa6EFa642",
				"name": "Mumble Manatee Finance",
				"decimals": 9
			},
			"0xc68a6fd7344230f45f79624e81e96028c74d50d2": {
				"ticker": "ELON",
				"address": "0xC68A6fd7344230F45f79624E81E96028C74D50D2",
				"name": "Safe Elon",
				"decimals": 12
			},
			"0xa77dbd204eb7e4064ea8fdb00888a3327d061c90": {
				"ticker": "SAPL",
				"address": "0xA77DbD204eb7e4064eA8Fdb00888A3327d061C90",
				"name": "Sushi Apple",
				"decimals": 10
			},
			"0x48f42efba5d863cdd5b65275ba3dc4563e271105": {
				"ticker": "CORGI",
				"address": "0x48F42efBA5D863cDd5b65275Ba3dc4563E271105",
				"name": "Mini Corgi",
				"decimals": 12
			},
			"0x46cb14f0d35f03f960be8e4d1cda46661805eca5": {
				"ticker": "YDR",
				"address": "0x46CB14F0d35F03f960BE8E4d1CdA46661805eCa5",
				"name": "YDRagon",
				"decimals": 18
			},
			"0x3cf52e56d6977a38be8c2c1796961b5e0a7b6394": {
				"ticker": "ATOM",
				"address": "0x3cf52E56d6977a38be8c2C1796961b5e0A7B6394",
				"name": "Atonomous Finance",
				"decimals": 9
			},
			"0x594dec546df261af7d4390c4c4f6cfb08922f7d8": {
				"ticker": "DGE",
				"address": "0x594deC546Df261aF7d4390C4C4f6CFb08922f7d8",
				"name": "Mini Doge",
				"decimals": 10
			},
			"0x641e3862ec51eaff9368e39279f46db2a9aa5c47": {
				"ticker": "SKY",
				"address": "0x641e3862eC51EafF9368e39279F46DB2a9aA5C47",
				"name": "SKY",
				"decimals": 18
			},
			"0xd74079738212e7c24c487d6b43a77acba16a597b": {
				"ticker": "DOGE",
				"address": "0xd74079738212E7C24C487d6b43A77acBA16A597B",
				"name": "Crypto Doge",
				"decimals": 9
			},
			"0xbaf83689eb6d342f95b24bf5bbd397b61bb4cf52": {
				"ticker": "APET",
				"address": "0xBaf83689eb6d342F95B24bF5BbD397B61bb4cF52",
				"name": "Ape Treasure",
				"decimals": 11
			},
			"0x8ab38639fe9b8297a88738420cb72d7b248b246b": {
				"ticker": "ARX",
				"address": "0x8ab38639fe9b8297a88738420cB72D7B248B246b",
				"name": "AvaRocks",
				"decimals": 12
			},
			"0x4ce3f65c19a46a8b9fb8e022a5f8b8b38d3203e3": {
				"ticker": "ELN",
				"address": "0x4Ce3f65c19a46a8b9FB8e022A5F8b8B38D3203E3",
				"name": "Safe Elon",
				"decimals": 9
			},
			"0x46624d1686ba349f67fceda35509ff2c6b4cddba": {
				"ticker": "SKULLZ",
				"address": "0x46624D1686bA349f67fcEDa35509fF2c6B4cddBA",
				"name": "Mad Skullz NFT & Gaming",
				"decimals": 9
			},
			"0x4610984cf06037800a1e6b42f484193a5d70f1bc": {
				"ticker": "BLAZE",
				"address": "0x4610984Cf06037800A1E6b42F484193A5d70F1BC",
				"name": "BLAZING APE",
				"decimals": 9
			},
			"0x4aa94104106aeff1ee5898e1fb61a69355adbf7c": {
				"ticker": "MARS",
				"address": "0x4aA94104106aEfF1ee5898e1fb61a69355aDBf7C",
				"name": "Exo Mars",
				"decimals": 10
			},
			"0x7086c48c997b8597a1692798155b4fcf2cee7f0f": {
				"ticker": "AVAT",
				"address": "0x7086C48c997b8597a1692798155B4fCf2cee7F0f",
				"name": "AVAT",
				"decimals": 6
			},
			"0x287e4d82d0844d020d3e5eafa95c9cad8f2e1fcd": {
				"ticker": "SECRET",
				"address": "0x287E4d82d0844d020d3E5EAFa95C9CAd8f2e1Fcd",
				"name": "SECRET DAO",
				"decimals": 9
			},
			"0x4860a28be33ad04dadfc91722728effa8430f836": {
				"ticker": "SPEAR",
				"address": "0x4860A28Be33aD04dAdFc91722728EFfa8430F836",
				"name": "SpearFinanceRewardsToken",
				"decimals": 9
			},
			"0x99ab4b3035a61964c4956a662044a4da3d4a2b10": {
				"ticker": "SLOT",
				"address": "0x99aB4b3035a61964c4956A662044a4DA3d4a2B10",
				"name": "Snowtomb Lot",
				"decimals": 18
			},
			"0x5abd84bdd512d8794694a5a131b2dc79ffbc5cde": {
				"ticker": "KAREN",
				"address": "0x5abd84bdd512d8794694A5A131b2Dc79ffBc5CdE",
				"name": "Karen Dao",
				"decimals": 9
			},
			"0x52eb19a050e595ebe0e0437359a3e72d5d667e74": {
				"ticker": "AKTF",
				"address": "0x52eb19A050e595EBE0E0437359a3E72D5d667E74",
				"name": "Akita Farm",
				"decimals": 12
			},
			"0xf65e52c5579d503b1477e62e4ce3bb1bf7ee1a92": {
				"ticker": "MGLD",
				"address": "0xF65E52c5579d503B1477E62e4CE3bB1Bf7Ee1A92",
				"name": "Meta Gold",
				"decimals": 12
			},
			"0xb09fe1613fe03e7361319d2a43edc17422f36b09": {
				"ticker": "BOG",
				"address": "0xB09FE1613fE03E7361319d2a43eDc17422f36B09",
				"name": "Bogged Finance",
				"decimals": 18
			},
			"0xebc241c706bb0b226fb07db99189a0f357985729": {
				"ticker": "WPAN",
				"address": "0xeBC241C706bB0B226FB07db99189A0f357985729",
				"name": "Wrapped Panda",
				"decimals": 10
			},
			"0x9d3d88788cc503a8ad5021ad6e780399a1f2b100": {
				"ticker": "CHI",
				"address": "0x9D3d88788CC503A8aD5021Ad6e780399A1F2B100",
				"name": "Queen Chimp",
				"decimals": 9
			},
			"0x0120bce55cdd2fc2d35d2dc46a03879763892381": {
				"ticker": "HCHC",
				"address": "0x0120bcE55cDD2FC2D35D2DC46a03879763892381",
				"name": "Hachi Chain",
				"decimals": 12
			},
			"0x3a5a5939ec134d693460fcd2c247b99d49e7bfbb": {
				"ticker": "GTHF",
				"address": "0x3A5A5939Ec134D693460fcd2C247B99d49E7bFBB",
				"name": "Glitch Fund",
				"decimals": 11
			},
			"0xf7dd4d8142c1fa34c64d8c22fcbba42208cc65eb": {
				"ticker": "420",
				"address": "0xF7dD4D8142c1fA34c64d8c22FcbBa42208cc65eb",
				"name": "Token 420",
				"decimals": 18
			},
			"0xb1bbdb8bcc3175061f1f700e66dd3868c00b1158": {
				"ticker": "TOMS",
				"address": "0xB1BBdb8BCc3175061F1f700e66Dd3868c00b1158",
				"name": "Fantom Swap",
				"decimals": 9
			},
			"0x5ccff6723f592c223e7b31c6872ba999a028653f": {
				"ticker": "KARMA",
				"address": "0x5Ccff6723F592c223E7b31c6872Ba999A028653F",
				"name": "KARMA",
				"decimals": 9
			},
			"0xa8985bafe89630cb9d8334cd29f8ee8c449155d4": {
				"ticker": "SPC",
				"address": "0xa8985BafE89630cb9D8334cd29F8EE8c449155D4",
				"name": "SpicyDAO",
				"decimals": 18
			},
			"0x3817ba5af3e3054a558ddfda87d9b7b7c2b94f5f": {
				"ticker": "LOVE",
				"address": "0x3817bA5aF3e3054A558ddFDA87d9b7b7C2B94f5f",
				"name": "LoveToken",
				"decimals": 18
			},
			"0x8a7488226194ff17042138c2627c95e8fb9cb176": {
				"ticker": "MOON",
				"address": "0x8a7488226194Ff17042138C2627c95e8FB9cb176",
				"name": "Moon Swap",
				"decimals": 10
			},
			"0xb57d1ce73090bc287a38ab6e0f450b0aa651e487": {
				"ticker": "SCRG",
				"address": "0xB57D1Ce73090Bc287A38Ab6e0F450b0aA651E487",
				"name": "Safe Corgi",
				"decimals": 12
			},
			"0xc2a4f65c29dbd82dbe92b59cc49b998005e91145": {
				"ticker": "GTH",
				"address": "0xc2a4F65C29dBD82dbE92B59Cc49B998005e91145",
				"name": "Mini Glitch",
				"decimals": 10
			},
			"0x48358bfaa1ec39aafcb0786c3e0342db676df93e": {
				"ticker": "CAT",
				"address": "0x48358BfAA1EC39AafCb0786c3e0342Db676Df93E",
				"name": "Cleopatra DAO",
				"decimals": 9
			},
			"0x82d2f5e72e2d0130d2970a4f04c0630add1ec52d": {
				"ticker": "HCH",
				"address": "0x82D2F5E72e2D0130D2970a4F04c0630adD1ec52d",
				"name": "Hachi Token",
				"decimals": 11
			},
			"0x9595feba703fd6662ab3f0c867ccb45153f86c3a": {
				"ticker": "ZAP",
				"address": "0x9595FEbA703Fd6662AB3F0c867cCB45153f86c3a",
				"name": "Mini Thunder",
				"decimals": 10
			},
			"0x2c053b4744dada953efc7a5c3b6c4b6dafc4271c": {
				"ticker": "GOLD",
				"address": "0x2c053b4744dada953eFc7a5c3b6c4b6DaFc4271C",
				"name": "Avax Gold",
				"decimals": 9
			},
			"0x95d16e89294d4afaac327a794ad378fa7d861c64": {
				"ticker": "GOLT",
				"address": "0x95D16E89294d4AFaac327A794Ad378Fa7d861C64",
				"name": "Gold Token",
				"decimals": 12
			},
			"0x6469ec6cce7cd4a3b9b217fde6832a9b32c8cf3d": {
				"ticker": "CRG",
				"address": "0x6469Ec6Cce7cD4a3b9b217FdE6832a9b32c8cF3d",
				"name": "Corgi Dapp",
				"decimals": 11
			},
			"0x4558eba4845e79de1a026546c00a8e7bdfc32067": {
				"ticker": "SGX",
				"address": "0x4558eBa4845e79DE1a026546c00A8E7bdfc32067",
				"name": "Subgenix Token",
				"decimals": 18
			},
			"0x7a82492b4a29deb7fb57309d4ecd44a41c0941dd": {
				"ticker": "SBA",
				"address": "0x7a82492B4a29Deb7Fb57309d4Ecd44a41C0941Dd",
				"name": "Little Shiba",
				"decimals": 10
			},
			"0x29d7be19da25e37243a03b8a25e62aef8f51a5f5": {
				"ticker": "NOBEL",
				"address": "0x29d7Be19DA25e37243a03B8A25E62AeF8f51A5F5",
				"name": "NobeliumDAO",
				"decimals": 1
			},
			"0xa1e82af4c2c3a2ea04b8eb8c33c2c4c0cc352918": {
				"ticker": "GHST",
				"address": "0xA1e82af4C2c3a2ea04B8eb8C33c2C4c0CC352918",
				"name": "Ghost NET",
				"decimals": 10
			},
			"0xc8c70f0b840fd397c2811e8106b7053f011f1041": {
				"ticker": "WHL",
				"address": "0xc8c70f0b840fD397C2811E8106B7053f011f1041",
				"name": "Whale Robot",
				"decimals": 10
			},
			"0xa85508ad8bfdc398686de6c04293d5547559cba0": {
				"ticker": "🥛",
				"address": "0xA85508AD8bFdc398686dE6C04293D5547559cBA0",
				"name": "Unicorn Milk",
				"decimals": 9
			},
			"0x4db1a96ee90a4fb08e0489f55bfaf5f5af294a13": {
				"ticker": "OSIR",
				"address": "0x4DB1A96ee90a4fb08E0489f55bfaf5F5aF294a13",
				"name": "osiris.holdings",
				"decimals": 9
			},
			"0x7662e870ee18867f2f1937affa976b726207e698": {
				"ticker": "DINO",
				"address": "0x7662E870EE18867F2f1937aFfA976B726207E698",
				"name": "Jurassic Nodes",
				"decimals": 18
			},
			"0x74dbeb4419c6822b905775a94fc444bb968ffedb": {
				"ticker": "FLK",
				"address": "0x74DbEb4419c6822b905775A94fC444BB968Ffedb",
				"name": "Floki Chain",
				"decimals": 11
			},
			"0x59903a261b9c89769af7e7e6615c2db88a18c35b": {
				"ticker": "HCI",
				"address": "0x59903a261B9C89769AF7E7E6615C2db88A18C35b",
				"name": "Hachi Network",
				"decimals": 11
			},
			"0xe1be13aba14ac1b99fba539e9c4172f2d3bdb338": {
				"ticker": "PINI",
				"address": "0xE1BE13ABA14ac1B99FbA539E9C4172F2D3BDb338",
				"name": "Pineapple Infinity",
				"decimals": 12
			},
			"0x599f1f060fdfda7054281cda51229ed7b96065d2": {
				"ticker": "NGMA",
				"address": "0x599f1F060FdFDA7054281cdA51229ED7b96065d2",
				"name": "nGMA DAO",
				"decimals": 9
			},
			"0x6a708a2f9906a8a7eaf7b8d5e75b0e83c88fc329": {
				"ticker": "SUB",
				"address": "0x6a708A2f9906a8a7eAF7B8D5e75b0e83C88fC329",
				"name": "Subzero",
				"decimals": 18
			},
			"0x482ee40f12af90a4aada9de35a81942324a92da3": {
				"ticker": "EXT",
				"address": "0x482ee40f12aF90a4aAda9de35A81942324A92dA3",
				"name": "ext",
				"decimals": 18
			},
			"0xf12762683b1f294f178b2c8690eed31fca1e144f": {
				"ticker": "DGEB",
				"address": "0xf12762683B1F294f178b2c8690eed31FCa1e144f",
				"name": "Doge Beast",
				"decimals": 10
			},
			"0xbb89e8ce73d3ca88b7ea959afc3f39a82c8c34fd": {
				"ticker": "SPAWN",
				"address": "0xbB89e8Ce73d3Ca88b7ea959aFc3F39A82C8c34fd",
				"name": "Avax Frogs Spawn",
				"decimals": 9
			},
			"0xd00fd0222ddb9e71eaede7cdee3606e775288713": {
				"ticker": "MRS",
				"address": "0xd00fD0222ddB9e71EAEDe7cdee3606e775288713",
				"name": "Mars Robot",
				"decimals": 9
			},
			"0x35a0eebcb9b9b6c20e2afd13ac78bda45d28562b": {
				"ticker": "TNDQ",
				"address": "0x35a0eebCb9B9B6C20e2aFd13AC78BDA45d28562B",
				"name": "Thunder Queen",
				"decimals": 11
			},
			"0x56cad4a07a19d8b9127361a66232dfeb0a03b65b": {
				"ticker": "HCI",
				"address": "0x56CAd4a07a19d8B9127361A66232dfEb0a03B65b",
				"name": "Hachi Diamond",
				"decimals": 10
			},
			"0xed6438b126264f5b6467226bbe888c5c5a4b207e": {
				"ticker": "WHLC",
				"address": "0xEd6438b126264f5b6467226bbE888c5c5a4b207e",
				"name": "Whale Classic",
				"decimals": 12
			},
			"0x2cd3c849dedbfd21f85af6b370646ccdbe022ca6": {
				"ticker": "TRS",
				"address": "0x2Cd3c849dedBfD21F85AF6b370646ccdbE022ca6",
				"name": "AVAX Treasure",
				"decimals": 9
			},
			"0xc111a3496ff543bb0888d755dd60e47c5a85c018": {
				"ticker": "FLOKI",
				"address": "0xC111A3496FF543Bb0888D755DD60e47C5a85C018",
				"name": "Floki BTC",
				"decimals": 12
			},
			"0x903791e83ccf7de04a6c0f0ca058b0af4f4272b6": {
				"ticker": "MOON",
				"address": "0x903791E83Ccf7de04A6C0f0cA058B0AF4f4272b6",
				"name": "Moon X",
				"decimals": 9
			},
			"0x1ee561cdfef92c663c289771b3a5053997b57a83": {
				"ticker": "JUICE",
				"address": "0x1Ee561CDfeF92c663c289771b3A5053997B57A83",
				"name": "JUICE",
				"decimals": 18
			},
			"0xf48e00cd491a4cb610ef72d922a4035d0acec211": {
				"ticker": "SGHT",
				"address": "0xF48e00cd491A4cb610eF72d922a4035d0acEc211",
				"name": "Sushi Ghost",
				"decimals": 9
			},
			"0x141f378fc0e1fb4b7f14c6d6f879f76302158151": {
				"ticker": "DOG",
				"address": "0x141f378FC0E1FB4b7F14C6D6f879F76302158151",
				"name": "Super Doge",
				"decimals": 12
			},
			"0xbb8ffd6930a87378ae289b70678010e2aa5bf617": {
				"ticker": "ETHB",
				"address": "0xBB8FfD6930a87378ae289B70678010e2aA5bF617",
				"name": "ETH Bridge",
				"decimals": 9
			},
			"0x9137167fee28d46982a1e7ac0c33ac578f637d39": {
				"ticker": "GOLD",
				"address": "0x9137167FeE28D46982a1E7aC0C33ac578F637D39",
				"name": "Avax Gold",
				"decimals": 9
			},
			"0x2b130d88a1d9c63f7dc43174f6824f56e2a3fe15": {
				"ticker": "GOSB",
				"address": "0x2b130d88A1D9C63F7dc43174F6824F56E2A3fE15",
				"name": "Ghost BTC",
				"decimals": 12
			},
			"0xc7a2dc5946bbb4ee0eed4b4e8da8b78ef41c5393": {
				"ticker": "SDGE",
				"address": "0xC7A2Dc5946BBB4ee0EED4b4e8Da8B78Ef41c5393",
				"name": "Super Doge",
				"decimals": 9
			},
			"0x31c4c046efad4b04b823a919cc0bdd0f663c87d4": {
				"ticker": "wsSB",
				"address": "0x31C4c046EFAD4B04b823a919CC0bDd0f663c87d4",
				"name": "Wrapped sSB",
				"decimals": 18
			},
			"0x31852b99cf9a3d88c5c2205d3fa17ef31cc6644f": {
				"ticker": "YETI",
				"address": "0x31852B99cf9A3d88c5C2205D3fa17Ef31CC6644F",
				"name": "Yeti Finance",
				"decimals": 18
			},
			"0xe06b2a26744fbcf2014255c8862a919a3eb4bf1f": {
				"ticker": "ELON",
				"address": "0xe06B2a26744fBcF2014255C8862a919a3Eb4BF1F",
				"name": "Elon Farm",
				"decimals": 11
			},
			"0xdd2584c8d7561afdd6eab7ed96cd5ddd2dfc0577": {
				"ticker": "WHALE",
				"address": "0xdd2584c8D7561aFDD6EaB7ED96cd5ddD2Dfc0577",
				"name": "Wrapped Whale",
				"decimals": 9
			},
			"0x97963eb9776151b471d346c296c4338f06819784": {
				"ticker": "HASHI",
				"address": "0x97963Eb9776151B471d346C296C4338F06819784",
				"name": "HappyShiba",
				"decimals": 18
			},
			"0x84c9fb160f6430f657649d1579d5c3f49af0522d": {
				"ticker": "STR",
				"address": "0x84c9fb160F6430f657649d1579D5C3f49af0522D",
				"name": "Zen Star",
				"decimals": 9
			},
			"0xf353847235dc829c7f167f0153dd42b9df18d6b7": {
				"ticker": "ASTRO",
				"address": "0xF353847235dc829c7f167f0153dD42B9df18D6B7",
				"name": "Astro Token",
				"decimals": 18
			},
			"0x37d5f6ad7eab2f9d877da1c0df1344e5bfbd0c8f": {
				"ticker": "MCHAIN",
				"address": "0x37D5F6Ad7EaB2F9D877dA1C0dF1344e5BfBd0c8f",
				"name": "MultiChain.org Bridge",
				"decimals": 9
			},
			"0x7daa5714ea91dfef78635f9fdea89f7d5b732aec": {
				"ticker": "MOON",
				"address": "0x7DAa5714ea91DfEF78635f9fdEA89f7d5b732aEc",
				"name": "Little Moon",
				"decimals": 11
			},
			"0xa6830d076d09d5cfeb9c5dbf782bc7ec4c44c1e0": {
				"ticker": "APL",
				"address": "0xA6830D076d09d5CFEB9c5dbF782bc7Ec4c44c1E0",
				"name": "Exo Pineapple",
				"decimals": 12
			},
			"0xe0bb6fed446a2dbb27f84d3c27c4ed8ea7603366": {
				"ticker": "HOOF",
				"address": "0xe0bb6feD446A2dbb27F84D3C27C4ED8EA7603366",
				"name": "Metaderby game token",
				"decimals": 18
			},
			"0x36c3d154cbb1fb3ece882a5f9ff3dabf489adf34": {
				"ticker": "GLT",
				"address": "0x36C3d154Cbb1Fb3ece882a5F9fF3dabf489AdF34",
				"name": "OMG Glitch",
				"decimals": 10
			},
			"0x69b87a8ed02246775be8b17711a8c2398204f257": {
				"ticker": "CHIMP",
				"address": "0x69B87A8Ed02246775bE8b17711a8C2398204F257",
				"name": "Chimp AVAX",
				"decimals": 9
			},
			"0xfe77997a1cc756230e2ddb990c72ae58f39c7b32": {
				"ticker": "SuFi",
				"address": "0xFe77997A1cC756230e2ddB990C72Ae58f39C7b32",
				"name": "Supplement Finance",
				"decimals": 9
			},
			"0x90dedd9d786709d4fcf9f73b22240e37b6d45962": {
				"ticker": "OXY",
				"address": "0x90dEDd9d786709d4FCf9F73B22240e37B6d45962",
				"name": "Oxy-Fi",
				"decimals": 18
			},
			"0xc03419747aa7d9f6c12e5c90785b3868f6ed51ad": {
				"ticker": "FKI",
				"address": "0xc03419747aa7D9F6c12e5c90785b3868F6ED51AD",
				"name": "Floki Farm",
				"decimals": 10
			},
			"0x397bbd6a0e41bdf4c3f971731e180db8ad06ebc1": {
				"ticker": "AVXT",
				"address": "0x397bBd6A0E41bdF4C3F971731E180Db8Ad06eBc1",
				"name": "Avaxtars Token",
				"decimals": 6
			},
			"0x5ce66ea0cb8b11c738cfac6b507ea42dbc0171aa": {
				"ticker": "WAR",
				"address": "0x5ce66ea0Cb8B11C738CFac6B507Ea42Dbc0171aA",
				"name": "WAR DAO",
				"decimals": 9
			},
			"0x99dbfc55429093180f101171d98091a40f480c0b": {
				"ticker": "CMP",
				"address": "0x99DbfC55429093180f101171D98091A40f480C0b",
				"name": "Crypto Chimp",
				"decimals": 9
			},
			"0x4467dd901d7fcc3674158b0c403fd81116df585d": {
				"ticker": "YUM",
				"address": "0x4467DD901d7FCC3674158b0C403Fd81116df585d",
				"name": "Cake Finance",
				"decimals": 11
			},
			"0xdef1fac7bf08f173d286bbbdcbeeade695129840": {
				"ticker": "CERBY",
				"address": "0xdef1fac7Bf08f173D286BbBDcBeeADe695129840",
				"name": "Cerby Token",
				"decimals": 18
			},
			"0x14210b5177e309ec8e3524cd944c0d72d74346d6": {
				"ticker": "VENUM",
				"address": "0x14210b5177e309ec8E3524cD944C0d72D74346d6",
				"name": "Venum",
				"decimals": 9
			},
			"0xc1722e262bbfd0839be7e857748ba3b5361c8832": {
				"ticker": "ELON",
				"address": "0xc1722e262BBfd0839bE7e857748bA3B5361C8832",
				"name": "Elon Starter",
				"decimals": 11
			},
			"0x8c30651ed62eaf230c0b427ca4471347aa87a227": {
				"ticker": "LAVA",
				"address": "0x8c30651eD62eaF230c0b427cA4471347AA87A227",
				"name": "LAVA",
				"decimals": 18
			},
			"0xf03be3f2482cc635e4eea16af755ba87867c19de": {
				"ticker": "FTMN",
				"address": "0xf03bE3F2482cc635e4EeA16Af755BA87867C19de",
				"name": "Fantom Network",
				"decimals": 9
			},
			"0x701f26ab61ad19acc9f218b42500d7598c50130c": {
				"ticker": "EGG",
				"address": "0x701f26AB61aD19Acc9F218b42500D7598c50130c",
				"name": "EGG",
				"decimals": 9
			},
			"0x68960ebaa7e6dc5643a95fef44589b6d28bfb201": {
				"ticker": "PURGE",
				"address": "0x68960EbAa7E6dC5643a95FEF44589B6d28BFb201",
				"name": "The Purge 2022",
				"decimals": 9
			},
			"0xe29d1b668d70543588ad3379cdbb3386fe0f66cd": {
				"ticker": "MICRO",
				"address": "0xe29D1b668d70543588aD3379cdbb3386fE0f66cd",
				"name": " Micro Nodes",
				"decimals": 9
			},
			"0x966562cc3537cc213d523519283a30920fa18555": {
				"ticker": "DGE",
				"address": "0x966562Cc3537cc213d523519283a30920fa18555",
				"name": "Safe Doge",
				"decimals": 9
			},
			"0xc2e16ab0bec01b95c434ec00f271c518d0f6a295": {
				"ticker": "APE",
				"address": "0xc2e16aB0bEC01b95c434ec00F271c518D0f6a295",
				"name": "Ape Island",
				"decimals": 9
			},
			"0xadbd0ab3da50ee385f1c13038795b85385a59103": {
				"ticker": "NXTG",
				"address": "0xADBD0AB3dA50ee385f1C13038795b85385A59103",
				"name": "Next Gene Games",
				"decimals": 9
			},
			"0xdfb2a363da4146b2bf2559daf6645b0502a3d732": {
				"ticker": "SLR",
				"address": "0xDfB2a363dA4146b2bf2559dAF6645B0502a3D732",
				"name": "Stellarium",
				"decimals": 18
			},
			"0xba647fc00e42a0bec263d608d96071ce0ae59719": {
				"ticker": "APL",
				"address": "0xBA647fc00E42a0BEc263d608D96071cE0ae59719",
				"name": "Apple Queen",
				"decimals": 9
			},
			"0x8b18069e76ce2c5a8ff0df20acb2997f229baf6c": {
				"ticker": "APE",
				"address": "0x8B18069e76Ce2C5a8fF0Df20ACb2997F229BAf6c",
				"name": "APE NODES (20APE PER NODE)",
				"decimals": 9
			},
			"0x7bd6d49198e3696a73f8fea4b1a32d0f354b0a69": {
				"ticker": "CYBER",
				"address": "0x7bD6D49198e3696a73f8fEa4b1a32D0F354B0A69",
				"name": "Cyber Node",
				"decimals": 1
			},
			"0xb9dfd4e99b08666913627f709cf9828b5b250055": {
				"ticker": "CHA",
				"address": "0xB9DFd4e99B08666913627f709Cf9828b5B250055",
				"name": "Chica INU",
				"decimals": 9
			},
			"0xe761afc921ec7ff11df025f8967d08e3daa805a8": {
				"ticker": "SNO",
				"address": "0xe761AFC921EC7FF11DF025f8967d08E3daA805a8",
				"name": "Snowball",
				"decimals": 9
			},
			"0x9b99768b1e367ec7381ca8be603e5f72f5e467bc": {
				"ticker": "PEDESTAL",
				"address": "0x9B99768B1e367eC7381ca8Be603E5F72f5e467bc",
				"name": "Pedestal Node Finance",
				"decimals": 18
			},
			"0x30f0901c5cfd47e82aef52238c41a3ad73b1ca80": {
				"ticker": "Eve",
				"address": "0x30F0901c5CfD47E82aeF52238c41A3Ad73b1cA80",
				"name": "Everland DAO",
				"decimals": 18
			},
			"0x066906c39d97bc58fa8c5ea0c400a377d708373d": {
				"ticker": "FLKL",
				"address": "0x066906C39d97Bc58FA8C5Ea0c400a377d708373d",
				"name": "Floki Ledger",
				"decimals": 10
			},
			"0x6999245e22fd83a68f1b7f684970e84764a3e658": {
				"ticker": "AKITA",
				"address": "0x6999245e22fd83a68F1b7F684970e84764a3E658",
				"name": "Akita Project",
				"decimals": 12
			},
			"0x440abbf18c54b2782a4917b80a1746d3a2c2cce1": {
				"ticker": "SHIBX",
				"address": "0x440aBbf18c54b2782A4917b80a1746d3A2c2Cce1",
				"name": "SHIBAVAX",
				"decimals": 18
			},
			"0x6768abc58e9d29556cb18635bf28fab889eb86bb": {
				"ticker": "MONEY",
				"address": "0x6768Abc58E9D29556cB18635BF28fAb889eB86bB",
				"name": "Money DAO",
				"decimals": 9
			},
			"0x7a5d4d1a4a9af2f92f3a2f853409aea357747472": {
				"ticker": "FUCK",
				"address": "0x7a5D4D1A4A9Af2F92f3A2f853409aea357747472",
				"name": "FUCK 2021",
				"decimals": 9
			},
			"0xef6f6521b5489a9199f1049a5488e1b8fffce44c": {
				"ticker": "CHIMP",
				"address": "0xEf6f6521b5489a9199F1049A5488e1b8FFFce44c",
				"name": "Crypto Chimp",
				"decimals": 9
			},
			"0xc47a5eca06559c88a3a80d1ecee8a23726743c53": {
				"ticker": "TOM",
				"address": "0xC47A5ECa06559c88a3a80d1eCeE8a23726743C53",
				"name": "Meta Fantom",
				"decimals": 9
			},
			"0xe764ea2b89973dc230931bae3fafde2c9591fc46": {
				"ticker": "HCHT",
				"address": "0xE764ea2B89973dC230931BAe3FaFde2C9591fc46",
				"name": "Hachi Token",
				"decimals": 11
			},
			"0xf08edc04c68e20944294b3b837d6c068f2c289e5": {
				"ticker": "FTND",
				"address": "0xf08edC04C68E20944294B3b837d6c068F2C289e5",
				"name": "Fantasy Thunder",
				"decimals": 10
			},
			"0x3a0501bb03567fc751b72059fd648de4414268c0": {
				"ticker": "AXM",
				"address": "0x3a0501bB03567FC751B72059Fd648de4414268C0",
				"name": "Fantastic Protocol AXM Token",
				"decimals": 18
			},
			"0x8499ce0bc7ab6d1c8e1d67cc40850ac4ebb801cd": {
				"ticker": "RFI",
				"address": "0x8499ce0Bc7aB6d1c8e1d67CC40850Ac4ebb801cD",
				"name": "Richie-Fi",
				"decimals": 18
			},
			"0xc036dac0498058e04eecb46e3bdc000bf46b14fe": {
				"ticker": "BAT",
				"address": "0xc036DAC0498058e04eEcB46E3BDC000bf46b14fe",
				"name": "Battle Nodes",
				"decimals": 9
			},
			"0x156dcf455c24856b8b92507fb74a9d211cb3c103": {
				"ticker": "ANGI",
				"address": "0x156dcF455c24856b8b92507fB74A9D211cB3c103",
				"name": "Angel Inu",
				"decimals": 10
			},
			"0xbb6680476b85f68b6aee1fa1829118b05c4c6999": {
				"ticker": "CAKE",
				"address": "0xbB6680476B85F68B6aee1fa1829118B05C4C6999",
				"name": "Cake Block",
				"decimals": 12
			},
			"0x822be31284598180e1f5d2084d4dcc008bfee9f0": {
				"ticker": "VAX",
				"address": "0x822Be31284598180e1F5d2084d4dCc008BfeE9F0",
				"name": "Avaxination",
				"decimals": 7
			},
			"0x008b6923a5e2290a1d8624ca9f0746a384499a0e": {
				"ticker": "ANG",
				"address": "0x008b6923a5e2290A1D8624ca9f0746a384499A0e",
				"name": "OMG Angel",
				"decimals": 10
			},
			"0x11bb362ae6c15103493e27b82f60e6ea399430cf": {
				"ticker": "MOON",
				"address": "0x11BB362AE6c15103493E27b82f60e6eA399430Cf",
				"name": "Crypto Moon",
				"decimals": 9
			},
			"0xef30b9ca3f4a42c6773c501ef1bbef7af9e6d924": {
				"ticker": "APE",
				"address": "0xeF30b9cA3f4a42C6773C501eF1BBeF7AF9E6d924",
				"name": "Ape AVAX",
				"decimals": 11
			},
			"0xd0095e52a5c4075f279ac3246ca66d34f885162f": {
				"ticker": "APED",
				"address": "0xd0095e52a5c4075F279AC3246Ca66D34f885162F",
				"name": "Ape Diamond",
				"decimals": 11
			},
			"0x772d1db345f50350fca0c6089a5be56f34757cf6": {
				"ticker": "SBAI",
				"address": "0x772D1dB345F50350fCA0C6089a5BE56F34757Cf6",
				"name": "Shiba INC",
				"decimals": 11
			},
			"0xbb48c261bd5cf62e963413ada6ac40c7a603fa38": {
				"ticker": "ELON",
				"address": "0xBb48C261BD5cf62E963413ada6ac40c7A603FA38",
				"name": "Elon Fund",
				"decimals": 11
			},
			"0xea6e5b6d392b39f4c42b38f9d846bfa9dd23c7d8": {
				"ticker": "DOG",
				"address": "0xea6e5b6D392b39F4C42b38f9d846BFa9Dd23c7d8",
				"name": "Doge INC",
				"decimals": 9
			},
			"0xa0105d7fc6190598523f568001a71164341b0a22": {
				"ticker": "SUB",
				"address": "0xA0105D7fc6190598523f568001A71164341b0A22",
				"name": "Subzero",
				"decimals": 18
			},
			"0xc7b86bb9c1f148505ed1c855001d8b7224f334d9": {
				"ticker": "DOGE",
				"address": "0xC7B86bb9c1F148505ED1c855001D8B7224f334d9",
				"name": "Wrapped Doge",
				"decimals": 12
			},
			"0x9ee2f95d6b26a921b7af2c6ce9fd3445fb1eca73": {
				"ticker": "NemesisDA",
				"address": "0x9EE2F95d6b26a921b7Af2c6cE9fd3445fb1eca73",
				"name": "Nemesis DAO",
				"decimals": 18
			},
			"0x733faf85c4816d7221ae96abe0a38fae83d31d79": {
				"ticker": "APL",
				"address": "0x733fAf85C4816d7221AE96aBe0A38FaE83d31d79",
				"name": "Apple Fund",
				"decimals": 12
			},
			"0x7086e045b78e1e72f741f25231c08d238812cf8a": {
				"ticker": "RACEX",
				"address": "0x7086e045b78E1e72F741F25231c08d238812CF8a",
				"name": "RaceX",
				"decimals": 18
			},
			"0x332c07b4cb1e031fa6fd99c976a088b566376e42": {
				"ticker": "OZARK",
				"address": "0x332C07b4cb1E031Fa6Fd99C976a088B566376E42",
				"name": "Ozark Cash",
				"decimals": 9
			},
			"0xcdfe48b5157161380fc6df4c5f023ba2ed9f396e": {
				"ticker": "SAMOAS",
				"address": "0xCDFE48b5157161380fc6DF4C5F023BA2Ed9f396E",
				"name": "Cookie Project",
				"decimals": 9
			},
			"0x11255417caa30c2acb9623eb751d8c208305c88f": {
				"ticker": "ACD",
				"address": "0x11255417CAA30C2ACB9623EB751d8C208305C88f",
				"name": "Ascenders",
				"decimals": 18
			},
			"0xe266fa8b0687277f810b5f7ceaff06ee8d14a9a3": {
				"ticker": "APE",
				"address": "0xE266fA8b0687277f810b5F7CeAFf06eE8d14A9a3",
				"name": "Little Ape",
				"decimals": 10
			},
			"0xef3a28035fe545cb4c86e110f1b72e3fc6a21206": {
				"ticker": "5TABLE",
				"address": "0xEF3a28035fE545cB4c86e110f1B72e3fC6a21206",
				"name": "5table Token",
				"decimals": 18
			},
			"0x67926d973cd8ee876ad210faaf7dffa99e414acf": {
				"ticker": "JLP",
				"address": "0x67926d973cD8eE876aD210fAaf7DFfA99E414aCf",
				"name": "Joe LP Token",
				"decimals": 18
			},
			"0xc024019e53ab2eccd14b3a2dbf1e6604a8026c1c": {
				"ticker": "LENS",
				"address": "0xC024019e53AB2eCcD14b3A2DBf1E6604A8026C1C",
				"name": "LensProtocol.eth",
				"decimals": 18
			},
			"0xe52e27607e07dbe8fecbc1f5dfde9bec5116f0ce": {
				"ticker": "PND",
				"address": "0xe52E27607e07dbE8FECbc1F5dfde9BEc5116f0ce",
				"name": "Project Panda",
				"decimals": 9
			},
			"0xa10bf60a568c25a69f58b3a0f9df2a3a39f852c9": {
				"ticker": "PINATA",
				"address": "0xA10Bf60a568C25A69F58b3a0F9df2a3A39F852c9",
				"name": "NFT Management",
				"decimals": 9
			},
			"0xcbcb29e4915058c2028154db46ab46333af53f94": {
				"ticker": "GUNS",
				"address": "0xcBcb29e4915058C2028154Db46ab46333aF53f94",
				"name": "Ghost Guns",
				"decimals": 9
			},
			"0x7598ebe093e459b398670e06ec70e686c3c5a094": {
				"ticker": "SEED",
				"address": "0x7598EBE093e459B398670e06ec70E686C3C5a094",
				"name": "SEED",
				"decimals": 18
			},
			"0x906d2a223ec25a699d1740180fc9ad7a8069dde6": {
				"ticker": "PRICH",
				"address": "0x906D2a223ec25a699D1740180fC9ad7A8069dDe6",
				"name": "Rich PRINTER",
				"decimals": 18
			},
			"0xafe3d2a31231230875dee1fa1eef14a412443d22": {
				"ticker": "DFIAT",
				"address": "0xAfE3d2A31231230875DEe1fa1eEF14a412443d22",
				"name": "DeFiato [via ChainPort.io]",
				"decimals": 18
			},
			"0x625fc9bb971bb305a2ad63252665dcfe9098bee9": {
				"ticker": "PLEB",
				"address": "0x625fc9bb971BB305A2Ad63252665DcFE9098bEE9",
				"name": "PlebToken",
				"decimals": 18
			},
			"0x7b380511bbd7db8b0613f16c351dd0e4e0e56735": {
				"ticker": "FKIX",
				"address": "0x7b380511bBd7db8B0613F16c351dd0E4E0E56735",
				"name": "Floki X",
				"decimals": 11
			},
			"0x79bc7d34949f56bafc460e511d45faff8762163d": {
				"ticker": "ASND",
				"address": "0x79Bc7D34949f56baFc460e511D45fAFF8762163D",
				"name": "Ascend Node",
				"decimals": 18
			},
			"0xc8b02cf3d9b34e6265f54fdc7e59eb09ac00de1f": {
				"ticker": "CHI",
				"address": "0xc8B02CF3D9b34E6265F54fdC7e59EB09ac00dE1f",
				"name": "Chimp Fund",
				"decimals": 10
			},
			"0xaf30c2c3372605edac2f48ab69b869c60ef8c9b6": {
				"ticker": "Source",
				"address": "0xaf30C2C3372605EdAc2F48aB69B869C60EF8C9b6",
				"name": "Source Node",
				"decimals": 18
			},
			"0x309da17b1c0ebd29f31e44ee3ec18ea5cdde498e": {
				"ticker": "PCC",
				"address": "0x309Da17B1c0eBd29F31E44eE3EC18Ea5CDdE498E",
				"name": "Pump Coin Capital",
				"decimals": 9
			},
			"0xd811945ed4f0386b8e684de5af16211e810c1b4a": {
				"ticker": "SCR",
				"address": "0xD811945ED4F0386b8E684DE5AF16211E810C1b4A",
				"name": "SecureDAO",
				"decimals": 18
			},
			"0xb9096473199052c37aa2176f29cbebd0f6030874": {
				"ticker": "ZAPC",
				"address": "0xb9096473199052c37aa2176F29CbEBD0F6030874",
				"name": "Thunder Classic",
				"decimals": 12
			},
			"0x4f8e9e5ba78ca9cc298a8afcccd1dadaf33b1012": {
				"ticker": "QPAN",
				"address": "0x4F8e9e5bA78ca9CC298A8afccCd1DADaf33b1012",
				"name": "Queen Panda",
				"decimals": 11
			},
			"0x0466a991af2ae353ec0176e161313646525336f3": {
				"ticker": "SCHI",
				"address": "0x0466A991AF2aE353eC0176e161313646525336F3",
				"name": "Sushi Hachi",
				"decimals": 10
			},
			"0x0f10648eae55626a96be861a7be010bdf5d7f9d9": {
				"ticker": "BRZF",
				"address": "0x0F10648EaE55626A96bE861a7be010bdf5D7f9d9",
				"name": "Breeze Finance",
				"decimals": 9
			},
			"0x65581c899bb03ba3d6205d770b80b65bdee8adbc": {
				"ticker": "FUFI",
				"address": "0x65581C899bB03ba3D6205d770b80b65BdEE8adbC",
				"name": "Future Finance",
				"decimals": 9
			},
			"0x33d22b15b20b1c52c9387d20fb58373f9d46a6ef": {
				"ticker": "G-Share",
				"address": "0x33D22B15b20b1c52C9387D20fb58373F9d46a6eF",
				"name": "GrapeFinance",
				"decimals": 18
			},
			"0xa2f783e5d9d71bdd3cde4993d4da08d669098c20": {
				"ticker": "TOMC",
				"address": "0xa2f783E5D9d71BDD3Cde4993d4dA08d669098c20",
				"name": "Fantom Chain",
				"decimals": 9
			},
			"0x2de5c601e5fe23958c103f5a6dacdbb63524b0e6": {
				"ticker": "SUB",
				"address": "0x2de5c601e5fE23958c103f5a6DacdBb63524b0e6",
				"name": "Subzero",
				"decimals": 18
			},
			"0x3de2d3915fe197ae7eea8c85df2b44dcbf1d2793": {
				"ticker": "MAKERSHARE",
				"address": "0x3de2D3915fE197aE7eeA8c85DF2B44Dcbf1d2793",
				"name": "MAKERSHARE",
				"decimals": 18
			},
			"0xa3512b0b085e8a492ea7b32113f65a2a5fae793b": {
				"ticker": "WINGS",
				"address": "0xA3512B0B085E8a492EA7B32113F65A2A5faE793B",
				"name": "Angel Markets",
				"decimals": 12
			},
			"0xa216f344fcd412c129e0c012798a137e332f8955": {
				"ticker": "AVAL",
				"address": "0xA216F344FCd412C129E0c012798A137E332F8955",
				"name": "Avalend Finance",
				"decimals": 18
			},
			"0x88aa17889c73541fb066b8fef8c9e00285ca1807": {
				"ticker": "CAKE",
				"address": "0x88Aa17889c73541fb066B8fef8C9e00285cA1807",
				"name": "Fantasy Cake",
				"decimals": 10
			},
			"0x41aac63564c5a6fab58518d0cb1fd7de5f3fe739": {
				"ticker": "FOXYMOON",
				"address": "0x41aaC63564c5a6fab58518D0cB1fD7de5f3fe739",
				"name": "MiniFoxyMoon",
				"decimals": 18
			},
			"0x4c2e4be2581b93f7e1db56c86e55635dfc7da06a": {
				"ticker": "AKA",
				"address": "0x4C2E4be2581B93F7E1DB56C86e55635DFC7DA06A",
				"name": "Meta Akita",
				"decimals": 10
			},
			"0x6fc28a16334b790a29414e078125e9d4ae40abc7": {
				"ticker": "ZFAN",
				"address": "0x6Fc28a16334b790a29414e078125e9D4ae40ABC7",
				"name": "Zen Fantom",
				"decimals": 9
			},
			"0x54c23a9763d28458d2d501ec0810e59e237f6cec": {
				"ticker": "SolarDA",
				"address": "0x54c23A9763D28458D2d501EC0810E59e237f6CeC",
				"name": "SolarDAO (https://t.me/SolarDAOAvax)",
				"decimals": 18
			},
			"0xcf8419a615c57511807236751c0af38db4ba3351": {
				"ticker": "AXIAL",
				"address": "0xcF8419A615c57511807236751c0AF38Db4ba3351",
				"name": "AxialToken",
				"decimals": 18
			},
			"0xb6f32d3ea1a97da3742e594d24531158fee9fdb0": {
				"ticker": "AVAX-45.5",
				"address": "0xB6f32D3ea1a97Da3742E594d24531158fEE9fDB0",
				"name": "AVAX-45.5 Strike Token",
				"decimals": 18
			},
			"0x97884ccaff1265e6cd1e110e5fd945cbeb4d02a2": {
				"ticker": "INU",
				"address": "0x97884CcAFf1265E6CD1E110e5fD945CBEB4D02A2",
				"name": "IHU INU",
				"decimals": 18
			},
			"0xdbbe7a58bdddd8f52747f38d2d57771728afa08b": {
				"ticker": "PWR",
				"address": "0xdBbE7A58bdddD8f52747f38D2d57771728afA08B",
				"name": "POWER",
				"decimals": 9
			},
			"0x402fd175049e95cef2cc9ca1fecdb6d9736e690d": {
				"ticker": "CATS",
				"address": "0x402FD175049e95CeF2cC9ca1FECdb6d9736e690d",
				"name": "CATOSHI",
				"decimals": 18
			},
			"0xf645334c4cc4740120c1210cfee03c204c4152c2": {
				"ticker": "AVP",
				"address": "0xF645334C4Cc4740120c1210cFeE03c204C4152c2",
				"name": "@avaxpepes",
				"decimals": 9
			},
			"0x09e09d10605567fc9f9cb89e988b96624b006b2f": {
				"ticker": "FLOKI",
				"address": "0x09E09D10605567fC9F9cB89E988B96624b006b2f",
				"name": "Super Floki",
				"decimals": 10
			},
			"0x9b43bddfe1c23517d49b7c76ee01dc4b5112f682": {
				"ticker": "DMD",
				"address": "0x9b43BdDfe1C23517D49B7c76eE01DC4B5112f682",
				"name": "Diamond Capital",
				"decimals": 9
			},
			"0x23f0ddad63fdb56dd480eb453c1a59a661ccd61c": {
				"ticker": "WAR",
				"address": "0x23f0dDAd63Fdb56dD480eb453C1a59A661ccd61c",
				"name": "War Games",
				"decimals": 9
			},
			"0xf4d340acd52841ca37251398e725048e73b6ffae": {
				"ticker": "CASH",
				"address": "0xF4D340acD52841CA37251398e725048e73B6ffAe",
				"name": "Cash Nodes (20CASH per Node)",
				"decimals": 9
			},
			"0x0aa0f0341937c2b286555f7f30ea3d9ab25fcfbd": {
				"ticker": "MOON",
				"address": "0x0AA0F0341937c2b286555f7F30eA3d9Ab25FCFBD",
				"name": "Moon Swap",
				"decimals": 10
			},
			"0xcedf067e3fc7312868ce1620b7cd471f89e21e82": {
				"ticker": "APL",
				"address": "0xcEdf067e3fc7312868ce1620B7CD471F89e21E82",
				"name": "Apple Ledger",
				"decimals": 10
			},
			"0x42f33f70be8a9c062b68ee64e7ff34dd2ed2f5d8": {
				"ticker": "CCC",
				"address": "0x42F33F70Be8A9C062B68ee64e7Ff34dD2ed2f5D8",
				"name": "Cross Chain Capital",
				"decimals": 9
			},
			"0x3f9679ee39a8ea88cea8fab111c3fbea12b4ee03": {
				"ticker": "BLAST",
				"address": "0x3F9679ee39A8Ea88CeA8fAB111c3fbea12B4EE03",
				"name": "Auto Blaster",
				"decimals": 9
			},
			"0xeacd4f4d93527cb5d0cdfe5930d612ccfd5af436": {
				"ticker": "BANANA",
				"address": "0xeacD4F4d93527CB5d0cDFE5930D612CCfD5Af436",
				"name": "GorillaNodes",
				"decimals": 18
			},
			"0x3883488f1f7a5e5f51d190acc3491581149fabd3": {
				"ticker": "NUKE",
				"address": "0x3883488f1F7a5E5f51D190ACC3491581149fabd3",
				"name": "Nuke Nodes",
				"decimals": 9
			},
			"0x8a8a6dd05dddcf46bc11454f1663888c5d182eae": {
				"ticker": "APE",
				"address": "0x8A8A6dd05DDDcf46BC11454F1663888c5D182eae",
				"name": "APE Nodes",
				"decimals": 9
			},
			"0x14e7c6b4f5602a8f17e40786676a27741a17868e": {
				"ticker": "LIBERTY",
				"address": "0x14E7c6B4F5602A8f17e40786676A27741A17868E",
				"name": "Prisoners Island",
				"decimals": 18
			},
			"0xba5d4fb1937ce8bd3931d7e15db15e355a7e54f5": {
				"ticker": "SMRS",
				"address": "0xBA5d4fb1937Ce8bd3931d7E15dB15E355A7e54f5",
				"name": "Sushi Mars",
				"decimals": 9
			},
			"0xefdbc7c17f6dad7c306ef28730a69928105fb0fd": {
				"ticker": "KGLT",
				"address": "0xefdBC7C17F6DAd7c306Ef28730A69928105fb0FD",
				"name": "King Glitch",
				"decimals": 12
			},
			"0x232d0d2687bdda2189cdffa1d7e13b0c6f44c15f": {
				"ticker": "Cube",
				"address": "0x232D0D2687bdDa2189cDFFA1D7E13b0c6F44c15F",
				"name": "Cube Finance",
				"decimals": 18
			},
			"0xf4625148efa2d3e160399b3ffb22230c9a4544ed": {
				"ticker": "PRINT",
				"address": "0xF4625148efa2d3E160399b3ffb22230C9a4544Ed",
				"name": "PrintCoin",
				"decimals": 18
			},
			"0xa168312eb48182ab2a5f302f4da96e427b5c9e97": {
				"ticker": "Phoenix",
				"address": "0xA168312Eb48182ab2a5f302f4da96E427b5C9E97",
				"name": "Phoenix Financial",
				"decimals": 18
			},
			"0xb0a6e056b587d0a85640b39b1cb44086f7a26a1e": {
				"ticker": "ODDZ",
				"address": "0xB0a6e056B587D0a85640b39b1cB44086F7a26A1E",
				"name": "OddzToken",
				"decimals": 18
			},
			"0x449259f3f1e981d80bbc575e3d118af6278e1ea6": {
				"ticker": "BATNET",
				"address": "0x449259f3F1E981D80bBC575e3d118af6278E1eA6",
				"name": "Battle Net Gaming",
				"decimals": 9
			},
			"0xb2c433bca2b5ceb383e27057d67d8ddd00c4d59e": {
				"ticker": "GUIZER",
				"address": "0xb2c433bCA2B5ceb383e27057d67d8Ddd00C4D59E",
				"name": "Guizer",
				"decimals": 9
			},
			"0x82be8a6fd11a11eec1bb832d5f6d1c5c285f4732": {
				"ticker": "GHST",
				"address": "0x82BE8a6Fd11a11EeC1Bb832D5f6d1c5C285F4732",
				"name": "Ghost Rise",
				"decimals": 9
			},
			"0x7eb5bc237cd2d1214ddefda166c8918cdfa1e20f": {
				"ticker": "100MB",
				"address": "0x7eB5bC237CD2D1214DdeFdA166C8918cdFa1E20f",
				"name": "100MB",
				"decimals": 18
			},
			"0xe79a47c8bed71bafe09884bd4d99a29b824fd4c8": {
				"ticker": "AGA",
				"address": "0xE79a47c8BEd71BAFE09884bd4d99A29b824Fd4C8",
				"name": "Agamotto",
				"decimals": 18
			},
			"0xbc6f589171d6d66eb44ebcc92dffb570db4208da": {
				"ticker": "wave",
				"address": "0xbc6f589171d6d66EB44ebCC92dFFb570Db4208da",
				"name": "wave token",
				"decimals": 18
			},
			"0x83f2cbad9b23e3bce1e8d5e2e7b0637f74f98e47": {
				"ticker": "MOON",
				"address": "0x83F2cbAd9B23E3BcE1e8D5e2E7b0637f74F98e47",
				"name": "Fantasy Moon",
				"decimals": 9
			},
			"0xc367cf133071d11fddc74bd65e915048d3bee3d4": {
				"ticker": "EVO",
				"address": "0xc367Cf133071d11Fddc74bd65e915048D3Bee3d4",
				"name": "EVO",
				"decimals": 18
			},
			"0x0bede42da30a4dcf801904b1b661ab49f4e72175": {
				"ticker": "LINK",
				"address": "0x0bEDE42da30a4DCF801904b1b661aB49f4E72175",
				"name": "Link Protocol",
				"decimals": 9
			},
			"0x7350610b1d972a5dd3b5cad1d2138be53520af44": {
				"ticker": "GUN",
				"address": "0x7350610B1d972a5dD3b5cAd1D2138BE53520Af44",
				"name": "Gun Finance",
				"decimals": 18
			},
			"0xd9615dc109a83796048c625ef1a5940cd38070ee": {
				"ticker": "BUD",
				"address": "0xd9615dC109A83796048C625eF1a5940CD38070ee",
				"name": "Bud Shares",
				"decimals": 18
			},
			"0x4be500b4d1b706137d39231437fb87fca93396d9": {
				"ticker": "AXI",
				"address": "0x4Be500B4d1B706137d39231437FB87fCA93396D9",
				"name": "AVAXSUKI",
				"decimals": 18
			},
			"0x0e75cdb914ddcca683357dd94d26caa39bc14b16": {
				"ticker": "PB",
				"address": "0x0E75CDb914DdCcA683357dD94d26Caa39BC14B16",
				"name": "PiggyBank",
				"decimals": 9
			},
			"0xef4de2014920afa636947c069f858f062324db93": {
				"ticker": "WAR",
				"address": "0xef4dE2014920AFA636947c069F858f062324db93",
				"name": "AWAR",
				"decimals": 9
			},
			"0x78ea3fef1c1f07348199bf44f45b803b9b0dbe28": {
				"ticker": "FLY",
				"address": "0x78Ea3fef1c1f07348199Bf44f45b803b9B0Dbe28",
				"name": "FLY",
				"decimals": 18
			},
			"0x9a6208e5ac1163a2bda6381a9b1eb6a375695c4e": {
				"ticker": "BAKT",
				"address": "0x9A6208e5aC1163a2BDA6381a9B1Eb6a375695c4E",
				"name": "Baby Akita",
				"decimals": 12
			},
			"0x163742b013b525749360f99597d12cce8bd754b9": {
				"ticker": "SHARKS",
				"address": "0x163742b013B525749360F99597D12Cce8BD754B9",
				"name": "Secret Sharks",
				"decimals": 9
			},
			"0x63c7f8a8489e69caf13ec190fbddccc9d44760b7": {
				"ticker": "PABLO",
				"address": "0x63c7f8A8489E69CAF13ec190FBDDcCC9D44760b7",
				"name": "Medellin City",
				"decimals": 18
			},
			"0xf824f37cfe2a1a0bae55dbd58701414a47a8f9d9": {
				"ticker": "GOSM",
				"address": "0xf824F37cFE2a1a0BAE55dbD58701414A47a8f9d9",
				"name": "Ghost Markets",
				"decimals": 12
			},
			"0xd92b8e4c49a4fae6b9824f01936283907433ef66": {
				"ticker": "TND",
				"address": "0xD92b8e4c49a4Fae6b9824f01936283907433Ef66",
				"name": "Thunder Classic",
				"decimals": 10
			},
			"0x70a916b789c37378c6ac5d1983b83ff02fb249c3": {
				"ticker": "ELN",
				"address": "0x70A916b789C37378c6aC5D1983b83FF02FB249c3",
				"name": "Safe Elon",
				"decimals": 9
			},
			"0x8d88e48465f30acfb8dac0b3e35c9d6d7d36abaf": {
				"ticker": "CNR",
				"address": "0x8D88e48465F30Acfb8daC0b3E35c9D6D7d36abaf",
				"name": "Canary",
				"decimals": 18
			},
			"0x3ba3bc077b669e059e0cb8b51c27879130f26e61": {
				"ticker": "ELND",
				"address": "0x3ba3bC077b669E059e0Cb8B51c27879130F26e61",
				"name": "Elon Diamond",
				"decimals": 10
			},
			"0xc379588ff6d525f24737885982776fd1bf273182": {
				"ticker": "SATOSHI",
				"address": "0xc379588FF6D525f24737885982776FD1bF273182",
				"name": "Satoshi Nodes",
				"decimals": 9
			},
			"0x4ec60b76c292425ec2807ab8d55833ba6fe2c526": {
				"ticker": "FIGHT",
				"address": "0x4eC60B76c292425eC2807aB8d55833BA6Fe2c526",
				"name": "Fight Club Game",
				"decimals": 9
			},
			"0x956725c16dd8c4146c142c89f70de66150ab3b5d": {
				"ticker": "TGHT",
				"address": "0x956725C16DD8c4146c142C89F70DE66150ab3B5d",
				"name": "Treasure Ghost",
				"decimals": 10
			},
			"0xa4cbabb67747bcd38dd15e05f98ccf698e94acb9": {
				"ticker": "PNPL",
				"address": "0xA4cbaBB67747bCD38Dd15E05f98CcF698E94ACb9",
				"name": "Pineapple BTC",
				"decimals": 9
			},
			"0x268f015d2d2c39ee36864054c4a00fbb6cd9282c": {
				"ticker": "MAG",
				"address": "0x268f015d2D2c39EE36864054C4A00fBB6CD9282c",
				"name": "Magic DAO",
				"decimals": 9
			},
			"0x40c99a7212e942d43bd4641c515e21bd31e7aa38": {
				"ticker": "CAPE",
				"address": "0x40C99a7212e942d43BD4641C515e21bd31E7aA38",
				"name": "Crypto Ape",
				"decimals": 12
			},
			"0xc23a0c962c61281f450c282a2eebba080ceeedc7": {
				"ticker": "SUBZERO",
				"address": "0xC23a0C962C61281F450c282A2EEBbA080Ceeedc7",
				"name": "SUBZERO",
				"decimals": 18
			},
			"0x4b97330158b8a068d9edae4b988f2bd0b5ba1c66": {
				"ticker": "APL",
				"address": "0x4b97330158B8a068d9EDAe4b988f2BD0b5BA1c66",
				"name": "Apple Inu",
				"decimals": 11
			},
			"0x94f9315f75ef0648edbcb560db6cb8011992b377": {
				"ticker": "TOMB",
				"address": "0x94F9315F75eF0648eDbCb560DB6CB8011992B377",
				"name": "Fantom Beast",
				"decimals": 11
			},
			"0xa04d5e346ca8771a401ff90a008e6474a0fb54e3": {
				"ticker": "LFLK",
				"address": "0xA04d5E346Ca8771a401FF90a008E6474a0FB54E3",
				"name": "Little Floki",
				"decimals": 12
			},
			"0xa5ebab84d666e550ff5aa29404e1638838734448": {
				"ticker": "TFIG",
				"address": "0xA5eBAb84d666e550ff5aa29404E1638838734448",
				"name": "TFIG",
				"decimals": 9
			},
			"0x87bade473ea0513d4aa7085484aeaa6cb6ebe7e3": {
				"ticker": "MOR",
				"address": "0x87BAde473ea0513D4aA7085484aEAA6cB6EBE7e3",
				"name": "Mor Stablecoin",
				"decimals": 18
			},
			"0x03a77a422db1311800a727683dedd3914d1378e4": {
				"ticker": "KONG",
				"address": "0x03A77a422db1311800A727683DEDD3914D1378e4",
				"name": "Crypto Kong",
				"decimals": 9
			},
			"0x10aee8a70bbf3322856afb071a7e60da446af4bf": {
				"ticker": "SHBS",
				"address": "0x10AeE8A70Bbf3322856afb071A7E60dA446Af4BF",
				"name": "Shiba Swap",
				"decimals": 12
			},
			"0x823fd620a1382cb9ecac988c37d4ff9b5e9b3bbf": {
				"ticker": "DB",
				"address": "0x823Fd620A1382Cb9ecAC988c37d4ff9b5e9B3bbF",
				"name": "DiamondBank",
				"decimals": 9
			},
			"0x92635c8186bfc5d9d18a714d7aa86406e7370c19": {
				"ticker": "LUNA2.0",
				"address": "0x92635c8186bFc5D9d18a714d7aa86406e7370C19",
				"name": "Terra Luna 2.0",
				"decimals": 18
			},
			"0x24f1e55fac88a8f37711ce183b3492b74a7c7b39": {
				"ticker": "PORN",
				"address": "0x24f1E55Fac88A8f37711Ce183b3492B74A7C7B39",
				"name": "Porn",
				"decimals": 18
			},
			"0xb654512afeef8246ceda872e04d06d212b5f9cbb": {
				"ticker": "SHOP",
				"address": "0xB654512AfEeF8246ceda872e04D06d212b5f9cBB",
				"name": "AVAX Shopify Payments",
				"decimals": 9
			},
			"0xe9b7cdaa8aba4efc12aa989946a3a4e754a3b72e": {
				"ticker": "AVAX-50",
				"address": "0xe9B7cdAa8Aba4EfC12AA989946a3A4E754a3b72e",
				"name": "AVAX-50 Strike Token",
				"decimals": 18
			},
			"0x721c299e6bf7d6a430d9bea3364ea197314bce09": {
				"ticker": "MILK2",
				"address": "0x721C299E6BF7D6a430d9bEA3364Ea197314bcE09",
				"name": "MilkyWay Token by SpaceSwap v2",
				"decimals": 18
			},
			"0xd79c9a21a2fedecdcbe5d12469f6672d72868f72": {
				"ticker": "NOBEL",
				"address": "0xd79c9A21a2feDeCDcBe5D12469f6672d72868F72",
				"name": "Nobelium DAO",
				"decimals": 18
			},
			"0x5f81769667b7490d9d2f37075e6ac05d0cbc62f9": {
				"ticker": "APEIN",
				"address": "0x5f81769667b7490D9d2f37075e6Ac05d0cBc62f9",
				"name": "APE ISLAND",
				"decimals": 9
			},
			"0x0802d66f029c46e042b74d543fc43b6705ccb4ba": {
				"ticker": "APE",
				"address": "0x0802d66f029c46E042b74d543fC43B6705ccb4ba",
				"name": "ApeCoin",
				"decimals": 18
			},
			"0x9a9c92ba95854d36da1966fc297572ddd824be74": {
				"ticker": "PXT",
				"address": "0x9a9c92Ba95854d36dA1966fc297572dDD824be74",
				"name": "ProjectXNodes",
				"decimals": 18
			},
			"0x35e11819b8b19fdb1770fb130c10c8d2fda01753": {
				"ticker": "BLOOD",
				"address": "0x35e11819b8B19fdB1770Fb130c10c8D2fda01753",
				"name": "blood.finance",
				"decimals": 18
			},
			"0x8aecc6006f46395633e61b8105ff98675963ec33": {
				"ticker": "SNIPER",
				"address": "0x8AECc6006f46395633E61B8105FF98675963ec33",
				"name": "Snip Protocol",
				"decimals": 9
			},
			"0x1dfae791a2e813ffb64157c5d167ace29ad39b44": {
				"ticker": "ICD",
				"address": "0x1dFAe791a2e813ffb64157C5d167aCe29ad39B44",
				"name": "Ice Cream DAO",
				"decimals": 9
			},
			"0xf03dccaec9a28200a6708c686cf0b8bf26ddc356": {
				"ticker": "YDR",
				"address": "0xf03Dccaec9A28200A6708c686cf0b8BF26dDc356",
				"name": "YDragon",
				"decimals": 18
			},
			"0x3917f6529b730aff574307e0cf4c82a269a43599": {
				"ticker": "WAWE",
				"address": "0x3917f6529B730Aff574307E0CF4C82a269A43599",
				"name": "Wave",
				"decimals": 9
			},
			"0x93229059c9d43b8dfc5004d128b81544be8fef58": {
				"ticker": "CGI",
				"address": "0x93229059C9D43b8dFc5004d128B81544be8feF58",
				"name": "Mini Corgi",
				"decimals": 10
			},
			"0x91a1700835230b8b3b06b5b3dd1fe70d48acbd91": {
				"ticker": "YAXIS",
				"address": "0x91A1700835230B8b3B06B5B3DD1Fe70D48ACbd91",
				"name": "yAxis",
				"decimals": 18
			},
			"0x80fabe6e0960ab934584be5d5496246b850fdf44": {
				"ticker": "SEX",
				"address": "0x80FAbe6e0960AB934584Be5d5496246B850FDF44",
				"name": "SEX",
				"decimals": 18
			},
			"0xedb52b210e83ed2c4d3a5bf4a8db42ff0737fa16": {
				"ticker": "SATOSHI",
				"address": "0xEdb52b210e83ED2c4D3A5bf4A8dB42FF0737fA16",
				"name": "Satoshi Kingdom",
				"decimals": 9
			},
			"0x08bc292953936a973f672dbdb49e1ee0f9ee1150": {
				"ticker": "NINJA",
				"address": "0x08bC292953936a973F672dbdb49E1Ee0f9EE1150",
				"name": "Ninja Shares",
				"decimals": 18
			},
			"0x5035d81f9616b6c1cc9cbcf61b0010bce84e68cb": {
				"ticker": "TORNADO",
				"address": "0x5035D81F9616B6c1cC9cbCf61b0010bcE84e68cb",
				"name": "Tornado Token",
				"decimals": 18
			},
			"0xc3797320dc0299995542134220ecc18609e3b664": {
				"ticker": "APEA",
				"address": "0xc3797320DC0299995542134220Ecc18609E3B664",
				"name": "Ape AVA",
				"decimals": 10
			},
			"0xbebcf93c2f56d8533ffd7829ea86b334c524e8d8": {
				"ticker": "STRA",
				"address": "0xBebcf93C2f56d8533FfD7829Ea86B334C524e8D8",
				"name": "Star AI",
				"decimals": 11
			},
			"0x5fe08c4e18c424e72bedafaff1f7c648a091fcf6": {
				"ticker": "ICE",
				"address": "0x5FE08c4e18c424e72bEDAfAFF1f7c648A091FcF6",
				"name": "IceCream Swap",
				"decimals": 9
			},
			"0x1a877b68bda77d78eea607443ccde667b31b0cdf": {
				"ticker": "PIGGY",
				"address": "0x1a877B68bdA77d78EEa607443CcDE667B31B0CdF",
				"name": "PIGGY",
				"decimals": 18
			},
			"0x7cc6a407a5a3e832b26c909a0e86a2160b861036": {
				"ticker": "GOLB",
				"address": "0x7cc6A407A5A3E832b26c909a0E86a2160b861036",
				"name": "Gold Beast",
				"decimals": 10
			},
			"0x97071750247e40ae6a678a8fb2433c49695f3a6f": {
				"ticker": "SIG",
				"address": "0x97071750247E40ae6A678A8fB2433C49695F3a6f",
				"name": "SigSauer Nodes",
				"decimals": 9
			},
			"0xfb881c914393becf391e6b0534be6522a0a00c1c": {
				"ticker": "DFIAT",
				"address": "0xfb881C914393BeCf391e6B0534bE6522A0a00C1c",
				"name": "DeFiato Native Token",
				"decimals": 18
			},
			"0x56ecbb9b61f018cfb139cff152e6d025fab5cb0c": {
				"ticker": "SHOP",
				"address": "0x56ECBB9b61F018cfb139cFf152E6D025FaB5cb0c",
				"name": "Shopify Payments",
				"decimals": 9
			},
			"0xbfa990e839fbdd7b1aa92c5d6c92b008a14714a9": {
				"ticker": "PUMPY",
				"address": "0xbFA990e839FBdD7b1Aa92C5D6c92b008A14714A9",
				"name": "PumpNation",
				"decimals": 18
			},
			"0x1fbfe184e4ca88734e468ab4a2f352b2a01a8ca2": {
				"ticker": "MOON",
				"address": "0x1FbFe184E4Ca88734E468ab4a2f352b2a01A8cA2",
				"name": "Moon Ledger",
				"decimals": 9
			},
			"0x395397615b5a61fb99207fba65216e7b14048069": {
				"ticker": "SPORE",
				"address": "0x395397615b5a61FB99207FBa65216e7b14048069",
				"name": "SPORE",
				"decimals": 18
			},
			"0x84ff81037331b1ad2b63bd6c832c23773471e4f4": {
				"ticker": "ZAPE",
				"address": "0x84fF81037331b1ad2B63BD6c832c23773471e4f4",
				"name": "Zen Ape",
				"decimals": 12
			},
			"0x5225b230d36e3d72968ad27f3ab05a70de0e0452": {
				"ticker": "ZSBA",
				"address": "0x5225B230d36e3D72968aD27f3AB05A70DE0e0452",
				"name": "Zen Shiba",
				"decimals": 12
			},
			"0x33695142ab1ff9712f852bec09c80c30ecf5e784": {
				"ticker": "CRG",
				"address": "0x33695142ab1Ff9712F852BeC09c80c30ECf5E784",
				"name": "Corgi Network",
				"decimals": 12
			},
			"0x4c9b4e1ac6f24cde3660d5e4ef1ebf77c710c084": {
				"ticker": "LYD",
				"address": "0x4C9B4E1AC6F24CdE3660D5E4Ef1eBF77C710C084",
				"name": "LydiaFinance Token",
				"decimals": 18
			},
			"0xa89d6cb14314101ba63c382a75e1d8937c37e8a1": {
				"ticker": "Yuzu",
				"address": "0xa89D6Cb14314101ba63c382A75e1D8937c37e8a1",
				"name": "Yuzu Nodes",
				"decimals": 18
			},
			"0xe6b6aff2257a76c13c7d83d60dea897ef0976682": {
				"ticker": "SipWine",
				"address": "0xE6B6aFf2257a76C13C7d83D60DEa897Ef0976682",
				"name": "Wine Printer",
				"decimals": 18
			},
			"0x853df32d50299920ac772a4ca212c2abc9acbcc7": {
				"ticker": "DMDM",
				"address": "0x853DF32d50299920AC772A4cA212c2aBC9ACbcC7",
				"name": "Diamond Mine",
				"decimals": 9
			},
			"0x1cf45d280ea2d452300a021fb9ec473a629d1ad4": {
				"ticker": "KO",
				"address": "0x1cf45D280EA2d452300a021FB9EC473A629d1AD4",
				"name": "WOODLEYKO",
				"decimals": 9
			},
			"0x8325d013f9bb9fe22c0dc777649f2533462ed871": {
				"ticker": "FPIN",
				"address": "0x8325d013F9BB9fE22C0dc777649F2533462Ed871",
				"name": "Fantasy Pineapple",
				"decimals": 12
			},
			"0xb05d747b64c58fbf2980a4f17243d5a42ea4094e": {
				"ticker": "BEE",
				"address": "0xb05D747b64c58fbf2980a4F17243D5a42ea4094E",
				"name": "BeetsDAO",
				"decimals": 18
			},
			"0xde9e52f1838951e4d2bb6c59723b003c353979b6": {
				"ticker": "SDOG",
				"address": "0xdE9E52F1838951e4d2bb6C59723B003c353979b6",
				"name": "Snowdog",
				"decimals": 9
			},
			"0xc71db6e5ae455b50b7ae4f8c3a1f83ed5fa99926": {
				"ticker": "BDH",
				"address": "0xC71DB6E5Ae455B50b7Ae4F8c3A1f83ED5fA99926",
				"name": "BandonHub",
				"decimals": 18
			},
			"0xe43b1790c3ee65f2ca450c8b066723eba4f7beea": {
				"ticker": "HIRO",
				"address": "0xE43B1790C3ee65F2CA450c8b066723eba4f7beeA",
				"name": "Hiroyuki inu",
				"decimals": 18
			},
			"0x1df23cf9f455e704de21f3ee411d5e155514afd2": {
				"ticker": "SPLASH",
				"address": "0x1df23CF9F455E704dE21f3Ee411D5e155514AFd2",
				"name": "Splash Games",
				"decimals": 9
			},
			"0xd63fb3c27e3016f14de2fa685bb03d61fcd0180f": {
				"ticker": "MARS",
				"address": "0xD63fb3c27e3016f14De2fa685BB03d61FCD0180f",
				"name": "Wrapped Mars",
				"decimals": 12
			},
			"0x2a18b3418a387ac463c8ce94864f3995710ea0a6": {
				"ticker": "PINS",
				"address": "0x2A18B3418A387ac463c8Ce94864f3995710ea0A6",
				"name": "Pineapple Swap",
				"decimals": 10
			},
			"0x3640ee18631ad34ac24dfc2694b58871f189a17a": {
				"ticker": "PNDT",
				"address": "0x3640Ee18631ad34ac24DFC2694B58871f189a17A",
				"name": "Panda Token",
				"decimals": 9
			},
			"0x76787b335bd0e2676295d3acb0082583104e98b5": {
				"ticker": "WINGS",
				"address": "0x76787b335Bd0e2676295D3aCb0082583104E98B5",
				"name": "Angel AVA",
				"decimals": 11
			},
			"0x21eb3515eca48692ab520ef3ee183984b0985974": {
				"ticker": "PNDN",
				"address": "0x21eB3515EcA48692ab520EF3ee183984b0985974",
				"name": "Panda Network",
				"decimals": 11
			},
			"0xd6435457bbb2bb6248a02dc74d5c22a1d13aae77": {
				"ticker": "ELON",
				"address": "0xd6435457BbB2bb6248a02dc74d5c22a1d13aAe77",
				"name": "Elon Protocol",
				"decimals": 12
			},
			"0xd3429642502480065fd1589e57ebe11915e0625d": {
				"ticker": "CHIMP",
				"address": "0xd3429642502480065Fd1589E57ebe11915e0625d",
				"name": "Chimp Finance",
				"decimals": 9
			},
			"0xd0f43e3fe30ad16629accefdc49fc94ac0d5ea76": {
				"ticker": "SKATHI",
				"address": "0xD0f43E3fe30aD16629AcCeFdc49Fc94ac0d5Ea76",
				"name": "SkadiDAO",
				"decimals": 9
			},
			"0xb596037302a07915c2dc8bbeb471f3c66f350959": {
				"ticker": "CSHB",
				"address": "0xB596037302A07915c2DC8bbeb471f3c66f350959",
				"name": "Crypto Shiba",
				"decimals": 12
			},
			"0xe460f3e0adc58357ea5e3ac1d839611dee826354": {
				"ticker": "SS",
				"address": "0xE460f3E0AdC58357ea5E3AC1D839611DEE826354",
				"name": "SecretSanta DAO",
				"decimals": 18
			},
			"0x4b6c8132de4fa3d336f34b8fb30cb8053c6fc66b": {
				"ticker": "Prout",
				"address": "0x4b6c8132de4Fa3d336f34B8fb30CB8053c6fC66B",
				"name": "Caca token",
				"decimals": 18
			},
			"0x35db4d0c8b036d2abfb4835d8f75af703f1300bd": {
				"ticker": "FLOKI",
				"address": "0x35dB4d0C8b036d2abFb4835d8F75af703F1300bD",
				"name": "Wrapped Floki",
				"decimals": 11
			},
			"0x113876998ec207a237e823c34155eb66205d03a5": {
				"ticker": "MTS",
				"address": "0x113876998ec207A237e823C34155eB66205D03a5",
				"name": "Mytest",
				"decimals": 9
			},
			"0x8e9ebd74e10cc61bc97cbd7716f9fb32c67a6abf": {
				"ticker": "CASH",
				"address": "0x8E9Ebd74E10cC61bC97cbd7716F9FB32C67a6aBf",
				"name": "AVA",
				"decimals": 9
			},
			"0x60285dc5038caf995913122eb94a189ee532e4b2": {
				"ticker": "QUANT",
				"address": "0x60285DC5038CaF995913122eB94a189ee532E4b2",
				"name": "Quant Protocol",
				"decimals": 9
			},
			"0xef3fab03dc7dc9ccc632406fc2d284fd262c176c": {
				"ticker": "HPP",
				"address": "0xEF3Fab03dC7DC9CcC632406fC2d284fD262C176c",
				"name": "HighPoint Presale",
				"decimals": 18
			},
			"0x5b35ff66e89110942df25ca18ec1d1f7e90d234f": {
				"ticker": "CORGI",
				"address": "0x5B35ff66E89110942dF25Ca18Ec1d1f7e90D234f",
				"name": "Exo Corgi",
				"decimals": 9
			},
			"0xff044e9bc225fad984cffd87238c1cb35a311c01": {
				"ticker": "WGLT",
				"address": "0xFF044E9BC225FaD984Cffd87238C1cb35A311c01",
				"name": "Wrapped Glitch",
				"decimals": 11
			},
			"0xa24330a694768e3a8ae23cea20cbe700cef6bd23": {
				"ticker": "PNPL",
				"address": "0xA24330A694768E3a8AE23ceA20Cbe700Cef6bd23",
				"name": "Pineapple Fund",
				"decimals": 12
			},
			"0x6121191018baf067c6dc6b18d42329447a164f05": {
				"ticker": "PIZZA",
				"address": "0x6121191018BAf067c6Dc6B18D42329447a164F05",
				"name": "Pizza",
				"decimals": 18
			},
			"0xff9e9957ccc8b60a2ce10a7d49c740fe6943861a": {
				"ticker": "AVAX-41.5",
				"address": "0xFf9e9957Ccc8B60a2ce10A7d49c740Fe6943861A",
				"name": "AVAX-41.5 Strike Token",
				"decimals": 18
			},
			"0xa76a60b1971fe93daed21c7ab00466657e737991": {
				"ticker": "CHIMP",
				"address": "0xa76A60b1971FE93daed21C7AB00466657E737991",
				"name": "Project Chimp",
				"decimals": 12
			},
			"0xa5b5617899be49536ef0085270569c4514431fc3": {
				"ticker": "FTMBR",
				"address": "0xa5B5617899be49536Ef0085270569C4514431fC3",
				"name": "Fantom/Avax Bridge",
				"decimals": 9
			},
			"0x8f589c209478499ead655ce0a918c1e6ba5007de": {
				"ticker": "MARS",
				"address": "0x8F589c209478499ead655CE0A918c1e6BA5007DE",
				"name": "Mars Token",
				"decimals": 11
			},
			"0xbb648df4e3d69cf5203e3d161f5cd2cc9cfd4350": {
				"ticker": "PLAYER",
				"address": "0xbb648df4E3d69CF5203e3D161f5CD2Cc9cfD4350",
				"name": "CryptoSteam PLAYER token",
				"decimals": 9
			},
			"0x4939b3313e73ae8546b90e53e998e82274afdbdb": {
				"ticker": "CCC",
				"address": "0x4939B3313E73ae8546b90e53E998E82274afDbDB",
				"name": "Cross Chain Capital V2",
				"decimals": 9
			},
			"0x464384444ddacbfc30e8549d07b88a53b4bf9b89": {
				"ticker": "GLTCH",
				"address": "0x464384444dDAcbFc30E8549D07B88A53b4Bf9b89",
				"name": "Crypto Glitch",
				"decimals": 9
			},
			"0xc9f72c9c047e35c852ba60d9c29c1fff0bd5ba11": {
				"ticker": "MEGA",
				"address": "0xC9F72C9C047e35C852bA60d9C29C1fFf0Bd5Ba11",
				"name": "MEGA DAO",
				"decimals": 9
			},
			"0xfcd541302f6fa86c16b51b4e82417f13e8e49b84": {
				"ticker": "MARS",
				"address": "0xfCd541302F6Fa86c16B51B4e82417f13e8E49B84",
				"name": "Mars DAO",
				"decimals": 11
			},
			"0x18a0a1fdaa037201710814e915ddc0d0de219335": {
				"ticker": "PANI",
				"address": "0x18a0A1FdAa037201710814E915dDc0d0De219335",
				"name": "Panda INC",
				"decimals": 10
			},
			"0x5e2f029eeababe7f7bcdaaa50a217ae5a3925658": {
				"ticker": "SECRETS",
				"address": "0x5e2f029eEABabe7f7BcDaAA50A217AE5A3925658",
				"name": "Secret Protocol",
				"decimals": 9
			},
			"0x9c87e91664987905df0a4d1dc78d3987d2c4a334": {
				"ticker": "CMP",
				"address": "0x9C87e91664987905df0A4d1DC78D3987D2C4A334",
				"name": "Sushi Chimp",
				"decimals": 11
			},
			"0xe1be057b41cb78759e717fd19625828f8f6d69cf": {
				"ticker": "GTG",
				"address": "0xE1bE057b41CB78759e717Fd19625828f8F6D69cf",
				"name": "GutenTag",
				"decimals": 18
			},
			"0x07a8761254e3d174d4daffe88ca32b77657432f1": {
				"ticker": "MARS",
				"address": "0x07A8761254E3D174D4dAffe88CA32b77657432f1",
				"name": "Little Mars",
				"decimals": 9
			},
			"0x7f95f2b61fc541c10192fa96d1235444c13fc6c6": {
				"ticker": "DB",
				"address": "0x7F95F2B61fc541c10192FA96D1235444C13Fc6c6",
				"name": "DiamondBank DAO",
				"decimals": 18
			},
			"0xa71089ab49e381b21ba68eba089be67bf22513f7": {
				"ticker": "BGLD",
				"address": "0xA71089Ab49e381B21Ba68eba089be67bF22513f7",
				"name": "Baby Gold",
				"decimals": 12
			},
			"0x1f4bb6932f3e6b18224d51c70ed2eac835af4b4b": {
				"ticker": "NFL",
				"address": "0x1f4BB6932F3e6b18224D51c70Ed2EAc835Af4B4B",
				"name": "SuperBowl LVI",
				"decimals": 9
			},
			"0xd390b21e1d1d2832c5454702fe4f2cfbe66c2bfd": {
				"ticker": "CRGL",
				"address": "0xd390b21e1d1d2832C5454702Fe4f2CFbE66C2BFD",
				"name": "Corgi Ledger",
				"decimals": 11
			},
			"0xf38dede21da8aaf01e82cb63e2883b836b30c101": {
				"ticker": "HEAD",
				"address": "0xF38DeDE21dA8aAf01e82Cb63e2883B836B30c101",
				"name": "HEAD Token",
				"decimals": 18
			},
			"0x4a43a02e735be95f41d11c127a80368a96e1c837": {
				"ticker": "PFKI",
				"address": "0x4A43a02E735bE95f41d11c127a80368A96E1C837",
				"name": "Project Floki",
				"decimals": 11
			},
			"0x79eae864864ddd07827b7398f5cede44675e2295": {
				"ticker": "SBA",
				"address": "0x79EaE864864DDd07827b7398F5ceDe44675e2295",
				"name": "Meta Shiba",
				"decimals": 11
			},
			"0x4dcff4808c4ce54d401299029965fdf00870d604": {
				"ticker": "CHIMP",
				"address": "0x4DcFF4808c4CE54d401299029965fDf00870D604",
				"name": "Chimp Chain",
				"decimals": 12
			},
			"0x9b67cb6d003dd62f61cd85b61029326213ddd41e": {
				"ticker": "FIRE",
				"address": "0x9B67cB6d003Dd62f61cd85b61029326213dDD41E",
				"name": "FireNodes",
				"decimals": 18
			},
			"0xcf06ec3c695861cdea515aa4c48654f08ca862d8": {
				"ticker": "ASND",
				"address": "0xcF06Ec3C695861cdeA515aA4C48654F08ca862d8",
				"name": "Ascend",
				"decimals": 18
			},
			"0x5d6f0fb06a067eabb97b8690409240a0bc9f856f": {
				"ticker": "AirCoin",
				"address": "0x5D6F0Fb06a067EaBB97b8690409240A0BC9F856f",
				"name": "AirCoinDAO",
				"decimals": 18
			},
			"0xf00c03a705a17ae2cce23880b26de4e1a4b3d50b": {
				"ticker": "OPEC",
				"address": "0xf00c03A705a17AE2cce23880b26dE4e1a4b3d50B",
				"name": "Opulence",
				"decimals": 18
			},
			"0x728faf72a33756bc52ecc2d4e471ac65bdeb4a64": {
				"ticker": "SXT",
				"address": "0x728Faf72a33756BC52Ecc2D4e471ac65bDEB4a64",
				"name": "Secret Treasure",
				"decimals": 9
			},
			"0xab87503b70dfe6a9849903feb073449b5593113e": {
				"ticker": "OSD",
				"address": "0xAB87503b70Dfe6a9849903fEB073449b5593113E",
				"name": "The Open DAO - OpenSea.io",
				"decimals": 9
			},
			"0x237bed318a3bb26bdb3790901a7a79cf53c7b80f": {
				"ticker": "BSN",
				"address": "0x237bed318A3bb26BDB3790901a7a79CF53c7B80F",
				"name": "Black Shark Nodes",
				"decimals": 8
			},
			"0x656b51cb40292f60bbbbb80262bafc28ca7115d5": {
				"ticker": "BIT",
				"address": "0x656B51CB40292F60BBBBb80262BaFC28Ca7115d5",
				"name": "Bitstream",
				"decimals": 18
			},
			"0x84c16ab797e33448478800998721143992f54a49": {
				"ticker": "SND",
				"address": "0x84C16ab797E33448478800998721143992f54a49",
				"name": "SpaceNodeDao",
				"decimals": 9
			},
			"0x9fafa3ee75c53400122f8c912d79c0486aa9d0d2": {
				"ticker": "MARS",
				"address": "0x9FaFa3EE75C53400122f8c912d79c0486aa9d0D2",
				"name": "Project Mars",
				"decimals": 12
			},
			"0x78f7dae9301c298dbe3d44630b9776af9c84d64c": {
				"ticker": "FIRE",
				"address": "0x78f7DAe9301c298dBE3d44630b9776aF9c84D64C",
				"name": "Fire Nodes",
				"decimals": 9
			},
			"0x76070326035651d931d97a507972c53f0cfa52c3": {
				"ticker": "SHIB",
				"address": "0x76070326035651d931D97A507972c53f0Cfa52C3",
				"name": "Shiba Markets",
				"decimals": 10
			},
			"0xb67a9374da03d4114a6fb8f0e7f2b82b5cb34ee3": {
				"ticker": "AGF",
				"address": "0xb67a9374Da03d4114a6FB8f0E7F2b82b5cB34ee3",
				"name": "Augmented Finance",
				"decimals": 18
			},
			"0xa9b9f0a572f2ae71206993a44762ac7be0dff4c6": {
				"ticker": "MAGIC",
				"address": "0xA9B9F0A572F2AE71206993A44762AC7be0DFf4c6",
				"name": "Magic Dao",
				"decimals": 9
			},
			"0x094bd7b2d99711a1486fb94d4395801c6d0fddcc": {
				"ticker": "TEDDY",
				"address": "0x094bd7B2D99711A1486FB94d4395801C6d0fdDcC",
				"name": "TEDDY",
				"decimals": 18
			},
			"0xaa17259ecc377e11f948da668f7dc2eb090f37ce": {
				"ticker": "MRSR",
				"address": "0xAA17259ECc377e11F948DA668f7dC2eB090f37cE",
				"name": "Mars Rise",
				"decimals": 10
			},
			"0xdf07059c2f8baa5d38d8ace3e0aa102cbdf290d2": {
				"ticker": "AKT",
				"address": "0xDF07059c2f8bAA5d38d8AcE3e0Aa102cBDf290D2",
				"name": "Akita BTC",
				"decimals": 11
			},
			"0xd44d8c4a7e208eead3865ed7a24e9d4e80122d97": {
				"ticker": "DB",
				"address": "0xD44D8c4a7E208eeAD3865ed7a24e9d4e80122D97",
				"name": "DiamondBank DAO",
				"decimals": 18
			},
			"0x3c38db6fd1138f8e4b3a16eebcb5d5125f5600c0": {
				"ticker": "STR",
				"address": "0x3c38Db6fd1138f8E4B3a16Eebcb5d5125f5600C0",
				"name": "Star Coin",
				"decimals": 9
			},
			"0xcd6a4fe07c031a16c29a6308342b2bb5e1d0396e": {
				"ticker": "SANG",
				"address": "0xcD6A4FE07C031a16c29A6308342b2Bb5E1D0396e",
				"name": "Super Angel",
				"decimals": 10
			},
			"0x26ac7309d773b09a4b3502d72802d4ddda5bc0f0": {
				"ticker": "DB",
				"address": "0x26Ac7309D773B09a4b3502D72802D4DDDA5bC0F0",
				"name": "DiamondBank DAO",
				"decimals": 18
			},
			"0xe8ffc32000067bc3530b2a7e6412a19d86b0773e": {
				"ticker": "WIN",
				"address": "0xE8ffC32000067bC3530b2a7e6412a19d86b0773E",
				"name": "Winter Storm DAO",
				"decimals": 9
			},
			"0x489b575e7f2551a3f2b11b9707df439a68f6bffc": {
				"ticker": "ANG",
				"address": "0x489B575e7f2551A3F2B11B9707DF439A68f6Bffc",
				"name": "Mini Angel",
				"decimals": 12
			},
			"0xc7acfc22477ed762dcb4ef67ba1023019b9ae9b4": {
				"ticker": "CMP",
				"address": "0xC7ACfc22477eD762dCB4Ef67ba1023019B9AE9B4",
				"name": "Safe Chimp",
				"decimals": 12
			},
			"0xe999a2dfdd262a56d54ac1232b8384878b32930b": {
				"ticker": "EELN",
				"address": "0xe999A2dFdd262a56d54ac1232b8384878B32930B",
				"name": "Exo Elon",
				"decimals": 9
			},
			"0x714c4a4122316f65fabd0a88694d0969150c7c9e": {
				"ticker": "APEA",
				"address": "0x714c4A4122316F65faBd0A88694d0969150C7c9e",
				"name": "Ape AI",
				"decimals": 11
			},
			"0x303045dbf9826745c296cac1ddfcb36c9088a7e8": {
				"ticker": "OG",
				"address": "0x303045dbf9826745c296CAc1DDfcb36c9088a7E8",
				"name": "OG Nodes",
				"decimals": 9
			},
			"0x79222e319c3dd4622c14f85e4e51c45a43ad37db": {
				"ticker": "BULL",
				"address": "0x79222E319c3DD4622c14F85e4e51C45a43ad37dB",
				"name": "Bull Nodes",
				"decimals": 5
			},
			"0x9109f7693c631453db6d2ae88d3b8134ff014bc5": {
				"ticker": "SNIPER",
				"address": "0x9109F7693c631453Db6d2ae88D3b8134ff014bc5",
				"name": "Sniper Games",
				"decimals": 9
			},
			"0x71c7b96d41291458b1b292809270f32c8802b971": {
				"ticker": "SAT",
				"address": "0x71c7b96D41291458B1B292809270f32c8802b971",
				"name": "Satellite Capital",
				"decimals": 9
			},
			"0x04c00828d8dda2c005df322133ccf9bfb6823ff4": {
				"ticker": "AU",
				"address": "0x04c00828d8dDa2C005df322133cCf9bfB6823ff4",
				"name": "AVAX USDT",
				"decimals": 9
			},
			"0x7916b259753ff18721ad59ee0eaed0fc03585c05": {
				"ticker": "PDRG",
				"address": "0x7916b259753ff18721Ad59ee0EAeD0Fc03585c05",
				"name": "Pendragon Protocol PDRG",
				"decimals": 18
			},
			"0x252e4a0ae38a5131ef405de567b6266fa6dd139f": {
				"ticker": "Flamingo",
				"address": "0x252e4A0AE38a5131eF405De567b6266fa6dD139f",
				"name": "Flamingo Financial",
				"decimals": 18
			},
			"0xb781f72d359b5f469169b554941a3b4870888de8": {
				"ticker": "PTCL",
				"address": "0xb781f72d359B5F469169B554941a3B4870888DE8",
				"name": "Particle",
				"decimals": 18
			},
			"0x260bbf5698121eb85e7a74f2e45e16ce762ebe11": {
				"ticker": "UST",
				"address": "0x260Bbf5698121EB85e7a74f2E45E16Ce762EbE11",
				"name": "Axelar Wrapped UST",
				"decimals": 6
			},
			"0xcc2653cffdd100f1f80bc7a12e30681b17cfdbbd": {
				"ticker": "DOGT",
				"address": "0xCc2653CFFdd100f1F80BC7a12E30681b17CFDBBd",
				"name": "Doge Token",
				"decimals": 11
			},
			"0xc672b4b39ecbab767859d4eea8b009f0d5a48137": {
				"ticker": "YUM",
				"address": "0xc672b4B39eCBab767859d4EEa8b009F0D5A48137",
				"name": "Mini Cake",
				"decimals": 12
			},
			"0x2d35660673278c12c530434524063f068480b2ed": {
				"ticker": "Planet",
				"address": "0x2D35660673278c12C530434524063F068480B2ed",
				"name": "Planet Nodes",
				"decimals": 18
			},
			"0xac42f172484f4de5f26959c612e23ddc206998ab": {
				"ticker": "LOCAL",
				"address": "0xAc42F172484f4de5F26959c612e23dDc206998ab",
				"name": "Local Terra Token (Wormhole)",
				"decimals": 6
			},
			"0x455a4a70635798433eb6a0e28db3cf097c94e88a": {
				"ticker": "BET",
				"address": "0x455A4a70635798433eb6a0e28dB3cf097C94e88A",
				"name": "Crypto Sports Betting",
				"decimals": 9
			},
			"0xd79e32796eba62ce68a116fe6f7d7391de1f8f58": {
				"ticker": "AGA",
				"address": "0xd79E32796eba62ce68a116fE6F7d7391dE1F8F58",
				"name": "Agamotto",
				"decimals": 18
			},
			"0x0e336e88d0a309093b697f7af07274041ff61eda": {
				"ticker": "ELON",
				"address": "0x0E336e88d0a309093b697F7af07274041Ff61EDa",
				"name": "Elon AI",
				"decimals": 12
			},
			"0x25adfc79cc8360cdbf323023e5b664e510502404": {
				"ticker": "SAKA",
				"address": "0x25aDFc79Cc8360CDbF323023e5b664E510502404",
				"name": "Safe Akita",
				"decimals": 11
			},
			"0xccbf7c451f81752f7d2237f2c18c371e6e089e69": {
				"ticker": "SDT",
				"address": "0xCCBf7c451F81752F7d2237F2c18C371E6e089E69",
				"name": "Stake DAO Token",
				"decimals": 18
			},
			"0xb00f1ad977a949a3ccc389ca1d1282a2946963b0": {
				"ticker": "BOOFI",
				"address": "0xB00F1ad977a949a3CCc389Ca1D1282A2946963b0",
				"name": "Boo Finance Token",
				"decimals": 18
			},
			"0x0bf6b54855968f1690f9dead00ef8532a3e64b23": {
				"ticker": "ELON",
				"address": "0x0Bf6B54855968F1690f9DEAd00EF8532A3E64b23",
				"name": "Elon Monster",
				"decimals": 11
			},
			"0xef37fa14be107de1ff7284209f011bb0208f7e98": {
				"ticker": "GOSC",
				"address": "0xeF37fA14be107dE1FF7284209f011Bb0208f7e98",
				"name": "Ghost Coin",
				"decimals": 10
			},
			"0x5dd43c5c3e1917fe7548bde3b972690f2e26b61f": {
				"ticker": "BARN",
				"address": "0x5DD43C5C3e1917fe7548BdE3B972690f2E26B61F",
				"name": "Barn Holdings",
				"decimals": 9
			},
			"0xa6b68c0bc7a3a38e3122d0a69ddbc75c3571849b": {
				"ticker": "BOB",
				"address": "0xA6b68c0BC7a3a38e3122D0A69DDBC75c3571849B",
				"name": "BOBs Appreciation Token",
				"decimals": 18
			},
			"0xc816b0598e45781e9ae20337f132d72666db6e54": {
				"ticker": "CASH",
				"address": "0xc816B0598E45781e9aE20337f132d72666DB6e54",
				"name": "Avax Cash",
				"decimals": 9
			},
			"0x4f5293cc21d31d1f3c990d13c7f80afbc2f89b47": {
				"ticker": "PINATA",
				"address": "0x4F5293CC21d31d1F3C990d13c7f80AFBC2F89B47",
				"name": "NFT Management",
				"decimals": 9
			},
			"0xe659c512d8fec73605e677d90b90a0d0189a4b14": {
				"ticker": "CRG",
				"address": "0xE659c512D8Fec73605E677D90B90a0D0189a4b14",
				"name": "Little Corgi",
				"decimals": 11
			},
			"0x8cb188b6ff6eb119ea531598f7352b8ce6405a9b": {
				"ticker": "ZTHN",
				"address": "0x8CB188B6fF6eB119eA531598f7352b8ce6405A9b",
				"name": "Zen Thunder",
				"decimals": 10
			},
			"0x022857055ce3c96adb9964b27d66bd8eedb5817b": {
				"ticker": "POLAR",
				"address": "0x022857055ce3C96aDB9964B27d66bD8eEDB5817b",
				"name": "Polar nodes",
				"decimals": 1
			},
			"0xe79550d65dfa84b3eb5b7cd1aa68ca88ece4ee6f": {
				"ticker": "STAR",
				"address": "0xE79550D65dFA84b3Eb5b7CD1aa68Ca88eCE4ee6F",
				"name": "Star AI",
				"decimals": 10
			},
			"0xb4d1c5637ae19984ec23ff29cdc23562720c35e7": {
				"ticker": "CPTINU",
				"address": "0xB4D1c5637Ae19984Ec23fF29cDc23562720c35e7",
				"name": "Captain Inu",
				"decimals": 9
			},
			"0x9c4da832b8e99a5ecddeeb9a79a7da62f3dc0ae6": {
				"ticker": "DRAGON",
				"address": "0x9C4Da832b8E99a5eCDdEeB9a79A7Da62F3DC0AE6",
				"name": "SpitFire",
				"decimals": 9
			},
			"0x437863ec16a7679e528b6dc247446f2a905edf02": {
				"ticker": "GOL",
				"address": "0x437863EC16a7679e528B6Dc247446f2A905eDf02",
				"name": "Zen Gold",
				"decimals": 10
			},
			"0xd6547beca7fc36d2040fa5330f4663eca0992446": {
				"ticker": "FROST",
				"address": "0xD6547bECA7fC36d2040fa5330f4663ECa0992446",
				"name": "FROST",
				"decimals": 18
			},
			"0x1f1579fc199660118429c2cd0a9c766396f867a2": {
				"ticker": "GHST",
				"address": "0x1F1579Fc199660118429C2Cd0a9C766396f867a2",
				"name": "Exo Ghost",
				"decimals": 12
			},
			"0x9bd566af9c74e0ffdd40d2a154d78928145b3858": {
				"ticker": "TND",
				"address": "0x9BD566af9C74e0fFDD40D2a154d78928145b3858",
				"name": "Thunder Infinity",
				"decimals": 11
			},
			"0x096038627c86150ffa50b6b194b61990e59a6dde": {
				"ticker": "MRSI",
				"address": "0x096038627C86150ffa50b6b194b61990E59a6DDE",
				"name": "Mars Inu",
				"decimals": 10
			},
			"0x37fef78818c0ec4c7260dfa2d62d1a42d9e24887": {
				"ticker": "EAPE",
				"address": "0x37fEf78818c0Ec4C7260dFa2D62D1a42d9E24887",
				"name": "Exo Ape",
				"decimals": 9
			},
			"0x91929e3a2afdd6752d5a96f95fee4767a50f30dd": {
				"ticker": "PSHARE",
				"address": "0x91929E3a2AFdD6752D5A96F95FEe4767a50f30dD",
				"name": "PSHARE",
				"decimals": 18
			},
			"0x736412a610cc662d39665cb0636f66aa3a9e55d0": {
				"ticker": "PABLO",
				"address": "0x736412A610cC662d39665cb0636F66aa3A9e55D0",
				"name": "Medellin City",
				"decimals": 18
			},
			"0xf91d5097a2d665d8fa57795882a6553a3cc6fd24": {
				"ticker": "MOE",
				"address": "0xF91d5097a2d665d8fa57795882A6553A3cc6FD24",
				"name": "Moe",
				"decimals": 18
			},
			"0x5ad7878b59acb2efc4f06392b45b39da1aca1609": {
				"ticker": "BSTR",
				"address": "0x5AD7878b59ACb2EfC4F06392B45B39DA1ACA1609",
				"name": "Baby Star",
				"decimals": 9
			},
			"0x1ecd47ff4d9598f89721a2866bfeb99505a413ed": {
				"ticker": "AVME",
				"address": "0x1ECd47FF4d9598f89721A2866BFEb99505a413Ed",
				"name": "AVME",
				"decimals": 18
			},
			"0x503125b1e9d6aaf7df029a57b620b12a5e1e6121": {
				"ticker": "FLKC",
				"address": "0x503125B1E9D6aaF7DF029a57B620b12a5E1E6121",
				"name": "Floki Cash",
				"decimals": 10
			},
			"0xacbe8342ceff12fa46687ac3583180ba2ebb7440": {
				"ticker": "CASH",
				"address": "0xacbE8342ceFF12fA46687AC3583180ba2ebB7440",
				"name": "CASH",
				"decimals": 9
			},
			"0x90fbe9dfe76f6ef971c7a297641dfa397099a13e": {
				"ticker": "ATL",
				"address": "0x90fBE9dfe76F6EF971c7A297641dfa397099a13e",
				"name": "Atlantis Loans",
				"decimals": 18
			},
			"0x107cdff313aa9aacb37995e5aea677d528cffefe": {
				"ticker": "3QF-D",
				"address": "0x107cDFf313aa9AAcb37995E5AeA677D528cFfEFE",
				"name": "3QF-D",
				"decimals": 18
			},
			"0x50a4b74baa7988019323df8ccdaf86f676019533": {
				"ticker": "QOM",
				"address": "0x50A4B74BAa7988019323Df8CCdAf86F676019533",
				"name": "Shiba Predator",
				"decimals": 18
			},
			"0x959b88966fc5b261df8359961357d34f4ee27b4a": {
				"ticker": "UNIV",
				"address": "0x959b88966fC5B261dF8359961357d34F4ee27b4a",
				"name": "Universe",
				"decimals": 18
			},
			"0xaa1ff216d701eaeed6d5c0765b72980cb2a6f9c2": {
				"ticker": "AKITA",
				"address": "0xaA1FF216d701EAEEd6d5c0765B72980cb2a6f9c2",
				"name": "Super Akita",
				"decimals": 11
			},
			"0xd1409097ac73bdc873f531909639b79f41e5bca6": {
				"ticker": "SNIPE",
				"address": "0xD1409097Ac73BDC873F531909639b79f41E5BCa6",
				"name": "Sniper Nodes",
				"decimals": 9
			},
			"0x53afdd319add3022e52f36377d81dbb532ce8c35": {
				"ticker": "CEASAR",
				"address": "0x53aFDD319ADd3022e52F36377d81DbB532ce8C35",
				"name": "Ape King",
				"decimals": 9
			},
			"0x8cd6aa872abdcc35a3285ada6b7b5de03b4abdf5": {
				"ticker": "AvaxDoge",
				"address": "0x8cd6aa872AbDCC35a3285ada6b7b5dE03b4AbDF5",
				"name": "Avax Doge",
				"decimals": 9
			},
			"0x97e7044ab98b7891546485f5f66edcc2f58be106": {
				"ticker": "EM",
				"address": "0x97E7044ab98B7891546485f5f66EDcc2F58be106",
				"name": "EMERALD",
				"decimals": 18
			},
			"0xee4d163d647a7fe40088fc39d56f0d94e1975766": {
				"ticker": "SSBA",
				"address": "0xEE4D163D647A7Fe40088fc39d56f0d94e1975766",
				"name": "Sushi Shiba",
				"decimals": 10
			},
			"0xdca798fdf19e8c0ff32059e758ed64853ab84e6c": {
				"ticker": "BDMD",
				"address": "0xdCA798fDF19e8c0FF32059e758ED64853Ab84e6c",
				"name": "Black Diamonds",
				"decimals": 9
			},
			"0xdda9dc57da194c8087ef9838d67ec615751fe18d": {
				"ticker": "ROBO",
				"address": "0xDDA9DC57DA194C8087EF9838D67EC615751Fe18d",
				"name": "AI Learning Protocol",
				"decimals": 9
			},
			"0xf4c97cb5bc62aa2b886630518dd1c16c0e3ff96f": {
				"ticker": "MetaTokenDAO",
				"address": "0xF4C97CB5BC62aa2b886630518dD1C16C0e3FF96F",
				"name": "Meta Token DAO",
				"decimals": 18
			},
			"0x56da6bf2fc95acb19ee5fe435fbdfa85b020a470": {
				"ticker": "SXT",
				"address": "0x56dA6bF2FC95ACB19eE5fE435fbDfA85b020A470",
				"name": "Secret Treasure",
				"decimals": 9
			},
			"0xbf1230bb63bfd7f5d628ab7b543bcefa8a24b81b": {
				"ticker": "CHRO",
				"address": "0xbf1230bb63bfD7F5D628AB7B543Bcefa8a24B81B",
				"name": "Chronicum",
				"decimals": 18
			},
			"0x64694fc8dfca286bf1a15b0903fac98217dc3ad7": {
				"ticker": "PGL",
				"address": "0x64694FC8dFCA286bF1A15b0903FAC98217dC3AD7",
				"name": "Pangolin Liquidity",
				"decimals": 18
			},
			"0xa340784a78cd9e3031b5ee789c977d4ba6c45c6e": {
				"ticker": "GLADIATOR",
				"address": "0xA340784A78CD9e3031B5EE789c977d4ba6C45C6E",
				"name": "GladiatorDAO",
				"decimals": 18
			},
			"0x4ee0e8fb28d3d384972ffdc266fe1b3838a1e2f3": {
				"ticker": "SUN",
				"address": "0x4ee0e8FB28D3D384972FFDC266FE1b3838a1E2F3",
				"name": "Aztec Nodes",
				"decimals": 18
			},
			"0x372c307425d92f165fafd6a2049015168d7ca560": {
				"ticker": "VXL",
				"address": "0x372c307425D92F165FAFD6A2049015168d7CA560",
				"name": "Voxel X Network",
				"decimals": 18
			},
			"0x9763bf82c49e295d0e32cb04e9f94040745d1f74": {
				"ticker": "OMBT",
				"address": "0x9763bF82c49E295d0E32Cb04e9f94040745d1f74",
				"name": "On My Block Token",
				"decimals": 18
			},
			"0xbcb4ed66bcf210326debd4ab5871e5ecbb7797e7": {
				"ticker": "NJA",
				"address": "0xBcB4eD66Bcf210326deBD4ab5871e5ecbb7797e7",
				"name": "Ninja Swap",
				"decimals": 9
			},
			"0x9b60a5ec982dea84140bc0a032a61c6a2eed9efc": {
				"ticker": "NFTDA",
				"address": "0x9b60a5Ec982dea84140bc0A032A61C6a2eeD9eFc",
				"name": "NFT DAO",
				"decimals": 18
			},
			"0x4abbc3275f8419685657c2dd69b8ca2e26f23f8e": {
				"ticker": "💎",
				"address": "0x4aBBc3275f8419685657C2DD69b8ca2e26F23F8E",
				"name": "diamondtoken.world 💎",
				"decimals": 9
			},
			"0x57789be2ee763774ba752167ceeb96c3d7f3fbce": {
				"ticker": "CGRM",
				"address": "0x57789BE2eE763774bA752167CEeb96c3D7F3FbCE",
				"name": "Cryptogram",
				"decimals": 9
			},
			"0x0ef6d15da2405c1c09b4f21a1d9a6778c122c221": {
				"ticker": "IMP",
				"address": "0x0ef6D15Da2405c1c09B4F21a1D9a6778c122C221",
				"name": "IMPERIAL NODE",
				"decimals": 18
			},
			"0x4a87c7850e10feafb310a76b852c812fb1e5d5b4": {
				"ticker": "ELN",
				"address": "0x4a87c7850e10FeAfB310A76B852C812fb1E5D5b4",
				"name": "Baby Elon",
				"decimals": 11
			},
			"0x0e5d3ee379fb33f6fab5c2773fdc8f9593d020db": {
				"ticker": "ELON",
				"address": "0x0E5d3Ee379FB33f6fAB5c2773FDC8f9593d020Db",
				"name": "Elon Swap",
				"decimals": 12
			},
			"0x932bc2933d1782ecbe1b8ce03b7fe3867fd72056": {
				"ticker": "DGT",
				"address": "0x932bC2933d1782ECBe1b8cE03B7fe3867fD72056",
				"name": "Degis Token",
				"decimals": 18
			},
			"0x8dc2e51eb27b0a5bc2ceab298ebc7e028488d677": {
				"ticker": "NOBEL",
				"address": "0x8Dc2E51eB27B0A5Bc2CeaB298Ebc7E028488D677",
				"name": "Nobelium DAO",
				"decimals": 1
			},
			"0xfe38a3b6f7949a70ab339e69c494ed75602079e5": {
				"ticker": "ELON",
				"address": "0xFe38A3b6F7949a70aB339e69c494ed75602079e5",
				"name": "Safe Elon",
				"decimals": 9
			},
			"0xcf7101f34dba0f3d5ffafd3d3aa2b3fc20c08775": {
				"ticker": "AVAO",
				"address": "0xCF7101f34DBA0f3d5fFafD3D3Aa2b3Fc20C08775",
				"name": "AvaOne",
				"decimals": 18
			},
			"0xbe3f699c1c0501e1af0baf4f82248856da9675e3": {
				"ticker": "PIN",
				"address": "0xbE3F699c1c0501E1aF0BAf4f82248856Da9675E3",
				"name": "Pineapple Block",
				"decimals": 11
			},
			"0x65489ad800a8d0e30187dbeb1a0e01313b0120af": {
				"ticker": "GAME",
				"address": "0x65489aD800A8D0E30187dbeB1a0E01313B0120AF",
				"name": "GameDAO",
				"decimals": 8
			},
			"0x906b1380a15d1bbf779c97eaddb13bae53c4d522": {
				"ticker": "FITFI",
				"address": "0x906b1380A15d1Bbf779c97eaDdb13bAE53c4d522",
				"name": "STEP.APP",
				"decimals": 18
			},
			"0x15dbfcb2ebf1324d4c460609b6a4d3be153bad97": {
				"ticker": "SOUL",
				"address": "0x15DBFCB2ebf1324D4c460609B6A4d3bE153BAD97",
				"name": "Soul",
				"decimals": 18
			},
			"0x49ae4986177001660bba867d77fb1229b6b618ea": {
				"ticker": "TOM",
				"address": "0x49AE4986177001660BBa867D77FB1229B6B618eA",
				"name": "Safe Fantom",
				"decimals": 9
			},
			"0xea2b203617fd613278915c2012fd4b32ad5cd7bb": {
				"ticker": "MUSK",
				"address": "0xEA2B203617FD613278915C2012fD4b32ad5Cd7bb",
				"name": "ELON",
				"decimals": 9
			},
			"0x21a6945c3b931c28db639ffc1fc0810104da7e5c": {
				"ticker": "DB",
				"address": "0x21a6945C3b931c28db639FFc1FC0810104Da7e5C",
				"name": "DiamondBank",
				"decimals": 18
			},
			"0xe9067be19c824d7ad82e35333152eb0c593ab454": {
				"ticker": "ASHARE",
				"address": "0xe9067be19c824d7ad82e35333152EB0c593AB454",
				"name": "ASHARES",
				"decimals": 18
			},
			"0x05af3a9ea35c411fe76574586edb5fdb4e439593": {
				"ticker": "AngelNode",
				"address": "0x05Af3A9ea35C411fE76574586EdB5FdB4e439593",
				"name": "Angel Node",
				"decimals": 18
			},
			"0xd47b37f49d1c5a9fe94d9daa64ead9d6f236f501": {
				"ticker": "BATTLE",
				"address": "0xd47b37f49D1c5a9FE94d9daA64EAD9d6F236f501",
				"name": "Battle Nodes - Gain Nodes Daily",
				"decimals": 9
			},
			"0x246405c5adcb8f2004840c7d073190e5119d56ee": {
				"ticker": "UZI",
				"address": "0x246405c5AdCb8f2004840c7d073190E5119d56eE",
				"name": "Uzi Inu",
				"decimals": 9
			},
			"0x6c7d3d94d9fafea24c9f7fabcfb0661a9b67193d": {
				"ticker": "Nodeon",
				"address": "0x6C7d3d94D9fafea24C9F7fAbcFB0661A9b67193d",
				"name": "Nodeon Protocol",
				"decimals": 18
			},
			"0x787ddb7eaef88878c1f77d48177ecc96a8ddfe4a": {
				"ticker": "DB",
				"address": "0x787dDb7eAEF88878C1f77D48177ecC96a8DDFe4a",
				"name": "DiamondBank",
				"decimals": 18
			},
			"0x9d6bd36e6356dd2c84ca79fc3083590644b431ca": {
				"ticker": "NBA",
				"address": "0x9d6Bd36E6356Dd2C84ca79FC3083590644b431CA",
				"name": "Nebula",
				"decimals": 9
			},
			"0xe108a2d6a133e5976e2f39730529f1a22fb5c0c8": {
				"ticker": "NOVATO",
				"address": "0xE108a2D6a133E5976e2f39730529F1A22Fb5C0c8",
				"name": "Novato Financial",
				"decimals": 18
			},
			"0xfa9a80cc465f37c7889b3e08d7b7cb19c9658260": {
				"ticker": "Dionys",
				"address": "0xfA9a80cC465F37C7889B3e08d7b7Cb19c9658260",
				"name": "Dionys DAO",
				"decimals": 18
			},
			"0xc125395ced79f18a32a0e2eacca6681b643e27db": {
				"ticker": "BANG",
				"address": "0xC125395cED79F18A32a0E2eaccA6681b643e27Db",
				"name": "Bang Bucks",
				"decimals": 9
			},
			"0x78c42324016cd91d1827924711563fb66e33a83a": {
				"ticker": "RELAY",
				"address": "0x78c42324016cd91D1827924711563fb66E33A83A",
				"name": "Relay Token",
				"decimals": 18
			},
			"0xd1816ebf59a23520382276cf94ee9c7e70d33ae9": {
				"ticker": "GETH",
				"address": "0xd1816EBf59A23520382276cf94Ee9C7e70d33ae9",
				"name": "Green Earth Token",
				"decimals": 9
			},
			"0xddb4cc473219851ad7e8be26425f0ad259ae81bb": {
				"ticker": "AKITA",
				"address": "0xDDB4CC473219851Ad7e8be26425F0aD259ae81bb",
				"name": "Baby Akita",
				"decimals": 12
			},
			"0xf4ef54730f45b6053c9eb31f0e1e566e8e98f2f5": {
				"ticker": "DB",
				"address": "0xF4eF54730F45b6053c9eB31F0E1E566e8E98f2F5",
				"name": "DiamondBank",
				"decimals": 18
			},
			"0x6810d9443a2a73601de550fcb4f79954ff23850e": {
				"ticker": "KSTR",
				"address": "0x6810d9443A2a73601dE550fCB4F79954Ff23850e",
				"name": "King Star",
				"decimals": 10
			},
			"0xa8371d6fe745a7707293baa5bc7060be283cd184": {
				"ticker": "DD",
				"address": "0xa8371d6fe745A7707293bAa5bC7060Be283Cd184",
				"name": "Degen",
				"decimals": 9
			},
			"0xc15e49693f15035a880bf177b95b1c04a5eb2b79": {
				"ticker": "TNDN",
				"address": "0xC15E49693f15035A880BF177b95b1c04a5Eb2B79",
				"name": "Thunder Network",
				"decimals": 12
			},
			"0x8831ca76b41fedd037815e6daa6a6f21b69be881": {
				"ticker": "CPTINU",
				"address": "0x8831CA76b41FEDd037815e6dAa6A6F21b69Be881",
				"name": "Captain INU",
				"decimals": 9
			},
			"0xf15609c283403123271cb266fc2ada15a1603879": {
				"ticker": "SPRT",
				"address": "0xF15609C283403123271Cb266FC2adA15A1603879",
				"name": "Spirit",
				"decimals": 18
			},
			"0xe50ca947fcfcf4a9a439611919698c0279ea9d5b": {
				"ticker": "OTEN",
				"address": "0xE50ca947FCfCf4A9a439611919698C0279Ea9d5B",
				"name": "OTEN DAO",
				"decimals": 18
			},
			"0x4d5bf8cf0e2b3484ac8e92715cc7fee45555514d": {
				"ticker": "LUCKY",
				"address": "0x4d5bF8cf0E2b3484Ac8E92715CC7fEe45555514D",
				"name": "Lucky DAO",
				"decimals": 1
			},
			"0x44a45a9baeb63c6ea4860ecf9ac5732c330c4d4e": {
				"ticker": "MEAD",
				"address": "0x44a45a9BaEb63c6ea4860ecf9ac5732c330C4d4E",
				"name": "Thors Mead V2",
				"decimals": 9
			},
			"0x7b13c1743517329b9f7f9aef56e6849034ed8f92": {
				"ticker": "POW",
				"address": "0x7B13c1743517329b9f7F9AEf56e6849034eD8F92",
				"name": "POWER",
				"decimals": 9
			},
			"0xe7e3eeb9b8e9a7802e151964de2eeb0d3ae455b4": {
				"ticker": "HCH",
				"address": "0xE7e3Eeb9B8e9a7802E151964De2eEB0D3ae455b4",
				"name": "Little Hachi",
				"decimals": 11
			},
			"0x0690d50bc9f8626275fe3fa4fca4d60d66205ff3": {
				"ticker": "DGEB",
				"address": "0x0690d50BC9F8626275fE3fa4fca4d60d66205ff3",
				"name": "Doge BTC",
				"decimals": 12
			},
			"0xa5e2cfe48fe8c4abd682ca2b10fcaafe34b8774c": {
				"ticker": "PSHARE",
				"address": "0xA5e2cFe48fe8C4ABD682CA2B10fCAaFE34b8774c",
				"name": "PSHARE",
				"decimals": 18
			},
			"0x6835118a649bc180dfb57921279690c5f3804541": {
				"ticker": "MCAP",
				"address": "0x6835118A649BC180DfB57921279690C5F3804541",
				"name": "Mac Capital",
				"decimals": 9
			},
			"0xdc144a73f147d65013ae88f1c4a00e2c19508c7a": {
				"ticker": "STAX",
				"address": "0xdC144A73F147d65013Ae88F1C4a00E2C19508C7a",
				"name": "Moon Stax",
				"decimals": 9
			},
			"0x8465be275b04115b8ad10c300f3c6d465c6fe267": {
				"ticker": "TUS",
				"address": "0x8465Be275b04115b8AD10c300F3C6D465c6FE267",
				"name": "Treasure Under Sea 0xf693248F96Fe03422FEa95aC0aFbBBc4a8FdD172",
				"decimals": 18
			},
			"0xf09857b533c5937969e3dea0efa4feea231a8803": {
				"ticker": "MED",
				"address": "0xf09857b533c5937969E3deA0EFa4FeEA231a8803",
				"name": "MEDUSA",
				"decimals": 18
			},
			"0xbb50481001c1cab110515c04df21327222e70263": {
				"ticker": "APE",
				"address": "0xBb50481001C1CAB110515C04df21327222E70263",
				"name": "Little Ape",
				"decimals": 10
			},
			"0x853cd7dc5ecf76d3a541dbfe7f161230d35fa18b": {
				"ticker": "RYV",
				"address": "0x853CD7dC5ECf76d3A541dbfE7f161230D35Fa18b",
				"name": "Ryvin",
				"decimals": 18
			},
			"0xa4b84262a2de0b32977faac30c304aded8e70a45": {
				"ticker": "Yuzu",
				"address": "0xa4b84262a2dE0b32977FaAc30c304aDEd8e70A45",
				"name": "Yuzu Financial",
				"decimals": 18
			},
			"0x376dfd131bae803f55f0fca0cfbb71cd800f1a74": {
				"ticker": "PABLO",
				"address": "0x376dFD131baE803F55f0fcA0cFbB71CD800f1A74",
				"name": "Medellin City",
				"decimals": 18
			},
			"0xa425fd6aac88842daad7c93e7dfe38ef746fdefb": {
				"ticker": "FKIC",
				"address": "0xa425Fd6AAc88842DaaD7C93e7dfE38EF746FDefb",
				"name": "Floki Cash",
				"decimals": 9
			},
			"0x89eed64a77dc4a4f5c896a1411a9b883ef26074e": {
				"ticker": "COS",
				"address": "0x89eEd64a77DC4a4f5C896A1411a9B883eF26074E",
				"name": "Cosmos",
				"decimals": 8
			},
			"0x658d4046d0b8aed3311d00f46421172865c51b3d": {
				"ticker": "SHOP",
				"address": "0x658D4046d0b8aED3311d00F46421172865c51b3D",
				"name": "Shopify Payment Processor",
				"decimals": 9
			},
			"0x8e4e33ac6f571606f6b1a6e3ca2f6528e2c0b29d": {
				"ticker": "ELNS",
				"address": "0x8e4e33ac6F571606F6b1a6e3CA2f6528E2c0b29d",
				"name": "Elon SV",
				"decimals": 10
			},
			"0x736defe199df07b494bd936ce4cbf06cb2625e66": {
				"ticker": "LUCK",
				"address": "0x736DEfE199dF07B494bd936ce4cbF06cB2625e66",
				"name": "Lucky Nodes",
				"decimals": 18
			},
			"0xf031f0b40cae62d4416f3a393c56c923b3baa264": {
				"ticker": "WAR",
				"address": "0xF031f0B40CAE62D4416F3a393c56c923B3BAa264",
				"name": "War Games",
				"decimals": 9
			},
			"0x4faa0e9e40db5c74d2fd20d703d574820ebca4b1": {
				"ticker": "GCH",
				"address": "0x4faA0E9E40DB5C74d2fD20D703d574820EBcA4B1",
				"name": "King Glitch",
				"decimals": 11
			},
			"0x5fdb1b0c730f0a347257bc9bdbe760498f97fb9e": {
				"ticker": "MARS",
				"address": "0x5fDb1B0C730f0A347257bc9bdbe760498f97Fb9e",
				"name": "Sushi Mars",
				"decimals": 10
			},
			"0xbcf7dd079bc0a52981ec3c2b25fb8b86d0cdc41f": {
				"ticker": "NOBEL",
				"address": "0xbcF7DD079BC0a52981ec3C2b25fB8B86D0CDc41f",
				"name": "Nobelium DAO",
				"decimals": 18
			},
			"0x6c1c0319d8ddcb0ffe1a68c5b3829fd361587db4": {
				"ticker": "POLAR",
				"address": "0x6C1c0319d8dDcb0ffE1a68C5b3829Fd361587DB4",
				"name": "POLAR",
				"decimals": 18
			},
			"0x78fe2102a8a96e62b676f94727a7c0da2efc485e": {
				"ticker": "FTD",
				"address": "0x78fe2102a8A96E62B676F94727A7c0DA2Efc485e",
				"name": "FortressDAO",
				"decimals": 18
			},
			"0x91f0f44b184c1518bae8ab9cae6843f4b3394e43": {
				"ticker": "APE",
				"address": "0x91F0F44b184c1518bae8Ab9CaE6843f4B3394E43",
				"name": "Ape X",
				"decimals": 11
			},
			"0x346a59146b9b4a77100d369a3d18e8007a9f46a6": {
				"ticker": "AVAI",
				"address": "0x346A59146b9b4a77100D369a3d18E8007A9F46a6",
				"name": "AVAI",
				"decimals": 18
			},
			"0x3b39f1327cc2a36c6bbbd2c294ce4d3d6d5abfb3": {
				"ticker": "GOL",
				"address": "0x3B39f1327CC2a36C6BBbd2C294cE4D3D6d5ABFb3",
				"name": "Gold Classic",
				"decimals": 12
			},
			"0x2ff8d56c7a6887729fdb2f7d879d58fdec958a9e": {
				"ticker": "GHST",
				"address": "0x2ff8d56c7a6887729fDB2f7d879D58fDeC958a9E",
				"name": "OMG Ghost",
				"decimals": 11
			},
			"0x9b64faa7b3096cd5a3d8ec22655d50263e26fbd5": {
				"ticker": "SATOSHI",
				"address": "0x9b64faA7b3096cD5A3D8EC22655d50263e26FbD5",
				"name": "Satoshi Nodes",
				"decimals": 9
			},
			"0xf945e994d8b42157cc3806eaed6dc85bb10dabd0": {
				"ticker": "LUCKY",
				"address": "0xF945e994D8B42157CC3806eAED6dC85bB10DaBd0",
				"name": "LUCKY",
				"decimals": 9
			},
			"0x81ef20506ab75e6167cb026ee411a8b7da281564": {
				"ticker": "AVOSB",
				"address": "0x81ef20506Ab75E6167Cb026eE411a8B7da281564",
				"name": "AVOSB",
				"decimals": 18
			},
			"0xf3d20c3efc6978a3444b6b4226985e66e7bc1d14": {
				"ticker": "GUN",
				"address": "0xF3d20C3EFC6978A3444b6b4226985e66E7BC1d14",
				"name": "Gun Nodes",
				"decimals": 9
			},
			"0x70d1fdff718c9795bde6d971de884ac7cca83bc6": {
				"ticker": "GHST",
				"address": "0x70d1Fdff718C9795bde6d971De884AC7Cca83BC6",
				"name": "Ghost NET",
				"decimals": 11
			},
			"0xcf29dd409aadfb3362521ba83f1326724f411a7d": {
				"ticker": "CHAC",
				"address": "0xcF29DD409AadFB3362521ba83f1326724F411A7d",
				"name": "Chapo Capital",
				"decimals": 9
			},
			"0xddea5d7269318fdf72d901581ee681e0d250f069": {
				"ticker": "EA",
				"address": "0xDdeA5d7269318FDf72D901581EE681E0d250F069",
				"name": "EA Game Studio",
				"decimals": 9
			},
			"0x2ff0f51aee45058de5ddc753e180e6259e555c70": {
				"ticker": "APEC",
				"address": "0x2fF0F51AeE45058de5dDC753E180E6259e555C70",
				"name": "Ape Classic",
				"decimals": 11
			},
			"0x840a0488b00af4bd51374d5983f361fe6c39513e": {
				"ticker": "SPLASH",
				"address": "0x840a0488b00AF4Bd51374D5983F361Fe6C39513E",
				"name": "Splash Games",
				"decimals": 9
			},
			"0xe9e8d6b6ce6d94fc9d724711e80784ec096949fc": {
				"ticker": "JLP",
				"address": "0xe9E8d6b6ce6D94Fc9d724711e80784Ec096949Fc",
				"name": "Joe LP Token",
				"decimals": 18
			},
			"0x7ce01d92625429fcc685113b3d65b622dc625c00": {
				"ticker": "PNAPL",
				"address": "0x7cE01d92625429fcc685113B3d65B622Dc625c00",
				"name": "Pineapple Protocol",
				"decimals": 9
			},
			"0xe93dd358182c2cad5371130d51dfa3b5c9302fad": {
				"ticker": "SPLASH",
				"address": "0xe93dD358182C2cAd5371130D51dfa3b5C9302Fad",
				"name": "Splash Games",
				"decimals": 9
			},
			"0xd69e3cbc243f737534a5bc060d19793769769bcc": {
				"ticker": "CAKE",
				"address": "0xD69E3cBC243F737534a5Bc060D19793769769BCc",
				"name": "Cake Markets",
				"decimals": 12
			},
			"0x66040549ff706c195b842b5c625df642b4919eb7": {
				"ticker": "SPK",
				"address": "0x66040549ff706c195b842B5c625DF642b4919eb7",
				"name": "Screenplay Kill",
				"decimals": 18
			},
			"0xd5edb15c7b41c180860bc961baf3fc4b66bf200b": {
				"ticker": "SNOW",
				"address": "0xD5EDB15C7b41C180860BC961BAF3FC4B66BF200B",
				"name": "Blizzard",
				"decimals": 9
			},
			"0x47aea8c54b82c64206ca8a45ede0cd2207a78d3e": {
				"ticker": "XAvax",
				"address": "0x47aEA8C54B82c64206cA8a45ede0cD2207A78d3e",
				"name": "Pendragon Protocol XAvax",
				"decimals": 18
			},
			"0x96fcacaf80a56e1f56124f7d765e928754e51a27": {
				"ticker": "$HASH",
				"address": "0x96fcaCaF80a56E1F56124f7d765e928754E51a27",
				"name": "Hash",
				"decimals": 18
			},
			"0x6a0e4ee6506c83c1bc3073fe9463e70b379680fb": {
				"ticker": "APE",
				"address": "0x6a0E4ee6506C83C1Bc3073Fe9463E70B379680fB",
				"name": "APE Nodes (20 APE/Node)",
				"decimals": 9
			},
			"0x60dfeac9d2a7b76c8a800ed5965cbfae3aaead27": {
				"ticker": "RENT",
				"address": "0x60DfEAC9d2a7b76C8A800ED5965CBFaE3aAead27",
				"name": "Renter.Cafe",
				"decimals": 18
			},
			"0x3a8d7d551b3c369c6cffd5c7267b518b134d65e8": {
				"ticker": "WINGS",
				"address": "0x3A8D7D551b3C369c6CfFd5C7267B518b134D65e8",
				"name": "Angel Rise",
				"decimals": 11
			},
			"0x6b6c55e0cbbf8564e1aeaf235b835ae0beb11dfe": {
				"ticker": "PXT",
				"address": "0x6b6C55e0cBBf8564e1AeAF235b835Ae0bEB11dfE",
				"name": "ProjectXNodes",
				"decimals": 18
			},
			"0xfcb3a43b6b5466105a318fd58e87728ad5658c7d": {
				"ticker": "NEIBR",
				"address": "0xfCB3a43B6b5466105A318fD58e87728ad5658c7d",
				"name": "TheNeighbours",
				"decimals": 9
			},
			"0x487e5c3732f6b8b8079c78a0f84d38196b76de5d": {
				"ticker": "SPLASH",
				"address": "0x487e5C3732f6B8b8079C78A0F84D38196B76De5d",
				"name": "Splash Game",
				"decimals": 9
			},
			"0x3f7a995423dfa6f6585349a7ec67d53c63f39df3": {
				"ticker": "DOG",
				"address": "0x3F7A995423DFa6f6585349a7Ec67d53C63F39DF3",
				"name": "DogNodes",
				"decimals": 18
			},
			"0xc36fd4476eab942125f6a85c2048da5908b9e536": {
				"ticker": "TOMD",
				"address": "0xC36fD4476EAB942125f6A85C2048Da5908B9E536",
				"name": "Fantom DAO",
				"decimals": 10
			},
			"0xf3cdbb017d5b882dc1256a536d488130ff95220f": {
				"ticker": "HCIN",
				"address": "0xf3cdbB017d5b882Dc1256A536D488130fF95220F",
				"name": "Hachi NET",
				"decimals": 11
			},
			"0x0742778cd9f4a0637c839fb24e848f1fa31fc24d": {
				"ticker": "ANG",
				"address": "0x0742778Cd9F4a0637C839FB24e848F1Fa31FC24d",
				"name": "OMG Angel",
				"decimals": 9
			},
			"0x090fe6c150d2e81e503c1520ea7d8a0d571a0229": {
				"ticker": "NXTGEN",
				"address": "0x090Fe6c150d2e81e503C1520Ea7d8a0d571a0229",
				"name": "Next Gen Nodes",
				"decimals": 9
			},
			"0x83c93abef3f0cc0210d09f62d5a54332c1803625": {
				"ticker": "ZEUS",
				"address": "0x83c93ABEf3F0cC0210d09f62d5a54332C1803625",
				"name": "Zeus v6",
				"decimals": 18
			},
			"0x6cc27c639fb1c66eaa74ae81b53d5f6ac432d64a": {
				"ticker": "PPC",
				"address": "0x6Cc27C639Fb1c66EAA74aE81b53D5F6Ac432d64A",
				"name": "Pepe Capital",
				"decimals": 9
			},
			"0xc250ff35ddae8868fb1c33b07112c7b66eb63247": {
				"ticker": "SKADE",
				"address": "0xC250Ff35dDae8868fB1c33b07112C7B66Eb63247",
				"name": "SkadiDAO",
				"decimals": 9
			},
			"0x8a208f04af8ace644e7381fcdbe406735940983c": {
				"ticker": "WINGS",
				"address": "0x8A208F04Af8ACe644e7381Fcdbe406735940983c",
				"name": "King Angel",
				"decimals": 9
			},
			"0x7d86f1eaff29f076576b2ff09ce3bcc7533fd2c5": {
				"ticker": "UNCL",
				"address": "0x7D86F1eafF29F076576b2Ff09CE3bcC7533fD2C5",
				"name": "UNCL",
				"decimals": 18
			},
			"0x0add05dc5ef7c622ced36af14cdf1ca360aeae86": {
				"ticker": "NART",
				"address": "0x0adD05dC5eF7c622CeD36aF14cDF1ca360AEae86",
				"name": "DarkForest",
				"decimals": 18
			},
			"0x2534a44756f9e445685f0eaf85fc9d71289ee27b": {
				"ticker": "ASTRO",
				"address": "0x2534A44756f9e445685f0EAF85fc9d71289eE27B",
				"name": "Astro",
				"decimals": 18
			},
			"0xd3ba51632db7cb76f2273614ef5dcfbb53677365": {
				"ticker": "BILB",
				"address": "0xD3bA51632db7cB76F2273614eF5DCFbb53677365",
				"name": "Billionaire Bunker",
				"decimals": 9
			},
			"0xad4a51ace0ae87c7e08fc58566630a367caa2691": {
				"ticker": "EGG",
				"address": "0xAD4A51aCE0Ae87c7E08FC58566630a367CaA2691",
				"name": "Egg Token",
				"decimals": 18
			},
			"0x0fa6072bfbfe1169bdb4ebf2c821f50dbcf97914": {
				"ticker": "AKTL",
				"address": "0x0Fa6072Bfbfe1169bdB4eBF2c821F50DbCf97914",
				"name": "Akita Ledger",
				"decimals": 9
			},
			"0x4945c9481485bf10b08e51e8aadaf86b11b841a6": {
				"ticker": "YETI",
				"address": "0x4945C9481485BF10B08E51E8AaDAF86B11b841A6",
				"name": "Yeti World NFT",
				"decimals": 9
			},
			"0x01771356e9db6c1c2c9301c93bd595e74966db77": {
				"ticker": "GLD",
				"address": "0x01771356e9dB6c1C2c9301C93bd595E74966Db77",
				"name": "Gold AVA",
				"decimals": 9
			},
			"0x7654d593d386fe43b3306b65601458b2542d55ed": {
				"ticker": "AVALPSDAO",
				"address": "0x7654D593d386fe43B3306B65601458b2542d55eD",
				"name": "AvalpsDAO",
				"decimals": 9
			},
			"0xd17584633bc8d190e5a14502976dad9640456d6d": {
				"ticker": "TNGL",
				"address": "0xD17584633bc8D190E5A14502976daD9640456D6d",
				"name": "Tangle",
				"decimals": 9
			},
			"0xd766bb3281c40fe3ccd5f770f141ec8bbe0e14ba": {
				"ticker": "DOGE",
				"address": "0xD766bb3281C40Fe3cCD5f770F141EC8bBe0e14BA",
				"name": "Doge AVA",
				"decimals": 9
			},
			"0xffe560d06fbfa40a2482945a00cd185bdbe96da6": {
				"ticker": "XASC",
				"address": "0xfFe560D06fBfA40A2482945A00Cd185bDBe96dA6",
				"name": "XASC",
				"decimals": 18
			},
			"0x02fe8adc1b58a63ccb1bb8c39930dcedd7cef4db": {
				"ticker": "VOID",
				"address": "0x02fE8adC1B58A63CCb1Bb8c39930DCEdD7cEF4dB",
				"name": "https://twitter.com/AvalancheVoid",
				"decimals": 18
			},
			"0xf266f18f60a99509f010473ee84ac8de214415b2": {
				"ticker": "QAPL",
				"address": "0xf266f18f60a99509f010473eE84ac8dE214415B2",
				"name": "Queen Pineapple",
				"decimals": 9
			},
			"0xe9f9d520b707c7757b62efadf85a97537f8a6045": {
				"ticker": "STE",
				"address": "0xE9F9D520b707c7757b62eFADf85a97537f8a6045",
				"name": "Strong Token EVer ",
				"decimals": 18
			},
			"0xb9f9d54520cd9f7f788d6e4552b672d1f8f30d38": {
				"ticker": "CAKE",
				"address": "0xb9F9D54520Cd9F7f788D6E4552B672d1f8F30d38",
				"name": "Cake Starter",
				"decimals": 10
			},
			"0xbd3936ec8d83a5d4e73eca625ecfa006da8c8f52": {
				"ticker": "URQA",
				"address": "0xbd3936eC8D83A5D4e73ecA625eCFa006da8C8F52",
				"name": "UREEQA Token [via ChainPort.io]",
				"decimals": 18
			},
			"0x8f4c50c6f1ea7fcc20bf17ef86f0e05d910f49f6": {
				"ticker": "VOLT",
				"address": "0x8F4c50c6F1ea7Fcc20Bf17EF86f0E05D910f49F6",
				"name": "AsgardDao",
				"decimals": 18
			},
			"0xe8bd3fb2e2966c90fb316e2145957fecc3f875c5": {
				"ticker": "BabyJoe",
				"address": "0xe8bd3fb2E2966c90FB316e2145957FeCc3F875C5",
				"name": "BabyJoe",
				"decimals": 18
			},
			"0x58286594ba08994823ef6d4368bfcf06b049ef68": {
				"ticker": "MGOL",
				"address": "0x58286594BA08994823eF6D4368bFCf06b049Ef68",
				"name": "Mini Gold",
				"decimals": 11
			},
			"0x95b069f5822ba2e4d69ddb368713e1d2fe7f748a": {
				"ticker": "PIGGY",
				"address": "0x95b069f5822Ba2e4d69DdB368713E1d2fE7f748a",
				"name": "Piggy",
				"decimals": 18
			},
			"0x1f3cb33517cdf379d687cbed2fd60f9655809f8a": {
				"ticker": "GLTCH",
				"address": "0x1F3CB33517CdF379D687cbed2Fd60F9655809F8a",
				"name": "Glitch Dapp",
				"decimals": 12
			},
			"0x87d0b9b95adc077987dc40d81c1e834a6c7d7554": {
				"ticker": "APL",
				"address": "0x87D0B9b95Adc077987DC40D81C1E834A6c7d7554",
				"name": "Project Pineapple",
				"decimals": 10
			},
			"0x41160139986dffe2011a07fbb5f4b316200a5419": {
				"ticker": "CSHARE",
				"address": "0x41160139986dFfE2011a07fbB5F4B316200A5419",
				"name": "CFNSHARE",
				"decimals": 18
			},
			"0xc0797553dbb0a9ba708232293bcb94132a499804": {
				"ticker": "MC2",
				"address": "0xc0797553DBB0a9BA708232293bcB94132A499804",
				"name": "mes couilles 2",
				"decimals": 18
			},
			"0x3711c397b6c8f7173391361e27e67d72f252caad": {
				"ticker": "COM",
				"address": "0x3711c397B6c8F7173391361e27e67d72F252cAad",
				"name": "COMPLUS",
				"decimals": 18
			},
			"0xcef914747c4303ae35ae7aa0ea101b069f027d30": {
				"ticker": "GLDS",
				"address": "0xcEF914747C4303ae35AE7aa0ea101B069F027d30",
				"name": "Gold SV",
				"decimals": 11
			},
			"0xe05fe0d5c9be72bbe2a19d225d2a99620a603894": {
				"ticker": "SYM",
				"address": "0xE05FE0D5c9be72BBe2a19d225D2A99620a603894",
				"name": "SymplicismDAO",
				"decimals": 5
			},
			"0x790159fe86737ce5662d434ebd43871449d06eae": {
				"ticker": "ISAX",
				"address": "0x790159fE86737CE5662D434Ebd43871449d06Eae",
				"name": "IS AVAX",
				"decimals": 18
			},
			"0xb565be4048816137037dbd544654a687cbd8a0d6": {
				"ticker": "ACULT",
				"address": "0xB565be4048816137037DBD544654A687CBD8A0d6",
				"name": "AVAX CULT",
				"decimals": 18
			},
			"0xd11f3f0ca105aad0566ba386c5b56feaca840898": {
				"ticker": "VALDAO",
				"address": "0xD11f3f0CA105aaD0566BA386C5B56FEaCa840898",
				"name": "ValdaoTokenV2",
				"decimals": 18
			},
			"0x8afa66861125574e116623c68c06d191cf815910": {
				"ticker": "KNIGHT",
				"address": "0x8AFA66861125574E116623C68C06D191cF815910",
				"name": "KnightsDAO",
				"decimals": 18
			},
			"0x11efa3ad4285605c00ff9f2d7da248a8bf19c01d": {
				"ticker": "MUT",
				"address": "0x11Efa3ad4285605C00ff9f2d7da248A8bF19c01d",
				"name": "Mu Test Coin",
				"decimals": 18
			},
			"0xebe2eae72d6eaa44a3bca32cfdf81d3a687917c2": {
				"ticker": "EVB",
				"address": "0xebe2eae72D6eAA44A3bCA32cFDF81D3A687917c2",
				"name": "EVERBURN",
				"decimals": 6
			},
			"0xcab679cd2143e206dd77e6d5ea2fd1f718a2dbfa": {
				"ticker": "ROBO",
				"address": "0xCaB679Cd2143E206dd77e6d5ea2FD1F718a2dBFa",
				"name": "Robotics AI",
				"decimals": 9
			},
			"0x36e6401164c2dbd4fae237742f6492bebd6cb4e3": {
				"ticker": "wMEMO",
				"address": "0x36E6401164c2DbD4fAE237742F6492bebd6CB4E3",
				"name": "Wrapped MEMO",
				"decimals": 18
			},
			"0x75485ed6ccf640053155c9e357d1754e0c38ccd0": {
				"ticker": "SEC",
				"address": "0x75485ed6CCf640053155C9E357d1754E0c38cCD0",
				"name": "Secret DAO",
				"decimals": 9
			},
			"0x17aebcfde308432d34625b1f57c5e3fd89b1a553": {
				"ticker": "CRGT",
				"address": "0x17aEbCfDe308432d34625B1f57c5E3fD89b1A553",
				"name": "Corgi Treasure",
				"decimals": 9
			},
			"0x22b2c1072b70574ac491e01b3fce347eb838be84": {
				"ticker": "GLDT",
				"address": "0x22B2C1072b70574Ac491e01B3fce347eb838BE84",
				"name": "Gold Treasure",
				"decimals": 12
			},
			"0xc94e0aed065f04b7e0e7dbfea9f301e3b2520920": {
				"ticker": "HCH",
				"address": "0xC94e0AeD065F04b7E0e7dBFea9f301E3b2520920",
				"name": "Wrapped Hachi",
				"decimals": 12
			},
			"0xc5a466a685e56fa1010c5d70b004a071f738946c": {
				"ticker": "ISHA",
				"address": "0xC5a466a685E56fa1010c5d70b004A071f738946C",
				"name": "IS HAPPY",
				"decimals": 18
			},
			"0xfeaa757c779f4fbfd5cc86c4b2f4ca6822ae9d11": {
				"ticker": "SOA",
				"address": "0xfeaa757c779F4FBFD5Cc86c4B2f4CA6822aE9D11",
				"name": "SonOfAVAX",
				"decimals": 2
			},
			"0x78eb828c194c938a35715bf4d4b1b45914aaf19c": {
				"ticker": "UP",
				"address": "0x78eb828c194c938a35715bF4D4b1b45914aaF19C",
				"name": "go up",
				"decimals": 18
			},
			"0x7641f2d208aceda501a9857ecb9145fcd5f56879": {
				"ticker": "DIAR",
				"address": "0x7641f2D208ACEDa501a9857EcB9145fCd5f56879",
				"name": "DiamondRock",
				"decimals": 9
			},
			"0x120ad3e5a7c796349e591f1570d9f7980f4ea9cb": {
				"ticker": "LUNA",
				"address": "0x120AD3e5A7c796349e591F1570D9f7980F4eA9cb",
				"name": "Axelar Wrapped LUNA",
				"decimals": 6
			},
			"0xe41e657baa0f11e5706d46861212e2dac320827e": {
				"ticker": "WINGS",
				"address": "0xE41e657BAA0F11e5706d46861212e2Dac320827E",
				"name": "Angel Dapp",
				"decimals": 12
			},
			"0x290a9785c2a07b585febac0965580cf687be26c9": {
				"ticker": "UBI",
				"address": "0x290a9785C2A07B585fEBAc0965580cf687bE26c9",
				"name": "Ubisoft",
				"decimals": 18
			},
			"0x5981cf4ee154fde0147775dae5a7d5ac605e5f9f": {
				"ticker": "WHALE",
				"address": "0x5981cf4Ee154Fde0147775Dae5A7d5ac605E5f9f",
				"name": "Baby Whale",
				"decimals": 10
			},
			"0xe6ebe271a0f9e87d7cd42be9bda1091d85ec9328": {
				"ticker": "SPLASH",
				"address": "0xe6EBE271a0F9E87D7cD42BE9bda1091D85eC9328",
				"name": "Splash Games",
				"decimals": 9
			},
			"0x901c0954192edfecd35fcb89e0bde2827276a4a3": {
				"ticker": "SLR",
				"address": "0x901c0954192eDfEcd35Fcb89e0bde2827276A4a3",
				"name": "Solar Finance",
				"decimals": 8
			},
			"0xbf77597b47491f3d341de5373ac7ab418e9e9fe2": {
				"ticker": "XMAS",
				"address": "0xbf77597b47491F3D341de5373aC7ab418e9e9fe2",
				"name": "XMas Token",
				"decimals": 18
			},
			"0x0e0100ab771e9288e0aa97e11557e6654c3a9665": {
				"ticker": "PGL",
				"address": "0x0e0100Ab771E9288e0Aa97e11557E6654C3a9665",
				"name": "Pangolin Liquidity",
				"decimals": 18
			},
			"0x7aab2538b073705abd90033f090aafd34306e80e": {
				"ticker": "APE",
				"address": "0x7AaB2538b073705aBD90033F090aafD34306e80e",
				"name": "APE NODES",
				"decimals": 9
			},
			"0x31930118e903b3c356165f827ae48f4f84e92c5d": {
				"ticker": "CRG",
				"address": "0x31930118E903b3c356165f827ae48f4f84e92c5D",
				"name": "Corgi Coin",
				"decimals": 12
			},
			"0x8b3785cda614d0f8bf5a8cf1dfbb3cc4466e58fc": {
				"ticker": "Shrapnel",
				"address": "0x8b3785CDA614D0F8Bf5a8cF1dfbB3CC4466E58fc",
				"name": "Shrapnel",
				"decimals": 18
			},
			"0xd9203841641c2a5943a1174e51de8a22c61e8ef5": {
				"ticker": "CORGI",
				"address": "0xd9203841641C2A5943A1174E51dE8a22C61e8EF5",
				"name": "Meta Corgi",
				"decimals": 12
			},
			"0xfe0fbde885467b4ad1c5131f7182cf646dc1fdfa": {
				"ticker": "ONYX",
				"address": "0xfE0fbDE885467B4aD1c5131F7182cF646dC1FdFa",
				"name": "ONYX",
				"decimals": 8
			},
			"0x7d58734172879f16a9294f69fe6b81daf0e7c638": {
				"ticker": "STAR",
				"address": "0x7d58734172879f16A9294F69FE6B81DAF0e7c638",
				"name": "Super Star",
				"decimals": 9
			},
			"0xcb46f94ecffd91d5d8c8e1bc92eab5f8c80185c3": {
				"ticker": "SC",
				"address": "0xcB46f94EcfFd91D5d8C8e1bC92Eab5f8C80185C3",
				"name": "Swole Cobra Dojo",
				"decimals": 6
			},
			"0x851bebc285fe7dc5ccd0772ff787017537a51f15": {
				"ticker": "LFKI",
				"address": "0x851BeBC285fE7dc5ccD0772FF787017537A51f15",
				"name": "Little Floki",
				"decimals": 10
			},
			"0xf1e8d742f6e50b9a7ff6f91e10af528684a393cc": {
				"ticker": "PLAT",
				"address": "0xf1E8D742F6E50b9A7ff6F91e10aF528684A393CC",
				"name": "Platinum Finance",
				"decimals": 9
			},
			"0x5b4112cb730262b1188eeb67469d63560f72b103": {
				"ticker": "REDALERT",
				"address": "0x5b4112cB730262B1188eEB67469D63560F72b103",
				"name": "REDALERT",
				"decimals": 18
			},
			"0x19bb2bd9be9c5cc67c898794bfdb2ace22cc4108": {
				"ticker": "KING",
				"address": "0x19BB2Bd9Be9C5cC67c898794bFDb2aCe22CC4108",
				"name": "Crypto Kings",
				"decimals": 9
			},
			"0xd7c3cc93c072e2e22d9c9466be526334d9806fab": {
				"ticker": "CHIMP",
				"address": "0xD7C3cC93C072E2e22D9c9466BE526334D9806fAb",
				"name": "Chimp AVAX",
				"decimals": 10
			},
			"0xed9d029843b17d6454d08570c05f33033f4bc631": {
				"ticker": "PSHARE",
				"address": "0xED9d029843B17D6454D08570c05f33033f4Bc631",
				"name": "PSHARE",
				"decimals": 18
			},
			"0x739ad047006d17b92e4c65bffc033d7b6635c373": {
				"ticker": "WFTM",
				"address": "0x739Ad047006d17B92e4c65bFFc033D7b6635C373",
				"name": "Wrapped Fantom",
				"decimals": 10
			},
			"0x7f55695624984dd39c0babf0c068ee04a2195061": {
				"ticker": "HRI",
				"address": "0x7f55695624984dd39C0BaBF0C068ee04a2195061",
				"name": "Hero Infinity Token",
				"decimals": 18
			},
			"0x8242051861ac824e5fe0bd443dc8f52dc4b5d961": {
				"ticker": "RingDA",
				"address": "0x8242051861Ac824E5FE0bd443dC8f52dc4b5d961",
				"name": "RingDAO (https://t.me/ringdaoavax)",
				"decimals": 18
			},
			"0x7e50813bc8a0aabb3aa802240cf8cea7ac2431e0": {
				"ticker": "WINGS",
				"address": "0x7e50813BC8A0aAbb3Aa802240CF8Cea7aC2431E0",
				"name": "Angel Block",
				"decimals": 9
			},
			"0xe546fa453cc23cfa6bea803a174d43fa376055d7": {
				"ticker": "APE",
				"address": "0xE546FA453Cc23CfA6BeA803A174D43FA376055d7",
				"name": "Ape Network",
				"decimals": 10
			},
			"0xa98ecc8da0d99f91cde0206a5aa6a0595eead687": {
				"ticker": "BHatter",
				"address": "0xa98ecc8da0D99f91Cde0206a5Aa6a0595EeAD687",
				"name": "BabyHatter",
				"decimals": 9
			},
			"0x80fd9cebe1b3af71c67b002c4cb0bd1f3973326d": {
				"ticker": "REST",
				"address": "0x80fd9CebE1b3af71C67b002c4Cb0bd1f3973326D",
				"name": "EveRest",
				"decimals": 9
			},
			"0x0cb8d30192f78493a10797df5940104651a5e2ba": {
				"ticker": "AKA",
				"address": "0x0CB8d30192F78493a10797DF5940104651a5E2ba",
				"name": "Zen Akita",
				"decimals": 9
			},
			"0x107420ee94325803346178be46d05f1c12766827": {
				"ticker": "GTHD",
				"address": "0x107420Ee94325803346178BE46d05F1C12766827",
				"name": "Glitch Dapp",
				"decimals": 11
			},
			"0x5bd2a7bed642f2e8e819df535c0bacb7af87e4c8": {
				"ticker": "IFNTP",
				"address": "0x5Bd2A7bed642f2E8e819Df535c0BACb7af87E4C8",
				"name": "Infinity Protocol",
				"decimals": 18
			},
			"0xf4a2c562f9ff74f0371bc7564b1f54a276aadcdc": {
				"ticker": "GODL",
				"address": "0xf4a2c562f9FF74f0371BC7564b1F54a276AADCDc",
				"name": "GODL",
				"decimals": 18
			},
			"0x52f9fa0e75ec58726a85eff0a76e2a39bae02aec": {
				"ticker": "KITSU",
				"address": "0x52f9fA0e75EC58726A85EFf0A76E2A39Bae02aec",
				"name": "Kitsune Inu",
				"decimals": 18
			},
			"0x8f24e89b1a7dd4dc28a0cbad76a06bf6d8dad532": {
				"ticker": "UMANI",
				"address": "0x8F24E89B1A7dd4dC28A0cBAD76A06BF6d8daD532",
				"name": "https://AVAXnos.umani.fun",
				"decimals": 9
			},
			"0xf49fe07b97bbfaa474614a43259d460e4c84f6fa": {
				"ticker": "UNCLE",
				"address": "0xF49fe07B97BBFAa474614a43259D460E4C84f6fa",
				"name": "Uncle",
				"decimals": 18
			},
			"0x32774f445a9d118795a2652eee94f228f6facbae": {
				"ticker": "YUM",
				"address": "0x32774f445A9d118795a2652EeE94f228F6FACbaE",
				"name": "Wrapped Cake",
				"decimals": 9
			},
			"0x2b29e2b8b9eafe18b4ca8e223241c9e7787a3fdc": {
				"ticker": "ML",
				"address": "0x2b29E2b8B9eAfE18B4cA8E223241C9e7787a3FDC",
				"name": "Mystery Launch Avax",
				"decimals": 9
			},
			"0xc1d02e488a9ce2481bfdcd797d5373dd2e70a9c2": {
				"ticker": "SHAKE",
				"address": "0xC1d02E488a9Ce2481BFdcd797d5373Dd2E70a9C2",
				"name": "SHAKE token by SpaceSwap v2",
				"decimals": 18
			},
			"0xf17890c81be90d6ff35f265535f69213e08951c4": {
				"ticker": "CHIMP",
				"address": "0xF17890c81Be90d6ff35f265535F69213e08951C4",
				"name": "Super Chimp",
				"decimals": 10
			},
			"0x7ce59b86c065047c1e82c02b5ceb23b85295e142": {
				"ticker": "LVI",
				"address": "0x7ce59b86c065047c1e82c02b5CeB23b85295E142",
				"name": "SuperBowl DAO",
				"decimals": 9
			},
			"0x38dcf0532699b880e6a125f7d918380524cd60a6": {
				"ticker": "0006648936.e9cb",
				"address": "0x38Dcf0532699b880E6a125F7d918380524CD60a6",
				"name": "0006648936.e9cb",
				"decimals": 18
			},
			"0xc3ee234e08a0a92b287ab1e9742f01082b14eb39": {
				"ticker": "AR",
				"address": "0xc3eE234E08A0a92B287aB1E9742F01082b14eB39",
				"name": "Arsenic Coin",
				"decimals": 18
			},
			"0xe7bffd765d526a4ec3ed0fab0afbd4c43959497c": {
				"ticker": "ACAT",
				"address": "0xE7bffd765d526A4eC3ED0Fab0aFBd4c43959497c",
				"name": "AssCatFund",
				"decimals": 18
			},
			"0xad5267ad3c7a0b6af2e1618e3e372eff497759f8": {
				"ticker": "PINATA",
				"address": "0xad5267ad3c7a0b6Af2E1618e3E372eFf497759F8",
				"name": "NFT Management",
				"decimals": 9
			},
			"0x7f58ead6be7ecdd2ec63dddcdda615b0e8291f31": {
				"ticker": "BUBBLE",
				"address": "0x7f58EAd6be7EcdD2EC63dDdcDda615B0e8291F31",
				"name": "BUBBLE FINANCE",
				"decimals": 8
			},
			"0xc4f10f6643711956cd601c4a834861bfdb7161fb": {
				"ticker": "💍",
				"address": "0xC4F10f6643711956cD601C4A834861bFdB7161FB",
				"name": "DiamondRing",
				"decimals": 9
			},
			"0x45e795010c5802a55bd72eb1692150026d63812e": {
				"ticker": "STAR",
				"address": "0x45e795010c5802A55BD72eb1692150026D63812e",
				"name": "Zen Star",
				"decimals": 10
			},
			"0x1a51686fb42861aa7e38c1cf8868877f43f82aa4": {
				"ticker": "WHEAT",
				"address": "0x1A51686Fb42861AA7E38c1CF8868877F43F82aA4",
				"name": "Wheat Token",
				"decimals": 18
			},
			"0x68d19e3ff9319470575b9ca78fa39538a3c1245d": {
				"ticker": "PABLO",
				"address": "0x68d19e3Ff9319470575b9CA78Fa39538a3C1245d",
				"name": "Medellin City",
				"decimals": 18
			},
			"0x8dcfef557cefd5da38d1808981f28022f2bb3642": {
				"ticker": "GLT",
				"address": "0x8DcFEF557CeFD5da38D1808981F28022F2BB3642",
				"name": "Glitch Queen",
				"decimals": 10
			},
			"0x5016a1a559dc0b3b7eee3bf794598c883f2b785a": {
				"ticker": "MAGIC",
				"address": "0x5016a1A559Dc0b3b7eEe3BF794598c883F2b785A",
				"name": "MAGIC DAO",
				"decimals": 9
			},
			"0x2640e2308092654b7a1f7d641f6e61f3b8276eb3": {
				"ticker": "SUC",
				"address": "0x2640E2308092654B7a1f7d641f6E61F3B8276eB3",
				"name": "Super Useful Coin",
				"decimals": 8
			},
			"0xcab31f2a42da63d789c833fe532ddbbf58adbe44": {
				"ticker": "MoonPad",
				"address": "0xcAb31f2a42da63d789c833Fe532dDBbf58aDbe44",
				"name": "MoonPad",
				"decimals": 8
			},
			"0x2127aaee952ef01bc1053a4b26e2053b0b4f6104": {
				"ticker": "BABYJOE",
				"address": "0x2127aAEE952EF01Bc1053A4b26E2053B0b4F6104",
				"name": "BABYJOE",
				"decimals": 9
			},
			"0x44754455564474a89358b2c2265883df993b12f0": {
				"ticker": "ZEE",
				"address": "0x44754455564474A89358B2C2265883DF993b12F0",
				"name": "ZeroSwapToken",
				"decimals": 18
			},
			"0xa9b24e2802272e815d56a008175130385e008e4a": {
				"ticker": "ALPS",
				"address": "0xA9B24E2802272E815d56a008175130385e008E4a",
				"name": "AvalpsDAO",
				"decimals": 9
			},
			"0x1c40d3fc6e87d44f174cbfcfdc2563043b5a8b77": {
				"ticker": "GHST",
				"address": "0x1c40d3fC6e87d44f174CbFCFDC2563043b5a8B77",
				"name": "Ghost Monster",
				"decimals": 9
			},
			"0x2cf51e73c3516f3d86e9c0b4de0971dbf0766fd4": {
				"ticker": "XIO",
				"address": "0x2CF51e73C3516f3d86E9C0B4De0971Dbf0766Fd4",
				"name": "XIO Network",
				"decimals": 18
			},
			"0x5e618d50fd8eda73ae3192b40113d45d1705b64d": {
				"ticker": "KEY",
				"address": "0x5E618D50FD8EdA73AE3192B40113D45d1705b64D",
				"name": "NFT Key Marketplace",
				"decimals": 9
			},
			"0xe7dcec1fae6ae7f1d255799730e8254f8204282a": {
				"ticker": "GUN",
				"address": "0xe7dceC1FAe6ae7f1D255799730E8254F8204282a",
				"name": "Gun Nodes",
				"decimals": 9
			},
			"0x979ffd8eed7a43629ea29581df4bfe2b3f224e47": {
				"ticker": "OML",
				"address": "0x979fFD8eEd7a43629eA29581DF4Bfe2b3F224e47",
				"name": "Omlira",
				"decimals": 18
			},
			"0x892bb36c427b6e64ab5d1d155e7c8a0b1791b28b": {
				"ticker": "GUEST",
				"address": "0x892bb36C427b6e64Ab5D1D155e7c8A0B1791b28B",
				"name": "Guest Token",
				"decimals": 18
			},
			"0x2dd0d1c14586731701706de1abf9b2dc47561645": {
				"ticker": "OXY",
				"address": "0x2dd0D1c14586731701706dE1ABf9b2Dc47561645",
				"name": "Oxy-Fi",
				"decimals": 18
			},
			"0x08e226793bb9593c88cfdac0bbaf4ef79eae1221": {
				"ticker": "bSHARE",
				"address": "0x08E226793Bb9593c88CfdAc0BbaF4ef79eAE1221",
				"name": "RagingBull Share",
				"decimals": 18
			},
			"0xcbfc244f51ac5789e680ddbbebaebd73ec0a76e5": {
				"ticker": "SS",
				"address": "0xcbFc244f51ac5789e680DDBBebAEBd73Ec0A76E5",
				"name": "SnowShiba",
				"decimals": 9
			},
			"0xc147ee445e9e7575d8200c0121ec4502f354bff4": {
				"ticker": "SCOOBY",
				"address": "0xC147ee445E9e7575d8200C0121EC4502f354BfF4",
				"name": "ScoobyDoo",
				"decimals": 18
			},
			"0x3a8ee4ff7ee718dca30f1f059b5c826311345d7d": {
				"ticker": "BATTLE",
				"address": "0x3a8Ee4Ff7ee718dCA30f1f059B5c826311345D7D",
				"name": "Battle Noodes",
				"decimals": 9
			},
			"0xe09c9c3f6b47106b893802a18cf494e01b5cf9a1": {
				"ticker": "TND",
				"address": "0xE09c9C3F6B47106B893802A18CF494e01B5CF9A1",
				"name": "Exo Thunder",
				"decimals": 12
			},
			"0xcf98695af7eacd36e036dedb6e192c1c9c7e66d6": {
				"ticker": "🐆",
				"address": "0xcf98695AF7eACd36E036DedB6E192C1c9C7E66d6",
				"name": "snowleopard",
				"decimals": 9
			},
			"0x84e70897e18b5f87eb1cfc1311752ebad9cafb1d": {
				"ticker": "MPDA",
				"address": "0x84E70897e18b5f87EB1CfC1311752EBad9CaFb1d",
				"name": "Meta Panda",
				"decimals": 11
			},
			"0x5ceba6de12caf59c83ead8ccb0c67c5920cd63dc": {
				"ticker": "MULTI",
				"address": "0x5ceBA6De12CaF59C83Ead8ccb0C67C5920CD63dc",
				"name": "Multiversum",
				"decimals": 6
			},
			"0x1e5ab54f76ad72e3de5d99bd86f0ee7623c9c08a": {
				"ticker": "Joseph",
				"address": "0x1E5aB54f76aD72e3De5D99bD86F0eE7623c9C08A",
				"name": "Joseph",
				"decimals": 9
			},
			"0x2db403acb10213cd406ec714f9d8e1aff5571da8": {
				"ticker": "PINATA",
				"address": "0x2dB403aCB10213cd406ec714f9D8e1aFf5571DA8",
				"name": "NFT Management",
				"decimals": 9
			},
			"0x7c0f42dac628d08e264d3c31bec0ff90a80b3360": {
				"ticker": "GLD",
				"address": "0x7c0F42dAc628d08e264d3C31Bec0FF90A80B3360",
				"name": "Mini Gold",
				"decimals": 11
			},
			"0x55562df62b458c5e29be122c2694a7ea332aa258": {
				"ticker": "PXT",
				"address": "0x55562Df62B458c5e29bE122C2694A7Ea332Aa258",
				"name": "ProjectXNodes",
				"decimals": 18
			},
			"0xb0d341954287c48d7dccc6268c2fa62000fd2e72": {
				"ticker": "SMT",
				"address": "0xB0d341954287c48d7dccc6268C2Fa62000Fd2e72",
				"name": "Secret Meta Treasures",
				"decimals": 9
			},
			"0x02a775627fe2ed44a33e41d4f07a847bf56f5bcd": {
				"ticker": "STR",
				"address": "0x02a775627FE2ED44A33E41d4f07A847bF56f5BCD",
				"name": "Star Markets",
				"decimals": 11
			},
			"0x6b2990dd032577709eb6063d1dff11e71542f817": {
				"ticker": "ANON",
				"address": "0x6b2990dD032577709EB6063D1dFf11E71542f817",
				"name": "Anon Swap",
				"decimals": 9
			},
			"0xb28983792cf69c75368baff00a268e7b22233574": {
				"ticker": "DB",
				"address": "0xb28983792cF69C75368baFf00A268e7B22233574",
				"name": "DiamondBank",
				"decimals": 18
			},
			"0x11030b2b2c274b90e982073fd21b03bfd0e699ea": {
				"ticker": "MWHL",
				"address": "0x11030b2b2c274B90e982073Fd21b03BFd0e699Ea",
				"name": "Mini Whale",
				"decimals": 12
			},
			"0xdb33990ee8927f9d120bdff5fd872c789f44f2f1": {
				"ticker": "MOON",
				"address": "0xDb33990eE8927f9D120bDff5fd872C789f44f2f1",
				"name": "Moon Swap",
				"decimals": 9
			},
			"0x749581a337a3ab0fa9b2cca0f4775361810cc0a6": {
				"ticker": "ZAP",
				"address": "0x749581a337a3ab0FA9b2ccA0F4775361810cC0A6",
				"name": "Thunder BTC",
				"decimals": 11
			},
			"0x9ad5604c7c7be1d44438b39ac725190d0189d60f": {
				"ticker": "$SUN",
				"address": "0x9Ad5604c7C7Be1D44438B39ac725190D0189d60F",
				"name": "AztecNodes",
				"decimals": 18
			},
			"0xf436ea4c4f2e49f0679895aeae39dab698350eaa": {
				"ticker": "ZRST",
				"address": "0xf436Ea4C4f2e49F0679895aEAE39dab698350eAa",
				"name": "ZeroShift",
				"decimals": 9
			},
			"0x479c93e3e01198cf7c592a89b5687c40a20d0a19": {
				"ticker": "GUN",
				"address": "0x479c93e3E01198CF7C592a89B5687c40A20d0A19",
				"name": "Gun Games",
				"decimals": 9
			},
			"0x6cf7e3814dd76ac20086359802dd2eba5d99012d": {
				"ticker": "Yuzu",
				"address": "0x6Cf7e3814dD76AC20086359802Dd2eba5D99012D",
				"name": "Yuzu Node",
				"decimals": 18
			},
			"0x885d748c00a279b67a7749ec6b03301700dd0455": {
				"ticker": "MAXI",
				"address": "0x885d748C00A279B67A7749EC6b03301700dd0455",
				"name": "Maximus",
				"decimals": 18
			},
			"0x0dd2d581994132c919827a2d5401ed6ffdaeffc9": {
				"ticker": "SUS",
				"address": "0x0dd2d581994132C919827A2d5401ed6FFDAEFFc9",
				"name": "Amogus.Finance",
				"decimals": 18
			},
			"0xde78e85a2c8317bfff2d360ba52915e4891dc31a": {
				"ticker": "FAPL",
				"address": "0xdE78E85A2c8317BFfF2D360ba52915E4891DC31A",
				"name": "Fantasy Pineapple",
				"decimals": 11
			},
			"0xb04956a79616e8b462d2fb7b0edddc09c91e0b30": {
				"ticker": "UND",
				"address": "0xB04956A79616E8b462d2FB7b0EDddc09C91E0B30",
				"name": "UNINODEFork",
				"decimals": 9
			},
			"0x955cab04fab56a9f0bae3982ad7eab04e1c44e24": {
				"ticker": "testAURA",
				"address": "0x955cAB04Fab56A9F0bae3982Ad7eab04E1c44E24",
				"name": "testAurora",
				"decimals": 5
			},
			"0x2038837ffaa98076519e7af8ace586fe2d6ebbfd": {
				"ticker": "FRIDA",
				"address": "0x2038837ffAA98076519e7af8ACE586fe2d6EbbFd",
				"name": "Frida",
				"decimals": 18
			},
			"0x6a59df64074fd9713755a2054ba0bc446bdb93a1": {
				"ticker": "AvaxP2E",
				"address": "0x6a59df64074FD9713755A2054BA0Bc446bDb93A1",
				"name": "AvaxP2E",
				"decimals": 18
			},
			"0xf2cfc11093edb5a2dc7f49e70a3a3a9cd4f4fee4": {
				"ticker": "BRIG",
				"address": "0xF2CfC11093edB5A2DC7F49e70A3a3a9cd4F4FeE4",
				"name": "Brig Finance",
				"decimals": 18
			},
			"0x774adf2c0433069dc5d68f596bd5df35fb4fb1f6": {
				"ticker": "DFIAT",
				"address": "0x774Adf2C0433069Dc5D68F596bd5dF35fb4Fb1f6",
				"name": "DeFiato",
				"decimals": 18
			},
			"0xfc6c2951516f9beb88b9942adaa4d23c81abc572": {
				"ticker": "STHN",
				"address": "0xFc6c2951516f9bEB88b9942aDaA4d23c81aBC572",
				"name": "Super Thunder",
				"decimals": 10
			},
			"0xcef3cbc06e41d266feb146f09112811549029e9f": {
				"ticker": "AKT",
				"address": "0xCEF3CBc06E41D266feb146F09112811549029e9F",
				"name": "Akita Swap",
				"decimals": 11
			},
			"0xecc5748b1ff6b23f284ec81e8bf034409961d8dc": {
				"ticker": "OT-qiAVAX-28DEC2023",
				"address": "0xECC5748b1fF6b23f284EC81E8bf034409961d8Dc",
				"name": "OT Benqi AVAX 28DEC2023",
				"decimals": 18
			},
			"0x4b594d11047b07d688c93f61095531ab81dfafe1": {
				"ticker": "POWER",
				"address": "0x4B594D11047b07D688c93f61095531ab81dFafe1",
				"name": "Power Nodes",
				"decimals": 9
			},
			"0x960fa242468746c59bc32513e2e1e1c24fdfaf3f": {
				"ticker": "PGL",
				"address": "0x960FA242468746C59BC32513E2E1e1c24FDFaF3F",
				"name": "Pangolin Liquidity",
				"decimals": 18
			},
			"0x49966d27a8dc878c8d464e9414481cf422268b04": {
				"ticker": "SNIPER",
				"address": "0x49966d27a8Dc878c8d464e9414481cf422268b04",
				"name": "SniperNodes.io",
				"decimals": 9
			},
			"0xd58c88be5e7bd6fd45a64b6bde61f8027d26f705": {
				"ticker": "CFL",
				"address": "0xd58c88be5E7bd6fD45A64B6BDE61F8027d26f705",
				"name": "Carbon Fiber Landfill",
				"decimals": 18
			},
			"0x257e2cd15d94f113847281d658e2e031fb3e3640": {
				"ticker": "TSP",
				"address": "0x257E2cd15d94f113847281D658e2e031fb3E3640",
				"name": "TSHARE PRINTER",
				"decimals": 18
			},
			"0xb85692f62b2b85d104dea66fca1b51faf57e6f26": {
				"ticker": "DO-NOT-BUY2",
				"address": "0xB85692F62B2b85d104dEA66FCa1B51fAf57e6f26",
				"name": "DO-NOT-BUY2",
				"decimals": 18
			},
			"0x5bd4e575d9853d67dfd78ececb6c7623e03d98d5": {
				"ticker": "aFAX",
				"address": "0x5bd4e575d9853d67DFd78eceCb6C7623E03d98d5",
				"name": "aFAX",
				"decimals": 18
			},
			"0xb9b07f3c20c6bfadb35198233aa8425b4a9c5849": {
				"ticker": "HCIT",
				"address": "0xB9b07F3c20c6bFADb35198233Aa8425b4A9C5849",
				"name": "Hachi Token",
				"decimals": 12
			},
			"0x2f21bb07211e8760a95403296e6203527464c5a9": {
				"ticker": "WHL",
				"address": "0x2f21BB07211e8760A95403296E6203527464c5a9",
				"name": "Whale Inu",
				"decimals": 9
			},
			"0xe255de32070f9f78d1cb5ca04f91c1891464f08d": {
				"ticker": "FCC",
				"address": "0xE255De32070f9F78D1CB5ca04F91c1891464F08d",
				"name": "Fvck Cancel Culture",
				"decimals": 18
			},
			"0x66e8ec96ea89f05ae1890ffd8965f60fd9b54395": {
				"ticker": "DMTR",
				"address": "0x66e8Ec96eA89f05Ae1890fFD8965F60fd9b54395",
				"name": "Demeter",
				"decimals": 18
			},
			"0x1baca25ff61bdee06ce27679a9a63ae04402c06b": {
				"ticker": "SPACE",
				"address": "0x1BacA25Ff61bDEE06Ce27679a9A63Ae04402c06B",
				"name": "Space DAO",
				"decimals": 9
			},
			"0x78ea17559b3d2cf85a7f9c2c704eda119db5e6de": {
				"ticker": "AVE",
				"address": "0x78ea17559B3D2CF85a7F9C2C704eda119Db5E6dE",
				"name": "Avaware",
				"decimals": 18
			},
			"0xc743fb0ca2407dae05e6b8edd0870ad30bc8da52": {
				"ticker": "PN",
				"address": "0xC743Fb0CA2407DAE05E6B8EDD0870Ad30BC8Da52",
				"name": "ProbablyNothing",
				"decimals": 9
			},
			"0x6e1e533e6e5dab38379c643276286e6d34573bb5": {
				"ticker": "THRN",
				"address": "0x6e1E533e6e5DaB38379C643276286e6d34573BB5",
				"name": "ThronesDAO",
				"decimals": 18
			},
			"0x172b09e3d6ad122eb38bc8a51b5563857e24a49a": {
				"ticker": "DSNP",
				"address": "0x172b09e3d6Ad122eB38BC8a51B5563857e24a49A",
				"name": "DISNEY +",
				"decimals": 9
			},
			"0xc0a4f9822ef1b0bc60af282111eb81a2c40d985e": {
				"ticker": "TOX",
				"address": "0xC0A4f9822Ef1b0Bc60AF282111eB81a2C40d985e",
				"name": "Toxic Protocol",
				"decimals": 9
			},
			"0x01c2086facfd7aa38f69a6bd8c91bef3bb5adfca": {
				"ticker": "YAY",
				"address": "0x01C2086faCFD7aA38f69A6Bd8C91BEF3BB5adFCa",
				"name": "YAY Games",
				"decimals": 18
			},
			"0xdb6c0935fc746cfb0288ef8c52f1de0255e96c02": {
				"ticker": "VPND",
				"address": "0xDB6C0935fC746cfB0288EF8C52F1DE0255E96c02",
				"name": "VAPOR NODES",
				"decimals": 1
			},
			"0xe5caef4af8780e59df925470b050fb23c43ca68c": {
				"ticker": "FRM",
				"address": "0xE5CAeF4Af8780E59Df925470b050Fb23C43CA68C",
				"name": "Ferrum Network Token",
				"decimals": 18
			},
			"0x47aa3650cff9930f277d4670db138da818e1a3ca": {
				"ticker": "PLE",
				"address": "0x47aA3650CFF9930f277D4670dB138DA818E1a3CA",
				"name": "Plethori",
				"decimals": 18
			},
			"0x5fe5577ead2814d6de1914704d014dfc68db7a1f": {
				"ticker": "BETA",
				"address": "0x5Fe5577EAd2814D6dE1914704D014DFC68DB7a1f",
				"name": "Beta Token",
				"decimals": 18
			},
			"0xfe84ae6966c6be2aac9528ed4ae5774a8ee5ed5e": {
				"ticker": "MIMP",
				"address": "0xfE84ae6966c6be2AaC9528ED4ae5774A8EE5eD5e",
				"name": "MIMP",
				"decimals": 5
			},
			"0x9f125dd0a62fb88bf94e2513139d3e8f81305632": {
				"ticker": "ARAK",
				"address": "0x9F125DD0a62fB88BF94e2513139d3e8F81305632",
				"name": "ArakNode",
				"decimals": 9
			},
			"0x48e3724dbeb89c84ec28519aaefea678672cf697": {
				"ticker": "FireSifu",
				"address": "0x48E3724DBeB89C84eC28519aAeFEa678672cf697",
				"name": "Fire Sifu",
				"decimals": 18
			},
			"0x4f0cbe3089fe54907897a15bd7fdf4313759131f": {
				"ticker": "NINJA",
				"address": "0x4F0cbe3089Fe54907897a15BD7fDF4313759131F",
				"name": "Ninja Game",
				"decimals": 9
			},
			"0x942fefe6bf3a56f4adc100ce1c2da2d32da8712a": {
				"ticker": "IDM",
				"address": "0x942FeFe6Bf3A56F4aDc100CE1C2da2d32da8712A",
				"name": "IDM Token",
				"decimals": 9
			},
			"0x418e9a8fb0fdbe82ec1a971b76a0ba7cd6898eec": {
				"ticker": "ABC",
				"address": "0x418e9a8Fb0FDBE82EC1a971B76a0BA7cd6898eeC",
				"name": "Abracadabrant",
				"decimals": 18
			},
			"0xaa554c5ee10dc5bfa6e52f4f3800f4ffb787347d": {
				"ticker": "ARAK",
				"address": "0xaa554c5eE10DC5Bfa6E52F4F3800F4ffB787347D",
				"name": "ArakNode",
				"decimals": 9
			},
			"0x9b92585b8c93d81bf4412e143922fa19f970f719": {
				"ticker": "MAVERICK",
				"address": "0x9b92585B8c93D81BF4412E143922FA19F970f719",
				"name": "MAVERICK",
				"decimals": 8
			},
			"0x0568cabad439d9ed8edca3bb38d85f07960d91b9": {
				"ticker": "BLZD",
				"address": "0x0568cAbAd439d9eD8edCA3bb38D85f07960d91B9",
				"name": "Blizzard Ent",
				"decimals": 9
			},
			"0x9c846d808a41328a209e235b5e3c4e626dab169e": {
				"ticker": "FERT",
				"address": "0x9C846D808A41328A209e235B5e3c4E626DAb169E",
				"name": "chikn fert",
				"decimals": 18
			},
			"0x47d340629757f67f283055b6686317263fe8a139": {
				"ticker": "FLOJA",
				"address": "0x47D340629757F67F283055B6686317263fe8a139",
				"name": "Floki Ninja",
				"decimals": 9
			},
			"0xf7ef9dd406a45f8e17d6487553f857dea3ad38d8": {
				"ticker": "VGTI",
				"address": "0xf7ef9dd406A45f8e17d6487553f857dEa3AD38D8",
				"name": "Vegeta INFECTION",
				"decimals": 18
			},
			"0xe2ca809e48b6d45f12b8c6b85247a637504ddbed": {
				"ticker": "BRAND",
				"address": "0xE2cA809E48B6d45F12B8C6b85247a637504DDBed",
				"name": "Branded Financial",
				"decimals": 9
			},
			"0x3cbef762a500968986e3410a94cbf8daa5ccec84": {
				"ticker": "$Legend",
				"address": "0x3CBef762A500968986E3410a94CbF8daA5cceC84",
				"name": "$Legend",
				"decimals": 18
			},
			"0xff22ba30f9f715416adcef47e08e24272576b433": {
				"ticker": "DANA",
				"address": "0xFF22ba30F9f715416aDceF47E08E24272576b433",
				"name": "DAO Nation",
				"decimals": 9
			},
			"0xd0c7ad5733abe2e8d46fd01c706fe0d09a17f8cd": {
				"ticker": "TESTT",
				"address": "0xD0c7aD5733abE2e8d46fD01C706fe0D09A17f8cD",
				"name": "TESTT",
				"decimals": 9
			},
			"0x5ea4d8193cfcf431a22653db573410f8397ae65f": {
				"ticker": "ANG",
				"address": "0x5ea4D8193CFCF431a22653DB573410F8397aE65F",
				"name": "King Angel",
				"decimals": 10
			},
			"0x176a8404814ba627b16f51d7f94b4c41c8e7196b": {
				"ticker": "SHIB",
				"address": "0x176A8404814BA627b16f51d7f94B4C41C8e7196b",
				"name": "Shiba AI",
				"decimals": 11
			},
			"0xe5cc9bb854163ddb1b4525e7c538fc32de5000d3": {
				"ticker": "OCapital",
				"address": "0xE5cC9bb854163dDB1b4525e7c538fC32DE5000d3",
				"name": "Orange Capital",
				"decimals": 9
			},
			"0x3f6cbe61d851d3e7d3a68d0a994f5dead48d59e6": {
				"ticker": "100%",
				"address": "0x3F6cBe61D851D3e7D3A68d0A994F5dEad48D59e6",
				"name": "100%",
				"decimals": 8
			},
			"0xa54595ff0438e2372fd3ef622a4ad295436a78ab": {
				"ticker": "PFTOMB",
				"address": "0xa54595fF0438E2372FD3ef622A4aD295436a78AB",
				"name": "FTOMB Printer",
				"decimals": 18
			},
			"0xb092214663b7cb2e0497825aa4cecd5dbd3a178b": {
				"ticker": "GMA",
				"address": "0xB092214663B7CB2E0497825aa4CecD5DbD3a178B",
				"name": "GMA",
				"decimals": 9
			},
			"0x6dc941fb90b1563b6bc88c5fbaac63499cca328d": {
				"ticker": "SHIB",
				"address": "0x6DC941FB90b1563B6bC88C5FBAAC63499cCA328d",
				"name": "Safe Shiba",
				"decimals": 10
			},
			"0x8f6a7dee2386a1141e102eac0f7416fe766e7d54": {
				"ticker": "NODEON",
				"address": "0x8f6A7deE2386a1141E102eAC0f7416Fe766E7D54",
				"name": "NodeonProtocol",
				"decimals": 18
			},
			"0xb323c7f46f7f1ca0215553e794f7407c2fa71eeb": {
				"ticker": "ARMY",
				"address": "0xb323c7f46f7F1CA0215553E794F7407c2Fa71eeB",
				"name": "Army Token",
				"decimals": 18
			},
			"0x070c83cab53215705c9d2672ff9e982901b92ee4": {
				"ticker": "Diaomond",
				"address": "0x070C83cAb53215705c9d2672Ff9E982901b92ee4",
				"name": "Diamond Bank",
				"decimals": 9
			},
			"0x835c57b577332b28a576937ff1de5e3eccc284d2": {
				"ticker": "MDB",
				"address": "0x835C57B577332B28A576937Ff1dE5e3ECCc284d2",
				"name": "MindsDB",
				"decimals": 18
			},
			"0xd0f5abccbed45ea5fd1ab263a767e4dd48e9d5df": {
				"ticker": "FTM",
				"address": "0xD0f5ABcCbeD45eA5Fd1Ab263A767e4dD48e9d5dF",
				"name": "Fantom Block",
				"decimals": 9
			},
			"0xb27036f0d108b47380871c4e4c0fb3d56ae66fbd": {
				"ticker": "TND",
				"address": "0xb27036F0d108B47380871C4E4C0fB3d56Ae66fbd",
				"name": "Thunder Rise",
				"decimals": 11
			},
			"0xfbe94f72f5f0941e805e6602fe81b57d6b6c6260": {
				"ticker": "APE",
				"address": "0xfBE94F72f5F0941E805e6602fe81B57D6B6C6260",
				"name": "Ape Island",
				"decimals": 9
			},
			"0xcedde936b41d5e57c80cba19d64b5437d4c44cab": {
				"ticker": "T34U",
				"address": "0xCEDDe936b41d5E57C80Cba19d64b5437D4C44cAB",
				"name": "T34U",
				"decimals": 9
			},
			"0x6e54037f66f2fe0f8b08779382e6a8566d1b6a04": {
				"ticker": "ARAK",
				"address": "0x6E54037f66F2FE0F8b08779382e6A8566d1b6A04",
				"name": "ArakNode",
				"decimals": 9
			},
			"0xb3f52c360201201d366efd2cda4db78e00cb9d5d": {
				"ticker": "LYK",
				"address": "0xb3f52c360201201d366eFD2cDa4DB78e00cB9D5D",
				"name": "Luyuka",
				"decimals": 18
			},
			"0x51ba6b223b49ef34458819fcd5ca50a5d76b3c6f": {
				"ticker": "SafeDev",
				"address": "0x51BA6b223b49Ef34458819fcd5Ca50a5d76b3c6F",
				"name": "SAFEDEV",
				"decimals": 18
			},
			"0xe8a4db6093ae9d94579cef854018572baea91b95": {
				"ticker": "PDAO",
				"address": "0xe8a4dB6093aE9d94579CEf854018572BAEA91B95",
				"name": "Panda DAO Shares",
				"decimals": 18
			},
			"0xca0ac92cdd152c61cbde80adf9eed9dbb746b541": {
				"ticker": "EVB",
				"address": "0xcA0aC92CDD152C61cbDe80aDF9eEd9dBb746B541",
				"name": "EverBurn",
				"decimals": 3
			},
			"0x374e69fd66e2ba59a3c60a673e57e7d3be0a23c9": {
				"ticker": "APL",
				"address": "0x374E69fD66e2Ba59A3c60a673E57E7d3be0A23c9",
				"name": "Sushi Pineapple",
				"decimals": 11
			},
			"0x36ef3d2008c6691a9df8dfe713900ca1e22b0f74": {
				"ticker": "AKG",
				"address": "0x36eF3D2008C6691a9dF8dfE713900Ca1e22b0f74",
				"name": "AvaxKing",
				"decimals": 9
			},
			"0x4f91aee759c1dc37b42ece6464aa50a1c56be813": {
				"ticker": "SNOW",
				"address": "0x4f91Aee759C1dC37B42EcE6464Aa50A1c56bE813",
				"name": "Snow Nodes",
				"decimals": 18
			},
			"0x1682508daefdcb3423711acd22a0064cb975dffe": {
				"ticker": "SnowCapital",
				"address": "0x1682508DaeFdcB3423711acd22A0064cB975dffe",
				"name": "SnowCapital",
				"decimals": 18
			},
			"0x0b41177a3cf6c174c85c5a3cb4f1d48498fa1809": {
				"ticker": "Ato",
				"address": "0x0B41177a3CF6C174C85c5a3cb4F1d48498Fa1809",
				"name": "Atom Exchange",
				"decimals": 8
			},
			"0x52051965d659e465299b60dc173803809a0c4060": {
				"ticker": "MPRINT",
				"address": "0x52051965D659E465299B60Dc173803809a0C4060",
				"name": "MIM PRINT ",
				"decimals": 18
			},
			"0x7eb8714b5128348b173d3dd501ffb9d17c90c50d": {
				"ticker": "EXPLO",
				"address": "0x7eb8714b5128348b173D3DD501ffB9d17c90c50d",
				"name": "ExplorerDAO",
				"decimals": 18
			},
			"0x6d13197bdbaf67c84010a3fbc7e4ede3cbaf3832": {
				"ticker": "SFARM",
				"address": "0x6D13197bdBAf67c84010a3FbC7E4eDE3cbaF3832",
				"name": "SPELL FARM",
				"decimals": 18
			},
			"0x75dd1e62bde14278a1ac82e6f9c79cb6bbf7d32a": {
				"ticker": "VGTP",
				"address": "0x75DD1E62BdE14278A1aC82e6f9c79Cb6bbf7d32A",
				"name": "Vegeta PRINTER",
				"decimals": 18
			},
			"0x4a0fcb5ba04de2411c59020d7a747bad810722ca": {
				"ticker": "AXLF",
				"address": "0x4a0fCB5Ba04dE2411c59020d7a747baD810722Ca",
				"name": "Axxeleum Finance",
				"decimals": 18
			},
			"0x47f246e057811e88feaa72a14091b384efdf0a62": {
				"ticker": "CHIMP",
				"address": "0x47F246E057811e88FEAa72A14091B384efdf0A62",
				"name": "OMG Chimp",
				"decimals": 9
			},
			"0x3a39ac5ed74a0f06a7dcc3ad331393a176abb391": {
				"ticker": "LGB",
				"address": "0x3a39AC5ED74A0F06A7dcC3AD331393A176aBB391",
				"name": "Let's Go Brandon DAO",
				"decimals": 9
			},
			"0xe4b798b1823eae3f3f8e1fc9135a2fedd5e46f3f": {
				"ticker": "SIR",
				"address": "0xe4B798B1823EAE3f3F8E1Fc9135a2fEdD5E46F3F",
				"name": "SimpleReflection",
				"decimals": 9
			},
			"0x1bfb4052ad17a4f4ca13d64bff03d314f37f2109": {
				"ticker": "DOOM",
				"address": "0x1bfB4052ad17A4f4Ca13d64BFf03D314f37F2109",
				"name": "DOOM",
				"decimals": 8
			},
			"0xb0647c34f4a74acae8a08a510db43294e31de746": {
				"ticker": "PYS",
				"address": "0xb0647c34F4A74acAe8A08a510Db43294E31De746",
				"name": "PRINT YOUR SPELL ",
				"decimals": 18
			},
			"0x7fe77d912849790f7799aa570b00ce31b8b96a64": {
				"ticker": "SHB",
				"address": "0x7fe77d912849790F7799AA570B00ce31b8b96a64",
				"name": "Queen Shiba",
				"decimals": 12
			},
			"0x81440c939f2c1e34fc7048e518a637205a632a74": {
				"ticker": "CYCLE",
				"address": "0x81440C939f2C1E34fc7048E518a637205A632a74",
				"name": "Cycle Token",
				"decimals": 18
			},
			"0x50c4382d8f3a6c6bb14df73b6301ee6556927e6a": {
				"ticker": "123test",
				"address": "0x50c4382D8F3a6C6bb14dF73b6301EE6556927e6a",
				"name": "test",
				"decimals": 18
			},
			"0xcb30868c414084becf4a35fe89d1680563c32d0c": {
				"ticker": "AKT",
				"address": "0xCb30868C414084BEcF4a35fe89D1680563C32d0c",
				"name": "Wrapped Akita",
				"decimals": 10
			},
			"0xe7d627aad15c7dbdb362400c25f812617f8f3df0": {
				"ticker": "PIN",
				"address": "0xe7D627aad15c7dBdB362400C25f812617F8F3df0",
				"name": "Pineapple Treasure",
				"decimals": 10
			},
			"0x9039512e808b2727c2e38673b5373d74c874dc4e": {
				"ticker": "ELITE",
				"address": "0x9039512E808B2727c2e38673B5373D74C874DC4e",
				"name": "ELITE NODES",
				"decimals": 9
			},
			"0x6ccebe4d3f1d9cfc1e5a8ce56c0eac3d52423423": {
				"ticker": "GOD",
				"address": "0x6ccebe4d3f1d9CfC1e5a8cE56C0EAC3D52423423",
				"name": "GalaxyOlympusDAO",
				"decimals": 9
			},
			"0x0935dad49b922d688f8f8dbcb8fd73197eb53c13": {
				"ticker": "KRAKEN",
				"address": "0x0935DAd49B922D688F8F8DbCb8FD73197eb53C13",
				"name": "KrakenDAO",
				"decimals": 18
			},
			"0x483286d3ab8b8647264f7d0a8d9c328459082703": {
				"ticker": "NYN",
				"address": "0x483286D3AB8B8647264f7d0a8d9c328459082703",
				"name": "New Year Nodes",
				"decimals": 9
			},
			"0xd439d439d0dc8147901c3ca89fe04350693099a5": {
				"ticker": "MOON",
				"address": "0xD439d439D0DC8147901c3CA89Fe04350693099A5",
				"name": "Moon Diamond",
				"decimals": 9
			},
			"0xa96d40e7d710cc3fdfef93aa9e5f241ba13305e8": {
				"ticker": "Test",
				"address": "0xA96d40E7D710cc3fDfEf93Aa9e5f241Ba13305e8",
				"name": "TestDAO",
				"decimals": 9
			},
			"0x196d5d8f4ec51af5f4f48c7fe65e0dfd4bac6aeb": {
				"ticker": "AST",
				"address": "0x196d5d8F4ec51af5F4f48C7fE65e0dFD4baC6aEB",
				"name": "Asteroid",
				"decimals": 18
			},
			"0x26418d1fa95750d21fa0cb4f06febb19b42795e6": {
				"ticker": "BULLS",
				"address": "0x26418d1fA95750D21Fa0CB4f06fEBb19B42795e6",
				"name": "Bull Run",
				"decimals": 9
			},
			"0xdffff47ed7729adc7d9acb0c38eb0a89f5d32ff1": {
				"ticker": "SOSO",
				"address": "0xdffFf47ed7729Adc7d9ACB0c38eB0a89F5d32fF1",
				"name": "Soso",
				"decimals": 18
			},
			"0xcda6ed0066d79674a50314ac1eaa945f2b6acf12": {
				"ticker": "CHIMP",
				"address": "0xcDA6Ed0066D79674a50314aC1EAa945f2b6Acf12",
				"name": "Chimp Beast",
				"decimals": 12
			},
			"0xdcf5b32c79417a89a918453c2289cb7cdfdb13b7": {
				"ticker": "Test",
				"address": "0xDcf5b32c79417a89a918453C2289cB7CdfdB13b7",
				"name": "TestDAO",
				"decimals": 9
			},
			"0x9bdeb666b0337cc582f409a3ca88e0a6db3a21c8": {
				"ticker": "1DOLLAR",
				"address": "0x9BDEb666B0337Cc582F409a3CA88E0A6db3A21C8",
				"name": "DUMP AT $1",
				"decimals": 1
			},
			"0x962ac1de12314b757b1daaa59680b044f3667ec8": {
				"ticker": "MSTIM",
				"address": "0x962Ac1de12314B757B1DaaA59680B044F3667EC8",
				"name": "Master MIM",
				"decimals": 18
			},
			"0xc8b142ba084d8e7b60d43e65ddb79dd9db0a12bf": {
				"ticker": "META",
				"address": "0xc8B142Ba084d8E7b60d43E65ddb79DD9DB0a12Bf",
				"name": "Meta DAO",
				"decimals": 9
			},
			"0x92e34171e03239a917c1a41af224953cf58b0f06": {
				"ticker": "APL",
				"address": "0x92e34171E03239a917C1A41af224953cF58b0F06",
				"name": "Pineapple Token",
				"decimals": 9
			},
			"0xd8187f630a93a1d841dbbc99cd5fe06587a984de": {
				"ticker": "FORTUNE",
				"address": "0xd8187f630A93A1d841dbBC99cd5fe06587A984DE",
				"name": "Fortune Cookie",
				"decimals": 9
			},
			"0x1ffb6ffc629f5d820dcf578409c2d26a2998a140": {
				"ticker": "PGL",
				"address": "0x1fFB6ffC629f5D820DCf578409c2d26A2998a140",
				"name": "Pangolin Liquidity",
				"decimals": 18
			},
			"0xf22b6e3bf8b8890da375338555ae8fbb3195d7b2": {
				"ticker": "CAVAX",
				"address": "0xF22b6e3bf8b8890DA375338555Ae8FBB3195d7B2",
				"name": "AVAX CAPITAL",
				"decimals": 9
			},
			"0xe3cc346b7ec0f9d2cb7f93c72b5710fa7a9c277b": {
				"ticker": "LayDAO",
				"address": "0xE3Cc346B7eC0F9D2CB7F93C72B5710Fa7a9c277b",
				"name": "LayerZ DAO",
				"decimals": 9
			},
			"0xa5df2d7f7f608599c4d757d22bf2dd0fc77e8973": {
				"ticker": "RD",
				"address": "0xa5Df2d7f7F608599C4d757d22BF2DD0Fc77E8973",
				"name": "Radium Coin",
				"decimals": 18
			},
			"0x2f4fc16308be09e7611a047b9576881eb5298dd6": {
				"ticker": "BURN",
				"address": "0x2f4FC16308Be09e7611a047B9576881Eb5298dD6",
				"name": "Snow Burn",
				"decimals": 18
			},
			"0x2b3a6972a3450697c64a0b50ef0d859dbe657b77": {
				"ticker": "ETHP",
				"address": "0x2b3A6972a3450697c64a0B50EF0D859dBe657b77",
				"name": "ETHP",
				"decimals": 18
			},
			"0x937e077abaea52d3abf879c9b9d3f2ebd15baa21": {
				"ticker": "OH",
				"address": "0x937E077aBaEA52d3abf879c9b9d3f2eBd15BAA21",
				"name": "Oh! Finance",
				"decimals": 18
			},
			"0x9056c40949c6fe0fc8ae88a679a110df08b7e63a": {
				"ticker": "FlokiSant",
				"address": "0x9056C40949C6FE0FC8Ae88A679a110df08b7E63A",
				"name": "FlokiSanta (https://t.me/FlokiSantaAvax)",
				"decimals": 18
			},
			"0x2c02cfe9a025e232ab9bfd8b4f7c488d8ec1faf1": {
				"ticker": "DO-NOT-BUY22",
				"address": "0x2c02cFe9a025e232ab9BFd8b4f7c488d8Ec1Faf1",
				"name": "DO-NOT-BUY22",
				"decimals": 18
			},
			"0xc8c07ca3b6e3c156e2fb50aefd11c0a5c614a5c9": {
				"ticker": "AVAXP",
				"address": "0xC8C07cA3B6e3C156E2fb50AEfd11c0a5c614A5C9",
				"name": "AVAX PRINT",
				"decimals": 18
			},
			"0xab9a04808167c170a9ec4f8a87a0cd781ebcd55e": {
				"ticker": "aUST",
				"address": "0xaB9A04808167C170A9EC4f8a87a0cD781ebcd55e",
				"name": "Anchor Terra USD (Wormhole)",
				"decimals": 6
			},
			"0x7dd84b6958e06f8d9c0f41889658703c482dd53b": {
				"ticker": "Whit",
				"address": "0x7DD84b6958E06F8d9c0F41889658703c482dD53B",
				"name": "RABBIT",
				"decimals": 18
			},
			"0x5137fe50a88ee1f255f187c8170333a8decc5637": {
				"ticker": "GCH",
				"address": "0x5137fe50a88Ee1f255F187C8170333A8dECc5637",
				"name": "Zen Glitch",
				"decimals": 11
			},
			"0xc6ea6863c62f23bb2df49cf5209da194b4b5141c": {
				"ticker": "AKA",
				"address": "0xC6EA6863c62F23BB2df49CF5209dA194B4B5141c",
				"name": "Akita Treasure",
				"decimals": 12
			},
			"0xdb1b45346eb82dfe3dc8e710c432730cdae824be": {
				"ticker": "CHI",
				"address": "0xDb1b45346EB82dfe3dC8e710C432730CDAE824BE",
				"name": "Crypto Chimp",
				"decimals": 12
			},
			"0x536e911b8ba66c9a8697bf7d7b9924456abcc9e7": {
				"ticker": "STASH",
				"address": "0x536e911b8BA66c9a8697bF7d7b9924456ABCC9e7",
				"name": "Stash",
				"decimals": 5
			},
			"0x70b33ebc5544c12691d055b49762d0f8365d99fe": {
				"ticker": "PAPA",
				"address": "0x70b33ebC5544C12691d055b49762D0F8365d99Fe",
				"name": "PAPA",
				"decimals": 9
			},
			"0xe76dd53545f7ae2736069612d56ebab8c1683dae": {
				"ticker": "GLO",
				"address": "0xe76Dd53545f7ae2736069612D56EbAB8C1683daE",
				"name": "Glorycoin",
				"decimals": 18
			},
			"0xa80067563f9a14cf0ba7a43e740825e66a484631": {
				"ticker": "CRG",
				"address": "0xa80067563f9A14Cf0bA7A43e740825E66A484631",
				"name": "OMG Corgi",
				"decimals": 12
			},
			"0xa0056b28ac7e42e3361cc187cd546754a819112e": {
				"ticker": "MAGIC",
				"address": "0xA0056b28Ac7e42e3361cC187CD546754A819112e",
				"name": "Magic Node",
				"decimals": 9
			},
			"0x9cf70ec75fd09eed882df4116d62bc3922aad1fb": {
				"ticker": "MISL",
				"address": "0x9Cf70Ec75Fd09eEd882df4116D62Bc3922aAD1Fb",
				"name": "Missle NODE",
				"decimals": 9
			},
			"0x94389fbe9e371f165566a8eb3244a4b79af3faf3": {
				"ticker": "MAPE",
				"address": "0x94389fBe9E371f165566a8eB3244A4B79Af3faF3",
				"name": "MetaApes",
				"decimals": 9
			},
			"0x57f73f69314af53082acd987b225a1a78d877473": {
				"ticker": "Secret",
				"address": "0x57f73f69314aF53082acd987b225a1a78D877473",
				"name": "Secret Chain",
				"decimals": 9
			},
			"0xee6e54af39052871da2ae3bc0fe0733b52fbecc6": {
				"ticker": "WAWE",
				"address": "0xee6e54af39052871dA2aE3BC0fe0733b52FbECC6",
				"name": "Wave Money",
				"decimals": 18
			},
			"0x329a4fc4871e84e96541660bdf40e280920e4925": {
				"ticker": "ZSHB",
				"address": "0x329a4FC4871e84e96541660bDF40E280920e4925",
				"name": "Zen Shiba",
				"decimals": 9
			},
			"0x6b0b255631548be7cad2c5b09a294ae66ae84eb1": {
				"ticker": "SOB",
				"address": "0x6b0B255631548bE7CAd2C5b09a294ae66ae84eB1",
				"name": "SON OF BEERUS",
				"decimals": 9
			},
			"0x45cf95ab8a6cea5d5f8a46b4441e035e0a844d7c": {
				"ticker": "Flash",
				"address": "0x45cf95ab8a6CEA5D5f8a46B4441e035e0a844D7c",
				"name": "MoonFlash DAO",
				"decimals": 18
			},
			"0xf5eb4cdc937a1d0dc9459f9012cb273d4e5bc791": {
				"ticker": "STAR",
				"address": "0xf5eb4CdC937a1D0dc9459F9012Cb273D4e5Bc791",
				"name": "STAR DAO",
				"decimals": 9
			},
			"0xc38fcb7d9200fa3b4700155f3f8900440026d7ed": {
				"ticker": "VSH",
				"address": "0xC38fCB7D9200fA3B4700155F3F8900440026D7ed",
				"name": "valenshiba",
				"decimals": 9
			},
			"0x562f12992626c680aef3722a8a98ea6b56943521": {
				"ticker": "SLOP",
				"address": "0x562F12992626c680aEf3722a8a98eA6B56943521",
				"name": "Slopdog",
				"decimals": 18
			},
			"0xad50ed98daeabc14c9dcac8d08444a28ac5b6cef": {
				"ticker": "MGTH",
				"address": "0xaD50ed98daeabc14c9dCAc8D08444a28Ac5b6CEF",
				"name": "Mini Glitch",
				"decimals": 9
			},
			"0x0296738ee5e15373017414488bdced79523fed5c": {
				"ticker": "Cupid.Farm",
				"address": "0x0296738ee5E15373017414488BdcED79523feD5c",
				"name": "Cupid.Farm",
				"decimals": 9
			},
			"0x642eb464a9e0c276708817b58afeb303cb389252": {
				"ticker": "wBNBPPV2",
				"address": "0x642eB464A9e0c276708817b58aFEB303CB389252",
				"name": "wBNBPPV2",
				"decimals": 8
			},
			"0xcde6bd1be0f97d88f6354026cb0dfa0b78cbf083": {
				"ticker": "BIRD",
				"address": "0xcDe6bd1BE0f97d88f6354026cb0dFA0b78cBf083",
				"name": "BirdsEye NODES",
				"decimals": 9
			},
			"0x057fcca5971a7a55bbc6684f68c63e2f1702e670": {
				"ticker": "IME",
				"address": "0x057FCcA5971a7a55BBC6684F68C63E2f1702E670",
				"name": "Imperium Empires",
				"decimals": 9
			},
			"0x39d6441e2914ff2a25ad2b5eb5fcc7eccf6dbce6": {
				"ticker": "GSHARE",
				"address": "0x39d6441E2914fF2A25aD2B5Eb5Fcc7EcCf6dbcE6",
				"name": "GSHARES",
				"decimals": 18
			},
			"0x218ac1bc9ad8c0562988b03263540ec85c462f5a": {
				"ticker": "ANORTIS",
				"address": "0x218aC1bc9Ad8c0562988B03263540ec85C462F5a",
				"name": "Anortis",
				"decimals": 18
			},
			"0x03f8778c61dbe56170704e390d9d1a22b7cc1520": {
				"ticker": "AVAXMINER",
				"address": "0x03f8778C61dBE56170704e390D9d1a22b7CC1520",
				"name": "AVAXMINER",
				"decimals": 6
			},
			"0x76ad81f5cf5891b8a04a96915bc9eb5d92aa6cab": {
				"ticker": "Verify",
				"address": "0x76ad81F5cf5891B8a04a96915BC9eB5D92aA6CAb",
				"name": "Verify DeFi",
				"decimals": 9
			},
			"0x3a942c41211df46f49dd6ff89d5e714765199705": {
				"ticker": "ZAPC",
				"address": "0x3A942C41211Df46f49dd6Ff89d5e714765199705",
				"name": "Thunder Coin",
				"decimals": 12
			},
			"0xaeee354b19fe805db70f87dca481a5097a771965": {
				"ticker": "RISEN",
				"address": "0xaeEe354B19Fe805Db70f87Dca481A5097a771965",
				"name": "RISE Nodes",
				"decimals": 18
			},
			"0x352f04e75787faaed30c5e7295eada1007a392f4": {
				"ticker": "SWTC",
				"address": "0x352f04E75787fAAEd30C5E7295EaDA1007a392F4",
				"name": "SWTC token",
				"decimals": 18
			},
			"0x2f13f452b601ac73113c6585cacdcceaef060cbe": {
				"ticker": "CORK",
				"address": "0x2f13f452b601aC73113c6585cacDCCeAeF060cBE",
				"name": "Corkscrew",
				"decimals": 18
			},
			"0xe7945319af85f2ce30d4480b07f8b74974444482": {
				"ticker": "SNOOPY",
				"address": "0xE7945319af85F2Ce30d4480B07f8B74974444482",
				"name": "Snoopy Inu",
				"decimals": 18
			},
			"0x8a9c36bc3ced5ecce703a4da8032218dfe72fe86": {
				"ticker": "PGL",
				"address": "0x8a9c36BC3CEd5ECce703A4dA8032218Dfe72fE86",
				"name": "Pangolin Liquidity",
				"decimals": 18
			},
			"0x1bee6be8f1a19690564f6ab5bbed28f2f68eb770": {
				"ticker": "WHEAT",
				"address": "0x1BEe6bE8f1A19690564f6AB5bBeD28F2F68eb770",
				"name": "Wheat token",
				"decimals": 9
			},
			"0xa5699562b690f0a5baeb9333cceb6221787b5ca8": {
				"ticker": "GHOST",
				"address": "0xa5699562B690F0A5BAeB9333cCeB6221787b5Ca8",
				"name": "Avax Ghost",
				"decimals": 9
			},
			"0xa16693fda89e0c4dd9f2324ebd47118b57e6ba34": {
				"ticker": "avOHMINU",
				"address": "0xA16693FdA89E0c4dd9f2324eBd47118b57e6BA34",
				"name": "Ohm Inu DAO",
				"decimals": 9
			},
			"0xa47079c16ac80989c2694059682166e881eb4805": {
				"ticker": "SNOG",
				"address": "0xa47079c16Ac80989c2694059682166E881eb4805",
				"name": "Snow Games",
				"decimals": 9
			},
			"0xefe9fc9bfc6500e4813c5552e96e328e13a106a8": {
				"ticker": "HOOT",
				"address": "0xEfe9fC9bFC6500e4813c5552E96e328E13A106a8",
				"name": "Moonbirds",
				"decimals": 18
			},
			"0x7e409f20f63174324fb361f58facd30f6d5c8f68": {
				"ticker": "ELNC",
				"address": "0x7E409f20F63174324fB361F58faCd30F6D5C8F68",
				"name": "Elon Coin",
				"decimals": 11
			},
			"0xcc2f1d827b18321254223df4e84de399d9ff116c": {
				"ticker": "SMRT",
				"address": "0xCC2f1d827b18321254223dF4e84dE399D9Ff116c",
				"name": "SmartCoin",
				"decimals": 18
			},
			"0x514bf89a8579ab4fcce664aad2f88935e61864d7": {
				"ticker": "WHALE",
				"address": "0x514BF89a8579Ab4FccE664AAd2f88935e61864d7",
				"name": "Crypto Whale",
				"decimals": 11
			},
			"0xbe2a389079686d54d2b4e5005293bf134a7ff291": {
				"ticker": "SHB",
				"address": "0xBe2A389079686D54d2B4E5005293bF134a7fF291",
				"name": "Meta Shiba",
				"decimals": 10
			},
			"0x726573a7774317dd108accb2720ac9848581f01d": {
				"ticker": "CheemsX",
				"address": "0x726573a7774317DD108ACcb2720Ac9848581F01D",
				"name": "CheemsX",
				"decimals": 18
			},
			"0xd33df97747dd6becad26b2e61f818c94b7588e69": {
				"ticker": "$REACT",
				"address": "0xd33df97747dD6bEcAD26B2e61F818c94B7588E69",
				"name": "Rebase Aggregator Capital",
				"decimals": 18
			},
			"0xd8b8f051efb7d9c1938a3ad0ed159e86d1dc2acb": {
				"ticker": "Orange",
				"address": "0xd8b8F051EfB7d9C1938A3ad0Ed159e86D1dc2acB",
				"name": "Orange Node",
				"decimals": 18
			},
			"0xf3c93927e3faa88bd8538bf41f99d825de069410": {
				"ticker": "PPAL",
				"address": "0xf3C93927e3FAa88BD8538bF41F99D825DE069410",
				"name": "PayPal Finance",
				"decimals": 9
			},
			"0x168e07087f3b11a663ffc4dd0b08b96b1f99e42b": {
				"ticker": "SNOWSHIB",
				"address": "0x168E07087F3B11A663FfC4Dd0B08B96b1F99E42b",
				"name": "SnowShiba Dao",
				"decimals": 9
			},
			"0xf8d0c6c3ddc03f43a0687847f2b34bfd6941c2a2": {
				"ticker": "STRAW",
				"address": "0xf8D0C6c3ddC03F43A0687847f2b34bfd6941C2A2",
				"name": "Sundae Finance",
				"decimals": 18
			},
			"0xd7c295e399ca928a3a14b01d760e794f1adf8990": {
				"ticker": "DSLA",
				"address": "0xD7c295E399CA928A3a14b01D760E794f1AdF8990",
				"name": "DSLA",
				"decimals": 18
			},
			"0x2841a8a2ce98a9d21ad8c3b7fc481527569bd7bb": {
				"ticker": "SL3",
				"address": "0x2841A8a2ce98A9d21aD8C3B7Fc481527569bd7bb",
				"name": "SL3",
				"decimals": 9
			},
			"0x463cbad61c4112a8a52f6d6e14789822c9524efb": {
				"ticker": "BURGER",
				"address": "0x463cbAd61c4112A8a52F6D6E14789822c9524EFB",
				"name": "Floki Burger",
				"decimals": 9
			},
			"0x617724974218a18769020a70162165a539c07e8a": {
				"ticker": "OLIVE",
				"address": "0x617724974218A18769020A70162165A539c07E8a",
				"name": "OliveCash Token",
				"decimals": 18
			},
			"0x2daa27e5a182c44b29d8a2e31b74c5be09348f0f": {
				"ticker": "METEOR",
				"address": "0x2dAA27e5A182C44B29d8a2e31b74c5be09348f0F",
				"name": "METEOR",
				"decimals": 9
			},
			"0x72313d4536bd3e74988386039f99abe7a2694ee3": {
				"ticker": "TIMEv2",
				"address": "0x72313d4536bd3E74988386039f99AbE7a2694eE3",
				"name": "Wonderland TIMEv2",
				"decimals": 9
			},
			"0x9610b01aaa57ec026001f7ec5cface51bfea0ba6": {
				"ticker": "BUSD",
				"address": "0x9610b01AAa57Ec026001F7Ec5CFace51BfEA0bA6",
				"name": "Binance-Peg BUSD Token",
				"decimals": 18
			},
			"0x33d33a7faebdaa68c43250f35cdcb209c6081b70": {
				"ticker": "MSTRM",
				"address": "0x33D33a7FaebdAa68C43250F35cDCb209C6081b70",
				"name": "Master MIM",
				"decimals": 9
			},
			"0xc13e562d92f7527c4389cd29c67dabb0667863ea": {
				"ticker": "PGL",
				"address": "0xc13E562d92F7527c4389Cd29C67DaBb0667863eA",
				"name": "Pangolin Liquidity",
				"decimals": 18
			},
			"0x0b0c50db0f07be88d7620e264ee3bbc472c34aee": {
				"ticker": "GemMine",
				"address": "0x0B0C50DB0F07be88D7620e264ee3bBC472C34AEe",
				"name": "GemMine DAO",
				"decimals": 9
			},
			"0x75c411b47c0cef23e6c0c323303716f222853462": {
				"ticker": "PXT",
				"address": "0x75c411b47c0Cef23E6c0c323303716F222853462",
				"name": "ProjectXNodes",
				"decimals": 9
			},
			"0x9863bb53308796b77e05fd2f443f64664eb4f138": {
				"ticker": "PINI",
				"address": "0x9863Bb53308796b77e05Fd2F443f64664eb4f138",
				"name": "Pineapple Infinity",
				"decimals": 12
			},
			"0x5b7c882a059dfe17a58bda26db1ffc7f5c2a6a96": {
				"ticker": "Peony",
				"address": "0x5b7c882A059Dfe17a58Bda26DB1fFc7F5C2A6A96",
				"name": "Peony Node",
				"decimals": 18
			},
			"0x094bfac9894d2a2a35771d0bd6d2447689190f32": {
				"ticker": "CAT",
				"address": "0x094BFaC9894d2A2A35771D0BD6d2447689190F32",
				"name": "CAT",
				"decimals": 18
			},
			"0x34ec6d588d994b1f956571a31e677632a3c27fed": {
				"ticker": "BILB",
				"address": "0x34eC6d588d994B1f956571A31e677632a3C27FEd",
				"name": "The Millionare Bunker",
				"decimals": 9
			},
			"0xd17fe45550d833d82b65afa094fa51318a6c3fd1": {
				"ticker": "MILF2",
				"address": "0xd17Fe45550D833d82B65afa094fA51318A6c3FD1",
				"name": "Metaverse Investment Legacy Fund V2",
				"decimals": 9
			},
			"0xcb0e0087bfc1e4d7b294a9805c9dc022cbdec2ca": {
				"ticker": "MAV",
				"address": "0xCB0E0087bFC1E4D7b294a9805C9dC022cbdEc2ca",
				"name": "MetaverseAVA",
				"decimals": 18
			},
			"0x03056b3b2ae703172eb926a65d9cb61ee36db3ee": {
				"ticker": "VAXP",
				"address": "0x03056B3B2Ae703172eb926a65D9CB61ee36DB3EE",
				"name": "AVAX PRINTER",
				"decimals": 9
			},
			"0xcdfaa96409994d8e99d5d324b32a934fded0b6e6": {
				"ticker": "SCAT",
				"address": "0xcdFaa96409994d8E99D5d324b32A934fdED0b6E6",
				"name": "SnowCat DAO",
				"decimals": 18
			},
			"0xe2d1f63eca82ddc1b63558a14e08aed3fc656ce9": {
				"ticker": "NODEON",
				"address": "0xe2d1F63EcA82dDc1b63558A14e08AeD3FC656cE9",
				"name": "NodeonProtocol",
				"decimals": 18
			},
			"0x7c13c6c236effaf3bf832cf324e7dd8722436e35": {
				"ticker": "SMN",
				"address": "0x7C13c6C236eFfaf3bf832Cf324E7Dd8722436E35",
				"name": "Smartnodes",
				"decimals": 18
			},
			"0x25b8929e107bf9a3a4f90e6d34d8ad1a35fc98d6": {
				"ticker": "STRR",
				"address": "0x25B8929E107BF9a3A4f90e6d34d8ad1A35FC98D6",
				"name": "Star Robot",
				"decimals": 11
			},
			"0x641e3e7dfa09095cc4cf61bafd9518bd58169b39": {
				"ticker": "MARS",
				"address": "0x641e3E7dfa09095CC4cF61BAfd9518bd58169B39",
				"name": "Mars King",
				"decimals": 10
			},
			"0xc82efd664ffcfac88e62a2e98ddf209d7e08338a": {
				"ticker": "ENJ",
				"address": "0xC82eFd664Ffcfac88E62A2E98DdF209d7E08338A",
				"name": "enjin",
				"decimals": 9
			},
			"0xaade6a8195c161971ebcf150522b7a0d488f3504": {
				"ticker": "VOLT",
				"address": "0xAade6a8195C161971eBCF150522B7a0d488f3504",
				"name": "Fuse Nodes",
				"decimals": 18
			},
			"0x29d9013ca3db60bcf2adfc4d6229ee7dd716adc8": {
				"ticker": "WAWE",
				"address": "0x29D9013cA3db60bcF2AdfC4d6229EE7dD716Adc8",
				"name": "Wave Money",
				"decimals": 18
			},
			"0x11f75af09db58c66bcbb36c41a6594eaab18add6": {
				"ticker": "GUNS",
				"address": "0x11f75AF09DB58c66BCBB36C41A6594EAaB18adD6",
				"name": "Gun Node Games",
				"decimals": 9
			},
			"0x70cf379e3218c22225b4fc012d593e5a0e71baa1": {
				"ticker": "FANG",
				"address": "0x70CF379e3218c22225b4fc012D593e5A0e71baA1",
				"name": "Fantasy Angel",
				"decimals": 11
			},
			"0x9a8c21d41044f69c99b9688a1fffe3e2307045a5": {
				"ticker": "SGLD",
				"address": "0x9a8C21D41044F69c99B9688A1FFFe3e2307045A5",
				"name": "Safe Gold",
				"decimals": 10
			},
			"0x7d1e8650abd5f8363d63dc7ab838cec8c726dd38": {
				"ticker": "OT-xJOE-30JUN2022",
				"address": "0x7D1e8650aBD5f8363D63Dc7AB838ceC8c726Dd38",
				"name": "OT JoeBar 30JUN2022",
				"decimals": 18
			},
			"0x43f673dede1a30399e7b9b600ad78193db00d895": {
				"ticker": "SUS",
				"address": "0x43f673dEDE1A30399E7b9b600AD78193dB00D895",
				"name": "SUSSY",
				"decimals": 9
			},
			"0x4a582dca2799d7a8554b9148fa33a3a3c3c92699": {
				"ticker": "MCDAO",
				"address": "0x4a582Dca2799d7A8554b9148Fa33A3A3C3c92699",
				"name": "Multi Chain DAO",
				"decimals": 18
			},
			"0x277a8ba403abf080dabc501964c6802acc6dd9bf": {
				"ticker": "ENC",
				"address": "0x277A8bA403aBf080DABc501964c6802ACc6dd9bF",
				"name": "ENC",
				"decimals": 18
			},
			"0xe83ce6bfb580583bd6a62b4be7b34fc25f02910d": {
				"ticker": "AVABBC",
				"address": "0xe83cE6bfb580583bd6A62B4Be7b34fC25F02910D",
				"name": "Avalanche ABBC",
				"decimals": 8
			},
			"0x8193725302f4e183e9073a3f51a2bc5b4dbcffd3": {
				"ticker": "HUNT",
				"address": "0x8193725302F4E183e9073A3F51a2bc5B4dbCFFd3",
				"name": "Hunt Protocol",
				"decimals": 18
			},
			"0x1cd6afa33abd451f216287b6f81dc65895280b87": {
				"ticker": "ANG",
				"address": "0x1Cd6AfA33aBd451f216287b6F81DC65895280B87",
				"name": "Safe Angel",
				"decimals": 11
			},
			"0x186b8bd6cfa6e0aaf9f6fe6df71bf58824c6e339": {
				"ticker": "bullUSD",
				"address": "0x186B8Bd6Cfa6e0aaF9F6fe6Df71Bf58824C6E339",
				"name": "RagingBull Finance",
				"decimals": 18
			},
			"0xf8a29c274c94053fd4218784422489ee7d41ec19": {
				"ticker": "KANS",
				"address": "0xf8A29C274C94053FD4218784422489Ee7d41eC19",
				"name": "Kansoru Compiler",
				"decimals": 8
			},
			"0xfe0da90f3e8fc5e373c90d34e57f417a834ff177": {
				"ticker": "MCD",
				"address": "0xfe0dA90F3e8Fc5E373c90d34E57f417A834Ff177",
				"name": "Multi Chain Dao",
				"decimals": 9
			},
			"0x6e867aed13e82537b198ade269fbea02e01f5783": {
				"ticker": "EVB",
				"address": "0x6e867aEd13E82537B198aDe269fBea02E01f5783",
				"name": "EverBurn",
				"decimals": 6
			},
			"0x7879017371f5529ecb7026d4803b7a201c5cacc7": {
				"ticker": "🥚",
				"address": "0x7879017371f5529ecB7026D4803B7A201C5CaCC7",
				"name": "GoldenEgg",
				"decimals": 9
			},
			"0x4a6f91081e6d12a2221d0316375c4be7c5b2600b": {
				"ticker": "PIGGY",
				"address": "0x4A6F91081E6d12A2221d0316375c4bE7c5b2600b",
				"name": "Piggy",
				"decimals": 18
			},
			"0x4ab18d380e6a8f0b99a20900270191f788e79d15": {
				"ticker": "BYBT",
				"address": "0x4Ab18D380e6A8F0B99A20900270191F788E79d15",
				"name": "BYBIT",
				"decimals": 9
			},
			"0x2c03c6e83031ff6abd7bcabb3642e88a0bef1b7e": {
				"ticker": "Plutonium",
				"address": "0x2c03C6e83031fF6Abd7bcAbb3642e88a0Bef1B7e",
				"name": "Plutonium Node",
				"decimals": 18
			},
			"0xed0bcd401d46cb35ee391839ad16f5214c14fd50": {
				"ticker": "SSH",
				"address": "0xEd0bcD401D46cb35EE391839Ad16f5214c14FD50",
				"name": "Starship Super Heavy",
				"decimals": 9
			},
			"0x5e71ea25366ad3267cb8b27c2f00f2a59a0f4d09": {
				"ticker": "MNT",
				"address": "0x5E71Ea25366AD3267cB8B27c2f00F2a59a0F4d09",
				"name": "Mount",
				"decimals": 18
			},
			"0xeec60b14fe18988e4aaffadd88ffcab511323f26": {
				"ticker": "MAG",
				"address": "0xEec60b14fe18988e4aaFfadD88ffCab511323F26",
				"name": "Magnum DAO",
				"decimals": 9
			},
			"0x8f912d6b9539e4ff2319c57a90370c6f11a888c8": {
				"ticker": "HCH",
				"address": "0x8f912d6b9539E4FF2319c57A90370C6f11A888c8",
				"name": "Meta Hachi",
				"decimals": 12
			},
			"0xdc669748fad279d55d471e524dc3dedf259f1b68": {
				"ticker": "WAWE",
				"address": "0xDc669748FAD279D55D471e524DC3DEDf259f1b68",
				"name": "Wave Money",
				"decimals": 18
			},
			"0x88f8c0b239d06d0b118529d1790e7376f5ee2e16": {
				"ticker": "MAKERSHARE",
				"address": "0x88F8c0B239D06D0B118529D1790e7376f5ee2E16",
				"name": "MAKERSHARE",
				"decimals": 18
			},
			"0x5fb30a43265d1aed52da79dbdc7f317e0c84bef1": {
				"ticker": "KERMIT",
				"address": "0x5FB30A43265D1aED52DA79DBDC7f317E0c84BEF1",
				"name": "KERMIT Token",
				"decimals": 9
			},
			"0xfb0dde259929bc4af8370ab087b5cf3a36a18b42": {
				"ticker": "SKADIDAO",
				"address": "0xFB0dDe259929Bc4AF8370Ab087B5Cf3a36A18b42",
				"name": "SkadiDAO",
				"decimals": 18
			},
			"0x9fda7ceec4c18008096c2fe2b85f05dc300f94d0": {
				"ticker": "MFI",
				"address": "0x9Fda7cEeC4c18008096C2fE2B85F05dc300F94d0",
				"name": "MarginSwap",
				"decimals": 18
			},
			"0xd4fb68b1b161a155d38b7b8250f824fcb4f0f021": {
				"ticker": "PDAT",
				"address": "0xD4Fb68B1b161A155D38b7B8250f824fcb4f0F021",
				"name": "Panda Treasure",
				"decimals": 9
			},
			"0xc5e7520432ebdbf0a610287ac45fc231876f5e99": {
				"ticker": "YEL",
				"address": "0xc5E7520432EbDBf0a610287Ac45fc231876f5E99",
				"name": "YEL Token",
				"decimals": 18
			},
			"0xa8928950083d16fcaf89557256acb2879c07804a": {
				"ticker": "Nexus",
				"address": "0xa8928950083d16FCAF89557256aCb2879c07804a",
				"name": "Nexus Node",
				"decimals": 18
			},
			"0xa5565429f3ec4a429fa495329968556c3beffcf0": {
				"ticker": "ENRGY",
				"address": "0xa5565429f3ec4A429Fa495329968556c3beFFCF0",
				"name": "EnergyNodes",
				"decimals": 18
			},
			"0xeb3ca030ea5a15f2943d2ee027253e096844ed9e": {
				"ticker": "AmethystNODES",
				"address": "0xEB3cA030ea5A15F2943D2Ee027253E096844Ed9E",
				"name": "AmethystNODES",
				"decimals": 18
			},
			"0x0f5bbf61566a53a23c3fdff55f146c1317d2a601": {
				"ticker": "Frog",
				"address": "0x0F5bBf61566A53A23c3FdFF55f146C1317D2a601",
				"name": "FrogBank",
				"decimals": 9
			},
			"0xd2764d5608798610006a2a6505ea1fc0bea94825": {
				"ticker": "NEON",
				"address": "0xD2764d5608798610006A2a6505ea1fC0BeA94825",
				"name": "NodeonProtocol",
				"decimals": 18
			},
			"0x0fbbc880d13aed639db2c235b064d221b98731d6": {
				"ticker": "123",
				"address": "0x0FbBC880d13aed639db2C235b064d221b98731D6",
				"name": "123",
				"decimals": 18
			},
			"0x9668dad01dc9439875c8237e303a78189f50f71a": {
				"ticker": "WAWE",
				"address": "0x9668dAd01Dc9439875C8237e303A78189F50f71A",
				"name": " (Wormhole)",
				"decimals": 9
			},
			"0xab2712b217f0015b602c06e4fb66b8cf8b04f894": {
				"ticker": "TORTUGA",
				"address": "0xaB2712b217F0015B602C06E4fb66B8cf8B04F894",
				"name": "Wave",
				"decimals": 9
			},
			"0xe7400aea70fa73e352712a5c8d4657d45fa31134": {
				"ticker": "NODEON",
				"address": "0xe7400aEA70fa73E352712a5c8D4657D45fA31134",
				"name": "TortugaCoin",
				"decimals": 9
			},
			"0x828369e5a1c45186e51f3f77cf0dc1af3661c35d": {
				"ticker": "XAMP",
				"address": "0x828369E5A1c45186E51f3f77cF0DC1af3661C35D",
				"name": "NodeonProtocol",
				"decimals": 9
			},
			"0x690b833708523e1089afb66cc778a25d4df3f9a6": {
				"ticker": "BOOTS",
				"address": "0x690b833708523E1089AFB66cC778a25d4dF3f9a6",
				"name": "Antiample",
				"decimals": 9
			},
			"0xfcaa5f6d650be83c1e4a6e41ddf27c9b64dc76f0": {
				"ticker": "VTLK",
				"address": "0xFcAa5F6D650be83C1e4a6e41ddF27C9b64DC76f0",
				"name": "Boots Inu",
				"decimals": 18
			},
			"0x3ab94ed042ffeaa5216857ea64ce4eeafcc72ced": {
				"ticker": "CANDY",
				"address": "0x3aB94ed042ffEAA5216857eA64ce4EEafcC72cED",
				"name": "Vitalik Nodes",
				"decimals": 9
			},
			"0x7254000925e19d9bef3b156e9b0adc24c9761e0e": {
				"ticker": "KEEN",
				"address": "0x7254000925E19d9bEF3B156E9b0ADC24C9761E0E",
				"name": "CandyLand",
				"decimals": 18
			},
			"0x181493c58fc489dcf250ccc286a202b0c92b3743": {
				"ticker": "SHIB",
				"address": "0x181493C58Fc489dCF250ccc286A202b0C92b3743",
				"name": "KEEN Token",
				"decimals": 10
			},
			"0x6ced265935e65cf7608616047d6d5635522bee47": {
				"ticker": "KAI",
				"address": "0x6cEd265935e65cF7608616047d6d5635522beE47",
				"name": "Shiba AI",
				"decimals": 9
			},
			"0xb8017f211c404111ef6c94fc7fdd921e4d7702d8": {
				"ticker": "ACK",
				"address": "0xb8017F211c404111eF6c94fc7FDd921E4d7702D8",
				"name": "Cobra Kai Nodes",
				"decimals": 9
			},
			"0x39a13dc1d972a441e9b0bc22654f296084dc11f8": {
				"ticker": "DDAO",
				"address": "0x39A13dc1D972A441e9b0BC22654F296084dC11f8",
				"name": "ALT Coin Kings",
				"decimals": 9
			},
			"0xc69f85ae621417494bce07853c9939eb34b7ef90": {
				"ticker": "Rubi",
				"address": "0xC69f85AE621417494bcE07853c9939Eb34b7Ef90",
				"name": "Diamond DAO",
				"decimals": 8
			},
			"0x1b06ba6087223541ae14306d622aaf6ce8cb058c": {
				"ticker": "CORES",
				"address": "0x1B06bA6087223541aE14306d622Aaf6CE8cb058c",
				"name": "Rubic Cube",
				"decimals": 18
			},
			"0x6a6fae2d14a872d2b040f880d0ecdae11c6c3033": {
				"ticker": "SnowMan",
				"address": "0x6A6fae2d14a872D2b040F880d0ECdaE11c6c3033",
				"name": "Cores Nodes",
				"decimals": 9
			},
			"0x3ce909540f28230cbdcf976eba2654399eeb2837": {
				"ticker": "KANDY",
				"address": "0x3ce909540f28230CBDcf976Eba2654399eeB2837",
				"name": "SnowMan",
				"decimals": 18
			},
			"0xe280d014e763958130d733c87a5e770f98e40a8e": {
				"ticker": "GSHARE",
				"address": "0xe280D014e763958130D733c87A5e770F98E40A8E",
				"name": "Kandyland DAO",
				"decimals": 1
			},
			"0xea6887e4a9cda1b77e70129e5fba830cdb5cddef": {
				"ticker": "IMX.a",
				"address": "0xeA6887e4a9CdA1B77E70129E5Fba830CdB5cdDef",
				"name": "Grape Finance",
				"decimals": 18
			},
			"0x102191d329374da8e6370f4ca9e29fb77646e447": {
				"ticker": "SKADIDAO",
				"address": "0x102191D329374da8e6370F4Ca9E29Fb77646E447",
				"name": "IMX@avalanche",
				"decimals": 18
			},
			"0x52fb2186357a636e339cd4fb620d38dee258b8a4": {
				"ticker": "RGR",
				"address": "0x52fB2186357A636E339CD4FB620D38DeE258b8a4",
				"name": "Skadi DAO",
				"decimals": 18
			},
			"0x626a3ca80bec3d430a40323d691cef11e2a971b8": {
				"ticker": "VIK",
				"address": "0x626a3CA80BeC3D430A40323d691Cef11e2A971B8",
				"name": "RagnarokNODES",
				"decimals": 9
			},
			"0x96084fd213088e5a0d686fc7778f6d42974fe1d6": {
				"ticker": "MITO",
				"address": "0x96084FD213088e5A0d686Fc7778F6d42974fE1d6",
				"name": "Viking DAO",
				"decimals": 9
			},
			"0x27fe10e8f873caa711dcfa13b4138e461ada116e": {
				"ticker": "PXT",
				"address": "0x27FE10e8f873cAA711DCfA13B4138E461aDa116e",
				"name": "MUSK ITO",
				"decimals": 1
			},
			"0xf74c3c211f1d740d55d391381f8a4ee768bffa1e": {
				"ticker": "SKADI",
				"address": "0xF74C3C211f1d740D55D391381F8a4ee768BffA1e",
				"name": "Project X Financial",
				"decimals": 18
			},
			"0x431bdc9975d570da5ed69c4e97e27114bcd55a86": {
				"ticker": "ZOMBIE",
				"address": "0x431bDC9975D570da5eD69C4E97e27114BCd55a86",
				"name": "Skadi DAO",
				"decimals": 18
			},
			"0xdd600ede58b44c6295f1ac1ac3a60ec6063aad7a": {
				"ticker": "BLBT",
				"address": "0xDD600EdE58b44C6295f1AC1Ac3a60EC6063AAd7a",
				"name": "zombie.finance",
				"decimals": 9
			},
			"0x4ba976764325158bbb64f730005e9ecc7f3cf1bd": {
				"ticker": "AWL",
				"address": "0x4bA976764325158bBB64F730005E9eCC7F3cF1Bd",
				"name": "Block Bots",
				"decimals": 9
			},
			"0xe9151e4942e97a50f1b98e51f4592f9219aaf660": {
				"ticker": "Prosper",
				"address": "0xe9151e4942e97A50f1B98E51f4592F9219AaF660",
				"name": "AVAXWOLF https://t.me/AVAXWOLF1",
				"decimals": 18
			},
			"0x5aa2ff4ab706307d8b3d90a462c1ddc055655734": {
				"ticker": "NeBu",
				"address": "0x5AA2Ff4Ab706307d8B3D90A462c1ddC055655734",
				"name": "Prosperity",
				"decimals": 18
			},
			"0xf007c50335cf01222f02822677057f304065c5af": {
				"ticker": "STR",
				"address": "0xF007C50335CF01222F02822677057f304065C5aF",
				"name": "NebulaNodes",
				"decimals": 12
			},
			"0x0231d3faac61e7c127da299ab8034c57cdc8de8c": {
				"ticker": "STR",
				"address": "0x0231D3FAaC61e7c127Da299AB8034C57cdc8dE8c",
				"name": "Star Farm",
				"decimals": 10
			},
			"0x938fe3788222a74924e062120e7bfac829c719fb": {
				"ticker": "APEIN",
				"address": "0x938FE3788222A74924E062120E7BFac829c719Fb",
				"name": "Star DAO",
				"decimals": 18
			},
			"0x38f7c43a5af07e64964344a4d0ed16229a3fde8e": {
				"ticker": "MAGMA",
				"address": "0x38f7C43A5AF07e64964344a4d0ed16229A3fDE8E",
				"name": "Ape In",
				"decimals": 9
			},
			"0x1c487606344aeecc46f9f549707cb38b97977a0b": {
				"ticker": "aNEON",
				"address": "0x1c487606344aEECc46F9F549707Cb38B97977A0B",
				"name": "MAGMA Printer",
				"decimals": 18
			},
			"0xaedf7f0b2b979e2a13bfa4b07b6ea6ade3c3521c": {
				"ticker": "EVO",
				"address": "0xAEdF7f0b2B979e2A13bFA4B07b6EA6aDE3C3521C",
				"name": "Neon Nodes",
				"decimals": 9
			},
			"0xf8e944da1190704b810c90ad93b2d46574c68c1d": {
				"ticker": "BinX",
				"address": "0xf8e944Da1190704b810C90AD93B2D46574c68c1D",
				"name": "EVO Protocol",
				"decimals": 9
			},
			"0x717c6c975b375bd0f08717a0f4c97e1309b681a0": {
				"ticker": "ELON",
				"address": "0x717C6c975B375bD0f08717a0f4C97e1309B681A0",
				"name": "BinaryX",
				"decimals": 11
			},
			"0xd4d026322c88c2d49942a75dff920fcfbc5614c1": {
				"ticker": "DEP",
				"address": "0xD4d026322C88C2d49942A75DfF920FCfbC5614C1",
				"name": "Elon Treasure",
				"decimals": 18
			},
			"0xe1ddb18fe0133c9bcaa106e23616911132dec944": {
				"ticker": "EPAN",
				"address": "0xe1ddb18fE0133C9bcAA106E23616911132dEc944",
				"name": "DEAPCOIN",
				"decimals": 10
			},
			"0xf9b9053fb647eceb6bc46cd6ed74ebb9e3d0577d": {
				"ticker": "AVN",
				"address": "0xF9b9053FB647EcEB6Bc46Cd6ed74eBb9E3D0577D",
				"name": "Exo Panda",
				"decimals": 18
			},
			"0xa79916d5da371227f6ee0b5d0c69cc5f24ede28c": {
				"ticker": "WHITE",
				"address": "0xA79916D5dA371227F6eE0B5D0c69cC5F24eDe28c",
				"name": "Avangardium",
				"decimals": 9
			},
			"0xae9d2385ff2e2951dd4fa061e74c4d3dedd24347": {
				"ticker": "TOK",
				"address": "0xae9d2385Ff2E2951Dd4fA061e74c4d3deDD24347",
				"name": "WhiteHatDAO",
				"decimals": 8
			},
			"0x5feea815559a59cacf9017163b93a588475e609c": {
				"ticker": "MFAN",
				"address": "0x5fEEa815559A59cAcf9017163b93A588475E609C",
				"name": "TOK",
				"decimals": 10
			},
			"0x8bba53541b97a364609f0340ae8292720098cf21": {
				"ticker": "LEMON",
				"address": "0x8bBA53541b97a364609f0340Ae8292720098CF21",
				"name": "Mini Fantom",
				"decimals": 9
			},
			"0x69afdba5a7fec26ba2f0a3f42962b53ec5da59a3": {
				"ticker": "XAVABC",
				"address": "0x69afdBA5A7fec26bA2f0A3f42962b53Ec5DA59a3",
				"name": "Inu Safu Lemon",
				"decimals": 18
			},
			"0x0f34919404a290e71fc6a510cb4a6acb8d764b24": {
				"ticker": "BLZZ",
				"address": "0x0f34919404a290e71fc6A510cB4a6aCb8D764b24",
				"name": "Avalanche Blue Chip Index",
				"decimals": 18
			},
			"0x7be4ae06e4d0dbffa81063a5c5d07ac1ba1622e1": {
				"ticker": "MILK",
				"address": "0x7BE4ae06E4D0dBfFa81063A5C5d07Ac1Ba1622e1",
				"name": "Blizz.Finance Protocol Token",
				"decimals": 18
			},
			"0x612b6f08d7a149a668ef9ab16ea3147969b2e85b": {
				"ticker": "ORO",
				"address": "0x612B6F08d7A149a668EF9AB16EA3147969B2e85B",
				"name": "Dairy Money",
				"decimals": 18
			},
			"0x5aa93da158fcbd86d09d54951589e6663b1c845a": {
				"ticker": "BFT",
				"address": "0x5aa93Da158fcBD86d09d54951589E6663b1c845a",
				"name": "ORO Token",
				"decimals": 18
			},
			"0x862136e2ac00bd68a21c068b70ad9d53e0b75f13": {
				"ticker": "WDAO",
				"address": "0x862136E2AC00BD68A21c068B70ad9d53e0b75f13",
				"name": "BABY FISHINGTOWN",
				"decimals": 18
			},
			"0x2efb4fec95deaab8c199ca5089e0cfd52373f92d": {
				"ticker": "CRND",
				"address": "0x2EfB4FEc95DeAAb8c199CA5089E0cFD52373f92D",
				"name": "Tungsten DAO",
				"decimals": 18
			},
			"0x542931f51439af42661bdd8abcd476eaa632a38f": {
				"ticker": "MIMPay",
				"address": "0x542931f51439Af42661bDD8abCd476Eaa632A38f",
				"name": "CryptoNodes Coin",
				"decimals": 9
			},
			"0xd6c5bfa9feea5579498ea4b04fff86a9eb3a1a9d": {
				"ticker": "WOHM",
				"address": "0xd6C5BFa9FeEA5579498EA4b04Fff86A9eB3A1a9d",
				"name": "MIMPay",
				"decimals": 9
			},
			"0x009c582242fc906228d4a71a1c497c6a48676935": {
				"ticker": "ZTR",
				"address": "0x009c582242fc906228D4a71a1c497c6A48676935",
				"name": "WeedOhm",
				"decimals": 9
			},
			"0xa46b907d53fd30a6b9db0d8179d68590ebbb86d0": {
				"ticker": "CYBORG",
				"address": "0xA46b907D53Fd30A6B9dB0D8179D68590EBBb86d0",
				"name": "ZeroTaxRocket",
				"decimals": 2
			},
			"0x1744c349453e805a97283102d17a571f9e4eb7b3": {
				"ticker": "ROCK",
				"address": "0x1744c349453E805a97283102d17A571f9e4eB7b3",
				"name": "CYBORG",
				"decimals": 9
			},
			"0x9cb0827f399e02f4f40a383314b6650737cdd273": {
				"ticker": "RAIN",
				"address": "0x9CB0827F399E02f4f40A383314B6650737cDD273",
				"name": "Rock Project",
				"decimals": 18
			},
			"0xe49565fb6ac188bc1214f3ba3e0e074d831f04fc": {
				"ticker": "NAKA",
				"address": "0xE49565fb6aC188bc1214F3bA3e0E074d831f04Fc",
				"name": "Monsoon DAO",
				"decimals": 9
			},
			"0x4c9993c7107495020c2ce9a13d11839f48ecd2e6": {
				"ticker": "VOLTa",
				"address": "0x4C9993C7107495020c2Ce9A13d11839F48EcD2E6",
				"name": "Satoshi Nakamoto",
				"decimals": 9
			},
			"0x0a2a869bdd01a10ee844cdaa01b2cca33ce50a86": {
				"ticker": "GOL",
				"address": "0x0a2a869Bdd01a10EE844CDAA01B2CCA33ce50a86",
				"name": "Volta",
				"decimals": 10
			},
			"0x221caccd55f16b5176e14c0e9dbaf9c6807c83c9": {
				"ticker": "PGL",
				"address": "0x221Caccd55F16B5176e14C0e9DBaF9C6807c83c9",
				"name": "Sushi Gold",
				"decimals": 18
			},
			"0x5933a34abb305c1af377774ffbbac665938ff1a6": {
				"ticker": "WAWE",
				"address": "0x5933A34Abb305C1aF377774FFbbAC665938Ff1A6",
				"name": "Pangolin Liquidity",
				"decimals": 9
			},
			"0xd60a3a40114be1000afcf2caf45d835760eec2cc": {
				"ticker": "PLANCK",
				"address": "0xd60A3A40114BE1000AFcF2cAF45D835760eEc2cc",
				"name": "Wave",
				"decimals": 18
			},
			"0x361b1afc9b461241b443ce1db5a712d5d0345f85": {
				"ticker": "PKBK",
				"address": "0x361B1AFc9b461241B443ce1Db5a712D5D0345f85",
				"name": "Quantum Planck",
				"decimals": 9
			},
			"0x4e0dd45d7e79ddc823be41cc594be3e599a0e3b6": {
				"ticker": "CoinGecko",
				"address": "0x4E0dd45d7E79dDC823be41cc594Be3E599A0E3B6",
				"name": "Punk Blocks",
				"decimals": 18
			},
			"0xc91662677519a0b5b812d54180364c956714a357": {
				"ticker": "PIGGY",
				"address": "0xc91662677519a0b5B812D54180364c956714A357",
				"name": "CoinGecko.com",
				"decimals": 18
			},
			"0x54bc56ea02ecfc858fe8aa790954015b6a2b2977": {
				"ticker": "TIBET",
				"address": "0x54bC56ea02eCfC858fe8AA790954015B6a2b2977",
				"name": "Piggy Finance",
				"decimals": 9
			},
			"0xcd54d1d7880bb244dc7bdaf189d24ce5e53c2115": {
				"ticker": "BSHARE",
				"address": "0xCd54D1d7880bb244dC7bdAF189D24cE5E53c2115",
				"name": "AutoOHM",
				"decimals": 9
			},
			"0xe2e308baee1592ac66373473d6ff0086de2894ac": {
				"ticker": "ROYALE",
				"address": "0xe2e308Baee1592Ac66373473D6fF0086dE2894Ac",
				"name": "BSHARE",
				"decimals": 8
			},
			"0x9a5b35bfe661114f2d1ba02a81fa7192bf880129": {
				"ticker": "STAX",
				"address": "0x9a5b35BfE661114f2D1ba02a81Fa7192bF880129",
				"name": "Royale Token",
				"decimals": 9
			},
			"0x39aa4a3ed26f47a1abe31de8a3722834e94aa6a5": {
				"ticker": "FLY",
				"address": "0x39aA4a3eD26f47a1Abe31De8a3722834E94aa6a5",
				"name": "STAX DAO",
				"decimals": 9
			},
			"0x7b265be08d603905000d5c69014ae5b067dae5e9": {
				"ticker": "BRUSH",
				"address": "0x7B265be08D603905000d5C69014aE5b067dae5E9",
				"name": "FLY",
				"decimals": 9
			},
			"0x2f38ea64ba7e6438ed14f33d5a7e066565c87e65": {
				"ticker": "Solution",
				"address": "0x2F38Ea64ba7e6438eD14F33d5a7e066565c87e65",
				"name": "Paint Swap Token",
				"decimals": 9
			},
			"0x5234e7206131f7e122e8c2cdd6536d1b0510e871": {
				"ticker": "MND",
				"address": "0x5234E7206131f7e122e8C2CDd6536D1B0510e871",
				"name": "Solution",
				"decimals": 18
			},
			"0xd7c1687d574345a4158e52d79da029d88f4a2850": {
				"ticker": "RFI",
				"address": "0xD7C1687d574345a4158e52d79da029D88F4a2850",
				"name": "METAnode",
				"decimals": 18
			},
			"0x779595f212b53e4f1f51b0875405bf21b5e4d805": {
				"ticker": "AVOGE",
				"address": "0x779595F212B53e4f1F51b0875405Bf21B5e4D805",
				"name": "Remote.Finance",
				"decimals": 6
			},
			"0x3f8f5674dcd165a5e3f19da75f9ddf62eeed906b": {
				"ticker": "AVAINU",
				"address": "0x3F8f5674DCd165a5E3F19DA75f9Ddf62EEEd906b",
				"name": "Avoge",
				"decimals": 18
			},
			"0x82fe038ea4b50f9c957da326c412ebd73462077c": {
				"ticker": "HAT",
				"address": "0x82FE038Ea4b50f9C957da326C412ebd73462077C",
				"name": "Ava Inu",
				"decimals": 18
			},
			"0x3963f4b732a1c37ca989266a2a55fbabadccad8f": {
				"ticker": "spy",
				"address": "0x3963f4B732A1C37ca989266a2a55FbaBAdCCad8f",
				"name": "Joe Hat Token",
				"decimals": 9
			},
			"0x83dc7f38f94c5d0e0dc3695330367acb2a764907": {
				"ticker": "BEAR",
				"address": "0x83dC7F38F94c5D0e0dC3695330367acb2a764907",
				"name": "SPreadY",
				"decimals": 18
			},
			"0x353b12cba686fb1cbb91213a90518270914a252b": {
				"ticker": "THN",
				"address": "0x353B12CBa686fb1cbB91213a90518270914a252B",
				"name": "BEAR Token",
				"decimals": 11
			},
			"0x0fc468c8e2003c0e6ab0e60dbf02b01ce27b4c7f": {
				"ticker": "nRYU",
				"address": "0x0FC468c8E2003C0e6Ab0e60DBf02b01ce27B4c7f",
				"name": "King Thunder",
				"decimals": 18
			},
			"0x9734a1d0fadfc4ac2c2c5fc2f6a6190f5b9ce7c6": {
				"ticker": "MOUSE",
				"address": "0x9734a1d0FAdfC4Ac2c2c5fc2f6A6190f5B9Ce7c6",
				"name": "nRYU",
				"decimals": 18
			},
			"0xb5dd5eb826f5557d819d2fab042bcabc820a3282": {
				"ticker": "MGC",
				"address": "0xB5dD5EB826f5557d819D2fAB042BcaBC820a3282",
				"name": "Mouse Token",
				"decimals": 9
			},
			"0xff579d6259dedcc80488c9b89d2820bcb5609160": {
				"ticker": "LVT",
				"address": "0xff579d6259dEDcc80488c9b89d2820bCb5609160",
				"name": "Magic Carpet",
				"decimals": 18
			},
			"0xd169689f1363c2d367c357ff792f1ac0820d0af4": {
				"ticker": "STR",
				"address": "0xd169689F1363c2D367c357Ff792F1AC0820D0Af4",
				"name": "Louverture",
				"decimals": 10
			},
			"0x61dad5acbf61e77cf1ccac3f3e2da502b8b8cb73": {
				"ticker": "0xB",
				"address": "0x61dAd5ACBf61e77CF1CcAC3f3E2dA502B8B8cB73",
				"name": "Wrapped Star",
				"decimals": 18
			},
			"0xeee27d70d84b868e7b8313cdfb203ff073175592": {
				"ticker": "HAPE",
				"address": "0xeeE27D70d84b868e7B8313cdFb203fF073175592",
				"name": "0xBlock",
				"decimals": 9
			},
			"0x7249653ed19912492ebd0d06c9398c192ec22170": {
				"ticker": "SNOWY",
				"address": "0x7249653Ed19912492EBd0D06c9398c192ec22170",
				"name": "Fantastic Protocol SNOWY Token",
				"decimals": 18
			},
			"0xa88f6abaffd300f3e6d702afb8c5dab0d0517ec4": {
				"ticker": "ARC",
				"address": "0xA88F6AbAffd300F3e6d702afB8C5DAB0D0517EC4",
				"name": "Arcade",
				"decimals": 9
			},
			"0xfe11cd7fc8ea847afeaffcffd0667077fbb70d28": {
				"ticker": "MIAP",
				"address": "0xFe11cD7fC8Ea847aFeAfFCFFd0667077Fbb70d28",
				"name": "MagicInteretAP",
				"decimals": 9
			},
			"0xaf0fbc65cf439e58580fc9d80e55871bc2db0427": {
				"ticker": "ATOM",
				"address": "0xAf0FBC65cf439E58580fc9d80E55871bc2dB0427",
				"name": "Galaxyo",
				"decimals": 8
			},
			"0xb32ebf0f1d4d070e351d5a30c2137c0e12989ac6": {
				"ticker": "WETHP",
				"address": "0xB32EBf0F1d4d070E351d5a30c2137C0E12989aC6",
				"name": "WETH PRINT",
				"decimals": 18
			},
			"0x42d047534eef46fd68fad3d74726fe51be4eeb8f": {
				"ticker": "$CAESAR",
				"address": "0x42d047534eef46FD68fAD3d74726Fe51be4eeb8F",
				"name": "Caesar",
				"decimals": 18
			},
			"0x860a0d7fa931fa20aaccae02315fe2dde8995f93": {
				"ticker": "KIWI",
				"address": "0x860A0D7Fa931FA20AaCCAe02315fe2DDe8995F93",
				"name": "KiwiToken",
				"decimals": 18
			},
			"0x542fa0b261503333b90fe60c78f2beed16b7b7fd": {
				"ticker": "TRACTOR",
				"address": "0x542fA0B261503333B90fE60c78F2BeeD16b7b7fD",
				"name": "TRACTOR JOE",
				"decimals": 9
			},
			"0x43b6188384878363b41f524d807b6ed5882156a9": {
				"ticker": "FVP",
				"address": "0x43b6188384878363B41F524d807B6eD5882156A9",
				"name": "FisherPirate",
				"decimals": 9
			},
			"0xe328b3e90e9a72c819b7fa596833676f520fa144": {
				"ticker": "OPEC",
				"address": "0xe328b3e90E9a72C819b7fa596833676F520Fa144",
				"name": "Opulence Nodes",
				"decimals": 18
			},
			"0xc98a4e8a9c561429e5d255e8cfe7be4d3d0702b8": {
				"ticker": "APX",
				"address": "0xc98A4e8A9C561429e5D255e8cFE7Be4D3D0702b8",
				"name": "Apevax",
				"decimals": 18
			},
			"0xb2a940feef9e45f4620ed6e4c5d3369cf281d84a": {
				"ticker": "WIPEOUT",
				"address": "0xb2A940fEef9e45f4620Ed6e4c5D3369Cf281d84A",
				"name": "WIPEOUT",
				"decimals": 2
			},
			"0x3a01acedd8a8f70fa1f0913b2fc32a2137637cbf": {
				"ticker": "OXY",
				"address": "0x3a01AceDd8a8f70fa1f0913b2fc32A2137637CbF",
				"name": "Oxy FI",
				"decimals": 18
			},
			"0xbc62be5f1b315a0774469bf913b33c39b17654f1": {
				"ticker": "TRINITY",
				"address": "0xbC62Be5f1b315a0774469bf913B33c39b17654F1",
				"name": "TRINITY",
				"decimals": 8
			},
			"0x2b26b2de08cd824692667aa3972ab21de35316c9": {
				"ticker": "WAWE",
				"address": "0x2B26B2De08Cd824692667AA3972aB21De35316c9",
				"name": "Wave",
				"decimals": 9
			},
			"0xf32bb17ed40cca11bbc68034f9e6c9f409e9c354": {
				"ticker": "MIAP",
				"address": "0xf32Bb17ed40cca11bbC68034f9e6c9F409e9C354",
				"name": "MagicInternetAlgorithmicPrinter",
				"decimals": 18
			},
			"0xd4e2b919a2992b99a4991c310d6cb9d83526ca8c": {
				"ticker": "TECH",
				"address": "0xD4e2b919a2992b99A4991C310D6CB9d83526ca8c",
				"name": "Technical Nodes",
				"decimals": 18
			},
			"0x564ae3c8adb5f07648982c85f2c94a3b339eab50": {
				"ticker": "Mercury",
				"address": "0x564AE3C8adb5f07648982c85f2C94a3B339eAB50",
				"name": "Mercury Financial",
				"decimals": 18
			},
			"0xa72eab7327e48bde0ff873eaf1d4fecf6036504f": {
				"ticker": "GHOST",
				"address": "0xA72eAb7327e48bdE0FF873eAF1D4FecF6036504F",
				"name": "Ghost",
				"decimals": 9
			},
			"0xb80323c7aa915cb960b19b5cca1d88a2132f7bd1": {
				"ticker": "NADO",
				"address": "0xB80323c7aA915CB960b19B5cCa1D88a2132F7BD1",
				"name": "Nado",
				"decimals": 9
			},
			"0x202b6489581fbf5a465cd277c8811ec86be37c45": {
				"ticker": "CHA",
				"address": "0x202B6489581fbF5a465CD277C8811Ec86be37c45",
				"name": "Chica INU",
				"decimals": 9
			},
			"0xd9ff12172803c072a36785defea1aa981a6a0c18": {
				"ticker": "FUDGE",
				"address": "0xD9FF12172803c072a36785DeFea1Aa981A6A0C18",
				"name": "Sundae Finance",
				"decimals": 18
			},
			"0x0cf04c18dad9583c6c75e6c433c6ae687624a5c8": {
				"ticker": "TAI",
				"address": "0x0cf04C18DaD9583C6C75e6c433C6aE687624A5C8",
				"name": "TheAvaxIsland",
				"decimals": 18
			},
			"0x9f05bf860f7015507513dafca8f5ea61ee9f6c4e": {
				"ticker": "Ava",
				"address": "0x9F05Bf860F7015507513daFCA8f5ea61EE9F6c4E",
				"name": "Avax Floki (https://t.me/avaxfloki)",
				"decimals": 9
			},
			"0x22c6fb0d733c2d2acfa4a0ee159cf3b93319e07d": {
				"ticker": "KRPTN",
				"address": "0x22C6FB0d733c2D2acfA4A0eE159cf3b93319e07D",
				"name": "Krypton Finance",
				"decimals": 18
			},
			"0xdd2403317aaaedc51e81327c1f0e29c5e67de58f": {
				"ticker": "BOREA",
				"address": "0xdD2403317AAAedC51e81327C1f0e29C5E67dE58F",
				"name": "BOREA Token",
				"decimals": 18
			},
			"0x1eba6ec94440b369ebf86d00be0c0ab5efdb4b92": {
				"ticker": "MAKA",
				"address": "0x1eBa6EC94440b369ebF86D00BE0c0AB5efDB4b92",
				"name": "Meta Akita",
				"decimals": 11
			},
			"0x788ae3b5d153d49f8db649aacba1857f744b739e": {
				"ticker": "KITTY",
				"address": "0x788AE3b5D153d49F8DB649aacbA1857f744b739e",
				"name": "KITTY",
				"decimals": 18
			},
			"0x576aa34ff9287863bdda76762bfab8fb04ebf163": {
				"ticker": "PESO",
				"address": "0x576AA34ff9287863bDda76762BfAb8Fb04EBf163",
				"name": "Pesos Mexican",
				"decimals": 9
			},
			"0x43261975164830ada2ee8007e4aa524ab51b12cd": {
				"ticker": "STAR",
				"address": "0x43261975164830aDa2ee8007e4aA524aB51b12CD",
				"name": "Star DAO",
				"decimals": 10
			},
			"0xcce23d7f382a6da397d1ae915f7148aeba7be51a": {
				"ticker": "SORCERER",
				"address": "0xCCE23d7F382a6dA397D1Ae915F7148AEbA7be51A",
				"name": "SORCERER",
				"decimals": 4
			},
			"0x3e7c7e88276136a69b3fb1dbae5a6b8829412ce3": {
				"ticker": "WAWE",
				"address": "0x3e7c7e88276136a69B3FB1DBAE5A6B8829412Ce3",
				"name": "Wave Money",
				"decimals": 1
			},
			"0x00f455c6cebfa50dd4769077866538b18eb01c3c": {
				"ticker": "HCHD",
				"address": "0x00F455C6CeBfA50DD4769077866538b18EB01C3C",
				"name": "Hachi DAO",
				"decimals": 9
			},
			"0x314f3bee25e49ea4bcea9a3d1321c74c95f10eab": {
				"ticker": "DUNEV3",
				"address": "0x314f3bEe25E49Ea4BCEa9A3d1321C74c95F10eab",
				"name": "DUNEV3",
				"decimals": 18
			},
			"0x8fdccf493b49c92e5b88e9f6429ddd190f333740": {
				"ticker": "SPY",
				"address": "0x8fdCCf493B49c92e5B88e9f6429dDd190f333740",
				"name": "Spy Bot",
				"decimals": 9
			},
			"0x54b74d0703aca0ab9e5b150bb4743e24f9634aff": {
				"ticker": "AXI",
				"address": "0x54B74D0703aca0Ab9E5B150BB4743e24F9634Aff",
				"name": "AVAXSUKI",
				"decimals": 18
			},
			"0x40b7193926d9e7b78e890d772fcac641fc7848bb": {
				"ticker": "4SAFU",
				"address": "0x40b7193926D9e7B78e890D772fCac641FC7848bb",
				"name": "Forever Safu",
				"decimals": 18
			},
			"0xa171629637c3266b4247551ee67ffdd3f03fb0b5": {
				"ticker": "SIN",
				"address": "0xA171629637C3266b4247551eE67ffdD3f03Fb0b5",
				"name": "Syndicate Id Number",
				"decimals": 9
			},
			"0x3fee49909969b61610c17dd76ed522270d8e8fbf": {
				"ticker": "RABBIT",
				"address": "0x3FeE49909969b61610C17dd76Ed522270d8e8FBf",
				"name": "WhiteRabbit",
				"decimals": 18
			},
			"0x84cf9c972de7bf47d5dca97f8693e45fae6712f1": {
				"ticker": "PRELUDE",
				"address": "0x84cF9c972De7bf47D5dCa97F8693E45FaE6712F1",
				"name": "PRELUDE",
				"decimals": 5
			},
			"0x0a69cccb12fe8fba67eae48ed43706afb0274ea1": {
				"ticker": "PSDN",
				"address": "0x0a69CCcB12fe8FBa67EAE48eD43706afb0274ea1",
				"name": "Poseidon Nodes",
				"decimals": 1
			},
			"0x729719dc58a1fe9f632e0619135aa9523702df7b": {
				"ticker": "APE",
				"address": "0x729719dc58A1fe9F632e0619135AA9523702df7b",
				"name": "APE Finance",
				"decimals": 9
			},
			"0x496e6f025ed7e799e43fd1babe61497e95677734": {
				"ticker": "HYBRID",
				"address": "0x496e6F025ED7e799e43Fd1BaBe61497E95677734",
				"name": "Hybrid",
				"decimals": 18
			},
			"0x96397b3ad8d4b2cc6bac57d83d2e11c808fad957": {
				"ticker": "VIZZY",
				"address": "0x96397b3AD8D4b2cc6bac57d83d2e11C808FaD957",
				"name": "Vizzy Nodes",
				"decimals": 9
			},
			"0x414a7e299839865ecad68313d7868cd355d8c6a5": {
				"ticker": "WOA",
				"address": "0x414a7E299839865ECaD68313d7868Cd355D8C6a5",
				"name": "WORLD OF APES",
				"decimals": 9
			},
			"0x5d9e6655d103e8041168b502dc3b1eb6b98743c0": {
				"ticker": "ZAPD",
				"address": "0x5d9E6655d103E8041168b502DC3B1eb6b98743c0",
				"name": "Thunder Dapp",
				"decimals": 11
			},
			"0xbfd34ec9da14e5a4afe2dd8ffe9e622d70fd0ba2": {
				"ticker": "CLUNA",
				"address": "0xbFD34ec9DA14e5A4Afe2Dd8FFe9e622d70FD0Ba2",
				"name": "New Community Luna",
				"decimals": 18
			},
			"0x054a37a40b15a7e9503583d6fd404370572faf36": {
				"ticker": "RFI",
				"address": "0x054A37A40b15A7E9503583D6fD404370572faf36",
				"name": "RICHIE-FI",
				"decimals": 6
			},
			"0x5b4e35901f2f92b6c7814d418ab15d645ff7f3ed": {
				"ticker": "Test",
				"address": "0x5B4E35901F2f92b6C7814D418AB15D645ff7f3ED",
				"name": "TestDAO",
				"decimals": 9
			},
			"0x7289eaef1f33b30929f3c4e6cdb1b64caf52632f": {
				"ticker": "PBX",
				"address": "0x7289eaEf1F33B30929F3c4e6CDB1b64cAf52632F",
				"name": "Polarbets.io V2",
				"decimals": 9
			},
			"0xe28984e1ee8d431346d32bec9ec800efb643eef4": {
				"ticker": "PGL",
				"address": "0xe28984e1EE8D431346D32BeC9Ec800Efb643eef4",
				"name": "Pangolin Liquidity",
				"decimals": 18
			},
			"0xc1333a2e6f4a50ec20107c19fa73af9e8cc0419b": {
				"ticker": "BTCP",
				"address": "0xc1333a2e6F4a50Ec20107C19fA73Af9e8cC0419B",
				"name": "BTCP",
				"decimals": 6
			},
			"0xf99a31c39a5874df6228859877251ee7230a5977": {
				"ticker": "WEB3L",
				"address": "0xF99A31C39A5874Df6228859877251EE7230a5977",
				"name": "WEB3LAND",
				"decimals": 9
			},
			"0x2b3903fe15c41a053074d4f65685d37100794921": {
				"ticker": "BOOFIM",
				"address": "0x2b3903fE15C41a053074D4F65685D37100794921",
				"name": "BOOFI MONEY",
				"decimals": 18
			},
			"0x6c8e633396a5a1dd3000b4c1979851eec4995210": {
				"ticker": "ElonNode",
				"address": "0x6c8e633396a5A1DD3000B4C1979851EEC4995210",
				"name": "ElonNodes (https://t.me/ElonNodesAvax)",
				"decimals": 18
			},
			"0xc486e13f60977aeeab2a347dd7367c512ccb1a78": {
				"ticker": "UND",
				"address": "0xc486E13F60977AEeaB2A347DD7367C512cCb1A78",
				"name": "UNINODEFork",
				"decimals": 9
			},
			"0x46dbb42bfad95a26368e047d4561e497fd6a5786": {
				"ticker": "TERRA",
				"address": "0x46dbB42bFAD95a26368e047d4561E497FD6a5786",
				"name": "TERRA PRINTER",
				"decimals": 8
			},
			"0x3605df97985e06148fa1e217abc60bc9710852b4": {
				"ticker": "CPAN",
				"address": "0x3605df97985E06148FA1e217abC60BC9710852B4",
				"name": "CryptoPlanes",
				"decimals": 9
			},
			"0x48c398e1f8409d5f6445509ada13d83a8fd4273c": {
				"ticker": "FuckPutin",
				"address": "0x48C398e1F8409D5f6445509aDa13D83A8fD4273C",
				"name": "FuckPutin",
				"decimals": 18
			},
			"0x407cd0a3ce1617e96eb5d2187bab1c4d2d401e05": {
				"ticker": "CLOUD",
				"address": "0x407Cd0A3ce1617e96Eb5D2187bAB1C4D2d401E05",
				"name": "Cloud Money",
				"decimals": 1
			},
			"0x91e72c909ca2b491d9ad39fbcadf534aeaf20e04": {
				"ticker": "MTEST",
				"address": "0x91e72c909ca2b491d9aD39fBCADF534AeAf20e04",
				"name": "MTEST",
				"decimals": 18
			},
			"0xdaa3544457a6e6e4eb59614c5045e7421b017690": {
				"ticker": "META",
				"address": "0xDAa3544457A6E6E4Eb59614c5045e7421b017690",
				"name": "Meta Floki",
				"decimals": 1
			},
			"0x06f10c2d0f765039cdc7e69243801f5c782863b9": {
				"ticker": "JOENODES",
				"address": "0x06f10c2d0F765039CDc7e69243801f5c782863B9",
				"name": "JOE NODES",
				"decimals": 18
			},
			"0xe0654b773afb824e8c075417c220ffc44e8bc614": {
				"ticker": "NEBU",
				"address": "0xE0654b773Afb824E8C075417c220ffc44e8bC614",
				"name": "Nebula",
				"decimals": 9
			},
			"0xc0367f9b1f84ca8de127226ac2a994ea4bf1e41b": {
				"ticker": "BRIDGE",
				"address": "0xC0367f9b1f84Ca8DE127226AC2A994EA4bf1e41b",
				"name": "Cross-Chain Bridge Token",
				"decimals": 18
			},
			"0x021098ed6c15e8d85e240c828ff984bc41adacd5": {
				"ticker": "MOONFIRE",
				"address": "0x021098Ed6C15E8d85E240C828fF984BC41ADaCD5",
				"name": "MoonFire",
				"decimals": 9
			},
			"0x336bfafe9c94cc5f26932d32a230730df0fdf365": {
				"ticker": "ASTRNG",
				"address": "0x336BfaFE9C94cc5f26932d32a230730Df0FdF365",
				"name": "ArmstrongNODES",
				"decimals": 18
			},
			"0x7ff0dd2ff460d975d80b5e6225ab6037c2ad8048": {
				"ticker": "DGN2",
				"address": "0x7ff0Dd2ff460d975D80b5e6225ab6037c2ad8048",
				"name": "Degen Game 2",
				"decimals": 18
			},
			"0xcca89108251b26025a4ae56226d090309904cb48": {
				"ticker": "PPLE",
				"address": "0xCcA89108251b26025a4aE56226d090309904cB48",
				"name": "OMG Pineapple",
				"decimals": 12
			},
			"0x8a694bfb30eb2f8d2fa5373efa9acfdc873f7e8d": {
				"ticker": "CryptoThug",
				"address": "0x8A694BfB30Eb2f8D2fA5373EFA9acFDC873f7e8D",
				"name": "CryptoThugs Game",
				"decimals": 8
			},
			"0x2cb6fdd8093f57ac7770906b08cc767928c87c03": {
				"ticker": "AVAXination",
				"address": "0x2Cb6fDD8093f57AC7770906B08cc767928c87c03",
				"name": "AVAXination",
				"decimals": 18
			},
			"0x9bf76f5ca6ac83285688544c566b62512351e074": {
				"ticker": "SPRINT",
				"address": "0x9Bf76f5ca6Ac83285688544c566B62512351e074",
				"name": "SPELL PRINTER ",
				"decimals": 18
			},
			"0x4c981ba326a8d1c5239a7bb5471b3c1cd10b82fb": {
				"ticker": "GUN",
				"address": "0x4c981BA326A8D1c5239A7bb5471b3c1cd10B82FB",
				"name": "Gun Finance",
				"decimals": 18
			},
			"0x2b491841c7cc6c4833902a300e359c2d667e281d": {
				"ticker": "GTD",
				"address": "0x2b491841c7Cc6c4833902A300e359C2d667E281D",
				"name": "Gold Trust Doa",
				"decimals": 9
			},
			"0x302a53503c12adaaef65426640b2d0ac0179247e": {
				"ticker": "CasperNODES",
				"address": "0x302A53503C12ADaaEF65426640B2d0Ac0179247e",
				"name": "CasperNODES",
				"decimals": 18
			},
			"0x2e9b3e53341322ee9c5a2dafff3b559f5696483b": {
				"ticker": "NITRO",
				"address": "0x2e9B3e53341322ee9C5a2dafFf3B559F5696483b",
				"name": "Nitro Nodes",
				"decimals": 9
			},
			"0x61a28dc2e1b418fc2cc2649b0d1122188a1de1b6": {
				"ticker": "Fast",
				"address": "0x61A28Dc2e1B418fc2cc2649b0D1122188a1De1b6",
				"name": "Fast Node",
				"decimals": 18
			},
			"0xf115d2e23c09ced80c823f203e6a7508a31bb69b": {
				"ticker": "SOUL",
				"address": "0xF115d2e23c09CeD80C823F203E6A7508A31BB69B",
				"name": "SoulSwap",
				"decimals": 9
			},
			"0x5278b3bd08e3531316547b3324ef4968f30bd111": {
				"ticker": "SHARK",
				"address": "0x5278B3bD08e3531316547b3324EF4968f30Bd111",
				"name": "Shark Nodes",
				"decimals": 9
			},
			"0x92c6213b5823c0ef5838d2d3863ad9e9e3385226": {
				"ticker": "KEVIN",
				"address": "0x92C6213b5823C0Ef5838D2d3863AD9E9e3385226",
				"name": "Kevin Shares",
				"decimals": 18
			},
			"0x7a7588415d2c8d5873a6e3ef78c10b0923dcc73b": {
				"ticker": "RABBIT",
				"address": "0x7a7588415D2C8D5873a6E3ef78C10b0923DcC73B",
				"name": "WhiteRabbit",
				"decimals": 18
			},
			"0xb4eff5756466773e3d1738cdbb822a8c415f161a": {
				"ticker": "MAG",
				"address": "0xb4eff5756466773E3d1738cDBB822a8c415F161a",
				"name": "Magnet DAO",
				"decimals": 18
			},
			"0x4a4910b5422335133ddbe1b605524cb54b40f799": {
				"ticker": "MTestV2",
				"address": "0x4A4910B5422335133DdbE1B605524cB54B40F799",
				"name": "MTestV2",
				"decimals": 18
			},
			"0xa886c732703700efcbd8438768822c8e716cb0ec": {
				"ticker": "WOOD",
				"address": "0xA886C732703700efCbd8438768822c8e716cB0EC",
				"name": "Fief Wood",
				"decimals": 18
			},
			"0x5a8fe7175ba6d201104d8f996787596425ad1d82": {
				"ticker": "TOF",
				"address": "0x5a8FE7175BA6d201104d8f996787596425ad1d82",
				"name": "The Option Fund",
				"decimals": 9
			},
			"0x283b99d2c58accad95e0a42c995dd124984af8c3": {
				"ticker": "EUP",
				"address": "0x283b99d2C58aCcAd95E0a42c995dD124984af8c3",
				"name": "EUPHORIA",
				"decimals": 9
			},
			"0x0aefc6a018dc98e13a85d18bd4b5675e52ee93c5": {
				"ticker": "WINGS",
				"address": "0x0aeFC6a018DC98e13a85d18Bd4B5675e52ee93C5",
				"name": "Mini Angel",
				"decimals": 11
			},
			"0x2a8489ca1d79cbef1722c14f7a2353c52642b90d": {
				"ticker": "oSNO",
				"address": "0x2A8489CA1d79cBef1722c14f7A2353C52642b90D",
				"name": "OhSnow",
				"decimals": 18
			},
			"0x9c4b666b85693610b62bfa9296f81c0f95c6c545": {
				"ticker": "PLB",
				"address": "0x9c4b666B85693610b62BfA9296F81C0F95c6C545",
				"name": "PlebiscitaDAO https://t.me/PlebiscitaDAO",
				"decimals": 18
			},
			"0x7e78493322585c6c65111b2125efbb0968a502a2": {
				"ticker": "PNRGY",
				"address": "0x7E78493322585C6C65111b2125EFbB0968A502A2",
				"name": "Wind PRINTER",
				"decimals": 18
			},
			"0x9c132b4098e8c21b354a3c4c746d70250252eba2": {
				"ticker": "JEWELP",
				"address": "0x9C132B4098e8C21B354a3C4C746D70250252eBA2",
				"name": "JEWEL PRINTER",
				"decimals": 18
			},
			"0x7cf59275756dd75cc6117fb658156b5aae19ca48": {
				"ticker": "DE",
				"address": "0x7CF59275756DD75cC6117fB658156b5aaE19ca48",
				"name": "DEX King",
				"decimals": 8
			},
			"0x535c2b3ec68a0228c667c695307f9f6f570f80e7": {
				"ticker": "Green",
				"address": "0x535c2b3EC68A0228c667C695307f9F6f570F80E7",
				"name": "Green Fields",
				"decimals": 9
			},
			"0x6d3012f55877ccbb1377cfeac7fb0d066d6f6a77": {
				"ticker": "ROCKET",
				"address": "0x6d3012F55877CCbb1377cFEaC7fb0d066D6f6A77",
				"name": "ROCKET FINANCE",
				"decimals": 18
			},
			"0xfc0716f833d121dd1c36eddf845cdf6d3b031dee": {
				"ticker": "MMC",
				"address": "0xfC0716f833D121Dd1C36eDDF845CDf6D3b031deE",
				"name": "Multi Money Capital",
				"decimals": 9
			},
			"0x873801ae2ff12d816db9a7b082f5796bec64c82c": {
				"ticker": "WTF",
				"address": "0x873801Ae2ff12d816Db9a7B082F5796BEC64C82C",
				"name": "Waterfall Governance Token",
				"decimals": 18
			},
			"0xb5e482fa78e02361cd19bf168e541c530a2e5bbc": {
				"ticker": "TOSHIBA",
				"address": "0xb5e482fa78e02361cD19Bf168E541C530a2E5BbC",
				"name": "Toasted Shiba Inu",
				"decimals": 14
			},
			"0xc807ef97e601a8a2af1c8c0a90bb6f9c46fd8c09": {
				"ticker": "TOB",
				"address": "0xC807EF97e601a8a2AF1c8C0A90BB6F9c46FD8c09",
				"name": "Tower of Babel",
				"decimals": 9
			},
			"0x41088a5bb6ecd5858f48471867fef3bc5a8490dc": {
				"ticker": "BAM",
				"address": "0x41088A5Bb6ecd5858f48471867FEF3bc5A8490dC",
				"name": "Bamboo Finance",
				"decimals": 18
			},
			"0x939105c2d7f0ee617ec1efa6f266a2c54a6c8b90": {
				"ticker": "EverETH",
				"address": "0x939105c2D7f0EE617ec1EFa6f266A2C54A6c8B90",
				"name": "EverETH",
				"decimals": 8
			},
			"0xbca7f1998dc9ffb70b086543a808960a460abca7": {
				"ticker": "KITTY",
				"address": "0xbca7f1998Dc9FFB70b086543a808960a460aBcA7",
				"name": "KITTY",
				"decimals": 18
			},
			"0xd97511e57e9d68219049f47ec17c823d5d89b962": {
				"ticker": "FRD",
				"address": "0xd97511E57E9D68219049F47Ec17C823d5D89B962",
				"name": "FronRunDAO",
				"decimals": 9
			},
			"0x61a47034276eb993e1c5e67bf1375ad0a48f10f6": {
				"ticker": "NRK",
				"address": "0x61A47034276eb993e1C5E67BF1375aD0A48F10F6",
				"name": "NoahArk",
				"decimals": 9
			},
			"0x0d0e443895b448a8d7d0a7335e32e6e144617cf5": {
				"ticker": "jollybee",
				"address": "0x0D0e443895B448a8D7D0a7335e32e6e144617cf5",
				"name": "jollybee",
				"decimals": 18
			},
			"0x70928e5b188def72817b7775f0bf6325968e563b": {
				"ticker": "LUNA",
				"address": "0x70928E5B188def72817b7775F0BF6325968e563B",
				"name": "LUNA (Wormhole)",
				"decimals": 6
			},
			"0xa2536fcaf2af12a669eca96e5d8fdb2691c96d72": {
				"ticker": "Channe",
				"address": "0xa2536fCAf2aF12A669EcA96e5d8fDB2691C96D72",
				"name": "Channel DAO",
				"decimals": 8
			},
			"0x553373bae3df6dfb1da1e34aafdad2e5f9a104b0": {
				"ticker": "PUGGY",
				"address": "0x553373BaE3df6dFB1da1e34aafdad2e5F9a104B0",
				"name": "Puggy",
				"decimals": 9
			},
			"0x5b0c0a3e22582d238e7d16fdc70f87ca60aaa653": {
				"ticker": "FOM",
				"address": "0x5b0c0A3e22582D238e7D16fdc70f87CA60AAA653",
				"name": "FOMO Game",
				"decimals": 8
			},
			"0xe17ccf22add0bb9e40d73c193f057fac26e968b2": {
				"ticker": "HYBRID",
				"address": "0xE17cCf22Add0bB9e40D73C193f057FaC26E968B2",
				"name": "Hybrid Finance",
				"decimals": 18
			},
			"0xf24da28a1134f5ff7660f79dfe5d203fb7a2ec00": {
				"ticker": "MANTRA",
				"address": "0xF24da28a1134f5Ff7660F79dfe5d203Fb7a2Ec00",
				"name": "MANTRA",
				"decimals": 9
			},
			"0x783c08b5f26e3daf8c4681f3bf49844e425b6393": {
				"ticker": "AUSD",
				"address": "0x783C08b5F26E3daf8C4681F3bf49844e425b6393",
				"name": "Avaware USD",
				"decimals": 18
			},
			"0x674d81ff623c92c4e8b8f346f4ff3dfbcb80c088": {
				"ticker": "MIMP",
				"address": "0x674d81FF623c92C4E8b8F346F4fF3dFBCB80C088",
				"name": "MIM PRINTER",
				"decimals": 18
			},
			"0x35d8e70db1fbd03e7ffab522af31972feba2e249": {
				"ticker": "JPD",
				"address": "0x35d8E70Db1fbD03e7FFaB522AF31972FebA2E249",
				"name": "JEWEL PRINTER DAO",
				"decimals": 18
			},
			"0x8d064633dd19951dfaafbe6ebe955051cf04c953": {
				"ticker": "MM",
				"address": "0x8D064633dD19951dFAaFbE6ebe955051cf04C953",
				"name": "MILK ME ",
				"decimals": 18
			},
			"0xb9a9a46793ef80476d28ffc49f5b2e298f5d71ef": {
				"ticker": "IMP",
				"address": "0xB9a9A46793ef80476d28fFC49f5b2E298F5D71ef",
				"name": "IMPERIAL Nodes app.imperialnode.com",
				"decimals": 18
			},
			"0xb1e1f5d7bf095a8c23768c0fa8e104170d425252": {
				"ticker": "USHIB",
				"address": "0xb1E1F5D7BF095A8C23768C0fA8e104170d425252",
				"name": "SHIBA UNIV",
				"decimals": 8
			},
			"0x262599059120798d9231e8ddcb8465b7d08cd1ae": {
				"ticker": "MATRIX",
				"address": "0x262599059120798D9231E8DdcB8465b7D08cd1Ae",
				"name": "AVAXMATRIX ",
				"decimals": 8
			},
			"0x51fb7d776e68cc48efb4622a41ef9d0f06c06563": {
				"ticker": "ISMN",
				"address": "0x51fB7D776E68cC48efb4622a41eF9d0f06C06563",
				"name": "Instemina",
				"decimals": 18
			},
			"0x3ae11549259aa4cd6a9bc6f3d0b1f3d29100cab3": {
				"ticker": "ROCKET",
				"address": "0x3aE11549259AA4CD6a9bc6F3d0b1f3D29100cAB3",
				"name": "RocketMan",
				"decimals": 18
			},
			"0x6ea59dbbc9fbb8e57e985ecde702e9f0bffc116c": {
				"ticker": "https://www.trianglefinance.xyz",
				"address": "0x6EA59DBbc9fbB8E57e985ECdE702e9f0BfFC116C",
				"name": "TRI",
				"decimals": 18
			},
			"0x55ec27f018940625a15d0d862295ae7801853fe0": {
				"ticker": "ATM",
				"address": "0x55Ec27f018940625a15D0d862295ae7801853FE0",
				"name": "Atom Protocol",
				"decimals": 9
			},
			"0x6a32e52431490e706f81db7af4f5c9b25a3b5a01": {
				"ticker": "IMPR",
				"address": "0x6a32e52431490e706F81db7Af4F5C9b25a3B5a01",
				"name": "Imperio DAO",
				"decimals": 18
			},
			"0x6446488ae0d009cb9c4e4b7c3e6f33552d165862": {
				"ticker": "SPR",
				"address": "0x6446488Ae0d009cB9c4e4b7c3e6f33552d165862",
				"name": "SPELL MONEY PRINTER",
				"decimals": 18
			},
			"0xed8cd256b503771120315005dcaf6c53373e2dcf": {
				"ticker": "Parabola",
				"address": "0xEd8cd256B503771120315005DCAf6C53373E2Dcf",
				"name": "Parabola",
				"decimals": 2
			},
			"0x354e2b9712cb9e0d86f7ab3fcaea554821f96828": {
				"ticker": "NINJA",
				"address": "0x354e2b9712CB9E0d86F7Ab3fcaeA554821f96828",
				"name": "Ninja Nodes",
				"decimals": 1
			},
			"0xd84175daab17e1ab4060a971a3ebae26fc371ec3": {
				"ticker": "WIZ",
				"address": "0xD84175DAaB17E1aB4060a971a3EbAE26Fc371eC3",
				"name": "Wizard Nodes",
				"decimals": 9
			},
			"0x09f18f55900c9aa168d79a0962e8da71dab3c319": {
				"ticker": "GME",
				"address": "0x09F18f55900c9aA168D79a0962E8da71Dab3C319",
				"name": "Node Game",
				"decimals": 9
			},
			"0x2c92de4b6c97444f385aa0c6088a551d5fe40c3e": {
				"ticker": "MimPr",
				"address": "0x2c92de4B6C97444f385Aa0c6088A551d5fe40C3E",
				"name": "Magic Money",
				"decimals": 18
			},
			"0x180f79ac833009c9cb99cc3c5908beead2596706": {
				"ticker": "comet",
				"address": "0x180F79Ac833009C9CB99cC3C5908beeaD2596706",
				"name": "cometprinter",
				"decimals": 9
			},
			"0x85ad6da4f531715bb1915a9af077cffade78a4ef": {
				"ticker": "IQC",
				"address": "0x85Ad6DA4f531715bb1915a9AF077CffAde78A4ef",
				"name": "IQCoin",
				"decimals": 18
			},
			"0xe0ce60af0850bf54072635e66e79df17082a1109": {
				"ticker": "ICE",
				"address": "0xe0Ce60AF0850bF54072635e66E79Df17082A1109",
				"name": "IceToken",
				"decimals": 18
			},
			"0x7dfce792c83f283ecfe7ea7ed308f9b891073540": {
				"ticker": "FTOMB",
				"address": "0x7dFCe792c83F283ECfE7EA7ed308F9B891073540",
				"name": "Frozentomb Token",
				"decimals": 18
			},
			"0xf57b80a574297892b64e9a6c997662889b04a73a": {
				"ticker": "EXP",
				"address": "0xf57b80A574297892B64E9a6c997662889b04a73a",
				"name": "EXPToken",
				"decimals": 18
			},
			"0xbfb57353d1c1fecaa8a2c5ceb58e0297dcaa4411": {
				"ticker": "Ghos",
				"address": "0xBfB57353d1C1FECAA8a2C5CEB58E0297DCAa4411",
				"name": "Ghost Nodes",
				"decimals": 18
			},
			"0x1f1fe1ef06ab30a791d6357fdf0a7361b39b1537": {
				"ticker": "SFI",
				"address": "0x1F1FE1eF06ab30a791d6357FdF0a7361B39b1537",
				"name": "sled.finance",
				"decimals": 9
			},
			"0xb34fdb49739058d72f15c18e0f922c1f07314438": {
				"ticker": "ShiNo",
				"address": "0xb34fDb49739058d72f15c18E0f922C1F07314438",
				"name": "Shiba Nodes",
				"decimals": 9
			},
			"0xcda4e9ae9d2387e45bddd4d717bf8edceed3a00e": {
				"ticker": "STAC",
				"address": "0xcDa4E9aE9d2387E45BDdD4D717Bf8EDCEeD3a00E",
				"name": "STARTCOIN",
				"decimals": 8
			},
			"0xf2e4a7c6d028006a4fe65ba7ea308051d380fd9b": {
				"ticker": "TOC",
				"address": "0xf2e4A7C6d028006a4Fe65bA7eA308051d380fd9B",
				"name": "Treasury of the City",
				"decimals": 18
			},
			"0x5ed549d94d87346c4087bbbec602f7f8c72b411d": {
				"ticker": "GOL",
				"address": "0x5ED549D94d87346c4087BbbeC602F7F8c72b411D",
				"name": "Gold Coin",
				"decimals": 9
			},
			"0xaebfc4ddeeb02fdd96fe932443269d413c585bd4": {
				"ticker": "CYBER",
				"address": "0xAeBfc4DDEeB02fDD96Fe932443269d413c585bD4",
				"name": "Cyber Node",
				"decimals": 1
			},
			"0xded5438e89214382c3bea3e3da523b511d1c5084": {
				"ticker": "AJwl",
				"address": "0xdED5438E89214382C3BeA3e3DA523b511D1C5084",
				"name": "AJewel",
				"decimals": 9
			},
			"0x5e1d950822b2653412f75f6cc173a08b04fa7585": {
				"ticker": "KING",
				"address": "0x5e1D950822B2653412F75f6Cc173a08B04fA7585",
				"name": "King Finance",
				"decimals": 18
			},
			"0x0cad9049696167321ef349234fe96866541e4f8b": {
				"ticker": "PEN",
				"address": "0x0cAD9049696167321ef349234FE96866541e4F8B",
				"name": "PennySwap ",
				"decimals": 9
			},
			"0x838a68e0e6d193a10198984bdeedc356bd82a15d": {
				"ticker": "TNT",
				"address": "0x838A68E0E6d193A10198984bDeEdC356Bd82A15d",
				"name": "TNT",
				"decimals": 18
			},
			"0x64f0107b1f0660bf08bead75faa05c1cef719286": {
				"ticker": "JUNGLE",
				"address": "0x64f0107b1f0660BF08BeAd75fAa05c1CEf719286",
				"name": "Jungle Money",
				"decimals": 1
			},
			"0x19613fec180746831bdf819e0790f35741040dfd": {
				"ticker": "SBR",
				"address": "0x19613Fec180746831bdf819e0790F35741040DFD",
				"name": "Snowbear",
				"decimals": 9
			},
			"0x84bb23b4ccfb89f6a3079f7cde23c499d5a0519b": {
				"ticker": "HPT",
				"address": "0x84bb23B4cCfb89f6A3079f7Cde23C499D5a0519B",
				"name": "HighPoint Finance",
				"decimals": 18
			},
			"0x23b99a3ed72f032b11e8f8ae0d761996fdb114cc": {
				"ticker": "ok",
				"address": "0x23B99a3Ed72f032B11E8F8AE0d761996Fdb114Cc",
				"name": "okk",
				"decimals": 18
			},
			"0x2f059c5d9109787c50ec6360a806758ff45a7c15": {
				"ticker": "SD",
				"address": "0x2f059c5D9109787c50EC6360a806758ff45a7c15",
				"name": "SNOW DAO",
				"decimals": 18
			},
			"0x3425fd8a18992f0c1a15400a08fbbb50afd73dad": {
				"ticker": "APEPAY",
				"address": "0x3425FD8A18992F0C1a15400A08fBBb50afD73dAd",
				"name": "ApePay",
				"decimals": 18
			},
			"0xbcef68ea2691a19d27694cd108e2f436086e2c60": {
				"ticker": "AVAXPRINTER",
				"address": "0xbcef68ea2691A19D27694cd108E2F436086E2c60",
				"name": "avaxprinter",
				"decimals": 9
			},
			"0xd641e62525e830e98cb9d7d033a538a1f092ff34": {
				"ticker": "LVT",
				"address": "0xD641E62525e830e98Cb9D7D033a538A1f092ff34",
				"name": "Louverture v2",
				"decimals": 18
			},
			"0xb96426a0cf2afca62864be5b2ae8b9e8de53b335": {
				"ticker": "AZOT",
				"address": "0xB96426a0Cf2AFcA62864be5b2Ae8b9e8De53B335",
				"name": "Azot Nodes",
				"decimals": 18
			},
			"0x125997ac684996e003b92365a99cdf8e8fcc8926": {
				"ticker": "MAFIA",
				"address": "0x125997AC684996E003b92365A99cDF8e8FCC8926",
				"name": "MAFIA NODES",
				"decimals": 18
			},
			"0xbdb54b677b1c7cad43b15137658f09c466a85c6e": {
				"ticker": "FB",
				"address": "0xBdB54b677B1c7CaD43b15137658f09C466A85c6e",
				"name": "FireBank DAO",
				"decimals": 9
			},
			"0x6c381110bf0e7b721c4dddf6168c90ee2320f19d": {
				"ticker": "ARMY",
				"address": "0x6c381110Bf0E7b721c4dddf6168c90ee2320F19D",
				"name": "Army Token",
				"decimals": 18
			},
			"0x54e1089368ca397f295142b9f60c6e5e51b30f36": {
				"ticker": "SNOW",
				"address": "0x54e1089368cA397f295142b9F60c6E5E51B30f36",
				"name": "Snowflake",
				"decimals": 1
			},
			"0x19db57bf90689906f287b27910857ae580c0b3a0": {
				"ticker": "FKA",
				"address": "0x19db57bF90689906f287b27910857aE580C0b3A0",
				"name": "Floki King Avax",
				"decimals": 18
			},
			"0x9adcbba4b79ee5285e891512b44706f41f14cafd": {
				"ticker": "PXT",
				"address": "0x9ADCbba4b79eE5285E891512b44706F41F14CAFd",
				"name": "Project X Token",
				"decimals": 18
			},
			"0x6e913f683f15c02477b70d77c9c9e0ad4d4ddfb7": {
				"ticker": "OLP",
				"address": "0x6E913F683f15c02477B70d77C9C9E0Ad4D4DDfB7",
				"name": "Olympus Printer",
				"decimals": 8
			},
			"0xc09cc683d77c0aa3a8f20d4d32a8e360b4bd7332": {
				"ticker": "MoonDoge",
				"address": "0xC09cc683D77c0Aa3a8f20d4d32A8e360B4bD7332",
				"name": "MoonDoge V2",
				"decimals": 18
			},
			"0xf72d4efb08ff4a3c10b29e75cfab29d2d2a563b1": {
				"ticker": "APT",
				"address": "0xf72d4EfB08FF4a3C10B29e75CfaB29d2d2A563b1",
				"name": "Angele Printer Time",
				"decimals": 18
			},
			"0x9f3c3c7df958ecb2758a937531b2107bf7a1e5c2": {
				"ticker": "KGC",
				"address": "0x9f3C3C7Df958ECb2758A937531B2107Bf7A1E5c2",
				"name": "KingdomQuest",
				"decimals": 9
			},
			"0xd3df6106346ac91ce76d1adf291a68581d3fa782": {
				"ticker": "MNDL",
				"address": "0xd3DF6106346Ac91CE76D1AdF291a68581D3fa782",
				"name": "MNDL Token",
				"decimals": 6
			},
			"0xdaf256f3a828d56298e84da25fe1176681ca1491": {
				"ticker": "SANTA",
				"address": "0xdAF256F3a828D56298e84da25Fe1176681CA1491",
				"name": "SANTA NODES",
				"decimals": 9
			},
			"0xb4b4a5cc69dccf98921d746d2a32c828d57ee0f0": {
				"ticker": "MAX",
				"address": "0xb4b4A5cC69DCcF98921D746d2a32C828d57EE0f0",
				"name": "Maximum Money",
				"decimals": 9
			},
			"0xe279d1dac60e45b4cfb63cf3ae2bedfefa343802": {
				"ticker": "RND",
				"address": "0xE279d1DaC60e45b4cFb63cF3AE2BEdFEFA343802",
				"name": "Redlight Node",
				"decimals": 8
			},
			"0x40ecca1f1276a6462321e1e2a15f795bd189de88": {
				"ticker": "SAI",
				"address": "0x40eccA1f1276A6462321e1E2a15f795BD189DE88",
				"name": "Saitanobi",
				"decimals": 9
			},
			"0xd59284ecb8a2c2e30fc5b0f6b26b326702dfa102": {
				"ticker": "BERRY",
				"address": "0xd59284ECb8A2c2e30fC5b0f6b26B326702DFa102",
				"name": "Strawberry Finance",
				"decimals": 1
			},
			"0x4e864a3540932f70c5ebba1ec6204efa41c2aa36": {
				"ticker": "MORPH",
				"address": "0x4E864a3540932F70c5EBBa1EC6204EFA41C2aa36",
				"name": "Morpheus DAO",
				"decimals": 9
			},
			"0xcd6d9c09f88fc3d743fe68b47c05bb50eed7222d": {
				"ticker": "ADI",
				"address": "0xCd6d9c09f88FC3D743FE68B47c05bB50EEd7222D",
				"name": "Aditus",
				"decimals": 9
			},
			"0x4c36c58e81ad955b0dfd592a9de073cddffea6e0": {
				"ticker": "DAIp",
				"address": "0x4C36c58e81AD955B0DFD592a9DE073cdDfFea6e0",
				"name": "DAI PRINTER",
				"decimals": 18
			},
			"0x9e6832d13b29d0b1c1c3465242681039b31c7a05": {
				"ticker": "STOMB",
				"address": "0x9e6832D13b29d0B1C1c3465242681039b31C7a05",
				"name": "Snowtomb Token",
				"decimals": 18
			},
			"0x4e12ceaa6d37a7e8de6d4c336d16fcda96ff9367": {
				"ticker": "$HOTEL",
				"address": "0x4E12cEaa6d37a7e8dE6d4C336d16fCda96ff9367",
				"name": "HotelEmpire",
				"decimals": 9
			},
			"0x4b08340eeb154640ac59ffed86333ab24e58264a": {
				"ticker": "$DB",
				"address": "0x4b08340Eeb154640ac59ffED86333Ab24e58264A",
				"name": "Diamond Bank",
				"decimals": 18
			},
			"0x511d35c52a3c244e7b8bd92c0c297755fbd89212": {
				"ticker": "BETA",
				"address": "0x511D35c52a3C244E7b8bd92c0C297755FbD89212",
				"name": "Beta Token",
				"decimals": 18
			},
			"0x63e938ac02f2a3b01a0eedbe9e5a40cb11e780aa": {
				"ticker": "toi",
				"address": "0x63E938AC02F2a3B01A0EedbE9e5a40cb11E780aa",
				"name": "salut",
				"decimals": 18
			},
			"0x92a626ae67dec6a38e9365d02d25bcfee39347c0": {
				"ticker": "MP",
				"address": "0x92A626ae67dEc6a38E9365D02D25BCfeE39347C0",
				"name": "MIM Payer",
				"decimals": 9
			},
			"0xbcddbdfeda14f74b3a9e0d3d572c390a813b9a38": {
				"ticker": "Hom",
				"address": "0xbCDDBDfEda14F74b3a9E0D3D572c390A813B9A38",
				"name": "Home Alone",
				"decimals": 8
			},
			"0xa09e3ab0cd5ee25196f851954f111371d9b11605": {
				"ticker": "WRABBIT",
				"address": "0xa09E3aB0cd5eE25196F851954F111371D9b11605",
				"name": "WonderlandRabbit",
				"decimals": 18
			},
			"0x5c7f2ca9c70d1b4e964141a2d3928d1b733bf8bf": {
				"ticker": "THUNDER",
				"address": "0x5c7f2cA9c70d1B4e964141A2d3928d1b733BF8Bf",
				"name": "Thunder Token",
				"decimals": 18
			},
			"0x2620b988df55298254f7660ecf0da1de9862e103": {
				"ticker": "WAVE",
				"address": "0x2620B988DF55298254F7660ECf0da1dE9862e103",
				"name": "WAVE",
				"decimals": 5
			},
			"0xf69c2fcd9128d49dfa22348c69177f9380438eb8": {
				"ticker": "NFSG",
				"address": "0xf69c2fcd9128d49DfA22348C69177f9380438eB8",
				"name": "NFTSoccerGames",
				"decimals": 6
			},
			"0x8f1703165410915ce02731a5d2f831e3c7650ee8": {
				"ticker": "LAVA",
				"address": "0x8f1703165410915ce02731a5D2f831e3C7650ee8",
				"name": "Lava Nodes",
				"decimals": 18
			},
			"0x6b3283561018adba25c8e2c824d5c1d7a781f8b5": {
				"ticker": "VOLTa",
				"address": "0x6B3283561018ADBa25c8e2C824D5C1D7A781F8b5",
				"name": "Volta",
				"decimals": 9
			},
			"0x34d4790def1fdcf2e008e01bb3f7cb05a49020f0": {
				"ticker": "PHRN",
				"address": "0x34d4790DeF1fDCF2e008E01BB3f7cB05A49020F0",
				"name": "Pharaon Nodes",
				"decimals": 1
			},
			"0x7aa5e29df26a897fade56ec575410f25d34eb7f7": {
				"ticker": "COLD",
				"address": "0x7aA5E29df26a897faDE56EC575410f25d34eb7f7",
				"name": "ICED NODES",
				"decimals": 18
			},
			"0x5a6c1df9fb4bc085525af34b2b4f7db0eac6f6ee": {
				"ticker": "ASS",
				"address": "0x5A6C1Df9Fb4bC085525Af34B2B4f7db0eAc6F6Ee",
				"name": "Arctic Snow Seal",
				"decimals": 9
			},
			"0x945a6869819cdad404dc80647a2085957cb1f28b": {
				"ticker": "LEO",
				"address": "0x945a6869819cdAD404DC80647a2085957CB1f28b",
				"name": "Leonidas",
				"decimals": 5
			},
			"0x266b09f6f69185fe4b54a59c8de8e9fd21c0f9f8": {
				"ticker": "AvaxHODL",
				"address": "0x266b09f6f69185Fe4B54A59c8DE8e9FD21c0f9F8",
				"name": "AvaxHODL",
				"decimals": 9
			},
			"0x81cc64417ef348735e534b2b2b770f336cf45652": {
				"ticker": "MULTI",
				"address": "0x81CC64417EF348735E534B2B2b770F336cf45652",
				"name": "Multiverse",
				"decimals": 18
			},
			"0xbfc553e9e7d5eefc96a8064c07a19e692d6c7aa4": {
				"ticker": "Hecto",
				"address": "0xbFc553e9E7D5EEfc96A8064c07A19e692d6C7aa4",
				"name": "Hector DAO",
				"decimals": 6
			},
			"0xdcff9a4f33899db75950b8fb6bba3ce27978760b": {
				"ticker": "BANKSY",
				"address": "0xdcfF9A4F33899DB75950B8FB6bba3Ce27978760b",
				"name": "BanksyDAO",
				"decimals": 18
			},
			"0xbd6187812271fbd6fff3ffbc39a060741e3bd044": {
				"ticker": "AVATAR",
				"address": "0xbD6187812271Fbd6Fff3FfBC39A060741E3BD044",
				"name": "AvatarDAO",
				"decimals": 18
			},
			"0x155dbbb7ce079f7b8ecfff9688adf6e6efb69a24": {
				"ticker": "FLOT",
				"address": "0x155dBbB7CE079f7b8EcfFf9688AdF6e6efb69a24",
				"name": "Frozentomb Lot",
				"decimals": 18
			},
			"0x89245428b5983b54d50576c891f426069dd2f096": {
				"ticker": "Ma",
				"address": "0x89245428b5983b54d50576c891f426069Dd2f096",
				"name": "Max Speed",
				"decimals": 8
			},
			"0xa248fd54a02c06db58b4c2ea5f05704848776861": {
				"ticker": "SF",
				"address": "0xA248fd54A02C06db58B4C2ea5F05704848776861",
				"name": "Scuderia Ferrari",
				"decimals": 18
			},
			"0x4a7bd88e1e45dd4675998f57f7b1ed613eda29b4": {
				"ticker": "bJOE",
				"address": "0x4a7Bd88e1e45dD4675998f57F7b1ED613eDa29b4",
				"name": "BabyJoe",
				"decimals": 9
			},
			"0xc2a4d411ac8ada649b85c14671ebfb4aa4589772": {
				"ticker": "PLASMA",
				"address": "0xc2a4D411ac8adA649b85c14671eBFb4Aa4589772",
				"name": "Quantum Plasma",
				"decimals": 18
			},
			"0xf1d55bf3a413612e54f0cd29d89abcfc03f7fd70": {
				"ticker": "METAW",
				"address": "0xf1D55Bf3A413612E54f0Cd29D89AbCFc03F7fd70",
				"name": "Meta World",
				"decimals": 1
			},
			"0x5133e64ea372ce805bfb0603fd16470e6d10306d": {
				"ticker": "Ocean",
				"address": "0x5133e64EA372cE805bFb0603fd16470E6d10306D",
				"name": "Ocean",
				"decimals": 9
			},
			"0x4b4b0ff68d19951264db424264f97655ff3dfc51": {
				"ticker": "Bucks",
				"address": "0x4b4b0fF68D19951264Db424264f97655FF3Dfc51",
				"name": "Bucks",
				"decimals": 9
			},
			"0x64e7d1839d10f0b61a1c3bf4b35678a218033a66": {
				"ticker": "SHIELD",
				"address": "0x64e7d1839D10F0B61a1c3BF4B35678A218033A66",
				"name": "SHIELD FINANCE",
				"decimals": 18
			},
			"0x25362e3cb46fe4a75aa155852cdf9a5eb1c4b719": {
				"ticker": "SWING",
				"address": "0x25362e3cB46Fe4a75aA155852CDF9a5Eb1c4b719",
				"name": "Swing DAO",
				"decimals": 18
			},
			"0x1263fea931b86f3e8ce8afbf29f66631b7be9347": {
				"ticker": "BNPL",
				"address": "0x1263fEA931b86f3e8cE8AFBf29f66631b7be9347",
				"name": "BNPL Pay",
				"decimals": 18
			},
			"0x2969505e2c20861b9679613c3b65a49bfe71faca": {
				"ticker": "RICE",
				"address": "0x2969505E2c20861b9679613C3B65A49BfE71FACA",
				"name": "Rice Finance",
				"decimals": 8
			},
			"0x57671f82bea180f7ac947e7537b7b971ef452430": {
				"ticker": "LIFE",
				"address": "0x57671f82BeA180f7ac947e7537B7B971ef452430",
				"name": "Life",
				"decimals": 9
			},
			"0xac53b3dfb93ccceae015e7b5c1cef4681a2d3d9e": {
				"ticker": "iSKEEN",
				"address": "0xAC53b3dFB93CCcEaE015E7B5C1Cef4681a2D3d9e",
				"name": "iSKEEN Shares",
				"decimals": 18
			},
			"0x030ab0f9276af2c244e68eba194cc0f6f143a930": {
				"ticker": "ANCAP",
				"address": "0x030Ab0F9276Af2C244E68Eba194cc0f6F143a930",
				"name": "AnarchoCapital",
				"decimals": 18
			},
			"0x9a1788a1513d8a23871d4611c9682b960531e08d": {
				"ticker": "Tea",
				"address": "0x9A1788a1513D8a23871d4611c9682B960531E08d",
				"name": "Tear Drop",
				"decimals": 8
			},
			"0x786f54c3f61e228dc22760b2374ae37d8174f465": {
				"ticker": "payTIME",
				"address": "0x786f54c3F61E228Dc22760b2374Ae37d8174f465",
				"name": "payTIME",
				"decimals": 9
			},
			"0xd3d64127a0869497cdcb272553a857dc62890d95": {
				"ticker": "CAPITOL",
				"address": "0xD3d64127a0869497cdCb272553A857dC62890D95",
				"name": "CAPITOL FINANCE",
				"decimals": 8
			},
			"0x2f8e9651e70bb7f3e56352836ed0c4fe7b56c7aa": {
				"ticker": "SCHISM",
				"address": "0x2F8e9651e70bb7F3e56352836ED0c4fE7b56c7AA",
				"name": "SCHISM",
				"decimals": 8
			},
			"0xbd1fdedb3c5b7f64c08e547c70a69204e773adf0": {
				"ticker": "SAKT",
				"address": "0xbd1fdEDb3c5B7f64C08E547c70a69204e773aDf0",
				"name": "Super Akita",
				"decimals": 12
			},
			"0x32975907733f93305be28e2bfd123666b7a9c863": {
				"ticker": "INK",
				"address": "0x32975907733f93305BE28E2bfd123666b7A9c863",
				"name": "INK",
				"decimals": 18
			},
			"0xc70a8977cf8c6549e2df28a1d3797d802ab05c26": {
				"ticker": "sNODE",
				"address": "0xc70a8977CF8C6549e2df28a1d3797d802Ab05C26",
				"name": "Stealth Node",
				"decimals": 18
			},
			"0xbddb4b80398851e2a83eb8b1276f828ed4cbe2d2": {
				"ticker": "SPA",
				"address": "0xBdDb4B80398851e2a83EB8B1276f828eD4cbE2d2",
				"name": "SpaceDAO",
				"decimals": 9
			},
			"0xd9702f5e3b0eb7452967cb82529776d672bdc03f": {
				"ticker": "NEKO",
				"address": "0xD9702F5E3b0eb7452967CB82529776D672bdC03F",
				"name": "Lucky Cat",
				"decimals": 8
			},
			"0xd80d4329dd376b0750f0988781737272b1dab3fb": {
				"ticker": "Jewelp",
				"address": "0xD80d4329dD376B0750F0988781737272B1dAB3Fb",
				"name": "JEWEL Printer",
				"decimals": 18
			},
			"0x98a6f922d405d25bdc965b1c23d1ebb31dfa8000": {
				"ticker": "ROCK",
				"address": "0x98a6f922d405d25bdc965B1c23D1EBb31DFA8000",
				"name": "AVA ROCK",
				"decimals": 9
			},
			"0x1bbe1def0590790484f34fab4be0a2f32889dc52": {
				"ticker": "FLEX",
				"address": "0x1BBE1dEf0590790484F34fab4bE0a2f32889Dc52",
				"name": "REFLEXOR",
				"decimals": 9
			},
			"0xa11980f7536c6de235a07c61fe4071607bffb317": {
				"ticker": "aFROG",
				"address": "0xA11980F7536c6dE235A07C61fE4071607BffB317",
				"name": "AliceFrog",
				"decimals": 9
			},
			"0xd73b80ac24fb963d6811084dcff57b41fbf581b9": {
				"ticker": "GARGA",
				"address": "0xD73B80ac24fb963d6811084dCff57B41fbF581B9",
				"name": "GARGANTUA PRINTER",
				"decimals": 8
			},
			"0x096c33d32db8d6c1bc15220c52b3c941c0cc4b15": {
				"ticker": "MILK",
				"address": "0x096c33d32db8d6C1bc15220C52B3C941C0cC4B15",
				"name": "MILK ",
				"decimals": 18
			},
			"0x430026966ab5749028ccf51e78fa96b8a31cd780": {
				"ticker": "KingAvax",
				"address": "0x430026966AB5749028cCF51e78fa96B8A31cd780",
				"name": "KingAvax.Money",
				"decimals": 9
			},
			"0x901d4000334b14cc9430ec0f4a1faa3031196a1e": {
				"ticker": "CryptoWar",
				"address": "0x901D4000334B14cC9430Ec0f4A1FAA3031196A1E",
				"name": "CryptoWars Game",
				"decimals": 8
			},
			"0x148a75df15e7acb1219fe0e6f63d40659ed63e76": {
				"ticker": "UMANI",
				"address": "0x148A75df15E7ACb1219Fe0e6F63d40659ed63e76",
				"name": "https://avax.umani.fun",
				"decimals": 9
			},
			"0xc12e249fabe1c5eb7c558e5f50d187687a244e31": {
				"ticker": "BLUE",
				"address": "0xc12e249FaBe1c5Eb7C558E5F50D187687a244E31",
				"name": "Blue Token",
				"decimals": 18
			},
			"0x86ddc67db53b3152a66790d3c2bcf7b2940123c1": {
				"ticker": "MP",
				"address": "0x86dDC67DB53B3152a66790d3C2bCF7B2940123c1",
				"name": "MIM PRINTER",
				"decimals": 18
			},
			"0x0ec5bcf1677e577cfa40fecd9db4dfcc173b92d4": {
				"ticker": "RA1N",
				"address": "0x0Ec5bCF1677e577cFA40fEcD9dB4DFCc173B92D4",
				"name": "RA1N Token",
				"decimals": 18
			},
			"0x49ffdd8ff5dd3f227d4ade4d2a6fdb642ac27cfd": {
				"ticker": "FOHMFORKS",
				"address": "0x49FFDd8FF5dd3F227D4aDe4D2A6fDB642AC27cFD",
				"name": "OHM FORKS REFUGEE DAO",
				"decimals": 18
			},
			"0x131b70415ba8072a2a3bfafb5aca41bc66c638af": {
				"ticker": "CORGI",
				"address": "0x131B70415bA8072A2a3BFAfb5ACA41bC66c638af",
				"name": "Corgi NET",
				"decimals": 12
			},
			"0x813f49678ce0e6bdffa9471a5f6e94737eb4cf90": {
				"ticker": "Spellcaster",
				"address": "0x813F49678cE0e6BDffA9471a5f6e94737eB4Cf90",
				"name": "Spellcaster",
				"decimals": 6
			},
			"0x25c0e97e982d320127afa04f9837b819ececc8d3": {
				"ticker": "CPR",
				"address": "0x25c0E97E982d320127aFA04F9837B819EcEcc8D3",
				"name": "CIPHER FINANCE",
				"decimals": 8
			},
			"0x2064bf6e6850f8108d6b96344c29947275161f18": {
				"ticker": "MYTC",
				"address": "0x2064bF6e6850F8108d6B96344c29947275161f18",
				"name": "Mythic",
				"decimals": 9
			},
			"0xbc2568521ed891fe7635cff3323ed2f988a7c04b": {
				"ticker": "ICE",
				"address": "0xbc2568521eD891fE7635cFf3323ed2f988a7C04B",
				"name": "ICE DAO",
				"decimals": 18
			},
			"0xf497ba63412ae3283d1bfe8d7f4e7e66c516c83c": {
				"ticker": "PANTHER",
				"address": "0xf497BA63412ae3283d1Bfe8d7f4E7e66c516C83c",
				"name": "Panther Inu",
				"decimals": 18
			},
			"0xacc5c7ed59c260d92e69a2b714aabdde0240af44": {
				"ticker": "FLH",
				"address": "0xACC5c7Ed59c260d92e69a2b714AAbdDe0240af44",
				"name": "FlashDAO ",
				"decimals": 18
			},
			"0x98324ed7bd876ffd9ae25268c992a796c5f56a99": {
				"ticker": "VLGR",
				"address": "0x98324ed7bD876FFd9AE25268c992A796c5F56A99",
				"name": "VillagerFinance",
				"decimals": 9
			},
			"0x12b2788ce1be7887dc65a16f62be7a2e4e648dcd": {
				"ticker": "ARENA",
				"address": "0x12b2788CE1BE7887DC65a16f62Be7A2e4e648dCD",
				"name": "ARENA",
				"decimals": 8
			},
			"0xfca4217a678e118404ce832b1f2bf9e2bfc4bee3": {
				"ticker": "PYTHON",
				"address": "0xFCa4217A678e118404Ce832B1F2bf9e2BFC4beE3",
				"name": "Python Nodes",
				"decimals": 9
			},
			"0x73268d88979fa9d1903c48ea25c474396e37e06d": {
				"ticker": "FUEL",
				"address": "0x73268D88979fA9D1903c48eA25c474396e37E06D",
				"name": "FuelNodes",
				"decimals": 18
			},
			"0xc304e27f865c3f7af8d04a8eb31d11dd14018eb5": {
				"ticker": "MAGMA",
				"address": "0xC304E27F865C3f7AF8D04A8eb31d11dD14018EB5",
				"name": "MAGMA Printer",
				"decimals": 18
			},
			"0x433ec0444518c5f5ca7ea962a3a6e8bf69910c0f": {
				"ticker": "CHRISTMAS",
				"address": "0x433Ec0444518C5F5ca7ea962a3A6E8bF69910c0F",
				"name": "Christmas Token",
				"decimals": 9
			},
			"0xd9eae369846ede32bf0bb4a2faf45e20b1eb1fd8": {
				"ticker": "VLGR",
				"address": "0xd9EaE369846Ede32BF0bb4A2fAf45E20B1eb1fD8",
				"name": "VillagerFinance",
				"decimals": 18
			},
			"0xa0ce376565719a1732874ef381e39cd863999198": {
				"ticker": "Crypt",
				"address": "0xa0ce376565719A1732874Ef381e39cd863999198",
				"name": "Crypto Storm",
				"decimals": 8
			},
			"0x70566ef06249c1d9025d890abec36bd9da832ee0": {
				"ticker": "ATM",
				"address": "0x70566EF06249c1d9025d890ABEc36bd9DA832EE0",
				"name": "Atom Protocol",
				"decimals": 18
			},
			"0xc60d349f4b019ca126028a903ba562d883abc134": {
				"ticker": "COLD",
				"address": "0xc60d349F4B019CA126028a903bA562d883abC134",
				"name": "Cold Node",
				"decimals": 18
			},
			"0x3cc3fa02cbc90f13e772ef239e257b8b65921d33": {
				"ticker": "BJOE",
				"address": "0x3Cc3fA02CBC90F13E772ef239e257B8B65921D33",
				"name": "Boomer Joe",
				"decimals": 18
			},
			"0x280f1a89f3bf8002afd49da08dcaf45d56a69143": {
				"ticker": "VLGR",
				"address": "0x280F1A89f3bf8002aFd49DA08dcAf45d56A69143",
				"name": "VillagerFinance",
				"decimals": 9
			},
			"0x6e48adb63e502721e3ad409e36dc3be23cbd2fe7": {
				"ticker": "MIAP",
				"address": "0x6e48adB63E502721e3AD409e36DC3Be23cBD2fe7",
				"name": "MagicInternetAP",
				"decimals": 18
			},
			"0x03d4b0f4ce22ea4bd29670653240fe2527719f1b": {
				"ticker": "SM",
				"address": "0x03d4B0f4CE22Ea4bd29670653240FE2527719F1b",
				"name": "SPELL MONEY",
				"decimals": 18
			},
			"0xb424316f45be954635a0d2eab1a50438cc719656": {
				"ticker": "BULLS",
				"address": "0xB424316F45bE954635a0d2eAB1A50438cC719656",
				"name": "BULL RUN",
				"decimals": 9
			},
			"0x2b0220c1bc6c318da704caf13baacfdc7b3f8dee": {
				"ticker": "LINU",
				"address": "0x2B0220C1bC6C318da704caF13baaCFDC7b3F8dEe",
				"name": "Luna Inu",
				"decimals": 8
			},
			"0xdbdfa1dda6d67ae02f0a0fa2634c5c5a008753b9": {
				"ticker": "WSB",
				"address": "0xDBDfa1Dda6D67aE02F0a0FA2634C5C5A008753B9",
				"name": "WallStreet Bets",
				"decimals": 9
			},
			"0xb1edd70ca22a8969f9ef96b8a2c9e0da07f5a27c": {
				"ticker": "VELVE",
				"address": "0xB1EdD70CA22A8969f9eF96B8a2c9e0DA07f5a27c",
				"name": "VELVET DAO",
				"decimals": 8
			},
			"0x6d630c884525f41735d1b5652966b407b6f76245": {
				"ticker": "SN",
				"address": "0x6D630c884525F41735d1B5652966b407B6F76245",
				"name": "StellarNodes",
				"decimals": 18
			},
			"0x8b2415a18efd6fb590b0a506ab4c254fbead7579": {
				"ticker": "POLAR",
				"address": "0x8b2415a18EFd6Fb590B0a506aB4c254FBEAD7579",
				"name": "Polar Nodes",
				"decimals": 1
			},
			"0x4eedcd82badcecd5f5ae95e3c4f407ae4bd44d77": {
				"ticker": "GOLD",
				"address": "0x4eEdcD82bAdCecd5f5aE95e3c4f407ae4bD44d77",
				"name": "Golden Finance",
				"decimals": 18
			},
			"0x3c528e834fd9008325f32c1cf1e276d2d822b143": {
				"ticker": "ECMP",
				"address": "0x3c528e834FD9008325f32C1CF1e276D2d822B143",
				"name": "Exo Chimp",
				"decimals": 10
			},
			"0x8e8148078f913a36c9d8c7fb2da8b479c77c6ba5": {
				"ticker": "BLOOD",
				"address": "0x8E8148078F913a36C9d8C7FB2da8b479c77c6bA5",
				"name": "BLOOD",
				"decimals": 18
			},
			"0x5a99870986eea95892505fcb49a5ea2e37ab5d41": {
				"ticker": "OMEN",
				"address": "0x5A99870986eEa95892505Fcb49A5Ea2E37aB5d41",
				"name": "OMEN",
				"decimals": 8
			},
			"0x9b9e9a7bca69c0adaedf0fa47a43f5c21d2686d4": {
				"ticker": "SSD",
				"address": "0x9B9E9A7BCa69c0AdAEdF0fa47A43F5c21D2686d4",
				"name": "SECRET SOCIETY DAO",
				"decimals": 18
			},
			"0x25503b6d41e274705858b7edfc80a6166325ee36": {
				"ticker": "Island",
				"address": "0x25503B6d41e274705858B7EDFC80a6166325ee36",
				"name": "Crypto Island",
				"decimals": 9
			},
			"0x320ada89dbfa3a154613d2731c9bc3a4030dba19": {
				"ticker": "FROST",
				"address": "0x320aDa89DbFA3A154613D2731c9BC3a4030DbA19",
				"name": "FROST",
				"decimals": 18
			},
			"0x1e5e26af04af6b1e215199baf8b5d8c64838f0b0": {
				"ticker": "3ULL",
				"address": "0x1e5e26aF04Af6b1e215199Baf8B5d8C64838f0b0",
				"name": "PLAYA3ULL Token",
				"decimals": 18
			},
			"0xe1c62bf5728072d14d19634db651ca2613f10546": {
				"ticker": "GEM",
				"address": "0xE1c62BF5728072d14d19634dB651ca2613f10546",
				"name": "Gemini DAO",
				"decimals": 9
			},
			"0xf54501fbc97d5cb9c8b96480996a397e07516b2e": {
				"ticker": "ANYAN",
				"address": "0xf54501FBc97d5CB9C8B96480996a397E07516b2E",
				"name": "AvaNYAN",
				"decimals": 18
			},
			"0xca98dfb6f4f3f7446833730dda5cd929de23da9f": {
				"ticker": "BOOH",
				"address": "0xca98Dfb6F4f3f7446833730dDa5CD929De23da9F",
				"name": "CasperFinance",
				"decimals": 9
			},
			"0x6c84870de6eeb23b81e915108cb0ff19dcc9d6a4": {
				"ticker": "HRCLS",
				"address": "0x6C84870de6eeB23b81E915108cB0fF19dcc9d6a4",
				"name": "Hercules Finance",
				"decimals": 1
			},
			"0xc58f8d5dd6c4862f1f51b1d1fd4340e2aaeb5924": {
				"ticker": "UPRINTER",
				"address": "0xC58f8d5Dd6c4862f1F51B1d1FD4340E2aaeb5924",
				"name": "ULTIMATE PRINTER",
				"decimals": 8
			},
			"0x8d5f02cab3f2741cf6e165e61ae6f853772c7b8b": {
				"ticker": "ACH",
				"address": "0x8D5F02CaB3f2741CF6e165e61ae6f853772C7B8B",
				"name": "AvaxChads",
				"decimals": 9
			},
			"0x3e39bebae8611eb3b81e917d8ac80b092c5c698b": {
				"ticker": "OGN",
				"address": "0x3E39bEBAE8611Eb3B81e917D8Ac80b092c5C698b",
				"name": "Oxygen Nodes",
				"decimals": 18
			},
			"0x7c381d45550ba43964af17215af44db8ba4f5a6d": {
				"ticker": "JEET",
				"address": "0x7c381D45550bA43964af17215aF44DB8Ba4f5A6D",
				"name": "JEET COIN",
				"decimals": 9
			},
			"0x7e3dc9f520917569c0ee398d77fa1eff0bef0720": {
				"ticker": "WEAVE",
				"address": "0x7e3dC9F520917569C0ee398D77Fa1Eff0BEF0720",
				"name": "WeaveToken",
				"decimals": 18
			},
			"0x2fc4926c2ac926428a7a6b3e07dd2a17b1c4c28d": {
				"ticker": "SH",
				"address": "0x2FC4926C2ac926428a7A6b3e07dD2a17b1c4c28D",
				"name": "StakHolders",
				"decimals": 9
			},
			"0x92de7a3f3385887a4fce3f107c12489bfe06deae": {
				"ticker": "ICE",
				"address": "0x92DE7A3F3385887A4fcE3F107c12489BfE06Deae",
				"name": "ICE NODES",
				"decimals": 9
			},
			"0x1927dc4ce06b0e8f9b5675b3f885492b9f78c955": {
				"ticker": "DMTR",
				"address": "0x1927DC4ce06b0E8F9B5675b3F885492b9f78C955",
				"name": "Demete",
				"decimals": 18
			},
			"0xbf28cfd91cf9239d699f6f449f20d29a0eedaac0": {
				"ticker": "LUP",
				"address": "0xbf28cfd91cf9239d699f6F449F20D29A0eedaAc0",
				"name": "LunaPrinter",
				"decimals": 9
			},
			"0x625a31265d01b355e2e2a90412803191246bc949": {
				"ticker": "PMP",
				"address": "0x625a31265D01B355e2E2a90412803191246bc949",
				"name": "PONZU MONEY PRINTER",
				"decimals": 18
			},
			"0x693c99e1de9c5bf199525fa70933e319a8a4eebf": {
				"ticker": "💳",
				"address": "0x693C99E1DE9C5Bf199525fA70933E319a8A4EEbf",
				"name": "VISA Metaverse",
				"decimals": 9
			},
			"0xa39e7c85afef643ff926e03b604f6d543020afb4": {
				"ticker": "AVIBA",
				"address": "0xA39e7c85AFEf643Ff926E03B604f6d543020Afb4",
				"name": "Aviba",
				"decimals": 6
			},
			"0xd2c7f583cda4173565bebb5f1d8f62c2dc485f05": {
				"ticker": "SPEED",
				"address": "0xD2C7f583cDa4173565BeBB5f1d8f62C2dc485f05",
				"name": "Speed Proto",
				"decimals": 9
			},
			"0x343e9313f34faeef303c54214449216d6aafd7e7": {
				"ticker": "JPM",
				"address": "0x343e9313F34FaeeF303C54214449216D6AaFd7E7",
				"name": "JEWEL PRINTING MACHINE",
				"decimals": 18
			},
			"0x1541fe5a444fb27d2c17f6cb62f38b1fca2a3e5d": {
				"ticker": "RND",
				"address": "0x1541fe5a444fB27D2C17F6CB62f38B1Fca2a3e5d",
				"name": "Redlight Node",
				"decimals": 8
			},
			"0xa2f406e969b6e12bcf2ef4d517aa4fdbdc479a90": {
				"ticker": "AVAXFARM",
				"address": "0xa2F406e969b6e12bcF2EF4d517aa4fDbDc479A90",
				"name": "AVAXFARM",
				"decimals": 9
			},
			"0x85272c4380b25e9047bc749d18c265550fd17017": {
				"ticker": "LAYER",
				"address": "0x85272c4380B25E9047Bc749D18C265550fd17017",
				"name": "LayerFort",
				"decimals": 9
			},
			"0x9d90d4e5d918a64cdd9031f489a7ce4f85ce07ba": {
				"ticker": "Saitam",
				"address": "0x9D90D4E5d918A64cDD9031f489A7ce4f85Ce07BA",
				"name": "Saitama Rudolph",
				"decimals": 18
			},
			"0x423c8aa17d317552be999acb04ba9c6f826fcdc8": {
				"ticker": "BEN",
				"address": "0x423c8Aa17d317552be999Acb04BA9C6F826FCdC8",
				"name": "Benzinga Finance",
				"decimals": 9
			},
			"0x03e8d118a1864c7dc53bf91e007ab7d91f5a06fa": {
				"ticker": "DEXTF",
				"address": "0x03E8D118A1864c7Dc53bf91e007ab7D91f5A06fA",
				"name": "DEXTF Token",
				"decimals": 18
			},
			"0x1dc15cd99bfc8cf9a18ea0318d77421d7f2ad13f": {
				"ticker": "kAVAX",
				"address": "0x1dc15cd99bfC8cF9a18Ea0318d77421d7f2aD13F",
				"name": "kAVAX",
				"decimals": 18
			},
			"0x5717910a90f9a4c44a573206eed1bb49fcb08c03": {
				"ticker": "NSIS",
				"address": "0x5717910a90f9a4c44a573206Eed1bb49fcb08c03",
				"name": "Nemesis DAO",
				"decimals": 1
			},
			"0x94f1f8e585f1f86a5fc1994896feb30af5c353e6": {
				"ticker": "TAPE",
				"address": "0x94F1f8e585F1F86A5fC1994896FeB30AF5C353e6",
				"name": "TapeCoin",
				"decimals": 9
			},
			"0xa3700a758d21a05830ca98b733d0c211d4dda58e": {
				"ticker": "APED",
				"address": "0xA3700a758D21a05830cA98b733D0c211d4dDa58E",
				"name": "ApeDao",
				"decimals": 18
			},
			"0x5c59b34eac734793b7e4e289321334d0793b19f9": {
				"ticker": "DB",
				"address": "0x5c59B34Eac734793b7e4e289321334d0793b19f9",
				"name": "DiamondBank DAO",
				"decimals": 18
			},
			"0xfad011a46517335de96cd0e4a2dbe781b2d32465": {
				"ticker": "JMP",
				"address": "0xfAd011a46517335de96Cd0e4A2dBE781B2D32465",
				"name": "JEWEL MONEY PRINTER",
				"decimals": 18
			},
			"0x673cdd07506ac015752227ca316a1920043f6d28": {
				"ticker": "QIP",
				"address": "0x673CdD07506aC015752227Ca316a1920043f6D28",
				"name": "QI PRINTER",
				"decimals": 18
			},
			"0x9baf39d524708dc034cc4dd848c9faa33763a1ce": {
				"ticker": "NIT",
				"address": "0x9BaF39D524708DC034CC4dd848C9FaA33763a1Ce",
				"name": "Node Investment Trust",
				"decimals": 18
			},
			"0xe0db753624abd96ebbe69e8b2c3b928079c93ab0": {
				"ticker": "AVASROID",
				"address": "0xe0DB753624ABd96EbbE69e8B2c3b928079c93ab0",
				"name": "AVASTEROID",
				"decimals": 6
			},
			"0xc931f61b1534eb21d8c11b24f3f5ab2471d4ab50": {
				"ticker": "aaBLOCK",
				"address": "0xC931f61B1534EB21D8c11B24f3f5Ab2471d4aB50",
				"name": "Blocknet",
				"decimals": 8
			},
			"0x525c12e5b0b381ebbfd7dc069639648b849c77e1": {
				"ticker": "EEVEE",
				"address": "0x525c12e5B0b381eBbFD7DC069639648b849C77e1",
				"name": "Eevee Finance",
				"decimals": 18
			},
			"0x16a74d4cfed1725ce014917836f0f00ab175122f": {
				"ticker": "OPEC",
				"address": "0x16a74D4cfed1725Ce014917836f0F00Ab175122F",
				"name": "Opulence Nodes",
				"decimals": 18
			},
			"0xb4fd77918382472ae06b24ac515cee3d8097ba66": {
				"ticker": "MAG",
				"address": "0xb4fD77918382472AE06B24AC515CEE3d8097bA66",
				"name": "MagnetDAO",
				"decimals": 18
			},
			"0x2dabc305e118fafc4c4a2ea11c6b65e60fb9649f": {
				"ticker": "OMNITEST",
				"address": "0x2DAbc305E118faFc4c4A2eA11C6B65e60FB9649f",
				"name": "OmniverseTest",
				"decimals": 18
			},
			"0xc0cae277d69e6242769fb2fbc63a3808babbb745": {
				"ticker": "cLuna",
				"address": "0xc0cAe277d69e6242769Fb2FbC63a3808BAbbb745",
				"name": "New Community Luna",
				"decimals": 18
			},
			"0xfd3c4ed1094b33804eec218d9a84351c6919bf7e": {
				"ticker": "Xma",
				"address": "0xFd3c4eD1094b33804EEC218D9a84351c6919BF7e",
				"name": "Xmas Nodes",
				"decimals": 18
			},
			"0x8db457a95f11007d8b1010216e92d5cd670ba5b5": {
				"ticker": "Plasm",
				"address": "0x8dB457A95f11007d8b1010216E92D5Cd670ba5B5",
				"name": "Plasma Energy",
				"decimals": 8
			},
			"0xef692c5719d573af7ea9fabce16502139ed94917": {
				"ticker": "NKL",
				"address": "0xEf692c5719d573aF7EA9FAbcE16502139Ed94917",
				"name": "NickelDAO",
				"decimals": 18
			},
			"0x260e417326d8aa64788689ef22f8f98229bb9f04": {
				"ticker": "Entrop",
				"address": "0x260E417326d8aA64788689Ef22F8F98229bB9F04",
				"name": "Entropy Protocol",
				"decimals": 6
			},
			"0x148e47affed52225e3b6eee3045a290057e8fc7e": {
				"ticker": "CNV",
				"address": "0x148E47affed52225E3B6eeE3045a290057E8fc7e",
				"name": "Concave DAO",
				"decimals": 9
			},
			"0xee0f5c0c2c8cfb1473d441868c174de3f3736f70": {
				"ticker": "BPNG",
				"address": "0xee0f5c0c2c8cFB1473d441868C174de3F3736F70",
				"name": "BABYPANGOLIN ",
				"decimals": 18
			},
			"0x790cfdc6ab2e0ee45a433aac5434f183be1f6a20": {
				"ticker": "GINUX",
				"address": "0x790CFDc6aB2e0eE45a433aAC5434F183BE1f6A20",
				"name": "Green Shiba Inu",
				"decimals": 18
			},
			"0x0f0f563b330d5043790f57917174bc7001139e91": {
				"ticker": "DYNO",
				"address": "0x0F0F563B330D5043790F57917174bc7001139e91",
				"name": "DYNO Bit",
				"decimals": 9
			},
			"0xa9a5b6a93744cb9658c21409ea60838290e9a62d": {
				"ticker": "veAGG",
				"address": "0xA9a5B6a93744Cb9658c21409Ea60838290e9a62D",
				"name": "veAggregator",
				"decimals": 9
			},
			"0x112354d919028a5dca6a12b21c8025a79759f78d": {
				"ticker": "STAR",
				"address": "0x112354D919028A5dcA6A12b21c8025A79759F78D",
				"name": "Star Nodes (starnodes.finance)",
				"decimals": 1
			},
			"0xae5ec205fbbcda04a686980116ec460d48f1fe25": {
				"ticker": "Alpaca",
				"address": "0xAe5EC205fbbcDA04A686980116EC460D48f1Fe25",
				"name": "Alpaca Finance",
				"decimals": 9
			},
			"0xe95d57bc7f0778935d79d0accc5b4c3ab9913094": {
				"ticker": "AVAXD",
				"address": "0xE95D57Bc7F0778935d79d0Accc5B4c3Ab9913094",
				"name": "Avax Dao",
				"decimals": 18
			},
			"0xcc05b06c4dcdb3b385cab5fdf15de07da4c10014": {
				"ticker": "RICE",
				"address": "0xCc05B06c4Dcdb3b385CAB5fDf15De07dA4c10014",
				"name": "Rice Finance",
				"decimals": 18
			},
			"0x113f413371fc4cc4c9d6416cf1de9dfd7bf747df": {
				"ticker": "JLP",
				"address": "0x113f413371fC4CC4C9d6416cf1DE9dFd7BF747Df",
				"name": "Joe LP Token",
				"decimals": 18
			},
			"0x69e209a50e2808edb1d15e1d9687bb2d5644dcf0": {
				"ticker": "CZ",
				"address": "0x69E209A50E2808edB1D15E1D9687bB2d5644Dcf0",
				"name": "CEEZEEZ",
				"decimals": 18
			},
			"0x001393fa4328949b5fa58b5613b62621a0429090": {
				"ticker": "GUN",
				"address": "0x001393fa4328949b5FA58B5613B62621a0429090",
				"name": "GUCCINODE",
				"decimals": 9
			},
			"0xd3bb7b5b233c4a2cd0756e8786fe8e0e669f807e": {
				"ticker": "KIAO",
				"address": "0xd3Bb7B5B233C4A2cd0756E8786FE8E0e669F807e",
				"name": "KiaoWallet",
				"decimals": 18
			},
			"0x7e29afead0a3a70246f47cefdee8be3772323e76": {
				"ticker": "DOUBLE",
				"address": "0x7E29AfeaD0a3a70246f47CEFdeE8bE3772323e76",
				"name": "DoubleDAO",
				"decimals": 18
			},
			"0x444398d9b4444612112b610fe995e7a758899f02": {
				"ticker": "Aloha",
				"address": "0x444398D9B4444612112B610fe995e7a758899F02",
				"name": "The Bahamas",
				"decimals": 9
			},
			"0x702192247b6f8daab49cea73721e662bfce33f20": {
				"ticker": "CYBER",
				"address": "0x702192247B6F8dAAB49cea73721E662bfcE33F20",
				"name": "Cyber Node",
				"decimals": 1
			},
			"0xab6a35e3269211b7d22a1852f3b25a9a096dc645": {
				"ticker": "THORN",
				"address": "0xab6a35e3269211b7d22a1852f3b25A9a096dc645",
				"name": "Thorn DAO",
				"decimals": 18
			},
			"0xf3554c3e6e7678ec7bc9bf6a762544f7b2790aa9": {
				"ticker": "Cybe",
				"address": "0xf3554c3e6e7678ec7Bc9bf6A762544F7B2790aA9",
				"name": "Cyber Reality",
				"decimals": 2
			},
			"0xc256d4b2d554a928a4fb69dece80b0f728972235": {
				"ticker": "WEB3AVAX",
				"address": "0xc256D4B2D554A928a4fb69dece80B0F728972235",
				"name": "WEB3AVAX",
				"decimals": 9
			},
			"0x48aa737a95a7d3a5f5ab763ca9fbb4a541639e14": {
				"ticker": "SYNR",
				"address": "0x48Aa737A95a7D3a5f5Ab763Ca9fbB4a541639e14",
				"name": "Syndicate",
				"decimals": 9
			},
			"0x22a867b4081f60faed98453d6aaa51eb1d942d2b": {
				"ticker": "CHEEMS",
				"address": "0x22A867b4081F60FAED98453d6aaa51Eb1d942D2b",
				"name": "CHEEMS INU",
				"decimals": 9
			},
			"0xd1620158a779135879a88e06476d244e9062de74": {
				"ticker": "WP",
				"address": "0xD1620158a779135879A88E06476d244E9062de74",
				"name": "Wealth Printer",
				"decimals": 18
			},
			"0x8add53423d8de1161a431d351a25777dbaf44d4a": {
				"ticker": "PYT",
				"address": "0x8add53423D8De1161a431d351A25777DbaF44d4A",
				"name": "PRINT YOUR TIME ",
				"decimals": 18
			},
			"0xafbf55577f4191f41ff114dd7d106bd83a46c56e": {
				"ticker": "BMAN",
				"address": "0xAfbF55577F4191f41Ff114dd7d106BD83a46c56e",
				"name": "Burning Man",
				"decimals": 9
			},
			"0xbfce8af3269623263be156fe0d9d425c5241a412": {
				"ticker": "WISP",
				"address": "0xbFcE8Af3269623263be156Fe0D9D425C5241A412",
				"name": "Wisp",
				"decimals": 5
			},
			"0xee9d8bb202ff9b847ab74b0dbf2a226c654117d0": {
				"ticker": "Token",
				"address": "0xee9d8bb202ff9b847ab74b0Dbf2a226C654117D0",
				"name": "Token",
				"decimals": 18
			},
			"0xf1aae7b5bfa76975f74b2d51c71ca281af0beb6d": {
				"ticker": "DRS",
				"address": "0xF1aae7B5Bfa76975f74B2D51c71ca281AF0beB6d",
				"name": "DAO REGUEE SOCIETY",
				"decimals": 18
			},
			"0xbd63785a900ddaea87dfb268e42372fac8c33550": {
				"ticker": "ENIGMA",
				"address": "0xbd63785a900dDAEA87Dfb268E42372faC8C33550",
				"name": "Enigma Protocol",
				"decimals": 9
			},
			"0xb11157dc044198b4dd60b36c56a97d6cce6f170e": {
				"ticker": "SNOW",
				"address": "0xb11157DC044198b4dd60B36C56A97d6ccE6f170e",
				"name": "SNOW",
				"decimals": 9
			},
			"0x3859e4756a1682b7c1db8be588088aee8764c86e": {
				"ticker": "METALAND",
				"address": "0x3859E4756a1682b7c1Db8Be588088aeE8764C86e",
				"name": "METALAND",
				"decimals": 8
			},
			"0x57500e82dcf7c2d59cf2b3f7f391fbf048bd6eb4": {
				"ticker": "Hyp",
				"address": "0x57500e82dCf7c2D59cF2B3f7F391FbF048BD6eb4",
				"name": "Hype Train",
				"decimals": 8
			},
			"0xd966d9601e0e1d45f74deaaaf086ae30e1afd2d5": {
				"ticker": "ETHP",
				"address": "0xD966D9601e0E1d45f74deaAAf086AE30e1AFD2d5",
				"name": "ETHPAY",
				"decimals": 9
			},
			"0x1c3c3a7138ddee7309aa71bbecf2834f4b3cb810": {
				"ticker": "STN",
				"address": "0x1c3C3a7138ddee7309AA71BBECF2834f4b3cB810",
				"name": "Sustain dao",
				"decimals": 18
			},
			"0x74c2a8a0161ff501d44a71c2e2dfdb5485912b1f": {
				"ticker": "PRMD",
				"address": "0x74c2A8a0161FF501d44A71C2E2dFDB5485912B1F",
				"name": "Pyramid",
				"decimals": 18
			},
			"0x336445619a32fbba78c24798f58b3c31e59ee820": {
				"ticker": "ICYp",
				"address": "0x336445619a32FbbA78C24798f58B3C31E59eE820",
				"name": "ICY PRINT",
				"decimals": 18
			},
			"0x6df21b31dfe76c54a67d7e500b27a819680be22e": {
				"ticker": "AYSB",
				"address": "0x6DF21B31DFe76c54A67D7E500b27A819680Be22E",
				"name": "AsYouShouldBe",
				"decimals": 9
			},
			"0xaf6700f6264799f3f1d580f2df20a487371a6093": {
				"ticker": "THRN",
				"address": "0xAF6700F6264799f3F1D580f2DF20A487371A6093",
				"name": "Thrones DAO",
				"decimals": 18
			},
			"0xd2a342c6a293032c6c8a16ce993043a30d14bcf3": {
				"ticker": "CAKE",
				"address": "0xd2a342c6A293032c6c8A16CE993043A30D14bcf3",
				"name": "OMG Cake",
				"decimals": 11
			},
			"0xa209e88be7f86852c46b9b895f7da0207d6d7dda": {
				"ticker": "AOHM",
				"address": "0xA209e88bE7f86852c46B9b895F7dA0207D6d7DDA",
				"name": "AVAOHM",
				"decimals": 18
			},
			"0x5503a0f98d30671028127b468566970f28be4d3d": {
				"ticker": "Chick",
				"address": "0x5503A0f98d30671028127B468566970f28Be4d3D",
				"name": "Chick Earn",
				"decimals": 18
			},
			"0x974af4dd5773efd1e6f199ddc9e66064c6efd317": {
				"ticker": "PEPE",
				"address": "0x974aF4Dd5773EFD1E6f199dDC9e66064C6EFD317",
				"name": "pepeBank",
				"decimals": 9
			},
			"0xd6ace76ae38008a37b9e871758854c169c2affee": {
				"ticker": "NINJA",
				"address": "0xd6aCe76aE38008A37b9E871758854C169C2AffEe",
				"name": "NINJA",
				"decimals": 6
			},
			"0x0b479dff0a54d667aeb3852f256680a27b0fede5": {
				"ticker": "VIPER",
				"address": "0x0B479DFf0A54D667Aeb3852f256680a27B0feDE5",
				"name": "VIPER",
				"decimals": 8
			},
			"0x20cdc4e7ad3bd0614a13c303933f994123d0b740": {
				"ticker": "TOX",
				"address": "0x20CDC4e7aD3bD0614A13C303933F994123d0b740",
				"name": "ToxinDAO",
				"decimals": 9
			},
			"0xbcd8824f5f7b94d4f279ba3ba693fdf48ac3841e": {
				"ticker": "MBC",
				"address": "0xBCD8824f5F7B94D4F279Ba3bA693FdF48Ac3841E",
				"name": "Multi Bank Capital",
				"decimals": 9
			},
			"0x68e4705b61d2e7a9bcccba11e4e76d85d4ed143e": {
				"ticker": "NSIS",
				"address": "0x68E4705B61D2E7A9bcCCBa11E4e76d85D4eD143E",
				"name": "Nemesis",
				"decimals": 18
			},
			"0x9028394e6dc6cbad4e3199fc456b3722db5aae50": {
				"ticker": "MNT",
				"address": "0x9028394e6dC6CbaD4e3199FC456b3722dB5aaE50",
				"name": "MOUNT NODES",
				"decimals": 18
			},
			"0xa0520a9b12d93fcf2a57b8472a23ecef4e3eb104": {
				"ticker": "DARKSIDE",
				"address": "0xa0520a9b12D93fCF2a57b8472a23ecef4E3Eb104",
				"name": "DARKSIDE",
				"decimals": 8
			},
			"0x295e2679713fc7254b38ac3e8435d614aa3ce9ff": {
				"ticker": "DD",
				"address": "0x295E2679713Fc7254b38Ac3E8435D614aa3ce9Ff",
				"name": "Dating Doge",
				"decimals": 18
			},
			"0x2cee629eabbe91e3387ba47a64eb0b30373fc9a0": {
				"ticker": "Come",
				"address": "0x2cEe629eaBbe91E3387BA47a64eb0b30373FC9A0",
				"name": "Comet DAO",
				"decimals": 8
			},
			"0xd1f7ecb4557fd4d149d08ecec99c2a44fd6cb6be": {
				"ticker": "PAR",
				"address": "0xD1F7ECB4557fd4D149D08eceC99C2a44FD6CB6Be",
				"name": "ParaDAO",
				"decimals": 9
			},
			"0x8df214b9511b6306982ec72514671c1113eef7ae": {
				"ticker": "LLBOG",
				"address": "0x8Df214B9511B6306982EC72514671c1113EEf7AE",
				"name": "LONG LIVE BOGDANOFF",
				"decimals": 18
			},
			"0x7ff069e135867976c7d02ef19c1cd05faf0b8be0": {
				"ticker": "SPRINT",
				"address": "0x7ff069E135867976c7D02EF19C1CD05FaF0b8bE0",
				"name": "SPELLPRINT",
				"decimals": 18
			},
			"0xb9c40190c6b234dc268316e3ca520a9772fc1f72": {
				"ticker": "MMAN",
				"address": "0xb9C40190c6b234dC268316E3ca520a9772Fc1f72",
				"name": "mountainmanavax.com",
				"decimals": 1
			},
			"0x434dfe9f45907da9e6fe9804dd289cc05ffd54cd": {
				"ticker": "AVPRNT",
				"address": "0x434DFE9f45907dA9e6Fe9804Dd289cC05ffd54cd",
				"name": "Avax Printer",
				"decimals": 9
			},
			"0xf6282cfa1117bb3585aa641e506675975acc4c38": {
				"ticker": "AD",
				"address": "0xF6282CFa1117bB3585Aa641e506675975Acc4c38",
				"name": "AVA DAO",
				"decimals": 18
			},
			"0xe7d06bb5079340bfd68bc3f5e52b5f65b01d75cd": {
				"ticker": "SPELLp",
				"address": "0xe7D06bB5079340BFD68BC3F5e52b5f65b01d75cd",
				"name": "SpellPrinter",
				"decimals": 18
			},
			"0x570471e5e2c6bf1dd5be0631ed1e9589c246fc02": {
				"ticker": "UND",
				"address": "0x570471E5E2C6BF1Dd5be0631ED1E9589C246fC02",
				"name": "UniverseNode",
				"decimals": 9
			},
			"0x2593ae552e2e1d5a18223d1defe01dcee2c54151": {
				"ticker": "FIRE",
				"address": "0x2593ae552e2e1d5A18223D1DEfe01DcEe2c54151",
				"name": "FireBank DAO",
				"decimals": 18
			},
			"0x17103cae8c45713c8f94c2c279d166b46480cc9a": {
				"ticker": "SIN",
				"address": "0x17103cae8C45713c8f94C2c279D166b46480cc9A",
				"name": "Syndicate Id Number",
				"decimals": 6
			},
			"0x27a70a0df15137ac7d1a320d7ea2e87966fa499e": {
				"ticker": "SAVAX",
				"address": "0x27a70A0DF15137Ac7d1A320D7ea2e87966Fa499e",
				"name": "ShinyAvaxToken",
				"decimals": 18
			},
			"0x729dec0569e25d41a16646a379e766ec439abe7a": {
				"ticker": "PLAYMATES",
				"address": "0x729DEC0569e25d41a16646A379e766Ec439Abe7a",
				"name": "Redlight Node District",
				"decimals": 1
			},
			"0xb2465127efe62c0b20692d132ceafd9c6caaefb8": {
				"ticker": "Saturn",
				"address": "0xB2465127efe62C0b20692D132cEAFD9c6cAaEfb8",
				"name": "Saturn Node",
				"decimals": 18
			},
			"0x49fa35e4973919bee9bf2e63f108400ce1a4a00f": {
				"ticker": "ORANGE",
				"address": "0x49fa35e4973919BEE9bF2e63f108400CE1A4a00F",
				"name": "Orange DAO",
				"decimals": 9
			},
			"0x86cacb08fafbf72778a310a68f8a990a128a612e": {
				"ticker": "LOST",
				"address": "0x86CacB08fAFbf72778A310a68f8a990a128A612E",
				"name": "Lost Worlds",
				"decimals": 18
			},
			"0x9e8cd582ab199bbe91bbe46eddf5a7341ebddca0": {
				"ticker": "BTRACTOR",
				"address": "0x9e8cD582aB199BbE91bBe46Eddf5a7341EbddCa0",
				"name": "TRACTOR JOE BABY",
				"decimals": 18
			},
			"0x0755fa2f4aa6311e1d7c19990416c86f17d16f86": {
				"ticker": "ETHP",
				"address": "0x0755FA2F4AA6311E1D7C19990416c86F17D16F86",
				"name": "ETHP",
				"decimals": 6
			},
			"0x2774ea726985583664a817f238f79eccb8604f83": {
				"ticker": "CATGIRL",
				"address": "0x2774Ea726985583664a817f238f79eCcb8604F83",
				"name": "CatGirl",
				"decimals": 18
			},
			"0x9c6dc61ba03272f73ac4dc24b837ef5ce399bfb8": {
				"ticker": "NAVY",
				"address": "0x9c6dc61ba03272f73Ac4DC24b837ef5CE399Bfb8",
				"name": "Pirate Nodes",
				"decimals": 1
			},
			"0x714163c14773fb2852e90608218905795f789da5": {
				"ticker": "cNode",
				"address": "0x714163c14773FB2852E90608218905795f789Da5",
				"name": "Chain Node",
				"decimals": 18
			},
			"0x909298825d78b0419ffc2d79e26296c9554e69b1": {
				"ticker": "VISA",
				"address": "0x909298825d78b0419fFC2d79e26296C9554E69B1",
				"name": "VISA",
				"decimals": 8
			},
			"0xfc5db99c2a95a02925f5f171819218ea7e89b368": {
				"ticker": "SNIPER",
				"address": "0xfC5Db99c2A95a02925F5F171819218EA7e89B368",
				"name": "Sniper Node Game",
				"decimals": 9
			},
			"0x72a024f6bf55a915534c6c9826d81b43821dcce0": {
				"ticker": "DRO",
				"address": "0x72A024F6BF55A915534C6c9826d81B43821dccE0",
				"name": "Distro Inu",
				"decimals": 9
			},
			"0x7fc191d4c21ed3099a03c03a46ac3f3dcccc7ca4": {
				"ticker": "GOLD",
				"address": "0x7Fc191D4C21eD3099A03C03a46aC3f3DCCcC7Ca4",
				"name": "Pirates Money",
				"decimals": 1
			},
			"0xc085e169c6f86538ab1a62e6e3e372ecf26711bb": {
				"ticker": "Cicada",
				"address": "0xc085e169c6f86538AB1A62e6E3e372ECf26711BB",
				"name": "Cicada",
				"decimals": 18
			},
			"0x293f2e276a4720628e7dd05693b3003d2e2e1f40": {
				"ticker": "ASTRO",
				"address": "0x293f2E276a4720628E7dD05693B3003D2E2E1f40",
				"name": "Astro Money",
				"decimals": 1
			},
			"0xa356e1ad58d35e114468bf64b1a8fd0ad1110d51": {
				"ticker": "EMN",
				"address": "0xA356E1aD58D35E114468bf64B1A8FD0aD1110D51",
				"name": "EmeraldNode",
				"decimals": 9
			},
			"0xe700f6a5ce8d5379b20f51f9cfea09cf89d2e05a": {
				"ticker": "CRL",
				"address": "0xe700F6A5cE8d5379b20f51f9CFeA09CF89D2e05A",
				"name": "Circle Nodes",
				"decimals": 18
			},
			"0xb18008a3ccee8023b4ca4b63c482e4581d3c7031": {
				"ticker": "CBG",
				"address": "0xB18008a3CceE8023b4cA4b63c482E4581d3c7031",
				"name": "Chainbing",
				"decimals": 9
			},
			"0x1b79a7c3f68e490347a43af88e22c116f156cb21": {
				"ticker": "BOOT",
				"address": "0x1B79A7C3F68E490347a43AF88e22C116F156cB21",
				"name": "BOOTS",
				"decimals": 18
			},
			"0x10c4a8ee756bd23013681fa8e53e5410ae0d3c62": {
				"ticker": "PRM",
				"address": "0x10c4a8ee756bd23013681fa8E53e5410Ae0D3c62",
				"name": "Prometheus DAO",
				"decimals": 9
			},
			"0x7960843b7e55cf19e6dab7398c8d99271573ae59": {
				"ticker": "HeC",
				"address": "0x7960843B7E55cF19E6Dab7398C8d99271573ae59",
				"name": "HeroesChained",
				"decimals": 8
			},
			"0xc5cf8ccf4ab8a0acafbf00b7a2206cd4894c2196": {
				"ticker": "ETHPPV2",
				"address": "0xc5Cf8cCf4aB8a0aCAFBF00b7A2206CD4894c2196",
				"name": "ETHPPV2",
				"decimals": 6
			},
			"0xe1c1a8dcd6ae8b17cc2923a82ddb9bf8827095b7": {
				"ticker": "CORE",
				"address": "0xE1C1a8DCD6aE8b17cC2923A82Ddb9bf8827095B7",
				"name": "Core Nodes",
				"decimals": 6
			},
			"0x35f85799fe350e34ff0e292ca48fb5f32fc028fc": {
				"ticker": "VRPAD",
				"address": "0x35f85799fE350e34fF0e292cA48fb5F32FC028Fc",
				"name": "VR GamePad",
				"decimals": 9
			},
			"0x4e28c592792da95421e2904c2272785308246f66": {
				"ticker": "NTC",
				"address": "0x4e28c592792da95421E2904c2272785308246f66",
				"name": "Nodes Treasury Capital",
				"decimals": 9
			},
			"0x9699b0ca70141087b0a804828bbbd746d1b7d6bd": {
				"ticker": "Crypt",
				"address": "0x9699b0cA70141087b0A804828bBbD746d1B7d6bD",
				"name": "Crypto King",
				"decimals": 8
			},
			"0x98749f8e74e8812a19c9fe97be34dabe6bea86ac": {
				"ticker": "TurboPump",
				"address": "0x98749F8e74E8812A19C9fE97bE34daBe6bea86AC",
				"name": "TurboPump",
				"decimals": 8
			},
			"0x37ab273ff19eecaf6bafd93c8c47ca50309ca7c3": {
				"ticker": "DGN",
				"address": "0x37Ab273FF19EeCaf6BaFd93C8c47CA50309ca7C3",
				"name": "Degen Game",
				"decimals": 18
			},
			"0x3cf9927d03f4ee1b4ea3bfbb5b983a1b19cbabda": {
				"ticker": "DubaiNC",
				"address": "0x3cF9927d03f4Ee1B4ea3bfBb5B983a1B19cbABDa",
				"name": "Dubai Node Capital",
				"decimals": 18
			},
			"0x2f455ff545642be1e69ecfbbff7db91ee451d4e5": {
				"ticker": "Rudolp",
				"address": "0x2f455FF545642Be1E69eCFBbfF7db91eE451D4E5",
				"name": "Rudolph Inu v2",
				"decimals": 18
			},
			"0x859b9701395a25e92afba8dd18b5aedafb138ef8": {
				"ticker": "uAVAX",
				"address": "0x859b9701395a25e92AFbA8dD18B5aEdafb138Ef8",
				"name": "uAVAX",
				"decimals": 18
			},
			"0x58df98831e4cd71d6aa3af7de3201fe7dfb8fef3": {
				"ticker": "WAGER",
				"address": "0x58DF98831e4CD71d6AA3Af7dE3201fe7DfB8feF3",
				"name": "Water Tiger 2022",
				"decimals": 18
			},
			"0x7812ff6fdc672fd04d11e07bc555f38428acfe10": {
				"ticker": "SANTA",
				"address": "0x7812Ff6FDc672fd04D11E07bC555F38428ACFe10",
				"name": "SANTA Token",
				"decimals": 18
			},
			"0xa6d3571c50dcb220794783fb08915e1df3b331b3": {
				"ticker": "SANTA",
				"address": "0xa6D3571C50dCb220794783fb08915E1df3B331b3",
				"name": "SnowSanta Dao",
				"decimals": 9
			},
			"0xafb2780cbb58b2af27023eb2a0e60c8ca0eee9bb": {
				"ticker": "BCDT",
				"address": "0xaFb2780CBb58b2AF27023Eb2a0e60c8Ca0eEE9bb",
				"name": "Blockchain Certified Data Token",
				"decimals": 18
			},
			"0xd197c8b939f199c7d70d030c1931d3c24442df60": {
				"ticker": "SLUSH",
				"address": "0xD197c8B939F199c7d70d030c1931d3C24442dF60",
				"name": "Slushie Capital",
				"decimals": 9
			},
			"0x0180979a1441f0718a38b95c10cf4c6c81fbb4e0": {
				"ticker": "SPELLPRINT",
				"address": "0x0180979A1441f0718a38B95C10cf4C6C81FBb4e0",
				"name": "SPELL PRINTER",
				"decimals": 18
			},
			"0xd1f5e76c187f978597fa8f027a040c0b268dede5": {
				"ticker": "PKM",
				"address": "0xd1F5e76c187f978597FA8F027a040c0b268DeDe5",
				"name": "PokemonII",
				"decimals": 18
			},
			"0x45ed9138247010c552d7f5c1b0832773afac7e60": {
				"ticker": "LIFE",
				"address": "0x45ed9138247010C552d7f5c1B0832773AfaC7e60",
				"name": "New Life",
				"decimals": 18
			},
			"0x9635646eb720031a515e76eea251d6ac9538bbc5": {
				"ticker": "dogAVAX",
				"address": "0x9635646eB720031A515E76EEA251d6ac9538BBC5",
				"name": "dogAVAX",
				"decimals": 8
			},
			"0x27d4835ca8bf17929f081b43eb2ad78d19c9c658": {
				"ticker": "MoonRock",
				"address": "0x27d4835Ca8bF17929f081b43EB2Ad78d19c9c658",
				"name": "MoonRock",
				"decimals": 8
			},
			"0x30a45c7173b180da3bae25564dd7f78324ac4318": {
				"ticker": "WBNBPPV2",
				"address": "0x30A45C7173b180dA3Bae25564dD7f78324AC4318",
				"name": "WBNBPPV2",
				"decimals": 6
			},
			"0x0db16b244bb1a9ca200b0f053040a1f3ab1363d1": {
				"ticker": "VOLTa",
				"address": "0x0Db16b244Bb1a9ca200B0f053040a1F3aB1363D1",
				"name": "Volta",
				"decimals": 9
			},
			"0xb89bda3bbac4509cd06dbc3e9aa98a0ec9f550ed": {
				"ticker": "AVSTD",
				"address": "0xB89BdA3BBac4509cd06dbc3e9Aa98a0ec9f550eD",
				"name": "Avax Standard",
				"decimals": 14
			},
			"0x1287bd32c4dd24dc8add1485fb7b7073555ed21a": {
				"ticker": "HINU",
				"address": "0x1287bD32c4dD24Dc8aDD1485FB7B7073555ed21a",
				"name": "Hayate INU",
				"decimals": 9
			},
			"0x3f5626f89d6d72681fa017d5badb61ee248fed06": {
				"ticker": "LEMON",
				"address": "0x3f5626F89D6d72681FA017D5bADb61ee248fed06",
				"name": "LEMON SWAP",
				"decimals": 8
			},
			"0x2d3928df293a56f8a0a384c6ea7b7c812ba54d93": {
				"ticker": "JOMAMA",
				"address": "0x2d3928Df293a56F8A0A384C6Ea7b7C812BA54d93",
				"name": "Joes Mama",
				"decimals": 9
			},
			"0xe6be133d17de8c6ba6d1600d8af69156e641f517": {
				"ticker": "QUEST",
				"address": "0xe6BE133D17De8C6Ba6D1600D8AF69156e641f517",
				"name": "QUEST NODES",
				"decimals": 9
			},
			"0xa088e7a17ad8c2102fc8645ad210704d36962673": {
				"ticker": "SHIELD",
				"address": "0xa088e7A17ad8C2102fC8645AD210704D36962673",
				"name": "Shield Finance",
				"decimals": 1
			},
			"0x08067084da109133498a318e74e3f47758b52635": {
				"ticker": "PANTHER",
				"address": "0x08067084dA109133498A318e74e3f47758b52635",
				"name": "PANTHER",
				"decimals": 8
			},
			"0x6990f0e17ffb1c5c0bcfe700e17f9bfc1c8fa431": {
				"ticker": "PEGT",
				"address": "0x6990f0e17fFB1C5C0BcFe700E17f9bfC1c8fa431",
				"name": "Pegasis DAO",
				"decimals": 18
			},
			"0xbff9bdd7c7c503bdcad70977ec26af92ea0f1561": {
				"ticker": "STRM",
				"address": "0xBff9bDd7c7c503BdcAD70977ec26Af92Ea0F1561",
				"name": "Storm Nodes",
				"decimals": 1
			},
			"0xb5623f774e568fed0259cf1c28a705405dd5967f": {
				"ticker": "MJEWEL",
				"address": "0xB5623F774e568FeD0259cF1c28A705405Dd5967f",
				"name": "MJEWEL",
				"decimals": 6
			},
			"0x925ecb5f5b1aa01319418a809dc5d231d675a1e0": {
				"ticker": "RMG",
				"address": "0x925eCB5f5B1aA01319418A809Dc5d231D675A1e0",
				"name": "RED META GALAXY",
				"decimals": 8
			},
			"0xf3f8772f92028bfb6d641c28bbcf1dbded424767": {
				"ticker": "MISTEL",
				"address": "0xf3F8772F92028bFb6D641c28bbcf1DbDEd424767",
				"name": "Mistel Finance",
				"decimals": 9
			},
			"0xf367949d944cc6d6e3cba5393365bd41c2b0514c": {
				"ticker": "Frog",
				"address": "0xf367949D944cc6d6E3cba5393365BD41c2B0514C",
				"name": "Frog Nation Dao",
				"decimals": 9
			},
			"0xded6d1e453dfe67ec3459dd1dd91e9528266d7ff": {
				"ticker": "SNOW",
				"address": "0xDeD6d1e453dfE67Ec3459dd1DD91e9528266D7FF",
				"name": "Snowflake DAO",
				"decimals": 9
			},
			"0xea4ef7572ad84c7b0742be145837b20eda375da4": {
				"ticker": "KITTY",
				"address": "0xEa4eF7572ad84c7b0742BE145837b20eda375DA4",
				"name": "Kity Kat Node",
				"decimals": 18
			},
			"0x075f84bfc36b39d29922c826295f0445b4ac48a8": {
				"ticker": "NGU",
				"address": "0x075f84bfC36b39d29922c826295F0445B4ac48a8",
				"name": "NumberGoUp",
				"decimals": 18
			},
			"0x983e65bd53b9f89e25d20c2c9ff837d149345c46": {
				"ticker": "CryptoGate",
				"address": "0x983e65BD53b9F89E25d20c2c9Ff837d149345C46",
				"name": "CryptoGate",
				"decimals": 8
			},
			"0x75b1ebdf426be85560d6cfbc323d7940ae08a62d": {
				"ticker": "BANKSY",
				"address": "0x75b1eBdf426bE85560D6cfbC323D7940AE08a62d",
				"name": "BANKSY FINANCE",
				"decimals": 8
			},
			"0xfcc5701143cb38c2e2f895aea476fc14a8dd4439": {
				"ticker": "Sandstorm",
				"address": "0xFcc5701143cb38c2e2f895AeA476Fc14a8Dd4439",
				"name": "Sandstorm",
				"decimals": 8
			},
			"0x90b5ffdd489557b9b8f72c3f15967b1e42e9502a": {
				"ticker": "WM",
				"address": "0x90B5FfdD489557B9b8f72c3f15967b1e42e9502a",
				"name": "Western Money",
				"decimals": 18
			},
			"0x84998e746e1618875fc14fde39ce3b2be3cdc7ef": {
				"ticker": "RUBY",
				"address": "0x84998e746E1618875FC14fDE39ce3B2be3cdC7EF",
				"name": "RUBY NODES",
				"decimals": 18
			},
			"0xb5a200abea0ce51fddf5542bd327af587599a5e0": {
				"ticker": "Weapon",
				"address": "0xb5a200ABEA0Ce51fddF5542Bd327AF587599a5e0",
				"name": "Mega Weapon",
				"decimals": 9
			},
			"0xa49db72ab0d87cdf57ff4f268a6fa333fb33595c": {
				"ticker": "AXP",
				"address": "0xa49dB72Ab0d87cdf57FF4F268a6fa333fB33595C",
				"name": "AXP",
				"decimals": 18
			},
			"0x44d84281498db79149bd0950642105fbff9f3ecb": {
				"ticker": "Orang",
				"address": "0x44d84281498Db79149bd0950642105FBFF9f3ecB",
				"name": "Orange DAO",
				"decimals": 2
			},
			"0x7f041ce89a2079873693207653b24c15b5e6a293": {
				"ticker": "LOOT",
				"address": "0x7f041ce89A2079873693207653b24C15B5e6A293",
				"name": "LOOT",
				"decimals": 18
			},
			"0x4b0c0ccfdf6dfd0300257a35ca39a73176626c57": {
				"ticker": "STEAM",
				"address": "0x4B0c0ccFDf6Dfd0300257a35cA39a73176626c57",
				"name": "STEAM",
				"decimals": 9
			},
			"0x4a2e71bf93be5028d431e5da02e0e9b57b027c7c": {
				"ticker": "SHIBL",
				"address": "0x4A2e71BF93be5028d431e5Da02E0E9B57b027c7c",
				"name": "Shiba Lovers",
				"decimals": 9
			},
			"0x6956c9018cc4573027d93420902a7efab3fb53d9": {
				"ticker": "CORGI",
				"address": "0x6956c9018cC4573027D93420902a7EfAB3fb53D9",
				"name": "Sushi Corgi",
				"decimals": 18
			},
			"0x598b287cc1cfd417a99fcbf48ea53c6fd4e48e6b": {
				"ticker": "AVALPS",
				"address": "0x598B287cC1cfd417a99fcBF48EA53c6fd4E48e6b",
				"name": "AvalpsDAO",
				"decimals": 9
			},
			"0x222ef4af02c07080770a454a5b194c185ad9353e": {
				"ticker": "PERFORMANCE",
				"address": "0x222Ef4af02c07080770a454a5b194C185Ad9353E",
				"name": "PERFORMANCE",
				"decimals": 8
			},
			"0x94e914c9334c7d05cab1dad741f06af356dc3dcd": {
				"ticker": "COMET",
				"address": "0x94e914c9334C7d05cAb1DaD741f06Af356dC3dCD",
				"name": "CometNODES",
				"decimals": 18
			},
			"0xae74a79d8b2e850a7ea6a4961d4ce1cd962a065f": {
				"ticker": "EYE.NODE",
				"address": "0xAe74a79D8B2e850a7ea6A4961D4CE1Cd962A065F",
				"name": "EYES NODEs",
				"decimals": 8
			},
			"0xdb3ce44d98f1963c8da5f5f3d5db31684ceb2b8e": {
				"ticker": "DRC.a",
				"address": "0xDb3ce44d98F1963C8DA5f5F3d5db31684ceb2b8E",
				"name": "DRC Avalanche",
				"decimals": 18
			},
			"0xeb26f8de9c37edf94fe30a52bb76ce8b8a9c29d7": {
				"ticker": "NRGY",
				"address": "0xEb26f8dE9C37EDf94FE30a52Bb76ce8B8A9C29d7",
				"name": "Wind Finance",
				"decimals": 18
			},
			"0xa7473c4456f6ebc60e6c498b956fd7dce5794fbe": {
				"ticker": "LTNM",
				"address": "0xA7473c4456f6Ebc60E6C498b956FD7dCE5794fbe",
				"name": "Bitcoin Latinum",
				"decimals": 9
			},
			"0x2bbf582f97fb5c52954910c66acf22a43079f91d": {
				"ticker": "Atom",
				"address": "0x2bBF582F97FB5C52954910c66aCF22A43079f91D",
				"name": "Atom Verse",
				"decimals": 18
			},
			"0xd5490d3fa6f07231eff895920242250395059930": {
				"ticker": "PTCL",
				"address": "0xD5490D3fa6F07231EFF895920242250395059930",
				"name": "Particle",
				"decimals": 9
			},
			"0x83ea813d64ac2927663db269cf7ae71db76c6ec9": {
				"ticker": "BMPRINT",
				"address": "0x83EA813D64aC2927663db269Cf7aE71DB76c6EC9",
				"name": "BOOFI MONEY PRINTER",
				"decimals": 18
			},
			"0x07800812d87accf37c2209b73c0ad128a4a1e6c7": {
				"ticker": "MFER",
				"address": "0x07800812D87ACcF37C2209b73c0Ad128A4a1e6C7",
				"name": "Mfernodes",
				"decimals": 18
			},
			"0x11c0b7d24b194c2955b55ba2a3889ed0e65ae0c9": {
				"ticker": "ASHIB",
				"address": "0x11C0B7d24b194C2955b55bA2a3889ed0E65Ae0c9",
				"name": "AVA SIBA INU",
				"decimals": 18
			},
			"0x1aff23615200d09965c3c810f213e9e193d75ffc": {
				"ticker": "AXAVA",
				"address": "0x1Aff23615200D09965c3C810f213e9E193D75FfC",
				"name": "ALIENXAVA",
				"decimals": 18
			},
			"0xbbf84680678c889da64ac6b6fff8fb814f444544": {
				"ticker": "GLT",
				"address": "0xBbf84680678c889Da64AC6b6FFF8FB814f444544",
				"name": "Zen Glitch",
				"decimals": 9
			},
			"0x3eec17cb537dd1e5a6669b1395cce303308b5e07": {
				"ticker": "Auror",
				"address": "0x3EeC17Cb537dD1e5A6669b1395CcE303308b5e07",
				"name": "Aurora Game",
				"decimals": 8
			},
			"0x67c9cfaf0d2aba34b15bd15fc02482809ecafcff": {
				"ticker": "WenRolex",
				"address": "0x67c9CFAF0d2Aba34B15BD15FC02482809ECAfcFF",
				"name": "WenRolex",
				"decimals": 9
			},
			"0xab547a8dc764408ce9dea41bdf689aeeb349da12": {
				"ticker": "OWL",
				"address": "0xAB547A8Dc764408Ce9DEA41bDf689aeEB349Da12",
				"name": "Owl Token",
				"decimals": 18
			},
			"0x18711b34d5f72b837848abd8be33a5be08fa7923": {
				"ticker": "POLAR",
				"address": "0x18711b34d5f72B837848abD8bE33a5be08fA7923",
				"name": "Polar Nodes",
				"decimals": 1
			},
			"0x5e1319d89698f7e452cff41fb099d1606716c336": {
				"ticker": "SNOWNODES",
				"address": "0x5e1319D89698f7E452CFf41fb099D1606716c336",
				"name": "SNOWFLAKE NODES",
				"decimals": 18
			},
			"0xe732853b8e8402c637bd901bc06961794a8281af": {
				"ticker": "🧿",
				"address": "0xe732853B8E8402C637bD901BC06961794a8281AF",
				"name": "illuminati",
				"decimals": 9
			},
			"0xf30585ac66933b44898d4e3faa5b44e43b98226e": {
				"ticker": "JOEP",
				"address": "0xf30585aC66933b44898D4E3faa5B44E43b98226e",
				"name": "Joe Printer",
				"decimals": 1
			},
			"0x17489997cbaa4cec6692a50da2e31d5fa9bb9821": {
				"ticker": "DGF",
				"address": "0x17489997CbAA4CeC6692A50DA2E31d5Fa9bb9821",
				"name": "DigitalFund",
				"decimals": 18
			},
			"0x9d959cc0746c854a075daa6eae3909b228b341a6": {
				"ticker": "DS",
				"address": "0x9D959CC0746c854a075daa6EAe3909b228b341A6",
				"name": "DefiSyndicate",
				"decimals": 18
			},
			"0x274d56cacf2651a1d3ab8cc95da5e0f747f1556f": {
				"ticker": "LVT",
				"address": "0x274D56CACf2651a1D3Ab8CC95da5e0f747F1556f",
				"name": "Louverture",
				"decimals": 18
			},
			"0x9e8bd057167ff3c4c2c9f3e516e3a27e4dd6af81": {
				"ticker": "VOLT",
				"address": "0x9E8Bd057167fF3c4C2c9f3E516e3a27e4dd6AF81",
				"name": "Fuse Nodes",
				"decimals": 9
			},
			"0xbfa0a82428102ad23f910ce17abf9fc828cca9fa": {
				"ticker": "FIRE",
				"address": "0xbfa0a82428102AD23F910CE17AbF9fC828Cca9fa",
				"name": "Fire Fist Nodes",
				"decimals": 1
			},
			"0xdfb79e342d63798e71f1fa65acbb5b4b2766cd4b": {
				"ticker": "WRABBIT",
				"address": "0xDFB79e342D63798E71f1Fa65AcBB5b4b2766CD4b",
				"name": "WhiteRabbit",
				"decimals": 18
			},
			"0xe43bdb2fa79ddcc3aa5ed6d377af74daf90e7c1f": {
				"ticker": "FAVAX",
				"address": "0xE43bdb2Fa79DDcC3aa5eD6D377af74Daf90e7c1F",
				"name": "AvaxFactory",
				"decimals": 18
			},
			"0xbd06493bc4643b5ae449460ea1fa4c231e3e88bc": {
				"ticker": "AvaxGate",
				"address": "0xbD06493BC4643B5AE449460Ea1Fa4c231E3e88bC",
				"name": "AvaxGate",
				"decimals": 9
			},
			"0x688b05aeef2eea5d272d4fce4ccb7647e4edadd5": {
				"ticker": "PTM",
				"address": "0x688b05aeef2EEA5D272d4FcE4cCB7647e4EDAdD5",
				"name": "PRINT THAT MIM  ",
				"decimals": 18
			},
			"0xa3b888269ade15d68b8a10e6e00b9df8139f9799": {
				"ticker": "CHASE",
				"address": "0xa3B888269AdE15D68B8A10E6E00b9Df8139f9799",
				"name": "ChaseAVAX",
				"decimals": 18
			},
			"0xf1480d8c6dc383d9d8033244e5db2d3751dd7fcc": {
				"ticker": "DeFi",
				"address": "0xF1480d8C6DC383D9d8033244E5Db2D3751dd7FCC",
				"name": "Defi Syndicate",
				"decimals": 18
			},
			"0x3c1dbb443799db2cd2d87a3ad407c38ada9d5bb5": {
				"ticker": "YETIDAO",
				"address": "0x3C1Dbb443799dB2CD2d87a3Ad407C38ADa9D5BB5",
				"name": "YetiDAO",
				"decimals": 9
			},
			"0xcb05f70296693e85ce2e0b2e0221d9d6cc902250": {
				"ticker": "Pixe",
				"address": "0xcB05f70296693E85Ce2E0B2e0221D9d6cC902250",
				"name": "Pixel Shiba",
				"decimals": 8
			},
			"0x01c5e2a4af2bd9f62c7d55c6105b8a64e6905be5": {
				"ticker": "Hatter",
				"address": "0x01c5E2a4af2bd9f62c7d55C6105b8a64e6905be5",
				"name": "Hatter",
				"decimals": 9
			},
			"0xfc9f19e6ee380ebf8757d73a12b1f13e5a39d534": {
				"ticker": "ADM",
				"address": "0xfc9f19e6ee380Ebf8757D73A12B1F13E5a39d534",
				"name": "Andromeda",
				"decimals": 8
			},
			"0x0ff75f82ef77456430b5cba2eecd6e7a7116a936": {
				"ticker": "SRA",
				"address": "0x0Ff75f82EF77456430B5Cba2eeCd6E7A7116A936",
				"name": "SierraDAO",
				"decimals": 1
			},
			"0x7f7d4657019043e37635c280119e97787022ab71": {
				"ticker": "CEO",
				"address": "0x7f7D4657019043E37635C280119E97787022Ab71",
				"name": "Tycoon CEO",
				"decimals": 9
			},
			"0x4ba98adf4f96e6b0a5ecb6ab41637db3c7923568": {
				"ticker": "FKA",
				"address": "0x4BA98ADf4f96e6B0A5ecb6ab41637dB3c7923568",
				"name": "Floki King Avax",
				"decimals": 9
			},
			"0xdd79dd0355a601b23af0c62075a2d08b0a1a415f": {
				"ticker": "BTime",
				"address": "0xdd79Dd0355a601B23af0c62075A2d08B0a1A415f",
				"name": "babyTime",
				"decimals": 9
			},
			"0x245eff4ca225f64ce07ef25770783cf9be0c827a": {
				"ticker": "CHEEMS",
				"address": "0x245Eff4cA225F64Ce07eF25770783cF9BE0c827A",
				"name": "CHEEMS INU V2",
				"decimals": 9
			},
			"0x62054a1dee7b4a34388c93f299087f6b760dc9d0": {
				"ticker": "SSHIB",
				"address": "0x62054a1deE7B4A34388c93F299087f6b760Dc9D0",
				"name": "SnowShib",
				"decimals": 18
			},
			"0x5869f911b60c16c17011b9a7fef4514f82745b70": {
				"ticker": "MIAP",
				"address": "0x5869f911B60C16c17011B9A7FEF4514F82745B70",
				"name": "MagicInternetAP",
				"decimals": 18
			},
			"0x03bf0a1d5f72f31d64cf2c66d0e10eaf7171d506": {
				"ticker": "BOOBA",
				"address": "0x03BF0a1D5F72F31D64CF2c66D0E10Eaf7171D506",
				"name": "BoobaDao",
				"decimals": 9
			},
			"0x28f3de6b04eaa6ab46975315a7ad5da3be639146": {
				"ticker": "RND",
				"address": "0x28F3DE6b04eaA6ab46975315a7ad5dA3be639146",
				"name": "Redlight Node District",
				"decimals": 9
			},
			"0xd769bdfc0caee933dc0a047c7dbad2ec42cfb3e2": {
				"ticker": "CHART",
				"address": "0xD769bDFc0CaEe933dc0a047C7dBad2Ec42CFb3E2",
				"name": "ChartEx",
				"decimals": 18
			},
			"0xc28900b02cd7ab9ede69ea873827c6ed90f9ee24": {
				"ticker": "NAKAMOTO",
				"address": "0xC28900B02Cd7AB9eDE69ea873827c6ED90f9Ee24",
				"name": "NAKAMOTO",
				"decimals": 8
			},
			"0x237fd7109efbc2be82c091aaaaafab21fb227e29": {
				"ticker": "FEES",
				"address": "0x237Fd7109eFBc2BE82C091AaaAAfab21fB227E29",
				"name": "FEES PROTOCOL",
				"decimals": 8
			},
			"0xe95d16c5d0bf64bea3d85813b11a81da27fce332": {
				"ticker": "SATURN",
				"address": "0xe95D16C5D0bf64BeA3d85813b11A81Da27fce332",
				"name": "Saturn Money",
				"decimals": 1
			},
			"0xc50eb6c984e172f69ec5b219d815622c39eb01be": {
				"ticker": "EMRLD",
				"address": "0xc50eB6c984E172F69EC5B219D815622C39EB01BE",
				"name": "Emerald Protocol V2",
				"decimals": 9
			},
			"0x54e90234257f58075c3da580ab4f02e30a5a2d62": {
				"ticker": "HOST",
				"address": "0x54E90234257F58075C3dA580AB4f02E30A5a2D62",
				"name": "HOST",
				"decimals": 18
			},
			"0xbc8c68965a46c94d17b94ada1ffb28cb08a0288d": {
				"ticker": "FORCE",
				"address": "0xBc8c68965A46C94D17B94aDa1Ffb28CB08A0288D",
				"name": "StarWars Avax",
				"decimals": 18
			},
			"0x7f64a65c0d38d4150d73178d869663dcce4c0141": {
				"ticker": "SNOW",
				"address": "0x7f64A65C0d38D4150D73178D869663DccE4c0141",
				"name": "SnowToken",
				"decimals": 18
			},
			"0x0972a3d9f777d5f7da164f56661720a5ae1b5069": {
				"ticker": "Honey",
				"address": "0x0972A3D9F777d5f7dA164F56661720A5Ae1B5069",
				"name": "Honey",
				"decimals": 18
			},
			"0x532ecd67d9ef25fc7a2156b0c52a67573480fdcb": {
				"ticker": "INF",
				"address": "0x532eCd67d9eF25Fc7a2156B0C52A67573480FdcB",
				"name": "Infinity Verse",
				"decimals": 1
			},
			"0x124c13b3c595373178deb09517a4bb9b7bb438f7": {
				"ticker": "PLAR",
				"address": "0x124C13B3c595373178DEb09517a4bB9b7bB438F7",
				"name": "Polar DAO",
				"decimals": 18
			},
			"0x55b1a124c04a54eefdefe5fa2ef5f852fb5f2f26": {
				"ticker": "ETHM",
				"address": "0x55b1a124c04A54eeFDEFE5FA2Ef5f852FB5f2f26",
				"name": "Ethereum Meta",
				"decimals": 18
			},
			"0x4913ec7b945deb195b967ece52a3c7655ab0e918": {
				"ticker": "MTC",
				"address": "0x4913EC7B945DEb195b967ece52a3c7655ab0e918",
				"name": "MotleyCapital",
				"decimals": 18
			},
			"0xd9725696540f26c463efbd8c3edd5205820ba107": {
				"ticker": "MIC",
				"address": "0xd9725696540f26c463eFBd8C3edd5205820bA107",
				"name": "MicroCap",
				"decimals": 9
			},
			"0x924952d98bca1a65b9e27efcd8fccaeb9208724c": {
				"ticker": "BOOBA",
				"address": "0x924952d98bCA1a65B9E27EfCd8FCcAEB9208724c",
				"name": "BoobaDao",
				"decimals": 9
			},
			"0xe1a238df6d27faa7d86e88601448742c87c7b2f1": {
				"ticker": "SUNRISE",
				"address": "0xe1A238DF6d27FaA7d86E88601448742C87c7b2f1",
				"name": "SUNRISE",
				"decimals": 8
			},
			"0x97e11edca170e0df8877df1da373a5fcb93f4548": {
				"ticker": "BINGO",
				"address": "0x97E11EdCA170e0DF8877dF1dA373a5fCB93F4548",
				"name": "BINGO ",
				"decimals": 8
			},
			"0xdb2dbcc27f3f830ec03311a7478403f5ae82e6e4": {
				"ticker": "SDAI",
				"address": "0xdb2dbCc27f3F830Ec03311A7478403F5Ae82e6E4",
				"name": "Secret DAI",
				"decimals": 9
			},
			"0xe440bc4339d1a28dccac889d5ac4260129aef423": {
				"ticker": "LUAN",
				"address": "0xe440Bc4339d1A28DccaC889d5ac4260129AeF423",
				"name": "AVALuan",
				"decimals": 18
			},
			"0x147d3283db78263cc6f0bc9d6008625c77b9d2d6": {
				"ticker": "PLE",
				"address": "0x147D3283DB78263cC6f0bc9d6008625C77b9d2d6",
				"name": "Plethori",
				"decimals": 1
			},
			"0x5e430f88d1be82eb3ef92b6ff06125168fd5dcf2": {
				"ticker": "DIBS",
				"address": "0x5E430F88D1BE82EB3eF92b6fF06125168fD5DCf2",
				"name": "dibs.money",
				"decimals": 18
			},
			"0x568d6c37e4bb13e37ec4651ffe8752199d62732a": {
				"ticker": "TokenV2",
				"address": "0x568d6c37e4BB13E37Ec4651fFE8752199D62732a",
				"name": "TokenV2",
				"decimals": 18
			},
			"0xc1ca8254f7c8f588a101b2440250dfc5081b883b": {
				"ticker": "BosonDAO",
				"address": "0xC1CA8254f7c8F588A101B2440250DFc5081B883b",
				"name": "BosonDAO",
				"decimals": 18
			},
			"0xd1aae43b7577782ee98a5b40a1b1bf0900e2287b": {
				"ticker": "Star",
				"address": "0xd1AAe43B7577782Ee98a5B40a1B1bF0900e2287b",
				"name": "StarNodes",
				"decimals": 1
			},
			"0x38c74c90cb32ea5c5f8f074b6149cea263fe7e83": {
				"ticker": "BATTLE",
				"address": "0x38C74C90Cb32eA5c5f8f074b6149cea263FE7e83",
				"name": "Battle Node Game",
				"decimals": 9
			},
			"0x87ead12d6d0b9e6996b0e43f6d79ad2e7ac83e7a": {
				"ticker": "SMRT",
				"address": "0x87eAD12D6d0B9e6996B0e43F6d79Ad2e7ac83e7a",
				"name": "Smart Money Nodes",
				"decimals": 1
			},
			"0x5426fcaaa9b59c9eea27d2c5814f963a56c3673c": {
				"ticker": "RISEN",
				"address": "0x5426FCAAa9B59C9EEA27d2c5814F963A56C3673c",
				"name": "Nodes Rise",
				"decimals": 9
			},
			"0xa1fe84b4ff423add5033ce82b3d53a672a1e22b1": {
				"ticker": "UNR",
				"address": "0xa1Fe84b4fF423Add5033Ce82B3d53a672A1E22b1",
				"name": "UNIRocket",
				"decimals": 9
			},
			"0xce0052745a182fff07acb9dd834041e5f29319b2": {
				"ticker": "HIGGS",
				"address": "0xce0052745A182fff07aCB9dd834041E5F29319b2",
				"name": "Higgs",
				"decimals": 18
			},
			"0xdf9e874894196919f7b2bcae336463103dea0e3d": {
				"ticker": "PLAGUE",
				"address": "0xDF9E874894196919f7B2bCae336463103DEA0E3D",
				"name": "PlagueDao.com",
				"decimals": 9
			},
			"0xf69d33c61668ac92a0a90592a173a77dee19ddbe": {
				"ticker": "NOBELP",
				"address": "0xF69D33C61668Ac92a0a90592A173a77DeE19DdBe",
				"name": "NobeliumPrinter",
				"decimals": 9
			},
			"0x29debf83eb8bc91ce9a13ac7e42d37e6e2abb480": {
				"ticker": "KDAO",
				"address": "0x29DEBf83Eb8Bc91ce9A13Ac7E42D37E6e2Abb480",
				"name": "KDAO",
				"decimals": 9
			},
			"0xa0b81e95a4b7e5c5b6b23a64f98d7f8e66471f78": {
				"ticker": "ANDROMEDA",
				"address": "0xa0b81e95A4B7e5c5B6b23a64F98d7f8e66471F78",
				"name": "ANDROMEDA",
				"decimals": 2
			},
			"0xda2392bc52ea78d419cbef4b71842680047b9617": {
				"ticker": "JUICE",
				"address": "0xDa2392BC52Ea78D419CBEf4B71842680047B9617",
				"name": "Space Farmers",
				"decimals": 18
			},
			"0xfc83490b4e4c8723bf4615ba8f96963a4f9129c3": {
				"ticker": "OCEAN",
				"address": "0xFc83490b4e4C8723bF4615Ba8F96963a4f9129C3",
				"name": "Ocean Money",
				"decimals": 1
			},
			"0x22552014c32966f3b6be27ed9d6d52d5c9452985": {
				"ticker": "AXI",
				"address": "0x22552014c32966F3b6bE27eD9D6D52D5C9452985",
				"name": "Avax Suki - avaxsuki.com",
				"decimals": 1
			},
			"0x2b2a2f2891f3a666513599ca7935a181db5e9ed6": {
				"ticker": "SHREK",
				"address": "0x2B2a2f2891f3A666513599ca7935a181Db5E9eD6",
				"name": "Shrek Token",
				"decimals": 18
			},
			"0x472d21510a227176ae740f1acdf79d964a3dc1f8": {
				"ticker": "AVAXUP",
				"address": "0x472d21510A227176ae740f1acdf79d964A3dC1f8",
				"name": "AvaxUp",
				"decimals": 9
			},
			"0x1d87c1c7c192d97d617e441139ead6a682f2c0b9": {
				"ticker": "bJoe",
				"address": "0x1D87c1C7C192D97d617E441139EAD6a682f2C0b9",
				"name": "BabyJoe",
				"decimals": 9
			},
			"0xb912bb9fcbfef42e8dfb3fa50a937ceacb5d60ed": {
				"ticker": "BANTER",
				"address": "0xb912BB9fcBfEF42e8dFb3fA50A937ceacb5D60Ed",
				"name": "CryptoBanter",
				"decimals": 9
			},
			"0xc4fb2404bbd356ce0a108b96c7ecc9fa033b18de": {
				"ticker": "PFIZER",
				"address": "0xc4FB2404bBd356Ce0A108B96C7EcC9fa033b18de",
				"name": "Pfizer Nodes",
				"decimals": 2
			},
			"0xf5d0584bbe28b9aa738e27521bbcecfe246082a5": {
				"ticker": "SEA",
				"address": "0xF5D0584bbE28b9aa738e27521bBcecfE246082a5",
				"name": "OpenSea.io Bridge",
				"decimals": 9
			},
			"0xe10292acae724692e473dde4f4668b2612f5cf8a": {
				"ticker": "WonderGame",
				"address": "0xE10292acAe724692E473dDe4F4668B2612f5cf8A",
				"name": "WonderGame",
				"decimals": 8
			},
			"0xd4fd6a2b8dc1faf375f280afa7f8fda728271730": {
				"ticker": "GN",
				"address": "0xd4fD6a2B8dc1Faf375F280afa7F8Fda728271730",
				"name": "Good Night",
				"decimals": 18
			},
			"0xe52a4e6fbbce10ca405f74bb4bb3051dcb0ca376": {
				"ticker": "FCAT",
				"address": "0xe52a4E6fbbCE10cA405F74BB4bb3051DCB0ca376",
				"name": "FLUFFY NEEKO",
				"decimals": 18
			},
			"0x6c4253c4bfbc08d813787dc5541da8ac3b52efe0": {
				"ticker": "AQUA",
				"address": "0x6c4253C4bfBC08D813787DC5541da8aC3b52EFE0",
				"name": "Aqua Nodes",
				"decimals": 1
			},
			"0x5ef0663ec871758e4dc37cb0d2e2f1c2dfbb2baf": {
				"ticker": "FRUIT",
				"address": "0x5EF0663EC871758e4dc37Cb0d2E2f1c2DFBB2baf",
				"name": "Fruit Nodes",
				"decimals": 18
			},
			"0x41894faf95ddadca17cceceb6ddb2a1d91ae2703": {
				"ticker": "3Node",
				"address": "0x41894faF95dDadCa17ccECEB6DdB2A1D91AE2703",
				"name": "3Node.finance",
				"decimals": 18
			},
			"0x6afd5a1ea4b793cc1526d6dc7e99a608b356ef7b": {
				"ticker": "STORM",
				"address": "0x6AFD5A1ea4b793CC1526d6Dc7e99A608b356eF7b",
				"name": " STORM Token",
				"decimals": 18
			},
			"0xb317d3b9f1fc46d81f80444a168d3e71dd97f74c": {
				"ticker": "LEO",
				"address": "0xB317D3b9F1fc46d81F80444A168d3E71dd97f74c",
				"name": "Leonidas",
				"decimals": 5
			},
			"0xe82e4cc509e541c44e6c4bcbaadd8067e5bafa5a": {
				"ticker": "ADOGE",
				"address": "0xE82E4Cc509e541C44e6C4bcBaAdd8067E5bAFa5a",
				"name": "Avadoge",
				"decimals": 9
			},
			"0xf1e83357c7c93278e1f4752c8762c1f8a6fc91ac": {
				"ticker": "JP",
				"address": "0xf1E83357c7C93278E1f4752C8762c1F8A6Fc91ac",
				"name": "JEWEL PRINTER",
				"decimals": 18
			},
			"0x2239460ae2534578f57171411cf1716b36984c1e": {
				"ticker": "GRIMEX",
				"address": "0x2239460aE2534578f57171411cF1716b36984c1e",
				"name": "SpaceGrime",
				"decimals": 18
			},
			"0x1d0cff8b96045d6b338cf330eae66249e3572a6f": {
				"ticker": "OOTH",
				"address": "0x1D0cfF8b96045D6B338cF330eaE66249E3572A6F",
				"name": "Oothereum",
				"decimals": 18
			},
			"0xc2c2e0654b9ffa998d2f01021bb52ee949bbd5b8": {
				"ticker": "💎",
				"address": "0xc2C2e0654B9fFA998d2F01021bB52eE949bbd5B8",
				"name": "DiamondPump",
				"decimals": 9
			},
			"0x2b6a134b3ea64adfe0f467bc262d4e56f19183f9": {
				"ticker": "LDAO",
				"address": "0x2b6A134b3EA64adfE0F467bc262D4e56f19183F9",
				"name": "Life DAO",
				"decimals": 9
			},
			"0x6622779f02bf8c9476d2d9f32c2811bb4ef60f9a": {
				"ticker": "PLAR",
				"address": "0x6622779f02bF8c9476D2D9F32C2811bB4eF60F9A",
				"name": "PolarDAO",
				"decimals": 18
			},
			"0x74084bff114dc0925f18f2b4ea7294af5d8b94c1": {
				"ticker": "ARMOR",
				"address": "0x74084BfF114dc0925f18f2b4Ea7294af5D8B94C1",
				"name": "ARMOR PROTOCOL",
				"decimals": 8
			},
			"0xe15597df046a660cc5b4101dd7184391541563ef": {
				"ticker": "PENTA",
				"address": "0xE15597dF046a660cc5b4101Dd7184391541563Ef",
				"name": "Pentagon Nodes",
				"decimals": 18
			},
			"0x126390a5a74d2404286ed282c01c85836b1981d4": {
				"ticker": "OXY",
				"address": "0x126390a5A74d2404286Ed282c01c85836B1981D4",
				"name": "OXY-FI",
				"decimals": 1
			},
			"0x9cee03512a9a251bb41b43d52b113d2df4b4efc4": {
				"ticker": "CASH",
				"address": "0x9CEE03512a9A251BB41b43d52B113d2DF4b4EFc4",
				"name": "CASH",
				"decimals": 8
			},
			"0x77e4972d83a69eef10db6d45350a1065dbd0cf03": {
				"ticker": "COM",
				"address": "0x77e4972D83A69eEf10db6D45350A1065DbD0cf03",
				"name": "Comet Money",
				"decimals": 18
			},
			"0x67116baaa5177eb08dce1017261b17b54444e024": {
				"ticker": "BABYLINK",
				"address": "0x67116bAAa5177Eb08dcE1017261b17B54444e024",
				"name": "https://babylink.xyz BABYLINK",
				"decimals": 9
			},
			"0xcda21fefcf2172f3f4a314c0c701c0cff5183ada": {
				"ticker": "RShare",
				"address": "0xcDa21fEFCf2172F3f4A314C0c701C0cff5183aDA",
				"name": "RShare",
				"decimals": 18
			},
			"0xacd465d4843caa46ffbb265455290e04cbfba6c5": {
				"ticker": "STRONG",
				"address": "0xacd465d4843cAA46FFbB265455290e04cBfbA6C5",
				"name": "Strong Finance",
				"decimals": 1
			},
			"0x353dce1bdad904b9350997c464f9139c1313417c": {
				"ticker": "IdeaChain",
				"address": "0x353DcE1BDAD904b9350997c464F9139c1313417C",
				"name": "IdeaChain",
				"decimals": 6
			},
			"0xb1d6feaeb6cc2e35814daf0eecb8f3578505e8ca": {
				"ticker": "META",
				"address": "0xb1d6fEaeb6cc2E35814daF0eEcB8f3578505E8cA",
				"name": "MetaFloki",
				"decimals": 18
			},
			"0xffb3723d6a24dda355850c07d08a26d199a53991": {
				"ticker": "THUNDER",
				"address": "0xFFB3723D6a24Dda355850C07D08a26d199a53991",
				"name": "Thunder",
				"decimals": 18
			},
			"0x31c88451ca41fc16062782838c17da359c80a7b2": {
				"ticker": "JP",
				"address": "0x31C88451Ca41fC16062782838c17da359c80a7b2",
				"name": "JEWEL PRINT",
				"decimals": 18
			},
			"0xd6468471f075944a276f8df41bc1d2f04f903e29": {
				"ticker": "BASE.p",
				"address": "0xd6468471f075944A276F8dF41bC1d2f04f903e29",
				"name": "BASE PROTOCOL",
				"decimals": 8
			},
			"0x1b1a02c7f627d42a8291bf915382e17c08a3e791": {
				"ticker": "ATM",
				"address": "0x1B1a02c7f627D42a8291bf915382E17c08A3E791",
				"name": "Atom Protocol",
				"decimals": 18
			},
			"0x78088b6af65e585124ab084eb20454958c69dca1": {
				"ticker": "Island",
				"address": "0x78088b6af65e585124Ab084eB20454958C69DCA1",
				"name": "Island",
				"decimals": 18
			},
			"0x61ecd63e42c27415696e10864d70ecea4aa11289": {
				"ticker": "RUGPULL",
				"address": "0x61eCd63e42C27415696e10864d70ecEA4aA11289",
				"name": "RUGPULL",
				"decimals": 18
			},
			"0xc5dc2a345d38e8648c5d944d93a8ea666c959517": {
				"ticker": "simpDAO",
				"address": "0xC5dc2A345d38E8648c5D944D93A8Ea666c959517",
				"name": "simpDAO",
				"decimals": 9
			},
			"0xac3b3e0a8d91b8e268f3124a6d07eaa2031bfad7": {
				"ticker": "GLDC",
				"address": "0xAC3b3e0a8d91b8e268f3124a6d07eaA2031BFAd7",
				"name": "Gold Coin",
				"decimals": 11
			},
			"0xc510043c8d4d2cb5e220000fb6af445f8b9930be": {
				"ticker": "Cobr",
				"address": "0xC510043c8D4d2cb5e220000fB6af445f8B9930bE",
				"name": "Cobra Swap",
				"decimals": 8
			},
			"0xed38643906fcb6bbe739fecfe3b89d66dd871ef4": {
				"ticker": "FC",
				"address": "0xEd38643906fCb6bbE739feCFe3B89d66dD871ef4",
				"name": "FC",
				"decimals": 9
			},
			"0xb0b55e0feacf6308fb11254fbd22c2eeed45ed5f": {
				"ticker": "SHUEYR",
				"address": "0xB0b55e0FEAcf6308fb11254fbd22C2eEEd45ed5f",
				"name": "Shuey Rhon Rhon Avax",
				"decimals": 18
			},
			"0x10d1d005b662907e10d1a88a8ebb1cd646ae37b1": {
				"ticker": "BOSON",
				"address": "0x10d1d005B662907E10d1a88a8EbB1cD646Ae37b1",
				"name": "BosonDAO",
				"decimals": 18
			},
			"0x1bf78f4921f643af9384a0702619feb09beea554": {
				"ticker": "TYPH",
				"address": "0x1bF78F4921f643aF9384A0702619feB09bEeA554",
				"name": "Typhoon token",
				"decimals": 18
			},
			"0x96973185b205baddffa182cd088e73e6a635b674": {
				"ticker": "GALAX",
				"address": "0x96973185b205bAddfFA182cD088e73E6A635b674",
				"name": "GALAXYCOIN",
				"decimals": 18
			},
			"0x81bcea22cc43b285d6f6c39ba2a72bb99f707008": {
				"ticker": "BUSHIDO",
				"address": "0x81BCeA22CC43b285d6f6C39Ba2a72bb99f707008",
				"name": "BushidoDAO",
				"decimals": 18
			},
			"0x319255e2807bac991b7e68b9ba0f9c201b5f0c88": {
				"ticker": "Moon",
				"address": "0x319255e2807Bac991b7e68B9Ba0F9C201b5f0C88",
				"name": "MoonFlash DAO",
				"decimals": 18
			},
			"0xd5daa2dc9aca69033d5b1337f818cea8e3edd053": {
				"ticker": "SHEEPv2",
				"address": "0xD5dAa2DC9ACA69033d5b1337f818CEA8e3eDd053",
				"name": "SheepGame V2",
				"decimals": 9
			},
			"0xf1fc836b7345acad53c9353861876fa0a52952d0": {
				"ticker": "$RETH",
				"address": "0xf1fC836B7345ACad53C9353861876Fa0A52952D0",
				"name": "RETHINK",
				"decimals": 18
			},
			"0xd37a8a2b11fcc9b2b2fd4a979ebdcfeeb645c15d": {
				"ticker": "SWING",
				"address": "0xd37a8A2B11FCC9B2B2fd4a979ebDCFEeB645C15d",
				"name": "Swing Dao",
				"decimals": 18
			},
			"0x9802b0181535dd4dfc89d257230f62c87e363249": {
				"ticker": "WHL",
				"address": "0x9802B0181535Dd4dfc89D257230F62c87E363249",
				"name": "Whale Nodes",
				"decimals": 18
			},
			"0x0029583419a9eb345d095ad1a7d41789bc8cfe58": {
				"ticker": "COSMOS",
				"address": "0x0029583419A9eB345D095AD1a7d41789BC8cFe58",
				"name": "COSMOS",
				"decimals": 6
			},
			"0x5debbe2b6f857271381db05fd980b6e01bdb9338": {
				"ticker": "ELMNT",
				"address": "0x5dEbBe2b6f857271381db05fD980B6E01bdb9338",
				"name": "ELEMENTS NODES",
				"decimals": 18
			},
			"0x94a462c9165f8a1d26b79b1e5a95652578e56aef": {
				"ticker": "THOR",
				"address": "0x94A462C9165f8A1d26B79b1e5A95652578E56aef",
				"name": "THORHAMMER NODES",
				"decimals": 1
			},
			"0xa465745a0c31aede8d56519c87ea44d51ea643e7": {
				"ticker": "CRYPTO",
				"address": "0xA465745a0c31aEdE8d56519c87eA44D51ea643E7",
				"name": "CryptoCoin",
				"decimals": 18
			},
			"0x00b4f18a0e116050b1699c8aff25da8681722467": {
				"ticker": "OW",
				"address": "0x00b4F18a0E116050B1699c8Aff25Da8681722467",
				"name": "OceanWave",
				"decimals": 9
			},
			"0x9c8e99eb130aed653ef90fed709d9c3e9cc8b269": {
				"ticker": "HTZ",
				"address": "0x9C8E99eb130AED653Ef90fED709D9C3E9cC8b269",
				"name": "Hertz Token",
				"decimals": 18
			},
			"0x2f867f28f1f2c8eb36a01d2e44324a9b2e86d885": {
				"ticker": "CRABP",
				"address": "0x2F867f28f1f2c8EB36A01d2e44324a9b2e86d885",
				"name": "Crabada Printer",
				"decimals": 9
			},
			"0xab241b9e5acbcea4157b52600cdbc5a73da0242b": {
				"ticker": "SEA",
				"address": "0xAB241B9e5aCbCeA4157B52600CDBC5a73DA0242b",
				"name": "Deep Sea",
				"decimals": 9
			},
			"0xecee7b9b7aa9eff9f89f1fe7fc77a2e5f6e491ea": {
				"ticker": "WDOGEALLY",
				"address": "0xECeE7B9B7aa9efF9f89f1fE7fC77a2e5f6e491eA",
				"name": "Wrapped Doge Alliance",
				"decimals": 18
			},
			"0xe2e2739d9634980842695f1eaab85ee61b03c458": {
				"ticker": "BOOFIp",
				"address": "0xE2E2739D9634980842695F1eaAb85ee61B03c458",
				"name": "BOOFI PRINTER",
				"decimals": 18
			},
			"0x0782e6aab9836be918a110aa216a5da3525e2528": {
				"ticker": "BLCKDAO",
				"address": "0x0782E6aAb9836be918A110AA216a5da3525E2528",
				"name": "BlackSecretDAO",
				"decimals": 18
			},
			"0x80c6762e21a679447c35062d016f4ff18847dca4": {
				"ticker": "ETH",
				"address": "0x80C6762E21A679447C35062d016F4fF18847DCa4",
				"name": "ETH NODES",
				"decimals": 9
			},
			"0xdf7c54fef58daff3a662eafb014c2074a343ce50": {
				"ticker": "EIGHT",
				"address": "0xDf7c54FEf58dAFf3a662EAFB014C2074a343cE50",
				"name": "Eight Finance",
				"decimals": 18
			},
			"0x62ebda79ae83402b2c426e6114fef075ea9cb79d": {
				"ticker": "DREAM",
				"address": "0x62EbDA79aE83402B2C426e6114FEF075eA9cb79D",
				"name": "Dream Token",
				"decimals": 18
			},
			"0x0190e9b1bfc8daad3af565e9dc626844ffd35508": {
				"ticker": "KishuTama",
				"address": "0x0190E9b1BfC8daAD3AF565E9Dc626844Ffd35508",
				"name": "KishuTama",
				"decimals": 9
			},
			"0x43b8ffbefe4dcb7c0c0df4216ab12b33836072d8": {
				"ticker": "GLOBE",
				"address": "0x43B8FFbEfE4DCb7c0c0df4216ab12b33836072d8",
				"name": "Global Finance",
				"decimals": 18
			},
			"0x523e8a3b59ecc58ce7827fdce9c0919858cae948": {
				"ticker": "SNOVA",
				"address": "0x523E8a3B59Ecc58cE7827fdCe9C0919858cae948",
				"name": "SuperNova Money",
				"decimals": 1
			},
			"0xc9fd748d2f9df527dca69f8a7f776a9906e3eed6": {
				"ticker": "xGAS",
				"address": "0xC9FD748D2F9Df527DcA69F8a7F776a9906e3eED6",
				"name": "Gas DAO AVAX",
				"decimals": 1
			},
			"0xb44dc1e6c2d9871ba48139acf46ae161df7fb2eb": {
				"ticker": "AUDI",
				"address": "0xb44dC1E6c2d9871Ba48139acF46aE161Df7fB2Eb",
				"name": "AUDIT DAO",
				"decimals": 8
			},
			"0x4c73449a0dbda326bdd20236c1aa79b45142becc": {
				"ticker": "BOOBA",
				"address": "0x4C73449A0DbDA326BDd20236C1aA79b45142bECC",
				"name": "BoobaDao",
				"decimals": 9
			},
			"0x4e6963bb2abebc009c4e11d651829bf185f99339": {
				"ticker": "cyberWav",
				"address": "0x4E6963bb2ABebc009c4E11d651829BF185f99339",
				"name": "cyberWave ",
				"decimals": 9
			},
			"0x942556699c1190b050d1c8ea79a1b2d517a25057": {
				"ticker": "wFTM.p",
				"address": "0x942556699c1190b050d1C8Ea79a1B2D517a25057",
				"name": "wFTM.p",
				"decimals": 8
			},
			"0x90cd7097cacd42aac2060bfdabbddeeae0e6a1da": {
				"ticker": "Pear",
				"address": "0x90Cd7097cACD42AAc2060bFDAbbDdeEAe0e6a1Da",
				"name": "Pearl Jam",
				"decimals": 6
			},
			"0x948a4a95a9d0bc642e2b37f29c5d181231e80afa": {
				"ticker": "Shinja",
				"address": "0x948a4A95a9D0bC642E2b37f29c5d181231e80AFA",
				"name": "SHIBNOBI",
				"decimals": 8
			},
			"0xbdd046e50f480209fcda88fe0b6a624e91057375": {
				"ticker": "SQUIDX",
				"address": "0xbDd046e50f480209FcdA88fE0b6a624e91057375",
				"name": "SQUIDGAME AVAX",
				"decimals": 18
			},
			"0x805302fea76b8f1559a5fd9c7056f7b0b0d1265f": {
				"ticker": "STRONG",
				"address": "0x805302FEa76B8F1559A5Fd9C7056F7B0B0D1265f",
				"name": "StrongHold Nodes",
				"decimals": 1
			},
			"0x58a6af6df37b138bcc10001434bb94abef19baa5": {
				"ticker": "REACT",
				"address": "0x58A6aF6df37B138Bcc10001434Bb94abEf19bAa5",
				"name": "Rebase_Aggregator_Capital",
				"decimals": 18
			},
			"0xf05e236a139cb19851cd5568a85094d6ee331fac": {
				"ticker": "ZSHARE",
				"address": "0xF05e236A139CB19851cD5568A85094D6EE331fAc",
				"name": "ZSHARE",
				"decimals": 18
			},
			"0x1b35ecf4c451c7e8a89bcd6dc694a52e27dab1d7": {
				"ticker": "PLTN",
				"address": "0x1B35ECf4c451C7E8A89bcd6dC694A52e27dAb1D7",
				"name": "Plutonium Node",
				"decimals": 12
			},
			"0x54f59dcceb37adc5486ea45bbb0fe0d333c41f85": {
				"ticker": "ZEUS",
				"address": "0x54f59DCCeb37aDc5486Ea45BBb0fe0d333c41F85",
				"name": "Zeus Finance",
				"decimals": 1
			},
			"0xd3397eca4248dc63218df60671aa6d975f4ef4b4": {
				"ticker": "KoalaSwap",
				"address": "0xD3397eCa4248Dc63218DF60671Aa6d975F4eF4b4",
				"name": "KoalaSwap",
				"decimals": 8
			},
			"0xd606199557c8ab6f4cc70bd03facc96ca576f142": {
				"ticker": "GDL",
				"address": "0xD606199557c8Ab6F4Cc70bD03FaCc96ca576f142",
				"name": "Gondola",
				"decimals": 18
			},
			"0x4f9a1d52269ac335e01ee1c0f5e9cd0e50df4737": {
				"ticker": "REACT",
				"address": "0x4f9a1d52269AC335e01Ee1c0F5e9cd0E50Df4737",
				"name": "Rebase_Aggregator_Capital",
				"decimals": 18
			},
			"0x30c8b918b5c16b33fcb178d26f7f787555b94ee8": {
				"ticker": "SanctionInu",
				"address": "0x30c8b918b5C16B33FCb178D26f7F787555B94ee8",
				"name": "SanctionInu",
				"decimals": 18
			},
			"0xd7e7013beebf5d3de2b5c5457736bafff5d92f31": {
				"ticker": "NATR",
				"address": "0xD7E7013bEEBf5D3dE2B5C5457736BAFfF5D92f31",
				"name": "Dark Forest",
				"decimals": 9
			},
			"0x1d680cd07f24cf3c7e48f9e4e2742aebcefdfb47": {
				"ticker": "EVM",
				"address": "0x1d680Cd07F24cF3c7E48F9e4e2742AEBcEFdfB47",
				"name": "EVM PROTOCOL",
				"decimals": 8
			},
			"0x4e8f3f08a5c32504a37477564a0ad08743b65cca": {
				"ticker": "BEAST",
				"address": "0x4e8F3F08A5C32504a37477564a0aD08743b65CCA",
				"name": "Beast Nodes",
				"decimals": 18
			},
			"0x6092300432269a94cb8efa211f964f9783f16e1e": {
				"ticker": "Aprico",
				"address": "0x6092300432269A94Cb8eFA211F964F9783f16E1e",
				"name": "Apricot Protocol",
				"decimals": 8
			},
			"0x02d2e1135dbe76456c148e6f9eef23e913973e4a": {
				"ticker": "RoboHero",
				"address": "0x02D2e1135DBe76456c148e6F9eeF23E913973E4A",
				"name": "RoboHero",
				"decimals": 8
			},
			"0xdeabb6e80141f5e557ecbdd7e9580f37d7bbc371": {
				"ticker": "PGL",
				"address": "0xdeaBb6e80141F5E557EcBDD7e9580F37D7BBc371",
				"name": "Pangolin Liquidity",
				"decimals": 18
			},
			"0x3ab6689fa33dded3488f0c452a1e4bde65345aed": {
				"ticker": "MIAP",
				"address": "0x3AB6689Fa33dDed3488F0C452A1E4bdE65345AeD",
				"name": "MagicInternetAP",
				"decimals": 1
			},
			"0xe668f8030bf17f3931a3069f31f4fa56efe9dd54": {
				"ticker": "WSPP",
				"address": "0xe668f8030bf17f3931A3069f31f4Fa56EFE9DD54",
				"name": "WolfSafePoorPeople",
				"decimals": 18
			},
			"0x4b65624cd437a8358e86c36cff4d284ce1cd0472": {
				"ticker": "BASE",
				"address": "0x4B65624cD437a8358e86c36Cff4d284ce1CD0472",
				"name": "BASE PROTOCOL",
				"decimals": 8
			},
			"0xef31b4fed4a8db1813fc4e739bb92be1821a01c7": {
				"ticker": "HRKT",
				"address": "0xef31b4fEd4A8db1813Fc4E739Bb92Be1821A01C7",
				"name": "Hero Kitty",
				"decimals": 18
			},
			"0x73b4f12b7a679e05b4d6310eaf4326e85b86bd50": {
				"ticker": "Yield",
				"address": "0x73b4F12B7a679e05b4D6310EAF4326E85B86Bd50",
				"name": "Yield Owl",
				"decimals": 18
			},
			"0x52fabb67761bcd14849e8c85c267c50606e78ef4": {
				"ticker": "Sifu",
				"address": "0x52fABB67761bcd14849e8C85c267C50606E78Ef4",
				"name": "Sifucoin",
				"decimals": 18
			},
			"0x92ca4f731fdc7a790959285405828fdc8c017186": {
				"ticker": "SHDW",
				"address": "0x92Ca4F731fdC7A790959285405828FdC8C017186",
				"name": "SHADOW NODES",
				"decimals": 1
			},
			"0xe148becb115d848430ca39c661bfaa0d47e20c63": {
				"ticker": "DREX",
				"address": "0xe148BECB115D848430cA39c661BfaA0D47e20C63",
				"name": "DREX",
				"decimals": 9
			},
			"0x7b9e29d252bdddaee235b21ce12cf8581b497a7c": {
				"ticker": "RISEN",
				"address": "0x7b9E29d252BDddAee235b21Ce12cf8581B497A7C",
				"name": "Nodes Rise",
				"decimals": 18
			},
			"0xee0a8a9c432311e001aee14372f0bbe399becbeb": {
				"ticker": "HeC",
				"address": "0xeE0A8A9C432311E001AeE14372f0BbE399BECBEb",
				"name": "HeroesChained",
				"decimals": 8
			},
			"0x2934046a6a7d47eb0adeebcac8e5f67717793807": {
				"ticker": "ATM",
				"address": "0x2934046A6a7d47Eb0aDEebCAc8e5f67717793807",
				"name": "Atom Protocol",
				"decimals": 9
			},
			"0xa2c4a30e2870937ccf7690240b07a1b1b440f7da": {
				"ticker": "DIABLO",
				"address": "0xA2c4A30E2870937CCF7690240b07A1B1B440f7da",
				"name": "DIABLO",
				"decimals": 1
			},
			"0x6007fca39b5398feac4d06d75435a564a086bab8": {
				"ticker": "SPC",
				"address": "0x6007FCA39B5398FeaC4D06D75435A564A086Bab8",
				"name": "Spicycoin",
				"decimals": 9
			},
			"0x6988d814a41f11153e4898c6a0c4303d8159fb6a": {
				"ticker": "WebRTC",
				"address": "0x6988d814a41f11153E4898c6A0C4303d8159fB6A",
				"name": "WebRTC",
				"decimals": 8
			},
			"0x931c2f247a98e4dddc05bda75b5aad757a609e29": {
				"ticker": "Ange",
				"address": "0x931c2F247a98E4DdDC05bDa75B5AAD757a609e29",
				"name": "Angel Wing",
				"decimals": 8
			},
			"0xc70e87479019600c54fb58f48b30ecb8d6e68a30": {
				"ticker": "EMPIRE",
				"address": "0xC70E87479019600c54fb58f48B30ecB8d6E68a30",
				"name": "Empire Nodes",
				"decimals": 1
			},
			"0x5dac87d6d3214dec641ae4144404e6ef296b9df8": {
				"ticker": "RDD",
				"address": "0x5dAC87D6d3214DeC641ae4144404e6ef296b9Df8",
				"name": "RugDocDAO",
				"decimals": 9
			},
			"0x1de5e3bbbd2ca6cd6f0ea64f31238ba12e47b507": {
				"ticker": "APOCALYPSE",
				"address": "0x1De5E3bBbD2cA6Cd6F0Ea64F31238Ba12e47B507",
				"name": "APOCALYPSE",
				"decimals": 4
			},
			"0x05a1d18bcf676c35a7e5b7f5abed4084257f8382": {
				"ticker": "FB",
				"address": "0x05a1d18bcf676C35A7E5B7f5aBED4084257f8382",
				"name": "Fire Bank",
				"decimals": 6
			},
			"0xda643017d007966741d8394a293d4efc843a0f49": {
				"ticker": "AVCLA",
				"address": "0xDa643017D007966741D8394A293D4EFc843a0F49",
				"name": "Avax Cola",
				"decimals": 18
			},
			"0x53aa4868680decdc58585f1b1b2aa3a900a60db8": {
				"ticker": "ATM",
				"address": "0x53aA4868680dECdC58585F1b1B2Aa3a900a60DB8",
				"name": "Atom Protocol",
				"decimals": 18
			},
			"0xfd9dd15d81d78711e4d76b7f8aede6f49f02c5b8": {
				"ticker": "XOMB",
				"address": "0xFd9dd15d81d78711E4d76B7f8AeDE6f49f02C5b8",
				"name": "Xomb Finance",
				"decimals": 18
			},
			"0x80c6c7cfdfd696f9db42faedac62d397711e64b6": {
				"ticker": "AZT",
				"address": "0x80C6C7CfDFD696f9DB42fAEDAC62d397711e64B6",
				"name": "AztecDAO",
				"decimals": 18
			},
			"0xef9571dfd324b4408bdb77f3cfbb5b3caa51a5b5": {
				"ticker": "ATM",
				"address": "0xEF9571dFD324b4408bdB77f3CFbb5B3CAA51A5b5",
				"name": "Atom",
				"decimals": 18
			},
			"0x5575b755d83b68c3bb748e9018a6ef9ae53ed44a": {
				"ticker": "QUANT",
				"address": "0x5575b755d83B68C3Bb748e9018a6EF9Ae53eD44a",
				"name": "Quant Protocol",
				"decimals": 18
			},
			"0xf72d5440b7f1ad38b52cbd8d35bd6a1d16428f60": {
				"ticker": "ISLAND",
				"address": "0xF72D5440B7F1aD38B52CBD8d35BD6a1d16428f60",
				"name": "The Island",
				"decimals": 9
			},
			"0x3ad55555df23a3b488c1b00b72fcf2613c5aa0e4": {
				"ticker": "ETHP",
				"address": "0x3aD55555df23a3B488c1b00B72FCF2613c5aa0e4",
				"name": "ETHPv2",
				"decimals": 1
			},
			"0x975369edb3393a19d78a63b79a3a20d3b9218eb8": {
				"ticker": "LVTp",
				"address": "0x975369edb3393A19D78a63b79A3A20d3b9218eb8",
				"name": "LouverturePrinter",
				"decimals": 6
			},
			"0xa6f7a50503e1a8dea61bd89414386bb268edfba5": {
				"ticker": "PINU",
				"address": "0xa6F7a50503e1a8dEA61bD89414386bb268eDfBA5",
				"name": "Piccolo Inu",
				"decimals": 9
			},
			"0x66b39b2f4bf041f397b382cbd22afed29d88fdc0": {
				"ticker": "Crypt",
				"address": "0x66b39b2f4Bf041F397B382CBD22afEd29d88FDC0",
				"name": "Crypto Chamber",
				"decimals": 8
			},
			"0xfa669e048d9ef9a66a78791f7227d7acef0ccae0": {
				"ticker": "JAEL",
				"address": "0xfA669E048D9EF9A66a78791f7227D7ACef0cCae0",
				"name": "Trader Jew",
				"decimals": 18
			},
			"0xc759b50062f372b3eb9f0b161a9056b10e7baa09": {
				"ticker": "STRN",
				"address": "0xC759B50062F372b3Eb9F0b161a9056B10E7bAa09",
				"name": "Saturn Finance",
				"decimals": 18
			},
			"0x4eb49a2f9a79053866fae806fac95a3ef5b92c05": {
				"ticker": "CAKE",
				"address": "0x4Eb49A2F9A79053866faE806fAc95A3eF5B92C05",
				"name": "CakeDao",
				"decimals": 9
			},
			"0xf3c0409787b1180bd9f3ce8663324e043b83f7d1": {
				"ticker": "SHIT",
				"address": "0xF3C0409787b1180BD9F3Ce8663324e043b83f7d1",
				"name": "Shitcoiners",
				"decimals": 18
			},
			"0xc16091be41e3126b497ddc25427fd7bfb4f354b3": {
				"ticker": "UMAM",
				"address": "0xc16091bE41E3126B497Ddc25427fd7bFb4f354B3",
				"name": "https://umam.link Use Me Abuse Me",
				"decimals": 18
			},
			"0x00132e496c73262594e4eedb3a870bcdeadff38b": {
				"ticker": "RDS",
				"address": "0x00132e496c73262594e4eedB3a870BCDeaDff38B",
				"name": "ROGUE DAO SOCIETY",
				"decimals": 18
			},
			"0x46fccf447f74ec97a521615fce111118597071f7": {
				"ticker": "RGK",
				"address": "0x46fCCf447f74ec97a521615fcE111118597071f7",
				"name": "Ragnarok",
				"decimals": 9
			},
			"0x7a261f45b99740e7c628df6bc02e256428842541": {
				"ticker": "NITRO",
				"address": "0x7A261f45b99740e7C628dF6BC02e256428842541",
				"name": "Nitro Nodes",
				"decimals": 1
			},
			"0xba056eae1dd8264ceaa8aa8cfc126bba935a31ed": {
				"ticker": "CLE",
				"address": "0xba056eaE1dd8264CeAa8Aa8cFC126BBa935A31eD",
				"name": "CLE",
				"decimals": 9
			},
			"0x333551387314367e489d329ccba09f08636d4bab": {
				"ticker": "GOL",
				"address": "0x333551387314367e489D329ccbA09F08636d4baB",
				"name": "GOLD POCKET",
				"decimals": 8
			},
			"0xe0c3beff2f0e2d37134917b5c782f87d9b61d22c": {
				"ticker": "HoN.p",
				"address": "0xe0C3BefF2f0E2D37134917b5c782f87D9B61D22c",
				"name": "HoN printer",
				"decimals": 8
			},
			"0xb59d9daa9589fe3a4775216abb44b3e9e39ee0ad": {
				"ticker": "LOL",
				"address": "0xb59D9daA9589fe3A4775216Abb44b3e9E39Ee0AD",
				"name": "League of Legends",
				"decimals": 8
			},
			"0xc5ed556fd16479bd1d9c95598ce6552a4847cb0c": {
				"ticker": "GriezmannDAO",
				"address": "0xc5Ed556fd16479BD1d9C95598Ce6552A4847cB0C",
				"name": "GriezmannDAO",
				"decimals": 18
			},
			"0x6e542d1ae0d144b93526086ff15c423847bb6135": {
				"ticker": "Unam",
				"address": "0x6e542d1ae0d144b93526086Ff15C423847Bb6135",
				"name": "Inspired by Umam",
				"decimals": 9
			},
			"0xd73b0914957d49125a8a464bf2425a1bdf35901b": {
				"ticker": "RISEN",
				"address": "0xd73B0914957d49125A8A464bF2425a1bdf35901B",
				"name": "NodesRise",
				"decimals": 18
			},
			"0xe486e76e7dd885d96889c548ea136ee13dda52c9": {
				"ticker": "MYSTIC",
				"address": "0xE486E76e7dd885d96889C548Ea136Ee13DDA52c9",
				"name": "Mystic",
				"decimals": 18
			},
			"0xd60effed653f3f1b69047f2d2dc4e808a548767b": {
				"ticker": "UFARM",
				"address": "0xd60efFED653f3f1B69047F2D2dC4E808A548767b",
				"name": "UNIFARM Token",
				"decimals": 18
			},
			"0xf838962cc12bdaa2db258f0ce1c674d27be8e5d5": {
				"ticker": "BLOCK",
				"address": "0xf838962Cc12bDAa2DB258f0CE1C674D27Be8E5D5",
				"name": "BLOCK DAO",
				"decimals": 9
			},
			"0xbfee87a8536e9191ac2efed14ce203d517e1650d": {
				"ticker": "ZILLA",
				"address": "0xbfEE87a8536E9191Ac2EFed14CE203D517E1650d",
				"name": "Zilla Token",
				"decimals": 18
			},
			"0x1e68c60f5bc87190f5e0da50f5001a4e422d50e2": {
				"ticker": "EVOL",
				"address": "0x1E68C60f5bc87190f5e0Da50F5001A4E422D50e2",
				"name": "Evolution",
				"decimals": 18
			},
			"0x818db6402ad69feef9637183cfc7d95a12b10f4d": {
				"ticker": "Gig",
				"address": "0x818DB6402AD69feEF9637183cfc7d95A12b10f4d",
				"name": "Giga Chad",
				"decimals": 8
			},
			"0x3972c772c768466d505e2c263ef98113b5752c80": {
				"ticker": "BOTWAR",
				"address": "0x3972c772C768466D505E2C263EF98113b5752c80",
				"name": "BOTWAR",
				"decimals": 9
			},
			"0x061efcbbfdfb7ad3b05c8ec9a724ca9411927024": {
				"ticker": "Spac",
				"address": "0x061EFcbbFdFB7aD3b05c8Ec9a724ca9411927024",
				"name": "Space Link",
				"decimals": 8
			},
			"0x52a3b1e4fb89adb784f58a667bac57e398b2ecde": {
				"ticker": "INFECT",
				"address": "0x52a3b1e4Fb89aDb784f58a667bAC57e398b2ECde",
				"name": "InfectionDAO",
				"decimals": 9
			},
			"0x033b62e0d064cf097f47f57cd72160dfe47ed502": {
				"ticker": "FINO",
				"address": "0x033b62e0D064cF097f47F57CD72160Dfe47ed502",
				"name": "FINO.FINANCE",
				"decimals": 18
			},
			"0x9a8e86ebd09432b30a25a9ca4ae9a7b273e0c04a": {
				"ticker": "DOE",
				"address": "0x9A8E86ebD09432B30a25a9ca4ae9A7b273E0C04a",
				"name": "Dogs Of Elon",
				"decimals": 18
			},
			"0xafa31b961cc03bbac5e92af24994f32ebee1a6f6": {
				"ticker": "SMR",
				"address": "0xafA31B961cc03BBac5e92Af24994F32EbEE1A6F6",
				"name": "Samurai Shiba",
				"decimals": 18
			},
			"0xdd0a1d20b616c49f8f9eac050f110f9e47dcf3e5": {
				"ticker": "NASAM",
				"address": "0xDD0A1d20B616c49F8f9Eac050F110F9e47dcF3E5",
				"name": "NASA Metaverse",
				"decimals": 9
			},
			"0xb180883ef761a1d5eb100f744b9f6cb3472d284e": {
				"ticker": "MIMP",
				"address": "0xB180883EF761A1d5EB100f744B9F6cb3472D284E",
				"name": "MIMPRINTER",
				"decimals": 9
			},
			"0x554e53f430e55f6a22ba220e605dd6f2f96c0b4f": {
				"ticker": "CINU",
				"address": "0x554E53F430E55F6a22bA220E605DD6f2f96C0b4f",
				"name": "CHEEMSINU",
				"decimals": 8
			},
			"0xbad50884170109a854b678de25900414a7096b7b": {
				"ticker": "wAVAX.p",
				"address": "0xBad50884170109a854b678de25900414a7096b7B",
				"name": "wAVAX PRINTER",
				"decimals": 8
			},
			"0x20fc9927ab198d6be0ba05388416bac667f6f1e9": {
				"ticker": "NEWTON",
				"address": "0x20fc9927aB198d6BE0ba05388416bAc667F6F1E9",
				"name": "NEWTON",
				"decimals": 8
			},
			"0x41bd38a9ad7548f4a93982c4fea60825628abc39": {
				"ticker": "THOR",
				"address": "0x41bD38a9AD7548F4A93982C4fea60825628abC39",
				"name": "Thors Hammers Node",
				"decimals": 1
			},
			"0xab637afc1e477a6f0f424cf38ea56afe6028d268": {
				"ticker": "RND",
				"address": "0xAb637afC1e477A6F0f424cF38eA56AFe6028D268",
				"name": "Redlight Node District",
				"decimals": 9
			},
			"0x23ba4272a5c5b5641c751521d57e3413c6703131": {
				"ticker": "Sins",
				"address": "0x23ba4272A5c5b5641c751521d57E3413C6703131",
				"name": "SIN",
				"decimals": 9
			},
			"0x71e01195c1b698333a7d0e7c3a68511e831c5a03": {
				"ticker": "STLLR",
				"address": "0x71E01195C1B698333A7d0E7C3A68511e831C5a03",
				"name": "Stellar Nodes",
				"decimals": 18
			},
			"0xba5092d8239b63dfe81382c15485604c85d43dfa": {
				"ticker": "WAGMI",
				"address": "0xBa5092d8239B63DFE81382C15485604c85D43DFa",
				"name": "WAGMI",
				"decimals": 9
			},
			"0x7664d35e23515ecf9c2ebf625c1a88091ee18285": {
				"ticker": "ZEUS",
				"address": "0x7664d35e23515Ecf9c2EBf625c1a88091eE18285",
				"name": "Zeus v2",
				"decimals": 18
			},
			"0x4a26b6bcbedaca376616b39d91d1c55205079885": {
				"ticker": "PF",
				"address": "0x4a26B6bCbeDaca376616b39D91D1c55205079885",
				"name": "Pixel Forest",
				"decimals": 18
			},
			"0x1d2a4e19a3bf230380e7a05b01925055fe977b0b": {
				"ticker": "APEB",
				"address": "0x1d2a4e19A3BF230380E7a05b01925055fe977b0B",
				"name": "APE Burn",
				"decimals": 18
			},
			"0xeecde2c870d8b17e5dd158faac8e6a87f1513fbd": {
				"ticker": "PEAK",
				"address": "0xEEcDE2C870D8B17e5dD158FaAC8e6A87F1513fBD",
				"name": "SummitDAO",
				"decimals": 9
			},
			"0xb95e47530179af128a7c0e870073f6232c0a8526": {
				"ticker": "CHERRY",
				"address": "0xB95e47530179aF128a7C0e870073f6232c0A8526",
				"name": "Cherry Finance",
				"decimals": 18
			},
			"0x8c0727382ea4ed9a18542c733009fe3cb171d784": {
				"ticker": "SPARTA",
				"address": "0x8C0727382EA4Ed9a18542C733009fE3CB171d784",
				"name": "SPARTA",
				"decimals": 8
			},
			"0xf6bf11cbe8685422045142311613397a00fee527": {
				"ticker": "CHICK",
				"address": "0xf6bF11CbE8685422045142311613397A00fEe527",
				"name": "CHICK DAO",
				"decimals": 18
			},
			"0x698384611a03382891a1f3ffb33526820ffd12f9": {
				"ticker": "Testicle",
				"address": "0x698384611a03382891a1F3ffB33526820fFD12F9",
				"name": "Testicle Printer",
				"decimals": 18
			},
			"0x5a9b174c24267b5992a0f5d40de2790b0445d77a": {
				"ticker": "LAVAX",
				"address": "0x5a9b174C24267b5992A0f5d40dE2790B0445d77a",
				"name": "LUCKY AVAX",
				"decimals": 18
			},
			"0xcaf5191fc480f43e4df80106c7695eca56e48b18": {
				"ticker": "AKITA",
				"address": "0xcaF5191fc480F43e4DF80106c7695ECA56E48B18",
				"name": "Akita Inu",
				"decimals": 18
			},
			"0xa976670b315ff50879e2a5d3a4f27f333f63f523": {
				"ticker": "BTCM",
				"address": "0xa976670B315Ff50879e2a5D3A4F27F333f63f523",
				"name": " AVAX CHICKS",
				"decimals": 9
			},
			"0x229730965977a252b46cf8f843a761657762fe20": {
				"ticker": "Volta",
				"address": "0x229730965977A252b46cF8F843A761657762Fe20",
				"name": "BitcoinMining",
				"decimals": 9
			},
			"0x31b504ac705034d09665dc9c631dc1fbc95e2a0b": {
				"ticker": "FIAT",
				"address": "0x31b504Ac705034D09665Dc9C631dC1fBC95E2a0b",
				"name": "VoltaDAO",
				"decimals": 18
			},
			"0x45cdaf3fd17bd31d9830fa977159162dd2431683": {
				"ticker": "KIOO",
				"address": "0x45CdaF3Fd17BD31d9830Fa977159162DD2431683",
				"name": "FIAT",
				"decimals": 18
			},
			"0x2fc1258a03e158a2a1049c4151d511d40115d30a": {
				"ticker": "ANCAP",
				"address": "0x2FC1258a03e158a2a1049c4151d511d40115D30A",
				"name": "KIOO",
				"decimals": 18
			},
			"0x8043ce8c9dacb79506af2728a43e538eb87eb097": {
				"ticker": "ISLAND",
				"address": "0x8043ce8C9dAcB79506aF2728A43e538Eb87Eb097",
				"name": "Anarcho Capital",
				"decimals": 9
			},
			"0x6c6364292ce564f9a12b89da3705101a2be20159": {
				"ticker": "PGL",
				"address": "0x6C6364292CE564f9a12B89DA3705101A2BE20159",
				"name": "Pangolin Liquidity",
				"decimals": 18
			},
			"0x16a7cf0d5855ecebbb3b7261e853be64de0ad961": {
				"ticker": "SamouraiShiba",
				"address": "0x16a7cf0d5855EcebbB3B7261e853be64De0ad961",
				"name": "SamouraiShiba",
				"decimals": 9
			},
			"0x883e5172b118c479eca4442d1ed07ea1257ff755": {
				"ticker": "ORCHI",
				"address": "0x883e5172b118C479EcA4442D1ED07ea1257FF755",
				"name": "ORCHID PROTOCOL",
				"decimals": 8
			},
			"0x339fdaaf2087266cec70bc388437f8fc2c4ea144": {
				"ticker": "ATLAS",
				"address": "0x339FdAAf2087266CEC70Bc388437f8FC2c4Ea144",
				"name": "Atlas Finance",
				"decimals": 18
			},
			"0x9977f9feb0da5e60f609a4851a648ea6df56f7cb": {
				"ticker": "FNP",
				"address": "0x9977f9FEB0da5E60F609a4851a648eA6df56F7cB",
				"name": "FIRENEST PROTOCOL",
				"decimals": 1
			},
			"0xb6feeca48573a8d0f7805b74bdba6e95ba7fb8df": {
				"ticker": "WallSt",
				"address": "0xb6FEeCa48573A8d0F7805B74bdBa6E95BA7Fb8df",
				"name": "WallStreet Dao",
				"decimals": 18
			},
			"0x0ebd9537a25f56713e34c45b38f421a1e7191469": {
				"ticker": "OOE",
				"address": "0x0ebd9537A25f56713E34c45b38F421A1e7191469",
				"name": "OpenOcean",
				"decimals": 18
			},
			"0x10409bcdb4a193a20104a7c4bae0b3d15a91d88b": {
				"ticker": "WINK",
				"address": "0x10409bcDb4a193A20104a7c4bAE0b3D15a91D88B",
				"name": "Wink Money",
				"decimals": 18
			},
			"0xd26acaaee8be66f9770a520dfc9f155a95b064b4": {
				"ticker": "XMAS",
				"address": "0xD26AcAAEE8be66f9770A520dfc9f155a95B064B4",
				"name": "XmasDao",
				"decimals": 9
			},
			"0x133e28326478e2e75d7aa1cb731df02221889cce": {
				"ticker": "BOOBA",
				"address": "0x133E28326478e2e75D7Aa1cB731DF02221889cCe",
				"name": "BoobaDao",
				"decimals": 9
			},
			"0x0dea74f924d4eb1c0a29c63244ab0db6356f116f": {
				"ticker": "SYM",
				"address": "0x0dea74F924d4eB1C0A29c63244AB0DB6356F116F",
				"name": "SymplicismDAO",
				"decimals": 18
			},
			"0x03ae91819b551217ee058dd38852c9af250d63bd": {
				"ticker": "AURIC",
				"address": "0x03aE91819B551217EE058Dd38852C9af250d63BD",
				"name": "Auric Nodes",
				"decimals": 18
			},
			"0xc68b26fd3076c89085fc647e7acc2a27c3d84cb8": {
				"ticker": "BSFM",
				"address": "0xc68B26Fd3076C89085FC647E7Acc2a27c3D84cb8",
				"name": "Baby Safe Moon",
				"decimals": 9
			},
			"0x96f1278e299f9f605fac499a62656a35e7495416": {
				"ticker": "ALPHA",
				"address": "0x96f1278e299F9f605FaC499a62656A35E7495416",
				"name": "Alpha DAO",
				"decimals": 18
			},
			"0x519cf9b9cc02ceb98dab327a0d9a021cad7f4273": {
				"ticker": "BURGER",
				"address": "0x519cf9B9cC02cEB98dab327A0D9A021cAd7F4273",
				"name": "Floki Burger",
				"decimals": 1
			},
			"0xeb16a19e130a07335fea7bb3c3914173760d1bbd": {
				"ticker": "PHTM",
				"address": "0xEB16A19E130A07335feA7Bb3c3914173760d1bbd",
				"name": "Phantom Money",
				"decimals": 1
			},
			"0xdf3c47fb5072a14b4bc490d7534d6f99e2b961ea": {
				"ticker": "PNGN",
				"address": "0xdF3C47fB5072A14B4Bc490d7534d6F99e2b961Ea",
				"name": "Penguin Finance",
				"decimals": 18
			},
			"0x9a5743a6c745ec4aaa071fc3aab19d1ec7d7a907": {
				"ticker": "oSNO",
				"address": "0x9a5743a6c745EC4AaA071fC3AAb19D1Ec7d7A907",
				"name": "OhSnow",
				"decimals": 18
			},
			"0x19fbfa5987efecceff41b190759a9d883a4bea21": {
				"ticker": "GNIWS",
				"address": "0x19fbfA5987EFEccEFf41b190759a9d883A4beA21",
				"name": "Gniws",
				"decimals": 9
			},
			"0x1adcb017a45efce210179d5618b66c604056af2f": {
				"ticker": "BM",
				"address": "0x1AdcB017a45Efce210179d5618b66C604056aF2f",
				"name": "BLACK MIRROR",
				"decimals": 8
			},
			"0x87ee0cd0eb32fd43abe9a6296e7ce7cf8b1fc6ad": {
				"ticker": "DUPXO",
				"address": "0x87ee0cD0EB32FD43Abe9A6296E7Ce7cf8B1fc6aD",
				"name": "DUPXO TOKEN",
				"decimals": 18
			},
			"0xd51a20a1330e843c1eb64f5288fd832df8e94299": {
				"ticker": "CBRA",
				"address": "0xD51a20A1330E843C1eb64f5288fD832df8E94299",
				"name": "Cobra",
				"decimals": 18
			},
			"0x7d4060dd95376bd119cf7010cb83555efd865084": {
				"ticker": "GladiatorDAO",
				"address": "0x7D4060Dd95376bD119Cf7010Cb83555Efd865084",
				"name": "GladiatorDAO",
				"decimals": 18
			},
			"0xb4008fca9c0234153f325bd71e203cda613ba81c": {
				"ticker": "CHRO",
				"address": "0xb4008fcA9c0234153f325BD71e203cdA613Ba81c",
				"name": "Chronicum",
				"decimals": 9
			},
			"0x9df88cee3cf52a95faafce37fe691a6a37177351": {
				"ticker": "SPELLMONEY",
				"address": "0x9dF88ceE3Cf52A95FaAfcE37FE691a6A37177351",
				"name": "SPELL MONEY PRINTER",
				"decimals": 18
			},
			"0x59bbf4f9956ab2c80ade20c822e62a319ec41d78": {
				"ticker": "NRGY",
				"address": "0x59bbf4f9956AB2C80adE20C822e62A319Ec41d78",
				"name": "Wind Finance",
				"decimals": 18
			},
			"0x6ba4f417a28b728495e4e9459a17e62292684172": {
				"ticker": "NOSHARE",
				"address": "0x6bA4f417A28b728495E4E9459a17e62292684172",
				"name": "NOSHARE",
				"decimals": 18
			},
			"0x97c9bffb088c5193222e995350da5b09ec58d607": {
				"ticker": "GuitarHero",
				"address": "0x97C9BFfB088C5193222e995350dA5B09EC58d607",
				"name": "GuitarHero",
				"decimals": 6
			},
			"0x189bb2136264b341788a56037f4aa40cfcf531f7": {
				"ticker": "ANTIMATTER",
				"address": "0x189BB2136264b341788a56037F4aa40CfcF531F7",
				"name": "ANTIMATTER",
				"decimals": 8
			},
			"0x6824c32c0a944e6fefb769ad6243445928864358": {
				"ticker": "LOKI",
				"address": "0x6824C32C0A944E6fEfB769ad6243445928864358",
				"name": "LOKI",
				"decimals": 6
			},
			"0xb355f4f4cc84a9429a59d5c2b98d77016f7ec482": {
				"ticker": "BTCBR",
				"address": "0xB355f4F4CC84a9429a59d5c2B98d77016f7EC482",
				"name": "BitcoinBR",
				"decimals": 18
			},
			"0x8d87ae897b9f436a84a4538988053eac94b6bbbd": {
				"ticker": "Gri",
				"address": "0x8D87aE897B9f436A84a4538988053eAc94B6bbBd",
				"name": "Grim printer",
				"decimals": 9
			},
			"0xf0b9a991698198cec05d629201cffd0183df9cca": {
				"ticker": "BSN",
				"address": "0xf0B9A991698198cEc05D629201cFfD0183dF9CcA",
				"name": "Boson Dao",
				"decimals": 1
			},
			"0xa3a5d5f06f53095032ba1a618af334e99d6d532d": {
				"ticker": "BANANA",
				"address": "0xA3A5d5f06F53095032Ba1A618AF334e99d6D532D",
				"name": "Retro Monkey Land",
				"decimals": 1
			},
			"0xb83428be38733e56b1c0abc1f1070b8f5a63beb4": {
				"ticker": "HMRT",
				"address": "0xb83428BE38733E56b1C0ABC1f1070B8F5a63BeB4",
				"name": "HMRT Exchange Token",
				"decimals": 18
			},
			"0x81b6858e28834aa299b75540be60e681cbb7de80": {
				"ticker": "BTV",
				"address": "0x81b6858e28834Aa299B75540bE60e681CbB7DE80",
				"name": "BuffedTigerVax",
				"decimals": 18
			},
			"0x07f8b52fdc5c5d8bccdb0a30d45c75e04684b8c1": {
				"ticker": "LAND",
				"address": "0x07F8B52fDc5C5d8bccdb0a30d45c75E04684b8C1",
				"name": "Land Verse",
				"decimals": 1
			},
			"0x5cf82673d148aa5895d35399375710677d3c571a": {
				"ticker": "FlokiKingAvax🤴🐶🔺",
				"address": "0x5Cf82673D148Aa5895d35399375710677D3c571a",
				"name": "FlokiKingAvax🤴🐶🔺",
				"decimals": 9
			},
			"0x3416c027238a8f8d5cd19a015b353c2f2f0f4c8d": {
				"ticker": "Mixe",
				"address": "0x3416C027238A8f8d5CD19a015B353c2F2F0f4c8d",
				"name": "Mixer Protocol",
				"decimals": 8
			},
			"0x5da71ea0cd0a2af112a43b724fc0bd382edd7c96": {
				"ticker": "ARCPAY",
				"address": "0x5dA71ea0Cd0a2af112a43b724FC0bd382Edd7c96",
				"name": "Arcade Pay",
				"decimals": 9
			},
			"0x600dbaf8eea6b83418189f415c9409056c1e28b3": {
				"ticker": "LFLOKI",
				"address": "0x600dBAf8EeA6B83418189F415C9409056C1e28B3",
				"name": "LUCKY FLOKI",
				"decimals": 18
			},
			"0xd8d4c3e232458a349e32716a278db9a62ef30e80": {
				"ticker": "Golde",
				"address": "0xd8d4C3e232458a349E32716A278Db9A62Ef30E80",
				"name": "Golden Eye",
				"decimals": 8
			},
			"0x72f798c7ddf63e6bf93afe858649a49b0b9e8970": {
				"ticker": "SuperNova",
				"address": "0x72F798C7DDF63e6bf93afE858649a49B0b9e8970",
				"name": "SuperNova",
				"decimals": 8
			},
			"0xa877b9d357d6461eeb390df96f1968ed0b4a2389": {
				"ticker": "LIGHT",
				"address": "0xA877b9d357D6461eeb390df96F1968ed0b4a2389",
				"name": "LightNodes",
				"decimals": 18
			},
			"0xffe04bf98c7111360bf7a6c56b343915543cd941": {
				"ticker": "GSHARE",
				"address": "0xFfE04Bf98C7111360Bf7A6c56b343915543cD941",
				"name": "GSHARE",
				"decimals": 18
			},
			"0x29947104b86ad70d32d0d453d8d176c0c3754029": {
				"ticker": "WSHARE",
				"address": "0x29947104B86AD70d32d0d453D8d176C0C3754029",
				"name": "WSHARE",
				"decimals": 18
			},
			"0xa26b2b6a9e9bd2e637edcd6d1c9b93479dbaae45": {
				"ticker": "Shinja",
				"address": "0xA26B2B6a9e9BD2E637eDCd6D1C9b93479dBaae45",
				"name": "SHIBNOBI",
				"decimals": 8
			},
			"0x810bfb9cc38155df69b9d37cad298c0328d4e310": {
				"ticker": "FNODE",
				"address": "0x810bFb9cc38155DF69B9D37CAD298C0328d4e310",
				"name": "FrontNode",
				"decimals": 9
			},
			"0xc49529f094054814d1e808d61e458ccf705d6890": {
				"ticker": "CANDY",
				"address": "0xc49529f094054814D1e808D61e458cCf705D6890",
				"name": "Candy CANE FINANCE",
				"decimals": 18
			},
			"0xf639757040294e3229d5367dfc64f7c5916ef753": {
				"ticker": "SN",
				"address": "0xF639757040294e3229d5367dfC64F7C5916Ef753",
				"name": "Snow Nodes",
				"decimals": 18
			},
			"0x93bb6d5b2819b42d44489673242740db9db247d2": {
				"ticker": "SSHIB",
				"address": "0x93BB6D5B2819B42D44489673242740db9DB247D2",
				"name": "SnowShib",
				"decimals": 18
			},
			"0x536a88870436c5b7b6ff6c4291b76c2028140d5f": {
				"ticker": "WAVE",
				"address": "0x536A88870436c5b7b6fF6c4291b76c2028140d5f",
				"name": "Waves Money",
				"decimals": 1
			},
			"0xfe87dd247b6d3139a1be93d72c24953b36cc3d82": {
				"ticker": "ShiftGame",
				"address": "0xFe87DD247B6d3139a1bE93d72c24953b36Cc3d82",
				"name": "Shift",
				"decimals": 8
			},
			"0x3075fcb2671d9ae4dd376bc4a2d2b309c08686a6": {
				"ticker": "Alchemist",
				"address": "0x3075fCb2671D9Ae4dD376BC4A2D2B309c08686A6",
				"name": "Alchemist",
				"decimals": 18
			},
			"0xdfc6f59fe6c35b3134f97d502c0e5bfa32653f23": {
				"ticker": "ApeU",
				"address": "0xDFC6f59fe6C35b3134f97d502C0E5bfa32653f23",
				"name": "Ape Universe",
				"decimals": 18
			},
			"0x5bc623dbaaa7f230b4d110f7524667f6bc6674eb": {
				"ticker": "PORK",
				"address": "0x5Bc623DBaAa7f230B4d110f7524667F6bC6674eb",
				"name": "Porker",
				"decimals": 18
			},
			"0x84de6f76ba88a1bdfe57bbed51f9b0270ccdfcc5": {
				"ticker": "CLONEX",
				"address": "0x84De6F76ba88A1BDfE57BBEd51f9b0270CCdFCc5",
				"name": "ClonexNode",
				"decimals": 18
			},
			"0x9168a01d728c22a0037a18713826da8d9085049c": {
				"ticker": "PYD",
				"address": "0x9168A01d728c22A0037a18713826DA8D9085049c",
				"name": "Pyramidum Verse",
				"decimals": 8
			},
			"0x664a6e09dab935b65d45ce901381c5102a71828f": {
				"ticker": "Volta",
				"address": "0x664a6e09DAB935b65D45Ce901381c5102A71828f",
				"name": "Volta",
				"decimals": 9
			},
			"0xe6ce9c59cbd9ef7b40227fbf47c180ad9124d604": {
				"ticker": "GLOBAL",
				"address": "0xe6ce9c59CBD9Ef7b40227fBf47C180aD9124d604",
				"name": "Global Money",
				"decimals": 1
			},
			"0x373b614fb6c30d3a01d67c4d8c3430bfaec12f9c": {
				"ticker": "COCONUT",
				"address": "0x373b614FB6c30D3a01d67C4D8c3430bFaEC12f9c",
				"name": "COCONUT",
				"decimals": 6
			},
			"0xe0e37da9a0d3be9e2311b413ff8ab6c2d8bf61f1": {
				"ticker": "ARTHURINU",
				"address": "0xe0e37da9a0d3Be9E2311b413ff8aB6c2D8BF61f1",
				"name": "ARTHURINU",
				"decimals": 9
			},
			"0x0f560bef13ed9c04b045b91e921f73070ba33858": {
				"ticker": "$MULTI",
				"address": "0x0F560BEF13Ed9C04B045B91e921F73070bA33858",
				"name": "Multiverse",
				"decimals": 18
			},
			"0x1cd2528522a17b6be63012fb63ae81f3e3e29d97": {
				"ticker": "MND",
				"address": "0x1cd2528522A17B6Be63012fB63AE81f3e3e29D97",
				"name": "Mind Music",
				"decimals": 9
			},
			"0x71bc385a0b7e0afbe861a993b0d0844a5cc1f413": {
				"ticker": "WAR",
				"address": "0x71bc385A0b7e0afbe861a993b0D0844A5CC1f413",
				"name": "WAR NODES",
				"decimals": 9
			},
			"0x696b89b58649894ff95657d780995ac5ee991aa6": {
				"ticker": "HORIZON",
				"address": "0x696b89B58649894ff95657d780995aC5eE991aa6",
				"name": "Horizon Finance",
				"decimals": 16
			},
			"0x128cff469c1a3a9734b3ffa17fe07c597e29c7b0": {
				"ticker": "OXY",
				"address": "0x128Cff469C1A3a9734b3fFA17fe07c597E29C7B0",
				"name": "Oxy-Fi",
				"decimals": 9
			},
			"0xf97f86d3d89c7858608a0e0e8c8475189aeb1427": {
				"ticker": "Magm",
				"address": "0xf97F86D3D89c7858608a0E0E8c8475189AEb1427",
				"name": "Magma Printer",
				"decimals": 9
			},
			"0xcdee8bb6ed9927f96fd954db2b8035be6283fd87": {
				"ticker": "KEVIN",
				"address": "0xcDeE8BB6eD9927f96Fd954db2B8035bE6283fD87",
				"name": "Kevin",
				"decimals": 9
			},
			"0x7f82197845dd9afdf03adf129d721086f9c50969": {
				"ticker": "EarlyDuck",
				"address": "0x7F82197845dD9aFdF03ADf129d721086F9c50969",
				"name": "Early Duck",
				"decimals": 18
			},
			"0x7abf90d1e83f6f210cc07cc85b6191e806963f95": {
				"ticker": "XSHARES",
				"address": "0x7ABF90d1E83f6f210cc07cc85b6191E806963f95",
				"name": "XSHARE Token",
				"decimals": 18
			},
			"0x705b7221ccca38df865831f8995226d9537cdef9": {
				"ticker": "JEWELP",
				"address": "0x705b7221Ccca38df865831F8995226d9537cdEf9",
				"name": "JEWELPRINT",
				"decimals": 9
			},
			"0x999ed93893ee287c33ccfe926312459e379f2897": {
				"ticker": "YOUTH",
				"address": "0x999ED93893ee287C33CcFe926312459e379f2897",
				"name": "YOUTH",
				"decimals": 18
			},
			"0x1c4de69e55654b90e7a20bc27ed11d6a8b8aac14": {
				"ticker": "SUPER",
				"address": "0x1C4DE69e55654B90E7a20BC27eD11D6A8B8AaC14",
				"name": "Super Nodes",
				"decimals": 18
			},
			"0x03980efa9b0c3735baa70c2ec0b139ea51153008": {
				"ticker": "CHESHIRE",
				"address": "0x03980EFa9b0C3735bAA70c2Ec0B139eA51153008",
				"name": "Cheshire AVAX - cheshireavax.com",
				"decimals": 1
			},
			"0xc98bd749eabf23ceb3bb61c722b3a92de34bd53b": {
				"ticker": "$SIA",
				"address": "0xc98bD749eaBF23CEB3BB61c722b3A92de34bD53B",
				"name": "Shiba Inu Avax",
				"decimals": 9
			},
			"0x265960cf53a1b3ac9641e3372ec2f6a196498601": {
				"ticker": "TXJ",
				"address": "0x265960cf53A1b3aC9641E3372eC2f6a196498601",
				"name": "ToxicJoe",
				"decimals": 9
			},
			"0xc163dd560598893226d5914590123155b6b44b9e": {
				"ticker": "Ope",
				"address": "0xC163Dd560598893226d5914590123155B6B44b9E",
				"name": "Open Mind",
				"decimals": 6
			},
			"0x50638ad7ca0bf62e0f84a4efd2e138c2ddf7483c": {
				"ticker": "bJOE",
				"address": "0x50638aD7Ca0Bf62E0f84a4EFd2e138c2dDf7483C",
				"name": "bTraderJoe",
				"decimals": 18
			},
			"0x670f68798d54fd52a9135be2ea5ea79e8627983f": {
				"ticker": "GLA",
				"address": "0x670F68798D54FD52a9135BE2EA5EA79E8627983f",
				"name": "Glacier Nodes",
				"decimals": 1
			},
			"0x77af9a742e7f60df0d36ddf987e0e816810602db": {
				"ticker": "THOR",
				"address": "0x77Af9A742E7f60df0d36ddF987E0e816810602Db",
				"name": "Thors Hammer Protocol",
				"decimals": 1
			},
			"0xaee6cbba9d289cdad42faad18b2641ddfb833506": {
				"ticker": "AP",
				"address": "0xAEE6cBbA9D289CdAD42FaaD18b2641DdFb833506",
				"name": "AvaxPad",
				"decimals": 9
			},
			"0xa80758d4d830ad3a421095248e45d4ab73f52853": {
				"ticker": "BadBoys",
				"address": "0xA80758d4d830aD3A421095248e45D4aB73F52853",
				"name": "BadBoys",
				"decimals": 8
			},
			"0xc5087c6ae890454e548ac3ea4b19dd6bd34afdd2": {
				"ticker": "ARCJEWEL",
				"address": "0xc5087c6aE890454E548ac3ea4B19DD6BD34AFDD2",
				"name": "ARCJEWEL",
				"decimals": 9
			},
			"0x9ab07d79e2ab8c62338b3fa8c532753885ee920f": {
				"ticker": "BAGOJEWEL",
				"address": "0x9AB07D79E2AB8C62338b3fA8c532753885Ee920F",
				"name": "Bag O' Jewels",
				"decimals": 9
			},
			"0xdd75bc10fda7aa5d01f7d3d042f44f7db94dbbb9": {
				"ticker": "RBX",
				"address": "0xDd75BC10FDA7aA5d01f7d3D042F44F7Db94Dbbb9",
				"name": "RBX Token",
				"decimals": 5
			},
			"0x2053bf077b92d54b638b212374eb126a6572d2ed": {
				"ticker": "FRONTO",
				"address": "0x2053bf077B92d54b638b212374Eb126a6572d2ed",
				"name": "FrontoDAO",
				"decimals": 18
			},
			"0x367f454c142f7aabb5ba4415840f8d4f468eabd9": {
				"ticker": "SNode",
				"address": "0x367f454c142F7aabB5ba4415840F8d4f468eaBd9",
				"name": "SuperNode",
				"decimals": 18
			},
			"0x8c5a0457e407acd036fc362fd97d2dec0c912338": {
				"ticker": "SHIBAJORDAN",
				"address": "0x8C5A0457E407acD036Fc362fd97d2DEc0C912338",
				"name": "SHIBAJORDAN",
				"decimals": 9
			},
			"0x44a63056e1678fd54c25ac5c37b8cbebe008404a": {
				"ticker": "AP",
				"address": "0x44a63056e1678fd54c25AC5C37b8Cbebe008404A",
				"name": "AvaxPad",
				"decimals": 9
			},
			"0xb8edc9145e21a7c3345b848ca73300fa35150b0f": {
				"ticker": "AUA",
				"address": "0xB8eDc9145E21A7C3345b848cA73300fA35150B0F",
				"name": " Atlantis Universe",
				"decimals": 9
			},
			"0x334dcd4067d8b5ff42ba03134c368394a0244d9a": {
				"ticker": "MAGMA",
				"address": "0x334DcD4067D8b5FF42Ba03134c368394A0244d9a",
				"name": "MAGMA",
				"decimals": 9
			},
			"0x21c5402c3b7d40c89cc472c9df5dd7e51bbab1b1": {
				"ticker": "TUNDRA",
				"address": "0x21c5402C3B7d40C89Cc472C9dF5dD7E51BbAb1b1",
				"name": "TUNDRAToken",
				"decimals": 18
			},
			"0x87afcc933967573bc7a36c0df67dd909b032d97c": {
				"ticker": "MOON",
				"address": "0x87AFcC933967573bc7A36C0df67dD909b032d97c",
				"name": "Moon Money",
				"decimals": 1
			},
			"0xc77f84d342e63ee8dff5e441d190b5a90b728b98": {
				"ticker": "AVORGI",
				"address": "0xC77F84D342e63eE8dfF5E441d190b5a90B728B98",
				"name": "AVORGI",
				"decimals": 9
			},
			"0x89176d15a1022431ae856186d77a975d152adfaf": {
				"ticker": "XED",
				"address": "0x89176D15a1022431AE856186d77A975D152aDfaf",
				"name": "AVAXED",
				"decimals": 18
			},
			"0x9c6b4f4335059f9b7c6b97136d2e89994c34bda6": {
				"ticker": "Bul",
				"address": "0x9C6B4F4335059f9B7c6b97136d2E89994C34Bda6",
				"name": "Bull Nodes",
				"decimals": 18
			},
			"0xd6dd533b562dacba0fd67db9967a6c0d6ae406bf": {
				"ticker": "BOOBA",
				"address": "0xD6DD533B562DaCba0Fd67Db9967a6c0D6AE406bF",
				"name": "BoobaDao",
				"decimals": 9
			},
			"0xad7476c49d3f82a144f4836aacb9b069c188b759": {
				"ticker": "SLD",
				"address": "0xad7476c49D3f82a144f4836aACb9b069c188b759",
				"name": "Soldier",
				"decimals": 18
			},
			"0x852db00abc4dc2657f67c464b4cf1a537aae40b0": {
				"ticker": "LEAF",
				"address": "0x852db00AbC4dC2657F67C464B4cf1A537aae40b0",
				"name": "TreeHouse Finance",
				"decimals": 9
			},
			"0xabece15190d7b778802997abb2af1409bbf12cd6": {
				"ticker": "APC",
				"address": "0xABecE15190D7B778802997aBb2Af1409bbF12Cd6",
				"name": "Alpha Centauri",
				"decimals": 9
			},
			"0x3c9df080de0f30fa8049414e5426e7b4f4b25858": {
				"ticker": "SOLG",
				"address": "0x3c9Df080dE0F30Fa8049414E5426E7B4f4b25858",
				"name": "Soldier Games",
				"decimals": 9
			},
			"0x3adfda83fb8baa8d9ceba70389367af7d9e98fc2": {
				"ticker": "Pea",
				"address": "0x3aDFdA83Fb8baa8D9ceBa70389367af7d9e98fc2",
				"name": "Peak HODL",
				"decimals": 8
			},
			"0xfb7498d4c98ca8ccc2b9871d1576c560966c95d6": {
				"ticker": "A4FI",
				"address": "0xfB7498d4c98CA8cCc2B9871D1576c560966C95d6",
				"name": "A4.Finanace",
				"decimals": 9
			},
			"0xa814a0313a9eb4dfc39ba63a17ef69d65cff4d2b": {
				"ticker": "IMPERIUM",
				"address": "0xa814a0313a9EB4DFc39Ba63a17eF69d65Cff4D2b",
				"name": "IMPERIUM",
				"decimals": 8
			},
			"0x83cc77a8accd38bc8dd547bcd365b1225d2f6118": {
				"ticker": "DR.REPOS",
				"address": "0x83CC77a8acCd38bc8dD547bcd365b1225D2f6118",
				"name": "DR.REPOSER",
				"decimals": 18
			},
			"0xf14be478a18d8e9fa4d688fc707e24c35fb1144e": {
				"ticker": "GINU",
				"address": "0xF14be478a18d8E9fA4D688FC707e24C35Fb1144E",
				"name": "GhozaliInu",
				"decimals": 9
			},
			"0xf5e22b4447e63e31032bea6174c68cb6aa70bdd3": {
				"ticker": "ICE",
				"address": "0xF5e22b4447E63e31032Bea6174C68CB6aA70Bdd3",
				"name": "Ice Money",
				"decimals": 1
			},
			"0x69fa6bbe659b7bc60b4d1d82460af8cd4eda6a43": {
				"ticker": "GHS",
				"address": "0x69FA6bbE659b7Bc60b4d1D82460af8CD4eda6a43",
				"name": "Sushi Ghost",
				"decimals": 9
			},
			"0x8fd7dcd872224b34b4ac53d49c78b73d9e2d1025": {
				"ticker": "PS",
				"address": "0x8fd7dcd872224B34b4aC53d49c78B73D9E2d1025",
				"name": "ProbablySomething",
				"decimals": 9
			},
			"0x9c7127310adb584afde49b005e7295823edb2467": {
				"ticker": "Frog",
				"address": "0x9C7127310ADB584AfDe49b005e7295823EdB2467",
				"name": "FrogDao",
				"decimals": 9
			},
			"0x8a41abc167842416b8fc3adfa4e8c9aff979bd5b": {
				"ticker": "XLD",
				"address": "0x8a41ABC167842416B8fc3Adfa4E8c9AFf979BD5b",
				"name": "XcelDefi",
				"decimals": 9
			},
			"0xce7007fa3db1de4fb73f3b6abdcba0b35c382b50": {
				"ticker": "RBX",
				"address": "0xCe7007fA3dB1dE4Fb73F3b6ABDCbA0B35C382B50",
				"name": "Rbx Token",
				"decimals": 9
			},
			"0xdbd12aa929877a1d25db3d37af8fc9d0ec8a5180": {
				"ticker": "ASTRO",
				"address": "0xDbD12aa929877A1d25db3D37aF8Fc9d0ec8A5180",
				"name": "ASTRO Money",
				"decimals": 18
			},
			"0xe9f03a2f35ecdc08bde2c9527ddc17f78e2dfcaf": {
				"ticker": "ELCTRN",
				"address": "0xE9f03a2f35eCDc08BDe2C9527ddc17F78E2DfcAF",
				"name": "Electro Nodes",
				"decimals": 1
			},
			"0x372b73b2225f4f422f58a772d2dcaa9048904c1d": {
				"ticker": "HONEY",
				"address": "0x372B73B2225f4f422F58A772d2DCaa9048904C1d",
				"name": "Honey",
				"decimals": 9
			},
			"0x151140d641eac5afd4fca9f617960319c3e85b37": {
				"ticker": "Frozen",
				"address": "0x151140D641EAC5afD4FcA9F617960319c3E85b37",
				"name": "Frozen",
				"decimals": 9
			},
			"0x51f9b72fbd080350af697ce86f1e67494440f276": {
				"ticker": "Choc",
				"address": "0x51f9B72Fbd080350af697ce86F1E67494440f276",
				"name": "Chocolate Nodes",
				"decimals": 18
			},
			"0xfa5ef344c6e2955b0a4e1e41e2eacc80efabda98": {
				"ticker": "ZEPPELIN",
				"address": "0xfA5eF344C6e2955B0a4E1e41E2EACC80eFaBDA98",
				"name": "ZEPPELIN",
				"decimals": 8
			},
			"0xd796ff2bf062b3be96198039b275e1d931fc1bdb": {
				"ticker": "RED",
				"address": "0xd796FF2bF062B3bE96198039b275e1D931FC1BDb",
				"name": "RED DOGE",
				"decimals": 18
			},
			"0x3a0cd43f2d46ce2ba55531a6ffcfba5daa808a9f": {
				"ticker": "BALAS",
				"address": "0x3A0CD43f2D46cE2Ba55531a6FFCfBA5DaA808a9F",
				"name": "Balas Printer",
				"decimals": 2
			},
			"0x1c27af23a2e1d7cc1e092980ec0de47ec1db2265": {
				"ticker": "ARMY",
				"address": "0x1C27AF23a2e1D7cc1e092980eC0De47ec1dB2265",
				"name": "Army Token",
				"decimals": 18
			},
			"0x0a40bad2e68b49e009d624b7d5e9cb488954c316": {
				"ticker": "MP",
				"address": "0x0A40bad2E68b49E009D624B7D5E9cb488954c316",
				"name": "Medic Printer",
				"decimals": 18
			},
			"0x86f4e1f8baddfab4f2a7d9284bfa40583172bdd1": {
				"ticker": "WIND",
				"address": "0x86F4e1F8baddFAb4F2A7D9284BFa40583172BDd1",
				"name": "WINDVERSE",
				"decimals": 18
			},
			"0x79b912b43d6bebc27ce73a94790193425c572a3c": {
				"ticker": "PXT",
				"address": "0x79b912b43D6bEBc27cE73A94790193425C572a3c",
				"name": "Project X Nodes",
				"decimals": 1
			},
			"0xc7e2792e1896039b409e944735cb48c66a685feb": {
				"ticker": "PONZI",
				"address": "0xc7E2792E1896039B409e944735CB48c66A685feb",
				"name": "Ponzi Compiler",
				"decimals": 18
			},
			"0x5326f272a1d89d9b325c64e1a22f7e69a8206cc5": {
				"ticker": "CRAFT",
				"address": "0x5326f272a1d89D9B325c64e1A22F7e69a8206cc5",
				"name": "TaleCraft.io",
				"decimals": 18
			},
			"0x8feb879b3635ce5f905fa455032f690fda8805dd": {
				"ticker": "PUSSY",
				"address": "0x8fEB879B3635Ce5F905fa455032f690FDA8805dD",
				"name": "PussyDao",
				"decimals": 18
			},
			"0x4ea87c567bf5644e97af7c8e1cb3742c99108177": {
				"ticker": "NPTN",
				"address": "0x4EA87C567bF5644E97Af7c8E1cb3742C99108177",
				"name": "Neptune Protocol",
				"decimals": 1
			},
			"0x9b6c9bf18f8168af0f894cce068f3cc9bf7bbd35": {
				"ticker": "Crypt",
				"address": "0x9b6c9bf18f8168af0F894cce068F3CC9BF7bBd35",
				"name": "Crypto Space",
				"decimals": 8
			},
			"0xe5d043314f999c91cf82dc8796c0c041f000fe4a": {
				"ticker": "SHIBA",
				"address": "0xe5d043314f999C91cF82DC8796C0c041F000fE4a",
				"name": "Baby Shiba",
				"decimals": 18
			},
			"0x80d18b1c9ab0c9b5d6a6d5173575417457d00a12": {
				"ticker": "axlATOM",
				"address": "0x80D18b1c9Ab0c9B5D6A6d5173575417457d00a12",
				"name": "Axelar Wrapped ATOM",
				"decimals": 6
			},
			"0x3700a92dd231f0cac37d31dbcf4c0f5ccb1db6ca": {
				"ticker": "GRAVE",
				"address": "0x3700a92dd231F0CaC37D31dBcF4c0f5cCb1db6Ca",
				"name": "grave.finance",
				"decimals": 18
			},
			"0x79d9a03747e0654ae595ac619ea446e9a7bf9d76": {
				"ticker": "VAXBOOST",
				"address": "0x79D9A03747e0654aE595AC619Ea446e9a7bF9d76",
				"name": "Avaxine Booster",
				"decimals": 18
			},
			"0xfc13886874efb4078a7773735714147019bb010c": {
				"ticker": "BTP",
				"address": "0xfC13886874Efb4078a7773735714147019bb010C",
				"name": "BTCPAY",
				"decimals": 9
			},
			"0x713b76b9ca5527839bc16848530a665137917c52": {
				"ticker": "PII",
				"address": "0x713b76b9CA5527839BC16848530A665137917C52",
				"name": "Panda Inu Infection",
				"decimals": 18
			},
			"0x1911f5652beb676724b4245902af1d246717699d": {
				"ticker": "THORN",
				"address": "0x1911F5652bEB676724b4245902Af1D246717699D",
				"name": "Thornail Protocol",
				"decimals": 1
			},
			"0x8f49036e337af6f3bf9d7600c0559c43706b01a3": {
				"ticker": "FLUID",
				"address": "0x8F49036e337AF6f3bf9d7600c0559C43706B01a3",
				"name": "Fluidly Exchange",
				"decimals": 1
			},
			"0x8b1d98a91f853218ddbb066f20b8c63e782e2430": {
				"ticker": "ORCA",
				"address": "0x8B1d98A91F853218ddbb066F20b8c63E782e2430",
				"name": "OrcaDAO",
				"decimals": 18
			},
			"0x5cf3bc8f0ce2fa1174e2fb805bf2f29081af37dc": {
				"ticker": "APE",
				"address": "0x5CF3bc8F0cE2fa1174E2fB805bf2F29081AF37DC",
				"name": "ApeMoon",
				"decimals": 9
			},
			"0x12291d4fff3a4d74eccf11496344f6665b5db394": {
				"ticker": "LACTOSE",
				"address": "0x12291D4fFF3a4D74ECcf11496344f6665b5db394",
				"name": "DAIryMachine",
				"decimals": 9
			},
			"0xcc01c5aab9a4a4c6da78e2c4132a48b14a0e34ff": {
				"ticker": "NIT",
				"address": "0xCc01C5AAb9a4A4C6Da78e2c4132A48B14a0e34Ff",
				"name": "Node Investment Trust",
				"decimals": 9
			},
			"0x8f81196e3e6ff0d7bb5cd14aa534d4189faae7a2": {
				"ticker": "HashMask",
				"address": "0x8F81196e3E6FF0d7bb5Cd14aA534d4189FaaE7A2",
				"name": "HashMask",
				"decimals": 8
			},
			"0xfc904876fa8bd0ee4e80ad1aa2ff380fee3013a5": {
				"ticker": "HONEY",
				"address": "0xfC904876Fa8bd0EE4e80AD1AA2ff380FEe3013A5",
				"name": "WASPvsBEE",
				"decimals": 9
			},
			"0x2dfd66c45d80ff6c61dff9f65daff14cd94f1d58": {
				"ticker": "ETEHEN",
				"address": "0x2dfD66c45d80Ff6C61dff9f65daFf14cd94f1d58",
				"name": "ETEHEN",
				"decimals": 18
			},
			"0xdce186a08193f3e97a75e1a21dc5adfb2b1378ac": {
				"ticker": "Paradigm",
				"address": "0xDCe186A08193f3e97A75e1A21DC5adFb2B1378aC",
				"name": "Paradigm",
				"decimals": 9
			},
			"0x48941d5a3b7b97f4fea21a7609b9af414e2550d1": {
				"ticker": "SOLAR",
				"address": "0x48941d5a3B7B97F4FEa21A7609B9AF414e2550d1",
				"name": "Solar",
				"decimals": 18
			},
			"0x00313b64e4c58ab56ebc4a7afc9b73b5146aab43": {
				"ticker": "Gambi",
				"address": "0x00313B64E4c58AB56Ebc4a7afC9B73B5146aaB43",
				"name": "Gambit DAO",
				"decimals": 8
			},
			"0x6760f9fb100369810a1e12f59b7164c1ded7f271": {
				"ticker": "GRIMACE",
				"address": "0x6760f9fb100369810A1e12f59B7164C1deD7f271",
				"name": "GRIMACE UNIV",
				"decimals": 6
			},
			"0x954a6a4def64fc55446292048e5b0fa4a8aed1d9": {
				"ticker": "SKOL",
				"address": "0x954a6a4DeF64fC55446292048e5B0fa4a8AeD1d9",
				"name": "Axe Nodes",
				"decimals": 1
			},
			"0xc267de311610ad9bf8f3ce03f855551c95d780cb": {
				"ticker": "AVATAR",
				"address": "0xc267De311610aD9Bf8f3Ce03F855551C95d780Cb",
				"name": "AVATAR",
				"decimals": 8
			},
			"0xfa301b1433f7d8bc47a54c2c1b877bddba8bb32c": {
				"ticker": "FARM",
				"address": "0xfA301b1433F7d8BC47A54C2c1B877bDDBA8bB32c",
				"name": "FARM",
				"decimals": 18
			},
			"0xf97d917a0ef1bb6919c36489eb0fd00db41ffba8": {
				"ticker": "AVAMIM",
				"address": "0xf97D917a0ef1Bb6919C36489Eb0Fd00dB41fFBa8",
				"name": "AVA MIM",
				"decimals": 6
			},
			"0x071be81655846ff6137baf4691665e05f78b5fbb": {
				"ticker": "xRing",
				"address": "0x071be81655846FF6137BAf4691665E05f78b5fBB",
				"name": "Ring Financial",
				"decimals": 1
			},
			"0x475adff2193fdfc4c67cac82dd29363712886e4f": {
				"ticker": "ONYX",
				"address": "0x475AdfF2193Fdfc4c67cac82dD29363712886e4F",
				"name": "Onyx Nodes",
				"decimals": 18
			},
			"0x897bbf5960de4ff99460c823f2c11e36f6ae9eee": {
				"ticker": "EthaVerseDAO",
				"address": "0x897bbF5960De4Ff99460c823f2c11e36F6Ae9EeE",
				"name": "EthaVerseDAO",
				"decimals": 18
			},
			"0xbe74a0539d266aece4805f68edf58f3725828823": {
				"ticker": "PONZI",
				"address": "0xbe74A0539d266Aece4805f68edF58f3725828823",
				"name": "Ponzi Compiler",
				"decimals": 18
			},
			"0x7b6932dafdaff7fd7535c4ccdfc01bab444d0976": {
				"ticker": "ElectricNode",
				"address": "0x7b6932daFdafF7fd7535c4CCDFC01BaB444D0976",
				"name": "Electric Node",
				"decimals": 18
			},
			"0x004afc5cb978c0da7160ca5d7dddc179f9a007cc": {
				"ticker": "STRONGp",
				"address": "0x004Afc5cb978c0Da7160ca5D7dDDc179f9a007cC",
				"name": "STRONG Nodes Printer",
				"decimals": 1
			},
			"0xabbf3d0e50ef60c74e544eafb4d42834f5c14875": {
				"ticker": "SnowDrop",
				"address": "0xABbf3d0E50EF60c74E544eAfb4d42834F5C14875",
				"name": "SnowDrop",
				"decimals": 8
			},
			"0x74ac93a43a562b9d4c6abc83dc24b1197033f174": {
				"ticker": "JEWELP",
				"address": "0x74ac93A43a562b9D4C6abC83dC24b1197033F174",
				"name": "Jewel Print",
				"decimals": 18
			},
			"0xbf9429ac21b44b1982c7d74859eb27bb8827ef6b": {
				"ticker": "APEU",
				"address": "0xBf9429ac21b44B1982c7D74859EB27BB8827Ef6b",
				"name": "Ape Universe",
				"decimals": 18
			},
			"0x4346949294222726e385ad5f0f631fe89f4a9ca6": {
				"ticker": "Ho",
				"address": "0x4346949294222726E385AD5f0F631Fe89f4a9ca6",
				"name": "How are you?",
				"decimals": 18
			},
			"0x360465d774a6514800c713f7263af73e8974ab5f": {
				"ticker": "SOLAR",
				"address": "0x360465d774a6514800c713F7263Af73E8974aB5F",
				"name": "Solar Finance",
				"decimals": 1
			},
			"0x4a11e78771d101e8c298939ffcd6350d986ce317": {
				"ticker": "ALIGN",
				"address": "0x4A11e78771D101E8C298939fFCD6350d986Ce317",
				"name": "Aligned Capital",
				"decimals": 18
			},
			"0xba3fc8790c19ab9a5649a7c5d76706b65f0075a5": {
				"ticker": "PTDAO",
				"address": "0xBa3fc8790c19aB9A5649a7C5D76706B65F0075A5",
				"name": "PRINTDAO",
				"decimals": 9
			},
			"0x9b64ac77fb518d0e4bdd16367cc08a32bd996e1c": {
				"ticker": "FLAMI",
				"address": "0x9b64Ac77fb518d0E4bdD16367cc08a32BD996e1c",
				"name": "KING FLAMINGO",
				"decimals": 18
			},
			"0xc7bbbe598d6ecda439fb692bcfebc89248f116f5": {
				"ticker": "NA",
				"address": "0xC7BbbE598d6eCda439fb692bcfebC89248f116f5",
				"name": "Nodes Airdrop",
				"decimals": 18
			},
			"0x4db46704e9a24eae42e5a9d3ae9a49f3eb0b9a5a": {
				"ticker": "ETHPPV2",
				"address": "0x4Db46704e9A24eAe42E5A9D3Ae9A49F3eB0b9a5a",
				"name": "ETHPPV2",
				"decimals": 6
			},
			"0x59dc6a952297cc7fa21648f536185f2c9552e813": {
				"ticker": "BUBBLE",
				"address": "0x59dc6a952297Cc7fA21648f536185f2c9552E813",
				"name": "Bubble Finance",
				"decimals": 18
			},
			"0x12ed2771c849c29ff3413099ddef6dc987624144": {
				"ticker": "MCF",
				"address": "0x12ED2771c849C29ff3413099DDEf6dC987624144",
				"name": "Microchain Finance",
				"decimals": 9
			},
			"0xeec5f70877f8205459c9a50bde8b81696fa63335": {
				"ticker": "WHL",
				"address": "0xEec5f70877F8205459C9A50Bde8B81696fA63335",
				"name": "Whale Nodes",
				"decimals": 1
			},
			"0x614fd9a6f48e3350e4815864e0ed2b55b2e9dae6": {
				"ticker": "PPC",
				"address": "0x614FD9A6F48E3350E4815864E0Ed2B55b2E9daE6",
				"name": "Phoenix Printer Capital",
				"decimals": 6
			},
			"0xe878329175816e0305f096be58190460256c06e7": {
				"ticker": "WND",
				"address": "0xe878329175816e0305f096be58190460256C06e7",
				"name": "WindDAO",
				"decimals": 9
			},
			"0xf72021db8123524f778dc317c75065e163155779": {
				"ticker": "PENTA",
				"address": "0xf72021db8123524f778DC317c75065e163155779",
				"name": "Pentagon Finance",
				"decimals": 18
			},
			"0x2b16bc9a2576b3d236ef679fa4850a59778c0c62": {
				"ticker": "Scream",
				"address": "0x2B16bC9a2576b3d236EF679FA4850A59778C0C62",
				"name": "Scream",
				"decimals": 9
			},
			"0x30af8ff8a452223c3646d9556ec16e817cff89a0": {
				"ticker": "LightNodes",
				"address": "0x30AF8Ff8A452223C3646d9556eC16E817CFf89a0",
				"name": "LightNodes",
				"decimals": 18
			},
			"0x3bf1e3bd17b15fa38c17ab8d6aa78804f7b53ee8": {
				"ticker": "QN",
				"address": "0x3bf1E3BD17b15fa38c17Ab8D6aA78804F7b53ee8",
				"name": "Quick Node",
				"decimals": 8
			},
			"0x1c7b6bae72e65dba3e2ac6a4129a67372a8f49a9": {
				"ticker": "GALAXYR",
				"address": "0x1C7B6BaE72e65dBa3e2Ac6A4129A67372a8f49A9",
				"name": "Galaxy Printers",
				"decimals": 6
			},
			"0xebaa5d4e6314ba2b757d55302014c67b6413beee": {
				"ticker": "CHILL",
				"address": "0xEBaa5d4e6314ba2b757d55302014C67B6413BeEE",
				"name": "The Island",
				"decimals": 9
			},
			"0x75b8404f3979996fa7f861fee210a4346afed7fe": {
				"ticker": "APE",
				"address": "0x75B8404f3979996fa7F861FEE210A4346Afed7fE",
				"name": "APE COIN",
				"decimals": 1
			},
			"0x2c804d2a4578491014c9f8394e07a228df9b2576": {
				"ticker": "BBY",
				"address": "0x2c804D2a4578491014c9f8394e07A228Df9b2576",
				"name": "BabylonDAO",
				"decimals": 18
			},
			"0xd25605e5c5f093bec16dae8ec1737ce18badfacc": {
				"ticker": "NXS",
				"address": "0xd25605e5c5f093BEC16dae8ec1737CE18baDFaCC",
				"name": "Nexus DAO",
				"decimals": 18
			},
			"0x4b8e50cbf099533cf1c44b2740077431e09b9c0c": {
				"ticker": "AVULT",
				"address": "0x4B8E50CBF099533cF1C44B2740077431E09B9c0c",
				"name": "AVAXCULT",
				"decimals": 9
			},
			"0xaecf6da2f67077094ef14943b39612d62bc5e855": {
				"ticker": "MNT",
				"address": "0xAecf6da2f67077094EF14943B39612D62bC5E855",
				"name": "Mount Nodes",
				"decimals": 18
			},
			"0x2877bacd0f45f26f50a1a046956064a1ad7f5cdd": {
				"ticker": "SourceDoge",
				"address": "0x2877bacd0f45f26f50A1a046956064a1AD7F5CdD",
				"name": "SourceDoge",
				"decimals": 18
			},
			"0x8edaa085b2d5d2a92b45b5cb560344b471aa6ce8": {
				"ticker": "wBTC.p",
				"address": "0x8EDaa085b2D5D2a92b45B5CB560344B471Aa6CE8",
				"name": "wBTC.p",
				"decimals": 8
			},
			"0xcfff279d463a8099d9c40d97d653e4cecb604592": {
				"ticker": "APOLO",
				"address": "0xCfFf279d463A8099d9c40d97D653e4CEcb604592",
				"name": "Apolo Finance",
				"decimals": 18
			},
			"0x9a928d7dcd8d7e5cb6860b7768ec2d87b8934267": {
				"ticker": "BAMBOO-V2",
				"address": "0x9a928D7dcD8D7E5Cb6860B7768eC2D87B8934267",
				"name": "BambooToken v2",
				"decimals": 18
			},
			"0xfc740028d21aae8040fefa32207138423ed5d6b7": {
				"ticker": "BlockWarrior",
				"address": "0xFC740028d21aaE8040FefA32207138423eD5d6B7",
				"name": "BlockWarrior",
				"decimals": 4
			},
			"0xf8bb89e05c5e7551b605efeb616fb3126fac6a66": {
				"ticker": "EUP",
				"address": "0xf8Bb89e05c5E7551B605eFeB616fB3126fAC6a66",
				"name": "Euphoria",
				"decimals": 18
			},
			"0x66a4891c314ac10cf18e3897ee8aed5c5ee31f1b": {
				"ticker": "SN",
				"address": "0x66A4891C314aC10cf18e3897Ee8aed5C5ee31f1b",
				"name": "Supernova",
				"decimals": 9
			},
			"0xe9d00cbc5f02614d7281d742e6e815a47ce31107": {
				"ticker": "CRACK",
				"address": "0xE9D00cBC5f02614d7281D742E6E815A47ce31107",
				"name": "Crack.Fi",
				"decimals": 9
			},
			"0x235f8e64116898b1ec49e6feb20ef94c0833554a": {
				"ticker": "EMPIRE",
				"address": "0x235f8e64116898B1Ec49e6Feb20EF94c0833554a",
				"name": "Empire Nodes",
				"decimals": 1
			},
			"0x38030edf5f4be733ca07ea45c262827ca1d73109": {
				"ticker": "TITAN",
				"address": "0x38030EdF5f4be733ca07Ea45C262827ca1D73109",
				"name": "Titan Protocol",
				"decimals": 1
			},
			"0xa500fd6c9ee44d54ce36090f759588e9597565b2": {
				"ticker": "MNCDAO",
				"address": "0xa500Fd6C9eE44d54ce36090F759588E9597565b2",
				"name": "MemeNodeCapitalDAO",
				"decimals": 9
			},
			"0x9678f9df8ee97aa125a21240f31a82a0621cf93b": {
				"ticker": "LUNA",
				"address": "0x9678f9DF8eE97Aa125a21240F31A82a0621cF93b",
				"name": "LUNA NODES",
				"decimals": 1
			},
			"0x4271b8801f005a5ee0332140e19ecbcc55b71946": {
				"ticker": "BOB",
				"address": "0x4271B8801F005A5Ee0332140E19ecbCC55b71946",
				"name": "Boba Fett Avax",
				"decimals": 9
			},
			"0x9572ae322e6a2de8101ee775f774795b0ffe50b6": {
				"ticker": "MTRK",
				"address": "0x9572Ae322e6a2DE8101EE775F774795B0fFe50B6",
				"name": "Meta Turkey",
				"decimals": 9
			},
			"0xa3ab4c0192f42240216c3a2e22bfa7ad2af501ef": {
				"ticker": "SafeDAO",
				"address": "0xA3ab4C0192F42240216c3a2e22BFA7AD2Af501eF",
				"name": "SafeDAO",
				"decimals": 8
			},
			"0x9bd6182d3b42b98089bb565fe6935983f856540d": {
				"ticker": "CryptoWar",
				"address": "0x9bd6182D3b42b98089Bb565FE6935983f856540d",
				"name": "CryptoWars Game",
				"decimals": 8
			},
			"0xfdc4b47edad25be0d0ec017991b47760212c1aa3": {
				"ticker": "JADE",
				"address": "0xfDC4B47EDad25be0D0Ec017991B47760212C1aa3",
				"name": "JADE",
				"decimals": 1
			},
			"0x733247e62158e561c4b92d5fa57eef2005ecee08": {
				"ticker": "EXM",
				"address": "0x733247E62158E561c4b92d5fa57eEf2005ECeE08",
				"name": "Excellium Nodes",
				"decimals": 1
			},
			"0x896afaf43cbf15b1bcf1be946d7bce7cf59d4509": {
				"ticker": "SEA",
				"address": "0x896afaf43CBF15b1bcF1BE946D7bCe7Cf59D4509",
				"name": "DeepSeaDAO",
				"decimals": 1
			},
			"0xd5e08c50a96c2479392ec454e6ad67f5bbb21377": {
				"ticker": "Crypt",
				"address": "0xd5E08C50A96c2479392eC454E6AD67f5BBB21377",
				"name": "Crypto Freak",
				"decimals": 2
			},
			"0xf8f9b0dabd9338728928bddddbfa40d8e595d82d": {
				"ticker": "DOGEN",
				"address": "0xF8f9B0DaBD9338728928BDdDDbfa40D8e595d82d",
				"name": "Doge Nodes",
				"decimals": 1
			},
			"0x241ccc6bb9f3ac8d59d3bda69a6638240808353c": {
				"ticker": "PrintMim",
				"address": "0x241ccC6bb9f3Ac8D59D3bda69a6638240808353c",
				"name": "PrintMim",
				"decimals": 18
			},
			"0x860c28b613e0e8a10e936ae649c6c30af80fa5cb": {
				"ticker": "MIMAP",
				"address": "0x860c28B613e0e8a10e936Ae649c6c30af80fa5cb",
				"name": "MagicInternetAP",
				"decimals": 18
			},
			"0x5e1eabe734d0ea5973f033f7d416c87159bedec7": {
				"ticker": "Alpha",
				"address": "0x5e1EAbe734D0eA5973f033f7D416c87159BEdec7",
				"name": "Alpha DAO",
				"decimals": 18
			},
			"0x5657afef18fca403c0f9cb1a8eb8338404d5ca00": {
				"ticker": "JTP",
				"address": "0x5657afEf18FcA403c0f9cB1a8eb8338404D5cA00",
				"name": "Joe The Printer",
				"decimals": 9
			},
			"0xb91d77c8f0a1eede3357bb8dc57a6ec9b3d38d25": {
				"ticker": "MRD",
				"address": "0xb91D77c8F0a1EedE3357BB8Dc57A6eC9b3d38d25",
				"name": "MrNodes",
				"decimals": 6
			},
			"0xa80964be0f31229dd7b6567a33e383880ff1983e": {
				"ticker": "FLASK",
				"address": "0xA80964BE0f31229Dd7B6567a33E383880ff1983E",
				"name": "Flask",
				"decimals": 9
			},
			"0x5a44422beaaa38031f57720d88697105be6970be": {
				"ticker": "NGM",
				"address": "0x5A44422beaAA38031f57720d88697105be6970BE",
				"name": "e-Money NGM staking token",
				"decimals": 6
			},
			"0x894129a7ffe1f300d4fa88c61ffcbaed9019daf2": {
				"ticker": "AVAXREFI",
				"address": "0x894129a7ffe1F300D4FA88c61fFCBAEd9019Daf2",
				"name": "AVAXREFI",
				"decimals": 6
			},
			"0x353905c7430756f79b8fd1341ac41563b91b001b": {
				"ticker": "AFA",
				"address": "0x353905c7430756F79b8Fd1341aC41563B91B001b",
				"name": "Alice Frog AVAX",
				"decimals": 18
			},
			"0x75bb7f3ef1de175418a82285e4492e1b963b1e1c": {
				"ticker": "BOWL",
				"address": "0x75bB7F3Ef1DE175418a82285e4492e1B963B1E1c",
				"name": "SUPER BOWL INU",
				"decimals": 8
			},
			"0xc86a07f6ed1a46627ed434191ae0ddb167068e20": {
				"ticker": "One",
				"address": "0xc86a07f6eD1A46627eD434191AE0DDb167068E20",
				"name": "1TokenDao",
				"decimals": 18
			},
			"0xe6a34797a75cf9912524f7a55c5d0d3dc6d1c257": {
				"ticker": "Missle",
				"address": "0xE6A34797a75cF9912524f7A55c5D0D3DC6d1c257",
				"name": "Missle",
				"decimals": 8
			},
			"0xd14fee0e18d1ba745216c1ce26b075b44e0c654e": {
				"ticker": "WINK",
				"address": "0xD14fee0E18D1Ba745216c1CE26b075B44E0c654e",
				"name": "WinkMoney",
				"decimals": 2
			},
			"0xe9df9dd95f1f6cf8215c0f4ae25f39a34ff93753": {
				"ticker": "RND",
				"address": "0xe9DF9dD95F1F6Cf8215c0f4aE25F39A34Ff93753",
				"name": "Redlight Node District",
				"decimals": 9
			},
			"0xc5a32c65cc16a963a752e3d0cd97633d1365de40": {
				"ticker": "CAT",
				"address": "0xc5a32c65Cc16A963a752e3d0CD97633D1365dE40",
				"name": "Capital Aggregator Token",
				"decimals": 9
			},
			"0x484d732976095c6244882874a8c07429aca647ee": {
				"ticker": "wFTM.p",
				"address": "0x484D732976095c6244882874A8C07429ACA647EE",
				"name": "wFTM.printer",
				"decimals": 8
			},
			"0x99f6ffce4f17f1bf5927d8d60afdf8fb1ca20a6e": {
				"ticker": "Lan",
				"address": "0x99f6FFcE4F17f1BF5927d8d60aFDF8Fb1CA20A6E",
				"name": "Land Verse",
				"decimals": 18
			},
			"0xd73d65f11faefd441a0fb584ffff1a3d267fcf05": {
				"ticker": "GDLK",
				"address": "0xd73D65f11FAEFd441a0fB584FFFf1A3D267FcF05",
				"name": "GDLK",
				"decimals": 18
			},
			"0xb4cfc4140397c91709bc1ab1781335b78018e566": {
				"ticker": "mymommaalwayssaidlifeislikeaboxofchocolates",
				"address": "0xb4Cfc4140397C91709bC1AB1781335b78018E566",
				"name": "mymommaalwayssaidlifeislikeaboxofchocolates",
				"decimals": 18
			},
			"0xbf7ca6bce8ec680317e6eaf7c105ddecacc38c86": {
				"ticker": "Ninja",
				"address": "0xbF7cA6Bce8ec680317E6EaF7C105dDECacC38C86",
				"name": "Ninja avax",
				"decimals": 18
			},
			"0xb163429632d8e7e3c7c8547f7b73a3348b00abee": {
				"ticker": "SKYFALL",
				"address": "0xb163429632D8e7e3C7c8547F7B73A3348B00aBEe",
				"name": "SKYFALL",
				"decimals": 8
			},
			"0x22151e7053b6390c35c517e11dbbfdb969638837": {
				"ticker": "GhostDAO",
				"address": "0x22151e7053b6390C35c517e11DBBFDB969638837",
				"name": "GhostDAO",
				"decimals": 18
			},
			"0x1775f3e04cdfed4afbda32f523b5a8c041907edc": {
				"ticker": "Havaxii",
				"address": "0x1775f3E04cdfeD4afbDA32f523b5A8C041907Edc",
				"name": "PizzaHavaxii",
				"decimals": 6
			},
			"0x2479c5df9940042b851e70e4f922c7324721c6fd": {
				"ticker": "MANG",
				"address": "0x2479c5Df9940042B851E70e4F922C7324721c6fD",
				"name": "MANGO DAO",
				"decimals": 8
			},
			"0x43528a4e85f050027dce2d52955c39aba27a2a3d": {
				"ticker": "INU",
				"address": "0x43528a4E85F050027dCe2d52955C39aBA27A2a3d",
				"name": "INU SAFU LEMON",
				"decimals": 18
			},
			"0x032ff0a74f39b4450879d42e84d7bb58d97c5d4a": {
				"ticker": "POLAR",
				"address": "0x032Ff0A74f39B4450879d42e84d7bb58D97c5d4a",
				"name": "Polar Bear",
				"decimals": 9
			},
			"0x1762f180381ca1de617aa1f1b90a359376e67ca5": {
				"ticker": "ASPELL",
				"address": "0x1762f180381Ca1de617Aa1F1b90a359376E67cA5",
				"name": "AVA SPELL",
				"decimals": 6
			},
			"0xd8338e19ba66330725584703f541d2c43988a0ad": {
				"ticker": "SPN",
				"address": "0xd8338e19Ba66330725584703f541D2C43988A0Ad",
				"name": "SpookyNode",
				"decimals": 9
			},
			"0x138fc061bf269786ae7b51272324c126ccdf3d32": {
				"ticker": "RIPBOG",
				"address": "0x138Fc061bF269786Ae7B51272324C126CcDf3d32",
				"name": "RipBodganoff",
				"decimals": 18
			},
			"0xbd83010eb60f12112908774998f65761cf9f6f9a": {
				"ticker": "BOO",
				"address": "0xbD83010eB60F12112908774998F65761cf9f6f9a",
				"name": "SpookyToken",
				"decimals": 18
			},
			"0x57628823d8a869476fcc0af4ef6e50f1ec28b864": {
				"ticker": "Pilt",
				"address": "0x57628823d8A869476fcc0af4eF6e50F1EC28b864",
				"name": "PILTOVER",
				"decimals": 9
			},
			"0x3f337e437b9472c86672c48a437d6dcb7363c176": {
				"ticker": "AVAXinu",
				"address": "0x3F337E437B9472c86672C48a437d6dcB7363c176",
				"name": "AVAXinu",
				"decimals": 8
			},
			"0x76076880e1ebbce597e6e15c47386cd34de4930f": {
				"ticker": "OPUS",
				"address": "0x76076880e1EBBcE597e6E15c47386cd34de4930F",
				"name": "Canopus",
				"decimals": 18
			},
			"0x941aab36a8e0abaf7d7370a26e4da0bd621135b9": {
				"ticker": "avaWOOL",
				"address": "0x941AAB36a8E0AbAF7d7370a26e4da0bD621135B9",
				"name": "avaWOOL",
				"decimals": 18
			},
			"0x1fe4751d9bdabac8d90067056cb45ab6823d2c12": {
				"ticker": "ARGON",
				"address": "0x1fE4751d9bDabaC8D90067056cB45AB6823d2C12",
				"name": "ArgonToken",
				"decimals": 18
			},
			"0x4b74bcf3ec2661955c4ad42a45d647221f7c8d96": {
				"ticker": "DB",
				"address": "0x4B74bcF3Ec2661955C4ad42A45D647221f7C8D96",
				"name": "Diamond Bank Dao",
				"decimals": 5
			},
			"0xd756d16ceaf35544a43bf9a6d3dee628f09fcc95": {
				"ticker": "BBULS",
				"address": "0xD756d16ceAF35544A43Bf9A6D3dee628F09FCc95",
				"name": "BBULS",
				"decimals": 18
			},
			"0x8aa3801ab38a2a8a97442907babf0e5c4dfbf739": {
				"ticker": "MNT",
				"address": "0x8AA3801ab38A2A8A97442907BABF0e5c4DfBF739",
				"name": "Mount Node",
				"decimals": 18
			},
			"0xf114465d45be0ba1dd4f83fafbf33f98af7af3e9": {
				"ticker": "APLM",
				"address": "0xF114465d45bE0bA1Dd4F83faFBf33F98af7Af3e9",
				"name": "Apple Monster",
				"decimals": 12
			},
			"0x333cb47f000da21151ca60761245c202e9d03f8b": {
				"ticker": "FRC",
				"address": "0x333cB47f000DA21151CA60761245C202E9D03F8B",
				"name": "Fractals",
				"decimals": 5
			},
			"0x7b4d017396e7c23ad8fd8c4c7f7df1479f4fbe8e": {
				"ticker": "SantaFloki",
				"address": "0x7b4d017396e7C23aD8FD8c4C7f7DF1479F4fbE8E",
				"name": "SantaFloki",
				"decimals": 18
			},
			"0xe0c6290a292b346c5714cc3c09aaac7e2fe6b79c": {
				"ticker": "CPPR",
				"address": "0xE0C6290A292b346c5714cC3C09AAAc7e2Fe6B79C",
				"name": "COPPER FINANCE",
				"decimals": 18
			},
			"0xe09e1b08c14fe83de57bccdcc1076a03c23596ee": {
				"ticker": "FEB",
				"address": "0xE09E1b08C14FE83De57bccDCc1076A03c23596ee",
				"name": "ForeverBurn",
				"decimals": 6
			},
			"0x761c562e9f3fa19cebffe1ddec804c13d10c07ad": {
				"ticker": "LIO",
				"address": "0x761c562E9F3fa19cebFfe1DdEC804C13D10c07ad",
				"name": "LION KING",
				"decimals": 2
			},
			"0xb192dd720e4afc9d2f0883ac02564593ae525559": {
				"ticker": "ALF",
				"address": "0xB192dd720e4AFc9d2f0883aC02564593ae525559",
				"name": "Alice in Frogland",
				"decimals": 6
			},
			"0x7c12e313d6995ee5e130e87d3b01819bec0c7dbb": {
				"ticker": "MULTI",
				"address": "0x7c12E313d6995eE5E130E87D3B01819bec0c7dBB",
				"name": "Multiverse",
				"decimals": 18
			},
			"0x928722af2ab0478cb3fa131e008f10323e11b5e1": {
				"ticker": "Acash",
				"address": "0x928722AF2AB0478cB3FA131E008f10323e11B5e1",
				"name": "Avatarcash",
				"decimals": 18
			},
			"0x6bd3d40c0fd724c198cd69be4361aa2e6c467f7f": {
				"ticker": "Boson",
				"address": "0x6Bd3D40C0FD724c198cD69BE4361aA2e6C467F7f",
				"name": "BosonDAO",
				"decimals": 18
			},
			"0x9fea36baf4c8db4cebc8b6a0a9886febe7d83f0f": {
				"ticker": "CAMEO",
				"address": "0x9FeA36Baf4C8dB4ceBC8B6A0a9886feBE7D83f0f",
				"name": "CAMELEO LAND",
				"decimals": 18
			},
			"0xeb06b7bd0502939953302d65b0cd5f197f5231ec": {
				"ticker": "COWBOY",
				"address": "0xEb06b7bd0502939953302D65B0CD5F197f5231eC",
				"name": "Western Money",
				"decimals": 1
			},
			"0xd66090c8a8f0356e4893dcfd0a735d10016f7cf3": {
				"ticker": "OXY",
				"address": "0xd66090C8a8F0356e4893dcfD0a735d10016f7cf3",
				"name": "Oxy_Fi",
				"decimals": 18
			},
			"0x9f6aedca032b1092e08b848fc9d6f29139370837": {
				"ticker": "GAIA",
				"address": "0x9f6aEDcA032b1092E08b848FC9D6F29139370837",
				"name": "GAIA DAO",
				"decimals": 9
			},
			"0x04a20bc5319ba3a643d99de23bdcdffa0dae9b6f": {
				"ticker": "VDAO.a",
				"address": "0x04a20Bc5319BA3a643d99De23BdcDfFa0dAE9B6f",
				"name": "VaultDao",
				"decimals": 9
			},
			"0xe0e778a5c1f8d7af0e4d63134d4af239f4a12337": {
				"ticker": "SAKANASWAP",
				"address": "0xe0e778a5C1F8D7af0e4d63134D4Af239f4a12337",
				"name": "SAKANASWAP",
				"decimals": 9
			},
			"0xfc108f21931576a21d0b4b301935dac80d9e5086": {
				"ticker": "IronICE",
				"address": "0xfC108f21931576a21D0b4b301935DAc80d9E5086",
				"name": "Iron Finance ICE Token",
				"decimals": 18
			},
			"0xa93de5a916742ad9c9642477d34d345c24eda480": {
				"ticker": "BALAS",
				"address": "0xA93DE5A916742ad9c9642477d34D345C24eDa480",
				"name": "BalasPrinter",
				"decimals": 2
			},
			"0x2dd97c192ada967311f350b4a07de77b23f7429c": {
				"ticker": "SHOGUN",
				"address": "0x2dD97c192ada967311F350b4a07dE77B23F7429c",
				"name": "Shogun Protocol",
				"decimals": 1
			},
			"0x6e6effbb26fa9daf4b961364b7d5246b58760903": {
				"ticker": "AvAlps",
				"address": "0x6E6efFBb26FA9DaF4b961364b7D5246b58760903",
				"name": "AvAlps DAO",
				"decimals": 9
			},
			"0xab48f2889198352412b2c95940ae5d512d2ab537": {
				"ticker": "REFLECTION",
				"address": "0xab48F2889198352412b2c95940aE5d512D2Ab537",
				"name": "REFLECTION",
				"decimals": 5
			},
			"0xa3eb859d8ccb2dde5918aef9488b7ae8aa3908f0": {
				"ticker": "SPC",
				"address": "0xa3EB859d8CcB2DDE5918aef9488b7Ae8aa3908f0",
				"name": "SpaceNode",
				"decimals": 9
			},
			"0x68f23fbd0baf74a9c922c8060c3ec737968c5f27": {
				"ticker": "WoD",
				"address": "0x68f23FBD0BAF74A9c922C8060c3Ec737968C5F27",
				"name": "World of Dogs",
				"decimals": 9
			},
			"0xadfe140eac7fdb53de4d0270e1834cda89f57021": {
				"ticker": "Anode",
				"address": "0xADfe140eAc7FdB53De4D0270e1834cDA89f57021",
				"name": "Alpha Nodes",
				"decimals": 18
			},
			"0xcf321a5abfaeb9df213324f2ab6d2ac5ea1a82d5": {
				"ticker": "MOONSHARK",
				"address": "0xCf321a5aBfaeB9dF213324F2aB6D2aC5EA1A82d5",
				"name": "MoonShark",
				"decimals": 9
			},
			"0x6f7ff98d45fe0fd847e69748e5e0fe1c0b243867": {
				"ticker": "VXLN",
				"address": "0x6F7FF98d45Fe0Fd847e69748e5e0fe1C0B243867",
				"name": "AVAXOLUTION",
				"decimals": 8
			},
			"0xb0bd9ce393901b9af556bc20f25c2e4272fd7ea8": {
				"ticker": "Sant",
				"address": "0xB0bd9cE393901b9aF556bc20F25C2E4272FD7Ea8",
				"name": "Santa Clown",
				"decimals": 8
			},
			"0xaf13869b57926806f22fca2924c2bd0d9ad2df4b": {
				"ticker": "MIMp",
				"address": "0xAf13869b57926806F22fCA2924c2BD0D9ad2dF4b",
				"name": "MIM Printer",
				"decimals": 9
			},
			"0x32b825bde2c6bad4377d927c5af64479bb2c2470": {
				"ticker": "MP",
				"address": "0x32B825bde2c6BaD4377D927c5aF64479BB2c2470",
				"name": "MIMPAYER",
				"decimals": 6
			},
			"0xf8e956eb72db45b4d8855b903079ad85023f92f2": {
				"ticker": "AvaXola",
				"address": "0xf8E956eb72DB45B4d8855B903079Ad85023f92f2",
				"name": "AvaXola",
				"decimals": 18
			},
			"0x6dcc4e1a02851a84e50df4537be0e121fd95addd": {
				"ticker": "NOVAM",
				"address": "0x6DCC4e1A02851A84E50dF4537BE0E121fD95aDDD",
				"name": "Nova Money",
				"decimals": 18
			},
			"0x6033f3d6c653c9870a7475844663c48ddcbb5d62": {
				"ticker": "NODL",
				"address": "0x6033f3d6C653c9870A7475844663C48dDCBb5d62",
				"name": "NODLindex",
				"decimals": 9
			},
			"0x616726a56743f6fff7364df1632f9b2dd08b75f6": {
				"ticker": "ARCTICSHIBA",
				"address": "0x616726A56743f6FfF7364DF1632F9b2dd08B75f6",
				"name": "Arctic Shiba",
				"decimals": 9
			},
			"0xa005b96ef624a6646763715b667343341d590623": {
				"ticker": "romeFOS",
				"address": "0xA005B96EF624a6646763715b667343341d590623",
				"name": "FarmersOnly's Swap Rome Coin",
				"decimals": 18
			},
			"0xf2f7ce610a091b94d41d69f4ff1129434a82e2f0": {
				"ticker": "GG",
				"address": "0xF2F7CE610a091B94d41D69f4fF1129434a82E2f0",
				"name": "GalaxyGoggle",
				"decimals": 9
			},
			"0xf03ab1c1fccbcb470c47f3eb642db31c30d240a4": {
				"ticker": "MUG",
				"address": "0xf03AB1c1FCCbCb470c47f3EB642DB31c30d240A4",
				"name": "MoonMug",
				"decimals": 18
			},
			"0xbd0f95bbd7ab077fcecfd237a8419958b5177a67": {
				"ticker": "EthDAO",
				"address": "0xbd0F95BBD7AB077fCecFD237A8419958B5177A67",
				"name": "EtheriumDAO",
				"decimals": 8
			},
			"0x1e9950b8060ff5efb7884b7f906745307f77e0e1": {
				"ticker": "EMERALD",
				"address": "0x1E9950B8060FF5EFB7884b7f906745307F77e0E1",
				"name": "EMERALD",
				"decimals": 8
			},
			"0xc1961a1f11e129d2f2dd20c8cb9871cfc4a1be4a": {
				"ticker": "BTCp",
				"address": "0xC1961a1f11e129d2f2dd20C8Cb9871CfC4A1be4a",
				"name": "BTCpay",
				"decimals": 18
			},
			"0x96710777c55671678f55983786a1d102559140aa": {
				"ticker": "Hard",
				"address": "0x96710777C55671678F55983786A1D102559140aA",
				"name": "Hardhat",
				"decimals": 18
			},
			"0x10c62627282cecd3d4908403d8c19e29ab600913": {
				"ticker": "GM",
				"address": "0x10C62627282ceCD3D4908403D8c19E29Ab600913",
				"name": "GM",
				"decimals": 9
			},
			"0xd30246d56b69c696a23cc8bb06a7fa00c3c343b1": {
				"ticker": "DRIP",
				"address": "0xD30246d56B69C696A23CC8BB06A7fa00c3C343B1",
				"name": "Dripfinance",
				"decimals": 9
			},
			"0x79cd3e1d7af50bc801be2ef6c6cedfb191f9e983": {
				"ticker": "AvADAO",
				"address": "0x79CD3e1D7aF50bC801bE2Ef6C6CedFb191F9E983",
				"name": "AVALPS",
				"decimals": 18
			},
			"0xc5ccbe99c09e22a71d92db6f5780f77c31ee3640": {
				"ticker": "SMN",
				"address": "0xC5CcbE99c09e22a71d92Db6F5780F77C31EE3640",
				"name": "SMARTNODES",
				"decimals": 18
			},
			"0xe10a6583a800950c0449ed05acc4592d8fed92c5": {
				"ticker": "SHIBAVAX",
				"address": "0xe10A6583a800950C0449eD05ACc4592D8feD92c5",
				"name": "Shibavax",
				"decimals": 9
			},
			"0xc763f8570a48c4c00c80b76107cbe744dda67b79": {
				"ticker": "BETS",
				"address": "0xc763f8570A48c4c00C80B76107cbE744dDa67b79",
				"name": "BetSwirl Token",
				"decimals": 18
			},
			"0xd81ce652cd4213c073c9cfa8b7f6b00611c220fb": {
				"ticker": "DB",
				"address": "0xD81Ce652CD4213C073c9cfa8b7F6b00611C220Fb",
				"name": "Diamond Bank Dao",
				"decimals": 5
			},
			"0x4316a6bdb19508e3888565caaa0f16aa78a4681a": {
				"ticker": "xRing",
				"address": "0x4316a6BDb19508E3888565caaA0f16Aa78A4681a",
				"name": "Ring Financial",
				"decimals": 18
			},
			"0x38f3bc4c4c9f9189de3f66a67e1b8c13d1c92ec3": {
				"ticker": "MNODE",
				"address": "0x38F3Bc4C4c9F9189De3F66A67E1b8c13D1C92Ec3",
				"name": "MetaNode",
				"decimals": 9
			},
			"0x82de3511e4b7746f85387f0308fd7eb3de2126ab": {
				"ticker": "$WP",
				"address": "0x82De3511E4b7746f85387F0308Fd7eB3DE2126Ab",
				"name": "Wonder Printer",
				"decimals": 18
			},
			"0xd5280111de430f2d2609a91012e7dde1905a7f21": {
				"ticker": "JOC",
				"address": "0xd5280111de430f2D2609A91012E7dde1905a7F21",
				"name": "Joseph Capital",
				"decimals": 9
			},
			"0xbf7c0d02fab4f333d311e749d5a8feb8561fa86b": {
				"ticker": "LOOT",
				"address": "0xbf7c0d02fAB4f333d311E749D5A8Feb8561Fa86B",
				"name": "LOOT",
				"decimals": 8
			},
			"0x5fa3c882eef3a8d49c4466eeb7fbabb75a88643f": {
				"ticker": "CHAM",
				"address": "0x5FA3c882eeF3a8d49C4466eEb7fbaBB75a88643f",
				"name": "Champion",
				"decimals": 18
			},
			"0x4cf160ee56ba68ae37e9b5aaad687d23f9a4a7dc": {
				"ticker": "BTP",
				"address": "0x4cf160EE56ba68ae37e9B5aAAD687D23f9A4A7DC",
				"name": "BTCPAY",
				"decimals": 9
			},
			"0x407aaac52e04a208225174b70933beb6204d4faf": {
				"ticker": "SANTA",
				"address": "0x407AAAc52e04a208225174B70933bEb6204D4fAf",
				"name": "Santa Nodes",
				"decimals": 1
			},
			"0x11daaf3dfc15ee2cb8b3d298da751bb320e47dff": {
				"ticker": "PMINT",
				"address": "0x11dAAF3DFC15ee2cb8b3d298DA751bB320E47dff",
				"name": "Peppermint Finance",
				"decimals": 18
			},
			"0xca1068444196cdfe676fd15a29f35e502580a69e": {
				"ticker": "ANTG",
				"address": "0xCa1068444196cdfE676Fd15A29F35e502580A69E",
				"name": "AntGold",
				"decimals": 18
			},
			"0xe501dfad0372d4ef7cafddd47195e1c151002062": {
				"ticker": "BRIDGE",
				"address": "0xE501DFad0372D4eF7Cafddd47195e1c151002062",
				"name": "Avax Bridge Nodes",
				"decimals": 9
			},
			"0x64ddf59e2c26030d47f74524119bef0039ebe4aa": {
				"ticker": "FREEZ",
				"address": "0x64Ddf59E2C26030D47F74524119bef0039ebe4aA",
				"name": "FREEZY NODE",
				"decimals": 18
			},
			"0xd011048e0bdac71b26c543d3254c7b9f4f6fe2fd": {
				"ticker": "GroFI",
				"address": "0xd011048E0BDAc71b26c543D3254c7b9f4F6Fe2Fd",
				"name": "Growth Finance",
				"decimals": 18
			},
			"0xee9e3a3bffbc84e39d4ebc0bdd03cd2341adef8d": {
				"ticker": "OXY",
				"address": "0xee9E3A3BFfbc84E39d4Ebc0BDd03Cd2341AdeF8D",
				"name": "Oxy-Fi",
				"decimals": 9
			},
			"0xedd1f5c01410ea54f7da78c368bdadc2974b8878": {
				"ticker": "DIAMD",
				"address": "0xedd1f5c01410ea54F7da78c368Bdadc2974B8878",
				"name": "Diamonds",
				"decimals": 18
			},
			"0x464b2ceeddd0fe3e72a5e4807d7959c95f0b06e6": {
				"ticker": "TPG",
				"address": "0x464b2CEedDD0fe3E72A5E4807d7959c95F0b06e6",
				"name": "Topo Gigio",
				"decimals": 18
			},
			"0xaf922e4f065231c8bc79b6084eec420b26d3c7eb": {
				"ticker": "BSN",
				"address": "0xAF922e4f065231c8bc79B6084eec420B26D3C7EB",
				"name": "Boson Dao",
				"decimals": 1
			},
			"0x3849f304ee9d85cb9260cf90ddb6eb78060e3b2c": {
				"ticker": "BREEZE",
				"address": "0x3849f304eE9D85cb9260CF90DDb6EB78060e3b2C",
				"name": "BREEZE NODES",
				"decimals": 9
			},
			"0xed6bb7c4d8e835653b910df2bdf9196cde216684": {
				"ticker": "CHASE",
				"address": "0xeD6bb7c4d8E835653B910DF2bDF9196CDE216684",
				"name": "CHASE",
				"decimals": 9
			},
			"0x5533bc171dfab48e8bf0079af3cb78b2330e46f3": {
				"ticker": "Rudolp",
				"address": "0x5533BC171DFAB48E8BF0079Af3cb78b2330e46f3",
				"name": "Rudolph Inu",
				"decimals": 9
			},
			"0x4a62be9c2f9a875e399214e6dd8c8764629177f0": {
				"ticker": "RING",
				"address": "0x4a62bE9c2F9a875E399214E6Dd8c8764629177f0",
				"name": "RING",
				"decimals": 1
			},
			"0x84a7cc1c1c614bbcd491f04a735ff4a421922a70": {
				"ticker": "Discovery",
				"address": "0x84A7Cc1C1C614bBCD491F04A735ff4A421922a70",
				"name": "Discovery",
				"decimals": 2
			},
			"0x7789f4f5b5af47e32fd7b15c7f783b4437999529": {
				"ticker": "DEGENODE",
				"address": "0x7789F4f5B5AF47E32fd7B15C7f783B4437999529",
				"name": "DEGENODE",
				"decimals": 8
			},
			"0xb89fb1408a0f03b158a27351c41baf50a2090045": {
				"ticker": "WebRTC",
				"address": "0xb89fb1408A0f03b158A27351C41bAf50A2090045",
				"name": "WEB RTC",
				"decimals": 8
			},
			"0xda68efd31ad6dba4f062011abdf327e1d2f105da": {
				"ticker": "RING",
				"address": "0xDA68EFd31ad6dbA4F062011aBdF327e1D2F105DA",
				"name": "LOTR Dao",
				"decimals": 1
			},
			"0xb4bba29e78cda418e99e41714178d2ef9c49760d": {
				"ticker": "GRAPE",
				"address": "0xb4bba29e78cdA418E99E41714178d2eF9c49760D",
				"name": "Grape Finance",
				"decimals": 18
			},
			"0x7aed00dc825f249c9e5fc97ed861c717d6012f85": {
				"ticker": "EUPHORIA",
				"address": "0x7AEd00DC825f249c9e5fC97eD861C717d6012f85",
				"name": "EUPHORIA",
				"decimals": 6
			},
			"0x18fb8d0c80c88fe7ef0472f93a62fa0fa56861c2": {
				"ticker": "DMTR",
				"address": "0x18fb8d0C80c88fE7eF0472f93A62fA0FA56861c2",
				"name": "Demeter",
				"decimals": 18
			},
			"0xa32e2c1180836983cf93b6e685dfb6aa1c063883": {
				"ticker": "COSMOS",
				"address": "0xa32e2c1180836983cf93b6E685dfB6aA1c063883",
				"name": "Cosmos Dog",
				"decimals": 18
			},
			"0x73458ba892e52bb40dea2e72dca6cc8ae84fb2f3": {
				"ticker": "Tita",
				"address": "0x73458BA892e52BB40DeA2e72DCA6cC8AE84fb2f3",
				"name": "Titan Hunters",
				"decimals": 6
			},
			"0xd6c98bf7c24c05b72e703488675afdd57cf475d4": {
				"ticker": "GLXY",
				"address": "0xd6C98bf7c24c05B72e703488675AFDd57cF475d4",
				"name": "Galaxy Money",
				"decimals": 1
			},
			"0xb63af57136459cf016b54cce5702c3dfaf297660": {
				"ticker": "GEM",
				"address": "0xb63Af57136459CF016B54cCe5702c3dFaF297660",
				"name": "GEM NODES",
				"decimals": 1
			},
			"0x98d5a722a282f9a9ff63da8a880929cdf587d8d0": {
				"ticker": "ICE",
				"address": "0x98D5a722a282F9A9fF63dA8A880929cDF587D8D0",
				"name": "Icefall",
				"decimals": 18
			},
			"0xaafd2577fb67366d3c89db0d627c49d769ee2e5d": {
				"ticker": "SUSD",
				"address": "0xAafd2577Fb67366d3C89DB0d627C49D769ee2e5D",
				"name": "Stabilize USD",
				"decimals": 18
			},
			"0x09e3794910c24a4875bd6ee1b2600ddee349073e": {
				"ticker": "Oxyge",
				"address": "0x09e3794910C24A4875Bd6Ee1B2600DDEE349073e",
				"name": "Oxygen Protocol",
				"decimals": 8
			},
			"0x6e306f4504396a0b7d9de00575544c898a21a2e3": {
				"ticker": "IMHOTEP",
				"address": "0x6E306F4504396A0B7D9De00575544C898A21A2e3",
				"name": "Imhotep",
				"decimals": 18
			},
			"0xc0a47f76cb3433ed34247b256a629b4d2dfc7b3b": {
				"ticker": "HUNT",
				"address": "0xC0A47F76Cb3433ED34247b256a629B4d2DFc7B3B",
				"name": "Cross-Chain Capital",
				"decimals": 18
			},
			"0x6924268db458f47e15db1aa6b25f3455750bf24c": {
				"ticker": "UFO",
				"address": "0x6924268DB458f47E15db1Aa6b25F3455750bF24C",
				"name": "U.F.O City",
				"decimals": 5
			},
			"0x2f3ac5502b376c4a22e7aedf07241ebb8cccb9e5": {
				"ticker": "ATREUS",
				"address": "0x2F3ac5502B376c4a22E7AEDf07241eBB8cccB9e5",
				"name": "ATREUS",
				"decimals": 6
			},
			"0xccefe8eb36d314da217c8bef111567017b14cab7": {
				"ticker": "STAR",
				"address": "0xCCefE8Eb36d314dA217c8BEF111567017B14cab7",
				"name": "Starnodes",
				"decimals": 9
			},
			"0x7c01d486deb4d30f7698724e9f335705b427cdf6": {
				"ticker": "KATANA",
				"address": "0x7C01D486dEb4D30f7698724E9f335705B427cdf6",
				"name": "Samurai Finance",
				"decimals": 18
			},
			"0xe556689036db5669ebe1a27187501843c9242552": {
				"ticker": "KANS",
				"address": "0xe556689036dB5669Ebe1a27187501843c9242552",
				"name": "Kansoru Compiler",
				"decimals": 9
			},
			"0xd149279e3294933ed1396c84a8f84f76e16c20b1": {
				"ticker": "UFO",
				"address": "0xd149279E3294933Ed1396c84a8f84F76E16C20b1",
				"name": "U.F.O City",
				"decimals": 18
			},
			"0x211a03bd9dd9b3ccad6b355ad82334592e3fcedd": {
				"ticker": "LAND",
				"address": "0x211A03bd9dD9b3ccAd6B355aD82334592E3Fcedd",
				"name": "Land Verse",
				"decimals": 9
			},
			"0x5503cc2b6b49396fb32b7cbbdd1a9d8b2bec1de1": {
				"ticker": "IRON",
				"address": "0x5503CC2b6B49396fB32b7cBbdd1A9D8b2BEC1dE1",
				"name": "Iron Dao",
				"decimals": 1
			},
			"0xab7daa2b42040d71193dc80ee451c72bdfe611bb": {
				"ticker": "BOOBA",
				"address": "0xab7Daa2B42040d71193Dc80ee451c72bdfe611Bb",
				"name": "BoobaDao",
				"decimals": 9
			},
			"0x95d16b76a4f29dbdb2d9ea2c4d0a31e2a1d830b3": {
				"ticker": "AKBASH",
				"address": "0x95d16B76A4F29dBdb2D9Ea2c4D0a31e2a1D830B3",
				"name": "AKBASH",
				"decimals": 9
			},
			"0x00000000215a30531b1a07e01655f25c378d3c31": {
				"ticker": "Hypercube",
				"address": "0x00000000215A30531b1a07e01655F25c378d3c31",
				"name": "Hypercube Money",
				"decimals": 18
			},
			"0x54d55614f632580603ae0d43d514df8b59d8e6c6": {
				"ticker": "DARK",
				"address": "0x54d55614F632580603AE0D43D514dF8b59D8e6C6",
				"name": "Dark Nodes",
				"decimals": 1
			},
			"0xed2f05e2f0a6ee2374fe09407e08a6f1bf83a4aa": {
				"ticker": "EPR",
				"address": "0xED2f05e2F0A6EE2374fe09407E08A6F1Bf83a4Aa",
				"name": "EPRINTER",
				"decimals": 6
			},
			"0xde81c6101b545c0853123198696d73f1c3f19489": {
				"ticker": "SDOGV2",
				"address": "0xde81C6101b545C0853123198696d73F1c3F19489",
				"name": "SnowDogV2",
				"decimals": 9
			},
			"0xe1cfa856f249eb7f328c7164e1d1c623b285c3aa": {
				"ticker": "GG",
				"address": "0xE1CfA856F249eb7f328c7164e1d1c623b285c3Aa",
				"name": "Galactic Gold",
				"decimals": 18
			},
			"0xe6b9d092223f39013656702a40dbe6b7decc5746": {
				"ticker": "ANGLE",
				"address": "0xE6B9d092223f39013656702A40dbE6B7DeCc5746",
				"name": "ANGLE",
				"decimals": 18
			},
			"0x0ba2b39991ebd1810a07658d7d9a65e16275cc9c": {
				"ticker": "DB",
				"address": "0x0Ba2B39991EBD1810a07658d7d9A65E16275Cc9C",
				"name": "Diamond Bank Dao",
				"decimals": 5
			},
			"0x1471dcb04fdcf0959e0a897b733df19a8faa566a": {
				"ticker": "DREAMS",
				"address": "0x1471DCb04fdCF0959E0a897b733dF19A8FaA566a",
				"name": "Dream Nodes",
				"decimals": 1
			},
			"0xe5870876cbe0674e5407281cd7fff61a767e6961": {
				"ticker": "SPARK",
				"address": "0xe5870876Cbe0674e5407281CD7fFf61A767E6961",
				"name": "Sparkling Finance",
				"decimals": 18
			},
			"0xe35bec53c8a0d5e78276fc0ce20f6c5c96630dfb": {
				"ticker": "SpaceGame",
				"address": "0xe35Bec53c8a0d5e78276Fc0ce20f6C5C96630DFB",
				"name": "SpaceGame",
				"decimals": 8
			},
			"0x326de30e805416b28de3633deff6cf44f3651291": {
				"ticker": "SPELLMP",
				"address": "0x326dE30E805416B28De3633DefF6cf44f3651291",
				"name": "SPELL MONEY PRINTER",
				"decimals": 18
			},
			"0x0dcf35f82f19a930f01392db38a5037ce3cce235": {
				"ticker": "MILK",
				"address": "0x0dCF35f82f19a930f01392db38A5037ce3CCe235",
				"name": "Dairy Money",
				"decimals": 5
			},
			"0xfa7d7f15be13c764b3b532ff9da7b7a6513755a1": {
				"ticker": "VIKING",
				"address": "0xfA7d7F15bE13C764B3b532Ff9DA7b7a6513755a1",
				"name": "Viking Nodes",
				"decimals": 18
			},
			"0x5ef90fcc89ba90aa8855a059bebed15bc0d8810c": {
				"ticker": "SEC",
				"address": "0x5EF90FCC89Ba90Aa8855A059BEbED15BC0D8810c",
				"name": "Secure Finance",
				"decimals": 18
			},
			"0xc13f380116cb25c4aa8e99ef302ed5c0b8034fec": {
				"ticker": "STRZ",
				"address": "0xc13f380116cB25c4aa8e99eF302Ed5c0B8034fEC",
				"name": "MorningStar Nodes",
				"decimals": 1
			},
			"0xf77cd7eefce9ab8350f7a32d0fd4bc6655ed9ceb": {
				"ticker": "COMET",
				"address": "0xf77cD7EEfce9AB8350f7A32d0FD4BC6655Ed9ceB",
				"name": "CometNode",
				"decimals": 9
			},
			"0xc713c20455ef272a02c4014cc9c2c64c7bb84e9f": {
				"ticker": "EcoCrypto",
				"address": "0xC713C20455Ef272a02C4014cC9c2c64c7bB84e9F",
				"name": "EcoCrypto ",
				"decimals": 8
			},
			"0x224633f41e3e2128f9c4449872257fd940049342": {
				"ticker": "PTPp",
				"address": "0x224633f41e3e2128F9c4449872257fD940049342",
				"name": "PTP Printer",
				"decimals": 18
			},
			"0x6bc1ca78d6914813e5c576dce2e30f4f771de6c7": {
				"ticker": "VPND",
				"address": "0x6bc1ca78d6914813e5C576Dce2E30f4F771DE6C7",
				"name": "VaporNodes",
				"decimals": 18
			},
			"0x2e4230bbf00a01296a0916b43434b70a4a942981": {
				"ticker": "CRAV",
				"address": "0x2E4230BBf00A01296A0916b43434B70a4a942981",
				"name": "CRAVERSE",
				"decimals": 6
			},
			"0xcfd48098261816d2cb38129960227d4ae8e2b648": {
				"ticker": "Bex",
				"address": "0xCFD48098261816D2cb38129960227D4Ae8e2B648",
				"name": "Byte Ex",
				"decimals": 18
			},
			"0x655821e8f5f49d0d8ca736b66a9a699f2355714f": {
				"ticker": "RING",
				"address": "0x655821e8f5F49D0d8CA736B66a9a699F2355714F",
				"name": "RING Nodes",
				"decimals": 1
			},
			"0x4f8523f590d8993bf655a26bcefe92cb42fd676f": {
				"ticker": "EARTH",
				"address": "0x4f8523f590D8993Bf655a26BCeFE92cB42Fd676F",
				"name": "EARTH Finance",
				"decimals": 18
			},
			"0x7d91edc0ccd7d0af51d34d26bcae7cc09053d4f6": {
				"ticker": "Flash",
				"address": "0x7D91edc0cCd7D0AF51D34d26bCaE7Cc09053d4f6",
				"name": "Flash Nodes",
				"decimals": 1
			},
			"0xb22fdbbf1ea270de7f5494e93d990607ea6a6878": {
				"ticker": "EXM",
				"address": "0xB22FdbBF1ea270dE7F5494e93D990607EA6a6878",
				"name": "Excellium Nodes",
				"decimals": 5
			},
			"0xa2776a53da0bf664ea34b8efa1c8ab4241a10968": {
				"ticker": "BLIZZ",
				"address": "0xa2776A53Da0bf664EA34B8Efa1c8aB4241A10968",
				"name": "Blizzard",
				"decimals": 18
			},
			"0x16cd790276b987050f33c28f452d90ad0e24df64": {
				"ticker": "GM",
				"address": "0x16Cd790276B987050F33C28f452D90ad0E24df64",
				"name": "GM",
				"decimals": 9
			},
			"0x6293bd9d427ba0bfe40641f6fb9df061aea1e5bd": {
				"ticker": "DWNBD",
				"address": "0x6293Bd9D427ba0bFE40641F6fb9df061aea1e5bd",
				"name": "DOWNBAD",
				"decimals": 6
			},
			"0x4eff1a51fefb6708ca796e09c2e4ecc1a1657057": {
				"ticker": "MULTI",
				"address": "0x4eFf1A51FEfb6708Ca796E09C2e4ECC1a1657057",
				"name": "Multiversum",
				"decimals": 5
			},
			"0x35a235798b6d2c3c4a5bbbdb03de0c794dde3d46": {
				"ticker": "Frogfather",
				"address": "0x35a235798B6d2C3c4A5BbbDB03dE0c794dDe3d46",
				"name": "Frogfather",
				"decimals": 18
			},
			"0x48a31101dd18c9952f1bdfbaea4d307296d6f07b": {
				"ticker": "PMAKER",
				"address": "0x48a31101DD18c9952f1bdfBaEa4d307296D6F07B",
				"name": "PlayMatesMaker",
				"decimals": 6
			},
			"0x17f63ba115e09cbdb4548eb8a1743b4796cc378b": {
				"ticker": "MEAN",
				"address": "0x17f63bA115e09cbDB4548eb8A1743B4796Cc378B",
				"name": "Mean Finance",
				"decimals": 9
			},
			"0x1b7d9c2844f2e4704cce50761abe6d491233dc6c": {
				"ticker": "SSHIB",
				"address": "0x1b7D9c2844f2e4704cce50761AbE6D491233dC6C",
				"name": "SnowShib",
				"decimals": 18
			},
			"0x6a4f97674c92b0417af864013dbceea8cf285547": {
				"ticker": "Olympus",
				"address": "0x6a4f97674c92B0417af864013dbCEeA8CF285547",
				"name": "Olympus Printer",
				"decimals": 18
			},
			"0x2031efd9536f8ace100becf2416487ea1a8d7468": {
				"ticker": "SOAP",
				"address": "0x2031efD9536f8acE100Becf2416487EA1A8D7468",
				"name": "Soap Finance",
				"decimals": 9
			},
			"0x062946593416ba728d2da3c1f1c30456aae838a1": {
				"ticker": "CLAVAX",
				"address": "0x062946593416ba728d2Da3C1F1C30456aae838A1",
				"name": "dailyavaxplay @dailyavaxplay",
				"decimals": 8
			},
			"0x3e0f20f6d2b5639e92141b6d0ac4c89ff3a94b54": {
				"ticker": "BLOC",
				"address": "0x3E0f20f6d2B5639e92141b6d0Ac4C89FF3a94b54",
				"name": "Blockcloud",
				"decimals": 9
			},
			"0x885efa0adfdb93466136fb1cca7713035ac28808": {
				"ticker": "IMPR",
				"address": "0x885EFA0AdFdB93466136Fb1cca7713035aC28808",
				"name": "Imperio DAO",
				"decimals": 9
			},
			"0xfa73784709a7dfee9787a2beb4d4484870be7175": {
				"ticker": "Akit",
				"address": "0xFa73784709A7dFee9787a2beb4d4484870be7175",
				"name": "Akita DAO",
				"decimals": 18
			},
			"0x249030bc9c5c8e40e07409984109ae8b4811ad3b": {
				"ticker": "FlokiDao",
				"address": "0x249030bc9c5C8e40E07409984109Ae8b4811Ad3B",
				"name": "Floki Dao",
				"decimals": 9
			},
			"0x2fb86477480483d1874c51d10292b9e68181f225": {
				"ticker": "ENC",
				"address": "0x2FB86477480483d1874c51D10292b9e68181F225",
				"name": "Encrypted DAO",
				"decimals": 18
			},
			"0xdac5e0760d582ed673f2c695cc6a60526a18f018": {
				"ticker": "HORUS",
				"address": "0xDac5E0760D582eD673F2C695cC6A60526a18f018",
				"name": "Horus DAO",
				"decimals": 18
			},
			"0xbad10a7299abf3a061e21770cddb4a1764cd1a97": {
				"ticker": "ZEUS",
				"address": "0xbaD10a7299Abf3A061E21770cddB4A1764cD1a97",
				"name": "Zeus Dao",
				"decimals": 18
			},
			"0x95896e9920c3d06c276aa3a2746b23fe91456940": {
				"ticker": "DS",
				"address": "0x95896e9920C3d06c276aA3A2746b23Fe91456940",
				"name": "DefiSyndicate",
				"decimals": 18
			},
			"0xae65a8a0a30dc3d3e2ecb0d061b334730849c933": {
				"ticker": "SMR",
				"address": "0xAE65A8A0a30dC3d3e2ecB0D061B334730849c933",
				"name": "Samurai Shiba",
				"decimals": 8
			},
			"0xfeaa8e2fb2036e8b16a7e38c20dec904ce3f5b4b": {
				"ticker": "STAR",
				"address": "0xfEAA8E2fb2036E8b16a7e38C20DEc904ce3F5b4B",
				"name": "Star Finance",
				"decimals": 1
			},
			"0x3d5b4c3115c1700eec3b734bc7b38784e53be9d7": {
				"ticker": "PXT",
				"address": "0x3d5B4C3115c1700EEC3b734Bc7b38784E53BE9D7",
				"name": "Project X Nodes",
				"decimals": 1
			},
			"0x264cad9b582abc214edb9f7972e1446cb2211d46": {
				"ticker": "MINA",
				"address": "0x264cAd9B582abc214edB9f7972E1446cb2211d46",
				"name": "MINA",
				"decimals": 18
			},
			"0x4eafa9990b03e344dddb2b52702c871690c1d0c6": {
				"ticker": "DMTR",
				"address": "0x4eafa9990b03E344ddDB2b52702c871690C1d0C6",
				"name": "Demeter",
				"decimals": 18
			},
			"0xb88e3edb378ed7ddef10b86962d97fa0b8defb6d": {
				"ticker": "SASHI",
				"address": "0xb88E3eDB378eD7ddeF10B86962d97fA0b8DEFb6D",
				"name": "SashimiDAO",
				"decimals": 9
			},
			"0x822614dcc4f35a9e9d29090e58d658a360ab1e25": {
				"ticker": "CINUA",
				"address": "0x822614dcc4F35a9e9D29090E58d658a360aB1E25",
				"name": "CHEEMSINUAVAX",
				"decimals": 9
			},
			"0x1c72870d374375285b257b259012ae1180111648": {
				"ticker": "Hawaii",
				"address": "0x1c72870d374375285B257B259012ae1180111648",
				"name": "Hawaii",
				"decimals": 9
			},
			"0x276247a4213ba439513451e6e3cd397bf768ec8e": {
				"ticker": "EXE",
				"address": "0x276247a4213Ba439513451E6E3cd397Bf768ec8E",
				"name": "EXUDOS",
				"decimals": 18
			},
			"0x62a4f3280c02c8cc3e9ff984e4aad94f8f7fea26": {
				"ticker": "BABYPangolin",
				"address": "0x62a4f3280C02C8Cc3E9ff984e4aaD94f8F7fEA26",
				"name": "BABYPangolin",
				"decimals": 9
			},
			"0x0806f788a9c23ab60086dbd7fbf7f0191c2670f1": {
				"ticker": "YVRS",
				"address": "0x0806F788A9C23aB60086dbd7fBF7F0191c2670f1",
				"name": "YOUNIVERSE",
				"decimals": 6
			},
			"0x9f074b54f1422bfa2e2a9f22388e97470b522146": {
				"ticker": "130-AVAX",
				"address": "0x9F074B54F1422BfA2e2a9F22388e97470b522146",
				"name": "130-AVAX Strike Token",
				"decimals": 18
			},
			"0x86bf08de13cec89e4b3732fac6f339b4d00ac0ce": {
				"ticker": "PRMD",
				"address": "0x86Bf08DE13ceC89E4B3732faC6F339B4D00ac0cE",
				"name": "Pyramids",
				"decimals": 18
			},
			"0x6a804b22c8b9965d6fcf7c12366cb85e417701cf": {
				"ticker": "TAVAX",
				"address": "0x6A804b22c8b9965D6fcF7C12366CB85E417701cf",
				"name": "Terrific Protocol Peg-0.01AVAX",
				"decimals": 18
			},
			"0xb60d38562946b29af70dddbbae4abbc609324098": {
				"ticker": "ISLAND",
				"address": "0xb60d38562946b29af70dddbbae4aBBc609324098",
				"name": "AVAX Island",
				"decimals": 5
			},
			"0xa5274988ec4d84c885799cf2fe85d2b353cd07ad": {
				"ticker": "MART",
				"address": "0xa5274988eC4d84C885799cF2FE85d2b353CD07aD",
				"name": "MetaArt (Wormhole)",
				"decimals": 18
			},
			"0x73b9cff73b05bd68bdd3f762518828c02a47ab07": {
				"ticker": "QTM",
				"address": "0x73b9Cff73b05Bd68Bdd3F762518828C02a47ab07",
				"name": "Quantum Verse",
				"decimals": 1
			},
			"0x00be972d33f3e25469c18183e727b9dcf52c2e5a": {
				"ticker": "SPACE",
				"address": "0x00BE972D33F3E25469c18183E727B9dCF52C2e5a",
				"name": "Space Finance",
				"decimals": 1
			},
			"0x6723ed0d444c83c354594956ab00a7d3212a88c1": {
				"ticker": "QTM",
				"address": "0x6723Ed0D444C83c354594956aB00a7D3212A88C1",
				"name": "Quantum Verse",
				"decimals": 1
			},
			"0xb183c1c7d7da73d3d9c1dcdfa020f2eed3fb8973": {
				"ticker": "COKE",
				"address": "0xB183C1C7D7Da73D3d9c1dCDfa020F2eed3fb8973",
				"name": "Cocaine Cash",
				"decimals": 9
			},
			"0x7dbe740fc2732eb4a0978aa860a102ab547bdfdd": {
				"ticker": "STAR",
				"address": "0x7DBe740fC2732Eb4A0978AA860A102AB547BdFDD",
				"name": "STAR",
				"decimals": 8
			},
			"0x236a6dd53529f34ec497275fe82e6b4980876717": {
				"ticker": "ETRNL",
				"address": "0x236A6dd53529f34ec497275FE82e6B4980876717",
				"name": "Eternal Money",
				"decimals": 1
			},
			"0x3159ba29310bfacd3c01ae3741f1f09f51db7123": {
				"ticker": "DGIV",
				"address": "0x3159ba29310BfaCd3c01Ae3741F1F09f51Db7123",
				"name": "Doggiverse",
				"decimals": 18
			},
			"0xadc325eb420eca68022f4e6ffb5b6ef62547ce72": {
				"ticker": "ELP",
				"address": "0xAdc325EB420ecA68022f4e6FFB5b6ef62547Ce72",
				"name": "Elpi DAO",
				"decimals": 18
			},
			"0xdf540b78a58c10641208e76ad867194ffd9cf65d": {
				"ticker": "FP",
				"address": "0xdF540b78a58c10641208e76Ad867194fFD9Cf65d",
				"name": "FlakePrinter",
				"decimals": 9
			},
			"0xdad21e53aec6fa90e52f573807f0378350190686": {
				"ticker": "UNP",
				"address": "0xDad21E53AeC6FA90E52F573807f0378350190686",
				"name": "UniPrinter",
				"decimals": 9
			},
			"0xf2adfb112fda390b8e233a0a88349730a14133eb": {
				"ticker": "XSHARE",
				"address": "0xF2Adfb112FDA390B8E233a0A88349730A14133eB",
				"name": "XSHARE",
				"decimals": 18
			},
			"0xc29da3de62731a3c8804615aaba73143e710c13c": {
				"ticker": "MIMPAY",
				"address": "0xc29Da3DE62731A3C8804615aaBa73143E710c13C",
				"name": "MIMPayer",
				"decimals": 18
			},
			"0x0b7b5f822b88674bb1bd2823c7dc893165ae2fe9": {
				"ticker": "Nuclea",
				"address": "0x0B7B5f822b88674BB1BD2823c7Dc893165AE2Fe9",
				"name": "Nuclear Protocol",
				"decimals": 8
			},
			"0x353af195c0d82820c59427bfbd318a49971aae1a": {
				"ticker": "SPACE",
				"address": "0x353af195c0d82820c59427BFbD318A49971AAe1a",
				"name": "Space",
				"decimals": 9
			},
			"0xee73202ee64ac83a30768f3407edde05640aad92": {
				"ticker": "bJOE",
				"address": "0xEE73202Ee64aC83A30768F3407EDDe05640AAd92",
				"name": "BabyJoe",
				"decimals": 9
			},
			"0x77c16a22cc112acbb240f237dde93f96b77385ef": {
				"ticker": "SLAYER",
				"address": "0x77C16a22cc112aCbb240F237dDE93f96b77385ef",
				"name": "SLAYER",
				"decimals": 2
			},
			"0x69ac2119808aea56e88cf7269c85b96744cc7c19": {
				"ticker": "AIR",
				"address": "0x69ac2119808Aea56E88Cf7269c85B96744cc7C19",
				"name": "Air Nodes",
				"decimals": 1
			},
			"0xdc05bff771cd5eb1bdc3a76950144ffd8d64efc0": {
				"ticker": "SUN",
				"address": "0xdC05BFF771Cd5eb1BdC3A76950144fFd8d64EFc0",
				"name": "Sun",
				"decimals": 18
			},
			"0xda9f903dd76acffb80c0faf636cc2454d17f23da": {
				"ticker": "PINU",
				"address": "0xda9F903Dd76AcFfb80C0FAF636Cc2454d17f23DA",
				"name": "Playmate INU",
				"decimals": 9
			},
			"0xf762b8903ac4caf87a97d43ac4b1fe65d1ef7fe2": {
				"ticker": "AvaxDoge",
				"address": "0xf762B8903aC4CAF87A97d43Ac4b1fe65D1ef7fE2",
				"name": "AvaxDoge",
				"decimals": 9
			},
			"0x236d6ae1e386b7a6ee982369bac5b424bb296e75": {
				"ticker": "MDAO",
				"address": "0x236D6aE1e386B7a6Ee982369bAc5B424bB296e75",
				"name": "MIM DAO",
				"decimals": 9
			},
			"0x2e7a121cff202ac14c8a264e191404eae3ddea12": {
				"ticker": "LAND",
				"address": "0x2E7a121CfF202Ac14c8a264E191404EAe3ddea12",
				"name": "Land Verse",
				"decimals": 9
			},
			"0xb4bb81f9bb55706ddaf0410ebb941073577f5e56": {
				"ticker": "MILK",
				"address": "0xB4Bb81F9bB55706DDAf0410Ebb941073577f5E56",
				"name": "Dairy Money",
				"decimals": 5
			},
			"0xd320b8fbdff6c4393aafa5cc1d4eed9635eec183": {
				"ticker": "ETHS",
				"address": "0xd320b8FbDFF6C4393AAFA5Cc1d4eEd9635EEC183",
				"name": "Etherstones",
				"decimals": 18
			},
			"0xfa283e6877a2439789325e1282c6a1b054567c51": {
				"ticker": "SATURN",
				"address": "0xFa283E6877a2439789325e1282C6a1b054567C51",
				"name": "Saturn Money",
				"decimals": 1
			},
			"0x6029ef3b8d13893126a10c0275fe3468aaf6277a": {
				"ticker": "JOEP",
				"address": "0x6029ef3B8D13893126A10c0275FE3468AAf6277A",
				"name": "JoePrinter",
				"decimals": 18
			},
			"0x645da3b9bc5b18d540bef600bd17bee7b61bb597": {
				"ticker": "PEPE",
				"address": "0x645DA3b9bC5B18d540BEF600Bd17bee7B61Bb597",
				"name": "PEPEtinginu",
				"decimals": 1
			},
			"0x6f3773e9639170c17d711933240045c5c413b48f": {
				"ticker": "Snowtama",
				"address": "0x6f3773e9639170C17D711933240045c5C413b48F",
				"name": "Snowtama",
				"decimals": 18
			},
			"0xb2237e7d0ded13a8bd2cc18af812b0b8a956133f": {
				"ticker": "ASPUD",
				"address": "0xb2237E7D0ded13A8Bd2cc18af812B0b8a956133F",
				"name": "CouchPotatoAVAX",
				"decimals": 9
			},
			"0xb147656604217a03fe2c73c4838770df8d9d21b8": {
				"ticker": "BLIZZ",
				"address": "0xB147656604217a03Fe2c73c4838770DF8d9D21B8",
				"name": "Blizzard",
				"decimals": 18
			},
			"0xa4a3861565d38612408d5d9f608dd8ba411257af": {
				"ticker": "SKYLINE",
				"address": "0xA4a3861565d38612408d5d9f608DD8BA411257AF",
				"name": "SKYLINE",
				"decimals": 9
			},
			"0x9adcacd977cce96ebebaebd1bbd7acd7abf170cd": {
				"ticker": "ISLAND",
				"address": "0x9ADcAcD977Cce96eBebaeBd1bBD7aCd7abf170CD",
				"name": "Island Dao",
				"decimals": 18
			},
			"0xd72dd8ec8b0bf259ed93196b295b9195b592d35b": {
				"ticker": "SkyCash",
				"address": "0xD72dd8ec8B0bf259ed93196B295b9195b592d35B",
				"name": "SkyCash",
				"decimals": 8
			},
			"0xdf0486753e0563e170c62053168a72f6f1dcd677": {
				"ticker": "ATM",
				"address": "0xDF0486753e0563E170c62053168A72F6F1DCD677",
				"name": "Atom Protocol",
				"decimals": 18
			},
			"0xc5e9336aceae75eabfed042fb4fc5ff6b1c63d93": {
				"ticker": "NXS",
				"address": "0xc5E9336acEAe75EabFeD042FB4fc5Ff6b1C63d93",
				"name": "NEXUS DAO",
				"decimals": 18
			},
			"0x12282b67a84b2e187edad62f5f61d7a9f2f3de04": {
				"ticker": "ETHPPP",
				"address": "0x12282b67A84B2E187edaD62f5F61D7A9F2f3de04",
				"name": "ETH X3 PRINTER",
				"decimals": 1
			},
			"0xf0e34b7e549880941d79520d3d8063d6ac48e1b9": {
				"ticker": "$PYRA",
				"address": "0xf0e34B7e549880941D79520D3D8063D6aC48e1b9",
				"name": "Pyramide Nodes",
				"decimals": 18
			},
			"0x57ff308fb6a8101638c460795a4aa6a93ddab7c7": {
				"ticker": "MTL",
				"address": "0x57ff308fb6A8101638c460795A4aA6a93ddaB7C7",
				"name": "MetaLand",
				"decimals": 18
			},
			"0xf5a4e66c42cb854432130a0ebb1d4602d220adc9": {
				"ticker": "BOSON",
				"address": "0xF5a4E66C42cb854432130a0EBB1D4602D220ADc9",
				"name": "BosonDAO",
				"decimals": 9
			},
			"0x717249ea7d90c6f1850f70a4064206f8b8b0095c": {
				"ticker": "RedShine",
				"address": "0x717249eA7d90c6f1850f70a4064206f8B8B0095C",
				"name": "RedShine",
				"decimals": 8
			},
			"0x9b82c814558047dc9981841469692cf34db65e02": {
				"ticker": "Nodeon",
				"address": "0x9b82C814558047dc9981841469692cf34DB65E02",
				"name": "Nodeon Protocol",
				"decimals": 18
			},
			"0xfb4b9030daa58458b82eae377b0109e9cd4c806e": {
				"ticker": "BB",
				"address": "0xfb4B9030daA58458b82eae377B0109e9cd4c806E",
				"name": "Big Brain Capital DAO",
				"decimals": 9
			},
			"0x924157b5dbb387a823719916b25256410a4ad470": {
				"ticker": "SLOT",
				"address": "0x924157B5dbB387A823719916B25256410a4Ad470",
				"name": "Snowtomb Lot",
				"decimals": 18
			},
			"0x08b51a43d239069b4af322904ec0727318b54c6a": {
				"ticker": "PLAY",
				"address": "0x08B51A43d239069b4aF322904Ec0727318b54C6A",
				"name": "Hyper Games DAO",
				"decimals": 18
			},
			"0x61d264652c708b0c8d497fa7f6b4cd501e94831a": {
				"ticker": "CHEEMSVX",
				"address": "0x61d264652c708b0c8d497fA7f6B4CD501e94831a",
				"name": "CHEEMSVAX",
				"decimals": 9
			},
			"0x4cf024672ef8f57f493051e1e924dcc714c5633b": {
				"ticker": "AER",
				"address": "0x4cf024672eF8F57F493051e1E924dcC714c5633B",
				"name": "AERO PROTOCOL",
				"decimals": 8
			},
			"0x90158a1598c14fdfb81a8088853c19f4fc28db6a": {
				"ticker": "StellarNodes",
				"address": "0x90158a1598C14FdFB81A8088853C19f4FC28dB6A",
				"name": "StellarNodes",
				"decimals": 18
			},
			"0xfb2293de545a44e154f0ab5596429f68fcf845a7": {
				"ticker": "Silico",
				"address": "0xFb2293dE545a44e154f0ab5596429f68FcF845A7",
				"name": "Silicon DAO",
				"decimals": 18
			},
			"0x6f3e0ec2457f6e82fcbf10e8aecdebcd33453464": {
				"ticker": "MetaNBA",
				"address": "0x6F3E0Ec2457F6E82fcbF10E8aeCDebcD33453464",
				"name": "MetaNBA",
				"decimals": 8
			},
			"0x449ef6cd62dfeb12ace476082dd26b728582b24c": {
				"ticker": "KNIGHT",
				"address": "0x449ef6cD62dFeB12ACe476082Dd26B728582B24c",
				"name": "KNIGHT PROTOCOL",
				"decimals": 1
			},
			"0x9ee4d1818047dab6324932f82fda518bbe38c38d": {
				"ticker": "STAR",
				"address": "0x9eE4D1818047dab6324932f82FDA518BBE38c38D",
				"name": "StarNodes",
				"decimals": 9
			},
			"0xe88575e5366719339963adaef7f49c6f1d087d6b": {
				"ticker": "JHT",
				"address": "0xe88575E5366719339963adAEF7F49C6f1d087D6B",
				"name": "Joe's Harvest Time",
				"decimals": 6
			},
			"0xc19ef3d5fbdfdbd75f8e422b04673fd6dbf1da31": {
				"ticker": "WEBN",
				"address": "0xC19eF3d5fBdfDBD75f8e422b04673Fd6DbF1Da31",
				"name": "WEB3NODE",
				"decimals": 9
			},
			"0xc5768c6b2a73d46fc2e6e71cd6a5502f0153bde7": {
				"ticker": "FUEL",
				"address": "0xc5768C6B2A73d46Fc2e6E71CD6A5502F0153bDe7",
				"name": "FUEL NODES",
				"decimals": 18
			},
			"0x40ede8136e9d18069fae1ff90f3a862d8c91b6c4": {
				"ticker": "22SPRINT",
				"address": "0x40ede8136e9d18069fAe1Ff90F3a862d8c91B6c4",
				"name": "2022 SPELL PRINTER",
				"decimals": 18
			},
			"0xed37394a8f2faa39a7ebf4323a487b29b7837125": {
				"ticker": "BB",
				"address": "0xEd37394A8F2faA39a7EbF4323A487b29b7837125",
				"name": "BingBong",
				"decimals": 18
			},
			"0x777467165e162a239cb575ebe9b6f3ee3c838720": {
				"ticker": "Frog",
				"address": "0x777467165e162a239Cb575EBE9b6F3Ee3C838720",
				"name": "FrogNation",
				"decimals": 18
			},
			"0x7baca7b1b6a3f84d2f913ffaf693530dbfdd9954": {
				"ticker": "DGIV",
				"address": "0x7bACa7B1b6a3f84d2F913Ffaf693530DbFdD9954",
				"name": "Doggiverse",
				"decimals": 18
			},
			"0x18b76171ba0f37fefaabe79c68328057dc4b8e1a": {
				"ticker": "MINT",
				"address": "0x18B76171Ba0F37FefaAbe79C68328057dc4B8e1A",
				"name": "Metaverse Index Token",
				"decimals": 18
			},
			"0x7ee753f1b35c059f869697a12850dc47a69bbf85": {
				"ticker": "Degen",
				"address": "0x7eE753f1B35c059F869697A12850Dc47a69bbf85",
				"name": "Degen Nodes",
				"decimals": 1
			},
			"0xefba0731fb0114ac57b8ed84f313f443590f4c14": {
				"ticker": "GRIM",
				"address": "0xEFbA0731fB0114ac57B8ED84f313F443590F4C14",
				"name": "Grim Money",
				"decimals": 5
			},
			"0xbd530af25c5dfee24808282f792f872a9a11c876": {
				"ticker": "ALICE",
				"address": "0xBD530af25c5DFEe24808282f792f872a9a11c876",
				"name": "Alice Frog",
				"decimals": 18
			},
			"0xab61762f4e527446376375c46b373f0025d8f380": {
				"ticker": "SMRT",
				"address": "0xab61762f4E527446376375C46B373f0025d8f380",
				"name": "Smart Finance",
				"decimals": 1
			},
			"0x161f1e4ffcc701d631cbd2110324df223b6e334e": {
				"ticker": "SEED",
				"address": "0x161f1e4ffCc701D631cbD2110324DF223b6e334e",
				"name": "SEED",
				"decimals": 18
			},
			"0x1a15d6b3b3425e216a8ee057c3f23fe989b721b1": {
				"ticker": "GLACIUS",
				"address": "0x1A15d6b3b3425e216A8Ee057c3f23FE989b721b1",
				"name": "Glacius",
				"decimals": 18
			},
			"0xbace58d805a9d8d150709e98432e8bdb11bbc1e9": {
				"ticker": "ALPHA",
				"address": "0xBacE58d805a9d8D150709e98432e8BDB11BBc1e9",
				"name": "Alpha DAO",
				"decimals": 1
			},
			"0xe61452b2120b73b99646e6470e1a15ce83725d82": {
				"ticker": "RND",
				"address": "0xE61452b2120b73b99646e6470E1A15ce83725d82",
				"name": "Redlight Node District",
				"decimals": 9
			},
			"0x37f1d6b63a5ac2be87a62fc043cdd3bc5c6930c5": {
				"ticker": "WAVE",
				"address": "0x37f1d6b63a5ac2BE87a62fc043cdd3bC5c6930C5",
				"name": "Wave Money",
				"decimals": 5
			},
			"0xe134cd781dbcc22ff18baa464561dd4ead42d743": {
				"ticker": "BOSON",
				"address": "0xe134cD781DBCc22Ff18BAA464561dD4eAd42D743",
				"name": "Boson",
				"decimals": 9
			},
			"0x2115f83922ad1d732bbd3b3a255f861d6b95079f": {
				"ticker": "LAYA",
				"address": "0x2115f83922Ad1D732bbd3b3A255f861d6B95079F",
				"name": "Himalaya Token",
				"decimals": 18
			},
			"0x8f470257abadd2be1b76f90087ff134bbf540753": {
				"ticker": "Myster",
				"address": "0x8f470257ABAdd2BE1b76f90087Ff134BBf540753",
				"name": "Mystery Inu",
				"decimals": 9
			},
			"0x34ff2ce3a5dbf672fab0b04fe7a755299a005547": {
				"ticker": "FLA",
				"address": "0x34ff2Ce3A5DbF672faB0b04Fe7a755299A005547",
				"name": "King Floki Avax",
				"decimals": 9
			},
			"0xdd0e03c8302d574b39e4271f5be8d38a6d2726d6": {
				"ticker": "DMTR",
				"address": "0xdD0e03c8302d574b39e4271F5bE8D38A6D2726D6",
				"name": "Demeter",
				"decimals": 5
			},
			"0x4accbf910ee102bcdd961c7289e0bf9b68e9bafe": {
				"ticker": "REEF",
				"address": "0x4aCcbF910EE102bCdD961c7289E0bf9b68E9BAfE",
				"name": "REEF",
				"decimals": 9
			},
			"0x30f78c55ebbbe9ea346e84be9bc499f76e865fc0": {
				"ticker": "Blizzar",
				"address": "0x30f78C55EbBBE9EA346E84be9BC499F76e865fC0",
				"name": "Blizzard Chain",
				"decimals": 4
			},
			"0x44503ae47174ec0de684c2f460a733a68bf0bd7e": {
				"ticker": "SWING",
				"address": "0x44503Ae47174EC0dE684C2f460A733A68bf0bD7E",
				"name": "Swing DAO",
				"decimals": 18
			},
			"0x1d6e474cea9424c26cc049a2c7cc23771cc5dac8": {
				"ticker": "RisePrint",
				"address": "0x1D6E474cEA9424C26cC049a2C7CC23771cC5DaC8",
				"name": "RisePrint",
				"decimals": 6
			},
			"0x54ca3497de44a4407a6d91678ef75fc8e3015b35": {
				"ticker": "CryptoDream",
				"address": "0x54CA3497dE44a4407A6D91678EF75FC8e3015b35",
				"name": "CryptoDream",
				"decimals": 8
			},
			"0x8ef78560e7aeebc38b1f8912b188824bde9d33e1": {
				"ticker": "DD",
				"address": "0x8EF78560e7AEEBc38b1f8912b188824bdE9D33e1",
				"name": "Desert DAO",
				"decimals": 9
			},
			"0x82f3be8c4f81be0dfe2fcae68ba77469e090ce61": {
				"ticker": "FNX",
				"address": "0x82f3BE8c4F81BE0dfe2fcAe68BA77469E090Ce61",
				"name": "Phoenix Nodes",
				"decimals": 1
			},
			"0xb54ae2a8c3ab3ce7c55e2bf6b207ce2960a214ce": {
				"ticker": "ASND",
				"address": "0xB54ae2a8c3ab3CE7c55E2BF6B207Ce2960A214CE",
				"name": "Ascend Node Club",
				"decimals": 18
			},
			"0xd5b11952c841f978e234f700e12104306c446336": {
				"ticker": "BNBp",
				"address": "0xD5B11952c841f978e234f700e12104306C446336",
				"name": "BNBPrint",
				"decimals": 9
			},
			"0xf0fec91da23fb771e2983e5820e6ea23fcd98bbe": {
				"ticker": "KONG",
				"address": "0xF0feC91dA23FB771E2983E5820E6EA23fCD98BbE",
				"name": "Zilla Finance",
				"decimals": 18
			},
			"0x8b0ff81c0c6d623885d8bd3cd774efaf7572fa19": {
				"ticker": "TOA",
				"address": "0x8B0Ff81C0c6d623885D8Bd3cD774EfaF7572fa19",
				"name": "TigerOfAvax",
				"decimals": 9
			},
			"0xbaaa6b4a1e20d5c7577f9f0613691ed0838d16b8": {
				"ticker": "Cybe",
				"address": "0xBaaA6b4A1E20D5C7577F9F0613691Ed0838d16B8",
				"name": "Cyber Crypto",
				"decimals": 8
			},
			"0x54ba274f786159135ca39273650eb286783046b4": {
				"ticker": "PEBBLEp",
				"address": "0x54bA274f786159135CA39273650eb286783046b4",
				"name": "PEBBLEp",
				"decimals": 9
			},
			"0x7c6d197af514a302e0be9e8caa207697bfcd2bfc": {
				"ticker": "UFO",
				"address": "0x7C6d197Af514a302E0be9e8CAa207697bfCD2Bfc",
				"name": "U.F.O City",
				"decimals": 1
			},
			"0xafb58e2edcea16e9dd4df15ad5c868e59e2e0901": {
				"ticker": "MINDS",
				"address": "0xaFb58e2eDcEa16E9dd4DF15AD5C868e59E2E0901",
				"name": "BiggerMINDS",
				"decimals": 18
			},
			"0xd94a546cf38f42070dcdb0aa789c8324c5c39064": {
				"ticker": "BARS",
				"address": "0xd94A546Cf38f42070DcdB0aa789c8324C5c39064",
				"name": "BARSTOOL FINANCE",
				"decimals": 9
			},
			"0x0dda3844c71976b5d9769b8d367d3238f41b95f4": {
				"ticker": "AVOLID",
				"address": "0x0dDa3844C71976B5d9769b8d367d3238F41B95F4",
				"name": "Avolidly",
				"decimals": 9
			},
			"0x86f1850740ae650855e71fa7d4e3e1bfbe77774f": {
				"ticker": "1000XMAS",
				"address": "0x86f1850740AE650855E71fa7d4e3E1BFbE77774f",
				"name": "1000XMAS",
				"decimals": 9
			},
			"0xd32b4ed603667624d63794944a2963a694d1ba86": {
				"ticker": "LAND",
				"address": "0xd32b4ED603667624d63794944a2963a694d1Ba86",
				"name": "Land Verse",
				"decimals": 9
			},
			"0x23ad734665f47c371a6d853b133d2c1ff12bba2c": {
				"ticker": "ALPHATEST2",
				"address": "0x23aD734665f47c371A6d853b133d2c1ff12Bba2c",
				"name": "ALPHATEST2",
				"decimals": 9
			},
			"0x3ac1182dabf1841cddd2336b198b857fe22bec4c": {
				"ticker": "BGLD",
				"address": "0x3Ac1182DABF1841CdDD2336b198B857fe22beC4C",
				"name": "BridgeGold",
				"decimals": 18
			},
			"0xb8f9b37cc7a110480eb886b35bef35bc1befb7a5": {
				"ticker": "$INUVAX",
				"address": "0xB8F9B37CC7A110480eB886B35beF35bC1befB7a5",
				"name": "Shibainuvax",
				"decimals": 9
			},
			"0x20cf74c2874f4fbadfbc928a0761ecd66b65ac4c": {
				"ticker": "AZTEC",
				"address": "0x20cf74C2874f4FBADFbc928A0761eCD66b65ac4C",
				"name": "AZTEC MONEY",
				"decimals": 18
			},
			"0xd03392cf4f60fab2fbea38b7d2b826c70b0208a0": {
				"ticker": "WOLFI",
				"address": "0xD03392CF4f60FaB2FbEA38B7d2B826C70b0208A0",
				"name": "WOLFI",
				"decimals": 9
			},
			"0x619eaec11bb820e1745f5f31e16cc336675011ff": {
				"ticker": "BRAVE",
				"address": "0x619eAeC11bB820E1745F5F31e16cC336675011fF",
				"name": "Brave Nodes",
				"decimals": 1
			},
			"0x3509f19581afedeff07c53592bc0ca84e4855475": {
				"ticker": "xUSD",
				"address": "0x3509f19581aFEDEff07c53592bc0Ca84e4855475",
				"name": "xDoller Stablecoin",
				"decimals": 18
			},
			"0x89aedc519f976948973b4e2c202571fecf17d0bf": {
				"ticker": "THUNDER",
				"address": "0x89AeDC519f976948973B4e2c202571FECf17d0bF",
				"name": "Thunder Nodes",
				"decimals": 18
			},
			"0x88784dc627f0a1ebdd6b470524719c089d18e55e": {
				"ticker": "VEGETA",
				"address": "0x88784DC627F0a1ebdd6B470524719c089D18E55e",
				"name": "Vegeta Inu",
				"decimals": 9
			},
			"0xa2e772c89f1737dae888ed77e7b4f7b1c4be540f": {
				"ticker": "test24",
				"address": "0xa2e772c89F1737dAE888ED77E7B4F7B1c4bE540F",
				"name": "test24",
				"decimals": 18
			},
			"0xf9a075c9647e91410bf6c402bdf166e1540f67f0": {
				"ticker": "SING",
				"address": "0xF9A075C9647e91410bF6C402bDF166e1540f67F0",
				"name": "Sing Token",
				"decimals": 18
			},
			"0x38f676584b838da837a4d72ecec3aba505d37c26": {
				"ticker": "SGD",
				"address": "0x38F676584B838Da837a4D72eCEC3aba505D37c26",
				"name": "Shiba Galaxy DAO",
				"decimals": 18
			},
			"0x19fcece1cc64ba0ea1387a91fc08278d55130b79": {
				"ticker": "ARG",
				"address": "0x19FCECe1cc64Ba0Ea1387A91FC08278D55130b79",
				"name": "Argos Finance",
				"decimals": 18
			},
			"0x73f3e1384b76f29be96a6b2a49caddfb8533e518": {
				"ticker": "Oasi",
				"address": "0x73f3e1384B76F29BE96A6B2a49CAddFB8533E518",
				"name": "Oasis APP",
				"decimals": 8
			},
			"0xb75f3be3aa419a40c3f467ffa656748ef305539b": {
				"ticker": "POTUSPRINTER",
				"address": "0xb75f3BE3Aa419a40c3F467ffA656748eF305539b",
				"name": "POTUSPRINTER",
				"decimals": 9
			},
			"0xe825dc936779bb9c15a72d69ddb2457e5b489846": {
				"ticker": "BANDS",
				"address": "0xE825dc936779bb9C15A72D69ddb2457E5B489846",
				"name": "Pimps N Hoes Game",
				"decimals": 18
			},
			"0xc4567de354b65f00084cc338b6d04b9ce77c49dc": {
				"ticker": "Playmates",
				"address": "0xc4567de354b65f00084cc338b6D04B9ce77c49DC",
				"name": "Redlight Node District",
				"decimals": 9
			},
			"0x91de0b71603e151916c033eb2e400a2cd07ca279": {
				"ticker": "Vikin",
				"address": "0x91DE0b71603e151916C033EB2E400A2cD07CA279",
				"name": "Viking Nodes",
				"decimals": 18
			},
			"0x4bc2a97bda325c5b4aa3afef0f2b8b055940481b": {
				"ticker": "KingReward",
				"address": "0x4Bc2a97bDA325C5b4AA3AFEF0f2B8B055940481b",
				"name": "KingReward",
				"decimals": 6
			},
			"0x551d9640b33002eecf2a8949e861dab8ab2ddee5": {
				"ticker": "WAVE",
				"address": "0x551D9640B33002EecF2a8949E861Dab8aB2DdEE5",
				"name": "Waves Money",
				"decimals": 1
			},
			"0xb629fac57e7ff5b26b8e711bbda60c0d75a420c2": {
				"ticker": "CHICK",
				"address": "0xB629faC57e7ff5b26b8e711bbda60c0D75A420c2",
				"name": "ChickEarn",
				"decimals": 6
			},
			"0x945318f951fb982618cdcf7a4a77577dbe93a4d6": {
				"ticker": "PristineDAO",
				"address": "0x945318f951FB982618CDCF7A4a77577DBe93a4D6",
				"name": "PristineDAO",
				"decimals": 18
			},
			"0xb34c76318088a29742f4c029353a4bef6f567829": {
				"ticker": "SLIP",
				"address": "0xB34c76318088A29742f4C029353a4BeF6f567829",
				"name": "Slippage",
				"decimals": 9
			},
			"0x565a329cd39d904655aa07363fe27c397bfcd968": {
				"ticker": "WAVE",
				"address": "0x565A329Cd39D904655Aa07363fE27C397BFCd968",
				"name": "Wave Money",
				"decimals": 5
			},
			"0x9c4f88408f9f003fb10f106e7a69989bb4f3452f": {
				"ticker": "BURN",
				"address": "0x9c4f88408f9f003Fb10f106E7A69989bB4f3452f",
				"name": "Burn",
				"decimals": 18
			},
			"0x0f6ff8c521bd003a27df39d6feb35bbc462011c2": {
				"ticker": "LAND",
				"address": "0x0f6Ff8c521Bd003a27dF39d6fEB35bbc462011c2",
				"name": "Land Verse",
				"decimals": 9
			},
			"0x15a5af84ae7e57e48b997204a41af6ceecb81c17": {
				"ticker": "RND",
				"address": "0x15A5aF84ae7E57e48B997204A41Af6CEECB81C17",
				"name": "Redlight Node District",
				"decimals": 9
			},
			"0xcaa5b5195b172f5fc518d305d697b13b90125f83": {
				"ticker": "InfinityNodes",
				"address": "0xCaa5B5195B172F5fc518D305D697b13b90125F83",
				"name": "InfinityNodes",
				"decimals": 9
			},
			"0x1e49e1e9a9e9b648db42009215a276993914fb12": {
				"ticker": "LAF",
				"address": "0x1e49E1E9a9e9B648dB42009215A276993914fb12",
				"name": "LayerFort",
				"decimals": 9
			},
			"0xf2b641121788219fee38dfb3b60709a1ec3ef3fe": {
				"ticker": "RND",
				"address": "0xf2b641121788219FEe38DfB3B60709A1eC3EF3FE",
				"name": "Redlight Node District",
				"decimals": 9
			},
			"0x22869208a24ced29a1150fa214846d45bf5c7b11": {
				"ticker": "DEGEN",
				"address": "0x22869208a24CEd29a1150fA214846d45Bf5C7B11",
				"name": "DegenFi",
				"decimals": 9
			},
			"0xa2006f9872d763caea2b9e526094afe9c4ba6423": {
				"ticker": "KingEarnAvax",
				"address": "0xA2006f9872D763CAEa2b9e526094afE9C4BA6423",
				"name": "KingEarnAvax",
				"decimals": 9
			},
			"0x0eae20b784a63e0c4b4ae46bab49ac993ef121c8": {
				"ticker": "SHINAVAX",
				"address": "0x0eaE20b784A63E0c4B4AE46bAB49ac993ef121C8",
				"name": "Shinavax",
				"decimals": 18
			},
			"0xda1a6d0b1d88d69ab7e77a1bed4ad53575158667": {
				"ticker": "INF",
				"address": "0xDa1a6D0b1d88D69Ab7e77a1BED4Ad53575158667",
				"name": "Infinity Verse",
				"decimals": 18
			},
			"0x3198eca691cfae98734baf94bfbd6f9627d0ac24": {
				"ticker": "THUNDER",
				"address": "0x3198eCA691cfaE98734baf94bFBd6F9627d0ac24",
				"name": "Thunder Nodes",
				"decimals": 18
			},
			"0x1dfce16b56ac064da3a4702277d14be72d3dfdfd": {
				"ticker": "ACF",
				"address": "0x1DFCE16B56aC064dA3a4702277D14BE72d3DFDFD",
				"name": "Alice Frog",
				"decimals": 18
			},
			"0xe253ead75304375a20c9254e79024a63eb36edab": {
				"ticker": "Playmates",
				"address": "0xE253EaD75304375A20C9254E79024a63EB36EDAB",
				"name": "Redlight Node District",
				"decimals": 9
			},
			"0x4e6647cc478d0c1a5c7963d213e28eede6c4e95f": {
				"ticker": "PRINCE",
				"address": "0x4E6647cC478D0c1a5c7963D213E28EEDe6c4E95f",
				"name": "SonOfMidas",
				"decimals": 9
			},
			"0xebcf0d14709105f4cf5de388df78da69801f7d74": {
				"ticker": "GLAC",
				"address": "0xebcf0D14709105F4cf5de388df78DA69801F7D74",
				"name": "GlacierLaunch",
				"decimals": 18
			},
			"0xb2c41b3b50f3d4509efa62cb8603165196857c11": {
				"ticker": "SWING",
				"address": "0xb2C41B3b50f3D4509efa62CB8603165196857C11",
				"name": "SwingDAO",
				"decimals": 9
			},
			"0xe937357a89caa92f5a983553244297c2d47362c9": {
				"ticker": "JAWS",
				"address": "0xe937357a89cAa92f5A983553244297C2d47362c9",
				"name": "AVASHARK",
				"decimals": 18
			},
			"0x6f86a1e738aa443149f85e1989deadc09e035bb5": {
				"ticker": "FNX",
				"address": "0x6f86a1E738AA443149F85e1989DeADC09e035BB5",
				"name": "Phoenix Node",
				"decimals": 18
			},
			"0xeba4f898dffd39462338cfae16ad22687c1838ce": {
				"ticker": "DB",
				"address": "0xEba4f898dFFd39462338cFAE16ad22687C1838Ce",
				"name": "Diamond Bank Dao",
				"decimals": 5
			},
			"0x93fbd145920a93918b03a0a5ba03b8075ccb6186": {
				"ticker": "donttellmewhattodounlessyouarenaked",
				"address": "0x93fBD145920a93918B03a0a5BA03B8075cCB6186",
				"name": "donttellmewhattodounlessyouarenaked",
				"decimals": 18
			},
			"0x627cd035336380a5e7a242fa0665b02c2cd636af": {
				"ticker": "STAR",
				"address": "0x627Cd035336380A5e7a242FA0665B02c2cD636aF",
				"name": "STARSHIP NODES",
				"decimals": 18
			},
			"0xc1ecd59553e481328f5e48fc127a027db714ae2d": {
				"ticker": "FOM",
				"address": "0xC1ecd59553e481328f5E48fC127a027dB714AE2D",
				"name": "FOMO Protocol",
				"decimals": 8
			},
			"0x6fefd97f328342a8a840546a55fdcfee7542f9a8": {
				"ticker": "agEUR",
				"address": "0x6feFd97F328342a8A840546A55FDcfEe7542F9A8",
				"name": "agEUR",
				"decimals": 18
			},
			"0xac2b23217cac4f6f5b538220cddf7c8c516bae95": {
				"ticker": "RAV",
				"address": "0xaC2B23217CAC4f6F5b538220cDdF7C8C516BAe95",
				"name": "Rave Finance",
				"decimals": 9
			},
			"0x6514bd95ff9ef98d4443e4ce201be8e5f6dc5586": {
				"ticker": "BTZ",
				"address": "0x6514Bd95fF9Ef98d4443e4ce201Be8E5F6dc5586",
				"name": " KOI DAO",
				"decimals": 4
			},
			"0x26a79f89575c77a457e05018039621d7effbe1af": {
				"ticker": "SFT",
				"address": "0x26a79F89575c77A457e05018039621D7EFfBE1aF",
				"name": "Bitanza",
				"decimals": 18
			},
			"0x55bd90b4030ddf92d6c306fe1bee05042b9499c9": {
				"ticker": "OXY",
				"address": "0x55Bd90b4030Ddf92D6C306FE1Bee05042B9499c9",
				"name": "Snow Factory Token",
				"decimals": 9
			},
			"0x3d8f74620857dd8ed6d0da02ceb13fd0ed8ba678": {
				"ticker": "ONX",
				"address": "0x3D8f74620857dd8ED6D0dA02ceB13fd0Ed8Ba678",
				"name": "Oxy-Fi",
				"decimals": 18
			},
			"0x747be1bf7f8db7549facb799ed27f5fbc2e48ba1": {
				"ticker": "SWING",
				"address": "0x747BE1bf7F8DB7549facb799eD27F5fBc2E48ba1",
				"name": "OnX.finance",
				"decimals": 1
			},
			"0xbd55da0ba8ee32d49213b7a844fb21fe9c26ffe1": {
				"ticker": "PRMD",
				"address": "0xbd55DA0BA8ee32D49213B7a844Fb21fE9c26fFe1",
				"name": "Swing Dao",
				"decimals": 18
			},
			"0x83099ba97efeca136a3c84efd2f21f6ae9f5fb54": {
				"ticker": "Bi",
				"address": "0x83099BA97EfeCA136a3C84EFd2F21f6AE9f5Fb54",
				"name": "Pyramid Money",
				"decimals": 8
			},
			"0x15eb7057f7bbae5791fdfdd66dda2dd988257c7a": {
				"ticker": "NOVATO",
				"address": "0x15Eb7057f7bbae5791fDFDd66Dda2DD988257C7a",
				"name": "Bit Mafia",
				"decimals": 18
			},
			"0x9d1c3e5bf3bb08f7aef8f22b010bf12e685572bd": {
				"ticker": "DEGEN",
				"address": "0x9D1C3E5bf3BB08f7aEf8f22b010BF12e685572bD",
				"name": "Novato Staking Token",
				"decimals": 18
			},
			"0x4c75fd176362189ca555437c23c8468255584e91": {
				"ticker": "THOR",
				"address": "0x4C75fD176362189ca555437C23C8468255584E91",
				"name": "DegenFi",
				"decimals": 18
			},
			"0x6560051ead959645eeec16cbbf81fecc384ab95a": {
				"ticker": "AVAXN",
				"address": "0x6560051eAD959645eEec16cbbF81fecC384Ab95a",
				"name": "Thor Nodes",
				"decimals": 18
			},
			"0x3e5a9f09923936427ad6e487b24e23a862fcf6b7": {
				"ticker": "OTO",
				"address": "0x3e5a9F09923936427aD6e487b24E23a862FCf6b7",
				"name": "AVAXNation",
				"decimals": 5
			},
			"0xf5a7df17213ed435078cb0b76960cd5cbaa54c92": {
				"ticker": "SOR",
				"address": "0xF5A7df17213ed435078Cb0b76960cD5cBAa54C92",
				"name": "OTO Protocol",
				"decimals": 5
			},
			"0x5bcde17c50652ad6148c970f10bf06647137a7e4": {
				"ticker": "LOA",
				"address": "0x5BcDE17c50652aD6148C970F10Bf06647137A7E4",
				"name": "Sorion Money",
				"decimals": 18
			},
			"0x97cf77be99ead6339300c52b3720bfc3578354f5": {
				"ticker": "KAIRU",
				"address": "0x97CF77BE99EAd6339300C52b3720bfc3578354f5",
				"name": "LOAlexandria",
				"decimals": 18
			},
			"0x7d00a783a8328a710e465a86f27627bf5eef17ae": {
				"ticker": "VOLCA",
				"address": "0x7D00A783A8328a710e465a86f27627bf5eEf17aE",
				"name": "KAIRU TOKEN",
				"decimals": 18
			},
			"0xa5e3c7cbc615b16939c79fe9fe1a40102c908dec": {
				"ticker": "BURGER",
				"address": "0xA5e3c7cBc615B16939C79fe9Fe1a40102C908dEC",
				"name": "VOLCANO CITY",
				"decimals": 8
			},
			"0x3c0b85f23de35bc92820892ea9fe772fd02cec76": {
				"ticker": "OXG",
				"address": "0x3C0b85f23de35bC92820892ea9FE772fd02Cec76",
				"name": "FLOKI BURGER",
				"decimals": 18
			},
			"0xd5b1a5019ec4110a10f20075cb01d668a247d6be": {
				"ticker": "LCR",
				"address": "0xd5B1a5019EC4110A10F20075cB01d668A247d6BE",
				"name": "Oxygen Finance",
				"decimals": 9
			},
			"0x24b1935cde3d1e4a3c1eb81f9f6b5f3c5747671a": {
				"ticker": "SCAT",
				"address": "0x24B1935cde3d1E4A3C1eB81F9f6b5f3C5747671A",
				"name": "LowCapRocket",
				"decimals": 18
			},
			"0x65a8d0d8a4dcd1652c3057a318baeff8c61d14a8": {
				"ticker": "MN",
				"address": "0x65a8D0D8A4dCD1652C3057A318BAefF8c61D14a8",
				"name": "SnowCatDAO",
				"decimals": 9
			},
			"0xfefe24a60b71a37ce9f09e43ee3326a317a72fa1": {
				"ticker": "AAVAX",
				"address": "0xFEFe24A60B71a37Ce9f09E43Ee3326a317a72FA1",
				"name": "MaybeSomething",
				"decimals": 9
			},
			"0xe6780cc551df332ce7a05a6f59fb2a26e4b0f9ad": {
				"ticker": "JEWELBAGZ",
				"address": "0xE6780CC551DF332ce7a05A6F59fB2A26e4b0f9aD",
				"name": "AAVAX",
				"decimals": 9
			},
			"0x53d46d0ce9668cbc421d5114fa7b4963939d767f": {
				"ticker": "NRGY",
				"address": "0x53D46d0ce9668Cbc421D5114Fa7b4963939D767F",
				"name": "Bagz O Jewels",
				"decimals": 18
			},
			"0x3200671c9dea39c2ace5b3d695b0d333d897a3e0": {
				"ticker": "SHADOW",
				"address": "0x3200671C9dea39c2Ace5B3d695b0D333D897A3E0",
				"name": "Wind Finance",
				"decimals": 8
			},
			"0x2ae20e5e0bd3ae63300b1672fe4baf59aa9314ce": {
				"ticker": "PANTHER",
				"address": "0x2AE20e5E0Bd3AE63300B1672FE4bAf59aA9314ce",
				"name": "ShadowDAO",
				"decimals": 9
			},
			"0x5ea686bd59c26ab0a67a3ba608d9ac839b68a0b3": {
				"ticker": "A-Team",
				"address": "0x5Ea686bd59c26aB0a67a3Ba608D9aC839b68a0b3",
				"name": "PantherSwap DAO",
				"decimals": 6
			},
			"0xae21d31a6494829a9e4b2b291f4984aae8121757": {
				"ticker": "CREAM",
				"address": "0xAE21d31a6494829a9E4B2B291F4984AAE8121757",
				"name": "A-Team",
				"decimals": 18
			},
			"0xd262633ef91c5a74e9f67b7fe98abf73a5abdcd3": {
				"ticker": "Ic",
				"address": "0xd262633eF91c5A74E9f67b7Fe98aBF73a5abDCD3",
				"name": "IceCream Finance",
				"decimals": 8
			},
			"0xa87f270ecc2bfede980a1d293111b1a8718344e0": {
				"ticker": "AVAXBABY",
				"address": "0xA87f270ECC2bfEdE980a1d293111b1a8718344E0",
				"name": "Icy Tower",
				"decimals": 18
			},
			"0x35238147a5e59d14e2bf1b09d4c507521aafe1eb": {
				"ticker": "TRIP",
				"address": "0x35238147a5e59d14E2bF1B09d4C507521aafE1eB",
				"name": "Avax Baby @BabyAvaxStealth",
				"decimals": 18
			},
			"0x9eb4a27180b5b62c06ad94b08dd4ea8c61918bc0": {
				"ticker": "LUCK",
				"address": "0x9eB4a27180B5B62C06ad94b08Dd4eA8c61918BC0",
				"name": "Psilocybin DAO",
				"decimals": 1
			},
			"0xaaa927c4b2c8c6b3067b2c1871dbf11ac0f493a4": {
				"ticker": "Peepo.Q",
				"address": "0xAAa927C4B2c8c6B3067B2C1871DBF11ac0F493a4",
				"name": "Lucky Nodes",
				"decimals": 8
			},
			"0x04c621fb1d553a9e45d90f5c4c369cb1bd4b701b": {
				"ticker": "HOST",
				"address": "0x04C621Fb1d553A9e45D90f5c4C369Cb1bD4b701B",
				"name": "PeepoQuests t.me/PeepoQuests",
				"decimals": 18
			},
			"0x155f794b56353533e0afbf76e1b1fc57dfad5bd7": {
				"ticker": "CSHARE",
				"address": "0x155f794b56353533E0AfBF76e1B1FC57DFAd5Bd7",
				"name": "HOST",
				"decimals": 18
			},
			"0xcf87e25ed6b35c6751922a2b2dd75c0480aa73eb": {
				"ticker": "TICK",
				"address": "0xCF87e25Ed6b35C6751922a2b2Dd75C0480AA73EB",
				"name": "IceCream Shares",
				"decimals": 18
			},
			"0x6da767394f079c1524b3e797d2110c23c3ecf322": {
				"ticker": "SNOWY",
				"address": "0x6da767394f079c1524B3e797D2110C23c3ecf322",
				"name": "Clock Nodes",
				"decimals": 18
			},
			"0x6447c868b938494b10e60df8d45bb0b55f4ccf90": {
				"ticker": "ATM",
				"address": "0x6447c868b938494B10e60Df8d45bB0b55F4ccf90",
				"name": "Fantastic Protocol SNOWY Token",
				"decimals": 18
			},
			"0x348aa98693149f06f6b7c118d9a219a55059e3b8": {
				"ticker": "TENSION",
				"address": "0x348Aa98693149f06F6B7c118d9a219A55059E3B8",
				"name": "DaiPrinter",
				"decimals": 8
			},
			"0x8468c950371db222c73e55a70b8ce4da7ea1d7e1": {
				"ticker": "MP",
				"address": "0x8468c950371Db222c73E55A70B8cE4dA7Ea1D7E1",
				"name": "TENSION",
				"decimals": 18
			},
			"0xcbb6bab07251fa24a40bffa0744931e93dfee4a3": {
				"ticker": "cometnode",
				"address": "0xCBB6bAB07251fa24a40bfFA0744931e93DfeE4A3",
				"name": "MIMpayer",
				"decimals": 9
			},
			"0xf7fefb13bd3939d003dd3557fdadedc8ad9ea016": {
				"ticker": "INF",
				"address": "0xf7FEfB13bD3939d003Dd3557fDAdedc8ad9Ea016",
				"name": "cometnodes ",
				"decimals": 5
			},
			"0xd055aa84e7109c8585e36c0af1c411b78269916c": {
				"ticker": "SEA",
				"address": "0xd055aa84E7109c8585e36c0aF1C411b78269916C",
				"name": "Infinity Verse",
				"decimals": 18
			},
			"0xccd333cb3d27d88d29780461f7dcd2c9e593d0f6": {
				"ticker": "Whal",
				"address": "0xcCd333cb3d27d88d29780461F7DCD2C9e593d0f6",
				"name": "DeepSea DAO",
				"decimals": 18
			},
			"0xb9ddd5064b04a71d7a2481fb9051946cb7602bf8": {
				"ticker": "0X",
				"address": "0xB9dDD5064b04a71d7a2481fb9051946Cb7602Bf8",
				"name": "Whale Nodes",
				"decimals": 18
			},
			"0xe891423307f53b843ab985cda44cccca2527dcfb": {
				"ticker": "WOLF",
				"address": "0xe891423307F53B843AB985CdA44ccCCa2527dCFB",
				"name": "0xDAO.fi",
				"decimals": 18
			},
			"0xc074d811e5bbe40bbc0af0ee69548e302bb84ce2": {
				"ticker": "KATA",
				"address": "0xc074D811e5BBe40BbC0AF0EE69548e302bB84ce2",
				"name": "Grey Wolf DAO",
				"decimals": 18
			},
			"0xd1aae539432ba380fe0bd53e805a6a371496f64e": {
				"ticker": "TOMB",
				"address": "0xd1Aae539432ba380fe0BD53E805a6a371496f64e",
				"name": "KatanaDAO",
				"decimals": 18
			},
			"0x856db9fc59923b7a9ee36143b42a642985d95c14": {
				"ticker": "BSHARE",
				"address": "0x856Db9fC59923B7a9Ee36143B42A642985d95c14",
				"name": "TOMB",
				"decimals": 18
			},
			"0x8805691931762f613b5de616756f6115ad79a388": {
				"ticker": "PLAYMATES",
				"address": "0x8805691931762F613B5de616756F6115Ad79A388",
				"name": "Bunny Shares",
				"decimals": 1
			},
			"0x9c6b070cb6cac2864f30ebeec7f6eee7162020e8": {
				"ticker": "PLAGUE",
				"address": "0x9c6b070cb6CAC2864F30eBEEc7f6Eee7162020e8",
				"name": "Redlight Node District",
				"decimals": 18
			},
			"0xe7829e748f6b4c2c7eaf86a5f864e6d9e2f896d6": {
				"ticker": "QTM",
				"address": "0xe7829E748F6b4c2c7Eaf86a5f864e6d9E2f896D6",
				"name": "Avax Plague",
				"decimals": 18
			},
			"0x10f6f2b97f3ab29583d9d38babf2994df7220c21": {
				"ticker": "TEDDY",
				"address": "0x10f6f2b97F3aB29583D9D38BaBF2994dF7220C21",
				"name": "QUANTUM VERSE",
				"decimals": 18
			},
			"0x8fb656730cd975764c6aea35a7d13d671470689c": {
				"ticker": "WDOG",
				"address": "0x8Fb656730cD975764C6aea35a7d13d671470689C",
				"name": "TeddyDoge",
				"decimals": 18
			},
			"0xe119495efe7e6bafdca70095251c6f04d042e7ea": {
				"ticker": "LEDGER",
				"address": "0xe119495Efe7e6bAfdcA70095251c6f04d042E7ea",
				"name": "Winter Dog",
				"decimals": 8
			},
			"0x88fb57ff965be11793dc538b7afdcc996d14296a": {
				"ticker": "MILK",
				"address": "0x88FB57fF965Be11793dc538b7AFDCC996d14296a",
				"name": "Ledger Coin",
				"decimals": 5
			},
			"0xa2ada8dc48952bf315f46e74da0a22f425f05ed2": {
				"ticker": "LUCK",
				"address": "0xA2ada8dC48952Bf315F46E74dA0a22f425F05Ed2",
				"name": "MILK",
				"decimals": 18
			},
			"0x491eb76c0eaa22188ac3a158aacf8afe80255b14": {
				"ticker": "Lamb",
				"address": "0x491EB76c0eAA22188aC3A158aACF8afE80255b14",
				"name": "Lucky Nodes",
				"decimals": 2
			},
			"0x3bfeb8a18e3b75a8138bec2332134658e6b5a11a": {
				"ticker": "ALOHA",
				"address": "0x3BFeb8A18e3B75a8138bec2332134658E6B5A11a",
				"name": "Lambo Inu",
				"decimals": 8
			},
			"0x6c4c85dcf7a7e0f26f8053ce64c53f785a137b41": {
				"ticker": "DECENTRA",
				"address": "0x6C4c85dCf7A7e0f26f8053CE64C53f785A137b41",
				"name": "ALOHA",
				"decimals": 9
			},
			"0xeb9b4c8c38753b2c78a734327c9505d8999b7aac": {
				"ticker": "DEGEN",
				"address": "0xeb9b4C8C38753B2c78A734327c9505d8999b7AAC",
				"name": "DegenFi",
				"decimals": 9
			},
			"0x46c906da466210a61f72ba11c130949db067a7ca": {
				"ticker": "RUGN",
				"address": "0x46c906DA466210a61F72Ba11c130949DB067a7cA",
				"name": "RugNodes",
				"decimals": 1
			},
			"0xe7b171811412ace0a66e91c43c385dbe0b9bf0a9": {
				"ticker": "TEST",
				"address": "0xE7B171811412aCe0A66E91C43C385Dbe0B9bf0A9",
				"name": "TEST",
				"decimals": 5
			},
			"0x27222834e8ca04557fce6b39e08f2a71d730d409": {
				"ticker": "HACKER",
				"address": "0x27222834E8Ca04557FcE6b39e08F2A71D730D409",
				"name": "Hacked",
				"decimals": 8
			},
			"0x10b9c0beeeb4f3c75ab4b84718b85e106c24711a": {
				"ticker": "BLIGHT",
				"address": "0x10B9c0BeEEb4f3C75ab4B84718B85E106c24711A",
				"name": "BLIGHT",
				"decimals": 8
			},
			"0x3589126532527718e2bf90b82e0c73fdb824b93c": {
				"ticker": "CORGI",
				"address": "0x3589126532527718E2bf90b82E0c73FDB824b93C",
				"name": "CORGI",
				"decimals": 18
			},
			"0x11856ce124f3d522820b231e90d649de8bbbdf54": {
				"ticker": "USDCq",
				"address": "0x11856ce124F3d522820B231e90D649De8bBBDF54",
				"name": "USDCQUIRI",
				"decimals": 9
			},
			"0x4a95695e3830b3b7749f2c3ab300cce6575696bf": {
				"ticker": "Infinit",
				"address": "0x4a95695e3830b3B7749f2C3ab300cCE6575696Bf",
				"name": "Infinity Rocket ",
				"decimals": 8
			},
			"0xc4ad51fc93ce632f6d3f45a8aef60f1c0ef7f3be": {
				"ticker": "MCH",
				"address": "0xc4AD51Fc93CE632F6D3F45A8aeF60F1c0ef7F3BE",
				"name": "Multi Chain Holdings",
				"decimals": 9
			},
			"0xc7d77af7971697f62dd9999296d10400bcb4eae3": {
				"ticker": "TIARA",
				"address": "0xc7d77AF7971697F62dD9999296d10400BcB4EAE3",
				"name": "MariGold",
				"decimals": 6
			},
			"0x7a3a62718cbfee4cfb3c84cf5bc7b1a4607e9429": {
				"ticker": "SWEETS",
				"address": "0x7A3a62718CBFee4CFB3C84Cf5BC7B1a4607e9429",
				"name": "Sweetie",
				"decimals": 18
			},
			"0xaccb8b837e7ec3dd0df3ae917abf06f767bbbc6a": {
				"ticker": "UNREAL",
				"address": "0xACCB8B837e7EC3Dd0dF3Ae917abf06F767BBbc6a",
				"name": "UNREAL",
				"decimals": 8
			},
			"0xc03aee0c6a78461bb9fe93e5f7d453f83c21bcb9": {
				"ticker": "DRKSA",
				"address": "0xc03aeE0C6a78461Bb9Fe93E5F7D453F83c21BCB9",
				"name": "DarkSea DEFI",
				"decimals": 1
			},
			"0x325a9cd1e40b8d33bbc4389d1fa0c7281c339953": {
				"ticker": "AsSafeAsItGets",
				"address": "0x325a9cd1e40b8D33Bbc4389D1fA0C7281c339953",
				"name": "ASAIG",
				"decimals": 18
			},
			"0x1f6a6e78b7451b65bc58d88c0ecd4af04513858a": {
				"ticker": "Ope",
				"address": "0x1F6a6E78b7451B65Bc58d88C0ECd4aF04513858A",
				"name": "Open Platform",
				"decimals": 9
			},
			"0x947ca8f9f1d13bc002fb4a423e652631d62d6ee2": {
				"ticker": "ZEUS",
				"address": "0x947Ca8F9F1D13bc002Fb4A423e652631D62d6eE2",
				"name": "ZEUS NODE FINANCE",
				"decimals": 6
			},
			"0xd73c95684f9841c4bc22358447456e5563eef50b": {
				"ticker": "APEN",
				"address": "0xd73C95684f9841c4BC22358447456e5563EeF50B",
				"name": "ApeNodes Finance",
				"decimals": 9
			},
			"0xe211bce07706fe00ea1b5288f86b96f46c024073": {
				"ticker": "SYN",
				"address": "0xE211bce07706FE00ea1b5288f86B96f46c024073",
				"name": "Synergy DAO",
				"decimals": 18
			},
			"0x23745165a5af193087d3b1088213788d8c2afb31": {
				"ticker": "VHUE",
				"address": "0x23745165a5Af193087d3b1088213788d8c2afB31",
				"name": "Vivihue Token",
				"decimals": 6
			},
			"0x5dfdb4016738b57d3f4f00d5f705e04dade65563": {
				"ticker": "ATM",
				"address": "0x5DFdb4016738b57d3f4F00d5f705e04DaDE65563",
				"name": "ATOM PROTOCOL",
				"decimals": 8
			},
			"0x4d13de15d3520ff2b37c04b79a50bf6d6dbca5cb": {
				"ticker": "AliceFrog",
				"address": "0x4d13DE15d3520ff2b37C04b79A50bf6D6dbcA5cb",
				"name": "Alice Frog AVAX",
				"decimals": 18
			},
			"0x11f583f03fbb7406993384b36ffadeee6554744a": {
				"ticker": "LAVA",
				"address": "0x11f583f03fbb7406993384b36FFaDeee6554744A",
				"name": "Lava Financial",
				"decimals": 18
			},
			"0x3497b57977b527927dda33b56c15456f38bb8b52": {
				"ticker": "OCCUPY",
				"address": "0x3497B57977b527927DDA33b56C15456f38BB8b52",
				"name": "Occupy Finance",
				"decimals": 9
			},
			"0x64b68e7be63db5ad46db2538b3531619dfec9f09": {
				"ticker": "BANANA",
				"address": "0x64B68e7Be63Db5ad46Db2538B3531619dfec9f09",
				"name": "Retro Monkey Land",
				"decimals": 8
			},
			"0x99c92a786d83c2c1654719d9213f4973f7530d70": {
				"ticker": "MMD",
				"address": "0x99c92a786d83c2c1654719D9213F4973f7530d70",
				"name": "Moon Mission DAO",
				"decimals": 18
			},
			"0x3592b6c3c6f5ff5fe87fa5c63b2b98fa3a6b005f": {
				"ticker": "ATAMA",
				"address": "0x3592b6c3C6f5FF5FE87fA5C63b2b98Fa3a6B005F",
				"name": "Avatama",
				"decimals": 9
			},
			"0x064c95b206fbb2021f3e0647de9015298f4e7edc": {
				"ticker": "RING",
				"address": "0x064c95B206FbB2021f3e0647DE9015298F4e7EDC",
				"name": "RING Financial",
				"decimals": 1
			},
			"0xfa9be12ea53cd7172075c84bbb9f5e023f36d6d5": {
				"ticker": "ASHIB",
				"address": "0xfA9be12eA53cD7172075C84BbB9f5e023f36d6d5",
				"name": "AvaxShiba",
				"decimals": 9
			},
			"0xc2bab98c2bb033e5f306b29c7d129f2ca8341d0e": {
				"ticker": "SpaceDoge",
				"address": "0xc2bAb98C2bb033e5F306b29c7D129F2CA8341D0e",
				"name": "SpaceDoge",
				"decimals": 18
			},
			"0x96652753b50071948c6449c001bb4cccc8ea921d": {
				"ticker": "Paper",
				"address": "0x96652753b50071948c6449C001bb4cCcC8EA921d",
				"name": "Printer Financial",
				"decimals": 18
			},
			"0x8b5f57888ff266a793152de562a9bcd97b380621": {
				"ticker": "WTR",
				"address": "0x8b5f57888ff266A793152De562a9bcd97B380621",
				"name": "WaterDAO ",
				"decimals": 18
			},
			"0x2f64360895cabc137eabe1eb382b371674d45989": {
				"ticker": "COME",
				"address": "0x2f64360895CAbC137EAbe1eB382b371674d45989",
				"name": "COMET ",
				"decimals": 9
			},
			"0x5054f4dfcf35a78f2994d2d3998af2d2eaeee712": {
				"ticker": "VOLCA",
				"address": "0x5054f4DfCF35A78f2994d2D3998aF2D2eaeEE712",
				"name": "VolcaNodes",
				"decimals": 1
			},
			"0xa39d199c97eaa2e5e5bca7f682285894b535ac85": {
				"ticker": "Chic",
				"address": "0xA39D199c97EAa2e5E5BcA7F682285894B535ac85",
				"name": "Chick Earn",
				"decimals": 18
			},
			"0x9efd7a80e87f305fdaf1ce925b168774a939e4f2": {
				"ticker": "$SIA",
				"address": "0x9EFD7a80e87F305fdaF1Ce925B168774a939e4f2",
				"name": "Saiyan Inu Avax",
				"decimals": 9
			},
			"0x71e6c672fb36e548dc04ffe4e5183df655956fe4": {
				"ticker": "IDLE",
				"address": "0x71E6c672Fb36E548DC04fFe4E5183df655956fE4",
				"name": "Burgr Idle",
				"decimals": 9
			},
			"0xfa3527352a7737ba2d47128e3b29c77d794e793e": {
				"ticker": "PTCLP",
				"address": "0xFa3527352a7737BA2D47128e3b29C77D794e793E",
				"name": "PTCL PRINTER",
				"decimals": 6
			},
			"0x90b8df1919d28f05720bcc9e3c7c8c2623e73f04": {
				"ticker": "FCC",
				"address": "0x90B8dF1919d28f05720BCc9e3c7C8C2623E73F04",
				"name": "FoxCub Coin",
				"decimals": 18
			},
			"0x394fbef9ae8697ba875acfa3c8fe9fca405402c3": {
				"ticker": "HUSKI",
				"address": "0x394fbEF9aE8697BA875ACfA3c8Fe9FCA405402c3",
				"name": "Husky Inu",
				"decimals": 18
			},
			"0x05272dfb1cea3b64673a9a2596b0d9b9f5a32076": {
				"ticker": "Twitch",
				"address": "0x05272DFB1Cea3b64673A9a2596b0d9B9f5a32076",
				"name": "TWITCH NODES",
				"decimals": 9
			},
			"0x5258d72d6d5c2c032fbe82b25401c98d91e61f22": {
				"ticker": "Thu",
				"address": "0x5258D72D6d5C2C032fbe82b25401c98D91e61F22",
				"name": "Thug Doge AVAX",
				"decimals": 18
			},
			"0xe2d3c19bc574dce01c65c522e3217b5e438930dc": {
				"ticker": "Grimace",
				"address": "0xE2D3C19BC574dcE01c65c522E3217B5E438930DC",
				"name": "Grimace Coin",
				"decimals": 18
			},
			"0xf7a6c976a0469ebf8cbd96e950c76ebda5a19804": {
				"ticker": "META",
				"address": "0xF7A6C976A0469EBf8CBd96E950c76EBDA5A19804",
				"name": "META FLOKI",
				"decimals": 8
			},
			"0x6ab6b1ae0a42b42633eb3a0baba1094aa11c1006": {
				"ticker": "Banana",
				"address": "0x6Ab6b1AE0a42b42633eb3A0BAba1094AA11c1006",
				"name": "Gorilla Nodes",
				"decimals": 18
			},
			"0xa81ed722d45b1c011c01c3250eaacf7107e1a87f": {
				"ticker": "NickelDAO",
				"address": "0xa81eD722d45b1C011c01C3250eaacf7107E1a87f",
				"name": "NickelDAO",
				"decimals": 18
			},
			"0xebb5603c8407f133e60c296f6524c593a5282f65": {
				"ticker": "Huliusyoung",
				"address": "0xebb5603C8407f133e60c296f6524C593A5282F65",
				"name": "Hulius Young",
				"decimals": 9
			},
			"0xa4b250df20bb271f799d1f1268abdb342b1cd32f": {
				"ticker": "HUNT",
				"address": "0xA4B250df20BB271f799D1f1268AbDb342b1Cd32F",
				"name": "Cross Chain Capital",
				"decimals": 18
			},
			"0x030d612e4b09ae1712c7b2afe47690aa66818069": {
				"ticker": "Utimate",
				"address": "0x030d612E4b09aE1712C7b2AFe47690Aa66818069",
				"name": "Utimate Printer",
				"decimals": 18
			},
			"0x72e9bf2bfbade2f1a52310d3b97f0febdf18ea41": {
				"ticker": "STAR",
				"address": "0x72e9BF2BfBadE2f1a52310d3b97f0fEBdf18EA41",
				"name": "Starnodes",
				"decimals": 9
			},
			"0x53901014fb756aa0cadba82252dd62f189113765": {
				"ticker": "COMET",
				"address": "0x53901014Fb756AA0cAdba82252Dd62F189113765",
				"name": "Comet Nodes",
				"decimals": 18
			},
			"0x76a3f4fcf29e66bd8ad08872c9869abe5aec9575": {
				"ticker": "NIT",
				"address": "0x76A3F4FcF29E66bd8AD08872c9869AbE5AEc9575",
				"name": "Node Investment Trust",
				"decimals": 18
			},
			"0x83624d945ba38a81ebd6de0496fa4ae8b955930a": {
				"ticker": "ARAK",
				"address": "0x83624d945bA38a81ebD6DE0496fA4AE8B955930a",
				"name": "Araknode",
				"decimals": 18
			},
			"0xc6798ca8276931252026f742dd0e4513745fae81": {
				"ticker": "CYBER",
				"address": "0xC6798cA8276931252026f742dd0E4513745Fae81",
				"name": "CyberNode",
				"decimals": 18
			},
			"0xd2edbadd56d074dd134c5e820e7970f20da1c201": {
				"ticker": "Predato",
				"address": "0xD2EdBadd56D074dD134c5e820e7970F20Da1c201",
				"name": "Predator Protocol",
				"decimals": 8
			},
			"0xe8dc8a807941916de0665a7e34329e184ecfe8dc": {
				"ticker": "$FKA",
				"address": "0xe8dC8A807941916DE0665a7e34329E184ECfe8dc",
				"name": "Floki King Avax",
				"decimals": 9
			},
			"0xd9c173e526a992b397b8937966c5bec0df5185a4": {
				"ticker": "USDCp",
				"address": "0xD9C173e526a992B397B8937966C5bec0dF5185A4",
				"name": "USDCPrinter",
				"decimals": 18
			},
			"0xe3080244fff5be940fea8dda3194157c97f302da": {
				"ticker": "DD",
				"address": "0xe3080244FfF5Be940Fea8dda3194157c97F302dA",
				"name": "Dating Doge",
				"decimals": 8
			},
			"0x97b864ec76c6fff9c5a72eb5d49f06bef6b2a736": {
				"ticker": "ROCKEFELLER",
				"address": "0x97B864eC76C6FfF9C5a72EB5d49f06bEf6B2a736",
				"name": "ROCKEFELLER",
				"decimals": 8
			},
			"0x1b081268ea677854c3f63de61582bb585198e870": {
				"ticker": "STEALTH",
				"address": "0x1B081268EA677854C3f63DE61582bb585198e870",
				"name": "Stealth",
				"decimals": 9
			},
			"0x086e1841cbe6053ac07a3958c3b3a827d8a56472": {
				"ticker": "MetaMask.io",
				"address": "0x086e1841cBe6053AC07a3958c3B3a827D8A56472",
				"name": "MetaMaskAirDrop (ONLY 1,000,000 Supply)",
				"decimals": 9
			},
			"0x2f151656065e1d1be83bd5b6f5e7509b59e6512d": {
				"ticker": "PGL",
				"address": "0x2F151656065E1d1bE83BD5b6F5e7509b59e6512D",
				"name": "Pangolin Liquidity",
				"decimals": 18
			},
			"0x93e6c622af1244b8b8bf64e0c0f7cf339f9882c4": {
				"ticker": "MNT",
				"address": "0x93e6c622AF1244B8B8bf64E0C0f7cf339F9882C4",
				"name": "Mount Nodes",
				"decimals": 18
			},
			"0x455a5a08da48a77d8d5148d1feb7333da22a8c0c": {
				"ticker": "RISEN",
				"address": "0x455a5A08DA48a77D8D5148d1feb7333Da22A8c0C",
				"name": "NodesRise",
				"decimals": 18
			},
			"0xecca68b361ef4d6d3bd7dd037c19c122d5dcc970": {
				"ticker": "Fuel",
				"address": "0xecCa68B361ef4D6D3bD7DD037c19c122d5dCc970",
				"name": "FuelNodes",
				"decimals": 18
			},
			"0x030afbbf8b85ccfdc860b014439fd89ebc3e71a4": {
				"ticker": "SPECTA",
				"address": "0x030afBBf8B85cCfdC860b014439Fd89EbC3E71a4",
				"name": "SpectaToken",
				"decimals": 18
			},
			"0xee21b6a6d20fba5b22a05ea9a635ceef4076eab3": {
				"ticker": "WDOG",
				"address": "0xEE21B6A6d20FbA5b22A05ea9A635CeEF4076Eab3",
				"name": "Winter Dog",
				"decimals": 18
			},
			"0xa82a82d9007eacfcfbcb4d236f5cfc31bbb788ec": {
				"ticker": "ATM",
				"address": "0xA82A82d9007EAcfcfbcB4d236f5cFc31Bbb788Ec",
				"name": "ATOM Protocol",
				"decimals": 18
			},
			"0x8aa86f897d69eb988b78b33245446bdda8b1ee57": {
				"ticker": "SONA",
				"address": "0x8aa86f897D69Eb988B78b33245446BDdA8B1EE57",
				"name": "SonOfAVAX",
				"decimals": 18
			},
			"0xd85cbd5b091e3d08f87367ab845513c2d7de5d3c": {
				"ticker": "UniPay",
				"address": "0xd85cBD5B091e3d08f87367Ab845513c2d7De5D3C",
				"name": "UniPay",
				"decimals": 9
			},
			"0x34be8799bbb7ef9f7ebe6b28e73d7c1b33bbace4": {
				"ticker": "AVAXM",
				"address": "0x34BE8799Bbb7eF9F7ebE6B28e73d7c1b33bbAce4",
				"name": "Avax Mystery",
				"decimals": 9
			},
			"0xcb0d164a1af708e5d5e97eede5f87f17d87f2f9c": {
				"ticker": "CHILL",
				"address": "0xcb0d164a1AF708e5D5E97eeDE5F87f17D87F2F9C",
				"name": "CHILL",
				"decimals": 9
			},
			"0x98cb35db93b64e7f89c54df3e8331f26ce611d8b": {
				"ticker": "🧿",
				"address": "0x98cB35db93b64E7F89C54Df3e8331F26ce611d8B",
				"name": "illuminati",
				"decimals": 9
			},
			"0x2509e20eb6f125fedf357947343a0e5dff64c23d": {
				"ticker": "STAR",
				"address": "0x2509e20EB6f125FedF357947343A0E5dFf64c23d",
				"name": "StarshipDAO ",
				"decimals": 9
			},
			"0x72780b61f92c610c3d6657bbbae90ace212446e2": {
				"ticker": "USDCRWRD",
				"address": "0x72780b61f92c610C3D6657bbbae90ACE212446E2",
				"name": "USDCRWRD",
				"decimals": 9
			},
			"0xf391628b1261735a16b337d4a006f2a3614b37b1": {
				"ticker": "Ava",
				"address": "0xf391628b1261735A16b337d4a006f2a3614b37b1",
				"name": "Avax Factory",
				"decimals": 18
			},
			"0xc4069e71cd035ca51ecb46ea0c924b07466fba33": {
				"ticker": "LTN",
				"address": "0xc4069e71cD035CA51ECb46EA0c924B07466FbA33",
				"name": "Lightning",
				"decimals": 18
			},
			"0x137a16486fc9dc497436dec5664d1517ea75f465": {
				"ticker": "MAGIA",
				"address": "0x137A16486Fc9Dc497436deC5664D1517ea75f465",
				"name": "Magia Finance",
				"decimals": 18
			},
			"0x284748b6c0c1ec75c73285f4557fe8f51f800480": {
				"ticker": "MIMiverse",
				"address": "0x284748b6C0c1Ec75C73285f4557fE8F51F800480",
				"name": "MIMivese",
				"decimals": 9
			},
			"0x4e9e3ce362d46f9f61b4f72464d1ff5e453d3829": {
				"ticker": "PLAR",
				"address": "0x4e9e3Ce362d46f9F61b4f72464D1FF5e453d3829",
				"name": "Polar Dao",
				"decimals": 1
			},
			"0xe0f000b79fe95675e078837d022190d6be2621ad": {
				"ticker": "ARCWOOL",
				"address": "0xe0F000b79Fe95675e078837d022190d6be2621Ad",
				"name": "ARCWOOL",
				"decimals": 9
			},
			"0xcad2dae5db94cf4554adc429a6a815dfb5349cc4": {
				"ticker": "RICE",
				"address": "0xCad2dAE5db94cF4554aDc429A6a815DfB5349cc4",
				"name": "Rice Finance",
				"decimals": 5
			},
			"0x3fdb0fb4fafc4d2d7e99f65244eb24f1cc276030": {
				"ticker": "RYV",
				"address": "0x3fdB0fB4fafc4d2d7E99F65244eb24F1Cc276030",
				"name": "Ryvin Nodes",
				"decimals": 18
			},
			"0x0c71756399118d0e6e58ba3f474c70e3434a4d25": {
				"ticker": "DIZ",
				"address": "0x0c71756399118D0e6E58BA3f474c70e3434a4D25",
				"name": "DIZZY coin",
				"decimals": 18
			},
			"0x66d9031e5354f779adf4e111991e988e5fdc4500": {
				"ticker": "StarBattle",
				"address": "0x66d9031E5354f779ADF4e111991E988e5fdC4500",
				"name": "StarBattle",
				"decimals": 8
			},
			"0x80cc980d19cc23c2e666735bf22cfc1dcd00b7df": {
				"ticker": "AURORA",
				"address": "0x80cc980d19cc23c2E666735BF22Cfc1dcD00B7dF",
				"name": "AURORA",
				"decimals": 8
			},
			"0x3d450fc675c1b0de1cc97af4ac87ac063208da5d": {
				"ticker": "eSTNK.e",
				"address": "0x3d450fC675C1B0DE1cC97AF4aC87Ac063208dA5D",
				"name": "eSTNK.e",
				"decimals": 9
			},
			"0xa2b84bb73340fa3c2b0e71ec678f1ed107bcff42": {
				"ticker": "BTCPPV2",
				"address": "0xA2B84Bb73340Fa3c2B0e71eC678F1eD107BcFf42",
				"name": "BTCPPV2",
				"decimals": 6
			},
			"0x5d9d666619345b595f37483bd03cd2f417e11f1f": {
				"ticker": "KEN",
				"address": "0x5d9D666619345B595F37483BD03cd2f417E11F1f",
				"name": "Ken Inu",
				"decimals": 18
			},
			"0x2ced20fdfcbe72c27a607d0c9ab1c9ada598c60f": {
				"ticker": "ARA",
				"address": "0x2cEd20fdFcBe72C27a607d0c9Ab1C9aDa598C60F",
				"name": "Ara",
				"decimals": 9
			},
			"0xc814a758db34326166204e3f96443b0ff54c1a20": {
				"ticker": "Zeu",
				"address": "0xc814a758dB34326166204E3f96443b0fF54c1A20",
				"name": "Zeus DAO",
				"decimals": 18
			},
			"0xa97cc6015ec275cd0aafa51d1116d022d0d28af2": {
				"ticker": "ATM",
				"address": "0xa97cc6015ec275cD0AAfa51D1116d022D0D28aF2",
				"name": "Atom Protocol",
				"decimals": 8
			},
			"0x14f857c7de1ea750b904ccb7ac47057ae5cf2ca6": {
				"ticker": "Infinit",
				"address": "0x14f857c7DE1eA750B904cCB7aC47057aE5cF2ca6",
				"name": "Infinity Nodes",
				"decimals": 18
			},
			"0x47b95af3fc29ed8307284bb9aba694c8ff32c858": {
				"ticker": "BOMB",
				"address": "0x47B95Af3fc29eD8307284BB9ABa694c8Ff32c858",
				"name": "Bomb Nodes",
				"decimals": 9
			},
			"0xd0a04e759c73d96401e995e07ff06c8ed2e98a97": {
				"ticker": "RBX",
				"address": "0xd0A04e759c73d96401e995e07FF06c8Ed2E98a97",
				"name": "RBX Token",
				"decimals": 5
			},
			"0xded4a9caf43dcc92a19b16f617031f07c5f236f9": {
				"ticker": "LAST",
				"address": "0xDED4A9CAf43DCc92a19B16F617031F07C5f236F9",
				"name": "Last Chance Before Bear",
				"decimals": 18
			},
			"0xdc1fa0a5fa0d3ca7876a100c1648503a466d16fb": {
				"ticker": "RBEES",
				"address": "0xDc1fA0A5FA0D3CA7876a100C1648503a466d16fB",
				"name": "Redbees",
				"decimals": 18
			},
			"0x44e1fd645f2d77d7249bbd0878e991cc53b1e9ad": {
				"ticker": "PEACE",
				"address": "0x44e1Fd645F2D77d7249BBD0878e991Cc53b1e9Ad",
				"name": "Peace DAO",
				"decimals": 18
			},
			"0xc1ab2e7daf96746347b158ce2a86c1e197f44b24": {
				"ticker": "DEGEN",
				"address": "0xC1ab2E7dAF96746347b158Ce2A86c1e197f44b24",
				"name": "DegenFi",
				"decimals": 9
			},
			"0xf954f573b3568755bdea216121259cf7bb78eeb7": {
				"ticker": "Sashim",
				"address": "0xf954F573B3568755bdeA216121259CF7bb78Eeb7",
				"name": "Sashimi DAO",
				"decimals": 18
			},
			"0xa468ecf9bbdce3120bd1f2b6c9eb63d9e825baaa": {
				"ticker": "MysteryAVAX",
				"address": "0xA468ecF9BbdcE3120BD1f2B6c9EB63d9E825baAA",
				"name": "MysteryAVAX",
				"decimals": 9
			},
			"0x1ed592de7426a1d80a7bc03b97b0cb6d3af05afa": {
				"ticker": "BOS",
				"address": "0x1ed592De7426A1D80A7BC03b97B0Cb6D3AF05Afa",
				"name": "BookOfSanta",
				"decimals": 9
			},
			"0xdc519bc42c8c85a71c6cda6711bbf964e1ff6a0c": {
				"ticker": "SSTAR",
				"address": "0xdC519BC42C8c85A71c6CDa6711bBf964e1fF6A0C",
				"name": "Nebula Shares",
				"decimals": 18
			},
			"0xe2e617bf35bd370ab797a921a815283c0688c009": {
				"ticker": "RND",
				"address": "0xE2E617bf35bd370ab797A921a815283c0688c009",
				"name": "Redlight Node District",
				"decimals": 1
			},
			"0x152df2302a1c1dd2f62a8c2b921573a60aa6454b": {
				"ticker": "VER",
				"address": "0x152Df2302a1c1dD2f62a8c2b921573a60Aa6454b",
				"name": "Verstappen",
				"decimals": 18
			},
			"0x9b6e420d62b441e24783171ef4bcd78773848426": {
				"ticker": "FNX",
				"address": "0x9b6E420d62B441e24783171EF4BCd78773848426",
				"name": "Phoenix Nodes",
				"decimals": 1
			},
			"0x50b589254ddf730e4056bb6d037e798334e1f4de": {
				"ticker": "PLAR",
				"address": "0x50b589254Ddf730E4056bB6D037e798334E1f4De",
				"name": "PolarDAO",
				"decimals": 9
			},
			"0xe8f5a3926e9980410ed0ec45881f8c2bb2ae1b1b": {
				"ticker": "Snowman",
				"address": "0xE8f5a3926E9980410ED0Ec45881F8c2bB2AE1B1B",
				"name": "Snowman DAO",
				"decimals": 18
			},
			"0xc63f40fa7c8aab1ecbf74d79deae53f89ee009ce": {
				"ticker": "NOBELIUM",
				"address": "0xc63f40FA7c8AAb1eCBf74D79DEAe53F89ee009CE",
				"name": "Nobelium DAO",
				"decimals": 9
			},
			"0xc8536433b5d74b2bfbc511a8db6002d528d241b2": {
				"ticker": "ASTRO",
				"address": "0xc8536433b5D74B2Bfbc511a8db6002D528D241b2",
				"name": "Astronode",
				"decimals": 9
			},
			"0x0697d3602668003b26869629fb3b311d827b5e8a": {
				"ticker": "BANANA",
				"address": "0x0697D3602668003b26869629fb3B311D827B5E8A",
				"name": "Retro Monkey Land",
				"decimals": 18
			},
			"0xf2798acd323421a6c61811a2362ed2d53cd28c65": {
				"ticker": "AKITA",
				"address": "0xF2798Acd323421a6c61811A2362ed2D53CD28c65",
				"name": "Akita Inu Avax",
				"decimals": 9
			},
			"0xebd1c37cd8fbe603062c3ff4e8488e261ecafbee": {
				"ticker": "LockedDAO",
				"address": "0xeBd1C37Cd8Fbe603062C3fF4e8488e261eCaFbEe",
				"name": "LockedDAO",
				"decimals": 18
			},
			"0x09266f4e4eb76e050c22c96a7a3fdc64bf8b2608": {
				"ticker": "💍",
				"address": "0x09266F4e4eb76e050C22C96a7a3fdc64bF8B2608",
				"name": "DiamondRing",
				"decimals": 9
			},
			"0xdb44b21b9ff68fb0d372546944426d6f2b0adf28": {
				"ticker": "RING",
				"address": "0xDB44B21b9Ff68fB0d372546944426d6f2B0aDf28",
				"name": "RING Nodes",
				"decimals": 1
			},
			"0x13be373f7a1dd6071f9ca1bec0c41b6159ef3f9c": {
				"ticker": "ANIV",
				"address": "0x13Be373F7a1Dd6071f9ca1BeC0c41B6159Ef3F9c",
				"name": "Aniverse",
				"decimals": 18
			},
			"0xd61776fb5de59bbc3c2636642297d04e46ca266a": {
				"ticker": "SODZ",
				"address": "0xd61776FB5de59BBc3C2636642297D04e46CA266a",
				"name": "Sonofdogezilla",
				"decimals": 9
			},
			"0xf9191954b5933c34e60616881fc48a54b908de82": {
				"ticker": "WDOG",
				"address": "0xf9191954B5933c34e60616881FC48A54B908De82",
				"name": "WINTERDOG AVAX",
				"decimals": 9
			},
			"0xb99847a1df98f4d988aef84f57efe55c6132ab1b": {
				"ticker": "STG",
				"address": "0xb99847a1DF98F4D988AeF84f57EFe55c6132ab1b",
				"name": "StagDAO",
				"decimals": 9
			},
			"0xd1dad46cf857dd27253ec3ecca28ec3d8981ea43": {
				"ticker": "AVALPS",
				"address": "0xd1dad46cF857DD27253EC3ECcA28Ec3d8981ea43",
				"name": "AVALPS DAO",
				"decimals": 9
			},
			"0x0166b25b0f767d804d1073c07d0819990ce1eb37": {
				"ticker": "EEVEE",
				"address": "0x0166B25B0f767D804D1073c07D0819990CE1eB37",
				"name": "Eevee Inu",
				"decimals": 9
			},
			"0xc6e818361cb8542cd86482624f1c36de19c78543": {
				"ticker": "Comet",
				"address": "0xc6E818361cb8542Cd86482624f1C36dE19c78543",
				"name": "Comet",
				"decimals": 9
			},
			"0x05f6c35187840094868bb5163638d1d534d51afe": {
				"ticker": "CSP",
				"address": "0x05f6C35187840094868bB5163638d1d534D51afe",
				"name": "CrazySpellPrinter",
				"decimals": 9
			},
			"0x565567fa8e74c2240431a920922e5e1781f032e1": {
				"ticker": "SuperNov",
				"address": "0x565567Fa8e74c2240431a920922e5e1781f032e1",
				"name": "SuperNova DAO",
				"decimals": 8
			},
			"0x0c7f0735861f7f5a3beeefb1b4a32e9e72ed07ef": {
				"ticker": "MAG",
				"address": "0x0c7f0735861f7F5a3beEefB1b4a32E9e72ed07ef",
				"name": "MagnetDAO",
				"decimals": 9
			},
			"0x8f52ce8717c186a33d722083dff585fc5f72966d": {
				"ticker": "POSE",
				"address": "0x8f52cE8717c186A33D722083dfF585FC5F72966d",
				"name": "Poseidon",
				"decimals": 8
			},
			"0xfe04f83647db631650286988699e253ed72edbae": {
				"ticker": "TROLL",
				"address": "0xFE04F83647dB631650286988699e253ED72EDbAE",
				"name": "TROLL",
				"decimals": 9
			},
			"0x2706c5601987e909d8abe246392b36ee06f3a976": {
				"ticker": "AKD",
				"address": "0x2706c5601987E909D8abE246392b36eE06f3a976",
				"name": "Astro Kitty DAO",
				"decimals": 18
			},
			"0xd57f0857682430ecc48f2b1babe03bf35b32d3c1": {
				"ticker": "ASTR",
				"address": "0xd57f0857682430EcC48F2B1BaBe03bf35B32D3C1",
				"name": "ASTRA POOL",
				"decimals": 8
			},
			"0x38b01514bba8f45d111262ea3bb1e2e0cf233b3a": {
				"ticker": "SPRT",
				"address": "0x38b01514bba8f45d111262Ea3bb1E2E0cF233b3A",
				"name": "SpiritDAO",
				"decimals": 18
			},
			"0xb195af20a0fec7e2c95b22a1c5de86a2389e40d5": {
				"ticker": "SEA",
				"address": "0xb195Af20a0fEc7E2C95b22a1c5de86a2389E40d5",
				"name": "Sea",
				"decimals": 9
			},
			"0x5112355b38794daf3b05a075798b420dd83bc484": {
				"ticker": "NIT",
				"address": "0x5112355b38794Daf3B05A075798B420DD83bC484",
				"name": "Node Investment Trust",
				"decimals": 18
			},
			"0xc244e532984bf94b72e674158e6fddfe179a6f00": {
				"ticker": "CRABP",
				"address": "0xC244e532984BF94b72E674158e6fdDfE179a6F00",
				"name": "Crabada Printer",
				"decimals": 5
			},
			"0xe190c57759221829f8fe376d7a2db9c4753e0d53": {
				"ticker": "TIMEp",
				"address": "0xe190C57759221829F8Fe376d7a2Db9c4753E0D53",
				"name": "TIMEPRINT",
				"decimals": 9
			},
			"0x670d6b19fd1e305f6c23dd60fc4a615eff1e2134": {
				"ticker": "TORNADO",
				"address": "0x670d6b19fd1E305F6C23dd60fc4a615EFf1e2134",
				"name": "TORNADO",
				"decimals": 8
			},
			"0x2e2dd16b9e670f02a2c8dad7e5b59902a2a85679": {
				"ticker": "ARMADAO",
				"address": "0x2E2Dd16b9E670f02a2C8DAD7e5b59902a2a85679",
				"name": "ArmageddonFinance (discord.gg/hdFBGTRtVB)",
				"decimals": 18
			},
			"0xa0ebcb85157f59684cf243dc8057184f2bba5b9b": {
				"ticker": "ALGOR",
				"address": "0xA0ebcB85157F59684Cf243dC8057184f2BBA5b9b",
				"name": "AlgorToken",
				"decimals": 18
			},
			"0x59748a440d6546563dcdf3e052b78520ab5d5457": {
				"ticker": "SierraDAO",
				"address": "0x59748A440d6546563DCdF3e052B78520Ab5d5457",
				"name": "SierraDAO",
				"decimals": 18
			},
			"0xdbd28bbe06087af0cec9951d19d6708aa4cb4402": {
				"ticker": "BRANDO",
				"address": "0xdbD28BBe06087af0CEC9951D19d6708aa4CB4402",
				"name": "Brando Finance",
				"decimals": 18
			},
			"0x4c1057455747e3ee5871d374fdd77a304ce10989": {
				"ticker": "XMTL",
				"address": "0x4C1057455747e3eE5871D374FdD77A304cE10989",
				"name": "NovaXMetal",
				"decimals": 18
			},
			"0x660869f18288e7c1d8969cc30a36a30048221c5e": {
				"ticker": "SHIELD",
				"address": "0x660869f18288e7C1d8969cC30a36A30048221C5E",
				"name": "ShieldFinance",
				"decimals": 18
			},
			"0x2763a43cad630fc6629b599601e32f3940efedf4": {
				"ticker": "CURVEp",
				"address": "0x2763a43Cad630fc6629b599601e32f3940EfeDF4",
				"name": "Curve Printer",
				"decimals": 18
			},
			"0x6ca603a8d5c4bde24da24a582f16bde40fcff6ea": {
				"ticker": "BPEPE",
				"address": "0x6cA603A8D5c4BDE24Da24A582f16BDe40fCfF6eA",
				"name": "BabyPepe",
				"decimals": 9
			},
			"0xe57546bc2d4756a3702ef0c96decad940ea41555": {
				"ticker": "Matri",
				"address": "0xe57546BC2d4756a3702eF0C96DEcad940ea41555",
				"name": "Matrix Protocol",
				"decimals": 8
			},
			"0x64c01e1b74f48349d6dcf942ea319b25f7f826c5": {
				"ticker": "Simple",
				"address": "0x64C01E1b74F48349d6dcf942ea319b25F7F826c5",
				"name": "Simplex DAO",
				"decimals": 8
			},
			"0xc74924f8c32a79002a4bddadcd75a16ff0bcf58b": {
				"ticker": "GIFT",
				"address": "0xC74924F8C32a79002a4BDDADCD75a16Ff0BCF58B",
				"name": "GiftDao",
				"decimals": 8
			},
			"0xd74cca216f7c8fe33f0911da8698e2b3efc98835": {
				"ticker": "MONG",
				"address": "0xD74CCA216F7c8Fe33f0911da8698E2B3eFc98835",
				"name": "Mongoose",
				"decimals": 9
			},
			"0xd6785aadc35f7c029e79219947b72571dbef1453": {
				"ticker": "LOVE",
				"address": "0xd6785aAdc35f7C029E79219947B72571dbEF1453",
				"name": "AvaxLentine",
				"decimals": 18
			},
			"0x8848a133b73034c910b7d9d52a450b8ff04d5a5b": {
				"ticker": "PATRIOT",
				"address": "0x8848a133B73034C910B7d9d52A450b8fF04d5A5b",
				"name": "PATRIOT",
				"decimals": 8
			},
			"0x561d6b88ed3e224cd265c814f20ce1f346bf376d": {
				"ticker": "SnowBullet",
				"address": "0x561d6b88eD3E224Cd265C814F20CE1f346bf376D",
				"name": "SnowBullet",
				"decimals": 8
			},
			"0xe2ae686d91462a779939ced7f5f7145b5b442025": {
				"ticker": "MOONBOUNCER",
				"address": "0xE2ae686d91462a779939CED7F5f7145b5b442025",
				"name": "MOONBOUNCER",
				"decimals": 9
			},
			"0xbfb084412df5bad6ab702396ddc74d0ffcf628a9": {
				"ticker": "GFL",
				"address": "0xBFb084412dF5baD6ab702396ddC74D0FfcF628a9",
				"name": "Galactic Fight League",
				"decimals": 5
			},
			"0xbd9d606a853d9ddd4f82819aa6fb25708288542a": {
				"ticker": "💍",
				"address": "0xBd9d606a853D9dDD4f82819aA6Fb25708288542a",
				"name": "DiamondRing",
				"decimals": 9
			},
			"0xc374eaac333960f980479aa2ad27c60bdf6c2b7f": {
				"ticker": "COM",
				"address": "0xC374eAac333960F980479AA2Ad27C60bdf6c2B7f",
				"name": "Comet Money",
				"decimals": 18
			},
			"0x26eb2ebc9313fce90169f8bf3dec2656023d5329": {
				"ticker": "PCTL",
				"address": "0x26Eb2ebc9313fce90169f8Bf3dec2656023D5329",
				"name": "Particle Money",
				"decimals": 18
			},
			"0x979afc9a316e460c7a9eb0f2b74b7244439c20a8": {
				"ticker": "PATTERN",
				"address": "0x979afc9a316e460C7a9eb0f2b74b7244439C20A8",
				"name": "PATTERN",
				"decimals": 8
			},
			"0x9502dab5a0cf6c0c6e24e188c7fe703bd50961f4": {
				"ticker": "VKNGS",
				"address": "0x9502dAb5A0CF6c0c6e24e188C7Fe703BD50961f4",
				"name": "VIKING NODES",
				"decimals": 1
			},
			"0x304642de3d598b0f77bb93529caa5b5021ca6952": {
				"ticker": "CNC",
				"address": "0x304642dE3D598B0f77bb93529CAA5b5021Ca6952",
				"name": "Coinverse Node Capital",
				"decimals": 9
			},
			"0x7e9cbfbe6da281f4bb8fa6016daad28010d40b52": {
				"ticker": "BLIZZ",
				"address": "0x7e9cbFbe6Da281f4bB8FA6016dAad28010d40b52",
				"name": "Blizzard Financial https://www.nodeblizzard.com",
				"decimals": 8
			},
			"0x36a5c549f6cee488c8b64a3467a891437ed7cb8f": {
				"ticker": "Particl",
				"address": "0x36A5C549f6cEE488c8B64a3467A891437ED7Cb8F",
				"name": "Particle Chain",
				"decimals": 8
			},
			"0xf5f277894ce428b49c550090c7bde954e0cfa9d2": {
				"ticker": "$BASED",
				"address": "0xF5f277894CE428b49c550090c7bdE954e0CFa9d2",
				"name": "Based Protocol",
				"decimals": 5
			},
			"0x4b30999b2e86da715271b8495aadb9e06d4277c0": {
				"ticker": "HRCLS",
				"address": "0x4b30999B2E86DA715271B8495AAdb9E06D4277c0",
				"name": "Hercules Finance",
				"decimals": 18
			},
			"0x2d63bf11948413a53930f5155a5ad8a1c887c18b": {
				"ticker": "ELXR",
				"address": "0x2D63bf11948413a53930F5155a5Ad8a1c887C18B",
				"name": "Elexir",
				"decimals": 9
			},
			"0x6195de423da947f52ae8d55331c207c963ed2b3f": {
				"ticker": "bECH",
				"address": "0x6195DE423da947f52AE8d55331C207c963eD2b3f",
				"name": "Bridged Echelon",
				"decimals": 18
			},
			"0x2767c43f89b30b0243e4715f20927d934578ea54": {
				"ticker": "AGN",
				"address": "0x2767c43F89B30b0243e4715f20927d934578EA54",
				"name": "AVAGN",
				"decimals": 9
			},
			"0x9f6c94af0c7ef2430cefc9e82e8d01897490ec29": {
				"ticker": "Gree",
				"address": "0x9F6c94Af0c7eF2430cEfC9E82e8D01897490EC29",
				"name": "Green Energy",
				"decimals": 8
			},
			"0x2a5ad15778c7406b565a81c3ce9f1246258c212e": {
				"ticker": "HUSKY",
				"address": "0x2A5ad15778c7406B565a81C3Ce9F1246258C212E",
				"name": "HUSKY  NET",
				"decimals": 8
			},
			"0x81bdfa65b17bab32a548fdb5592761037c547a95": {
				"ticker": "PACMAN",
				"address": "0x81bDfa65b17BAB32A548FdB5592761037C547A95",
				"name": "PACMAN",
				"decimals": 8
			},
			"0xe3f7399d530eb2475135d10bed26435c97f484c6": {
				"ticker": "ROCKET",
				"address": "0xe3F7399D530eB2475135d10beD26435C97F484c6",
				"name": "Rocket Nodes",
				"decimals": 18
			},
			"0x4862825bba3ec1fb1c991a8765914327a90a8070": {
				"ticker": "TIMEPPV2",
				"address": "0x4862825bba3Ec1fb1C991A8765914327a90A8070",
				"name": "TIMEPPV2",
				"decimals": 9
			},
			"0x182646e84586dda1a2168a7c4ea3790d9834eb31": {
				"ticker": "JRINU",
				"address": "0x182646E84586dda1a2168a7C4EA3790d9834EB31",
				"name": "JoeRoganInu",
				"decimals": 6
			},
			"0x300a15a81e8380508c6e16d4195a226e3a38488a": {
				"ticker": "FOMC",
				"address": "0x300A15a81E8380508C6E16d4195A226E3A38488a",
				"name": "FOMC",
				"decimals": 9
			},
			"0x003bdfbb315c6b39d83e715abdfc6d5422c5ac6e": {
				"ticker": "ATOM",
				"address": "0x003bdFBb315c6b39d83e715ABdFC6d5422c5Ac6E",
				"name": "Atom Protocol",
				"decimals": 1
			},
			"0x93ea2bf18408a571b7baebf2c52c11d073fdc4ef": {
				"ticker": "OHMP",
				"address": "0x93Ea2BF18408A571b7BAEBF2c52c11D073fdC4EF",
				"name": "OHM Printer",
				"decimals": 6
			},
			"0xfd94f2f4650c866e5dc0d30e759fa678f4966ab5": {
				"ticker": "PIZZA",
				"address": "0xfD94f2f4650c866e5dC0D30e759fa678F4966aB5",
				"name": "Pizza Game",
				"decimals": 18
			},
			"0xa2c35ad3af4d080c6af0e98e2f6ad133fc45b7c6": {
				"ticker": "SOS",
				"address": "0xa2C35AD3aF4d080c6Af0e98e2F6AD133Fc45b7c6",
				"name": "OpenDAO",
				"decimals": 1
			},
			"0x6f586b788c813f8235c643c4a3018b9afbd599f1": {
				"ticker": "Crypt",
				"address": "0x6f586B788C813F8235c643c4A3018b9AFBD599F1",
				"name": "Crypto Mind",
				"decimals": 2
			},
			"0xd3f154f20e91c295bd620e69c11e55f31dbb041f": {
				"ticker": "ACASH",
				"address": "0xd3F154f20E91C295BD620e69C11E55f31dbB041f",
				"name": "AVAX Cash",
				"decimals": 9
			},
			"0x975947ea4c0b9694e6e11b9de58cb6de9abbf879": {
				"ticker": "HIY",
				"address": "0x975947EA4C0b9694E6E11b9dE58cB6de9abbF879",
				"name": "Happ-Inu Year",
				"decimals": 18
			},
			"0xdb5f3a16e87d432a494198e188958f9acff06df0": {
				"ticker": "UND",
				"address": "0xdB5f3a16e87D432A494198E188958f9aCff06Df0",
				"name": "UNINODEFork",
				"decimals": 9
			},
			"0x072a182d28ce3bd5f25afa19c84ff31000dd1fe2": {
				"ticker": "TI",
				"address": "0x072A182d28cE3bd5F25AFa19c84fF31000DD1Fe2",
				"name": "The Island",
				"decimals": 9
			},
			"0x91e44634288fab8a3dc9f81f2399ee00463940e0": {
				"ticker": "NEXTN",
				"address": "0x91e44634288FaB8a3Dc9f81f2399ee00463940E0",
				"name": "Next Node",
				"decimals": 18
			},
			"0xce4a921a8855cd288b348ecd670164c98ba45e44": {
				"ticker": "MASK",
				"address": "0xce4a921A8855CD288b348ecD670164c98ba45E44",
				"name": "Metamask Token",
				"decimals": 8
			},
			"0xb7837ab6000dbda8683951309c6614ae279ac62a": {
				"ticker": "denverFOS",
				"address": "0xb7837aB6000dbdA8683951309c6614Ae279AC62a",
				"name": "FarmersOnly's Swap Denver Coin",
				"decimals": 18
			},
			"0x7334391e2e9fd8386f238698f8ce24f5a4d41117": {
				"ticker": "NRGY",
				"address": "0x7334391E2e9FD8386f238698f8CE24f5A4D41117",
				"name": "WIND FINANCE",
				"decimals": 18
			},
			"0xce93ed0ff845f6d5021a1cb69df0e19a32b63f6a": {
				"ticker": "BL4CK",
				"address": "0xcE93ed0fF845F6D5021A1cb69dF0e19a32B63F6a",
				"name": " EverLaunch",
				"decimals": 18
			},
			"0x3e6972d5a20f1af9432cb2113fe24b94c8c006b6": {
				"ticker": "bJOE",
				"address": "0x3e6972d5A20F1AF9432Cb2113Fe24B94C8C006B6",
				"name": "BL4CK Fi",
				"decimals": 9
			},
			"0x542bc8671d41e1e8312f0d3137062d07e2516418": {
				"ticker": "WOLV",
				"address": "0x542Bc8671D41e1E8312f0D3137062d07e2516418",
				"name": "BabyJoe",
				"decimals": 18
			},
			"0x23fe77a916d42dce56509d6efcf98f8bd8f12fa1": {
				"ticker": "Genera",
				"address": "0x23FE77a916D42DCE56509d6eFCf98F8bd8F12FA1",
				"name": "Wolverine DAO",
				"decimals": 6
			},
			"0x5be6d339de3f2ccf752a88893b9824f27a0edf9c": {
				"ticker": "Haw",
				"address": "0x5Be6d339dE3F2ccf752a88893B9824F27A0EDf9c",
				"name": "General printer",
				"decimals": 8
			},
			"0xd7318d150e9e7d972ab36b4461718a174bec2875": {
				"ticker": "NRGY",
				"address": "0xD7318d150E9e7D972ab36b4461718A174BEC2875",
				"name": "Hawk Eye",
				"decimals": 18
			},
			"0x04a758a16a8a34a86678d54fe3d9dc553c1dc136": {
				"ticker": "ALF",
				"address": "0x04a758A16a8A34a86678D54fE3d9DC553c1DC136",
				"name": "Wind Finance",
				"decimals": 8
			},
			"0xc7f641317cb3ee7831095d27cad0b921a0d879ed": {
				"ticker": "THOR",
				"address": "0xC7f641317Cb3eE7831095d27cAd0B921a0d879ED",
				"name": "AliceFrog",
				"decimals": 1
			},
			"0x2887995fb327d85589467cbd89bc6961ddc12b41": {
				"ticker": "NewBor",
				"address": "0x2887995FB327d85589467CbD89BC6961DDC12B41",
				"name": "THOR'S LIGHTNING NODES",
				"decimals": 6
			},
			"0xd3e6a63d52ce11566cdeb1dfe8da03df5fb59fc8": {
				"ticker": "$LFT",
				"address": "0xd3e6A63D52CE11566CdEb1Dfe8da03df5FB59FC8",
				"name": "NewBorn Shiba",
				"decimals": 9
			},
			"0x1f2b36c78ebec34ee1c62c3a8a0fd6f9a3a441b7": {
				"ticker": "BOG",
				"address": "0x1f2B36C78EbEc34eE1c62C3A8a0FD6F9a3A441B7",
				"name": "Luffy Floki Token",
				"decimals": 18
			},
			"0x1d676c32e7c81f380bdbd5511525879d6689a2d5": {
				"ticker": "SSSI",
				"address": "0x1D676c32e7C81F380bdBD5511525879D6689A2d5",
				"name": "Bogdanoff",
				"decimals": 18
			},
			"0x904c57fce78c1f7d36c08335927c6624a6fb81cb": {
				"ticker": "NO",
				"address": "0x904C57fCE78c1F7D36c08335927C6624a6FB81CB",
				"name": "Shooting Star Shiba Inu",
				"decimals": 1
			},
			"0x51b0d15e9c4be684a44dbd837d3d120c215c2136": {
				"ticker": "Hecto",
				"address": "0x51b0D15E9C4bE684a44Dbd837D3D120c215C2136",
				"name": "Nobelium DAO",
				"decimals": 8
			},
			"0xc177c4836ba19bed340bf74b5bb8fa53cfb88c77": {
				"ticker": "THGIL",
				"address": "0xc177C4836Ba19bed340bf74B5bb8fA53Cfb88c77",
				"name": "THGIL",
				"decimals": 9
			},
			"0xb65c7a61ebbd5a95ddd613e29d7b66d191bb4698": {
				"ticker": "GKI",
				"address": "0xb65c7A61EbbD5A95ddd613e29d7B66D191BB4698",
				"name": "Galactic Kitty Inu",
				"decimals": 18
			},
			"0xf2cbee662d69b483ab230c446bbd4d717ed55174": {
				"ticker": "IND",
				"address": "0xF2Cbee662d69B483AB230C446bBd4d717ED55174",
				"name": "Intergalactic Doge",
				"decimals": 18
			},
			"0x404f44b52b0b37fcee4f6c46cbe861e26bb0e67b": {
				"ticker": "JUICE",
				"address": "0x404F44B52B0b37fCee4F6c46cbe861e26bB0E67B",
				"name": "JUICE",
				"decimals": 18
			},
			"0x3fde08d324c04379c27db5f2fdeb38de63938e35": {
				"ticker": "kmim",
				"address": "0x3fde08d324C04379C27dB5f2FDEb38de63938E35",
				"name": "kraken_earn_mim",
				"decimals": 6
			},
			"0x784efd2b1aeb676044e6fca6d38090d45dc625ca": {
				"ticker": "DRM",
				"address": "0x784efd2B1aeB676044e6FCa6d38090d45Dc625Ca",
				"name": "Dream DAO",
				"decimals": 18
			},
			"0xbe52a44dd857e877c1acffd55fb02b68653cd5da": {
				"ticker": "AVA",
				"address": "0xBE52a44Dd857E877C1acfFd55fB02B68653Cd5dA",
				"name": "AVA NODES  ",
				"decimals": 18
			},
			"0xdad2bbe1be2303f1b76817b20578f6c499b33cdf": {
				"ticker": "BBL",
				"address": "0xDAD2BbE1BE2303F1B76817B20578f6c499B33Cdf",
				"name": "Bubble Nodes",
				"decimals": 18
			},
			"0x0e384e2479ce6c682288172d7e4724802fcd53ee": {
				"ticker": "ASHARE",
				"address": "0x0e384E2479Ce6c682288172d7e4724802fcD53Ee",
				"name": "Atlas Finance",
				"decimals": 18
			},
			"0x385230db0e65590c66287503382c877e4cb71c0f": {
				"ticker": "PP",
				"address": "0x385230DB0e65590c66287503382C877e4cb71c0F",
				"name": "PEEPEE POOPOO",
				"decimals": 9
			},
			"0x45addbd8b2a59b414f6599b9be6b102f5be6d736": {
				"ticker": "SPRT",
				"address": "0x45aDdBd8B2a59B414F6599b9BE6b102f5BE6d736",
				"name": "SpiritDao",
				"decimals": 1
			},
			"0x05dfef7997a4aad7bc44d521ba7369a8f0992681": {
				"ticker": "NoCap",
				"address": "0x05DfEF7997A4AaD7BC44D521Ba7369a8f0992681",
				"name": "NodeCapital",
				"decimals": 18
			},
			"0xf8408413346f3c12dfac71b0b1e99ff618ea681d": {
				"ticker": "TLSTRZ",
				"address": "0xF8408413346f3C12dfac71B0b1E99ff618eA681d",
				"name": "Twinkle Little Starnodz",
				"decimals": 6
			},
			"0x53b89b81e6da8b2b4a5c9aa5f19145c8412245e4": {
				"ticker": "CBD",
				"address": "0x53B89B81e6dA8B2B4A5C9aA5F19145C8412245e4",
				"name": "CobraDao",
				"decimals": 18
			},
			"0x074609e055e0124d8a36d537bd9b8052db6458da": {
				"ticker": "wBNB.p",
				"address": "0x074609E055e0124D8A36d537Bd9B8052Db6458Da",
				"name": "wBNB.p",
				"decimals": 8
			},
			"0x45cf8b81b33a3e40b255a7388f66ce77189c5696": {
				"ticker": "WDV",
				"address": "0x45cF8b81B33a3e40b255a7388f66Ce77189c5696",
				"name": "WalkingDeadVirus",
				"decimals": 9
			},
			"0x318ec58a2a7ca2e44f0b2a7127ad2f3fe2f0d10f": {
				"ticker": "LKD",
				"address": "0x318ec58a2a7ca2e44F0B2A7127AD2f3fe2f0D10f",
				"name": "Lunar Kitty DAO",
				"decimals": 18
			},
			"0xed26360af664dce7aeeb2abb1a7deb21ca9aa2f0": {
				"ticker": "SWING",
				"address": "0xed26360Af664dcE7AeeB2abb1A7dEB21CA9aA2F0",
				"name": "SWINGDAO",
				"decimals": 9
			},
			"0x80deb1b3d6770b3659c1cfc741abd9a362750f06": {
				"ticker": "BPepe",
				"address": "0x80dEb1b3D6770b3659C1Cfc741aBD9A362750F06",
				"name": "BabyPepe",
				"decimals": 18
			},
			"0xebdccf79491f2a7ce71a4f094b60958222ec28a2": {
				"ticker": "RFI",
				"address": "0xEbDCcf79491f2A7ce71a4F094b60958222eC28a2",
				"name": "Test",
				"decimals": 6
			},
			"0xc9e13f3c0f739a80921b188276ccf448b970541b": {
				"ticker": "CANNA",
				"address": "0xC9e13F3c0f739a80921B188276cCF448B970541b",
				"name": "CannaDAO",
				"decimals": 9
			},
			"0xc60fffa3701dd7ff4fc564f3827901fedf740ffb": {
				"ticker": "PEEL",
				"address": "0xc60FfFa3701dd7ff4fC564F3827901feDf740fFb",
				"name": "BANANA",
				"decimals": 9
			},
			"0x55344c5075a43f1803a86e4525d2076ce1f5d401": {
				"ticker": "VTLK",
				"address": "0x55344C5075a43F1803A86E4525D2076ce1f5d401",
				"name": "VitalikNodes",
				"decimals": 18
			},
			"0x50b276820c66cbb6ac3f2ed26d5541390daa8729": {
				"ticker": "ETHS",
				"address": "0x50b276820C66Cbb6aC3f2ed26D5541390DaA8729",
				"name": "EtherStones",
				"decimals": 18
			},
			"0x468003b688943977e6130f4f68f23aad939a1040": {
				"ticker": "BEE",
				"address": "0x468003B688943977e6130F4F68F23aad939a1040",
				"name": "HoneyBee",
				"decimals": 18
			},
			"0x3d67f591a8aea6fcbe9015f499aec2bb7d32d477": {
				"ticker": "AOT",
				"address": "0x3D67f591a8AEa6FCBE9015F499Aec2BB7D32d477",
				"name": "AvaxOnTitan",
				"decimals": 18
			},
			"0x0636c2c8db60ad9159b54eb904e2fb3450cf49eb": {
				"ticker": "LAB",
				"address": "0x0636c2c8db60aD9159b54eb904E2fb3450Cf49Eb",
				"name": "Lab Finance",
				"decimals": 8
			},
			"0x182490d1afb40356c9960721ddd5e557a50bee91": {
				"ticker": "SSHARE",
				"address": "0x182490d1AFb40356C9960721ddD5E557A50BEE91",
				"name": "STAR SHARES",
				"decimals": 18
			},
			"0x45f330987fe1e58768e91b4d6bbc5a57658ce8d5": {
				"ticker": "PLAYMATES",
				"address": "0x45f330987fe1E58768E91b4d6BBC5A57658CE8D5",
				"name": "Redlight Node District",
				"decimals": 1
			},
			"0x073f39ce7a0de99992fd4c4693fd1895fe7a4adf": {
				"ticker": "JEET",
				"address": "0x073F39ce7a0DE99992FD4c4693FD1895Fe7a4ADF",
				"name": "Jeet Coin",
				"decimals": 9
			},
			"0xde073b6be5c9ac531265882d60b6632f4ad6c3c2": {
				"ticker": "SRHON",
				"address": "0xde073B6be5C9aC531265882d60b6632f4AD6c3c2",
				"name": "Shuey Rhon Rhon Avax",
				"decimals": 18
			},
			"0xa968c97d10b9f7d5fc05ef82df63518db5a71f07": {
				"ticker": "WDOG",
				"address": "0xA968C97d10b9F7d5Fc05EF82dF63518dB5A71F07",
				"name": "Winter Dog",
				"decimals": 18
			},
			"0xebc527b7842eaea4e3517895120373c9ce98b476": {
				"ticker": "PEP",
				"address": "0xebC527B7842eAEa4E3517895120373c9CE98b476",
				"name": "Pepperoni",
				"decimals": 9
			},
			"0xb32df1f186b142735dc428ba83ece44097cdcdd0": {
				"ticker": "WIFEAVAX",
				"address": "0xB32Df1F186b142735dC428Ba83EcE44097cdcdD0",
				"name": "MyWifeIsAvax",
				"decimals": 18
			},
			"0x4480b4ddfb15fe6518817ef024d8b493aff2db54": {
				"ticker": "BIRD",
				"address": "0x4480B4DdFb15fE6518817ef024D8B493afF2Db54",
				"name": "BirdToken",
				"decimals": 18
			},
			"0x4d4c4e78fe21a6422cd4c3f43280178365eaf25b": {
				"ticker": "$AVAFARM",
				"address": "0x4D4C4E78fe21A6422CD4c3F43280178365Eaf25b",
				"name": "AVAFARM",
				"decimals": 9
			},
			"0x612576633c945102cb263b80c9d88584b93e55fa": {
				"ticker": "PIG",
				"address": "0x612576633C945102CB263B80c9d88584B93e55fa",
				"name": "PigToken t.me/Pigprotect",
				"decimals": 18
			},
			"0x5725f8fcdb2c384df0ca2ca7cf58936ae67867bf": {
				"ticker": "CRL",
				"address": "0x5725f8fcdB2c384df0CA2ca7cf58936AE67867bf",
				"name": "Circle Nodes",
				"decimals": 1
			},
			"0xec7a60786b8d74c8bcf3979227b6530f175b65e9": {
				"ticker": "ALphaP",
				"address": "0xEc7a60786b8d74c8BcF3979227b6530f175b65E9",
				"name": "AlphaPrinter",
				"decimals": 6
			},
			"0x8c99c0b9ea39959530b627cb4c2c4e1b96b26286": {
				"ticker": "IMMORTAL",
				"address": "0x8C99c0b9Ea39959530b627cB4c2c4e1B96b26286",
				"name": "IMMORTAL",
				"decimals": 6
			},
			"0xcd262cc4c3131d111a5b69e71b26690fb6ffc5b5": {
				"ticker": "AERO",
				"address": "0xcD262cc4C3131d111A5b69e71B26690fb6ffC5B5",
				"name": "AeroSwap Token",
				"decimals": 18
			},
			"0x392dbc51cdb41330c814bf699d311f57a555cb14": {
				"ticker": "Silen",
				"address": "0x392dbC51cDB41330C814Bf699d311f57a555cB14",
				"name": "Silent Night",
				"decimals": 8
			},
			"0xd99393b68ed70568c470ff0ea8d60bafaa5440dd": {
				"ticker": "MIDAS",
				"address": "0xd99393B68ED70568c470Ff0eA8D60bAfAa5440dd",
				"name": " Star Sharks / t.me/StarSharksAvax",
				"decimals": 18
			},
			"0xeb21e86fc7ef7c99660da8f8c29c0d060db15018": {
				"ticker": "TWO",
				"address": "0xeB21E86FC7eF7c99660dA8f8c29C0D060Db15018",
				"name": "Son of MIDAS",
				"decimals": 18
			},
			"0x43486789c54d735a9e618c0bcab4a7cf9a831490": {
				"ticker": "HACHICO",
				"address": "0x43486789c54D735A9e618C0BCaB4a7cf9A831490",
				"name": "Two Two Two Two Printer",
				"decimals": 18
			},
			"0xec8486a329abbb99363dd2d57eece3080f0355f0": {
				"ticker": "BeeFarm",
				"address": "0xec8486a329ABbB99363DD2D57EeCe3080f0355F0",
				"name": "Hachico INU",
				"decimals": 8
			},
			"0xd80692965016dfd893b07b1bf1b9d3289326b7b9": {
				"ticker": "ETHPP",
				"address": "0xd80692965016DfD893b07b1Bf1B9d3289326B7b9",
				"name": "BeeFarm",
				"decimals": 6
			},
			"0xfa4b6db72a650601e7bd50a0a9f537c9e98311b2": {
				"ticker": "HSHARES",
				"address": "0xfa4B6db72A650601E7Bd50a0A9f537c9E98311B2",
				"name": "ETHPP",
				"decimals": 18
			},
			"0x9c48138d777a8e4f818033a46cc58cdf51bdfc2b": {
				"ticker": "LIGHT",
				"address": "0x9C48138D777a8E4F818033a46CC58cdF51BDFc2B",
				"name": "HERMES Shares",
				"decimals": 1
			},
			"0x6d23d1674d5ac1d8103992a82ccfe7775eede97c": {
				"ticker": "KGXY",
				"address": "0x6D23d1674D5ac1D8103992a82cCFe7775eeDE97C",
				"name": "Electro Nodes",
				"decimals": 18
			},
			"0xb871c0c4dc25feaae9bce753b5a2d9da5d54bbc4": {
				"ticker": "WST",
				"address": "0xb871c0C4dC25feAae9bcE753b5a2d9dA5d54bBc4",
				"name": "Kitty Galaxy",
				"decimals": 5
			},
			"0x302c6819b5a0e080118e2cf48ae2a5d1af44f1b5": {
				"ticker": "BirdseyeNodes",
				"address": "0x302C6819B5A0E080118E2cf48aE2a5d1aF44F1b5",
				"name": "Wall-Street Lion",
				"decimals": 18
			},
			"0x776aba470b891435309fce58b23cf55373af9eea": {
				"ticker": "CODE",
				"address": "0x776aba470B891435309fcE58B23cf55373aF9EEa",
				"name": "Birdseye Nodes",
				"decimals": 18
			},
			"0x7e1b7b160d14322fad35fe600d06b3f0db145e45": {
				"ticker": "PIMP",
				"address": "0x7e1B7b160D14322FAd35FE600D06b3f0Db145E45",
				"name": "Code",
				"decimals": 9
			},
			"0x9d071c77722d2cf6395a47e42c049d0e363972f3": {
				"ticker": "Futur",
				"address": "0x9d071c77722d2CF6395a47e42C049d0E363972f3",
				"name": "PIMP",
				"decimals": 4
			},
			"0xfca1eae8099bc53832258bf5d4286e2dcf7dae7f": {
				"ticker": "PEPPERONI",
				"address": "0xfcA1eAE8099Bc53832258BF5D4286e2Dcf7DAE7f",
				"name": "Future Protocol",
				"decimals": 18
			},
			"0xe18950c8f3b01f549cfc79dc44c3944fbd43fb76": {
				"ticker": "ICY",
				"address": "0xE18950c8F3b01f549cFc79dC44C3944FBd43fB76",
				"name": "Pizza Pals",
				"decimals": 6
			},
			"0xb8146b12567a8c4959de5dcd937d14bee918dc9b": {
				"ticker": "ETHA",
				"address": "0xB8146B12567A8C4959de5DCd937D14bEe918Dc9b",
				"name": "ICY.MONEY",
				"decimals": 9
			},
			"0x97dfe12fe0a81e94fc3fd429be7e3759d5776534": {
				"ticker": "ShowTime",
				"address": "0x97dfe12fe0A81e94Fc3FD429be7e3759d5776534",
				"name": "EthaVerseDao",
				"decimals": 8
			},
			"0x4f4f9639dd0e9b00260e9e634c37a2b1bb0b2336": {
				"ticker": "SSS",
				"address": "0x4F4F9639dd0e9b00260e9E634C37A2b1bb0B2336",
				"name": "ShowTime",
				"decimals": 18
			},
			"0x119538d30412a20fb37626959821df9ccdc92b23": {
				"ticker": "PRMD",
				"address": "0x119538D30412a20Fb37626959821dF9CCDC92B23",
				"name": "Shooting Star Shiba",
				"decimals": 18
			},
			"0xbeb8e048984fbb10543707e75f1a41603e9013e2": {
				"ticker": "MILK",
				"address": "0xbEB8e048984FbB10543707e75f1A41603E9013E2",
				"name": "Pyramid Money",
				"decimals": 18
			},
			"0x3ea736922beb3d6f80cc7de7507bb334090d68a4": {
				"ticker": "wAVAX.p",
				"address": "0x3Ea736922Beb3d6f80CC7De7507bb334090d68A4",
				"name": "Ben Nodes",
				"decimals": 8
			},
			"0xd7918c1743c77dc0bd87852ee51fac27bab30997": {
				"ticker": "KTTY",
				"address": "0xd7918c1743c77dC0bd87852eE51FAc27bAB30997",
				"name": "wAVAX printer",
				"decimals": 18
			},
			"0x211939e96043aa9314781a93301ceff12869396a": {
				"ticker": "EMPIRE",
				"address": "0x211939e96043aA9314781A93301CefF12869396A",
				"name": "Kitty Galaxy DAO",
				"decimals": 9
			},
			"0x358380412e9dad464b34cf35bc7ecaaf34fd2c2a": {
				"ticker": "FSHIB",
				"address": "0x358380412E9DaD464b34Cf35bC7ECaaF34fD2C2A",
				"name": "EmpireDAO",
				"decimals": 18
			},
			"0xf0445663d50d328e788d92591880bc6891a0dbb7": {
				"ticker": "MWD",
				"address": "0xf0445663d50D328e788D92591880bc6891a0dBb7",
				"name": "FLUFFY SHIBA",
				"decimals": 18
			},
			"0x70e2b3a596a76ce50b34c6a1793be73833c9cb75": {
				"ticker": "KSHARE",
				"address": "0x70e2b3A596A76ce50B34c6A1793be73833C9Cb75",
				"name": "Milky Way DAO",
				"decimals": 18
			},
			"0xf95e9c868ff5c20a3b397154f21722abb8e0e0d5": {
				"ticker": "RougeCoin",
				"address": "0xf95E9C868FF5c20A3b397154F21722abB8E0e0D5",
				"name": "KUSH SHARES",
				"decimals": 2
			},
			"0xc00dd95c5de5401e2c563cd1e21f4432b1882ec7": {
				"ticker": "PolarBear",
				"address": "0xC00Dd95c5DE5401e2c563Cd1E21f4432b1882Ec7",
				"name": "RougeCoin",
				"decimals": 6
			},
			"0x8ae38ae6f245b218db9573af37b0d80b94b2852d": {
				"ticker": "FUCK",
				"address": "0x8AE38AE6f245B218db9573AF37b0d80B94b2852d",
				"name": "PolarBear",
				"decimals": 18
			},
			"0xf280266d836545bd3a73a7a5dcc886d5a54b4738": {
				"ticker": "AMR",
				"address": "0xf280266d836545Bd3A73a7A5DcC886d5a54B4738",
				"name": "FUCKtoken",
				"decimals": 9
			},
			"0xe870452b5fda61eb496814c4cfda84ed3f4ae155": {
				"ticker": "DB",
				"address": "0xe870452b5fda61Eb496814c4Cfda84ED3f4AE155",
				"name": "Avax Maker https://t.me/avaxmaker",
				"decimals": 18
			},
			"0x2558b8fba8f678fd3029c927cd3780f5d3c266c7": {
				"ticker": "SCAT",
				"address": "0x2558b8FBa8f678Fd3029C927CD3780f5d3C266C7",
				"name": "DiamondBank DAO",
				"decimals": 9
			},
			"0x99db0cace60215d5effdc46240757f8453b3285e": {
				"ticker": "FINITY",
				"address": "0x99dB0CaCe60215D5EFFdC46240757f8453b3285e",
				"name": "SnowCatDAO",
				"decimals": 18
			},
			"0x71d9a9b1f590cf85312aa9190d5e0a2d5700db21": {
				"ticker": "WAB",
				"address": "0x71d9a9B1F590Cf85312aA9190D5E0a2d5700DB21",
				"name": " WAGMI",
				"decimals": 18
			},
			"0x1affbc17938a25d245e1b7ec6f2fc949df8e9760": {
				"ticker": "PAPER",
				"address": "0x1affBc17938a25d245e1B7eC6f2fc949df8E9760",
				"name": "Infinity Finance",
				"decimals": 18
			},
			"0x5d139423c138b8d26b860b7c15c983d777557ca6": {
				"ticker": "SROCKET",
				"address": "0x5D139423c138b8D26b860b7C15c983D777557CA6",
				"name": "WAKE AND BAKE",
				"decimals": 18
			},
			"0x61591886a48352c60a6c2388e646ec1eb8c35650": {
				"ticker": "REWARD",
				"address": "0x61591886a48352c60a6C2388e646Ec1eB8c35650",
				"name": "PAPER",
				"decimals": 6
			},
			"0xb15f02f9da8cd1f99e9dd375f21dc96d25ddd82c": {
				"ticker": "HERMES",
				"address": "0xB15f02F9Da8CD1f99E9dd375F21dc96D25ddd82C",
				"name": "SROCKET",
				"decimals": 18
			},
			"0x06cb414da9b02a1c275a4294324b34d16fafd92b": {
				"ticker": "MC",
				"address": "0x06cb414Da9b02a1c275A4294324b34d16FaFd92B",
				"name": "The Reward",
				"decimals": 9
			},
			"0x993163cad35162fb579d7b64e6695cb076ef5064": {
				"ticker": "MM",
				"address": "0x993163CaD35162fB579D7B64e6695cB076EF5064",
				"name": "HERMES",
				"decimals": 18
			},
			"0xb66082583422ebd6eda3ddaaeb7d2829e390d1ea": {
				"ticker": "ALIEN",
				"address": "0xb66082583422ebD6edA3dDAAEB7d2829E390D1EA",
				"name": "MetaVerse Capital",
				"decimals": 18
			},
			"0x21291e920ad8b1ad53a2e47466c8a29bad9b0162": {
				"ticker": "DSYN",
				"address": "0x21291E920aD8b1AD53a2e47466C8a29BAd9B0162",
				"name": "Million",
				"decimals": 18
			},
			"0x7879ee5db99ae6fae05a64964de55d27ba76bcec": {
				"ticker": "LV",
				"address": "0x7879eE5db99Ae6fae05a64964dE55D27bA76BcEc",
				"name": "Martian DAO",
				"decimals": 18
			},
			"0x34d3b962390f707ac8381d5f9d66b54e8724ac9f": {
				"ticker": "MCA",
				"address": "0x34D3b962390F707AC8381D5f9D66b54E8724ac9f",
				"name": "DeFi Syndicate",
				"decimals": 18
			},
			"0x55853edc67aa68ec2e3903ac00f2bc5bf2ca8db0": {
				"ticker": "YAXIS",
				"address": "0x55853EDC67Aa68EC2E3903Ac00f2bc5bf2ca8dB0",
				"name": "LandVerse",
				"decimals": 18
			},
			"0xf877f12a119d1e36e85044002f080a7826ae0030": {
				"ticker": "DGD",
				"address": "0xf877f12a119d1E36E85044002F080A7826aE0030",
				"name": "Mystercoin AVAX",
				"decimals": 18
			},
			"0x61db988de7d2084be02804c78218e89645a5dfe0": {
				"ticker": "ICE",
				"address": "0x61Db988De7D2084BE02804C78218e89645A5dFe0",
				"name": "yAxis V2",
				"decimals": 18
			},
			"0x2f398a6cfe8746a413c9e4dc7ae7df1394bcc3ee": {
				"ticker": "CORE",
				"address": "0x2f398A6CFe8746A413c9E4dc7Ae7DF1394BcC3eE",
				"name": "Doge Galaxy DAO",
				"decimals": 18
			},
			"0xe61a3488e378d7fe0db8f431af01c5ec406af3f1": {
				"ticker": "EINU",
				"address": "0xe61A3488E378d7FE0db8f431AF01C5eC406aF3F1",
				"name": "Mammoth ICE",
				"decimals": 9
			},
			"0xbd8ad88c6f30c60b219639075f4bbe89a30d9fc3": {
				"ticker": "FOX",
				"address": "0xBD8aD88C6f30c60B219639075F4BBe89A30D9Fc3",
				"name": "Core Nodes",
				"decimals": 9
			},
			"0xafe0ae04ba1511916b81c2563fa8747c48fa14ff": {
				"ticker": "FER",
				"address": "0xafE0Ae04ba1511916b81c2563fA8747c48Fa14Ff",
				"name": "ELK INU",
				"decimals": 18
			},
			"0x085ff3223ec0663f1563dabd8f86a46883c3f4c9": {
				"ticker": "TFCFOA",
				"address": "0x085Ff3223eC0663f1563dABd8F86A46883C3F4c9",
				"name": "FoxOfAvax https://t.me/foxofavax",
				"decimals": 8
			},
			"0x9242bcfb04205ea91eeb45d49472265ba9f11a7f": {
				"ticker": "MIAP",
				"address": "0x9242bcfb04205eA91EeB45D49472265Ba9F11a7F",
				"name": "ForEverRise",
				"decimals": 18
			},
			"0xd34d212a192f00a1dd752303cc761ed8205911d7": {
				"ticker": "SEA",
				"address": "0xd34D212a192f00a1dD752303Cc761Ed8205911D7",
				"name": "THE FIRST CLIFFORD FORK ON AVAX",
				"decimals": 1
			},
			"0x961c289745832b148b099827836c914f697fbca3": {
				"ticker": "UNI",
				"address": "0x961c289745832b148B099827836c914F697fbCA3",
				"name": "MIAP",
				"decimals": 9
			},
			"0x1083b4bece971e07e6907ef244605a798dca595b": {
				"ticker": "HON",
				"address": "0x1083B4bECE971E07E6907EF244605a798DcA595B",
				"name": "HonToken",
				"decimals": 18
			},
			"0x2bad92078e479c7cff345ae7e1ab1a5c542b24fe": {
				"ticker": "CSI",
				"address": "0x2bAd92078e479C7cff345Ae7e1ab1a5C542B24fE",
				"name": "Cosmic Shiba Inu",
				"decimals": 18
			},
			"0x1b5946b0352a7ab2152f4ec862f8378e1bd16efa": {
				"ticker": "CCD",
				"address": "0x1B5946B0352a7AB2152f4EC862F8378e1bd16eFA",
				"name": "Cosmic Cash DAO",
				"decimals": 18
			},
			"0x6c4d8ffe840e2ba7ac927074d2391ae8b26a953f": {
				"ticker": "MIT",
				"address": "0x6C4d8ffe840e2bA7aC927074d2391aE8b26a953F",
				"name": "MagicInternetTiger",
				"decimals": 6
			},
			"0x91372cad31f4bab69a3d6c29166fb38de6442adf": {
				"ticker": "MKI",
				"address": "0x91372CAd31F4BAb69A3D6c29166FB38de6442AdF",
				"name": "Miakhalifainu",
				"decimals": 6
			},
			"0xa1beb60e7b873ca969cc97ded967ee4487ec95d1": {
				"ticker": "Moon",
				"address": "0xA1bEb60e7b873Ca969cc97ded967ee4487Ec95d1",
				"name": "Moon Node",
				"decimals": 1
			},
			"0xb4dd2577272e88f21642847b58295c1b55b3aeff": {
				"ticker": "XANAX",
				"address": "0xB4dd2577272e88f21642847B58295C1B55B3aeFF",
				"name": "XanaX Bars",
				"decimals": 18
			},
			"0xf0a2b290cb7f02957b713be981f14b4867dce38f": {
				"ticker": "GLMR",
				"address": "0xF0A2B290CB7F02957b713bE981f14B4867DcE38F",
				"name": "Moonbeam AVAX",
				"decimals": 9
			},
			"0x85bd2e64e3f4418deba2c319dc177924e3f86b2c": {
				"ticker": "WOLF",
				"address": "0x85bD2e64e3F4418deba2C319dC177924E3F86B2c",
				"name": "Moonwolf",
				"decimals": 6
			},
			"0xc6521a4e7e385dc15aa0f58dee951e7970ede6f5": {
				"ticker": "Cosmi",
				"address": "0xc6521a4E7e385dC15aA0f58dEe951e7970EdE6F5",
				"name": "Cosmic Gas",
				"decimals": 8
			},
			"0x697768cbe01c6bc2f69f69d5799dd6d949e00bd6": {
				"ticker": "BDOGE",
				"address": "0x697768CbE01c6BC2F69f69D5799dD6D949e00bd6",
				"name": "Baby Doge",
				"decimals": 9
			},
			"0x5c058d0ab65e1a4290fae4bea988c13c488d1b78": {
				"ticker": "ENTROPY",
				"address": "0x5c058D0Ab65e1A4290fAe4Bea988C13C488d1B78",
				"name": "ENTROPY",
				"decimals": 8
			},
			"0xcc7033a7f778506ae0a9ed92a9deaff4bb40d19d": {
				"ticker": "Islan",
				"address": "0xcc7033A7f778506ae0a9eD92A9deaFf4Bb40D19D",
				"name": "maDAOgascar",
				"decimals": 9
			},
			"0x523047d4b7605841f50e1b6bc4401bf2af45fd1e": {
				"ticker": "UnivPrinter",
				"address": "0x523047d4b7605841F50e1B6Bc4401Bf2Af45Fd1E",
				"name": "UnivPrinter",
				"decimals": 9
			},
			"0x1e87df106c36051fbcd3e997be7a77a0a17c64df": {
				"ticker": "Met",
				"address": "0x1E87df106c36051FBCD3e997be7A77a0a17C64dF",
				"name": "Meta Center",
				"decimals": 8
			},
			"0x8de2239f47934b8449ad1c713e498e631cdc48ab": {
				"ticker": "Neutro",
				"address": "0x8De2239F47934B8449aD1C713e498e631cdc48Ab",
				"name": "Neutron Protocol",
				"decimals": 8
			},
			"0x1edcaa6ab03c7e9e67bb382baeec264f5cd0e493": {
				"ticker": "BATF",
				"address": "0x1EDcAa6AB03c7E9E67BB382baEeC264f5CD0E493",
				"name": "BatFinance",
				"decimals": 9
			},
			"0x4735721ed62713e3a141c939f4aa55ca8ad5f66a": {
				"ticker": "CHIHUA",
				"address": "0x4735721ED62713E3A141C939f4Aa55cA8aD5F66A",
				"name": "Chihuahuax",
				"decimals": 18
			},
			"0x646b18feff6424b5c575451433573dff8c5995af": {
				"ticker": "ALINALI",
				"address": "0x646b18FEFF6424b5c575451433573Dff8c5995af",
				"name": "Alina Li https://t.me/alinalidev",
				"decimals": 8
			},
			"0xb559ca613d7d4f974fc8f8ce521f64757ee02fa6": {
				"ticker": "METAP",
				"address": "0xB559CA613d7D4F974fc8F8Ce521f64757eE02fa6",
				"name": "Meta Infinity",
				"decimals": 18
			},
			"0xaac30c6e82ab581de60b33d5577fe735837ae99e": {
				"ticker": "BOOp",
				"address": "0xAAc30C6e82aB581DE60b33d5577Fe735837aE99E",
				"name": "BooPrint",
				"decimals": 18
			},
			"0x8f77879ff66dc9f090b97105394c7ef225c9ee97": {
				"ticker": "STAR",
				"address": "0x8f77879fF66Dc9f090B97105394C7eF225C9ee97",
				"name": "Star Nodes",
				"decimals": 8
			},
			"0xebe8477a928ed2212867bda81ed3bb77c16fa986": {
				"ticker": "Dice",
				"address": "0xebE8477a928eD2212867bDA81ed3bb77c16fa986",
				"name": "Dice DAO",
				"decimals": 18
			},
			"0x3f3fea5dd9b1940c2ee4822d7b7a06f19d58656e": {
				"ticker": "AVAPAY",
				"address": "0x3F3Fea5dd9b1940C2ee4822D7B7a06F19d58656e",
				"name": "AVAPAY",
				"decimals": 9
			},
			"0xbe1b4c109358bf31cdb1175d1b3639bf544ef9ca": {
				"ticker": "BLOCK",
				"address": "0xbE1b4c109358Bf31cDb1175D1b3639Bf544eF9CA",
				"name": "Secret Block Chain",
				"decimals": 9
			},
			"0x611d7d0d7fa60273f64718a7f927931f8833a22e": {
				"ticker": "HELA",
				"address": "0x611d7d0d7FA60273f64718A7f927931F8833A22E",
				"name": "HELA",
				"decimals": 6
			},
			"0x9cf74b370097e6fe0a5b38a85e9c708938d10b9a": {
				"ticker": "LOKI",
				"address": "0x9Cf74B370097e6fe0a5b38A85E9c708938d10B9a",
				"name": "LOKI",
				"decimals": 6
			},
			"0x3c5cdd2c649256f286e25a17148d559a5281c9be": {
				"ticker": "SWING",
				"address": "0x3c5cDd2c649256F286E25A17148D559a5281C9be",
				"name": "Swing DAO",
				"decimals": 9
			},
			"0x6d1a3780b44c1a4ca1596c1c7d57f80557d29d1d": {
				"ticker": "aREFLECT",
				"address": "0x6d1A3780B44C1A4CA1596c1c7d57f80557D29d1D",
				"name": "Avax Reflect",
				"decimals": 9
			},
			"0x9f6d4dfc7b9ef2b004f5de518131c370bfb18117": {
				"ticker": "RGK",
				"address": "0x9f6D4dFC7b9Ef2B004F5De518131c370Bfb18117",
				"name": "Ragnarok DAK ragnarokdao.finance",
				"decimals": 18
			},
			"0x2f2477d6f0f9051cdcb83c590520a03ec91bb4fa": {
				"ticker": "PIGGY",
				"address": "0x2f2477D6f0F9051cDcB83c590520a03eC91bb4Fa",
				"name": "PIGGY",
				"decimals": 18
			},
			"0x1139f697f3d3eb62ccd457853fb74d005886a4d6": {
				"ticker": "PRMD",
				"address": "0x1139F697F3d3EB62cCD457853FB74d005886a4D6",
				"name": "Pyramid Money",
				"decimals": 1
			},
			"0x396af1cd5e82e3943677adc590706a9fedcf244a": {
				"ticker": "FAKEXAVAX",
				"address": "0x396aF1cD5E82E3943677AdC590706A9FeDCf244A",
				"name": "FAKEXAVAX",
				"decimals": 18
			},
			"0xc3ec350c7158da64a8765f73e2348d3f69d9aa23": {
				"ticker": "SPP",
				"address": "0xc3Ec350c7158Da64A8765F73E2348D3f69d9Aa23",
				"name": "Sparta Printer",
				"decimals": 18
			},
			"0x901ef2cd29c5ba92a1844448dcd38cd6268b1114": {
				"ticker": "CNC",
				"address": "0x901eF2cd29c5BA92a1844448DCD38Cd6268B1114",
				"name": "Coinverse Node Capital",
				"decimals": 9
			},
			"0x56504f3e39533ce324bc8b814e532c4c5a4e6f0f": {
				"ticker": "MICRO",
				"address": "0x56504f3E39533ce324BC8b814E532c4c5A4E6f0F",
				"name": "MICROSWAP",
				"decimals": 18
			},
			"0x5764b8d8039c6e32f1e5d8de8da05ddf974ef5d3": {
				"ticker": "PGL",
				"address": "0x5764b8D8039C6E32f1e5d8DE8Da05DdF974EF5D3",
				"name": "Pangolin Liquidity",
				"decimals": 18
			},
			"0x31814342f827e1ff8cfe4bc6b945466b2b35a22c": {
				"ticker": "MILK",
				"address": "0x31814342F827e1fF8CFE4BC6B945466B2b35A22c",
				"name": "MILK",
				"decimals": 8
			},
			"0x76764d770a0530bfef36fc0ef8497caeb54b24f5": {
				"ticker": "QUA",
				"address": "0x76764d770a0530bfef36fc0Ef8497CAEb54B24f5",
				"name": "Quarashi",
				"decimals": 9
			},
			"0xee79d462e7ff155b9615c4cf3f0e309f89cb509b": {
				"ticker": "vRMP",
				"address": "0xEe79D462e7ff155B9615C4cF3f0e309f89cb509B",
				"name": "veRamp",
				"decimals": 18
			},
			"0xeebae716854433f085f1625596c4c4093e2d41be": {
				"ticker": "OMG",
				"address": "0xEebAE716854433F085F1625596C4C4093e2d41be",
				"name": "OMG",
				"decimals": 8
			},
			"0x81b4015b10dac9a8286e028d1eb08f17599680e9": {
				"ticker": "LVT",
				"address": "0x81b4015b10Dac9a8286e028d1Eb08F17599680e9",
				"name": "Louverture Finance louverture.finance",
				"decimals": 18
			},
			"0x334c2b4d444010c172c2172e0b1a9a8757f82fcd": {
				"ticker": "NFTL",
				"address": "0x334c2b4d444010c172C2172e0b1a9a8757f82fCD",
				"name": "NFT Lottery Ticket",
				"decimals": 18
			},
			"0x83c7412931398502922a35911e5fab221822f4b6": {
				"ticker": "BDAO",
				"address": "0x83c7412931398502922a35911E5Fab221822f4B6",
				"name": "Bamboo DAO Shares",
				"decimals": 18
			},
			"0x9633644cfbbed71183674103c14a0b01a88f077f": {
				"ticker": "pUNIV",
				"address": "0x9633644cfBbEd71183674103C14A0b01a88F077F",
				"name": "UNIV Printer",
				"decimals": 18
			},
			"0x023f0aa8438a9989a4122c14e450a58e1391134f": {
				"ticker": "ALICEFROG",
				"address": "0x023f0Aa8438A9989A4122C14E450A58e1391134f",
				"name": "AliceFrog (twitter.com/alicefrog_avax)",
				"decimals": 9
			},
			"0x0ac80e1753dea5e298e8a2b6cf53f161937806a1": {
				"ticker": "OTO",
				"address": "0x0aC80E1753deA5e298E8a2b6CF53f161937806A1",
				"name": "OTO V2",
				"decimals": 5
			},
			"0xe23060399dfee8a9f5011d1b120707c84e80f523": {
				"ticker": "MIMPPV2",
				"address": "0xe23060399dfEE8a9F5011D1B120707c84e80f523",
				"name": "MIMPPV2",
				"decimals": 6
			},
			"0x0dbdca94ad84fe38fe57e16c39b1d303aca92450": {
				"ticker": "SR",
				"address": "0x0dbdCa94Ad84Fe38Fe57e16c39B1d303ACA92450",
				"name": "StealthRocket",
				"decimals": 9
			},
			"0xaf767428acdefcb8c3654ff870c8d5a1014dd962": {
				"ticker": "BLACK",
				"address": "0xAf767428aCdeFcB8C3654fF870C8d5A1014dD962",
				"name": "BlackFriday",
				"decimals": 9
			},
			"0x7148bdaa53dc56b55c219f7b5decbeba38cd5a79": {
				"ticker": "BCASH",
				"address": "0x7148bdAA53dC56B55C219f7B5DeCBeBa38cd5A79",
				"name": "BANKSCASH",
				"decimals": 6
			},
			"0x5f6607a5dacb4bb123ea5e717331b1cd87a068f8": {
				"ticker": "DrunkFloki",
				"address": "0x5F6607a5dAcB4BB123eA5e717331b1CD87A068f8",
				"name": "Drunk Floki AVAX",
				"decimals": 9
			},
			"0x4fc2eeeb26b71ebcf1f3f011bcb9541fc7bbe122": {
				"ticker": "FRACTAL",
				"address": "0x4Fc2EEEB26b71Ebcf1f3f011BcB9541FC7bBe122",
				"name": "FractalNodes",
				"decimals": 18
			},
			"0x77570b267409b19f7ca0725f0d8588c72cdf1dd6": {
				"ticker": "THUNDER",
				"address": "0x77570b267409b19f7ca0725F0d8588c72cDF1DD6",
				"name": "THUNDER NODES",
				"decimals": 18
			},
			"0xaf563dc55b51991bc380e792c33897ac02411497": {
				"ticker": "CHICK",
				"address": "0xAf563dc55b51991bc380e792c33897AC02411497",
				"name": "CHICK-EARN",
				"decimals": 18
			},
			"0xeed43ea23753f733e88652a943b29262ff4c4f6d": {
				"ticker": "Stor",
				"address": "0xEeD43EA23753F733E88652A943b29262fF4c4f6D",
				"name": "Storm DAO",
				"decimals": 8
			},
			"0x6070b71358c5ace2fbe40439eb688383ad58fe82": {
				"ticker": "CHAIN",
				"address": "0x6070b71358C5AcE2fbe40439eB688383aD58fe82",
				"name": "Chain Port chainport.io",
				"decimals": 18
			},
			"0xb0c09af51251b6a463414b5ec70d72e44ffdb5f6": {
				"ticker": "AVA",
				"address": "0xB0c09Af51251b6a463414B5Ec70D72E44FFdb5F6",
				"name": "Ava Capital",
				"decimals": 18
			},
			"0xcc058c59b459471299d74577bb249e07617effbb": {
				"ticker": "PONZI",
				"address": "0xcc058c59B459471299d74577bb249E07617efFBB",
				"name": "Ponzi Compiler",
				"decimals": 18
			},
			"0x0ced9ab0ee1fd029b187b3610673977a7fd45945": {
				"ticker": "KZU",
				"address": "0x0cEd9aB0ee1fD029b187B3610673977a7FD45945",
				"name": "KUZURYU",
				"decimals": 18
			},
			"0x56671a1b895839bbe494cc808cbc17f18b2d3033": {
				"ticker": "PTP-Agg",
				"address": "0x56671a1B895839bBe494cC808cbc17f18B2d3033",
				"name": "Platypus Aggregator",
				"decimals": 9
			},
			"0x2a44ac7ef1cc348ebcbce069590a6c801043a7c2": {
				"ticker": "DE",
				"address": "0x2a44ac7ef1cC348EBCbCe069590A6c801043a7C2",
				"name": "DEX Sport",
				"decimals": 8
			},
			"0x95d5ecabef375f5288deadb2b5e8ee3186c23d55": {
				"ticker": "ACH",
				"address": "0x95d5ECaBef375f5288DeAdb2B5e8eE3186C23D55",
				"name": "AvaxChads",
				"decimals": 9
			},
			"0x6ae66ba4aab9f672d669bd64ada02b110ed4556c": {
				"ticker": "THOR",
				"address": "0x6aE66bA4AaB9f672D669Bd64AdA02b110ED4556c",
				"name": "THORv3 AirDrop",
				"decimals": 9
			},
			"0xa408d1c1ef0d58c3a4fe0ee65cc619eb394b9684": {
				"ticker": "ETHPRI",
				"address": "0xA408D1c1Ef0d58c3A4Fe0ee65cc619eb394B9684",
				"name": "ETH PRINT",
				"decimals": 6
			},
			"0xf55c183e32d114a90cdbca3e6fd534457f7c3868": {
				"ticker": "RUMBLE",
				"address": "0xF55c183E32D114A90cdBca3e6FD534457f7C3868",
				"name": "RUMBLE",
				"decimals": 18
			},
			"0xe873a98d24ac990af8bf46eca0f234e940388e68": {
				"ticker": "JoeD",
				"address": "0xE873a98d24ac990aF8Bf46eCa0f234e940388e68",
				"name": "JOEnnyDeep",
				"decimals": 9
			},
			"0xfa7bc029e29f41a87b5fbd9758eea70a4cd45c8e": {
				"ticker": "SRA",
				"address": "0xFa7BC029E29f41A87b5Fbd9758eEA70A4Cd45c8E",
				"name": "SierraDAO",
				"decimals": 18
			},
			"0x91393a1d53b2f4a08d62466832b3ce79310f1869": {
				"ticker": "FUSE",
				"address": "0x91393A1D53B2F4A08d62466832B3cE79310F1869",
				"name": "FuseProtocol",
				"decimals": 5
			},
			"0x9dfbe71b368e7f44829d75b7cefd61a3a8f7e3bf": {
				"ticker": "PLAR",
				"address": "0x9dFbe71b368e7f44829d75B7CeFd61A3a8F7e3bf",
				"name": "PolarDAO",
				"decimals": 18
			},
			"0x635d19f25201119a0d3d80fc55a1dad6190c50bc": {
				"ticker": "FNX",
				"address": "0x635d19F25201119A0D3d80Fc55a1Dad6190C50bC",
				"name": "Phoenix Node",
				"decimals": 18
			},
			"0xb2dc59e8141cd8aa3dbefbfe123b3cee2d4f05aa": {
				"ticker": "RBUNNY",
				"address": "0xB2DC59e8141Cd8aA3dbEfBfE123b3CEe2d4F05Aa",
				"name": "META BUNNY REWARDS",
				"decimals": 6
			},
			"0x3424d99af53dc4527dc8a1f8fb080844836a9a6b": {
				"ticker": "MIMP",
				"address": "0x3424D99Af53Dc4527DC8a1f8FB080844836A9A6B",
				"name": "MIMPRINTER",
				"decimals": 9
			},
			"0x46fffcc66088f6ed2a7ee30dc0456cd1c4bde74e": {
				"ticker": "SOV",
				"address": "0x46ffFCC66088F6eD2A7eE30DC0456CD1c4Bde74e",
				"name": "SonOfVegeta",
				"decimals": 6
			},
			"0x4395715e869784a540a4e0eddb2b773c0adf0222": {
				"ticker": "ARES",
				"address": "0x4395715e869784A540a4e0EDdB2B773c0ADF0222",
				"name": "ARES",
				"decimals": 6
			},
			"0x553c66f282f508c6bee1935170f4310e5cb82a1c": {
				"ticker": "BSN",
				"address": "0x553C66f282F508C6BEE1935170F4310e5cb82A1C",
				"name": "BosonDAO",
				"decimals": 18
			},
			"0x6cced333c4b876861ba883ff0789dce54defe63e": {
				"ticker": "ANTc",
				"address": "0x6CCed333C4B876861bA883Ff0789DcE54DeFe63e",
				"name": "Ant Network",
				"decimals": 18
			},
			"0x1ffb4f360b4fb54d5e1312bec8f01efa88c67331": {
				"ticker": "LAYER",
				"address": "0x1FFb4f360B4Fb54D5E1312bEc8f01eFA88C67331",
				"name": "LayerFort",
				"decimals": 18
			},
			"0x84f4df4d33be4c5f7f28fe8568fec987875434a8": {
				"ticker": "PREZ",
				"address": "0x84f4DF4D33bE4c5F7F28Fe8568feC987875434a8",
				"name": "Santas Workshop",
				"decimals": 9
			},
			"0x340eb798911fffc12df7d8d083e5bce89ab11836": {
				"ticker": "BULD",
				"address": "0x340eB798911FFFC12df7D8d083e5bCe89aB11836",
				"name": "Bulldog DAO",
				"decimals": 18
			},
			"0x7ea2e0ef3dc5ab4862e0b195e7077a23a6bdf6d6": {
				"ticker": "JPOT",
				"address": "0x7eA2E0EF3Dc5aB4862E0B195e7077A23a6bDf6D6",
				"name": "Golden Jackpot",
				"decimals": 18
			},
			"0x5ede350e84223fb50775fd91a723f2ca71034cf7": {
				"ticker": "aWOOL",
				"address": "0x5eDE350E84223fb50775fD91a723F2ca71034cf7",
				"name": "AWOOL",
				"decimals": 9
			},
			"0x5938956037534ae93089a91d427d7855f83726bf": {
				"ticker": "$CORDY",
				"address": "0x5938956037534Ae93089a91d427d7855f83726bF",
				"name": " MagicAvaxMoney",
				"decimals": 9
			},
			"0x88a425b738682f58c0ff9fcf2cceb47a361ef4cf": {
				"ticker": "TEMPO",
				"address": "0x88a425b738682f58C0FF9fcF2CceB47a361ef4cF",
				"name": "CordyCeps",
				"decimals": 9
			},
			"0x67b9b66addf9558158318cc0b6d2b9ad50d76c52": {
				"ticker": "AVAXCOLONY",
				"address": "0x67b9b66aDdf9558158318cC0B6D2B9AD50D76C52",
				"name": "tempo",
				"decimals": 10
			},
			"0x2f6bd09eea3aac91ae02ffb829847ba97e8949c6": {
				"ticker": "Psi",
				"address": "0x2F6bd09EEA3aAc91Ae02FfB829847bA97e8949C6",
				"name": "AvalancheColony V1",
				"decimals": 18
			},
			"0x359c0c8b365fc0e458bf545942ee10460f9a98bc": {
				"ticker": "DBalls",
				"address": "0x359c0c8B365fC0E458bF545942eE10460f9a98bc",
				"name": "PsiDAO",
				"decimals": 18
			},
			"0x903705e479fc6458e8ebcd003343b716e4aabc01": {
				"ticker": "ALPHAZERO",
				"address": "0x903705E479FC6458E8eBCd003343B716E4aABC01",
				"name": "Diamond Balls",
				"decimals": 18
			},
			"0x9439a1baa97eaafeaf1a5fc2a5125e2c95a04683": {
				"ticker": "FNX",
				"address": "0x9439A1BAa97eAAFEAf1a5FC2A5125e2C95A04683",
				"name": "ALPHAZERO",
				"decimals": 1
			},
			"0x41a064b9bd7797022479e41f7fbbd02bce6179fa": {
				"ticker": "Flash",
				"address": "0x41a064B9bD7797022479E41f7FBBD02bce6179fA",
				"name": "Phoenix Nodes",
				"decimals": 18
			},
			"0x09a8d13b228284072a595216455ab9c834808911": {
				"ticker": "GRZZLY",
				"address": "0x09A8d13B228284072a595216455ab9C834808911",
				"name": "FlashDAO",
				"decimals": 6
			},
			"0x244c1e0ff179e465b8a92087cb04e3bf5a1cb2c9": {
				"ticker": "Golde",
				"address": "0x244C1E0Ff179E465B8a92087Cb04E3bF5a1cB2c9",
				"name": "GRIZZLY",
				"decimals": 8
			},
			"0x40dc03187b563f42b8ffc9ebeca1c274ac57c230": {
				"ticker": "FTF",
				"address": "0x40dC03187B563f42B8FfC9eBEca1C274Ac57c230",
				"name": "Golden Ratio",
				"decimals": 9
			},
			"0xececd68256b4142860b4f2af51729ebbc734ad95": {
				"ticker": "GAL",
				"address": "0xECECD68256B4142860b4F2AF51729eBBc734ad95",
				"name": "FuckTheFed",
				"decimals": 6
			},
			"0x33b98b3926a9a31e696b1d30a96bf344b043df06": {
				"ticker": "GLO",
				"address": "0x33B98B3926A9a31e696b1d30a96Bf344B043dF06",
				"name": "Infinitum Galacti",
				"decimals": 18
			},
			"0x40c82db3521323252b81ddef1d3dcc8f920ef21b": {
				"ticker": "Loo",
				"address": "0x40c82Db3521323252b81DDEF1d3Dcc8f920ef21B",
				"name": "GloryDAO",
				"decimals": 18
			},
			"0x536141711986387b485c8ce78514605e3889e403": {
				"ticker": "MULTI",
				"address": "0x536141711986387B485C8CE78514605e3889E403",
				"name": "Loop Nodes",
				"decimals": 18
			},
			"0x1d18e0cd249dad4b4a844784c5888a9cb842c646": {
				"ticker": "BDA",
				"address": "0x1d18e0Cd249dad4B4a844784C5888A9cb842c646",
				"name": "MULTIVERSE",
				"decimals": 6
			},
			"0xcaf870dad882b00f4b20d714bbf7fceada5e4195": {
				"ticker": "iBFR",
				"address": "0xCaf870DaD882b00F4b20D714Bbf7fceADA5E4195",
				"name": "iBuffer Token",
				"decimals": 18
			},
			"0x5373b3a4f454024d433023c2452e515d4b990da4": {
				"ticker": "SKD",
				"address": "0x5373b3a4f454024d433023C2452e515d4B990DA4",
				"name": "Space Kitty DAO",
				"decimals": 18
			},
			"0x4e5a5f50b723aef211c8583450193bdf0b60d234": {
				"ticker": "CAKEDAO",
				"address": "0x4e5A5f50B723aEF211c8583450193BDf0b60d234",
				"name": "CAKE DAO",
				"decimals": 18
			},
			"0x09ad4a1746ab9ecc96bb67d919de3c6b54cc879f": {
				"ticker": "GF",
				"address": "0x09Ad4a1746Ab9ECC96BB67d919DE3C6b54cc879f",
				"name": "Good Fire",
				"decimals": 18
			},
			"0x603741cf1cac3e6e07ee6872d59328cb80290eb2": {
				"ticker": "GIGA",
				"address": "0x603741CF1Cac3e6E07eE6872D59328cB80290Eb2",
				"name": "GIGANO FINANCE",
				"decimals": 18
			},
			"0x0b6541e373088392c5685f9f9a7077c3f10d4e4f": {
				"ticker": "SNTA",
				"address": "0x0b6541e373088392C5685f9f9a7077C3f10D4E4f",
				"name": "Santa",
				"decimals": 18
			},
			"0x98732af1625958ec56406e87747aa68ad1cee1c0": {
				"ticker": "SN",
				"address": "0x98732af1625958ec56406e87747Aa68Ad1cEE1C0",
				"name": "Supernova DAO",
				"decimals": 18
			},
			"0x727cc1cefe1965023fc3cf493636a1345a7731d4": {
				"ticker": "NOBELIUM",
				"address": "0x727Cc1cEFe1965023Fc3Cf493636A1345A7731d4",
				"name": "Nobelium DAO",
				"decimals": 9
			},
			"0xd56ab2cb889fdc6909c51c91cb99c0e64fd673a4": {
				"ticker": "RYU",
				"address": "0xD56Ab2Cb889fdc6909c51C91cb99c0E64fD673A4",
				"name": "RYU",
				"decimals": 18
			},
			"0x8ae9e51fba4dbe2ce55e9f57f4fc4f891822471e": {
				"ticker": "WMAVX",
				"address": "0x8Ae9E51FBa4dBe2CE55e9f57f4FC4F891822471e",
				"name": "WoolfmoonAVX",
				"decimals": 6
			},
			"0x78160ac6a43fee46098b6124b3eb9b1a85f099ac": {
				"ticker": "Nebul",
				"address": "0x78160ac6a43FEe46098B6124B3eb9b1A85f099Ac",
				"name": "Nebula Cloud",
				"decimals": 8
			},
			"0xd7075324370489288146ef24572b5b4ba98bc446": {
				"ticker": "JABB",
				"address": "0xd7075324370489288146Ef24572B5B4bA98BC446",
				"name": "JABBA DAO",
				"decimals": 8
			},
			"0x4ecab85f0b4803b55564993aa2abb62fc2d536e1": {
				"ticker": "AVT",
				"address": "0x4ecaB85f0b4803b55564993aa2aBB62Fc2d536e1",
				"name": "AvaxTrain https://t.me/AvaxTrain",
				"decimals": 9
			},
			"0xa6d5ff588f653ea4724cab5f451ed75a501e3314": {
				"ticker": "JEWELp",
				"address": "0xA6d5FF588F653eA4724CAb5f451Ed75a501e3314",
				"name": "JEWEL PRINTER",
				"decimals": 18
			},
			"0x021d18d1ce4ade415051eadd390b46ea82fe6dbc": {
				"ticker": "DRGN",
				"address": "0x021d18d1ce4aDE415051eaDd390B46eA82fE6dbC",
				"name": "Dragon Nodes",
				"decimals": 18
			},
			"0x7909787fb9a91643a5e07f41bbc3ad49503acc14": {
				"ticker": "NDCP",
				"address": "0x7909787fB9a91643a5e07f41BbC3ad49503ACC14",
				"name": "NODAC Printer",
				"decimals": 9
			},
			"0x978b59dc43eabd4ae104e7a0685d8fcc36fbf13a": {
				"ticker": "AoB",
				"address": "0x978b59DC43eaBd4ae104E7a0685d8fCC36fBf13a",
				"name": "AVAXoverBTC",
				"decimals": 9
			},
			"0x916f603e84caf434b00ef4c6ce93f958ff1a1716": {
				"ticker": "RUN",
				"address": "0x916F603E84caF434B00EF4C6Ce93F958FF1A1716",
				"name": "RunNode",
				"decimals": 9
			},
			"0x66c8f0904c25dfa413a243424d3a5008538844e9": {
				"ticker": "SatAVAXtion",
				"address": "0x66c8F0904c25DFA413a243424d3a5008538844e9",
				"name": "SatAVAXtion",
				"decimals": 6
			},
			"0xb9da81e55c0aaa37a0212e24bb00949108d38db0": {
				"ticker": "SOMDS",
				"address": "0xB9DA81E55C0aAA37A0212e24bB00949108D38dB0",
				"name": "SonOfMidas",
				"decimals": 9
			},
			"0xa99c4e3f629a633e64b1b62728c31d7bd547e2a3": {
				"ticker": "AwardNode",
				"address": "0xa99c4e3F629a633E64b1b62728c31D7bD547e2A3",
				"name": "AwardNode",
				"decimals": 8
			},
			"0x2b5e770b2d08646f386e28dcce2f4c8fd6d6f1be": {
				"ticker": "UNIVP",
				"address": "0x2B5e770B2d08646F386e28dcCe2f4C8fd6D6f1Be",
				"name": "Universe Printer",
				"decimals": 6
			},
			"0x9bff351eaf8d116bca2f936559d60b8a7b482e43": {
				"ticker": "PHOENIX",
				"address": "0x9BFF351eaF8d116BcA2F936559D60b8A7B482E43",
				"name": "phoenix.finance",
				"decimals": 18
			},
			"0x4d0149969d7b4f906705ec89f13b56f9a87341ba": {
				"ticker": "APE",
				"address": "0x4d0149969d7b4F906705Ec89F13b56F9a87341BA",
				"name": "APE NODES",
				"decimals": 9
			},
			"0x14614a08e8a95eb645bd9c8b0dd5819fc0bb848b": {
				"ticker": "MTA",
				"address": "0x14614A08E8a95eb645Bd9c8b0dd5819FC0bb848b",
				"name": "MetAvax",
				"decimals": 8
			},
			"0x7074816687cc2c387d0ba30dc0ab515fb5716adc": {
				"ticker": "ISLAND",
				"address": "0x7074816687CC2c387d0Ba30dc0ab515FB5716Adc",
				"name": "Avalanche Island",
				"decimals": 18
			},
			"0xcdde7ec4568dbff0aa4cea49edbc7c6285629390": {
				"ticker": "Made",
				"address": "0xCDde7EC4568dbff0aA4cEA49edbc7C6285629390",
				"name": "Meta Made",
				"decimals": 18
			},
			"0x08fc454d3e818c173321c77f667d249418f24050": {
				"ticker": "ALAUNCH",
				"address": "0x08fC454d3E818c173321c77f667D249418F24050",
				"name": "AvaxLaunch",
				"decimals": 18
			},
			"0x8783aecd4391a06fb681f52198aae313de7307bd": {
				"ticker": "JCI",
				"address": "0x8783aEcD4391A06Fb681f52198Aae313De7307Bd",
				"name": "JurassicInu",
				"decimals": 6
			},
			"0x1a96a0d8a427e6172a2f799a2593fed0f810eef7": {
				"ticker": "Crypt",
				"address": "0x1a96A0D8a427e6172a2f799A2593FeD0f810eef7",
				"name": "Crypto Lab",
				"decimals": 8
			},
			"0x89ec31f55c0155223fb383ddfe574654052a3286": {
				"ticker": "Entrop",
				"address": "0x89eC31f55C0155223fB383dDFE574654052a3286",
				"name": "Entropy Protocol",
				"decimals": 8
			},
			"0x4278c623a84050d582894aca5b7ff790b95d284d": {
				"ticker": "Unicor",
				"address": "0x4278c623a84050D582894ACA5b7Ff790b95D284d",
				"name": "Unicorn Protocol",
				"decimals": 8
			},
			"0x9c4100b9d1d9f1cfd9f0125e4d739f8841dceb7e": {
				"ticker": "CDC",
				"address": "0x9c4100b9d1D9f1CFd9f0125E4D739F8841DCEb7E",
				"name": "Cross DAO Capital",
				"decimals": 18
			},
			"0x03afdfc8f69355f6f7fbabff5768538c3db951de": {
				"ticker": "GSI",
				"address": "0x03afdFC8F69355f6F7FBAbFF5768538C3dB951De",
				"name": "Galactic Shiba Inu",
				"decimals": 18
			},
			"0x64a8273692b084d2031c6c83c4c72887da129c5d": {
				"ticker": "Littl",
				"address": "0x64a8273692B084D2031C6c83C4c72887Da129c5d",
				"name": "Little Wing",
				"decimals": 8
			},
			"0xe25a22aa7fc471c220c7cb2dc67875ccf153913d": {
				"ticker": "hel",
				"address": "0xe25A22aa7Fc471c220C7CB2DC67875Ccf153913D",
				"name": "HELLO",
				"decimals": 18
			},
			"0xc6d605d42936d9361900ac8987986dde654f395e": {
				"ticker": "APC",
				"address": "0xc6d605D42936d9361900Ac8987986DDE654F395E",
				"name": "AVAXPay Capital",
				"decimals": 9
			},
			"0xcad0e10df547baa0a01aa09c6a399b602b6646da": {
				"ticker": "EXODUS",
				"address": "0xcaD0e10df547bAa0a01aA09c6A399b602B6646DA",
				"name": " MetaMusk",
				"decimals": 8
			},
			"0x182a24f0754b7987d48b50995b85195f4a0d7b8f": {
				"ticker": "GRIM",
				"address": "0x182A24F0754b7987d48B50995B85195f4A0D7b8F",
				"name": "EXODUS",
				"decimals": 9
			},
			"0x0423c774c3370f215c19600eb35fc1a5c19e5847": {
				"ticker": "UNIVP",
				"address": "0x0423c774C3370F215C19600eb35Fc1a5C19E5847",
				"name": "Grimace Coin",
				"decimals": 18
			},
			"0x9fbcb764a5531ba18cb3292d924ee4aec87f038f": {
				"ticker": "DOGEAVAX",
				"address": "0x9fbCB764a5531bA18CB3292d924Ee4Aec87F038F",
				"name": " AggregatedFinance",
				"decimals": 6
			},
			"0xb3c62b4275eed00f053d14f8bb3c70c889dbfcd9": {
				"ticker": "BUM",
				"address": "0xB3C62B4275eeD00f053d14f8bB3C70c889DbFCD9",
				"name": "Univ Printer",
				"decimals": 9
			},
			"0x75b062172937776e819868eff8a3d92ef5a36bb2": {
				"ticker": "Hydroge",
				"address": "0x75B062172937776e819868eFF8a3d92Ef5a36Bb2",
				"name": "DogeAvaxWeb3",
				"decimals": 8
			},
			"0x0b2977864eae97eb3860ba2137199b957aac7464": {
				"ticker": "TVL",
				"address": "0x0B2977864EAE97eB3860Ba2137199B957aac7464",
				"name": "Beach Bum DAO",
				"decimals": 9
			},
			"0xfa1155e236b097a69246841195c214648d027296": {
				"ticker": "WAWE",
				"address": "0xFa1155E236B097A69246841195c214648d027296",
				"name": "Hydrogen Chain",
				"decimals": 18
			},
			"0x1db82d5902b30f3055441e2a1c0cfef95ea71d30": {
				"ticker": "AVOGEP",
				"address": "0x1DB82d5902b30f3055441E2A1C0CFef95ea71d30",
				"name": "Travel Coin",
				"decimals": 6
			},
			"0x5d6fe6cda89e0503874056797a716d414261c1c3": {
				"ticker": "APECAP",
				"address": "0x5D6Fe6cda89E0503874056797A716d414261c1c3",
				"name": "WavesMoney",
				"decimals": 18
			},
			"0x92173de1e12f4e30d29e06029c64a31750d6a774": {
				"ticker": "PEAK",
				"address": "0x92173DE1e12F4E30d29E06029c64A31750d6a774",
				"name": "AVOGEPRINTER",
				"decimals": 18
			},
			"0x921d84cde32ace7a3ef4aada379c1c6df047a23a": {
				"ticker": "MVERSE",
				"address": "0x921d84Cde32AcE7a3EF4aAda379c1c6df047a23A",
				"name": "APE CAPITAL",
				"decimals": 18
			},
			"0xbf3cc45d6161588b73da617c8b64635d6d48ade8": {
				"ticker": "JINU",
				"address": "0xbF3CC45D6161588B73da617C8b64635d6D48adE8",
				"name": "Summit DAO",
				"decimals": 18
			},
			"0x394728d3f05e390489462fbb0c1a297409b6724c": {
				"ticker": "KSHARE",
				"address": "0x394728D3f05e390489462fbB0c1a297409b6724C",
				"name": "MITAVERSE",
				"decimals": 18
			},
			"0xf808b3049d1d3ee2ec5b1dbb6d947b81d8ca2651": {
				"ticker": "VUSDC",
				"address": "0xF808B3049D1D3ee2ec5B1DBB6D947b81d8ca2651",
				"name": "JoeInu",
				"decimals": 6
			},
			"0x20c2b81ad7a3161356fac4bf7718ece137b52d5e": {
				"ticker": "THANOSAX",
				"address": "0x20c2b81Ad7A3161356fAC4bF7718Ece137B52D5e",
				"name": "Kush Share",
				"decimals": 6
			},
			"0xd254a3cd6b8f627c010f25dd4832804ba1c4d29f": {
				"ticker": "PolarBear",
				"address": "0xD254A3CD6B8f627c010F25Dd4832804BA1c4D29F",
				"name": "Valentine USDC",
				"decimals": 6
			},
			"0x2130347de53a6c7fc6a036d2308f5c033852df37": {
				"ticker": "LAN",
				"address": "0x2130347De53A6C7fc6a036D2308F5C033852Df37",
				"name": "THANOSAVAX",
				"decimals": 18
			},
			"0xe3c12f93ab3e1d2eb640b051236098cd4f0cea98": {
				"ticker": "GPILL",
				"address": "0xe3C12F93Ab3e1d2EB640b051236098Cd4f0CeA98",
				"name": "PolarBear",
				"decimals": 1
			},
			"0x4d0c0c67627c9a0205ec039bd68d32f93d792332": {
				"ticker": "XTAG",
				"address": "0x4D0c0C67627C9A0205Ec039bD68D32F93D792332",
				"name": "LANDMARK",
				"decimals": 6
			},
			"0x907fc56e057607abb8af9261e5714d4c0a726a63": {
				"ticker": "Iren",
				"address": "0x907Fc56e057607abb8af9261e5714d4C0A726A63",
				"name": "GREEN PILL DAO",
				"decimals": 9
			},
			"0x8afa62fa8dde8888405c899d7da077a61a87eed3": {
				"ticker": "LIBRE",
				"address": "0x8afa62Fa8DdE8888405c899D7Da077A61a87EeD3",
				"name": "XTAG Token",
				"decimals": 18
			},
			"0x15c841043e13ffaa9a99fabea236d40f45615623": {
				"ticker": "BUSINESSES",
				"address": "0x15c841043e13fFAA9a99FabEa236D40F45615623",
				"name": "Irene DAO Capital",
				"decimals": 18
			},
			"0xcf0266f167da13ab63776cfac70331d41c9518c6": {
				"ticker": "ASTRA",
				"address": "0xCF0266F167da13ab63776CFac70331d41C9518C6",
				"name": "Libre",
				"decimals": 8
			},
			"0x291649704706258e760fc5a2a72d1b92f9555ff3": {
				"ticker": "CeloStarter",
				"address": "0x291649704706258E760FC5a2a72d1B92f9555fF3",
				"name": "$BUSINESSES",
				"decimals": 2
			},
			"0x0e8f778666f5104bf8fb7ba0a569d544910cdcdd": {
				"ticker": "CMT",
				"address": "0x0e8F778666f5104bF8FB7bA0A569d544910cDcDd",
				"name": "Astra Protocol",
				"decimals": 18
			},
			"0xe8681415c9ba7084cbfe55684cd4e7f45d934925": {
				"ticker": "CRL",
				"address": "0xe8681415c9bA7084cBFE55684cd4e7F45d934925",
				"name": "CeloStarter",
				"decimals": 1
			},
			"0xbc2b87311934cb0bf60fddc9738868ebe00dc189": {
				"ticker": "GOL",
				"address": "0xBC2b87311934CB0bf60fdDc9738868EBe00Dc189",
				"name": "COMMUNITY TOKEN",
				"decimals": 1
			},
			"0x2d67c42c491ee11bbf292ec8a9a14d74b6020662": {
				"ticker": "GREEN",
				"address": "0x2D67C42c491EE11BBF292eC8a9a14d74b6020662",
				"name": "Circle Nodes",
				"decimals": 18
			},
			"0x4501e37a79cf953116331a105b3644d07b74c0f8": {
				"ticker": "SafeMoonA",
				"address": "0x4501e37a79cf953116331a105B3644d07b74C0F8",
				"name": "GOLD MINE",
				"decimals": 9
			},
			"0x1afcfdf55745fe0cea027e1c608577e6eccf6524": {
				"ticker": "ShibaJordan",
				"address": "0x1AFCfdf55745fE0cEA027e1C608577e6eCCF6524",
				"name": "Green DAO",
				"decimals": 6
			},
			"0x322973ae4742398e4f80312be760cfcd98f0abad": {
				"ticker": "GRC",
				"address": "0x322973ae4742398e4f80312bE760cFcD98f0aBad",
				"name": "SafeMoonAvax",
				"decimals": 9
			},
			"0x9a1c916b689aac6e0bf95c7b1e930b819e2cea78": {
				"ticker": "Flokinobi",
				"address": "0x9a1c916b689Aac6E0Bf95C7B1E930b819E2cea78",
				"name": "ShibaJordan",
				"decimals": 9
			},
			"0xf595dd02840a06962a4645b80b00128436a90d3a": {
				"ticker": "SN",
				"address": "0xF595Dd02840A06962A4645b80b00128436a90D3A",
				"name": "GreenCandle",
				"decimals": 5
			},
			"0x6211ae2d1e1410bc74449bf31e6d094baedb2edf": {
				"ticker": "YouPick",
				"address": "0x6211Ae2D1E1410bC74449bf31E6D094BAEdb2EDf",
				"name": "Flokinobi",
				"decimals": 18
			},
			"0x42da02a683791fb3e6840dcf6d0483793ab04b6e": {
				"ticker": "NODEC",
				"address": "0x42da02a683791Fb3e6840DCF6D0483793ab04B6e",
				"name": "Stellar Nodes",
				"decimals": 18
			},
			"0x76e6842c3c3e55470100db42f18ce6de077d303a": {
				"ticker": "Inferno",
				"address": "0x76e6842C3c3e55470100db42F18cE6de077d303a",
				"name": "NowPrinting: LINK",
				"decimals": 8
			},
			"0x7fa9507c54b4aa11518859df9928629af62963dc": {
				"ticker": "AR",
				"address": "0x7Fa9507c54B4Aa11518859dF9928629Af62963DC",
				"name": "Node Compiler Avax",
				"decimals": 18
			},
			"0xb34cb278a88cdbe392e336e99d9a3c27b1dd1ff6": {
				"ticker": "RGK",
				"address": "0xB34cB278a88cdbe392E336e99D9A3c27B1DD1FF6",
				"name": "Inferno",
				"decimals": 18
			},
			"0x471c06095bd4b983b91997e3afc886c9b2cc25da": {
				"ticker": "Pulsa",
				"address": "0x471c06095bD4B983B91997e3AFC886c9b2CC25Da",
				"name": "Arsenic Coin",
				"decimals": 8
			},
			"0x046e0c72867c91b00ae4ac244db125e24efa747b": {
				"ticker": "TMB",
				"address": "0x046e0c72867C91b00ae4aC244Db125E24Efa747B",
				"name": "Ragnarok DAO ragnarokdao.finance",
				"decimals": 9
			},
			"0xbd68147bb07e2e1877b6f62df4ded66d445efe2c": {
				"ticker": "Crypt",
				"address": "0xBd68147BB07E2e1877b6F62DF4DEd66D445Efe2c",
				"name": "Pulsar DAO",
				"decimals": 4
			},
			"0xfe2c794925cf188150c4a019675d91a009dd22ce": {
				"ticker": "RISEN",
				"address": "0xFE2C794925cf188150C4a019675d91a009dd22cE",
				"name": "TimeBank",
				"decimals": 18
			},
			"0x8480454359067529449e027d2d36588a1efcb4c1": {
				"ticker": "DFLOKI",
				"address": "0x8480454359067529449e027D2d36588a1EFcB4c1",
				"name": "Crypto Force",
				"decimals": 9
			},
			"0x267bc7f31be35535d45b27593be3a4c622f9b842": {
				"ticker": "GLAC",
				"address": "0x267Bc7F31be35535D45b27593Be3a4c622F9B842",
				"name": "NodesRise",
				"decimals": 8
			},
			"0x559f8732d466d75c305382b10725faa288cf7e5a": {
				"ticker": "ALF",
				"address": "0x559F8732d466d75C305382B10725fAa288cf7e5A",
				"name": "DrunkFloki",
				"decimals": 8
			},
			"0x746fa1d1b95f4bbce49b1ebc447a25d8017ef51f": {
				"ticker": "EGM",
				"address": "0x746Fa1d1b95f4BBCE49B1ebC447A25d8017EF51f",
				"name": "GLACIER",
				"decimals": 18
			},
			"0xe79d17977a080141d1813dc9a7e04252385c52bb": {
				"ticker": "SLEO",
				"address": "0xe79D17977A080141D1813DC9a7E04252385C52bb",
				"name": "AliceFrog",
				"decimals": 9
			},
			"0xf481eec738c46f675e077ee966a680a19210af11": {
				"ticker": "BLANC",
				"address": "0xf481Eec738C46F675e077ee966A680a19210Af11",
				"name": "EnigmaDAO ",
				"decimals": 9
			},
			"0x3509dd12cb86cef96031daa9f95e9440f6d59ca5": {
				"ticker": "INF",
				"address": "0x3509dd12cb86CeF96031DAa9F95E9440f6D59ca5",
				"name": "Snow Leopard",
				"decimals": 8
			},
			"0x06ba3e871ae3c14026d3e8a819fb3b5f31fef1d3": {
				"ticker": "XTAG",
				"address": "0x06BA3e871ae3C14026d3E8a819fb3B5f31FeF1d3",
				"name": "Blanc",
				"decimals": 6
			},
			"0x6cae7bd38abc3e7f48644c7f9549c6010fa40708": {
				"ticker": "MMA",
				"address": "0x6cAE7Bd38Abc3E7F48644c7f9549C6010fa40708",
				"name": "Infinity Verse",
				"decimals": 18
			},
			"0xf6f5010d8245125c86e68bebcf32649d03d2c10f": {
				"ticker": "EEVEE",
				"address": "0xF6f5010D8245125c86e68beBcf32649D03D2c10f",
				"name": "XTAG Token",
				"decimals": 18
			},
			"0x8c206825582af7a6ba24627f2a8b072e27ffb0fd": {
				"ticker": "AstroComet",
				"address": "0x8c206825582AF7a6ba24627F2A8b072E27FFB0FD",
				"name": "Mad Meta Avax",
				"decimals": 9
			},
			"0x6f3472de23cecade2b0e87d74e3714a8575c8719": {
				"ticker": "Aqua",
				"address": "0x6f3472DE23CECade2b0e87d74E3714a8575C8719",
				"name": "Eevee Nodes",
				"decimals": 5
			},
			"0x7db3e766acc6cfecc2942af82b2d521734108bf4": {
				"ticker": "MPC",
				"address": "0x7dB3e766acc6CfeCc2942af82b2d521734108bF4",
				"name": "AstroComet",
				"decimals": 18
			},
			"0x8ea5f33e716c2f84007719ad2b3f82ce887d6e9a": {
				"ticker": "DAIPPV2",
				"address": "0x8eA5f33E716c2f84007719ad2b3F82Ce887d6E9A",
				"name": "Aquarius",
				"decimals": 6
			},
			"0x4cfc40541a88321b844ea3f111edf3489cfbd057": {
				"ticker": "Kimch",
				"address": "0x4CFc40541A88321b844Ea3f111edf3489CfBD057",
				"name": "Moon Pump Coin",
				"decimals": 8
			},
			"0x772cbd84e3e9402c18db959700bc76a9eca7a6d8": {
				"ticker": "JustUpAvax",
				"address": "0x772cBD84e3e9402c18DB959700Bc76a9Eca7a6D8",
				"name": "DAIPPV2",
				"decimals": 18
			},
			"0xf9a49321d3d34cf94c4abd1957c219572a646692": {
				"ticker": "FAVAX",
				"address": "0xf9A49321D3D34CF94c4AbD1957C219572A646692",
				"name": "Kimchi Inu",
				"decimals": 18
			},
			"0xce5bd2edcc6ee9dc9b017af7b3fe12b6d1a73473": {
				"ticker": "STF",
				"address": "0xce5bD2EDCC6ee9DC9B017af7b3fe12b6D1A73473",
				"name": "JustUpAvax",
				"decimals": 18
			},
			"0xf3f313bf563a9bc19909ad2effed365c5516a789": {
				"ticker": "MIM.p",
				"address": "0xf3F313bf563A9Bc19909AD2EfFED365C5516a789",
				"name": "MIM Printer",
				"decimals": 9
			},
			"0x892bc99cb162f558eefbd3a87cb05836847c415a": {
				"ticker": "JEWELPRINT",
				"address": "0x892BC99Cb162F558EeFbD3a87cB05836847C415a",
				"name": "JEWEL PRINTER",
				"decimals": 18
			},
			"0xe5dc147df07d6e143e1ad66ad3cc5b8aa7d52856": {
				"ticker": "ETHP100X",
				"address": "0xE5dc147df07d6e143E1Ad66aD3cC5B8Aa7d52856",
				"name": "ETHP100X",
				"decimals": 6
			},
			"0xc6c246c129007fb1d08d9a9459ed3abd6d64909d": {
				"ticker": "SANTF",
				"address": "0xc6C246C129007fB1d08d9A9459ed3aBd6d64909d",
				"name": "SANFA FROG",
				"decimals": 9
			},
			"0xec8e93ab0fdc0201e9c2515b5a60adbd048f7c1a": {
				"ticker": "VPND",
				"address": "0xeC8e93Ab0FDC0201E9c2515b5A60AdbD048F7C1A",
				"name": "Vapor Nodes",
				"decimals": 1
			},
			"0x0bfec5566c719a4361c634ed18f43bd684e1df62": {
				"ticker": "WINAVAX",
				"address": "0x0bFec5566C719a4361C634Ed18F43bd684e1df62",
				"name": "Winavax",
				"decimals": 18
			},
			"0xcaa3f1488b1550e217f0275b4c79bebee292add0": {
				"ticker": "PEBBLE",
				"address": "0xcAA3f1488B1550E217F0275B4C79BEbEE292add0",
				"name": "PEBBLE",
				"decimals": 18
			},
			"0x9e394558dcc330e1603d3c6f45474df4a0ca3452": {
				"ticker": "FRN",
				"address": "0x9e394558DcC330e1603d3c6f45474Df4A0CA3452",
				"name": "FrontRunner",
				"decimals": 9
			},
			"0xeacffab53713b6a56037b6a49135189d85c736d7": {
				"ticker": "SAIYAN",
				"address": "0xeAcFfAb53713B6A56037B6a49135189D85C736d7",
				"name": "SaiyanInu",
				"decimals": 6
			},
			"0xd9ec17a55f33750ec1873442427b78cded829d62": {
				"ticker": "EAG",
				"address": "0xd9EC17a55f33750ec1873442427b78CDED829d62",
				"name": "EagleAvax",
				"decimals": 9
			},
			"0x3cbe09af08903c7f0144287862a5bba0da049980": {
				"ticker": "VOLC",
				"address": "0x3cBe09AF08903c7F0144287862A5BBA0dA049980",
				"name": "VolcanoDAO Finance",
				"decimals": 9
			},
			"0x63bcdff587f599ded37b6c43fb02bb4c6d19af34": {
				"ticker": "HWG",
				"address": "0x63BcDff587f599DEd37b6c43fb02BB4C6D19aF34",
				"name": "HallowenGame",
				"decimals": 18
			},
			"0xad0f9e7189ab2b50d332dd114e2be23c00425e1e": {
				"ticker": "Spel",
				"address": "0xaD0F9e7189Ab2b50D332DD114e2BE23c00425e1e",
				"name": "Spell Printer",
				"decimals": 18
			},
			"0x46bbe6c673c49ca532fae30ad6b95304b51b379d": {
				"ticker": "COSMOS",
				"address": "0x46BbE6c673C49Ca532Fae30aD6B95304b51B379D",
				"name": "Cosmos",
				"decimals": 18
			},
			"0x1c34ecd1e49b1b178554b7bbdad9064396bd3ca9": {
				"ticker": "PLAYMATES",
				"address": "0x1C34ECD1e49B1B178554b7BBDaD9064396bD3Ca9",
				"name": "PLAYMATES",
				"decimals": 6
			},
			"0xb8867f56eae743509b0e96db49d45c0d9a0ec532": {
				"ticker": "PLAYMATES",
				"address": "0xB8867f56Eae743509b0E96dB49D45c0D9A0Ec532",
				"name": "Redlight Node District",
				"decimals": 18
			},
			"0x0b7569e1d181b4c8f3f32df2f2fe1975942a247d": {
				"ticker": "UNIVPrint",
				"address": "0x0b7569E1D181b4C8F3F32DF2f2FE1975942a247D",
				"name": "UNIV Print",
				"decimals": 1
			},
			"0xa80785858edd40cddd8da366c1b7cc4602f87cf9": {
				"ticker": "CADCOIN",
				"address": "0xa80785858EdD40CDDd8dA366c1B7CC4602f87cf9",
				"name": "CadmiumCoin",
				"decimals": 18
			},
			"0x02b4e7b330adbaa0ef8b5d19accca5090e161995": {
				"ticker": "MOONSAFE",
				"address": "0x02B4e7b330aDbAa0ef8b5D19ACcCA5090E161995",
				"name": "MoonSafe",
				"decimals": 9
			},
			"0x8b1dc58266758c56f6032c444b6bc45fe08b9e29": {
				"ticker": "ASTRO",
				"address": "0x8B1DC58266758c56f6032C444b6Bc45Fe08B9E29",
				"name": "Astro Token",
				"decimals": 18
			},
			"0x42654814972855aa4ae6e8fbb803a6af52eb7e5b": {
				"ticker": "LOOTp",
				"address": "0x42654814972855aA4aE6E8fbb803A6AF52Eb7e5B",
				"name": "LootPrinter",
				"decimals": 18
			},
			"0xc68d9471f521b55270e9f8f3d13d3cd501dd9480": {
				"ticker": "CryptoPulse",
				"address": "0xc68d9471F521B55270e9F8f3D13D3CD501dd9480",
				"name": "CryptoPulse",
				"decimals": 8
			},
			"0x956d986aa1df3718e7c2d489128902f6f85b817c": {
				"ticker": "DGIV",
				"address": "0x956d986AA1Df3718E7C2D489128902F6F85b817C",
				"name": "Doggiverse",
				"decimals": 18
			},
			"0x414e270f62951ad005196c69385bc14256ccb7e0": {
				"ticker": "NOBEL",
				"address": "0x414E270F62951AD005196c69385bC14256cCB7E0",
				"name": "NobeliumDAO",
				"decimals": 5
			},
			"0xbdcf567ca6b70e98fbc0ba90c524733735dc9304": {
				"ticker": "OBS",
				"address": "0xbdCF567cA6B70e98fbC0ba90c524733735Dc9304",
				"name": "Obsidian Nodes",
				"decimals": 18
			},
			"0x9804330f3e7bf35741cdca9bab413b42887c1995": {
				"ticker": "MAGNET",
				"address": "0x9804330f3e7bF35741cdca9bAB413B42887c1995",
				"name": "MagnetDAO",
				"decimals": 18
			},
			"0x39ba8554348a6cdbcd02b5fc0b9ea507e54504e7": {
				"ticker": "BUNNY",
				"address": "0x39bA8554348a6cdbcd02b5Fc0b9EA507E54504e7",
				"name": "Snow Bunny",
				"decimals": 9
			},
			"0xd65dd699cd79844d703e0eb646da7cc35bbbde40": {
				"ticker": "santafrog",
				"address": "0xD65DD699cd79844d703e0eb646dA7cC35BBbDE40",
				"name": "santafrog token",
				"decimals": 18
			},
			"0x5b96b904fa2a218f83ba2cadcfb581386cbb00fa": {
				"ticker": "FADE",
				"address": "0x5b96B904FA2A218F83BA2cAdCFB581386cBb00fa",
				"name": "FADE",
				"decimals": 9
			},
			"0x4bd3f9722993225f9bb93eb569695cac4b55cc81": {
				"ticker": "ExoMoon",
				"address": "0x4Bd3F9722993225f9bb93EB569695CaC4B55cC81",
				"name": "ExoMoon",
				"decimals": 8
			},
			"0x89f54c184e942c303bd7abc3841e6a89f5529f4c": {
				"ticker": "PLAYMATES",
				"address": "0x89f54C184E942c303bD7abc3841E6A89F5529f4c",
				"name": "Redlight Node District",
				"decimals": 18
			},
			"0x819af35836161c96653824125a1ec795d7fec7c7": {
				"ticker": "GMI",
				"address": "0x819Af35836161C96653824125A1Ec795d7fEc7c7",
				"name": "Gonna Make It",
				"decimals": 18
			},
			"0x873c980589b7a0f2bb9c3725281b148f430db433": {
				"ticker": "AVAXOLA",
				"address": "0x873c980589b7A0F2Bb9C3725281b148F430dB433",
				"name": "AvaXola",
				"decimals": 6
			},
			"0x84924434dac348ed16e18dcd51fa30612905d113": {
				"ticker": "BUNNY",
				"address": "0x84924434daC348ed16e18DcD51fa30612905D113",
				"name": "Bunny Token",
				"decimals": 18
			},
			"0xf9060d2ff0e1f4bd3fe5362abb20bfa7314002bc": {
				"ticker": "HISS",
				"address": "0xf9060D2ff0e1F4bD3FE5362AbB20bFA7314002bC",
				"name": "Anaconda DAO",
				"decimals": 18
			},
			"0x3b55a7e0280ec0af1a5eb552ee0c2de9f785eae0": {
				"ticker": "Alliance",
				"address": "0x3B55A7E0280Ec0af1a5EB552eE0c2DE9f785EAe0",
				"name": "Alliance",
				"decimals": 18
			},
			"0xbb3d89994151e0b7c309ffff302cac7aacaf220a": {
				"ticker": "NFGT",
				"address": "0xbb3d89994151e0b7C309fFff302Cac7AacaF220a",
				"name": "Nationaity Financial Gate Token",
				"decimals": 9
			},
			"0x53298af840a59004818ae7f5a35c2bb7ea67e2d1": {
				"ticker": "ARAK",
				"address": "0x53298AF840a59004818Ae7F5a35C2Bb7Ea67E2D1",
				"name": "ArakNode",
				"decimals": 9
			},
			"0xa004e7cf33fcdd21a48d1fd4b87a5eff0704e26c": {
				"ticker": "TraderJew",
				"address": "0xA004E7CF33FCdD21a48D1Fd4B87a5eFf0704E26C",
				"name": "JAEL",
				"decimals": 9
			},
			"0x46fc439a3e9a5f51df6447d87bbd909bb9c2e86d": {
				"ticker": "AvaxRise",
				"address": "0x46Fc439a3e9a5F51df6447d87BbD909bB9C2e86d",
				"name": "AvaxRise",
				"decimals": 9
			},
			"0xdfc3b7fadf67580a9bebaecfd837c3986df55e0d": {
				"ticker": "KATANA",
				"address": "0xDfc3B7FADf67580A9BEBAecFD837C3986dF55e0d",
				"name": "Katana INU",
				"decimals": 18
			},
			"0xbd940e1def95a47f956ad080353b33bceb0a6cce": {
				"ticker": "WheatDAO",
				"address": "0xBD940E1dEF95a47f956AD080353B33bCeb0a6CcE",
				"name": "WheatDAO",
				"decimals": 18
			},
			"0xf96b4ebd4536a879ed9811b3fa17c6c5914269aa": {
				"ticker": "VLD",
				"address": "0xf96B4eBd4536a879ed9811b3fa17C6C5914269aa",
				"name": "ValidNode",
				"decimals": 18
			},
			"0x0175fe16455e24cb115f95fa6f00e1f6210e180f": {
				"ticker": "SNOWFLAKE",
				"address": "0x0175FE16455e24cb115f95Fa6F00E1f6210E180f",
				"name": "Snowflake Nodes",
				"decimals": 1
			},
			"0xecc5c2015a9a5320bcbb08ee445d2cf374f3ac14": {
				"ticker": "NRGY",
				"address": "0xEcC5C2015a9A5320bcBB08EE445D2Cf374F3AC14",
				"name": "ENERGY",
				"decimals": 18
			},
			"0xc29304a3bba80a2c9b03c90dad98863c528fde4e": {
				"ticker": "COBRA",
				"address": "0xC29304a3BBA80a2c9b03C90dad98863c528FDE4E",
				"name": "King Cobra",
				"decimals": 18
			},
			"0xe5f914fe28abdaa2a68f10a68e95d220e2460008": {
				"ticker": "GNYMD",
				"address": "0xe5f914Fe28abdaA2A68F10a68e95d220e2460008",
				"name": "Ganymede",
				"decimals": 18
			},
			"0x9b94c8ac3437b20141b79c55207e0c6b2df90e3d": {
				"ticker": "IMPR",
				"address": "0x9b94c8ac3437B20141b79C55207E0C6b2DF90e3D",
				"name": "Imperio DAO",
				"decimals": 9
			},
			"0x95f44ad0c55a2031facea10e7bcfa0666a5ccfb9": {
				"ticker": "StarLink",
				"address": "0x95f44ad0C55A2031fACEa10e7BCfA0666a5CCfb9",
				"name": "StarLink",
				"decimals": 6
			},
			"0xba9055223bcb155d940085358aad0345431bd9a7": {
				"ticker": "CPR",
				"address": "0xbA9055223bcB155D940085358AaD0345431bD9A7",
				"name": "Heart Attack Node",
				"decimals": 18
			},
			"0x225dcf6eaa2a981cfa7ef71177469f3bd14973cf": {
				"ticker": "alicefrog",
				"address": "0x225DCF6EAA2a981cFA7EF71177469f3BD14973CF",
				"name": "alicefrog",
				"decimals": 18
			},
			"0x14fdf65b955ed863fbef9b306b854fa3ef905920": {
				"ticker": "CREAM",
				"address": "0x14fDf65B955ed863Fbef9B306B854Fa3ef905920",
				"name": "IceCreamDAO",
				"decimals": 9
			},
			"0xa771af61fdc16608402ac49d09941f5d9dfd1f06": {
				"ticker": "UFO",
				"address": "0xA771aF61FDc16608402AC49D09941F5d9dFD1f06",
				"name": " DEVIENT NETWROK",
				"decimals": 8
			},
			"0x7839e200962c089022007b04c7191bea900e4422": {
				"ticker": "PID",
				"address": "0x7839e200962c089022007b04C7191bea900E4422",
				"name": "UFO City",
				"decimals": 18
			},
			"0x6fd7cd95a6628bcc4cd071809585de12f1c277ca": {
				"ticker": "CaesarEarn",
				"address": "0x6fD7cD95A6628bCC4cD071809585DE12F1C277ca",
				"name": "ProtocolxDao",
				"decimals": 6
			},
			"0x9eae74f3085760d3d2d3636a8d8fba259cbeae48": {
				"ticker": "LASERJET",
				"address": "0x9EaE74f3085760D3d2D3636a8D8FBa259cbeae48",
				"name": "CaesarEarn",
				"decimals": 8
			},
			"0x38b1dfd8d79ba724fea3982915fed6b2f6745082": {
				"ticker": "Ocean",
				"address": "0x38b1dfd8D79BA724fEa3982915fed6B2F6745082",
				"name": "LASERJET",
				"decimals": 9
			},
			"0x2d4f2acc5b50c0d7e0835cf01077fd2443e4fa6a": {
				"ticker": "AKT",
				"address": "0x2d4F2aCc5b50C0d7E0835Cf01077fD2443e4fa6a",
				"name": "TidesOfOcean",
				"decimals": 18
			},
			"0x9f3dcffc0a489a9ec4e01e313222d6485dcd60f8": {
				"ticker": "YFII",
				"address": "0x9f3dcfFc0A489A9ec4E01e313222D6485DCD60F8",
				"name": "Akita DAO",
				"decimals": 9
			},
			"0x622ccbca4b1f72184aa931832307c3ef114da237": {
				"ticker": "FAVAX",
				"address": "0x622Ccbca4B1f72184aA931832307C3Ef114Da237",
				"name": "DFI.MONEY",
				"decimals": 8
			},
			"0x4c035d63ba6912a2446496ba81c1ce124683de7b": {
				"ticker": "SOTM",
				"address": "0x4C035D63BA6912a2446496Ba81c1cE124683dE7b",
				"name": "FAVAX",
				"decimals": 18
			},
			"0x5e869c58e0034152246d50fc65a0d38d9def6cae": {
				"ticker": "KINGFLOKIAVAX",
				"address": "0x5E869c58e0034152246D50FC65A0d38D9dEF6Cae",
				"name": "SonOfTonyMontana.com",
				"decimals": 9
			},
			"0xd4876a51cb439af52e5f6d44484886bd68f858fa": {
				"ticker": "MONOR",
				"address": "0xD4876a51cB439Af52e5F6D44484886bd68F858FA",
				"name": "KINGFLOKIAVAX",
				"decimals": 18
			},
			"0x05ef4a6b089972aa663e3dcc3fe217e1cc6436d2": {
				"ticker": "KONG",
				"address": "0x05EF4A6b089972aA663E3dCc3Fe217e1cC6436d2",
				"name": "Himalaya DAO",
				"decimals": 18
			},
			"0xc9d76d4d2d4beefe507ab74336d027ca0522ef01": {
				"ticker": "AVACAT",
				"address": "0xc9D76D4D2D4BeEfe507aB74336D027ca0522ef01",
				"name": "Kong Finance",
				"decimals": 9
			},
			"0xfd62e169d405b7125393b0c9aa5cbedf26d743bf": {
				"ticker": "$eGreen",
				"address": "0xFd62e169D405b7125393b0C9AA5CbedF26d743bF",
				"name": "AvaCat",
				"decimals": 5
			},
			"0xd5526c618c29f94d69ed7c677dba7af32694f124": {
				"ticker": "TMT",
				"address": "0xd5526c618c29f94D69Ed7C677dbA7aF32694F124",
				"name": "Ebony Green",
				"decimals": 9
			},
			"0x70b907f3f7231be122100dac51c1995e6804e346": {
				"ticker": "TRT",
				"address": "0x70B907f3F7231bE122100dac51C1995e6804E346",
				"name": "TheMidasTouch",
				"decimals": 6
			},
			"0x6eece47a60f92d04746e11988972c801a9be7a1e": {
				"ticker": "PEPE",
				"address": "0x6EeCe47A60F92D04746e11988972c801A9be7a1E",
				"name": "True Reflect",
				"decimals": 9
			},
			"0x058fab34ffa60ed9b2f17046dd4d80797f406346": {
				"ticker": "NOBEL",
				"address": "0x058fab34Ffa60eD9B2f17046DD4d80797F406346",
				"name": "Pepe",
				"decimals": 5
			},
			"0x0b1df14aacb65021b5f93f0a26f4bb9ba81542a7": {
				"ticker": "CryptoHunter",
				"address": "0x0B1Df14Aacb65021b5f93F0a26f4Bb9ba81542A7",
				"name": "NobeliumDAO",
				"decimals": 8
			},
			"0x527bd013ef55b1d85fa7482763992e38b5a65de0": {
				"ticker": "YO",
				"address": "0x527Bd013Ef55b1d85fA7482763992e38B5a65De0",
				"name": "CryptoHunters Game",
				"decimals": 18
			},
			"0x527a7f64cce75e0d761d0c736343e14e153b209e": {
				"ticker": "JTP",
				"address": "0x527A7F64CCe75E0D761D0C736343E14E153b209E",
				"name": "Yo Token ERC20 yobit.net/en/trade/YO/BTC",
				"decimals": 18
			},
			"0x26b8f1fd8b9081ebd0b5e2458df4dcbac4e7e217": {
				"ticker": "MAL",
				"address": "0x26B8f1FD8b9081ebd0B5e2458DF4dCBac4e7E217",
				"name": "Joe The Printer",
				"decimals": 9
			},
			"0x6d93afbe7a8c6d004145eda4fc6ee31f2464c0f6": {
				"ticker": "OHMP",
				"address": "0x6D93aFbE7a8C6D004145EDA4fc6EE31F2464c0F6",
				"name": "MagMaLamp",
				"decimals": 9
			},
			"0x790f803527180fc49e70101446af73ba7d08dda3": {
				"ticker": "Crypt",
				"address": "0x790F803527180fC49e70101446af73bA7d08DdA3",
				"name": "olymplus",
				"decimals": 8
			},
			"0x5713972ecfd1f0233daeedf1bc35773302b44d6e": {
				"ticker": "ELONSTAX",
				"address": "0x5713972ECfd1f0233dAeeDf1BC35773302B44D6E",
				"name": "Crypto Prince",
				"decimals": 18
			},
			"0x353fce2fa74eea764dfbd8ad4e663e9c1ea6de8c": {
				"ticker": "ABEAR",
				"address": "0x353Fce2fa74eea764dfBd8AD4e663E9C1ea6dE8c",
				"name": "ELONSTAX",
				"decimals": 9
			},
			"0x6a37cc89a580d4cab6ea53e2820a622ed1a7bd21": {
				"ticker": "CLD",
				"address": "0x6A37cc89A580D4CAb6Ea53E2820a622ED1A7Bd21",
				"name": "Ava Bear",
				"decimals": 9
			},
			"0x523d011b9d709a116309a2f6e8a68c1241b4eea5": {
				"ticker": "PROTON",
				"address": "0x523d011B9D709A116309a2f6E8A68C1241B4eEA5",
				"name": "Candyland DAO",
				"decimals": 8
			},
			"0x55ae921c082b65bd8c9266dca4cd705e9b41121a": {
				"ticker": "LYRAN",
				"address": "0x55AE921C082b65BD8c9266dcA4CD705e9b41121a",
				"name": "PROTON",
				"decimals": 18
			},
			"0x781c9ca04c0938b981b3383653ab186961ea9084": {
				"ticker": "KIKI",
				"address": "0x781c9cA04c0938b981B3383653Ab186961eA9084",
				"name": "LYRAN PROTOCOL",
				"decimals": 18
			},
			"0x5a0daf69e103f1e3c757a02fa4894d6ad4f407ee": {
				"ticker": "SHINAVAX",
				"address": "0x5A0DAF69E103F1e3C757a02FA4894d6aD4F407EE",
				"name": "KIKI",
				"decimals": 18
			},
			"0x25995c756961715a6aaa6942be0b7d667feb9bb0": {
				"ticker": "XRING",
				"address": "0x25995C756961715a6aAa6942BE0b7d667Feb9Bb0",
				"name": "Shinavax",
				"decimals": 18
			},
			"0x7a2fc1f6955888e161c3af68ee685e706b51a7ac": {
				"ticker": "KITE",
				"address": "0x7A2Fc1F6955888E161c3af68ee685E706B51a7AC",
				"name": "XRING",
				"decimals": 6
			},
			"0xe16c7ae4b50ed6cd0eb0a63dd74700f479020332": {
				"ticker": "Supe",
				"address": "0xE16c7ae4b50ED6cD0eb0a63DD74700f479020332",
				"name": "KITE420",
				"decimals": 8
			},
			"0x507ec83365e8ddcc27befa983703f58c88501a5e": {
				"ticker": "BAYC",
				"address": "0x507EC83365e8DDcC27befA983703F58C88501a5E",
				"name": "Super Power",
				"decimals": 18
			},
			"0x171d3b933db91832cccba14f90173db61f0d2903": {
				"ticker": "TBOT",
				"address": "0x171d3B933dB91832CCCBA14f90173Db61f0D2903",
				"name": "BORED APE YAK CLUB",
				"decimals": 8
			},
			"0x4aabebd44e434d9fe93dbb738749a1391790b079": {
				"ticker": "VPND",
				"address": "0x4aaBEBD44e434d9fE93dBB738749a1391790B079",
				"name": "TRANSFER BOT",
				"decimals": 1
			},
			"0x7bcc78331fbf1ceb11e133876178c05c4406ec64": {
				"ticker": "RGK",
				"address": "0x7bCC78331FBf1Ceb11e133876178c05C4406EC64",
				"name": "VAPOR NODES",
				"decimals": 18
			},
			"0x3c28bc62af6848fa3731e2ee89e359e51a58eb45": {
				"ticker": "THUGDOGE",
				"address": "0x3C28Bc62Af6848fa3731E2EE89e359e51a58EB45",
				"name": "Ragnarok DAO ragnarokdao.finance",
				"decimals": 18
			},
			"0x21d58ffa306958a126f1618537fcc299c84566cc": {
				"ticker": "WARN",
				"address": "0x21D58ffa306958a126f1618537FCc299c84566cc",
				"name": "THUG DOGE",
				"decimals": 18
			},
			"0xb1eaab3b5e1045488817bdd91d57fbab0000ed1a": {
				"ticker": "WAVE",
				"address": "0xB1eaab3B5e1045488817bdd91D57FbAB0000ED1A",
				"name": "WarNode",
				"decimals": 18
			},
			"0x26adfdf03c8a48131fa4621d99e40fd8a5a68551": {
				"ticker": "REINDEER",
				"address": "0x26ADfdf03C8A48131fA4621D99E40fD8A5a68551",
				"name": "WAVES MONEY",
				"decimals": 18
			},
			"0xde6c7e5675869daa122a9922a207869c30446bdc": {
				"ticker": "Infinit",
				"address": "0xDE6C7E5675869daA122a9922a207869C30446BDC",
				"name": "Reindeer DAO",
				"decimals": 8
			},
			"0x68f7f2ce33d21b11166e38c3f96c376a521ff172": {
				"ticker": "DogNodes",
				"address": "0x68f7f2ce33D21b11166E38C3f96c376a521ff172",
				"name": "Infinity DAO",
				"decimals": 18
			},
			"0x3a98aeba6ace46bfaf6e0321dd08b12653b76444": {
				"ticker": "KxK",
				"address": "0x3a98aebA6AcE46BFaf6E0321dd08b12653B76444",
				"name": "DogNodes",
				"decimals": 6
			},
			"0xc5ebceb59ba60a34deb67c40423c2fd9cb106d3e": {
				"ticker": "KINGDOGE",
				"address": "0xc5ebCEB59ba60a34dEb67c40423C2FD9cb106D3E",
				"name": "KONGxKING",
				"decimals": 9
			},
			"0xa1afcc973d44ce1c65a21d9e644cb82489d26503": {
				"ticker":"RUX",
				"address":"0xa1afcc973d44ce1c65a21d9e644cb82489d26503",
				"name":"RunBlox Token",
				"decimals": 18
			}
		}"#,
	).unwrap()
}
