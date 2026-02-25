# `getmemoryinfo`

Returns statistics about Floresta's memory usage.

## Usage

### Synopsis

```bash
floresta-cli getmemoryinfo
```

### Examples

```bash
floresta-cli getmemoryinfo 
```

## Returns

### Ok Response

- `locked` - (object) Memory usage statistics for locked memory
  - `used` - (numeric) The amount of locked memory currently used (bytes)
  - `free` - (numeric) The amount of locked memory currently free (bytes)
  - `total` - (numeric) The total amount of locked memory (bytes)
  - `locked` - (numeric) The amount of locked memory that is currently locked by the system (bytes)
  - `chunks_used` - (numeric) The number of allocated chunks
  - `chunks_free` - (numeric) The number of free chunks

## Notes

- Returns zeroed values for all runtimes that are not *-gnu or MacOS.
