| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.05x | 4.48x | 1.94x | 1.63x | 2.83x | 1.95x | 30/30 |
| materialize | 1.00x | 2.25x | 1.68x | 1.71x | 2.71x | 1.97x | 30/30 |
| perft_bulk_d4 | 1.06x | 3.97x | 1.61x | 1.23x | 2.58x | 1.94x | 29/30 |
| perft_full_d3 | 1.23x | 1.66x | 1.99x | 1.85x | 2.41x | 2.07x | 30/30 |
| fen_parse | 1.00x | 1.38x | 1.39x | 1.07x | 1.54x | 1.30x | 30/30 |
| fen_write_native_ep | 1.00x | 4.38x | 9.97x | 1.86x | 28.87x | 2.66x | 30/30 |
| snapshot | 1.00x | 1.82x | 1.15x | 4.32x | 1.16x | 1.33x | 30/30 |
| san_parse_all | 1.00x | 2.09x | N/A | 4.22x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 9.18x | N/A | 2.14x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 2.15x | 1.36x | 4.14x | 1.35x | 1.70x | 30/30 |
| make_unmake_full | 0.96x | N/A | N/A | 1.15x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.12x | 88.70x | 144.45x | 211.34x | 180.04x | 5/5 |
| chess960_materialize | 0.98x | 1.67x | 1.62x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.53x | 1.51x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 40.11x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.23x | 23.63x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.20x | 6.31x | 2.37x | 1.30x | 2.87x | 2.61x | 10/10 |
| corpus_materialize | 1.00x | 1.39x | 1.28x | 1.05x | 1.68x | 1.17x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.24x | 2.94x | 2.22x | 2.92x | 1.57x | 10/10 |
| replay_trusted_native | 1.06x | 0.93x | 1.69x | 1.61x | 2.28x | 2.70x | 9/10 |
| replay_checked_native | 1.01x | 2.19x | 1.15x | 9.08x | 8.89x | 10.55x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
