extern crate elements;
extern crate bitcoin;
extern crate rand;


use elements::{Transaction, TxOutWitness};
use elements::{secp256k1_zkp};

use elements::{AssetId, confidential, TxOut};
use elements::encode::deserialize;
use elements::hex::FromHex;

fn main() {

    let secp = secp256k1_zkp::Secp256k1::new();
    let tx: Transaction = deserialize(&Vec::<u8>::from_hex("0200000001015f2d99582ea16cc9451e8f9e19bdfedf43379ad0ad1fafb8ff66d22359017f020000000000fdffffff030a49492a5168e8cd96aa97f60f5b294cec1ca476ba2f8c1b4c87171f274422e99d08b3b32e88a1df9b226722afe3ad85304ac33f011199fd5cc84fa3c952fd4327a90325221684f36be833d1641c5ede4e72ee065ff8e5151604e123116de12e4e44d4160014d567c426a83a1e920a93d3b5cdc0362f80c5a96c0b4c3cf42cf177683e412511dbe585bff4021abab3e3d0a3b7de0e6c9109b0366609bec8fa07871b09ee66577ade9d47bf71189f94dcd31b313c014fb9baae678554035708296db5b49bc05e8e011b0be421f75438b839c6be86f3f9904c64d4515dbd160014a9cef783b24f4df35aaf6331427713acaa4a5d9d01230f4f5d4b7c6fa845806ee4f67713459e1b69e8e60fcee2e4940c7a0d5de1b20100000000000000f60000650000000000000043010001c57cafd385e2d848774a87f19409fd61500c10841b5bde34c89b928d81b9a76ffc5c999184465faf5c91377b1edb2d6aaa0168805bed16bff7a065bdbdd60935fd4e1060330000000000000001efe38f01b2cd1dba9db6a3478dae64e5a4210307895791c9bd31d4d4dddda68a10e9a660a93151c6a41bfc9711e07c7a338a135f0dbfe4b3254c43e6a4ae7c0caf0a61ed7263d68c6df68f62c980ba8bcdb7d20d3ebaa924423c6df9658ed73c11e672a9b20adb560fb8de9acdcee91324d0f86242163f17e11352cc336ece4854444fccdad0aab7f8bdc7d50bd248e606ffe612cd040c4837a5b659e34d221cdda13598f12903b3930f4acbba8a7596f4d91635e94aa8857f6c6ccb8b19595fe25a81c0ab9b324333f03309ef797418bc971bc40bf7a1eec55e08785b7dcfdb108a3210eb9002fc77c3a69e368a3ad4b490cdb77b1f82af9e93a665f52b8b20be24c59e7b49535d2708f5f17fb985370d6f636c9178ae382b82f8b77ba287a22dcd057bc465a7d49944e847a480c20cd4ce9c9149a568b221a32c9ec7068da8fa149562a347dcacca61f2dc75eb477c5bed669cdf28b907f827d7f6ba7f22100937a5595556fafd43664a54aecf3d502bbf57458bc74b77ddc7880fad23f4f384fdd965bbb2d3c339864a494b3e8a6cf00e5bb80134e9c815dfbd1bed4dd4c886f76ff50992bd493e58269536548d53876a952c002f58fff90582b8eb1629e885e39b529fdd73b188b2d492674aa56bc8bb4f2d983f86d7e484955e09c2cd42bf2f5c1091a2f675700fc689552e580cf074826e12970c8e1023b9e50c9c4236a904e77b7c9733cb47be4467bfbfec59c2b82be8d5d00383bb4e8ba11691f0fc4853c953979c2fd213abf3fdf523b5258002ea89a0de0db9799d78860580b2fd81e7ceecc0555476fd300f632d12c877f104cc8aac7a1f4c71cba1da9e4d22aafa5bbc4666d5942960399916037b106ff5d527c322eed885429a81fd2ee81edf8b7caf8c2ddc40536f6f800b2d6275dd5df471d7d3aab5285bff6250129661a43ac32ba10d442847365ba7e955d646bf0ae2df07d264a49731493bf14a90199a7f72d37b241b78db1bf8af6d1bbb04064b6bf86ce7c7b543fddaf18d890c57b3fef59d2cc098b174d229d1361b7dfe5b669166b4b3f91e170983fb60c05094bbcc4ea682da6979ab72de6a281f82ab3bdd1f0e9431b70267c101979c7972c329be5694e030f784211a236c49bb38ca77f4389b591a1e2a5e49aaa556ea68b17f45591e4bd38d467356db029ba496b334f22945926ea959f3a07f657f8bcf086c36caaac97cc651a41dc2b53ce9b82a6cf0c126c1fba7f2ad35cdae353994de1456e9539ecf10b5c4eec9ad1ed5e05152dbcfa448c016d3af78794b68322a2b1ff2d1d81b4d0208eb7065c7791cd0f4046505db5a99c0448a3b8fb27bf9ba085d5a4b1b5206e876f6242bd1da01803321a4bcf71676a949996d2b46ccce6f1f1308fbc775051c5e841f243da64b4a91a27a7192d7579a21dabded2798c3b1e99bbd2aca6b7d1f4a3028e0c018aca143bdcd842bbaa8e4ef9d70f12bd7af3f15a6fc0ce958c134fa749b2dcb25a518535239f9dbebc1034f1fd7e74c3808cf53094da2370e518660b73503a787cd0fc87f02ede9a90d0364e07fccaa9238be9ef472b7abf2378676d75cee2d85e640885e28eb5ff0c730944835122065898df69fc37444da4825a4f2524053ba834c277dce029b9a145c7382d94d64df190a4fb1cecc057413643a56f6958f87511a5bb0dccb66cf7a52b6815a6251d9b2def2a532289ab9d5dc50be8d9d399ab858e0495c2c6d487551805ea8a70b70044816ea7f8e51390ab7a23c4a58ceab9a508fff0a66b4d1ada652689a77670572e5c0e0af3b3d9ebd14f507dc1e2e61c22e1ddbcf16ecb760e41343e851240df95e99eb33c8fe84d7509f6abd8a3a42e98dbbc59299d618706ae42c8edba2e1dd08714c446fe12720dc394e80b868c5f1bd8caecea8547410977247ac554d3b6054f6c866f08f692f873a0cc41c801486cc81ef3eaed06cceeaf9706d6b8da55beaeb7ae69151de494864e30e74cc92c8aabdf80e7c589a1e0e1862333810bb3195817ebe8006b7ccc60670ba8ff76f6b2ae767ff0bee689e5c417d1f8cd09a690e8e841a3cfb1f071f6282e3adce2770d1bca0fc02d2cf7c0cabe11b154c1186985a1bbc7a96c6ed5149a0ea6986498fd588bbbffccc453dca3d391d29946cab5832cfd70304a774e509099ba244dc7c044acfad786dbff6f5060ec87b8e5d722d1773a37d6cd363226084848756bf36c81edb4ac676e2abe9f013d12d697cdababfbd1cbbcc3ca7a5efd49d571579c69c0f16498e129acc6aae9c7b95b583683ca10e3490e031a34b0f01db351a151a8d57b78617c6d2501a1e36d6c35cf9f361c5d0becaa991855b4bcb58d2bdd3270042a686072ce551f3cc2dd62fc799b4f71146cc6ccad33cdf4e6ca11992a5e2f824581c7af09da4d4a83b486173025c0e43b4baa38391f76736c99aca3b370d5a80689f1a21790b012096b97a81df2ae2dd2f944630a6654051a4d02927ef414400e8f0de0088387fa2791046a2bf4447256eff6b5da0ceaba87c20f498dfdfb773aa22901bf2d7a5b622ebe2f7a7a8a6ec5a0eb80361c3cfe91e4939d57d32b0c2f678cfd828c0507e75ed4e1afb09a61be08366832d7c2bdd4b1f882e72bfc62fb33a5daec270fc6e30ae8922f81a3afbc5c5e1899ea1485b2f7379452b803cd47c2eb355261e323f07aa5917df219100ebd0e83b4e48530f459961f5b2dfb0f4bcd6fdbfdec88cb7327ffcf4ba88646c332ddb553e55b61ae2ab9ea7dde27a8ac1ce4bb179933c5b1abd1859a9b687d8fbdf34851abf04ea102952b9024b70b1d9ffd83801dc3b733a3f8b33bd7aa3fdb1a8ddc743f3022086ceae8f0f43398f8bf069ff46b24d32deaaaef3d4cb93f150053da8b4aa50d4c4b9415004eb34b98459e9664e8370dfd9c6aa447284a3d350b09f610fa4ac4662f792cbb5478f11637da553a3a52638d7f362281acf3a8901f7ce2dd6cbce21a40a94577608d1cb068ec62717f6927a79a8c768043f16af90e97ad3a466b6115a76137456736da941d51e13e53be95522e01cfb06ee8a102518ff1bf4af8856322463fd5e920c2a1ea2a7fd403a792c0c94121b0c2a1535440ddfc96a5b2ca44ea073a192e9a427ceb4665faf26b1764c6321d592b3157d8153a2cedb5175eaea0cfead8b137f9cda7335c4d642e71bfe5cae223684b1cc83dfcc2ece5505f701be1a3cceda62e78c853b01aa6232c9bc4f7926bbc75b30fa921a3c663bfcf7236b007bcd1edb7d6f8e804e6fbab804fbcd864daaff334698af6a8cc806fc83534e0e95bd5e30e56cee45cde30e0cef551625ba7ce2f4c679d939969e43856d092cf4bad04ec84569d3a46238f8238e88ec3eec0bace677e85d0f0c3ead4bbf8f52ff217362f18fa5e6898d5f3b62e833fb45b9f8b079dd4ac5f17d6f8856ee4787c6639643c4e5772c04bcf66fe3dbbd286afd51d6cdbcac9d786b3806ec618ca1ad4763e2e18955e43c778d22b7fb9acd76ac39864ac2c2728057e46a30485371f5be8e67b400432e71d833ff8304d77de0869d80fbe4d40abf02f3dc3ca4d2135c8b0fecae2da40a8c2149611231c059859b3e5304338a70f5e3303723b5b76c0386f5fe566cf95c78cb2b14d4ba675f3ab7d2db9431c753808b3b123509245d585fab9e0e7f9f1a302739745698953be4868107b8432c96cc082a2e93044a4ed65b4775b06f90208f8589506a702615878a6188b890adb69da7b2495cb55f65b256c8b2babf85f16d94e824cd3760eac63688e341e9a3f5f7ebfbe56d9dce37b90e2fea0bec87e5dde7b8d3a048368eab5ae2cdd7df69341f5213c780ea79598e96f407bd35ad8ae653bca954cc565dc5f9a5e28a821b5cc3ce981457b11eecf8c6ca7e65515a1c8bce3ac2e7303af1ad0e9b1a00983a03fbb0cfe52d85f5beb8bebd969f7ce2c5e57160873beba3a06cda0d99a1b3a887b443d82a6a14a33a69ee64059c65e41b01f6825cdbf73089f3977f02b6008879265720693afd1aeecfb6c050f9f38b75681982b0ae91164ff83cde04410274b2450cb6fcceef45365b2473aeabc075cf4108d94a584aed4fb48afe9c90add923883e27c6c202ca4bf323e8756127d628e09d13df0449006e7d14a73125673ed8a8661a19749e5a8a01d768f500f6fbf1af08a73b7ca99117aca1d521d46e16ba12d953cfc839b17702c6f4a333e6dba3cfe450ed330e48974e751d88fa76d73036b8aeb434375c2f1b65f1a1505ba194eb4f2238d2f4ffa39e9035143de33bbc710f473d6a37e92c7d341e010f2bb545d31b50debbffda7c327be59b9469e240e2b37ca93db3545644d619c2f3173f8f5d646b545c0bf86008933308e8bfc4e1c410d901f2b1e79c74594258113212a737b286a9ac3108ed059661680f11a9d0c8994fa1188ae27339d14595733009eff14607bbb77f407f7b08bcc9973104807a34f52c4ce675c8894b3e43be483713da3440393f6079670dccc7bf4f54d81638bc411c49ec794611bff52742aafc059ee5702e91cc748da13969a1476c904d3fc261ccb022a7409f8f1ae8fbd7c2a8506f3bb2ad84ff55c93998ba367531f1c3b411ac58549a31bf205c102c67351956eab23285cba63601fbfae5498bcdc10cc173937b395e437fcb79b16482b13d824ef597144a73b8f64cf853d9b49cc93c93ff45bf63cca325ad928ceaffd59e5e8762202cdd9f95def9e4ba2d92dbc4d13d25cbc1c7aa482b4ff08e2c92e66acc9ba1475f8ec56c1dfb80af495eedcc03ea0336f18cc76ea057f815e5d2a3ca88fbc293d014862c4b505fc5e7d72876d434029b9c003d97401f604eda90dbcd76350d80529496f55ad8003fbb85f7752df6ceaf9d9ffc3f60fc0bde381fca70306b17837a81e2429c85027a1b377682c0b554a8a78252636384f5559a77573b0070c3baa520d716263b385a0defd90c2384b7ebc564a030c33cd6997b472b43942e063e44817f28f5ca7d5a1ff48d80ab6703600a7dd11395a3f64d8683d47b7c9bb83723134f850b45e3038f9395faab53d6d6302cf9c1a693c86d978addd808807ae9f0dd87ea4e98129a0bd0f0813509764c213eac369692900b35063f0dcbf7bf5961d017bdde3a1ada42c24b71a8ec1f4e49257a35d575327a24ff6049444d4e125257aa7f4c20588d978eadafec231b242b158daa825e6aa5d67066c1f16c004c9e315162aa8c9bbcb254ca8e180c8a177d505b9962b2991f85cf235680df8ea3d531fa393497041736a032c68eee70bf92145b429a0ba825aab62a7e9f023b7c34c7aa4da2251415cafa0badbb046f661ad11718013378697e12eed08f02c21de96c5d0a2fee464b69e02c382f5967a3ce115f18f09e99c9216da22518f510555f44852c4d43cfd4bd4746bb955dec5629cf89c08816ea2d43585b95e5f7beba50200f7cff83d9415476daa7e99853b5fe9440db3aa610ec1f8420a212c5bd55a1f2a5cc674ea0dde9240a9010821b4e6edb312b1f480743bcccffbb8d4716317ac74a2217d6ea49de513838bbddd890a35aea5c5b47ec286389055202da360387bfb2327eb38a2ec5521680fbf7c54b20b0c4d21e2f23046939fe683d63bc60c9451d5985cb3d74ca56ec7d848037298f5acf7f3059ea5f73c5213408b458184d334bca66ddc792c674c626e6e3e2e366b2284b21cd8c134e0783e7494b5278dcc29af3b17d5609c04e1d6bf3f21670befce3fab0f7b2d9b7e6c46656bf423b500df83f2bfd0cc7502db1a3d8320235064708115f45b9ae7810ecaf73304e153662620a247d3d6d2d4251441c4dcdd14fb9b33f2f52f1b4a05fa1cd8e77c3c79178ae0ad961a9737c01a0ec073e4743010001662279b3e4efb9e9b04d9859d348dbdecaf42dcbe0664d7d035851e52630e9f537632b1b090f530aa6a9d3e001feb931eeea6006d339890fbbf481f842a96d05fd4e106033000000000000000198b771005818add879ce29234db70c78e5a9d046b424d5e2cab6e9499bd13df7432a9ee058a2454d33bc1ddef43b73de3a83eff603d0ba0686b127c6eb7f2c02c5d46b6d58a4fe22cbd3fa865bf378465d0133b2fafc47c9aaf56cde01dd065b2457911e889d4bf1f63e1575799ebeed4efa00edb19898b1b3d7e1eac163cfa25550e5ea917f0912d54aa9dd9980aa8757219e94e93325d126fd6f31fc3bb8784ff910cbaf274e46e62d42f1e25b8d78bea8498ebfd3cd52bbc41f971a50bb235d83d014e420a92969a24292aa2692021ebb7717691b1bd0bc81eb80c9ede4174e64352238ccce5efe4ff231db947cdd03b03786042cc7ca7cab2253850abf322cb98f039890a5ab690c0803e0692d2482e03a2cc56ef08f4e1cadf082764775ad8aa85bd380ee8ebedd4115b3a4513f3baa630ce21476ab129668994c7ae936f0b439be17e5a13326081d19ae36211992752e08549632cfeedd4b7effd2e07405baa70ac0cdd6accbc1c8f4b9f5b3d27e2ee3cced0254927d649a61ebec2d67ce51746911512e9c9547365ffe406d3604d67f6d8cad7dea01eebfbb32a986ea7b8b4d78d8df0a368c5da4039f53787369b1be5ed937e8608cd115a7d54c6e3819c34014e0a3b69e192e7f195cb570add2b1c583017834d20f5f2b83038859a2888344eb6ac38a2f9f1e7aa7468454351c9df871c802060525844e72cadcce258469dff954aca0403efa569212e348440b35a94f75fbf237d9508f406f6b88dcb76c46108371c969518f46c32590ce4a527500564e2504401a8b74c5edda5c2037b5e43ba1682916c5e4bedfd559b475e6b9afacd2bf4bc2861497c3154b684dd5669b80c1548114feff221b4ba6fa9630f15a348942a5ff0fc1b6b7bbaf10de7e9268b3f3059895bad350eab5e6678eb379c78b7cca0a19206fe2fb51b9543eafe174af2007badb1a86ac43ee32386150ebc1ab27123dedafa2a3b7e6e6ce4b6c4686529a157fed25c4ae5f7e44e0f63bad17c8d7c88b507280c03e952b7af36962381699150a15dae23aec75d78708bf9aab8f9197d6d3ad97d179d0d7890e16fae101fd57d49d17e090c01ac5b72d53111df514917c735926fe66665b7ac08ad1249f99e1d0fd1bd2c11f32a5edf273dac9a9d904abab6551745f71cdd2bcf5b114c31bec42b4f7078e5da4076e140f6370133b593366786bf10be1dbe9dc411bca1b7ec80587bf7735ecd3e3dbaca3855a9df3b01782ef6c009a916b597ac1b968040c0fb2054c7685ff34851a2f0153ea1f9f0aea6781234b7848148e03fb0f067d77e272ac86a59cbd54a501fa763be2c3bc8f083ad2accbb3d8b2cfb933823510b5b86b7ba22724511eebd7c497dd6b79243c5985054fd01b1328f7dcee2d6e7090bda4763e7d0aca6fa7a283a84be84ab15aa95beb1b4b5d6760f39e42ed72d69f7ab0c6740e92a14ef3490cc5d610bcdf0300a8192a2bba672c3971782ff9847e7a6d479896d87e9d5e95fc1158cf6e30d44697be519ea5d312855e3ee7205cbfc1e496cd8909a46e8e167b3d8bbb61a824340ed8cb434ebca7789ed0430d85fdb7548840e42a63e5d201bb77bb66f9c515fc7ed6d812916962adfd42f621ee50cdfed8b9593d747e094e618c94f8198707c598f1c1ad019aeb7fee68c72e66656f43b56f443014986c0d3c7446500999316ae004082843accea6fb833c893046094dd2b995d84e4cd74789956a5289d6ad4ff2a766d0e7a64d91cc7f7dc6f225fbc27ca0f2d30e6d2ba6105e5e04aa6c261ccc0a0dad14311863c6bb863858707779f6a708c5feb9c55b612452ba50c10472b18b5e0abf69f019f4b90246b2d36795b598a809a4599412623d810c94baca6c369a82dffae8c711fc2d105e92a8d7b4c244f3eecdeca19698e79f83462d6870d0b91fa926331dea367a418be40b1d7d530b1f239b77b893bc5a4525013252cf199a3c66166eeab9e7876cb47a0d6f4cc9df5c83bdc5230a345b44a4c7362e02a091fda5d7ff731d73bd7bbcf3092494eb7e873f976a2015f3631dd0faf76c0f54159dfcb6796fd5144c3e31c998ad5b6028b9182445e9e25faa8a8b5c332a19947d5cb14c2a1dfb76313d711a2eea0ca56c4b4c9edb8cfd089f03672a499e6419fca6914052f7d0176bb2e607c2fc6a3de7e58c3f63ac54f2122af3a9135d5e58b28918b9c481f9f8b94a76c5b2cb790df5e726a6b4c24dd56809d3ba683941fbe3f4334e62d37990aa4b64efa848404cb090cf98d254f0cc8ef48090a46997c64fe4b129ac84663bfc2b0a25a02bc5898a199643af41cbad18631c5db596048cb1bb89f0ea9243ec8f72ee67a6a95eb751ba7f59ded999f2875ca40082a72b62f9bdec52142562d0a48c50730a940db9613596ebbd359237fea5ad701fb2b68bd64f0fbdc2485a8102444fd321777a6e22f093e15dc23eb158035054681053c2035326135e407681a633370138fc265a7cda97825423b08f7955cff40bf4590b38e482bd388741c17a3d79a40662fb41fd760e15f4b58d540fb66b3ad698365c339d8fc6babfb7d3b039af01a840859f9ac7ede34cca3d9fa2efe8eeb7c804b02225d039aa1db5a18dfbb96445742e08e643d897e3db984bce40167997547657432c88c81005d1eba0a5c2b27bbf2fe3f92dfdd0d59a49a1432d2352e912480b8a76b7618576b6db40a57af5d5bf3f2df7bf01e30f8b823a4b714c39082afc4f2f1ec198586ae3db5d5557fb054ab069680736da45187f264ef8d9a45d4b8d0eb9a54f5f074a9c5b6fd8d82550f290e599e3a73c0eac52dd94dc918ae4b985f6863a5783d764662ce64a0d3eafe619e1ffdaa3faf4a6985dfe6a58f7a132ddf896d9a757349b0a51d10432730111c391d926b96fa605714c7dbb1719d868fb7ef6995d63d4d5619653e865dcff21bc8713e6093daab77f2acf5491a66244d9d46335da8a16f63e2d296e83f540f5e4f574f1a5a1500a20a8190b0bbf60f0b401a4a5c333edd38728866bf26ac44ebaf7f41356c8fd60f1bd60112a0d9715c11c9b271d2b7a3cede68f4a88790c47ae1b027700cc9edc1a9a63f8f63b796e9ff2773d5138bc8d1fc98a84a7624b3cd5d04d6a11d4e55e08ad2f69f1f90cbca0a1db3f53008bb85f155385de4bd312af8920b1c63489525b9e0a97bc5eb54b1ef6173af2950559e9f56b24374a61c96458184ceabc34ac24021c26ba82c3e3687a3d8a8e28ec3aa563c94ac9125ae8b0b9881e071b8c862827970028e55cf0c7821411c824a5c4da44fbdee147c86f943b718387c8232291fc844a89aab8466736296b930fe971d0346527dcc71090ae165c7a8e8edc2c72a9af00d5c3fcd52ba9253ac15132045fd311203339e782c6e54ff26e3a950f2430f74961f554593c7c435ee5a79a0a0bef3bdc347ea7faa6b3e4579adafeff126d6ded61b116d20c2fa9e5b020f71395c4d1efc8f18adae9f5326d52962b0862793fc889d984069631cae5d4295aceea308881579e1c313cd09833d67097c247def9cca36eb72a0e069dfb30ec4e14148196b08fd416de071cbac34895c13b754f354af6aeffcb6d64531e48becab49697177ebe29d550c17f22626d9d073d0c40c5151d99ecf52a51cc47d6c2c2bc2087fd94b8ab3452eb3acdd0a2e091fa3fe7baccb70e69348854504562752cb5489fd65765719108e5f6387f231e72a61f33ad1f50580519f9e6693695c767cc7f0a60a9b6927630e874f7ad8a072ad1ea326c8b65d51b0cf247d83acbc6efde4628581bda8449c5278ba69115c6d0e40531cd19783c34ff963204830cadd81e263062521ce8cd1559bcff6fcd4b331238e8620348b494fd9248ef2f364fa9f836abec7b499685ec7d03287042a8ef3d68292d27fafad43c78d3564ec4415e1549471bb4c93957be34315b81ad020ee257fe8e6f8c6b8f429257bbc91015ff376ef698d4236c7d925aabd87fb8c50c737cfd4dd37791b1c43d7d925dda0af5cef8e5828993dff41d4019e61152b20017f18723d6f8589a72ee6aa61df954b11a094dfee7f8de82b4c6e0b8378cdc20965297622ee78b665df2fc9e0a59e2416d8101aab95a72d5ec8c6bacc46b8b052454bed1b1155e323c778b76cc12c4337a0456479e99cd954ee9b7a610ef2fdd85115c8761e48a15ee0dc75a1c44a0595783ee7a3b3f0be27bd901f1118ffd7967aa98fef47180b3c7314a238223aa468b3acd5c568ea74934fdc10b06f42b18b23d7a425cb4180a5f5e76de3f54064df11ff9cee21403be789639ea2946e88ecc81bcb492adf6d7652a600ca07d614a0916c89eee9348ab1e356e153bc37434e37bc7e581bb797251198ffc955e555ee20ce6118f175166ab25e25cbc847b825f969a66085f745eb64ad7ab1e193a7dadf466c29eff0ded9a9a82323f1d0ab88134ada28d3691e98e90ceeaec3369d466e24526e4eee3255e0266318c2851476b98af959eeacafd88994d597c52afd23d22578876cadbe26026e10753aef97ed889dd436d0e9369bafc8cd50dd0b6090cc3944b81063bbada167cebcab6866fb3b87791b6dff1de3b2aab67397df70c60ebefa9389e741fd5337113a1da0d7ccf733348eb91cf40d0c323baa3f692f9b45881efb66edaf4ea7ac6d88b6a93fb7237f6a9bb43902ccdb2ea3e6a342abfaa5e36d9b685bb1a97f0a2b94a69c674935fad7b38d7702bdff95d3aca3895f3b05c70cf1754cacc5980610f09b6e2cb3e89311638fd4b271805efc7486d2c4f0cf0910598427c66627c74ae12f29b63a7ad778e0aaa38f7b1cc23189603d01891ba26c14e4e96c0f7c86e63c73d8d1098661540b4daf2424c89be4991f07ed284a8765d92c6b52c080bdf62bca1428a018352baa32571d8d36da5653a91415dfe8d0c4aa46ed6766f4c162a05ddbccdf14ca9f84e2d487199467f338e81f3f5058bcd22e96a04119d036de1ceb872ca05954e53c86b85626a44856e5685fe35d8217af87349d24650059e00ec7b80d7c0496950a1c827f443db6933a756723cd529c25d7a93e3be32666bf2d20f533d6b3e1378296cafa3b468a3926e7754781923b12d9701bad179da601a623e25f76731a98e93712415496ea84f88dffd243f7ec9a480cc22358bb6705d46be5807e3aa49019e16f86d0c35af6c6e6f7019cc38f5dee215298ae1319bcf248aed114fdb0a5b342b8d250fdbc3bd5ba21e3232a7655bf424eaed8c810b6b1912fa31e37399a56aa33a9a6a76974175ac96b67c43c1ecfeb5de2b0c7b8d4f29993def838ace907acee23a1a7fb52d53078a4d544c858ba18412f152f9596b31a15107e428c96175b5fd49dafda407a0c86bfc9ff6a44217a635bbe6c672c85156f21ec664465ca9e6b9e2e4636aaf6c7d02a9cc28907f5b84d08b1a0a216c8a8494f165a637f5d4edcd0e272390d8704840cd62846db58c4f4c118f19ae3087d9198a05f8b2b256cbe176bc9604df06bda9b7e49d7ded17a059f32b28e1ed47421f32d425004261ce060a2e6af1b7f583f9d7b6e887103df1c359f9d043fdcf91220c57c438fe37b6e606492850ae87dd6f0fc4339777202464dd8ba10e119775ec36daffcca9aebe2208249ffb7068009612a15fd4ee01fb564b567c7c31a5179f88bd1e71d4bd0771b51651ed98ea4bead8deb759c86b2d705d16f4335457b0c9d3b0e9cff91f8df33f265600bd0cc8e8128918456726dfd3cbd8b8c45b21506dab2c49975e10440a0673bee73c2d757f48ea9e4c8de2c4a6eeb55d3127c78b95c2f13b4b0f66c9ddfaf6d28ca7070a6b713d28e8ecb64d175b2a4e8151323f0f9e1769b4ca1228f7d555ed851aacfd6bb782629e6f38aae70000").unwrap()).unwrap();
    let asset_id = Vec::<u8>::from_hex("b2e15d0d7a0c94e4e2ce0fe6e8691b9e451377f6e46e8045a86f7c4b5d4f0f23").expect("Valid asset id");
    let btc_asset = confidential::Asset::Explicit(AssetId::from_slice(&asset_id).unwrap());
    let btc_value = confidential::Value::Explicit(21_000_000 * 100_000_000);
    let spk = elements::script::Builder::new().push_int(1).into_script();

    let txout = TxOut {
        asset: btc_asset,
        value: btc_value,
        nonce: confidential::Nonce::Null,
        script_pubkey: spk,
        witness: TxOutWitness:: default(),
        // We don't care about witness here since all the blinding
        // factors/explicit values are already known.
    };

    let res = tx.verify_tx_amt_proofs(&secp, &[txout]);
    match res {
        Ok(_) => {}
        Err(e) => {
            panic!("{}", e);
        }
    }
}
