# Usage

## Run

Run the web app:

```sh
mise exec -- trunk serve --open
```

Open `http://127.0.0.1:8080/terra-rs/`.
Trunk rebuilds the app as source files change.

Run the native app:

```sh
cargo run --locked
```

## Preferences and constraints

The settings panel controls how countries are scored and filtered.
Results update as you make valid changes.
On narrow screens, settings and results share one page: results appear at the top, with settings at the bottom.
Scroll the lower settings area to edit preferences while watching the match counts and top three countries above.
Collapse setting groups to bring other controls into reach.
The two areas scroll independently.

- **Culture fit:** adjust power distance, individualism, masculinity, uncertainty avoidance, long-term orientation, and
  indulgence.
  Hover explanations describe each dimension.
- **Score importances:** choose the balance between Culture Fit and Quality of Life, then the balance of Happy Planet,
  Social Progress, and Human Freedom within Quality of Life.
- **Constraints:** set minimum scores, a minimum English-speaking ratio, temperature and sunshine ranges, and allowed
  continents.
- **Start from a country:** copy a reference country's cultural preferences or quality-of-life minimums.
- **Reset:** restore all default settings.

The overall balance slider runs from 100% Culture Fit on the left to 100% Quality of Life on the right.
It adjusts both shares in 10% steps; the midpoint gives each 50%.
The endpoint icons and labels also select their respective extremes.
For Quality of Life, drag the point inside the triangle: Happy Planet is at the top, Social Progress at the lower left,
and Human Freedom at the lower right.
Each corner gives that preference 100%; the center gives all three equal shares.
Grid lines mark 10% steps, and the highlighted lines show the selected shares, constant along lines parallel to the
opposite edge.
Dragging snaps to the nearest grid intersection, keeping the shares in 10% increments and totaling 100%.
Click a corner label to select that extreme, or focus the triangle and use arrow keys (Shift for larger steps).
Home restores equal shares.
The scoring model normalizes each group internally; existing saved importances retain their proportions.

Active minimum constraints require a value strictly above the minimum.
Active temperature and sunshine ranges require values strictly inside the bounds.
If nothing matches, relax the constraints or reset settings.

Settings are saved in browser storage.
Invalid saved settings fall back to defaults; if storage is blocked, the app remains usable for that visit.
The native development window does not persist settings.

## Results

The results panel combines two linked views:

| View            | What it shows                                                                                                                                                                                                                                                           |
| --------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Country ranking | A scrollable table of all matching countries, with ranks, flags, country names, overall scores, and stacked contribution bars. Red shows Culture Fit; blue-to-teal segments show Happy Planet, Social Progress, and Human Freedom contributions within Quality of Life. |
| World map       | Country scores on a 2D map, with a legend, hover details, and selection linked to the table and bars.                                                                                                                                                                   |

Select a country in any results view to highlight its ranking and buildup bar.
Hover over a bar to inspect its weighted contributions.
Every matching country is selectable in the ranking, including small countries that are difficult to click on the map.
Malta and Singapore use location markers at the map's resolution.

Ranks are assigned after filtering, consecutively from 1 with no gaps.
Equal scores are ordered by country ID.
Selection stays on the same country while it still matches; otherwise the best remaining match is selected, or selection
clears when no countries match.

On narrow screens, the full ranking table appears above the settings with smaller text, compact bars, and selectable
rows.
Long country names are shortened to fit; hover for the full name.
Expand **World map** to show the map.
Countries without eligible data appear neutral on the map; the legend also distinguishes infeasible countries.

## Interpreting scores

Culture Fit measures closeness to your six cultural preferences.
Quality of Life combines the three historical indices using your weights.
Overall score combines Culture Fit and Quality of Life.
The contribution bars show those weighted terms, not additional independent scores.

These are comparisons of historical national aggregate data.
They do not predict an individual's experience.
Source years and explanations are available within the results panel; [data provenance](../data/README.md) records the
dataset limitations.
The precise formulas and filtering rules are in [Scoring](DEVELOPMENT.md#scoring).
