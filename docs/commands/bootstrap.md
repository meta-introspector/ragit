### `rag bootstrap`

The `rag bootstrap` command provides a quick and easy way to initialize a new RAGIT knowledge base and see it in action. It's a great way to get a feel for the end-to-end workflow of RAGIT with a single command.

#### Usage

```bash
rag bootstrap
```

#### Workflow

The `bootstrap` command executes the following sequence of commands internally:

1.  `rag init`: Initializes a new RAGIT repository in the current directory.
2.  `rag add --all`: Adds all files in the current directory to the staging area.
3.  `rag build`: Builds the knowledge base index from the staged files.
4.  `rag query "What makes ragit special?"`: Runs a sample query against the newly built index to demonstrate the system's query capabilities.

This command is intended for demonstration and initial setup purposes. For more granular control over the repository, you should use the individual commands (`init`, `add`, `build`, `query`) directly.