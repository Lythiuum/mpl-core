/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import {
  Context,
  Pda,
  PublicKey,
  Signer,
  TransactionBuilder,
  transactionBuilder,
} from '@metaplex-foundation/umi';
import {
  Serializer,
  mapSerializer,
  struct,
  u8,
} from '@metaplex-foundation/umi/serializers';
import {
  ResolvedAccount,
  ResolvedAccountsWithIndices,
  getAccountMetasAndSigners,
} from '../shared';
import {
  Authority,
  AuthorityArgs,
  PluginType,
  PluginTypeArgs,
  getAuthoritySerializer,
  getPluginTypeSerializer,
} from '../types';

// Accounts.
export type RemovePluginAuthorityInstructionAccounts = {
  /** The address of the asset */
  asset: PublicKey | Pda;
  /** The collection to which the asset belongs */
  collection?: PublicKey | Pda;
  /** The owner or delegate of the asset */
  authority?: Signer;
  /** The account paying for the storage fees */
  payer?: Signer;
  /** The system program */
  systemProgram?: PublicKey | Pda;
  /** The SPL Noop Program */
  logWrapper?: PublicKey | Pda;
};

// Data.
export type RemovePluginAuthorityInstructionData = {
  discriminator: number;
  pluginType: PluginType;
  authorityToRemove: Authority;
};

export type RemovePluginAuthorityInstructionDataArgs = {
  pluginType: PluginTypeArgs;
  authorityToRemove: AuthorityArgs;
};

export function getRemovePluginAuthorityInstructionDataSerializer(): Serializer<
  RemovePluginAuthorityInstructionDataArgs,
  RemovePluginAuthorityInstructionData
> {
  return mapSerializer<
    RemovePluginAuthorityInstructionDataArgs,
    any,
    RemovePluginAuthorityInstructionData
  >(
    struct<RemovePluginAuthorityInstructionData>(
      [
        ['discriminator', u8()],
        ['pluginType', getPluginTypeSerializer()],
        ['authorityToRemove', getAuthoritySerializer()],
      ],
      { description: 'RemovePluginAuthorityInstructionData' }
    ),
    (value) => ({ ...value, discriminator: 10 })
  ) as Serializer<
    RemovePluginAuthorityInstructionDataArgs,
    RemovePluginAuthorityInstructionData
  >;
}

// Args.
export type RemovePluginAuthorityInstructionArgs =
  RemovePluginAuthorityInstructionDataArgs;

// Instruction.
export function removePluginAuthority(
  context: Pick<Context, 'identity' | 'programs'>,
  input: RemovePluginAuthorityInstructionAccounts &
    RemovePluginAuthorityInstructionArgs
): TransactionBuilder {
  // Program ID.
  const programId = context.programs.getPublicKey(
    'mplCore',
    'CoREzp6dAdLVRKf3EM5tWrsXM2jQwRFeu5uhzsAyjYXL'
  );

  // Accounts.
  const resolvedAccounts: ResolvedAccountsWithIndices = {
    asset: { index: 0, isWritable: true, value: input.asset ?? null },
    collection: { index: 1, isWritable: true, value: input.collection ?? null },
    authority: { index: 2, isWritable: false, value: input.authority ?? null },
    payer: { index: 3, isWritable: true, value: input.payer ?? null },
    systemProgram: {
      index: 4,
      isWritable: false,
      value: input.systemProgram ?? null,
    },
    logWrapper: {
      index: 5,
      isWritable: false,
      value: input.logWrapper ?? null,
    },
  };

  // Arguments.
  const resolvedArgs: RemovePluginAuthorityInstructionArgs = { ...input };

  // Default values.
  if (!resolvedAccounts.authority.value) {
    resolvedAccounts.authority.value = context.identity;
  }
  if (!resolvedAccounts.systemProgram.value) {
    resolvedAccounts.systemProgram.value = context.programs.getPublicKey(
      'splSystem',
      '11111111111111111111111111111111'
    );
    resolvedAccounts.systemProgram.isWritable = false;
  }

  // Accounts in order.
  const orderedAccounts: ResolvedAccount[] = Object.values(
    resolvedAccounts
  ).sort((a, b) => a.index - b.index);

  // Keys and Signers.
  const [keys, signers] = getAccountMetasAndSigners(
    orderedAccounts,
    'programId',
    programId
  );

  // Data.
  const data = getRemovePluginAuthorityInstructionDataSerializer().serialize(
    resolvedArgs as RemovePluginAuthorityInstructionDataArgs
  );

  // Bytes Created On Chain.
  const bytesCreatedOnChain = 0;

  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
  ]);
}