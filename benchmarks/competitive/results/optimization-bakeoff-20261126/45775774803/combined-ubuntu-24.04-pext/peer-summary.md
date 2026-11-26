| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.58x | 1.55x | 1.56x | 2.48x | 1.80x | 30/30 |
| materialize | 1.01x | 2.22x | 1.74x | 1.17x | 2.33x | 1.46x | 30/30 |
| perft_bulk_d4 | 1.00x | 4.13x | 1.61x | 1.33x | 2.58x | 2.09x | 30/30 |
| perft_full_d3 | 0.98x | 0.82x | 0.85x | 0.65x | 1.21x | 0.97x | 8/30 |
| fen_parse | 1.14x | 1.42x | 1.57x | 1.17x | 2.00x | 1.21x | 30/30 |
| fen_write_native_ep | 1.00x | 2.62x | 6.43x | 1.24x | 19.09x | 1.37x | 30/30 |
| snapshot | 1.25x | 1.92x | 1.25x | 3.23x | 1.32x | 1.25x | 30/30 |
| san_parse_all | 1.06x | 2.25x | N/A | 3.86x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.06x | N/A | 2.21x | N/A | N/A | 12/12 |
| apply_children_full | 1.37x | 1.41x | 0.93x | 3.34x | 1.01x | 1.18x | 22/30 |
| make_unmake_full | 0.99x | N/A | N/A | 0.97x | N/A | N/A | 1/6 |
| startpos | 0.91x | 1.00x | 81.40x | 115.51x | 185.54x | 119.89x | 4/5 |
| chess960_materialize | 0.99x | 1.73x | 1.45x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.85x | 1.44x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 20.39x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.03x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.01x | 19.26x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.83x | 1.83x | 1.40x | 2.54x | 2.09x | 10/10 |
| corpus_materialize | 0.98x | 1.95x | 1.74x | 1.29x | 2.80x | 1.55x | 10/10 |
| corpus_uci_sorted | 1.01x | 1.35x | 1.83x | 1.36x | 1.73x | 0.97x | 8/10 |
| replay_trusted_native | 0.99x | 0.61x | 1.11x | 1.19x | 2.10x | 2.42x | 7/10 |
| replay_checked_native | 1.01x | 2.46x | 0.96x | 5.30x | 6.01x | 7.11x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
