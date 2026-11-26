| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.91x | 4.28x | 1.31x | 1.43x | 1.75x | 1.62x | 30/30 |
| materialize | 0.93x | 1.89x | 1.33x | 1.04x | 2.74x | 1.21x | 28/30 |
| perft_bulk_d4 | 0.94x | 4.24x | 1.55x | 1.29x | 1.97x | 1.96x | 30/30 |
| perft_full_d3 | 0.93x | 0.93x | 0.99x | 0.77x | 1.14x | 1.22x | 16/30 |
| fen_parse | 0.86x | 1.18x | 1.32x | 1.22x | 1.56x | 1.28x | 30/30 |
| fen_write_native_ep | 0.93x | 3.11x | 6.54x | 1.28x | 20.19x | 1.61x | 30/30 |
| snapshot | 1.01x | 1.69x | 1.14x | 2.37x | 1.10x | 0.99x | 25/30 |
| san_parse_all | 0.94x | 1.61x | N/A | 3.63x | N/A | N/A | 12/12 |
| san_render_native | 0.91x | 11.17x | N/A | 2.54x | N/A | N/A | 12/12 |
| apply_children_full | 0.93x | 0.80x | 0.86x | 4.11x | 0.82x | 0.93x | 10/30 |
| make_unmake_full | 0.90x | N/A | N/A | 0.98x | N/A | N/A | 3/6 |
| startpos | 0.62x | 0.85x | 48.28x | 104.03x | 109.06x | 92.71x | 4/5 |
| chess960_materialize | 0.89x | 1.47x | 1.23x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.84x | 2.88x | 1.38x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.87x | 37.30x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.93x | 1.03x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.93x | 19.31x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.87x | 5.99x | 1.92x | 1.23x | 2.27x | 2.58x | 10/10 |
| corpus_materialize | 0.98x | 1.53x | 1.51x | 1.22x | 2.03x | 1.35x | 10/10 |
| corpus_uci_sorted | 0.91x | 1.25x | 1.76x | 1.48x | 1.75x | 1.00x | 9/10 |
| replay_trusted_native | 1.02x | 0.48x | 1.12x | 1.21x | 1.35x | 1.27x | 8/10 |
| replay_checked_native | 0.89x | 1.90x | 0.91x | 6.45x | 5.85x | 6.99x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
