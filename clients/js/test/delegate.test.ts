import { generateSigner } from '@metaplex-foundation/umi';
import test from 'ava';
// import { base58 } from '@metaplex-foundation/umi/serializers';
import {
  // Authority,
  DataState,
  // Delegate,
  // Key,
  // PluginHeaderAccountData,
  // PluginRegistryAccountData,
  // RegistryData,
  // RegistryRecord,
  create,
  // delegate,
  // fetchAsset,
  // getAssetAccountDataSerializer,
  // getDelegateSerializer,
  // getPluginHeaderAccountDataSerializer,
  // getPluginRegistryAccountDataSerializer,
  // PluginType,
  // freeze,
  // getPluginSerializer,
  // Plugin,
} from '../src';
import { createUmi } from './_setup';

test('it can delegate a new authority', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const assetAddress = generateSigner(umi);
  // const delegateAddress = generateSigner(umi);

  // When we create a new account.
  await create(umi, {
    dataState: DataState.AccountState,
    assetAddress,
    name: 'Test Bread',
    uri: 'https://example.com/bread',
  }).sendAndConfirm(umi);

  // // Then an account was created with the correct data.
  // const asset = await fetchAsset(umi, assetAddress.publicKey);
  // // console.log("Before Plugins:\n", asset);
  // const assetData = getAssetAccountDataSerializer().serialize(asset);

  // await delegate(umi, {
  //   assetAddress: assetAddress.publicKey,
  //   owner: umi.identity,
  //   delegate: delegateAddress.publicKey,
  // }).sendAndConfirm(umi);

  // const pluginData = await umi.rpc.getAccount(assetAddress.publicKey);
  // if (pluginData.exists) {
  //   const pluginHeader = getPluginHeaderAccountDataSerializer().deserialize(pluginData.data, assetData.length)[0];
  //   // console.log("After Plugins:\n", pluginHeader);
  //   t.like(pluginHeader, <PluginHeaderAccountData>{
  //     key: Key.PluginHeader,
  //     pluginRegistryOffset: BigInt(119),
  //   });
  //   const pluginRegistry = getPluginRegistryAccountDataSerializer().deserialize(pluginData.data, Number(pluginHeader.pluginRegistryOffset))[0];
  //   // console.log(JSON.stringify(pluginRegistry, (_, v) => typeof v === 'bigint' ? v.toString() : v, 2));
  //   t.like(pluginRegistry, <PluginRegistryAccountData>{
  //     key: Key.PluginRegistry,
  //     registry: [<RegistryRecord>{
  //       pluginType: PluginType.Delegate,
  //       data: <RegistryData>{
  //         offset: BigInt(117),
  //         authorities: [<Authority>{ __kind: 'Pubkey', address: delegateAddress.publicKey }],
  //       },
  //     }],
  //     externalPlugins: [],
  //   });
  //   const delegatePlugin = getDelegateSerializer().deserialize(pluginData.data, Number(pluginRegistry.registry[0].data.offset))[0];
  //   // console.log(delegatePlugin);
  //   t.like(delegatePlugin, <Delegate>{
  //     frozen: false,
  //   });
  // } else {
  //   t.fail("Plugin data not found");
  // }

  t.pass();
});

test('it can delegate several new authorities', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const assetAddress = generateSigner(umi);
  // const delegateAddress0 = generateSigner(umi);
  // const delegateAddress1 = generateSigner(umi);

  // When we create a new account.
  await create(umi, {
    dataState: DataState.AccountState,
    assetAddress,
    name: 'Test Bread',
    uri: 'https://example.com/bread',
  }).sendAndConfirm(umi);

  // // Then an account was created with the correct data.
  // const asset = await fetchAsset(umi, assetAddress.publicKey);
  // // console.log("Before Plugins:\n", asset);
  // const assetData = getAssetAccountDataSerializer().serialize(asset);

  // await delegate(umi, {
  //   assetAddress: assetAddress.publicKey,
  //   owner: umi.identity,
  //   delegate: delegateAddress0.publicKey,
  // }).sendAndConfirm(umi);

  // await delegate(umi, {
  //   assetAddress: assetAddress.publicKey,
  //   owner: umi.identity,
  //   delegate: delegateAddress1.publicKey,
  // }).sendAndConfirm(umi);

  // const pluginData = await umi.rpc.getAccount(assetAddress.publicKey);
  // if (pluginData.exists) {
  //   const pluginHeader = getPluginHeaderAccountDataSerializer().deserialize(pluginData.data, assetData.length)[0];
  //   // console.log("After Plugins:\n", pluginHeader);
  //   t.like(pluginHeader, <PluginHeaderAccountData>{
  //     key: Key.PluginHeader,
  //     pluginRegistryOffset: BigInt(119),
  //   });
  //   const pluginRegistry = getPluginRegistryAccountDataSerializer().deserialize(pluginData.data, Number(pluginHeader.pluginRegistryOffset))[0];
  //   // console.log(JSON.stringify(pluginRegistry, (_, v) => typeof v === 'bigint' ? v.toString() : v, 2));
  //   t.like(pluginRegistry, <PluginRegistryAccountData>{
  //     key: Key.PluginRegistry,
  //     registry: [<RegistryRecord>{
  //       pluginType: PluginType.Delegate,
  //       data: <RegistryData>{
  //         offset: BigInt(117),
  //         authorities: [<Authority>{ __kind: 'Pubkey', address: delegateAddress0.publicKey }, <Authority>{ __kind: 'Pubkey', address: delegateAddress1.publicKey }],
  //       },
  //     }],
  //     externalPlugins: [],
  //   });
  //   const delegatePlugin = getDelegateSerializer().deserialize(pluginData.data, Number(pluginRegistry.registry[0].data.offset))[0];
  //   // console.log(delegatePlugin);
  //   t.like(delegatePlugin, <Delegate>{
  //     frozen: false,
  //   });
  // } else {
  //   t.fail("Plugin data not found");
  // }

  t.pass();
});

test('a delegate can freeze the token', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const assetAddress = generateSigner(umi);
  // const delegateAddress = generateSigner(umi);

  // When we create a new account.
  await create(umi, {
    dataState: DataState.AccountState,
    assetAddress,
    name: 'Test Bread',
    uri: 'https://example.com/bread',
  }).sendAndConfirm(umi);

  // // Then an account was created with the correct data.
  // const asset = await fetchAsset(umi, assetAddress.publicKey);
  // // console.log("Before Plugins:\n", asset);
  // const assetData = getAssetAccountDataSerializer().serialize(asset);

  // await delegate(umi, {
  //   assetAddress: assetAddress.publicKey,
  //   owner: umi.identity,
  //   delegate: delegateAddress.publicKey,
  // })
  //   .append(freeze(umi, {
  //     assetAddress: assetAddress.publicKey,
  //     delegate: delegateAddress
  //   }))
  //   .sendAndConfirm(umi);

  // const pluginData = await umi.rpc.getAccount(assetAddress.publicKey);
  // if (pluginData.exists) {
  //   // console.log("Plugin Data:\n", pluginData.data);
  //   const pluginHeader = getPluginHeaderAccountDataSerializer().deserialize(pluginData.data, assetData.length)[0];
  //   // console.log("After Plugins:\n", pluginHeader);
  //   t.like(pluginHeader, <PluginHeaderAccountData>{
  //     key: Key.PluginHeader,
  //     pluginRegistryOffset: BigInt(119),
  //   });
  //   const pluginRegistry = getPluginRegistryAccountDataSerializer().deserialize(pluginData.data, Number(pluginHeader.pluginRegistryOffset))[0];
  //   // console.log(JSON.stringify(pluginRegistry, (_, v) => typeof v === 'bigint' ? v.toString() : v, 2));
  //   t.like(pluginRegistry, <PluginRegistryAccountData>{
  //     key: Key.PluginRegistry,
  //     registry: [<RegistryRecord>{
  //       pluginType: PluginType.Delegate,
  //       data: <RegistryData>{
  //         offset: BigInt(117),
  //         authorities: [<Authority>{ __kind: 'Pubkey', address: delegateAddress.publicKey }],
  //       },
  //     }],
  //     externalPlugins: [],
  //   });
  //   const delegatePlugin = getPluginSerializer().deserialize(pluginData.data, Number(pluginRegistry.registry[0].data.offset))[0];
  //   // console.log(delegatePlugin);
  //   t.like(delegatePlugin, <Plugin>{
  //     __kind: 'Delegate',
  //     fields: [{ frozen: true }]
  //   });
  // } else {
  //   t.fail("Plugin data not found");
  // }

  t.pass();
});