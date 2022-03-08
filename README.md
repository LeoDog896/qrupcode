# qrupcode
Data-driven QR code generator

## Usage

General guide:

`qrupcode ascii -q hello`
`qrupcode unicode hello`
`qrupcode png hello -o output.png`
`qrupcode jpg hello`
`qrupcode custom --dark # --light `

### Flags

`-q` - Quiet zone (padding around the QR code for scanners)

`-o (file)` - Output to a specific file. You can also use piping.