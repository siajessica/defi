declare module "zokrates-js" {
  export type Curve = "bn128" | "bls12_381" | "bls12_377" | "bw6_761";
  export type Scheme = "g16" | "gm17" | "marlin";

  export type VerificationKey = object;
  export type ProvingKey = Uint8Array;

  export type ResolveCallback = (
    location: string,
    path: string
  ) => ResolverResult;

  export interface CompileConfig {
    isolate_branches?: boolean;
  }

  export interface CompileOptions {
    curve?: Curve;
    location?: string;
    resolveCallback?: ResolveCallback;
    config?: CompileConfig;
  }

  export type Proof = {
    proof: object;
    inputs: string[];
  };

  export interface ResolverResult {
    source: string;
    location: string;
  }

  export interface ComputationResult {
    witness: string;
    output: string;
  }

  export interface Abi {
    inputs: Array<any>;
    outputs: Array<any>;
  }

  export interface CompilationArtifacts {
    program: Uint8Array;
    abi: Abi;
  }

  export interface SetupKeypair {
    vk: VerificationKey;
    pk: ProvingKey;
  }

  export type Options = {
    scheme: Scheme;
    curve: Curve;
  };

  export interface ZoKratesProvider {
    withOptions(options: Options): ZoKratesProvider;
    compile(
      source: string,
      compileOptions?: CompileOptions
    ): CompilationArtifacts;
    computeWitness(
      input: CompilationArtifacts | Uint8Array,
      args: any[]
    ): ComputationResult;
    setup(program: Uint8Array): SetupKeypair;
    universalSetup(size: number): Uint8Array;
    setupWithSrs(srs: Uint8Array, program: Uint8Array): SetupKeypair;
    generateProof(
      program: Uint8Array,
      witness: string,
      provingKey: Uint8Array
    ): Proof;
    verify(verificationKey: VerificationKey, proof: Proof): boolean;
    exportSolidityVerifier(verificationKey: VerificationKey): string;
    utils: {
      formatProof(proof: Proof): any[];
    }
  }

  export interface Metadata {
    version: string;
  }

  export function initialize(): Promise<ZoKratesProvider>;
  export var metadata: Metadata;
}
