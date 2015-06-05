<a name="v0.1.0-alpha3"></a>
## v0.1.0-alpha3 (2015-06-05)


#### Improvements

*   fixes ssh_keys api ([06c7b765](https://github.com/kbknapp/doapi-rs/commit/06c7b7656cc51ebf6944af7906846f96c707aaf6))
*   implements droplets neighbors command ([6f1d64c1](https://github.com/kbknapp/doapi-rs/commit/6f1d64c181bdb4252e579b141315133448b7568e))
*   fixes indentation of action responses ([a5f26cec](https://github.com/kbknapp/doapi-rs/commit/a5f26cec4bc0b97271a66c45abcec11c3b108a92))

#### Documentation

*   wip, improving docs ([c569bb3d](https://github.com/kbknapp/doapi-rs/commit/c569bb3d604c2cfee492b7a757ffb7387a4c2a03))

#### Bug Fixes

*   fixes failing doc tests ([75dac446](https://github.com/kbknapp/doapi-rs/commit/75dac446cec7632fa00a67a5e17eea96a38dcc49))
*   various fixes for droplet neighbor API ([8c9ad227](https://github.com/kbknapp/doapi-rs/commit/8c9ad2277cb700ad9578dd45a31aa44e8c9ec7d5))
*   fixes a droplet action API ([b5ac3bae](https://github.com/kbknapp/doapi-rs/commit/b5ac3baeda515177db4c3791639bd2bb5f0b07b1))
*   fixes image API ([4e032f2a](https://github.com/kbknapp/doapi-rs/commit/4e032f2adf1ccb2798421cbb7c7b52a3a3af2b5e))
*   corrects dns record update command ([db3e723c](https://github.com/kbknapp/doapi-rs/commit/db3e723c44fb5d4e5a9cf333e03f8ed296fd4c81))
*   fixes create domain API call ([8ab57fca](https://github.com/kbknapp/doapi-rs/commit/8ab57fca958e699869d9d82bba923301e9826f6e))
*   fixes ssh_key displaying ([f09869e8](https://github.com/kbknapp/doapi-rs/commit/f09869e87296e8d0639013a84f490472aa56eae7))

#### Features

*   allows retrieving anonymous arrays from digitalocean ([5aeee794](https://github.com/kbknapp/doapi-rs/commit/5aeee794749492c686ffac5e268c0073a204341b))



<a name="v0.1.0-alpha2"></a>
## v0.1.0-alpha2 (2015-06-04)


#### Documentation

*   adds more docs ([63a787f0](https://github.com/kbknapp/doapi-rs/commit/63a787f04f405cccce2530eca98713a784462317))

#### Improvements

*   adds body to request display() ([f972678c](https://github.com/kbknapp/doapi-rs/commit/f972678c9e9acce011b268e2a5347973f412f91c))

#### Features

*   adds network response objects ([49b225c0](https://github.com/kbknapp/doapi-rs/commit/49b225c0bfdb2f2f4b95a5ea8d7919b4e969c5a7))



<a name="v0.1.0-alpha"></a>
## v0.1.0-alpha (2015-06-03)


#### Features

*   implements listing all sizes and regions ([03b83d54](https://github.com/kbknapp/doapi-rs/commit/03b83d54576f12a33485b761b88c65afc78ed34a), closes [#11](https://github.com/kbknapp/doapi-rs/issues/11), [#10](https://github.com/kbknapp/doapi-rs/issues/10))
* **Account Actions**  implements account actions ([044ba2c2](https://github.com/kbknapp/doapi-rs/commit/044ba2c2a2c1fa6e9fb4c1f6f3c22646cb92acaa), closes [#2](https://github.com/kbknapp/doapi-rs/issues/2))
* **Domain Records**  implements domain records API ([5fcb828e](https://github.com/kbknapp/doapi-rs/commit/5fcb828e5d386f676c310128dfa62fdd71aebd1f), closes [#5](https://github.com/kbknapp/doapi-rs/issues/5))
* **Domains**  implements domains API ([2b59fe78](https://github.com/kbknapp/doapi-rs/commit/2b59fe788a05574d0c1bba9e9172fc3a9ce3ca1a), closes [#4](https://github.com/kbknapp/doapi-rs/issues/4))
* **Droplet Actions**  implements droplet actions API ([1699de98](https://github.com/kbknapp/doapi-rs/commit/1699de98082c089f0624d5b4a776721741424510), closes [#6](https://github.com/kbknapp/doapi-rs/issues/6))
* **Droplets**  implements droplets API ([e9155918](https://github.com/kbknapp/doapi-rs/commit/e9155918e6e3a567dfc3b734fe1015d74a243361), closes [#3](https://github.com/kbknapp/doapi-rs/issues/3))
* **Headers**  implements response headers ([556db476](https://github.com/kbknapp/doapi-rs/commit/556db476bb6e7996b89929cd36a9162924cbd519), closes [#12](https://github.com/kbknapp/doapi-rs/issues/12))
* **Image Actions**  implements image actions API ([583f9f71](https://github.com/kbknapp/doapi-rs/commit/583f9f71df8cc9054916f1d45bd7506207f58d20), closes [#8](https://github.com/kbknapp/doapi-rs/issues/8))
* **Images**  implements images API ([d2fc21e4](https://github.com/kbknapp/doapi-rs/commit/d2fc21e4a9298237741471bf3a07768485d0631b), closes [#7](https://github.com/kbknapp/doapi-rs/issues/7))
* **SSH Keys**  implements ssh keys API ([ee35655e](https://github.com/kbknapp/doapi-rs/commit/ee35655e6673cfaaf24be17360031b699570b7d5), closes [#9](https://github.com/kbknapp/doapi-rs/issues/9))
* **account**  implements account info ([079da60d](https://github.com/kbknapp/doapi-rs/commit/079da60dade586419820180c021ccfa3ee1ab4ad), closes [#1](https://github.com/kbknapp/doapi-rs/issues/1))

#### Improvements

* **Debugging**  adds debug! macro ([5291765e](https://github.com/kbknapp/doapi-rs/commit/5291765e2f7013426ec7bba08c42eb74fd09dd4f))

#### Documentation

*   adds more docs ([63a787f0](https://github.com/kbknapp/doapi-rs/commit/63a787f04f405cccce2530eca98713a784462317))
*   fixes failing doc tests ([eab280d2](https://github.com/kbknapp/doapi-rs/commit/eab280d2e9762e75c0cc99dd9d049277a1c648a3))
*   fixes typo in README.md ([ae9d4484](https://github.com/kbknapp/doapi-rs/commit/ae9d448425579921b73f5921799489543d4d0fee))
*   Adds more docs ([417d4463](https://github.com/kbknapp/doapi-rs/commit/417d4463f94656bde756b19227403a5273827d4e))
*   adds documentation ([ec9f379b](https://github.com/kbknapp/doapi-rs/commit/ec9f379be488a3ce3c145e42b6cb387c9a3adb79))
*   adds changelog ([dc8758ce](https://github.com/kbknapp/doapi-rs/commit/dc8758ce6d381a0a8d66aa2bcc643e3888879e62))
*   adding docs ([c1640458](https://github.com/kbknapp/doapi-rs/commit/c16404582d3cd1b50579d5d79cd8b396dcb924b8))
* **README.md**  changes the name ([12df922e](https://github.com/kbknapp/doapi-rs/commit/12df922ee46b5c7a5396dd895ac6619b260b3a22))

#### Bug Fixes

*   fixes issue with creating dns records ([2b3c2ebc](https://github.com/kbknapp/doapi-rs/commit/2b3c2ebc19212ed36d3bdd19e4873faf2edd0431))
* **Account Actions**  fixes a bug where not all actions are displayed ([babaa84a](https://github.com/kbknapp/doapi-rs/commit/babaa84a0c9b381d07077bac9e0964350639f4c1))
* **DNS Records**  fixes bug with duplicate record IDs ([09b47433](https://github.com/kbknapp/doapi-rs/commit/09b47433505ce1a58f742da3389a62ef40c53da9))



