| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.12x | 4.39x | 1.67x | 1.80x | 2.17x | 2.16x | 30/30 |
| materialize | 1.06x | 1.94x | 1.53x | 1.33x | 2.13x | 1.47x | 30/30 |
| perft_bulk_d4 | 1.13x | 3.88x | 1.57x | 1.43x | 2.01x | 2.06x | 30/30 |
| perft_full_d3 | 1.03x | 1.22x | 1.31x | 1.10x | 1.63x | 1.53x | 29/30 |
| fen_parse | 1.00x | 1.25x | 1.34x | 1.27x | 1.75x | 1.41x | 30/30 |
| fen_write_native_ep | 1.00x | 4.57x | 9.27x | 1.57x | 30.26x | 2.39x | 30/30 |
| snapshot | 1.00x | 1.15x | 1.02x | 1.47x | 1.01x | 0.94x | 21/30 |
| san_parse_all | 1.00x | 1.83x | N/A | 4.24x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.77x | N/A | 2.51x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.38x | 1.63x | 4.48x | 1.35x | 1.34x | 30/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.53x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.28x | 76.43x | 128.66x | 170.98x | 137.62x | 5/5 |
| chess960_materialize | 1.07x | 1.68x | 1.57x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.11x | 2.98x | 1.50x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 25.04x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.06x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.99x | 18.10x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.13x | 4.45x | 1.70x | 1.39x | 2.00x | 2.21x | 10/10 |
| corpus_materialize | 1.05x | 1.61x | 1.52x | 1.23x | 1.90x | 1.37x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.12x | 2.93x | 2.09x | 2.85x | 1.46x | 10/10 |
| replay_trusted_native | 1.01x | 0.61x | 1.62x | 1.98x | 2.22x | 2.24x | 8/10 |
| replay_checked_native | 0.99x | 2.16x | 1.05x | 7.14x | 6.31x | 8.18x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
