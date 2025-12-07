# Dump-Roblox-Api

Dumps all roblox classes and enums into either a luau table or Rust structs and values.

For luau enums, you might want to try `SomeEnum:GetEnumItems()` for EnumItems or `Enum:GetEnums()` for all enums
in luau itself.

The most helpful results returned by this script is `Class Names` and dumped `Classes`.

# Before compiling:
Make sure you have rust installed. That's all you need.

# Compile:
To compile, download the repository and run `cargo build -r` in the root of the repository.
The executable will be in `target/release`.

# Save to file:
- Linux and Windows:
`dump-roblox-api --some-options -abc > output.txt`
# Luau type definitions:

```lua
type DumpedMember = {
    ["Category"]: string,
    ["MemberType"]: string,
    ["Name"]: string,
    ["ValueType"]: {
        ["Name"]: string,
        ["Category"]: string
    }
}
type DumpedClass = {
    ["Members"]: {DumpedMember},
    ["MemoryCategory"]: string,
    ["Superclass"]: string,
    ["Tags"]: {string}
}
```

# Example output:
(truncated to keep short)
```lua
local ClassNames = {
	"Instance",
	"Accoutrement",
	"Accessory",
	...
	"VirtualUser",
	"VisibilityCheckDispatcher",
	"VisibilityService",
	"Visit",
	"VoiceChatInternal",
	"VoiceChatService",
	"WeldConstraint",
	"Wire"
}

local Classes = {
	["Instance"] = {
		["Members"] = {
			{
				["Category"] = "Behavior",
				["MemberType"] = "Property",
				["Name"] = "Archivable",
				["ValueType"] = {
					["Name"] = "bool",
					["Category"] = "Primitive"
				}
			},
			{
				["Category"] = "Data",
				["MemberType"] = "Property",
				["Name"] = "ClassName",
				["ValueType"] = {
					["Name"] = "string",
					["Category"] = "Primitive"
				}
			},
			...
			{
				["Category"] = "",
				["MemberType"] = "Event",
				["Name"] = "Destroying",
				["ValueType"] = {
					["Name"] = "",
					["Category"] = ""
				}
			},
			{
				["Category"] = "",
				["MemberType"] = "Event",
				["Name"] = "childAdded",
				["ValueType"] = {
					["Name"] = "",
					["Category"] = ""
				}
			}
		},
		["MemoryCategory"] = "Instances",
		["Superclass"] = "<<<ROOT>>>",
		["Tags"] = {
			"NotCreatable",
			"NotBrowsable"
		}
	},
	...
	["WeldConstraint"] = {
		["Members"] = {
			{
				["Category"] = "Behavior",
				["MemberType"] = "Property",
				["Name"] = "Active",
				["ValueType"] = {
					["Name"] = "bool",
					["Category"] = "Primitive"
				}
			},
			{
				["Category"] = "Behavior",
				["MemberType"] = "Property",
				["Name"] = "Enabled",
				["ValueType"] = {
					["Name"] = "bool",
					["Category"] = "Primitive"
				}
			},
			{
				["Category"] = "Parts",
				["MemberType"] = "Property",
				["Name"] = "Part0",
				["ValueType"] = {
					["Name"] = "BasePart",
					["Category"] = "Class"
				}
			},
			{
				["Category"] = "Parts",
				["MemberType"] = "Property",
				["Name"] = "Part1",
				["ValueType"] = {
					["Name"] = "BasePart",
					["Category"] = "Class"
				}
			}
		},
		["MemoryCategory"] = "PhysicsParts",
		["Superclass"] = "Instance",
		["Tags"] = {

		}
	},
	["Wire"] = {
		["Members"] = {
			{
				["Category"] = "Data",
				["MemberType"] = "Property",
				["Name"] = "Connected",
				["ValueType"] = {
					["Name"] = "bool",
					["Category"] = "Primitive"
				}
			},
			{
				["Category"] = "Data",
				["MemberType"] = "Property",
				["Name"] = "SourceInstance",
				["ValueType"] = {
					["Name"] = "Instance",
					["Category"] = "Class"
				}
			},
			{
				["Category"] = "Data",
				["MemberType"] = "Property",
				["Name"] = "SourceName",
				["ValueType"] = {
					["Name"] = "string",
					["Category"] = "Primitive"
				}
			},
			{
				["Category"] = "Data",
				["MemberType"] = "Property",
				["Name"] = "TargetInstance",
				["ValueType"] = {
					["Name"] = "Instance",
					["Category"] = "Class"
				}
			},
			{
				["Category"] = "Data",
				["MemberType"] = "Property",
				["Name"] = "TargetName",
				["ValueType"] = {
					["Name"] = "string",
					["Category"] = "Primitive"
				}
			}
		},
		["MemoryCategory"] = "Instances",
		["Superclass"] = "Instance",
		["Tags"] = {
			"NotBrowsable"
		}
	}
}

local Enums = {
	["AccessModifierType"] = {
		[0] = "Allow",
		[1] = "Deny"
	},
	["AccessoryType"] = {
		[0] = "Unknown",
		[1] = "Hat",
		[2] = "Hair",
		[3] = "Face",
		[4] = "Neck",
		[5] = "Shoulder",
		[6] = "Front",
		[7] = "Back",
		[8] = "Waist",
		[9] = "TShirt",
		[10] = "Shirt",
		[11] = "Pants",
		[12] = "Jacket",
		[13] = "Sweater",
		[14] = "Shorts",
		[15] = "LeftShoe",
		[16] = "RightShoe",
		[17] = "DressSkirt",
		[18] = "Eyebrow",
		[19] = "Eyelash"
	},
	["ActionType"] = {
		[0] = "Nothing",
		[1] = "Pause",
		[2] = "Lose",
		[3] = "Draw",
		[4] = "Win"
	},
	...
	["WrapLayerDebugMode"] = {
		[0] = "None",
		[1] = "BoundCage",
		[2] = "LayerCage",
		[3] = "BoundCageAndLinks",
		[4] = "Reference",
		[5] = "Rbf",
		[6] = "OuterCage",
		[7] = "ReferenceMeshAfterMorph",
		[8] = "HSROuterDetail",
		[9] = "HSROuter",
		[10] = "HSRInner",
		[11] = "HSRInnerReverse",
		[12] = "LayerCageFittedToBase",
		[13] = "LayerCageFittedToPrev"
	},
	["WrapTargetDebugMode"] = {
		[0] = "None",
		[1] = "TargetCageOriginal",
		[2] = "TargetCageCompressed",
		[3] = "TargetCageInterface",
		[4] = "TargetLayerCageOriginal",
		[5] = "TargetLayerCageCompressed",
		[6] = "TargetLayerInterface",
		[7] = "Rbf",
		[8] = "OuterCageDetail"
	},
	["ZIndexBehavior"] = {
		[0] = "Global",
		[1] = "Sibling"
	}
}

```