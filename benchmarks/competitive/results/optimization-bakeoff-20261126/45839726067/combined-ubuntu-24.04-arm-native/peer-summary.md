| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.41x | 1.68x | 1.86x | 2.18x | 2.17x | 30/30 |
| materialize | 1.00x | 1.95x | 1.54x | 1.34x | 2.15x | 1.49x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.89x | 1.57x | 1.44x | 2.02x | 2.07x | 30/30 |
| perft_full_d3 | 0.99x | 0.92x | 0.99x | 0.82x | 1.22x | 1.15x | 17/30 |
| fen_parse | 1.22x | 1.48x | 1.59x | 1.51x | 2.10x | 1.70x | 30/30 |
| fen_write_native_ep | 1.00x | 4.59x | 9.34x | 1.59x | 30.21x | 2.52x | 30/30 |
| snapshot | 0.99x | 1.11x | 1.05x | 1.51x | 1.05x | 0.94x | 24/30 |
| san_parse_all | 1.00x | 1.84x | N/A | 4.29x | N/A | N/A | 12/12 |
| san_render_native | 1.07x | 12.53x | N/A | 2.71x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.25x | 1.48x | 4.02x | 1.20x | 1.20x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.55x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.30x | 77.40x | 129.58x | 176.16x | 143.43x | 5/5 |
| chess960_materialize | 1.00x | 1.71x | 1.60x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.97x | 1.50x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 25.66x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.04x | 1.14x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.00x | 19.43x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.42x | 1.71x | 1.38x | 1.99x | 2.20x | 10/10 |
| corpus_materialize | 1.00x | 1.60x | 1.52x | 1.22x | 1.90x | 1.39x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.10x | 2.93x | 2.09x | 2.86x | 1.46x | 10/10 |
| replay_trusted_native | 1.00x | 0.61x | 1.67x | 1.95x | 2.20x | 2.24x | 8/10 |
| replay_checked_native | 1.07x | 2.27x | 1.12x | 7.48x | 6.76x | 8.68x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
