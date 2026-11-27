| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.39x | 1.68x | 1.80x | 2.18x | 2.16x | 30/30 |
| materialize | 1.00x | 1.94x | 1.53x | 1.33x | 2.15x | 1.48x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.88x | 1.59x | 1.43x | 2.01x | 2.06x | 30/30 |
| perft_full_d3 | 1.00x | 1.48x | 1.59x | 1.33x | 1.96x | 1.85x | 30/30 |
| fen_parse | 1.00x | 1.24x | 1.33x | 1.28x | 1.74x | 1.41x | 30/30 |
| fen_write_native_ep | 1.00x | 4.58x | 9.27x | 1.59x | 30.31x | 2.47x | 30/30 |
| snapshot | 1.01x | 1.15x | 1.01x | 1.49x | 1.01x | 0.94x | 19/30 |
| san_parse_all | 0.99x | 1.83x | N/A | 4.22x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 12.44x | N/A | 2.71x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.25x | 1.47x | 4.04x | 1.22x | 1.21x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.55x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.28x | 76.53x | 133.04x | 172.55x | 142.02x | 5/5 |
| chess960_materialize | 1.01x | 1.71x | 1.57x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.98x | 1.50x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 25.29x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.09x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.00x | 18.93x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 4.39x | 1.71x | 1.39x | 1.99x | 2.19x | 10/10 |
| corpus_materialize | 1.00x | 1.59x | 1.53x | 1.22x | 1.90x | 1.37x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.10x | 2.93x | 2.09x | 2.85x | 1.45x | 10/10 |
| replay_trusted_native | 0.98x | 0.63x | 1.65x | 1.99x | 2.19x | 2.25x | 8/10 |
| replay_checked_native | 1.01x | 2.34x | 1.14x | 7.67x | 6.88x | 8.83x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
