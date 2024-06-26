/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import {
  Context,
  Option,
  OptionOrNullable,
  Pda,
  PublicKey,
  Signer,
  TransactionBuilder,
  transactionBuilder,
} from '@metaplex-foundation/umi';
import {
  Serializer,
  array,
  mapSerializer,
  option,
  string,
  struct,
  u8,
} from '@metaplex-foundation/umi/serializers';
import {
  ResolvedAccount,
  ResolvedAccountsWithIndices,
  getAccountMetasAndSigners,
} from '../shared';
import {
  DataState,
  DataStateArgs,
  PluginAuthorityPair,
  PluginAuthorityPairArgs,
  getDataStateSerializer,
  getPluginAuthorityPairSerializer,
} from '../types';

// Accounts.
export type CreateV1InstructionAccounts = {
  /** The address of the new asset */
  asset: Signer;
  /** The collection to which the asset belongs */
  collection?: PublicKey | Pda;
  /** The authority signing for creation */
  authority?: Signer;
  /** The account paying for the storage fees */
  payer?: Signer;
  /** The owner of the new asset. Defaults to the authority if not present. */
  owner?: PublicKey | Pda;
  /** The authority on the new asset */
  updateAuthority?: PublicKey | Pda;
  /** The system program */
  systemProgram?: PublicKey | Pda;
  /** The SPL Noop Program */
  logWrapper?: PublicKey | Pda;
};

// Data.
export type CreateV1InstructionData = {
  discriminator: number;
  dataState: DataState;
  name: string;
  uri: string;
  plugins: Option<Array<PluginAuthorityPair>>;
};

export type CreateV1InstructionDataArgs = {
  dataState?: DataStateArgs;
  name: string;
  uri: string;
  plugins?: OptionOrNullable<Array<PluginAuthorityPairArgs>>;
};

export function getCreateV1InstructionDataSerializer(): Serializer<
  CreateV1InstructionDataArgs,
  CreateV1InstructionData
> {
  return mapSerializer<
    CreateV1InstructionDataArgs,
    any,
    CreateV1InstructionData
  >(
    struct<CreateV1InstructionData>(
      [
        ['discriminator', u8()],
        ['dataState', getDataStateSerializer()],
        ['name', string()],
        ['uri', string()],
        ['plugins', option(array(getPluginAuthorityPairSerializer()))],
      ],
      { description: 'CreateV1InstructionData' }
    ),
    (value) => ({
      ...value,
      discriminator: 0,
      dataState: value.dataState ?? DataState.AccountState,
      plugins: value.plugins ?? [],
    })
  ) as Serializer<CreateV1InstructionDataArgs, CreateV1InstructionData>;
}

// Args.
export type CreateV1InstructionArgs = CreateV1InstructionDataArgs;

// Instruction.
export function createV1(
  context: Pick<Context, 'payer' | 'programs'>,
  input: CreateV1InstructionAccounts & CreateV1InstructionArgs
): TransactionBuilder {
  // Program ID.
  const programId = context.programs.getPublicKey(
    'mplCore',
    'CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d'
  );

  // Accounts.
  const resolvedAccounts = {
    asset: {
      index: 0,
      isWritable: true as boolean,
      value: input.asset ?? null,
    },
    collection: {
      index: 1,
      isWritable: true as boolean,
      value: input.collection ?? null,
    },
    authority: {
      index: 2,
      isWritable: false as boolean,
      value: input.authority ?? null,
    },
    payer: {
      index: 3,
      isWritable: true as boolean,
      value: input.payer ?? null,
    },
    owner: {
      index: 4,
      isWritable: false as boolean,
      value: input.owner ?? null,
    },
    updateAuthority: {
      index: 5,
      isWritable: false as boolean,
      value: input.updateAuthority ?? null,
    },
    systemProgram: {
      index: 6,
      isWritable: false as boolean,
      value: input.systemProgram ?? null,
    },
    logWrapper: {
      index: 7,
      isWritable: false as boolean,
      value: input.logWrapper ?? null,
    },
  } satisfies ResolvedAccountsWithIndices;

  // Arguments.
  const resolvedArgs: CreateV1InstructionArgs = { ...input };

  // Default values.
  if (!resolvedAccounts.payer.value) {
    resolvedAccounts.payer.value = context.payer;
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
  const data = getCreateV1InstructionDataSerializer().serialize(
    resolvedArgs as CreateV1InstructionDataArgs
  );

  // Bytes Created On Chain.
  const bytesCreatedOnChain = 0;

  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
  ]);
}
