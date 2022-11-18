

// #[derive(Debug)]
// pub struct Rule{
//     id: String,
//     title: String,
//     description: String,
//     rule: String,
//     recommendation: String,
// }

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct RulesDataBase{
    pub id: String,
    pub title: String,
    pub description: String,
    pub location: Vec<String>,
    pub rule: String,
    pub recommendation: String,
}


pub fn gas_op_issues() -> Vec<RulesDataBase> {
    let rules = vec![
        RulesDataBase{
			id: "G-08".to_owned(),
			title: "Contracts using unlocked pragma.".to_owned(),
			description: "Contracts in scope use `pragma solidity ^0.X.Y` or `pragma solidity >0.X.Y`, allowing wide range of versions.".to_owned(),
			rule: "pragma solidity  (\\^|>)".to_owned(),
			recommendation: "Consider locking compiler version, for example `pragma solidity 0.8.6`. This can have additional benefits, for example using custom errors to save gas and so forth.".to_owned(),
            ..Default::default()
        },
        RulesDataBase{
			id: "G-08".to_owned(),
			title: "Cache Array Length Outside of Loop".to_owned(),
			description: "Reading array length at each iteration of the loop takes 6 gas (3 for mload and 3 to place memory_offset) in the stack. Caching the array length in the stack saves around 3 gas per iteration.".to_owned(),
			rule: "(for.*\\.length)".to_owned(),
			recommendation: "Store the array’s length in a variable before the for-loop.".to_owned(),
            ..Default::default()
        },
        RulesDataBase{
			id: "G-16".to_owned(),
			title: "Use a more recent version of Solidity".to_owned(),
			description: r"Use a solidity version of at least 0.8.2 to get compiler automatic inlining <br />Use a solidity version of at least 0.8.3 to get better struct packing and cheaper multiple storage reads <br /> Use a solidity version of at least 0.8.4 to get `bytes.concat()` instead of `abi.encodePacked(<bytes>,<bytes>)` and custom errors <br /> Use a solidity version of at least 0.8.12 to get `string.concat()` instead of `abi.encodePacked(<str>,<str>)` <br /> Use a solidity version of at least 0.8.13 to get the ability to use `using for` with a list of free functions".to_owned(),
			rule: "(pragma solidity \\^0.[8-9].[0-10]|pragma solidity >0.[8-9].[0-10]|pragma solidity 0.[8-9].[4-9]|pragma solidity \\^0.[4-7].[0-9]|pragma solidity >0.[4-7].[0-9]|pragma solidity 0.[4-7].[0-9])".to_owned(),
			recommendation: "Use `x = x + y` instead of `x += y`".to_owned(),
            ..Default::default()
		},
		//Will need manual review
		//TODO: improve detection to do away with manual review
		RulesDataBase{
			id: "G-14".to_owned(),
			title: "Functions guaranteed to revert when called by normal users can be marked `payable`".to_owned(),
			description: "If a function modifier such as `onlyOwner` is used, the function will revert if a normal user tries to pay the function. Marking the function as `payable` will lower the gas cost for legitimate callers because the compiler will not include checks for whether a payment was provided.".to_owned(),
			rule: "(?i:.*external.*only.\\w*|.*only.\\w*.*external|.*public.*only.\\w*|.*only.\\w*.*public)".to_owned(),
			recommendation: "Consider marking above functions as payable".to_owned(),
            ..Default::default()
		},
		RulesDataBase{
			id: "G-15".to_owned(),
			title: "`++i/i++` should be `unchecked{++i}/unchecked{i++}` when it is not possible for them to overflow, as is the case when used in `for/while` loops".to_owned(),
			description: "This saves 30-60 gas [per loop](https://gist.github.com/hrkrshnn/ee8fabd532058307229d65dcd5836ddc#the-increment-in-for-loop-post-condition-can-be-made-unchecked)".to_owned(),
			rule: "(i\\++|i \\+= 1|i\\--|[a-z,A-Z]*\\++\\)|[a-z,A-Z]*\\++[[:blank:]]\\)|[a-z,A-Z]*\\--|i \\-= 1)".to_owned(),
			recommendation: "Consider doing incrementation/decrementation `unchecked{}`".to_owned(),
            ..Default::default()
		},
        RulesDataBase{
			id: "G-17".to_owned(),
			title: "Multiple address mappings can be combined into a single mapping of an address to a struct, where appropriate".to_owned(),
			description: "Saves a storage slot for the mapping. Depending on the circumstances and sizes of types, can avoid a Gsset (20000 gas) per mapping combined.Reads and subsequent writes can also be cheaper when a function requires both values and they both fit in the same storage slot".to_owned(),
			rule: "(mapping\\(address.*|mapping \\(address.*)((.|\\n)*)(mapping\\(address.*|mapping \\(address.*)".to_owned(),
			recommendation: "Consider combining mappings where appropriate".to_owned(),
            ..Default::default()
		},
		RulesDataBase{
			id: "G-02".to_owned(),
			title: "Use `!= 0` instead of `> 0` for Unsigned Integer Comparison in require statements".to_owned(),
			description: "`!= 0` is cheapear than `> 0` when comparing unsigned integers in require statements.".to_owned(),
			rule: "(require.*>0|require.*> 0)".to_owned(),
			recommendation: "Use `!= 0` instead of `> 0`.".to_owned(),
			..Default::default()
		},
		RulesDataBase{
			id: "G-03".to_owned(),
			title: "Reduce the size of error messages (Long revert Strings).".to_owned(),
			description: "Shortening revert strings to fit in 32 bytes will decrease deployment time gas and will decrease runtime gas when the revert condition is met. Revert strings that are longer than 32 bytes require at least one additional mstore, along with additional overhead for computing memory offset, etc.".to_owned(),
			rule: "(require|revert)\\(.*,?.(\"|\').{33,}(\"|\')\\)".to_owned(),
			recommendation: "Shorten the revert strings to fit in 32 bytes, or use custom errors if >0.8.4.".to_owned(),
			..Default::default()
		},
		RulesDataBase{
			id: "G-04".to_owned(),
			title: "Use Custom Errors instead of Revert Strings.".to_owned(),
			description: "Custom errors from Solidity 0.8.4 are cheaper than revert strings (cheaper deployment cost and runtime cost when the revert condition is met)".to_owned(),
			rule: "(require|revert)\\(.*,?\".*\"\\)".to_owned(),
			recommendation: "Use custom errors instead of revert strings.".to_owned(),
			..Default::default()
		},
		RulesDataBase{
			id: "G-05".to_owned(),
			title: "No need to initialize variables with default values".to_owned(),
			description: "If a variable is not set/initialized, it is assumed to have the default value (0, false, 0x0 etc depending on the data type). Explicitly initializing it with its default value is an anti-pattern and wastes gas.".to_owned(),
			rule: "(uint[0-9]*[[:blank:]][a-z,A-Z,0-9]*.?=.?0;)|(bool.[a-z,A-Z,0-9]*.?=.?false;)|(int[0-9]*[[:blank:]][a-z,A-Z,0-9]*.?=.?0;)".to_owned(),
			recommendation: "Remove explicit default initializations.".to_owned(),
			..Default::default()
		},
		RulesDataBase{
			id: "G-06".to_owned(),
			title: "`++i` costs less gas compared to `i++` or `i += 1`".to_owned(),
			description: "`++i` costs less gas compared to `i++` or `i += 1` for unsigned integer, as pre-increment is cheaper (about 5 gas per iteration). This statement is true even with the optimizer enabled.".to_owned(),
			rule: "(i\\++|i \\+= 1|i\\--|[a-z,A-Z]*\\++\\)|[a-z,A-Z]*\\++[[:blank:]]\\)|[a-z,A-Z]*\\--|i \\-= 1)".to_owned(),
			recommendation: "Use `++i` instead of `i++` to increment the value of an `uint` variable. Same thing for `--i` and `i--`.".to_owned(),
			..Default::default()
		},
		RulesDataBase{
			id: "G-09".to_owned(),
			title: "Empty blocks should be removed or emit something".to_owned(),
			description: "Empty blocks should be removed or emit something. Waste of gas.".to_owned(),
			rule: "(function.*\\{\\s*\\})".to_owned(),
			recommendation: "The code should be refactored such that they no longer exist, or the block should do something useful, such as emitting an event or reverting.".to_owned(),
			..Default::default()
		},
		RulesDataBase{
			id: "G-10".to_owned(),
			title: "Use `calldata` instead of `memory` for read-only arguments in `external` functions.".to_owned(),
			description: "When a function with a `memory` array is called externally, the `abi.decode()` step has to use a for-loop to copy each index of the `calldata` to the `memory` index. Each iteration of this for-loop costs at least 60 gas (i.e. 60 * <mem_array>.length). Using calldata directly, obliviates the need for such a loop in the contract code and runtime execution.".to_owned(),
			rule: "function.?\\([^)]*\\[\\] memory [^)]*\\)[^{]*(external|view)[^{]*(external|view)".to_owned(),
			recommendation: "Use `calldata` instead of `memory`.".to_owned(),
			..Default::default()
		},
		RulesDataBase{
			id: "G-13".to_owned(),
			title: "Use assembly to check for address(0)".to_owned(),
			description: "Saves 6 gas per instance if using assembly to check for zero address".to_owned(),
			rule: "!=address\\(0\\)|!= address\\(0\\)".to_owned(),
			recommendation: "Consider using assembly to check for zero address checks".to_owned(),
			..Default::default()
		},
		RulesDataBase{
			id: "G-19".to_owned(),
			title: "Using `private` rather than `public` for constants, saves gas".to_owned(),
			description: "If needed, the value can be read from the verified contract source code. Savings are due to the compiler not having to create non-payable getter functions for deployment calldata, and not adding another entry to the method ID table".to_owned(),
			rule: "(public.?constant.?|constant.?public.?)[^=\\n\\(]*(=|;)".to_owned(),
			recommendation: "Consider changing above findings to `private`".to_owned(),
			..Default::default()
		},

    ];
    rules
}

pub fn low_issues() -> Vec<RulesDataBase> {
    let rules = vec![
		// L-01 - Unsafe ERC20 Operation(s)
		RulesDataBase{
			id: "L-01".to_owned(),
			title: "Unsafe ERC20 Operation(s)".to_owned(),
			description: "The return value of an external `transfer`/`transferFrom`/`approve` call is not checked".to_owned(),
			rule: "\\.transfer\\(|\\.transferFrom\\(|\\.approve\\(".to_owned(), // ".tranfer(", ".transferFrom(" or ".approve("
			recommendation: "Use `SafeERC20`, or ensure that the `transfer`/`transferFrom` return value is checked.".to_owned(),
			..Default::default()
		},
		// L-02 - Unspecific Compiler Version Pragma
		RulesDataBase{
			id: "L-02".to_owned(),
			title: "Unspecific Compiler Version Pragma".to_owned(),
			description: "A known vulnerable compiler version may accidentally be selected or security tools might fall-back to an older compiler version ending up checking a different EVM compilation that is ultimately deployed on the blockchain.".to_owned(),
			rule: "pragma solidity (\\^|>)".to_owned(), // "pragma solidity ^" or "pragma solidity >"
			recommendation: "Avoid floating pragmas for non-library contracts. It is recommended to pin to a concrete compiler version.".to_owned(),
			..Default::default()
		},
		// L-03 - Do not use Deprecated Library Functions
		RulesDataBase{
			id: "L-03".to_owned(),
			title: "Do not use Deprecated Library Functions".to_owned(),
			description: "The usage of deprecated library functions should be discouraged.".to_owned(),
			rule: "_setupRole\\(|safeApprove\\(|latestAnswer".to_owned(), // _setupRole and safeApprove are common deprecated lib functions
			recommendation: "Use `safeIncreaseAllowance` / `safeDecreaseAllowance` instead of `safeApprove`.".to_owned(),
			..Default::default()
		},
		// L-04 - Open TODOs
		RulesDataBase{
			id: "L-04".to_owned(),
			title: "Open TODOs".to_owned(),
			description: "There are many open TODOs throughout the various code files.".to_owned(),
			rule: "TODO".to_owned(),
			recommendation: "Remove TODO's before deployment".to_owned(),
			..Default::default()
		},
		// L-05 - ecrecover()
		RulesDataBase{
			id: "L-05".to_owned(),
			title: "`ecrecover()` not checked for signer address of zero".to_owned(),
			description: "The `ecrecover()` function returns an address of zero when the signature does not match. This can cause problems if address zero is ever the owner of assets, and someone uses the permit function on address zero. If that happens, any invalid signature will pass the checks, and the assets will be stealable. ".to_owned(),
			rule: "(address*[[:blank:]][a-z,A-Z,0-9]*.?=.?ecrecover.*;)".to_owned(),
			recommendation: "Add a check to ensure `ecrecover()` does not return an address of zero.".to_owned(),
			..Default::default()
		},
		// L-06 - `_safeMint()` should be used rather than `_mint()` wherever possible.
		RulesDataBase{
			id: "L-06".to_owned(),
			title: "`_safeMint()` should be used rather than `_mint()` wherever possible.".to_owned(),
			description: "`_mint()` is [discouraged](https://github.com/OpenZeppelin/openzeppelin-contracts/blob/d4d8d2ed9798cc3383912a23b5e8d5cb602f7d4b/contracts/token/ERC721/ERC721.sol#L271) in favor of `_safeMint()` which ensures that the recipient is either an EOA or implements `IERC721Receiver`.".to_owned(),
			rule: r"_mint\(.*\)".to_owned(),
			recommendation: "Use either [OpenZeppelin's](https://github.com/OpenZeppelin/openzeppelin-contracts/blob/d4d8d2ed9798cc3383912a23b5e8d5cb602f7d4b/contracts/token/ERC721/ERC721.sol#L238-L250) or [solmate's](https://github.com/transmissions11/solmate/blob/4eaf6b68202e36f67cab379768ac6be304c8ebde/src/tokens/ERC721.sol#L180) version of this function.".to_owned(),
			..Default::default()
		},
		// L-07 - Expressions for constant values such as a call to `keccak256()`, should use `immutable` rather than `constant`.
		RulesDataBase{
			id: "L-07".to_owned(),
			title: "Expressions for constant values such as a call to `keccak256()`, should use `immutable` rather than `constant`.".to_owned(),
			description: "".to_owned(),
			rule: ".*constant.*=.*keccak256\\(.*\\)".to_owned(),
			recommendation: "".to_owned(),
			..Default::default()
		},
		RulesDataBase{
			id: "L-08".to_owned(),
			title:"`abi.encodePacked()` should not be used with dynamic types when passing the result to a hash function such as `keccak256()`".to_owned(),
		  	description: "Use `abi.encode()` instead which will pad items to 32 bytes, which will [prevent hash collisions](https://docs.soliditylang.org/en/v0.8.13/abi-spec.html#non-standard-packed-mode) (e.g. `abi.encodePacked(0x123,0x456)` => `0x123456` => `abi.encodePacked(0x1,0x23456)`, but `abi.encode(0x123,0x456)` => `0x0...1230...456`). \"Unless there is a compelling reason, `abi.encode` should be preferred\". If there is only one argument to `abi.encodePacked()` it can often be cast to `bytes()` or `bytes32()` [instead](https://ethereum.stackexchange.com/questions/30912/how-to-compare-strings-in-solidity#answer-82739<br />If all arguments are strings and or bytes, `bytes.concat()` should be used instead".to_owned(),
		  	rule: "keccak(256)?.?\\(abi.encodePacked".to_owned(),
			recommendation: "".to_owned(),
			..Default::default()
		},
	];
	rules
}

//non critical
pub fn non_critical_issues() -> Vec<RulesDataBase> {
    let rules = vec![
		RulesDataBase{
			id: "N-01".to_owned(),
			title: "Use of `ecrecover()` is susceptible to signature malleability".to_owned(),
			description: "".to_owned(), // Impact should be empty.
			rule: "ecrecover".to_owned(),
			recommendation: "Use OpenZeppelin's `ECDSA` contract rather than calling `ecrecover()` directly.".to_owned(),
			..Default::default()
		},
		RulesDataBase{
			id: "N-02".to_owned(),
			title: "Declare `uint` as `uint256`".to_owned(),
			description: "".to_owned(),
			rule: " uint | int ".to_owned(),
			recommendation: "To favor explicitness, all instances of `uint`/`int` should be declared as `uint256`/`int256`.".to_owned(),
			..Default::default()
		},
		RulesDataBase{
			id: "N-03".to_owned(),
			title: "Large multiples of ten should use scientific notation (e.g. `1e6`) rather than decimal literals (e.g. `1000000`), for readability".to_owned(),
			description: "".to_owned(),
			rule: ".*10{6,}".to_owned(),
			recommendation: "".to_owned(),
			..Default::default()
		},
		// RulesDataBase{
		// 	id: "N-04".to_owned(),
		// 	title: "`constant`s should be defined rather than using magic numbers".to_owned(),
		// 	description: "".to_owned(),
		// 	rule: "((?![^\\n]*(uint|int|public))[^\\n]*)([[:blank:]]|\\()((?!(10|1e|32|256|128))[0-9e]{2,})".to_owned(),
		// 	recommendation: "To favor explicitness, all instances of `uint`/`int` should be declared as `uint256`/`int256`.".to_owned(),
		// 	..Default::default()
		// },
		RulesDataBase{
			id: "N-05".to_owned(),
			title: "The `nonReentrant` `modifier` should occur before all other modifiers".to_owned(),
  			description: "This is a best-practice to protect against reentrancy in other modifiers".to_owned(),
			rule: "function.?\\([a-zA-Z]*\\)[^\\}]*[[:space:]]((?!(external[[:space:]]|override[[:space:]]|view[[:space:]]|pure[[:space:]]|internal[[:space:]]|private[[:space:]]))[a-zA-Z]+[[:space:]])+[^\\}]*nonReentrant".to_owned(),
			recommendation: "".to_owned(),
			..Default::default()
		},
		RulesDataBase{
			id: "N-06".to_owned(),
			title: "Return values of `approve()` not checked".to_owned(),
			description: "Not all IERC20 implementations `revert()` when there's a failure in `approve()`. The function signature has a boolean return value and they indicate errors that way instead. By not checking the return value, operations that should have marked as failed, may potentially go through without actually approving anything".to_owned(),
			rule: "\\n((?![^=\\n]*function)[^=\\n]*)approve.?\\(".to_owned(),
			recommendation: "".to_owned(),
			..Default::default()
		},
		
	];
	rules
}
