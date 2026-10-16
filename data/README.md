# Data provenance

Country inputs are pinned in `data/raw/`.
Each file's hash, source URL, observation year where known, column mapping, units, and transformation are recorded in
`catalog.toml`.
[LICENSE](../LICENSE) contains code copyright and license notices; they do not establish a blanket license for upstream
datasets.
Unknown upstream terms remain marked unknown.

The world geometry is Natural Earth **5.1.2**, `geojson/ne_110m_admin_0_countries.geojson`, from the
[tagged dataset](https://github.com/nvkelso/natural-earth-vector/tree/v5.1.2).
Natural Earth publishes its map data as [public domain](https://www.naturalearthdata.com/about/terms-of-use/).
The catalog records the downloaded file's checksum.
Geometry retains the source's geographic conventions and boundaries; the app does not independently adjudicate them.

Sources are joined by country name.
Unmatched incomplete rows appear in `coverage.json`; a complete scoring row without an ISO ID is an error.
Map ISO exceptions use `ISO_A3_EH`, then `ISO_A3`, then `ADM0_A3`; Kosovo's `KOS` maps to `XKX`.

Known source limitations:

- Happy Planet is from 2019, Social Progress from 2022, and Human Freedom averages 2015–2020 from the 2022 release.
  Culture/language/climate sources have mixed or incompletely recorded observation dates.
- Country temperature mostly represents 1961–1990.
  Denmark and Norway use mainland-oriented World Bank Climate Knowledge Portal values because remote land holdings skew
  country-wide averages.
- Sunshine represents the median of available cities, not an area-weighted national measurement.
  The checked-in country-level summary is the frozen input; city-level wrangling is not part of the app.
- Full scoring eligibility requires climate data even without active climate constraints.
  This excludes some otherwise comparable countries.
- Missing values stay missing.
  Countries without complete scoring data remain neutral on the map.
  At this resolution Malta and Singapore are represented by their coordinates; all eligible countries have a location
  marker.
- These are national aggregate indices, not predictions of any individual's life or descriptions of every person in a
  country.

The map uses a fixed equirectangular projection.
Polygons, holes, and multipolygons are triangulated offline with `earcutr`; the Natural Earth file already splits island
polygons at the antimeridian.
No online tile service or runtime GIS server is used.
