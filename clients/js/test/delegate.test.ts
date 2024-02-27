import { generateSigner } from '@metaplex-foundation/umi';
import test from 'ava';
// import { base58 } from '@metaplex-foundation/umi/serializers';
import {
  AssetWithPlugins,
  DataState,
  PluginType,
  addAuthority,
  addPlugin,
  create,
  fetchAssetWithPlugins,
  updatePlugin,
} from '../src';
import { createUmi } from './_setup';

test('it can delegate a new authority', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const assetAddress = generateSigner(umi);
  const delegateAddress = generateSigner(umi);

  // When we create a new account.
  await create(umi, {
    dataState: DataState.AccountState,
    assetAddress,
    name: 'Test Bread',
    uri: 'https://example.com/bread',
    plugins: [],
  }).sendAndConfirm(umi);

  await addPlugin(umi, {
    assetAddress: assetAddress.publicKey,
    plugin: {
      __kind: 'Freeze',
      fields: [{ frozen: false }],
    },
  }).sendAndConfirm(umi);

  await addAuthority(umi, {
    assetAddress: assetAddress.publicKey,
    pluginType: PluginType.Freeze,
    newAuthority: {
      __kind: 'Pubkey',
      address: delegateAddress.publicKey,
    },
  }).sendAndConfirm(umi);

  const asset = await fetchAssetWithPlugins(umi, assetAddress.publicKey);
  // console.log(asset);
  t.like(asset, <AssetWithPlugins>{
    publicKey: assetAddress.publicKey,
    updateAuthority: umi.identity.publicKey,
    owner: umi.identity.publicKey,
    name: 'Test Bread',
    uri: 'https://example.com/bread',
    pluginHeader: {
      key: 3,
      pluginRegistryOffset: BigInt(119),
    },
    pluginRegistry: {
      key: 4,
      registry: [
        {
          pluginType: 2,
          offset: BigInt(117),
          authorities: [
            { __kind: 'Owner' },
            { __kind: 'Pubkey', address: delegateAddress.publicKey },
          ],
        },
      ],
    },
    plugins: [
      {
        authorities: [
          { __kind: 'Owner' },
          { __kind: 'Pubkey', address: delegateAddress.publicKey },
        ],
        plugin: {
          __kind: 'Freeze',
          fields: [{ frozen: false }],
        },
      },
    ],
  });
});

test('a delegate can freeze the token', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const assetAddress = generateSigner(umi);
  const delegateAddress = generateSigner(umi);

  // When we create a new account.
  await create(umi, {
    dataState: DataState.AccountState,
    assetAddress,
    name: 'Test Bread',
    uri: 'https://example.com/bread',
    plugins: [],
  }).sendAndConfirm(umi);

  await addPlugin(umi, {
    assetAddress: assetAddress.publicKey,
    plugin: {
      __kind: 'Freeze',
      fields: [{ frozen: false }],
    },
  }).sendAndConfirm(umi);

  await addAuthority(umi, {
    assetAddress: assetAddress.publicKey,
    pluginType: PluginType.Freeze,
    newAuthority: {
      __kind: 'Pubkey',
      address: delegateAddress.publicKey,
    },
  }).sendAndConfirm(umi);

  await updatePlugin(umi, {
    assetAddress: assetAddress.publicKey,
    plugin: {
      __kind: 'Freeze',
      fields: [{ frozen: true }],
    },
  }).sendAndConfirm(umi);

  const asset = await fetchAssetWithPlugins(umi, assetAddress.publicKey);
  // console.log(asset);
  t.like(asset, <AssetWithPlugins>{
    publicKey: assetAddress.publicKey,
    updateAuthority: umi.identity.publicKey,
    owner: umi.identity.publicKey,
    name: 'Test Bread',
    uri: 'https://example.com/bread',
    pluginHeader: {
      key: 3,
      pluginRegistryOffset: BigInt(119),
    },
    pluginRegistry: {
      key: 4,
      registry: [
        {
          pluginType: 2,
          offset: BigInt(117),
          authorities: [
            { __kind: 'Owner' },
            { __kind: 'Pubkey', address: delegateAddress.publicKey },
          ],
        },
      ],
    },
    plugins: [
      {
        authorities: [
          { __kind: 'Owner' },
          { __kind: 'Pubkey', address: delegateAddress.publicKey },
        ],
        plugin: {
          __kind: 'Freeze',
          fields: [{ frozen: true }],
        },
      },
    ],
  });
});
