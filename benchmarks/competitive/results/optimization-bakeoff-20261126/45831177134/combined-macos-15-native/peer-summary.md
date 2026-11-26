| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.96x | 4.25x | 1.51x | 1.44x | 1.96x | 1.84x | 29/30 |
| materialize | 0.99x | 2.02x | 1.41x | 1.13x | 2.22x | 1.30x | 28/30 |
| perft_bulk_d4 | 0.90x | 3.44x | 1.24x | 1.05x | 1.62x | 1.57x | 27/30 |
| perft_full_d3 | 0.98x | 1.60x | 1.80x | 1.37x | 2.04x | 2.04x | 30/30 |
| fen_parse | 0.94x | 1.19x | 1.35x | 1.26x | 1.66x | 1.54x | 29/30 |
| fen_write_native_ep | 0.97x | 4.36x | 8.39x | 1.83x | 25.69x | 2.08x | 30/30 |
| snapshot | 1.14x | 1.69x | 1.14x | 2.25x | 1.10x | 0.97x | 24/30 |
| san_parse_all | 1.06x | 1.78x | N/A | 3.99x | N/A | N/A | 12/12 |
| san_render_native | 1.14x | 13.04x | N/A | 2.93x | N/A | N/A | 12/12 |
| apply_children_full | 1.07x | 1.11x | 1.21x | 6.11x | 1.31x | 1.35x | 28/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.29x | N/A | N/A | 4/6 |
| startpos | 0.87x | 1.71x | 83.48x | 147.58x | 195.86x | 144.28x | 5/5 |
| chess960_materialize | 0.89x | 1.44x | 1.31x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.22x | 2.66x | 1.30x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.06x | 39.79x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.13x | 0.90x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 1.18x | 22.33x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.95x | 4.41x | 1.33x | 0.91x | 1.73x | 2.00x | 8/10 |
| corpus_materialize | 1.13x | 1.52x | 1.48x | 1.22x | 1.88x | 1.19x | 10/10 |
| corpus_uci_sorted | 1.02x | 1.82x | 2.72x | 1.92x | 2.54x | 1.44x | 10/10 |
| replay_trusted_native | 0.95x | 0.58x | 1.51x | 1.58x | 1.75x | 1.76x | 8/10 |
| replay_checked_native | 1.05x | 2.33x | 1.11x | 8.55x | 7.62x | 9.21x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
