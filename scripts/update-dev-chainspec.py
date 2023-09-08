#!/usr/bin/python3
import sys
import json
import uuid
import re


def get_balance_for_node(node_name):
    if re.match("validator-", node_name) or re.match("election-", node_name):
        return 1000000000000000000000000
    return 1000000000000000000000


work_dir = sys.argv[1]


with open(work_dir + "/devnet.template.json", 'r') as chainspec_file:
    chainspec = json.load(chainspec_file)

with open(work_dir + "/master.json", 'r') as master_file:
    nodes = json.load(master_file)


chainspec["id"] =  "da_devnet" + "_" + str(uuid.uuid1())
chainspec["name"] = "Avail-Devnet"
chainspec["chainType"] = "Live"
chainspec["protocolId"] = "da1"

rt = chainspec["genesis"]["runtime"]

rt["balances"]["balances"] = []
rt["staking"]["stakers"] = []
rt["staking"]["minNominatorBond"] = 10000000000000000000
rt["staking"]["minValidatorBond"] = 1000000000000000000000
rt["session"]["keys"] = []
rt["technicalCommittee"]["members"] = []
rt["elections"]["members"] = []

for node in nodes:
    rt["balances"]["balances"].append([
        nodes[node]["wallet"]["sr25519"]["ss58PublicKey"],
        get_balance_for_node(node)
    ])
    if re.match("validator-", node):
        rt["staking"]["stakers"].append([
            nodes[node]["wallet"]["sr25519"]["ss58PublicKey"],
            nodes[node]["wallet"]["sr25519"]["ss58PublicKey"],
            1000000000000000000000,
            "Validator"
        ])
        rt["session"]["keys"].append([
            nodes[node]["wallet"]["sr25519"]["ss58PublicKey"],
            nodes[node]["wallet"]["sr25519"]["ss58PublicKey"],
            {
                "babe": nodes[node]["wallet"]["sr25519"]["ss58PublicKey"],
                "grandpa": nodes[node]["wallet"]["ed25519"]["ss58PublicKey"],
                "im_online": nodes[node]["wallet"]["sr25519"]["ss58PublicKey"],
                "authority_discovery": nodes[node]["wallet"]["sr25519"]["ss58PublicKey"]
            }
        ])
    if re.match("tech-committee-", node):
        rt["technicalCommittee"]["members"].append(nodes[node]["wallet"]["sr25519"]["ss58PublicKey"])
    if re.match("election-", node):
        rt["elections"]["members"].append([
            nodes[node]["wallet"]["sr25519"]["ss58PublicKey"],
            1000000000000000000
        ])

new_app_keys = []
for key in rt["dataAvailability"]["appKeys"]:
    key[1]["owner"] = nodes["sudo-01"]["wallet"]["sr25519"]["ss58PublicKey"]
    new_app_keys.append(key)
rt["dataAvailability"]["appKeys"] = new_app_keys

rt["sudo"]["key"] = nodes["sudo-01"]["wallet"]["sr25519"]["ss58PublicKey"]

with open(work_dir + "/populated.devnet.chainspec.json", 'w') as f:
    json.dump(chainspec, f, indent = 4)



