| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.03x | 4.82x | 1.73x | 1.64x | 2.41x | 2.12x | 30/30 |
| materialize | 1.01x | 2.06x | 1.40x | 1.33x | 2.37x | 1.58x | 30/30 |
| perft_bulk_d4 | 1.00x | 4.02x | 1.61x | 1.36x | 2.30x | 2.04x | 30/30 |
| perft_full_d3 | 0.99x | 1.39x | 1.49x | 1.33x | 1.94x | 1.87x | 30/30 |
| fen_parse | 1.00x | 1.28x | 1.36x | 1.40x | 2.25x | 1.49x | 30/30 |
| fen_write_native_ep | 0.99x | 3.97x | 7.93x | 1.72x | 31.39x | 1.98x | 30/30 |
| snapshot | 1.00x | 1.81x | 1.02x | 3.99x | 1.00x | 1.01x | 25/30 |
| san_parse_all | 1.01x | 2.15x | N/A | 5.53x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.22x | N/A | 3.80x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.33x | 1.40x | 9.51x | 1.48x | 1.68x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.37x | N/A | N/A | 6/6 |
| startpos | 0.99x | 1.24x | 78.16x | 162.28x | 237.57x | 177.20x | 5/5 |
| chess960_materialize | 0.99x | 1.57x | 1.28x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.90x | 1.54x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.87x | 25.73x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.96x | 20.78x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 5.41x | 1.83x | 1.35x | 2.44x | 2.38x | 10/10 |
| corpus_materialize | 1.00x | 1.59x | 1.26x | 1.20x | 1.98x | 1.37x | 10/10 |
| corpus_uci_sorted | 0.99x | 1.76x | 2.25x | 1.81x | 2.15x | 1.40x | 10/10 |
| replay_trusted_native | 1.00x | 0.85x | 1.60x | 2.09x | 2.61x | 2.96x | 9/10 |
| replay_checked_native | 0.98x | 2.46x | 1.07x | 8.75x | 8.09x | 9.62x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
