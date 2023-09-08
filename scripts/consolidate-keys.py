#!/usr/local/bin/python
import sys
import json

work_dir = sys.argv[1]

with open(work_dir + "/nodecount.txt", 'r') as node_list:
    node_data  = node_list.read().splitlines()


master_list = {}

for node in node_data:
    node_info = {}
    node_info["tag_name"] = node
    tpl = {
        "title": "",
        "category": "SERVER",
        "sections": [
            {
                "id": "wallet",
                "label": "Wallet"
            }
        ],
        "fields": []
    }


    wallet = {}
    p2p = {}
    with open(work_dir + "/" + node_info["tag_name"] + ".wallet.ed25519.json", 'r') as ed25519_wallet:
        wallet["ed25519"] = json.load(ed25519_wallet)
    with open(work_dir + "/" + node_info["tag_name"] + ".wallet.sr25519.json", 'r') as sr25519_wallet:
        wallet["sr25519"] = json.load(sr25519_wallet)
    with open(work_dir + "/" + node_info["tag_name"] + ".private.key", 'r') as priv_key:
        p2p["private"] = priv_key.read()
    with open(work_dir + "/" + node_info["tag_name"] + ".public.key", 'r') as pub_key:
        p2p["public"] = pub_key.read()

    node_info["wallet"] = wallet
    node_info["p2p"] = p2p
    master_list[node_info["tag_name"]] = node_info

    tpl["title"] = "Wallet Credentials for " + node_info["tag_name"]
    tpl["fields"].append({
        "id": "notesPlain",
        "type": "STRING",
        "purpose": "NOTES",
        "label": "notesPlain",
        "value": "These credentials are automatically generated for the avail devnet"
    })
    tpl["fields"].append({
        "id": "recoveryPhrase",
        "type": "CONCEALED",
        "label": "Secret Phrase",
        "value": wallet["sr25519"]["secretPhrase"]
    })
    tpl["fields"].append({
        "id": "walletAddressSr25519",
        "type": "STRING",
        "label": "Sr25519 Address",
        "value": wallet["sr25519"]["ss58PublicKey"]
    })
    tpl["fields"].append({
        "id": "walletAddressEd25519",
        "type": "STRING",
        "label": "Ed25519 Address",
        "value": wallet["ed25519"]["ss58PublicKey"]
    })
    tpl["fields"].append({
        "id": "libP2PPub",
        "type": "STRING",
        "label": "Libp2p Public",
        "value": p2p["public"].strip()
    })
    tpl["fields"].append({
        "id": "libP2PPriv",
        "label": "Libp2p Private",
        "type": "CONCEALED",
        "value": p2p["private"].strip()
    })
    with open(work_dir + "/" + node_info["tag_name"] + ".op.tpl.json", 'w') as f:
        json.dump(tpl, f)

with open(work_dir + "/master.json", 'w') as f:
    json.dump(master_list, f)


