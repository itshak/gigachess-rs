| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.43x | 1.69x | 1.82x | 2.19x | 2.18x | 30/30 |
| materialize | 1.01x | 1.97x | 1.58x | 1.37x | 2.20x | 1.50x | 30/30 |
| perft_bulk_d4 | 1.04x | 4.01x | 1.64x | 1.49x | 2.10x | 2.15x | 30/30 |
| perft_full_d3 | 1.14x | 1.67x | 1.81x | 1.52x | 2.23x | 2.10x | 30/30 |
| fen_parse | 0.98x | 1.20x | 1.31x | 1.24x | 1.71x | 1.37x | 30/30 |
| fen_write_native_ep | 1.00x | 4.51x | 9.42x | 1.62x | 30.12x | 2.39x | 30/30 |
| snapshot | 1.01x | 1.16x | 1.00x | 1.49x | 1.00x | 0.94x | 21/30 |
| san_parse_all | 1.00x | 1.84x | N/A | 4.28x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 12.29x | N/A | 2.67x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.25x | 1.48x | 4.02x | 1.21x | 1.21x | 30/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.57x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.28x | 76.39x | 127.35x | 171.38x | 137.22x | 5/5 |
| chess960_materialize | 1.01x | 1.71x | 1.60x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.02x | 3.00x | 1.52x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 25.54x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.09x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.99x | 19.41x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.09x | 4.85x | 1.86x | 1.49x | 2.20x | 2.43x | 10/10 |
| corpus_materialize | 1.00x | 1.62x | 1.54x | 1.24x | 1.92x | 1.40x | 10/10 |
| corpus_uci_sorted | 0.98x | 2.07x | 2.88x | 2.08x | 2.82x | 1.44x | 10/10 |
| replay_trusted_native | 0.99x | 0.62x | 1.63x | 1.97x | 2.21x | 2.25x | 8/10 |
| replay_checked_native | 1.02x | 2.36x | 1.16x | 7.78x | 6.89x | 8.92x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
