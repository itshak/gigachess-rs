| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.97x | 4.18x | 1.74x | 1.52x | 2.65x | 1.78x | 30/30 |
| materialize | 1.07x | 2.09x | 1.62x | 1.73x | 2.70x | 1.94x | 30/30 |
| perft_bulk_d4 | 0.89x | 3.55x | 1.42x | 1.11x | 2.32x | 1.72x | 29/30 |
| perft_full_d3 | 0.98x | 1.45x | 1.70x | 1.64x | 2.13x | 1.76x | 30/30 |
| fen_parse | 0.98x | 1.25x | 1.31x | 1.17x | 1.47x | 1.25x | 30/30 |
| fen_write_native_ep | 1.01x | 5.05x | 10.37x | 1.93x | 29.81x | 2.19x | 30/30 |
| snapshot | 1.00x | 1.83x | 1.15x | 4.33x | 1.16x | 1.33x | 30/30 |
| san_parse_all | 0.98x | 2.08x | N/A | 4.30x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 9.25x | N/A | 2.14x | N/A | N/A | 12/12 |
| apply_children_full | 1.02x | 1.97x | 1.22x | 3.71x | 1.21x | 1.53x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.10x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.13x | 88.41x | 197.86x | 199.41x | 175.59x | 5/5 |
| chess960_materialize | 1.09x | 1.87x | 1.74x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.03x | 2.43x | 1.43x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 41.55x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.96x | 1.04x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.86x | 17.10x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.69x | 4.65x | 1.72x | 0.98x | 2.11x | 1.94x | 8/10 |
| corpus_materialize | 0.99x | 1.44x | 1.33x | 1.09x | 1.76x | 1.19x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.30x | 3.04x | 2.22x | 2.95x | 1.55x | 10/10 |
| replay_trusted_native | 1.00x | 0.81x | 1.47x | 1.53x | 2.04x | 2.37x | 9/10 |
| replay_checked_native | 0.99x | 2.10x | 1.10x | 8.72x | 8.69x | 9.83x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
