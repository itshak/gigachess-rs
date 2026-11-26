| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.98x | 4.38x | 1.44x | 1.42x | 2.13x | 1.77x | 29/30 |
| materialize | 0.97x | 1.97x | 1.49x | 1.19x | 2.31x | 1.44x | 30/30 |
| perft_bulk_d4 | 1.00x | 4.09x | 1.55x | 1.24x | 2.45x | 2.12x | 30/30 |
| perft_full_d3 | 1.01x | 1.06x | 1.26x | 0.96x | 1.58x | 1.76x | 23/30 |
| fen_parse | 1.07x | 1.22x | 1.40x | 1.08x | 1.84x | 1.23x | 28/30 |
| fen_write_native_ep | 1.00x | 2.92x | 6.72x | 1.12x | 19.34x | 1.55x | 30/30 |
| snapshot | 1.16x | 2.18x | 1.18x | 3.41x | 1.51x | 1.19x | 30/30 |
| san_parse_all | 1.01x | 1.86x | N/A | 3.94x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.12x | N/A | 2.23x | N/A | N/A | 12/12 |
| apply_children_full | 1.10x | 1.62x | 1.16x | 3.71x | 1.15x | 1.65x | 28/30 |
| make_unmake_full | 0.97x | N/A | N/A | 1.22x | N/A | N/A | 5/6 |
| startpos | 1.00x | 1.25x | 83.70x | 145.18x | 217.96x | 153.31x | 5/5 |
| chess960_materialize | 1.02x | 1.58x | 1.46x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.02x | 2.73x | 1.40x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.05x | 36.47x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.06x | 0.99x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.02x | 28.41x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 6.29x | 1.86x | 1.30x | 2.48x | 2.27x | 10/10 |
| corpus_materialize | 0.99x | 1.82x | 1.67x | 1.22x | 2.40x | 1.49x | 10/10 |
| corpus_uci_sorted | 1.03x | 2.34x | 3.00x | 2.47x | 3.01x | 1.71x | 10/10 |
| replay_trusted_native | 1.11x | 0.73x | 1.32x | 1.45x | 2.24x | 2.71x | 9/10 |
| replay_checked_native | 1.22x | 2.25x | 1.09x | 8.14x | 8.39x | 10.16x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
