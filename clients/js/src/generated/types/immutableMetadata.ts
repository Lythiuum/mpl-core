/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import { Serializer, struct } from '@metaplex-foundation/umi/serializers';

export type ImmutableMetadata = {};

export type ImmutableMetadataArgs = ImmutableMetadata;

export function getImmutableMetadataSerializer(): Serializer<
  ImmutableMetadataArgs,
  ImmutableMetadata
> {
  return struct<ImmutableMetadata>([], {
    description: 'ImmutableMetadata',
  }) as Serializer<ImmutableMetadataArgs, ImmutableMetadata>;
}
