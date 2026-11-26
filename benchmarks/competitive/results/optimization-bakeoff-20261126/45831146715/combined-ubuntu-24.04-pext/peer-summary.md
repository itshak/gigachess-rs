| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.07x | 1.69x | 1.47x | 2.28x | 1.73x | 30/30 |
| materialize | 1.00x | 2.15x | 1.58x | 1.68x | 2.54x | 1.84x | 30/30 |
| perft_bulk_d4 | 0.97x | 3.33x | 1.33x | 1.04x | 2.04x | 1.61x | 28/30 |
| perft_full_d3 | 0.94x | 1.34x | 1.54x | 1.51x | 1.98x | 1.64x | 30/30 |
| fen_parse | 1.06x | 1.25x | 1.38x | 1.15x | 1.56x | 1.11x | 29/30 |
| fen_write_native_ep | 0.99x | 4.37x | 9.93x | 1.85x | 29.40x | 2.16x | 30/30 |
| snapshot | 1.00x | 1.83x | 1.16x | 4.33x | 1.16x | 1.34x | 30/30 |
| san_parse_all | 1.04x | 1.99x | N/A | 4.45x | N/A | N/A | 12/12 |
| san_render_native | 0.98x | 9.02x | N/A | 2.11x | N/A | N/A | 12/12 |
| apply_children_full | 0.91x | 1.91x | 1.20x | 3.59x | 1.20x | 1.50x | 30/30 |
| make_unmake_full | 0.88x | N/A | N/A | 1.02x | N/A | N/A | 4/6 |
| startpos | 0.89x | 1.12x | 87.88x | 162.91x | 204.25x | 156.59x | 5/5 |
| chess960_materialize | 0.94x | 1.57x | 1.52x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.95x | 2.24x | 1.33x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 39.12x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.98x | 1.00x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.01x | 19.12x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.03x | 4.63x | 1.73x | 0.96x | 2.12x | 1.94x | 8/10 |
| corpus_materialize | 0.99x | 1.37x | 1.25x | 1.07x | 1.68x | 1.14x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.25x | 2.99x | 2.24x | 2.92x | 1.56x | 10/10 |
| replay_trusted_native | 0.86x | 0.77x | 1.35x | 1.40x | 1.96x | 2.22x | 8/10 |
| replay_checked_native | 1.00x | 2.03x | 1.03x | 8.57x | 8.19x | 9.62x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
