| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.87x | 4.48x | 1.39x | 1.28x | 1.83x | 1.58x | 28/30 |
| materialize | 0.98x | 2.07x | 1.53x | 1.08x | 2.27x | 1.43x | 29/30 |
| perft_bulk_d4 | 0.97x | 3.92x | 1.58x | 1.28x | 1.86x | 1.84x | 30/30 |
| perft_full_d3 | 1.02x | 1.59x | 1.87x | 1.37x | 2.15x | 2.03x | 30/30 |
| fen_parse | 1.02x | 1.32x | 1.39x | 1.48x | 1.90x | 1.49x | 30/30 |
| fen_write_native_ep | 0.91x | 2.89x | 6.32x | 1.24x | 20.42x | 1.62x | 28/30 |
| snapshot | 0.87x | 1.61x | 1.00x | 1.96x | 0.79x | 0.83x | 17/30 |
| san_parse_all | 0.95x | 1.84x | N/A | 4.18x | N/A | N/A | 12/12 |
| san_render_native | 0.90x | 11.31x | N/A | 2.71x | N/A | N/A | 12/12 |
| apply_children_full | 0.79x | 0.94x | 1.00x | 4.85x | 1.00x | 1.04x | 17/30 |
| make_unmake_full | 0.91x | N/A | N/A | 1.27x | N/A | N/A | 5/6 |
| startpos | 2.65x | 1.21x | 87.79x | 158.51x | 222.20x | 156.92x | 5/5 |
| chess960_materialize | 0.98x | 1.63x | 1.54x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.09x | 2.79x | 1.27x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.18x | 39.51x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.85x | 0.95x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.86x | 17.71x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.93x | 6.28x | 2.19x | 1.27x | 2.38x | 2.82x | 10/10 |
| corpus_materialize | 0.89x | 1.44x | 1.31x | 1.02x | 1.72x | 1.10x | 8/10 |
| corpus_uci_sorted | 0.96x | 1.71x | 2.35x | 1.71x | 2.44x | 1.21x | 10/10 |
| replay_trusted_native | 0.87x | 0.51x | 1.30x | 1.25x | 1.42x | 1.52x | 8/10 |
| replay_checked_native | 1.07x | 2.28x | 1.03x | 7.53x | 6.83x | 8.56x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
