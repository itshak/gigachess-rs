| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.93x | 4.14x | 1.73x | 1.49x | 2.04x | 1.79x | 30/30 |
| materialize | 0.96x | 2.01x | 1.38x | 1.56x | 2.31x | 1.60x | 30/30 |
| perft_bulk_d4 | 0.83x | 3.19x | 1.27x | 0.97x | 1.75x | 1.53x | 27/30 |
| perft_full_d3 | 0.92x | 1.32x | 1.43x | 1.38x | 1.84x | 1.54x | 30/30 |
| fen_parse | 1.03x | 1.28x | 1.61x | 1.46x | 1.92x | 1.57x | 30/30 |
| fen_write_native_ep | 0.94x | 3.99x | 7.57x | 1.63x | 29.22x | 2.12x | 30/30 |
| snapshot | 0.86x | 1.81x | 1.25x | 4.15x | 1.24x | 1.21x | 30/30 |
| san_parse_all | 0.96x | 1.69x | N/A | 5.18x | N/A | N/A | 12/12 |
| san_render_native | 0.89x | 9.02x | N/A | 3.83x | N/A | N/A | 12/12 |
| apply_children_full | 0.86x | 1.84x | 1.22x | 9.16x | 1.17x | 1.49x | 30/30 |
| make_unmake_full | 0.74x | N/A | N/A | 0.87x | N/A | N/A | 0/6 |
| startpos | 0.83x | 0.99x | 65.33x | 141.69x | 179.04x | 150.93x | 4/5 |
| chess960_materialize | 0.96x | 1.54x | 1.34x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.85x | 2.17x | 1.28x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 40.85x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.98x | 0.98x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.13x | 31.30x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.93x | 4.54x | 1.85x | 0.98x | 2.19x | 1.94x | 9/10 |
| corpus_materialize | 0.98x | 1.40x | 1.25x | 1.10x | 1.83x | 1.21x | 10/10 |
| corpus_uci_sorted | 0.99x | 1.83x | 2.25x | 1.87x | 2.23x | 1.59x | 10/10 |
| replay_trusted_native | 0.95x | 0.80x | 1.49x | 1.57x | 2.11x | 2.39x | 8/10 |
| replay_checked_native | 0.87x | 2.06x | 1.03x | 8.25x | 7.93x | 9.47x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
