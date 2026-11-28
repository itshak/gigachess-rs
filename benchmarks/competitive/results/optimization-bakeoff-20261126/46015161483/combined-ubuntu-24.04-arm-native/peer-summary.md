| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.44x | 1.70x | 1.82x | 2.20x | 2.20x | 30/30 |
| materialize | 1.02x | 1.98x | 1.55x | 1.35x | 2.18x | 1.51x | 30/30 |
| perft_bulk_d4 | 1.04x | 4.07x | 1.63x | 1.49x | 2.10x | 2.16x | 30/30 |
| perft_full_d3 | 1.14x | 1.68x | 1.81x | 1.52x | 2.23x | 2.11x | 30/30 |
| fen_parse | 1.01x | 1.24x | 1.33x | 1.26x | 1.76x | 1.41x | 30/30 |
| fen_write_native_ep | 0.99x | 4.55x | 9.28x | 1.57x | 29.95x | 2.42x | 30/30 |
| snapshot | 1.00x | 1.13x | 1.02x | 1.49x | 1.00x | 0.94x | 21/30 |
| san_parse_all | 1.00x | 1.83x | N/A | 4.27x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 12.41x | N/A | 2.68x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.25x | 1.47x | 4.01x | 1.21x | 1.20x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.57x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.28x | 76.71x | 129.43x | 175.19x | 142.01x | 5/5 |
| chess960_materialize | 1.01x | 1.72x | 1.57x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.02x | 3.02x | 1.52x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.02x | 25.89x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.29x | 1.40x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.01x | 19.43x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.10x | 4.86x | 1.89x | 1.51x | 2.22x | 2.41x | 10/10 |
| corpus_materialize | 1.01x | 1.62x | 1.55x | 1.23x | 1.94x | 1.40x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.10x | 2.92x | 2.07x | 2.86x | 1.45x | 10/10 |
| replay_trusted_native | 1.00x | 0.62x | 1.66x | 1.99x | 2.25x | 2.28x | 8/10 |
| replay_checked_native | 1.02x | 2.34x | 1.15x | 7.67x | 6.94x | 8.92x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
