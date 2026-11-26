| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.84x | 4.29x | 1.38x | 1.60x | 2.19x | 1.90x | 30/30 |
| materialize | 0.86x | 1.90x | 1.36x | 0.96x | 2.10x | 1.17x | 25/30 |
| perft_bulk_d4 | 1.01x | 4.18x | 1.66x | 1.33x | 2.26x | 1.93x | 30/30 |
| perft_full_d3 | 1.00x | 1.77x | 1.82x | 1.46x | 2.16x | 2.21x | 30/30 |
| fen_parse | 1.04x | 1.31x | 1.46x | 1.54x | 1.65x | 1.52x | 29/30 |
| fen_write_native_ep | 1.02x | 4.17x | 8.96x | 1.69x | 27.46x | 2.21x | 30/30 |
| snapshot | 1.02x | 1.60x | 1.10x | 2.45x | 1.15x | 1.06x | 25/30 |
| san_parse_all | 0.92x | 1.82x | N/A | 4.56x | N/A | N/A | 12/12 |
| san_render_native | 0.91x | 12.07x | N/A | 2.79x | N/A | N/A | 12/12 |
| apply_children_full | 0.91x | 0.93x | 1.07x | 5.39x | 1.01x | 1.17x | 20/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.19x | N/A | N/A | 6/6 |
| startpos | 1.10x | 0.73x | 55.92x | 135.42x | 117.98x | 129.36x | 4/5 |
| chess960_materialize | 1.53x | 1.79x | 1.80x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.36x | 2.86x | 1.40x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.16x | 40.03x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.35x | 1.00x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.16x | 19.73x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.17x | 5.36x | 1.75x | 1.35x | 2.36x | 2.49x | 10/10 |
| corpus_materialize | 1.30x | 1.56x | 1.48x | 1.23x | 2.21x | 1.24x | 10/10 |
| corpus_uci_sorted | 1.41x | 1.87x | 2.53x | 1.75x | 2.52x | 1.44x | 10/10 |
| replay_trusted_native | 1.15x | 0.58x | 1.61x | 1.54x | 1.83x | 1.62x | 8/10 |
| replay_checked_native | 1.27x | 2.28x | 1.00x | 8.40x | 6.58x | 7.94x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
