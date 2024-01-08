import { readFileSync, writeFileSync, copyFileSync } from "fs"
import { resolve, join, basename } from "path"

const packagePath = process.cwd()
const buildPath = join(packagePath, "./build")

const writeJson = (targetPath, obj) => writeFileSync(targetPath, JSON.stringify(obj, null, 2), "utf8")

async function createPackageFile() {
  const packageData = JSON.parse(readFileSync(resolve(packagePath, "./package.json"), "utf8"))
  const newPackageData = {
    ...packageData,
    main: "./index.js",
    types: "./index.d.ts",
    typesVersions: {
      "*": {
        chain: ["./chain/index.d.ts"],
        spec: ["./spec/index.d.ts"],
      },
    },
  }

  delete newPackageData.scripts
  delete newPackageData.devDependencies

  const targetPath = resolve(buildPath, "./package.json")
  writeJson(targetPath, newPackageData)
}

async function includeFileInBuild(file) {
  const sourcePath = resolve(packagePath, file)
  const targetPath = resolve(buildPath, basename(file))
  copyFileSync(sourcePath, targetPath)
}

async function run() {
  try {
    await createPackageFile()
    await includeFileInBuild("./readme.md")
    // await includeFileInBuild("./LICENSE")
  } catch (err) {
    console.error(err)
    process.exit(1)
  }
}

run()
