# `getrpcinfo`

Returns information about the RPC server.

## Usage

### Synopsis

```bash
floresta-cli getrpcinfo
```

### Examples

```bash
floresta-cli getrpcinfo 
```

## Returns

### Ok Response

- `active_commands` - (array) All active commands
  - (object) Information about an active command
    - `method` - (string) The name of the RPC command
    - `duration` - (numeric) The running time in microseconds
- `logpath` - (string) The complete file path to the debug log
