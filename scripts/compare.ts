import { ApiPromise, WsProvider } from 'https://deno.land/x/polkadot@0.2.40/api/mod.ts';
import { Option, Text, Vec } from 'https://deno.land/x/polkadot@0.2.40/types-codec/bundle.ts';
import { MetadataLatest, PalletConstantMetadataLatest, PalletMetadataLatest, PalletStorageMetadataV14,  PortableType,  RuntimeVersion,  Si1Field,  Si1Variant, SiTypeDef, StorageEntryMetadataLatest } from 'https://deno.land/x/polkadot@0.2.40/types/interfaces/types.ts';

// All the comparisons
class Compare {
    static storage = (palletName: string, left: Storage, right: Storage) => {
        const formatStorage = (types: Vec<PortableType>, v: StorageEntryMetadataLatest) => `name: ${v.name.toString()}, modifier: ${v.modifier.toString()}, type: ${getTypeName(types, v)}, docs: ${v.fallback.toString()}`;
        const diff: string[] = [];
    
        left.values.filter((v) => right.values.find((v2) => v2.name.eq(v.name)) == undefined).forEach((v) => diff.push(`(NEW) ${formatStorage(left.types, v)}`))
        right.values.filter((v) => left.values.find((v2) => v2.name.eq(v.name)) == undefined).forEach((v) => diff.push(`(REMOVED) ${formatStorage(right.types, v)}`))
    
        left.values.map((v, i) => [i, right.values.findIndex((v2) => v2.name.eq(v.name))]).filter((x) => x[1] != -1).forEach((x) => {
            const [v, v2] = [left.values[x[0]], right.values[x[1]]];
            
            if (v.modifier.toString() != v2.modifier.toString()) {
                diff.push(`(CHANGED) ${v.name.toString()} Modifier ${v2.modifier.toString()} -> ${v.modifier.toString()}`)
            }
    
            const [type1, type2] = [getTypeName(left.types, v), getTypeName(right.types, v2)];
            if (type1 != type2) {
                diff.push(`(CHANGED) ${v.name.toString()} Type`)
                diff.push(`  - ${type2}`)
                diff.push(`  + ${type1}`)
            }
        })
    
        storage.entires.push({palletName: palletName, values: diff })
    }

    static constants = (left: PalletConstantMetadataLatest[] , right: PalletConstantMetadataLatest[]) => {
        const diff: string[] = []
        left.map((e, index) => [index, right.findIndex((e2) => e2.name.eq(e.name))]).filter((x) => x[1] != -1).forEach((x) => {
            Compare.constantsInner(left[x[0]], right[x[1]]).forEach((val) => diff.push(val));
        });
    
        left.filter((e) => right.find((e2) => e2.name.eq(e.name)) == undefined).forEach((e) => diff.push(`(NEW) ${e}`))
        right.filter((e) => left.find((e2) => e2.name.eq(e.name)) == undefined).forEach((e) => diff.push(`(REMOVED) ${e}`))
    
        return diff;
    }

    static constantsInner = (left: PalletConstantMetadataLatest, right: PalletConstantMetadataLatest): string[]  => {
        const diff: string[] = []
        if (!right.value.eq(left.value)) {
            diff.push(`(CHANGED) ${right.name} Value`)
            diff.push(`  - ${right.value.toHuman()}`)
            diff.push(`  + ${left.value.toHuman()}`)
        }
    
        return diff;
    }

    static variants = (left: Si1Variant[], right: Si1Variant[]): string[] => {
        const diff: string[] = []
        left.map((e, index) => [index, right.findIndex((e2) => e2.name.eq(e.name))]).filter((x) => x[1] != -1).forEach((x) => {
            Compare.variantsInner(left[x[0]], right[x[1]]).forEach((val) => diff.push(val));
        });
    
        left.filter((e) => right.find((e2) => e2.name.eq(e.name)) == undefined).forEach((e) => diff.push(`(NEW) ${e}`))
        right.filter((e) => left.find((e2) => e2.name.eq(e.name)) == undefined).forEach((e) => diff.push(`(REMOVED) ${e}`))
    
        return diff;
    }
    
    static variantsInner = (left: Si1Variant, right: Si1Variant): string[] => {
        const diff: string[] = []
        if (right.index.cmp(left.index) != 0) {
            diff.push(`(CHANGED) ${right.name} Index ${right.index} -> ${left.index}`)
        }
    
        // We need to serialize the fields, remove the field `type` and then do the comparison
        const leftFields = left.fields.map((val) => {return {name: val.name, typeName: val.typeName, docs: val.docs}});
        const rightFields = right.fields.map((val) => {return {name: val.name, typeName: val.typeName, docs: val.docs}});
    
        if (leftFields.toString() != rightFields.toString()) {
            diff.push(`(CHANGED) ${right.name} Field`)
            diff.push(`  - ${right.fields.toString()}`)
            diff.push(`  + ${left.fields.toString()}`)
        }
    
        return diff;
    }

    static runtimeVersion = (left: RuntimeVersion, right: RuntimeVersion): string[]  => {
        const diff: string[] = [];
        const comparisons = [
            [right.specName, left.specName, `(CHANGED) Spec Name ${right.specName} -> ${left.specName}`],
            [right.implName, left.implName, `(CHANGED) Impl Name ${right.implName} -> ${left.implName}`],
            [right.authoringVersion, left.authoringVersion, `(CHANGED) Authoring Version ${right.authoringVersion} -> ${left.authoringVersion}`],
            [right.specVersion, left.specVersion, `(CHANGED) Spec Version ${right.specVersion} -> ${left.specVersion}`],
            [right.implVersion, left.implVersion, `(CHANGED) Impl Version ${right.implVersion} -> ${left.implVersion}`],
            [right.transactionVersion, left.transactionVersion, `(CHANGED) Transaction Version ${right.transactionVersion} -> ${left.transactionVersion}`],
            [right.stateVersion, left.stateVersion, `(CHANGED) State Version ${right.stateVersion} -> ${left.stateVersion}`],
        ]
        comparisons.filter((c) => c[0].toString() != c[1].toString()).forEach((c) => diff.push(c[2] as string));

        return diff;
    }

    static runtimeChain = (left: Text, right: Text): string[]  => {
        if (left.eq(right)) {
            return [];
        }
        return [`(CHANGED) Name ${right} -> ${left}`];
    }
}

type DiffEntry = { palletName: string, values: string[]};
type DiffEntires =  { name: string, entires: DiffEntry[]}

const writeToFile = (entires: DiffEntires[]) => {
    let textToWrite = ""

    entires.filter((e) => e.entires.length > 0).forEach((e) => {
        textToWrite += `${e.name}\n`;
        for (const elements of e.entires) {
            if (elements.palletName.length > 0) {
                textToWrite += `  ${elements.palletName}\n`
            }
            elements.values.forEach((val) => textToWrite += `    ${val.toString()}\n`)
        }
    })

    Deno.writeTextFileSync("diff.txt", textToWrite);
}

class Pallet {
    name: string;
    events: Si1Variant[] = []
    errors: Si1Variant[] = []
    calls: Si1Variant[] = []
    constants: PalletConstantMetadataLatest[] = []
    storage: Option<PalletStorageMetadataV14>

    constructor(pallet: PalletMetadataLatest, metadata: MetadataLatest) {
        this.name = pallet.name.toString();

        if (pallet.events.isSome) {
            const id = pallet.events.value.type.toNumber();
            this.events = metadata.lookup.types[id].type.def.asVariant.variants.toArray();
        }
        if (pallet.errors.isSome) {
            const id = pallet.errors.value.type.toNumber();
            this.errors = metadata.lookup.types[id].type.def.asVariant.variants.toArray();
        }
        if (pallet.calls.isSome) {
            const id = pallet.calls.value.type.toNumber();
            this.calls = metadata.lookup.types[id].type.def.asVariant.variants.toArray();
        }

        this.storage  = pallet.storage;
        this.constants = pallet.constants.toArray();
    }
}

class Runtime {
    api: ApiPromise
    pallets: Pallet[] = []
    metadata: MetadataLatest

    constructor(api: ApiPromise) {
        this.api = api
        this.metadata = api.runtimeMetadata.asLatest;
        this.metadata.pallets.forEach((p) => this.pallets.push(new Pallet(p, this.metadata)));
    }

    findPallet(name: string): Pallet | undefined {
        return this.pallets.find((p) => p.name === name);
    }
}

const serializeVariants = (variants: Vec<Si1Variant>): string => {
    let value = "";
    for (const variant of variants) {
        value += `{name: ${variant.name.toString()}, fields: [${serializeFields(variant.fields)}]}, `
    }
    return value.slice(0, -2)
}

const serializeFields = (fields: Vec<Si1Field>): string => {
    let value = ""
    for (const field of fields) {
        value += `{name: ${field.name.toString()}, typeName: ${field.typeName.toString()}, docs: ${field.docs.toString()}}, `
    }
    return value.slice(0, -2)
}

const getTypeName = (types: Vec<PortableType>, storage: SiTypeDef | StorageEntryMetadataLatest): string => {
    if ("isPrimitive" in storage) {
        if (storage.isPrimitive) {
            return storage.asPrimitive.toString()
        } 
        if (storage.isTuple) {
            const left = getTypeName(types, types[storage.asTuple[0].toNumber()].type.def)
            const right = getTypeName(types, types[storage.asTuple[1].toNumber()].type.def)
            return `(${left}, ${right})`
        } 
        if (storage.isSequence) {
            return `Sequence: ${getTypeName(types, types[storage.asSequence.type.toNumber()].type.def)}`
        }
        if (storage.isVariant) {
            return `Variants: [${serializeVariants(storage.asVariant.variants)}]`;
        }
        if (storage.isComposite) {
            return `Composite: [${serializeFields(storage.asComposite.fields)}]`;
        }
        if (storage.isArray) {
            return `Array: { len ${storage.asArray.len.toString()}, type: ${getTypeName(types, types[storage.asArray.type.toNumber()].type.def)} }`;
        }
    } else {
        if (storage.type.isPlain) {
            return getTypeName(types, types[storage.type.asPlain.toNumber()].type.def)
        }
        if (storage.type.isMap) {
            const key =  getTypeName(types, types[storage.type.asMap.key.toNumber()].type.def)
            const value =  getTypeName(types, types[storage.type.asMap.value.toNumber()].type.def)
            return `Key: ${key}, value ${value}`
        }
    }

    return storage.toString();
}

type Storage = {types: Vec<PortableType>, values: StorageEntryMetadataLatest[] }
const newStorage = (types: Vec<PortableType>, p: Pallet | undefined): Storage => { return { types, values: p != null ? p.storage.isSome ? p.storage.value.items.toArray() : [] : []} };

const endpoint1_url = Deno.args[0];
const endpoint2_url = Deno.args[1];
console.log(`First Runtime url: ${endpoint1_url}, Second Runtime url: ${endpoint2_url}`);

const api1 = await ApiPromise.create({ provider: new WsProvider(endpoint1_url) });
const api2 = await ApiPromise.create({ provider: new WsProvider(endpoint2_url) });

const [runtime1, runtime2] = [new Runtime(api1), new Runtime(api2)]
const [pallets1, pallets2] = [runtime1.pallets, runtime2.pallets]

const errors: DiffEntires = {name: "Errors", entires: []};
const events: DiffEntires = {name: "Events", entires: []};
const calls: DiffEntires = {name: "Calls", entires: []};
const storage: DiffEntires = {name: "Storage", entires: []};
const constants: DiffEntires = {name: "Constants", entires: []};
const runtimeVersion: DiffEntires = {name: "Runtime Version", entires: []};
const runtimeChain: DiffEntires = {name: "Runtime Chain", entires: []};

const comparisons = (palletName: string, p1: Pallet | undefined, p2: Pallet | undefined) => {
    errors.entires.push({ palletName, values: Compare.variants(p1 ? p1.errors : [], p2 ? p2.errors : []) });
    events.entires.push({ palletName, values: Compare.variants(p1 ? p1.events : [], p2 ? p2.events : []) });
    calls.entires.push({ palletName, values: Compare.variants(p1 ? p1.calls : [], p2 ? p2.calls : []) });
    constants.entires.push({ palletName, values: Compare.constants(p1 ? p1.constants : [], p2 ? p2.constants : []) });
}
const innerJoin = (pa: Pallet[], p2a: Pallet[]) => pa.map((p, i) => [i, p2a.findIndex((p2) => p2.name == p.name)]).filter((x) => x[1] != -1).map((x) => [pa[x[0]], p2a[x[1]]])
const leftJoin = (pa: Pallet[], p2a: Pallet[]) => pa.filter((p) => p2a.find((p2) => p2.name == p.name) == undefined)

// Errors, Events, Calls, Constants
leftJoin(pallets1, pallets2).forEach((p) => comparisons(p.name, p, undefined));
leftJoin(pallets2, pallets1).forEach((p) => comparisons(p.name, undefined, p));
innerJoin(pallets1, pallets2).forEach((x) => comparisons(x[0].name, x[0], x[1]));

// Storage
const [types, types2] = [runtime1.metadata.lookup.types, runtime2.metadata.lookup.types]
leftJoin(pallets1, pallets2).forEach((p) => Compare.storage(p.name, newStorage(types, p), newStorage(types2, undefined)));
leftJoin(pallets2, pallets1).forEach((p) => Compare.storage(p.name, newStorage(types, undefined), newStorage(types2, p)));
innerJoin(pallets1, pallets2).forEach((x) => Compare.storage(x[0].name, newStorage(types, x[0]), newStorage(types2, x[1])));

// Runtime version and Chain name
runtimeVersion.entires.push({palletName: "", values: Compare.runtimeVersion(runtime1.api.runtimeVersion, runtime2.api.runtimeVersion)});
runtimeChain.entires.push({palletName: "", values: Compare.runtimeChain(runtime1.api.runtimeChain, runtime2.api.runtimeChain)});

// Let's clean up our arrays
[errors, events, calls, storage, constants, runtimeVersion, runtimeChain].forEach((x) => x.entires = x.entires.filter((x) => x.values.length > 0));
writeToFile([errors, events, calls, storage, constants, runtimeVersion, runtimeChain])

Deno.exit()