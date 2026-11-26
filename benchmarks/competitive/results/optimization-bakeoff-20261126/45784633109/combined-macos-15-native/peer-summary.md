| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.55x | 1.47x | 1.53x | 2.08x | 1.83x | 30/30 |
| materialize | 1.03x | 1.93x | 1.47x | 1.02x | 2.19x | 1.27x | 27/30 |
| perft_bulk_d4 | 0.93x | 3.63x | 1.34x | 1.12x | 1.71x | 1.71x | 29/30 |
| perft_full_d3 | 1.05x | 1.61x | 1.73x | 1.31x | 1.98x | 1.98x | 30/30 |
| fen_parse | 1.10x | 1.29x | 1.44x | 1.35x | 1.91x | 1.55x | 30/30 |
| fen_write_native_ep | 1.05x | 4.04x | 8.91x | 1.68x | 26.47x | 2.19x | 30/30 |
| snapshot | 1.03x | 1.66x | 1.12x | 2.37x | 1.11x | 0.98x | 27/30 |
| san_parse_all | 1.01x | 1.88x | N/A | 3.96x | N/A | N/A | 12/12 |
| san_render_native | 1.03x | 11.39x | N/A | 2.58x | N/A | N/A | 12/12 |
| apply_children_full | 1.10x | 0.95x | 1.01x | 4.78x | 0.98x | 1.13x | 19/30 |
| make_unmake_full | 1.09x | N/A | N/A | 1.18x | N/A | N/A | 6/6 |
| startpos | 1.16x | 1.25x | 69.24x | 140.85x | 178.87x | 172.68x | 5/5 |
| chess960_materialize | 0.92x | 1.43x | 1.45x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.77x | 2.93x | 1.22x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.08x | 40.45x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.05x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.98x | 19.83x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.72x | 4.62x | 1.47x | 0.87x | 1.70x | 1.93x | 8/10 |
| corpus_materialize | 0.90x | 1.30x | 1.26x | 1.12x | 1.77x | 1.22x | 10/10 |
| corpus_uci_sorted | 1.00x | 1.85x | 2.61x | 1.82x | 2.43x | 1.39x | 10/10 |
| replay_trusted_native | 1.14x | 0.56x | 1.43x | 1.39x | 1.54x | 1.53x | 8/10 |
| replay_checked_native | 1.05x | 2.13x | 0.97x | 7.82x | 6.84x | 7.98x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
