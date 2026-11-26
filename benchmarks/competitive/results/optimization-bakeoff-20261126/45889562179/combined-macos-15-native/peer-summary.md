| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.94x | 4.91x | 1.66x | 1.71x | 2.38x | 2.02x | 30/30 |
| materialize | 1.04x | 2.10x | 1.65x | 1.09x | 2.24x | 1.41x | 27/30 |
| perft_bulk_d4 | 0.95x | 3.98x | 1.51x | 1.24x | 1.90x | 1.82x | 30/30 |
| perft_full_d3 | 0.92x | 1.50x | 1.67x | 1.28x | 1.87x | 1.92x | 30/30 |
| fen_parse | 1.00x | 1.20x | 1.36x | 1.29x | 1.82x | 1.40x | 30/30 |
| fen_write_native_ep | 0.97x | 4.25x | 13.95x | 1.76x | 28.36x | 2.25x | 30/30 |
| snapshot | 0.95x | 1.64x | 1.09x | 2.37x | 1.09x | 0.98x | 24/30 |
| san_parse_all | 0.96x | 1.70x | N/A | 3.66x | N/A | N/A | 12/12 |
| san_render_native | 0.97x | 12.56x | N/A | 2.80x | N/A | N/A | 12/12 |
| apply_children_full | 0.91x | 1.11x | 1.26x | 5.47x | 1.12x | 1.26x | 29/30 |
| make_unmake_full | 0.98x | N/A | N/A | 1.31x | N/A | N/A | 6/6 |
| startpos | 0.95x | 1.20x | 69.86x | 136.75x | 183.01x | 138.92x | 5/5 |
| chess960_materialize | 1.01x | 1.75x | 1.56x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.97x | 3.23x | 1.51x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 40.15x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.97x | 0.96x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.07x | 20.58x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.87x | 5.02x | 1.68x | 1.06x | 2.04x | 2.14x | 9/10 |
| corpus_materialize | 0.95x | 1.44x | 1.38x | 1.13x | 1.86x | 1.22x | 10/10 |
| corpus_uci_sorted | 1.02x | 1.89x | 2.64x | 1.94x | 2.63x | 1.42x | 10/10 |
| replay_trusted_native | 0.94x | 0.65x | 1.73x | 1.62x | 1.77x | 1.78x | 8/10 |
| replay_checked_native | 1.00x | 2.85x | 1.25x | 9.19x | 8.47x | 10.35x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
