| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.15x | 1.55x | 1.48x | 2.09x | 1.76x | 30/30 |
| materialize | 0.99x | 1.94x | 1.33x | 1.18x | 2.22x | 1.47x | 30/30 |
| perft_bulk_d4 | 0.97x | 3.80x | 1.49x | 1.34x | 2.16x | 1.85x | 30/30 |
| perft_full_d3 | 1.00x | 0.77x | 0.83x | 0.74x | 1.08x | 1.03x | 7/30 |
| fen_parse | 1.08x | 1.29x | 1.43x | 1.34x | 2.29x | 1.56x | 30/30 |
| fen_write_native_ep | 1.00x | 2.62x | 5.04x | 1.02x | 19.31x | 1.23x | 26/30 |
| snapshot | 0.57x | 1.15x | 1.14x | 3.06x | 1.14x | 0.86x | 24/30 |
| san_parse_all | 1.03x | 1.84x | N/A | 5.20x | N/A | N/A | 12/12 |
| san_render_native | 1.02x | 10.45x | N/A | 3.60x | N/A | N/A | 12/12 |
| apply_children_full | 0.96x | 1.35x | 0.83x | 5.23x | 0.90x | 1.02x | 15/30 |
| make_unmake_full | 1.00x | N/A | N/A | 0.99x | N/A | N/A | 3/6 |
| startpos | 1.01x | 1.14x | 68.28x | 130.16x | 204.19x | 154.14x | 5/5 |
| chess960_materialize | 1.00x | 1.42x | 1.18x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.44x | 1.32x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 29.58x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.99x | 20.01x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 5.35x | 1.77x | 1.36x | 2.48x | 2.37x | 10/10 |
| corpus_materialize | 1.01x | 1.61x | 1.29x | 1.16x | 2.00x | 1.40x | 10/10 |
| corpus_uci_sorted | 1.48x | 1.78x | 2.21x | 1.81x | 2.16x | 1.42x | 10/10 |
| replay_trusted_native | 1.00x | 0.51x | 1.01x | 1.28x | 1.64x | 1.91x | 7/10 |
| replay_checked_native | 1.00x | 1.97x | 0.89x | 6.66x | 6.69x | 8.05x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
