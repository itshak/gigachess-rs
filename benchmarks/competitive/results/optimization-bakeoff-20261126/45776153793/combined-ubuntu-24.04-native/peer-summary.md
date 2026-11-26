| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.96x | 1.69x | 1.43x | 2.21x | 1.72x | 30/30 |
| materialize | 1.00x | 2.06x | 1.59x | 1.60x | 2.44x | 1.84x | 30/30 |
| perft_bulk_d4 | 0.90x | 3.41x | 1.40x | 1.07x | 2.11x | 1.78x | 28/30 |
| perft_full_d3 | 0.85x | 0.75x | 0.86x | 0.83x | 1.13x | 0.93x | 7/30 |
| fen_parse | 0.79x | 1.21x | 1.08x | 0.97x | 1.28x | 1.02x | 19/30 |
| fen_write_native_ep | 1.04x | 2.96x | 6.55x | 1.32x | 18.50x | 1.65x | 30/30 |
| snapshot | 0.60x | 1.04x | 0.84x | 1.78x | 0.84x | 0.78x | 12/30 |
| san_parse_all | 0.89x | 1.61x | N/A | 3.74x | N/A | N/A | 12/12 |
| san_render_native | 0.95x | 8.61x | N/A | 2.01x | N/A | N/A | 12/12 |
| apply_children_full | 0.87x | 1.32x | 0.83x | 2.50x | 0.82x | 1.01x | 15/30 |
| make_unmake_full | 0.81x | N/A | N/A | 0.76x | N/A | N/A | 0/6 |
| startpos | 1.09x | 1.69x | 88.47x | 158.68x | 196.64x | 170.10x | 5/5 |
| chess960_materialize | 1.00x | 1.60x | 1.52x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.94x | 2.04x | 1.22x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.95x | 37.32x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.03x | 0.99x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.88x | 19.27x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 6.23x | 2.35x | 1.29x | 2.81x | 2.61x | 10/10 |
| corpus_materialize | 1.00x | 1.43x | 1.34x | 1.10x | 1.74x | 1.21x | 10/10 |
| corpus_uci_sorted | 1.00x | 1.42x | 1.86x | 1.37x | 1.84x | 0.98x | 8/10 |
| replay_trusted_native | 0.89x | 0.54x | 0.97x | 0.99x | 1.37x | 1.60x | 6/10 |
| replay_checked_native | 0.81x | 1.46x | 0.80x | 6.17x | 5.87x | 7.03x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
