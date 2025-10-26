# Jimbo 🃏

Your personal Balatro strategist. Jimbo analyzes your deck, jokers, and consumables to find the highest-scoring plays for any blind.

## What is Jimbo?

Jimbo is a command-line solver and simulator for [Balatro](https://www.playbalatro.com/) that helps you find optimal plays in any situation. Whether you're theorycrafting a wild combo, trying to beat a brutal blind, or just curious about the math behind your favorite build, Jimbo has your back.

## Features

- 🎯 **Optimal Play Calculation** - Finds the best scoring combination from your current hand
- 🎲 **Seed Support** - Reproducible results with optional seed input
- 🃏 **Full Game State** - Configure custom decks, jokers, consumables, vouchers, and blind conditions
- 📊 **Multiple Output Modes** - Clean terminal output or detailed JSON for scripting
- ⚡ **Fast Analysis** - Written in Rust for blazing-fast calculations
- 🎨 **TUI Mode** - Interactive terminal interface for exploring different scenarios

## Installation

### From Source (Rust required)

```bash
git clone https://github.com/yourusername/jimbo.git
cd jimbo
cargo build --release
cargo install --path .
```

### Pre-built Binaries

Download the latest release from the [Releases](https://github.com/yourusername/jimbo/releases) page.

## Usage

### Basic Command Structure

```bash
jimbo <COMMAND> [OPTIONS]
```

### Commands

#### `solve`

Analyzes your hand and finds the optimal play to maximize your score.

```bash
jimbo solve [OPTIONS]
```

**Options:**

- `--hand <CARDS>` - Your current hand (e.g., "AH KH QH JH 10H")
- `--deck <FILE>` - Path to deck configuration file (JSON)
- `--jokers <JOKERS>` - Comma-separated list of jokers (e.g., "joker,greedy_joker,lusty_joker")
- `--consumables <ITEMS>` - Comma-separated list of tarots/planets in inventory
- `--vouchers <VOUCHERS>` - Comma-separated list of active vouchers
- `--blind <TYPE>` - Blind type (e.g., "small", "big", "boss:the_hook")
- `--blind-score <AMOUNT>` - Required score to beat the blind
- `--blind-ability <ABILITY>` - Special blind ability (for boss blinds)
- `--seed <SEED>` - Optional seed for reproducible results
- `--output <FORMAT>` - Output format: `pretty` (default), `json`, `compact`
- `--show-alternatives` - Show top N alternative plays (default: 3)

**Examples:**

```bash
# Basic solve with hand and jokers
jimbo solve --hand "AH KH QH JH 10H" --jokers "joker,Blueprint,Brainstorm"

# Full configuration with blind
jimbo solve --hand "9C 7D 3H 2S KD" \
  --deck my_deck.json \
  --jokers "Ride_the_Bus,Hanging_Chad" \
  --blind boss:the_serpent \
  --blind-score 15000

# With seed for reproducibility
jimbo solve --hand "5H 5D 5C 8S 2H" --jokers "Baron" --seed 12345 --output json
```

#### `simulate`

Runs multiple simulations to find average/best-case scores over many hands.

```bash
jimbo simulate [OPTIONS]
```

**Options:**

- `--runs <N>` - Number of simulation runs (default: 1000)
- `--deck <FILE>` - Path to deck configuration file
- `--jokers <JOKERS>` - Comma-separated list of jokers
- `--vouchers <VOUCHERS>` - Comma-separated list of vouchers
- `--blind <TYPE>` - Blind type to simulate against
- `--seed <SEED>` - Base seed for simulations
- `--output <FORMAT>` - Output format: `summary` (default), `detailed`, `csv`

**Examples:**

```bash
# Simulate 1000 hands with a specific build
jimbo simulate --runs 1000 --jokers "Duo,Trio,Family" --deck standard.json

# Test against a specific boss blind
jimbo simulate --runs 500 --jokers "Sock_and_Buskin" --blind boss:the_psychic
```

#### `tui`

Launches the interactive terminal user interface.

```bash
jimbo tui
```

Navigate through menus to:
- Build your deck interactively
- Select jokers and consumables
- Configure blind conditions
- Visualize optimal plays in real-time

**Keybindings:**
- `Arrow keys` - Navigate
- `Enter` - Select/Confirm
- `Tab` - Switch panels
- `s` - Solve current configuration
- `r` - Randomize hand
- `q` - Quit

#### `config`

Manage configuration files for decks, joker sets, and presets.

```bash
jimbo config <SUBCOMMAND>
```

**Subcommands:**

- `init` - Create a new configuration file
- `validate` - Validate an existing configuration
- `list` - List all saved configurations
- `export` - Export current game state to config file
- `import` - Import configuration from file

**Examples:**

```bash
# Create a new deck configuration
jimbo config init --type deck --output my_deck.json

# Validate a configuration file
jimbo config validate --file my_deck.json

# List all saved configurations
jimbo config list
```

### Global Flags

These flags work with any command:

- `-v, --verbose` - Increase logging verbosity
- `-q, --quiet` - Suppress non-essential output
- `-h, --help` - Display help information
- `--version` - Display version information

## Configuration Files

### Deck Configuration (JSON)

```json
{
  "cards": [
    { "rank": "A", "suit": "Hearts" },
    { "rank": "K", "suit": "Hearts" }
  ],
  "enhancements": {
    "AH": "gold",
    "KH": "steel"
  },
  "editions": {
    "AH": "polychrome"
  }
}
```

### Game State Configuration (JSON)

```json
{
  "deck": "path/to/deck.json",
  "jokers": ["Joker", "Greedy_Joker", "Lusty_Joker"],
  "consumables": ["The_Fool", "The_Magician"],
  "vouchers": ["Overstock", "Tarot_Merchant"],
  "blind": {
    "type": "boss",
    "name": "The_Hook",
    "score_required": 25000,
    "ability": "debuff_2_cards_per_hand"
  },
  "seed": 12345
}
```

## Examples

### Find the best play for a Royal Flush build

```bash
jimbo solve \
  --hand "AH KH QH JH 10H" \
  --jokers "Baron,Photograph,Blueprint" \
  --vouchers "Hone" \
  --blind-score 50000
```

### Test a planet card strategy

```bash
jimbo simulate \
  --runs 2000 \
  --jokers "Space_Joker,Constellation,Astronomer" \
  --consumables "Pluto,Neptune,Jupiter"
```

### Interactive exploration

```bash
jimbo tui
```

## Roadmap

- [ ] Support for all base game jokers
- [ ] Support for all blind types
- [ ] Spectral card analysis
- [ ] Multi-round ante planning
- [ ] Stake difficulty modifiers
- [ ] Deck archetype recommendations
- [ ] Web interface (WASM build)

## Contributing

Contributions are welcome! Please feel free to submit issues, feature requests, or pull requests.

## License

[Your chosen license]

## Acknowledgments

- [Balatro](https://www.playbalatro.com/) by LocalThunk
- The Balatro community for theorycrafting and math breakdowns

---

*Jimbo is a fan-made tool and is not affiliated with or endorsed by LocalThunk or Balatro.*