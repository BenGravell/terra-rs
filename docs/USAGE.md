# Usage

## Preferences and constraints

The settings panel controls how countries are scored and filtered. Results update as you make valid changes. On narrow screens, use the **Settings** and **Results** tabs.

- **Culture fit:** adjust power distance, individualism, masculinity, uncertainty avoidance, long-term orientation, and indulgence. Hover explanations describe each dimension.
- **Score weights:** choose the balance between Culture Fit and Quality of Life, then the balance of Happy Planet, Social Progress, and Human Freedom within Quality of Life.
- **Constraints:** set minimum scores, a minimum English-speaking ratio, temperature and sunshine ranges, and allowed continents.
- **Start from a country:** copy a reference country's cultural preferences or quality-of-life minimums.
- **Reset:** restore all default settings.

Changing a weight redistributes its change equally among the other weights in its group, keeping the total at 100%. For example, increasing Happy Planet from 30% to 35% changes a 30/40/30 split to 35/37.5/27.5. When another weight reaches zero, the remaining reduction comes from the other available weights.

Active minimum constraints require a value strictly above the minimum. Active temperature and sunshine ranges require values strictly inside the bounds. If nothing matches, relax the constraints or reset settings.

Settings are saved in browser storage. Invalid saved settings fall back to defaults; if storage is blocked, the app remains usable for that visit. The native development window does not persist settings.

## Results

The results panel combines three linked views:

| View                | What it shows                                                                                                                                 |
| ------------------- | --------------------------------------------------------------------------------------------------------------------------------------------- |
| Country ranking     | All matching countries, ordered by overall score, with their rank and component scores.                                                       |
| Score contributions | Stacked bars showing how Culture Fit and Quality of Life contribute to overall scores, plus the selected country's Quality of Life breakdown. |
| World map           | Country scores on a 2D map, with a legend, hover details, and selection linked to the table and bars.                                         |

Select a country in any results view to inspect its breakdown. Every matching country is selectable in the ranking, including small countries that are difficult to click on the map. Malta and Singapore use location markers at the map's resolution.

Ranks are assigned among eligible countries before filtering, so filtered rankings can contain gaps. Tied scores share a rank. Selection stays on the same country while it still matches; otherwise the best remaining match is selected, or selection clears when no countries match.

Narrow layouts omit secondary score columns from the table. The selected-country summary and contribution bars retain those scores. Countries without eligible data appear neutral on the map; the legend also distinguishes countries outside the constraints.

## Interpreting scores

Culture Fit measures closeness to your six cultural preferences. Quality of Life combines the three historical indices using your weights. Overall score combines Culture Fit and Quality of Life. The contribution bars show those weighted terms, not additional independent scores.

These are comparisons of historical national aggregate data. They do not predict an individual's experience. Source years and explanations are available within the results panel; [data provenance](../data/README.md) records the dataset limitations. The precise formulas and filtering rules are in [Scoring](DEVELOPMENT.md#scoring).
