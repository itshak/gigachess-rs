| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.92x | 1.68x | 1.42x | 2.17x | 1.73x | 30/30 |
| materialize | 0.96x | 2.01x | 1.48x | 1.63x | 2.36x | 1.85x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.76x | 1.54x | 1.17x | 2.34x | 1.85x | 30/30 |
| perft_full_d3 | 0.96x | 0.85x | 0.99x | 0.95x | 1.33x | 1.03x | 14/30 |
| fen_parse | 1.08x | 1.52x | 1.62x | 1.19x | 1.48x | 1.17x | 30/30 |
| fen_write_native_ep | 1.04x | 3.20x | 6.59x | 1.31x | 17.97x | 1.60x | 30/30 |
| snapshot | 1.66x | 1.67x | 1.33x | 4.01x | 1.33x | 1.33x | 30/30 |
| san_parse_all | 1.06x | 1.77x | N/A | 4.27x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 9.04x | N/A | 2.10x | N/A | N/A | 12/12 |
| apply_children_full | 0.93x | 1.54x | 0.98x | 2.84x | 1.00x | 1.15x | 24/30 |
| make_unmake_full | 1.00x | N/A | N/A | 0.94x | N/A | N/A | 1/6 |
| startpos | 1.19x | 0.99x | 78.20x | 143.70x | 172.83x | 144.81x | 4/5 |
| chess960_materialize | 1.08x | 1.58x | 1.51x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.18x | 1.32x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 38.78x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.04x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.87x | 18.90x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 6.24x | 2.36x | 1.29x | 3.00x | 2.63x | 10/10 |
| corpus_materialize | 1.00x | 1.43x | 1.33x | 1.08x | 1.77x | 1.21x | 10/10 |
| corpus_uci_sorted | 1.00x | 1.46x | 1.87x | 1.38x | 1.80x | 0.96x | 8/10 |
| replay_trusted_native | 0.98x | 0.59x | 1.12x | 1.09x | 1.52x | 1.76x | 7/10 |
| replay_checked_native | 1.01x | 1.78x | 0.98x | 7.35x | 7.21x | 8.54x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
