| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.38x | 1.67x | 1.80x | 2.18x | 2.17x | 30/30 |
| materialize | 1.00x | 1.94x | 1.54x | 1.35x | 2.15x | 1.49x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.85x | 1.57x | 1.42x | 2.02x | 2.06x | 30/30 |
| perft_full_d3 | 1.07x | 1.57x | 1.70x | 1.43x | 2.09x | 1.99x | 30/30 |
| fen_parse | 1.00x | 1.22x | 1.31x | 1.24x | 1.73x | 1.39x | 30/30 |
| fen_write_native_ep | 1.00x | 4.41x | 9.19x | 1.57x | 29.95x | 2.45x | 30/30 |
| snapshot | 1.00x | 1.16x | 1.01x | 1.49x | 1.00x | 0.94x | 21/30 |
| san_parse_all | 1.00x | 1.84x | N/A | 4.21x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.71x | N/A | 2.53x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.25x | 1.47x | 4.03x | 1.21x | 1.20x | 30/30 |
| make_unmake_full | 1.03x | N/A | N/A | 1.55x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.28x | 76.27x | 127.23x | 176.84x | 143.55x | 5/5 |
| chess960_materialize | 1.00x | 1.71x | 1.56x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.98x | 1.50x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.98x | 25.62x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.09x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.98x | 19.38x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.98x | 4.37x | 1.67x | 1.34x | 1.96x | 2.16x | 10/10 |
| corpus_materialize | 0.99x | 1.58x | 1.51x | 1.22x | 1.88x | 1.37x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.11x | 3.12x | 2.08x | 2.87x | 1.45x | 10/10 |
| replay_trusted_native | 1.00x | 0.62x | 1.63x | 1.97x | 2.20x | 2.27x | 8/10 |
| replay_checked_native | 0.99x | 2.15x | 1.04x | 7.10x | 6.27x | 8.02x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
