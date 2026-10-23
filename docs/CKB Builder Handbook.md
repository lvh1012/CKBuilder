# **CKBuilder Handbook**

Welcome to the CKBuilder Handbook! This is a resource for all cohort members participating on the [***Community Keeps Building***](https://nervoscatalyst.org/community-keeps-building) Builders’ track.

Please note: some documents may refer to Lumos or Capsule for performing tasks. These tools are now deprecated and the best tools are mentioned below.

**How to use this Handbook**

This Handbook will guide you through learning and building on CKB, providing you with important resources that will help structure your study: starting from the basics, towards more advanced concepts.

The CKBuilder programme is designed to be something you can do on evenings or weekends, with a minimum expected commitment of 4-5 hours per week. Many people will put in more time than this, but what’s important is that you work at a steady pace that allows you time to absorb what you study. **If you have any questions**: please contact the programme director (Neon).

### Reporting standards

As you work through the documentation and tooling, **it is essential to start your own dev log on GitHub to record your learning and progress**. This is required irrespective of whether you are studying docs or building an application. Below are some reporting requirements that help to ensure everyone follows the same procedure. Here is an [example report](https://github.com/JouahriAli/ckb-builder-track/blob/main/ckb_weekly_report_w1.md).

Reimbursement is pro-rata and tied directly to your reporting log: if you miss out on reporting it will result in reduced reimbursement proportional to the number of weeks missed. Reports must be:

**Once per week.** Pick a day and submit your report every week on that day.  
**Contemporaneous**. Each published report must reflect the week just completed. You should not compile reports for all weeks at the end of the month. Multiple reports published at once to cover multiple weeks will count as just one report and reimbursed accordingly.  
**Published on GitHub** in your personal CKBuilder repository.  
**An accurate reflection** of your progress. Use your log as an opportunity to share what you learned, what you found interesting or challenging, and what you plan to work on moving forward. Include screenshots and other evidence of progress. You can use AI to tidy up your writing, but avoid using AI to generate your reports.

## Introduction

[**Introduction to Nervos CKB**](https://docs.nervos.org/docs/ckb-fundamentals/nervos-blockchain) - basic technical concepts and terminology explained

[**Getting started on CKB**](https://docs.nervos.org/docs/getting-started/how-ckb-works) -   

[**Quick start**](https://docs.nervos.org/docs/getting-started/quick-start) - set up your dev environment and project quickly using [OffCKB](https://docs.nervos.org/docs/sdk-and-devtool/offckb). Further information on networks and RPCs are available from this page.

[**CKB Academy lessons 1 and 2**](https://academy.ckb.dev/courses) - further learning to build your theoretical knowledge

[**Introduction to Script (smart contracts)**](https://docs.nervos.org/docs/script/intro-to-script) - a introduction to smart contracts on CKB 

You can also view some community-developed manuals to support your learning here:  
[**Learning CKB - 24 lessons across 5 phases**](https://website-sooty-chi-72.vercel.app/lessons) by Jnr.bit  
[**Learn CKB in 45 mins**](https://github.com/truthixify/learn-ckb-in-45-minutes) by truthixify

For a more detailed dive into the concepts and theory of CKB, a reading list is provided at the end of this document. This should be studied in your own time and is not a requirement prior to starting the beginner exercises.

## Beginner

**Basic exercises** - once you have set up your environment and understood the technical concepts, get started on these tutorials:

**Please note:** for all tutorials and practical exercises, please remember to retain screenshots as proof of participation and completion and record on your dev log

[**Transfer CKB**](https://docs.nervos.org/docs/dapp/transfer-ckb)  
**[Store Data on Cell](https://docs.nervos.org/docs/dapp/store-data-on-cell)**  
[**Create Fungible Token**](https://docs.nervos.org/docs/dapp/create-token)  
[**Create DOB (digital object)**](https://docs.nervos.org/docs/dapp/create-dob)  
[**Build a Simple Lock**](https://docs.nervos.org/docs/dapp/simple-lock)

The [**L1 developer course**](https://nervos.gitbook.io/developer-training-course/lab-exercise-setup) is another resource to understand the theoretical aspects of CKB. Many Lab exercises here use Lumos or Capsule which you can ignore.

**Learn building on CKB (Javascript, Typescript), using [CCC](https://docs.ckbccc.com/docs/CCC)** (Common Chain Connector), a beginner-friendly all-in-one tool to build applications with integrated support for major wallets:  
Explore [**CCC App**](https://docs.ckbccc.com/docs/ccc-app)  
Test your code in the [**CCC Playground**](https://docs.ckbccc.com/docs/playground)  
[**Code examples**](https://docs.ckbccc.com/docs/code-examples)  
[**API**](https://api.ckbccc.com/)

**Note:** For intensive smart contract development on CKB, it is **recommended to learn Rust**. Rust is well known for its efficiency and security and has strong tooling support on CKB.

[**Rust SDK**](https://docs.nervos.org/docs/sdk-and-devtool/rust)  
**[CKB-CLI](https://docs.nervos.org/docs/sdk-and-devtool/ckb-cli)** - a CKB command-line tool developed in Rust

For other programming languages:  
[**Go**](https://docs.nervos.org/docs/sdk-and-devtool/go)  
**[Java](https://docs.nervos.org/docs/sdk-and-devtool/java)**

Other devtools, including:  
[**Testnet faucet**](https://faucet.nervos.org/)  
**[CKB Debugger](https://github.com/nervosnetwork/ckb-standalone-debugger)**  
**[CKB Tools](https://ckb.tools/)** website

**Payment channels on CKB**  
CKB is designed to be highly secure and decentralised at its base layer. It is optimised for verification. At the same time, it supports scaling and high-throughput use cases on layers built on top. The most promising solutions are around payment channel technologies. On CKB, the main payment channels are **Fiber Network** and **Perun**.

[**Fiber Network**](https://www.fiber.world/) -  a Lightning-compatible p2p payment channel/swap network on CKB  
[**Documentation**](https://www.fiber.world/docs)  
**[Showcase](https://www.fiber.world/showcase)** - open source community projects on Fiber

[**Perun**](https://github.com/perun-network/perun-ckb-contract) - Ethereum-compatible p2p payment channel/swaps on CKB  


### Task: create your own basic application

**Now you can put what you’ve learned above to the test!**

Use your previous study and the tools above to create a basic application. It could, for example, allow users to generate their own tokens or mint spores (dobs). Alternatively, it could involve payment processing using CKB’s payment channels. Discuss your ideas with the programme director (Neon) and with members of CKB DevRel, they will be able to help guide you. If you have some development background already, you may wish to be more adventurous. 

If you need ideas, take a look at this opportunity map published recently: [**AI, machine payments, and Fiber in 2026: an opportunity map for CKB and Fiber developers**](https://talk.nervos.org/t/ai-machine-payments-and-fiber-in-2026-an-opportunity-map-for-ckb-and-fiber-developers/10665)

To aid your building, we also have a [**CKB AI scholarship initiative**](https://nervoscatalyst.org/ckb-ai-scholarship.html) which gives you access to subscription-grade AI models. Please note that this is a shared resource. Avoid unnecessary use to prevent reaching limits which may affect other users. You can also connect your AI to [**CKB AI MCP**](https://ckb-ai.ckbdev.com/) for better context during development.

There is always the chance a good idea opens the door to funding from the [Spark programme](https://talk.nervos.org/t/ckb-eco-fund-spark-program-mini-grant-initiative/8752) or the [CKB Community Fund DAO](https://talk.nervos.org/t/ckb-community-fund-dao-willing-to-back-every-ckb-buidler-up)!

#### **Further reading**

Deepen your understanding of CKB using this collection of core documents:  
[**GitHub - nervosnetwork/rfcs: This repository contains proposals, standards and documentations related to Nervos Network.**](https://github.com/nervosnetwork/rfcs)

## Intermediate

Want to further your understanding of CKB Development? Here are some modules to get you going:

**Script development course**  
[**Class 1: Validation Model**](https://docs.nervos.org/docs/script-course/intro-to-script-1)  
[**Class 2: Script Basics**](https://docs.nervos.org/docs/script-course/intro-to-script-2)  
[**Class 3: UDT**](https://docs.nervos.org/docs/script-course/intro-to-script-3)  
[**Class 4: WebAssembly on CKB**](https://docs.nervos.org/docs/script-course/intro-to-script-4)  
[**Class 5: Debugging**](https://docs.nervos.org/docs/script-course/intro-to-script-5)  
[**Class 6: Type ID**](https://docs.nervos.org/docs/script-course/intro-to-script-6)  
[**Class 7: Advanced Duktape Examples**](https://docs.nervos.org/docs/script-course/intro-to-script-7)  
[**Class 8: Performant WASM**](https://docs.nervos.org/docs/script-course/intro-to-script-8)  
[**Class 9: Cycle Reductions in Duktape Script**](https://docs.nervos.org/docs/script-course/intro-to-script-9)  
[**Class 10: Language Choices**](https://docs.nervos.org/docs/script-course/intro-to-script-10)

[**Detailed Rust scripting**](https://docs.nervos.org/docs/script/rust/rust-quick-start)

[**Detailed JS scripting**](https://docs.nervos.org/docs/script/js/js-quick-start)

[**Ecosystem scripts and libraries**](https://docs.nervos.org/docs/ecosystem-scripts/introduction)

[**Understanding Molecule and serialization**](https://docs.nervos.org/docs/serialization/serialization-molecule-in-ckb)

**Simple UDTs (sUDT)**   
A standard that defines the most basic implementation of a UDT (user defined token) fungible token on Nervos. An sUDT on Nervos is the equivalent of Ethereum tokens standards ERC20 or ERC777.

**[Standards document](https://docs-xi-two.vercel.app/docs/rfcs/0025-simple-udt/0025-simple-udt)**  
**[Tutorial](https://github.com/nervosnetwork/ckb-cli/wiki/UDT-%28sudt%29-Operations-Tutorial)**

**Nervos DAO**  
Nervos DAO enable users to lock CKBytes to get compensation from Nervos CKB secondary issuance. This process is similar to staking on other platforms. Nervos DAO provides a "virtual hardcap" for CKByte holders to insulate them from inflation.

**[Introduction](https://messari.io/copilot/share/understanding-nervos-dao-830c1d00-0afc-47aa-a65c-465701adc67f)**  
**[Deposit and Withdraw in Nervos DAO](https://github.com/nervosnetwork/rfcs/blob/master/rfcs/0023-dao-deposit-withdraw/0023-dao-deposit-withdraw.md)**  
**[Source code](https://github.com/nervosnetwork/ckb-system-scripts/blob/master/c/dao.c)**  
**[NervDAO - a browser implementation](https://github.com/ckb-devrel/nervdao)**

**Spore (DOBs)**  
Spore Protocol infuses digital objects (DOBs) with enduring value backed by tokenomics, redeemable at any time. Ensures true on-chain ownership, privacy, creative freedom and frictionless interaction.

**[Spore protocol introduction](https://docs.nervos.org/docs/tech-explanation/spore-protocol)**  
**[Comprehensive Spore documentation and tutorials](https://docs.spore.pro/)**  
**[DOB Cookbook](https://github.com/sporeprotocol/dob-cookbook)**  
**[How-to Recipes](https://docs.spore.pro/category/how-to-recipes)**  
**[SDK](https://docs.spore.pro/resources/spore-sdk), [Examples](https://docs.spore.pro/resources/examples), [Demos](https://docs.spore.pro/resources/demos), [Contracts](https://docs.spore.pro/resources/contracts)**  
**[DOB Decoder Standalone Server](https://github.com/sporeprotocol/dob-decoder-standalone-server)**

## Advanced

If you want to challenge yourself further and really grow your skills, study the following resources.

**Script-Sourced Rich Information (SSRI)**  
SSRI is is a protocol for strong binding of relevant information and conventions to the Script itself on CKB. It allows developers to embed arbitrary information/logic into the Script, describing its behavior and managing data to off-chain applications.

[**Introduction**](https://talk.nervos.org/t/en-cn-script-sourced-rich-information-script/8256/2)  
[**SDK for implementing SSRI-compliant smart contracts on CKB**](https://crates.io/crates/ckb-ssri-std)  
**[First prototype of SSRI-Compliant contract](https://github.com/Hanssen0/ssri-test)**  
**[Server for calling SSRI methods](https://github.com/ckb-devrel/ssri-server)**  
[**Further Documentation**](https://docs.rs/ckb-ssri-std/0.0.1/ckb_ssri_std/)  
[**Pausable UDT: The first fully SSRI compliant and production ready contract that exemplifies all use cases that SSRI protocol covers.**](https://github.com/Alive24/pausable-udt)  
**[Audit report](https://github.com/Alive24/pausable-udt/blob/master/20241224-Pausable-UDT-Final-Audit-Report.pdf)**

**RGB++ - Bitcoin’s Turing Catalyst**  
RGB++ is an **asset issuance protocol for Bitcoin that leverages the flexibility and programmability of CKB** to enable dynamic use cases for Bitcoin assets. It unlocks the full potential of Bitcoin for a rich application ecosystem without altering its foundational security. This production-ready protocol offers a trustless method for dApps to interact directly with native Bitcoin transactions, facilitated by a lightweight, decentralized verifier. Website: [www.rgbpp.com](http://www.rgbpp.com) 

[**Introduction**](https://rgbpp.com/docs/introduction)  
[**RGB++ Protocol Light Paper**](https://talk.nervos.org/t/rgb-protocol-light-paper-translation/7790)  
[**RGB++ Resource Hub**](https://rgbpp.com/docs/resources), including Script Standard, CKB Bitcoin SPV, API service for asset management, API endpoints and development tools   
[**RGB++ SDK**](https://github.com/ckb-devrel/ccc/tree/rgbpp-sdk)  
[**RGB++ Explorer**](https://explorer.rgbpp.io/en)

**xUDT - Extensible User Defined Token standard**  
xUDT (Extensible User-Defined Token) is a token standard for creating and managing fungible tokens on Nervos CKB. xUDT builds upon the foundational Simple User-Defined Token (sUDT), which uses a minimal, predefined validation model. In contrast, xUDT introduces extensibility by allowing developers to attach custom validation logic via external Scripts — enabling more advanced governance, minting rules, and token behaviors.

[**Introduction**](https://docs.nervos.org/docs/tech-explanation/xudt)  
[How xUDT works](https://docs.nervos.org/docs/ecosystem-scripts/xudt#how-xudt-works)  
[Enhance sUDT’s Programmability with xUDT](https://blog.cryptape.com/enhance-sudts-programmability-with-xudt)  
[xUDT RFC](https://github.com/nervosnetwork/rfcs/blob/master/rfcs/0052-extensible-udt/0052-extensible-udt.md)  

**iCKB**  
iCKB is a protocol that aims to solve the illiquidity problem of the NervosDAO by tokenizing NervosDAO deposits into a liquid iCKB token. Website: [www.ickb.org](http://www.ickb.org)

[**Whitepaper**](https://ickb.org/)  
**[Github repository](https://github.com/ickb/)**  
**[Audit report](https://scalebit.xyz/reports/20240911-ICKB-Final-Audit-Report.pdf)**  
**[iCKB SDK](https://github.com/ickb/sdk)**  
**[Unlocking CKB Liquidity: A Deep Dive into the iCKB Protocol](https://www.nervos.org/knowledge-base/Unlocking_CKB_Liquidity_iCKB)**

## Reading list: Understanding CKB

* [**Nervos Nation videos**](https://www.youtube.com/c/NervosNation) **explaining core CKB concepts visually at a high level, including UTXO/POW, RISC-V, Cell Model, RGB++, and more**  
* [**Nervos: An In-Depth Overview of a Blockchain Network Built for Modularity**](https://www.nervos.org/knowledge-base/nervos_overview_of_a_layered_blockchain)  
* [**Common Knowledge Base (CKB): Understanding Our Ethos**](https://www.nervos.org/knowledge-base/ckb_understanding_our_ethos)  
* [**A Deep Dive Into the Tokenomics of Nervos Network**](https://www.nervos.org/knowledge-base/tokenomics_of_nervos_network)  
* [**Where we're going, we don't need Accounts: the Future of Onboarding**](https://www.nervos.org/knowledge-base/account_abstraction_where_were_going)  
* [**Common Knowledge Base (CKB): a Blockchain Developer’s Dream**](https://www.nervos.org/knowledge-base/ckb_blockchain_developers_dream)  
* [**Bitcoin vs. CKB: Two Approaches to Achieving Sustainable Security**](https://www.nervos.org/knowledge-base/bitcoin_and_ckb_security_models)  
* [**Web5: Extra Decentralized**](https://www.nervos.org/knowledge-base/web5-extra-decentralized)  
* [**Comparing Blockchain Virtual Machines (VMs): EVM, WASM, SVM, and CKB-VM**](https://www.nervos.org/knowledge-base/comparing_blockchain_virtual_machines)

