searchState.loadedDescShard("frontmatter_gen", 0, "Frontmatter Gen (frontmatter-gen)\nContains the error value\nMaximum size allowed for frontmatter content (1MB)\nMaximum allowed nesting depth for structured data\nContains the success value\nConfiguration options for parsing operations.\nA specialized Result type for frontmatter operations.\nCommand Line Interface Module\nConfiguration Module\nSite Generation Engine\nError handling for the frontmatter-gen crate.\nExtracts and parses frontmatter from content with format …\nThis module provides functionality for extracting …\nReturns the argument unchanged.\nLoad options from environment variables or use defaults.\nCalls <code>U::from(self)</code>.\nMaximum allowed nesting depth\nMaximum allowed content size\nFront Matter Parser and Serialiser Module\nPrelude module for convenient imports.\nStatic Site Generator Module\nConverts frontmatter to a specific format.\nThis module defines the core types used throughout the …\nUtility Module\nWhether to validate content structure\nValidates input content against security constraints.\nCommand line arguments parser\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nProcess CLI commands\nBuilder for creating Config instances\nCore configuration structure.\nErrors specific to configuration operations\nConfiguration file error\nInvalid language code\nInvalid directory path with detailed context\nInvalid site name provided\nInvalid URL format\nServer configuration error\nTOML parsing error\nSets the base URL\nBase URL of the site.\nBuilds the Config instance\nCreates a new <code>Builder</code> instance for fluent configuration …\nSets the content directory\nPath to the directory containing content files.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nLoads configuration from a TOML file\nGets the unique identifier for this configuration\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nSets the language code\nLanguage of the site (e.g., “en” for English).\nSets the output directory\nPath to the directory where the generated output will be …\nSets the serve directory\nOptional directory to serve during development.\nGets whether the development server is enabled\nEnables or disables the development server\nFlag to enable or disable the development server.\nGets the server port if the server is enabled\nSets the server port\nPort for the development server.\nSets the site description\nDescription of the site.\nGets the site name\nSets the site name\nName of the site.\nSets the site title\nTitle of the site, displayed in the browser’s title bar.\nSets the template directory\nPath to the directory containing templates.\nValidates the configuration settings\nDetails about why the path was invalid\nThe path that was invalid\nRepresents a processed content file, including its …\nThe primary engine responsible for site generation.\nCopies static assets from the content directory to the …\nExtracts frontmatter metadata and content body from a file.\nReturns the argument unchanged.\nReturns the argument unchanged.\nOrchestrates the complete site generation process.\nGenerates HTML pages from processed content files.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nLoads and caches all templates from the template directory.\nCreates a new <code>Engine</code> instance.\nProcesses a single content file and prepares it for …\nProcesses all content files in the content directory.\nRenders a template with the provided content.\nError occurred during asset processing.\nCategories of front matter errors.\nConfiguration-related errors.\nError occurred during content processing.\nContent exceeds the maximum allowed size.\nProvides additional context for front matter errors.\nConversion-related errors.\nError occurred during conversion between formats.\nErrors that can occur during site generation.\nRepresents errors that can occur during front matter …\nError during front matter extraction.\nError occurred during file system operations.\nThe front matter format is invalid or unsupported.\nInvalid JSON front matter.\nInvalid language code.\nInvalid TOML front matter.\nInvalid URL format.\nInvalid YAML front matter.\nJSON front matter exceeds maximum nesting depth.\nError occurred whilst parsing JSON content.\nError occurred during metadata processing.\nNesting depth exceeds the maximum allowed.\nNo front matter content was found.\nGeneric error with a custom message.\nGeneric error during parsing.\nParsing-related errors.\nSerialization or deserialization error.\nError occurred during template processing.\nError occurred whilst parsing TOML content.\nUnsupported or unknown front matter format was detected.\nValidation-related errors.\nInput validation error.\nError occurred whilst parsing YAML content.\nReturns the category of the error.\nColumn number where the error occurred.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCreates a generic parse error with a custom message.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nLine number where the error occurred.\nSnippet of the content where the error occurred.\nCreates an unsupported format error for a specific line.\nCreates a validation error with a custom message.\nAdds context to an error.\nAdditional context information about the error.\nThe original IO error that caused this error.\nThe actual nesting depth\nThe line number where the unsupported format was …\nThe maximum allowed size\nThe maximum allowed depth\nThe actual size of the content\nThe original error from the YAML parser\nThe original error from the serde library\nDetects the format of the extracted frontmatter.\nExtracts frontmatter enclosed by the given start and end …\nExtracts JSON frontmatter from the content by detecting …\nExtracts raw frontmatter from the content, detecting YAML, …\nOptions for controlling parsing behaviour.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nMaximum allowed nesting depth.\nMaximum allowed number of keys.\nConvenience wrapper around <code>parse_with_options</code> using …\nParses raw front matter string into a <code>Frontmatter</code> object …\nConverts a <code>Frontmatter</code> object to a string representation …\nWhether to validate the structure.\nValidates a front matter structure against configured …\nBuild the static site\nArguments for the build subcommand\nBuild process error with context\nConfiguration error with context\nFile system error with path context\nServe the static site locally with hot reloading\nArguments for the serve subcommand\nServer error with context\nCommand-line interface for the Static Site Generator\nErrors specific to the Static Site Generator functionality\nAvailable subcommands for the Static Site Generator\nExecutes the static site generation command\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nMessage associated with the error\nPath associated with the error\nRepresents an array of values.\nRepresents a boolean value.\nRepresents the different formats supported for frontmatter …\nRepresents the frontmatter, a collection of key-value …\nJSON format.\nRepresents a null value.\nRepresents a numeric value.\nRepresents an object (frontmatter).\nRepresents a string value.\nRepresents a tagged value, containing a tag and a value.\nTOML format.\nUnsupported format.\nA flexible value type that can hold various types of data …\nYAML format.\nReturns the length of the array if the value is an array, …\nReturns the value as an array, if it is of type <code>Array</code>.\nReturns the value as a boolean, if it is of type <code>Boolean</code>.\nReturns the value as a float, if it is of type <code>Number</code>.\nReturns the value as an object (frontmatter), if it is of …\nReturns the value as a string slice, if it is of type …\nReturns the value as a tagged value, if it is of type …\nReturns the current capacity of the underlying HashMap\nClears the frontmatter while preserving allocated capacity\nChecks if the frontmatter contains a given key.\nEscapes special characters in a string (e.g., backslashes …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCreates a <code>Frontmatter</code> from an iterator of key-value pairs.\nRetrieves a reference to a value associated with a key.\nRetrieves a mutable reference to a value associated with a …\nAttempts to get a mutable reference to the array if the …\nInserts a key-value pair into the frontmatter.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nAttempts to convert the value into a <code>bool</code>.\nAttempts to convert the value into an <code>f64</code>.\nAttempts to convert the value into a <code>String</code>.\nChecks if the value is of type <code>Array</code>.\nChecks if the value is of type <code>Boolean</code>.\nChecks if the frontmatter is empty.\nChecks if the value is of type <code>Null</code>.\nChecks if a given key exists and its value is <code>null</code>.\nChecks if the value is of type <code>Number</code>.\nChecks if the value is of type <code>Object</code>.\nChecks if the value is of type <code>String</code>.\nChecks if the value is of type <code>Tagged</code>.\nReturns an iterator over the key-value pairs of the …\nReturns a mutable iterator over the key-value pairs of the …\nReturns the number of entries in the frontmatter.\nMerges another frontmatter into this one. If a key exists, …\nCreates a new, empty frontmatter.\nRemoves a key-value pair from the frontmatter.\nReserves capacity for at least <code>additional</code> more elements\nAttempts to convert the value into a <code>Frontmatter</code>.\nConverts the value to a string representation regardless …\nFile system operation failed\nInvalid operation\nPath validation failed\nResource not found\nPermission error\nErrors that can occur during utility operations\nReturns the argument unchanged.\nFile system utilities module\nCalls <code>U::from(self)</code>.\nLogging utilities module\nDetails about why the path was invalid\nThe path that was invalid\nTracks temporary files for cleanup\nCleans up all tracked temporary files\nCopies a file from source to destination\nCreates a directory and all parent directories\nCreates a new temporary file with the given prefix\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCreates a new temporary file tracker\nRegisters a temporary file for tracking\nValidates that a path is safe to use\nLog entry structure\nLog writer for handling log output\nOptional error details\nFormats the log entry as a string\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nLog level\nLog message\nCreates a new log entry\nCreates a new log writer\nTimestamp of the log entry\nWrites a log entry")