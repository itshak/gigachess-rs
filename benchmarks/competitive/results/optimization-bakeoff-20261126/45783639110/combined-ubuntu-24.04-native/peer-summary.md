| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.00x | 1.73x | 1.46x | 2.35x | 1.75x | 30/30 |
| materialize | 1.04x | 2.14x | 1.58x | 1.60x | 2.50x | 1.95x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.88x | 1.59x | 1.21x | 2.57x | 1.90x | 30/30 |
| perft_full_d3 | 1.00x | 1.51x | 1.77x | 1.68x | 2.26x | 1.83x | 30/30 |
| fen_parse | 1.02x | 1.37x | 1.47x | 1.11x | 1.34x | 1.15x | 29/30 |
| fen_write_native_ep | 0.99x | 3.10x | 6.52x | 1.32x | 17.96x | 1.62x | 30/30 |
| snapshot | 1.41x | 1.82x | 1.15x | 4.32x | 1.15x | 1.33x | 30/30 |
| san_parse_all | 1.00x | 1.91x | N/A | 4.42x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 9.11x | N/A | 2.12x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 1.81x | 1.12x | 3.27x | 1.14x | 1.35x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.15x | N/A | N/A | 6/6 |
| startpos | 1.09x | 1.12x | 88.97x | 161.22x | 189.73x | 173.25x | 5/5 |
| chess960_materialize | 1.00x | 1.72x | 1.60x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.28x | 1.39x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 39.62x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.04x | 1.01x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.09x | 21.18x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 6.43x | 2.43x | 1.34x | 2.95x | 2.69x | 10/10 |
| corpus_materialize | 1.01x | 1.43x | 1.32x | 1.09x | 1.75x | 1.20x | 10/10 |
| corpus_uci_sorted | 0.98x | 2.26x | 2.94x | 2.19x | 2.86x | 1.56x | 10/10 |
| replay_trusted_native | 1.00x | 0.71x | 1.30x | 1.29x | 1.77x | 2.10x | 8/10 |
| replay_checked_native | 1.00x | 2.08x | 1.13x | 8.52x | 8.17x | 9.77x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
