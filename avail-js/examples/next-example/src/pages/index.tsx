import { MouseEventHandler, useState } from "react"
import { ApiPromise, initialize, signedExtensions, types } from "avail-js-sdk"
import { isNumber } from "@polkadot/util"
import { SignerOptions } from "@polkadot/api/types"
import Head from "next/head"

export default function Home() {
  const [foundExtensions, setFoundExtensions] = useState<{
    [extensionName: string]: { version: string; enable: Function }
  }>({})
  const [extensionsInitialized, setExtensionsInitialized] = useState<Record<string, boolean>>({})
  const [availApi, setAvailApi] = useState<ApiPromise | undefined>()
  const [logs, setLogs] = useState<{ message: string; severity: "info" | "error" }[]>([])

  const findExtension = async () => {
    // Init Extension
    const { web3Enable } = await import("@polkadot/extension-dapp")
    await web3Enable("Example with extension")

    const web3Window = window as any
    if (web3Window.injectedWeb3 as any) {
      setFoundExtensions(web3Window.injectedWeb3)
    }
  }

  const getInjectorMetadata = (api: ApiPromise) => {
    return {
      chain: api.runtimeChain.toString(),
      specVersion: api.runtimeVersion.specVersion.toNumber(),
      tokenDecimals: api.registry.chainDecimals[0] || 18,
      tokenSymbol: api.registry.chainTokens[0] || "AVL",
      genesisHash: api.genesisHash.toHex(),
      ss58Format: isNumber(api.registry.chainSS58) ? api.registry.chainSS58 : 0,
      chainType: "substrate" as "substrate",
      icon: "substrate",
      types: types as any,

      /** !! IMPORTANT !!
       * This is the important part, we tell the extension how to handle our signedExtension (even if it seems it's already there)
       **/
      userExtensions: signedExtensions,
    }
  }

  const sendTx = async (extension: string) => {
    try {
      // Import extension utils
      const { web3Accounts, web3FromSource } = await import("@polkadot/extension-dapp")

      // Init API
      let api = availApi
      if (!(api && api.isConnected)) {
        api = await initialize()
        setAvailApi(api)
      }

      // Get correct extension account / injector
      const accounts = await web3Accounts()
      const filteredAccounts = accounts.filter((x) => x.meta.source === extension)
      if (filteredAccounts.length === 0) throw new Error("No account found")
      const account = filteredAccounts.find((x) => x.address.startsWith("5CDG")) || filteredAccounts[0]
      const injector = await web3FromSource(account.meta.source)

      // Inject our specific metadata once
      if (injector.metadata) {
        if (!extensionsInitialized[extension]) {
          const metadata = getInjectorMetadata(api)
          await injector.metadata.provide(metadata)
          // It would be wise to put this in a persistent storage to not ask everytime
          setExtensionsInitialized({ ...extensionsInitialized, [injector.name]: true })
        }
      }

      // Send the transaction
      const tx = api.tx.dataAvailability.submitData("0x123456")
      addLogs(`Sending tx with account ${account.address} and wallet ${extension}`, "info")
      await tx.signAndSend(
        account.address,
        { signer: injector.signer, app_id: 1 } as Partial<SignerOptions>,
        ({ status, isError, events }) => {
          if (isError) {
            addLogs("An error has occured, open console to view logs", "error")
            console.log(events)
          }
          if (status.isInBlock) {
            addLogs(`Transaction included in block: ${status.asInBlock}`, "info")
          }
        },
      )
    } catch (err: any) {
      addLogs(err.message ? err.message : err, "error")
    }
  }

  const addLogs = (message: string, severity: "info" | "error") => {
    setLogs((prevLogs) => [...prevLogs, { message, severity }])
  }

  const LogsDisplay = () => {
    return (
      <div style={{ marginTop: "24px" }}>
        <table style={{ width: "100%" }}>
          <tbody>
            {logs.map((log, index) => (
              <tr key={index}>
                <td
                  style={{
                    border: "1px solid black",
                    padding: "8px",
                    color: log.severity === "error" ? "red" : "white",
                  }}
                >
                  {log.message}
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    )
  }

  const Button = ({ onClick, label }: { label: string; onClick: MouseEventHandler<HTMLButtonElement> }) => {
    return (
      <button
        style={{ marginTop: "12px", border: "1px solid white", padding: "6px", borderRadius: "8px", width: "240px" }}
        onClick={onClick}
      >
        {label}
      </button>
    )
  }

  return (
    <>
      <Head>
        <title>Avail test app</title>
        <meta name="description" content="A simple app to test connection with polkadot js" />
      </Head>
      <div style={{ display: "flex", alignItems: "center", justifyContent: "center", flexDirection: "column" }}>
        <h1 style={{ fontSize: "32px" }}>Use extension</h1>

        <Button onClick={() => findExtension()} label={"Detect extensions"} />

        {Object.keys(foundExtensions).map((extension, i) => {
          return <Button key={i} onClick={() => sendTx(extension)} label={`Send TX with ${extension}`} />
        })}

        {logs.length > 0 && <LogsDisplay />}
      </div>
    </>
  )
}
