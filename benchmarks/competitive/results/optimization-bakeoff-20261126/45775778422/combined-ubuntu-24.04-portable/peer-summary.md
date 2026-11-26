| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 2.60x | 1.48x | 1.29x | 1.73x | 1.23x | 28/30 |
| materialize | 1.06x | 2.26x | 1.74x | 1.75x | 2.77x | 1.99x | 30/30 |
| perft_bulk_d4 | 1.00x | 2.90x | 1.55x | 1.15x | 1.92x | 1.71x | 30/30 |
| perft_full_d3 | 1.01x | 0.98x | 1.16x | 1.02x | 1.42x | 1.15x | 22/30 |
| fen_parse | 0.92x | 1.37x | 1.36x | 1.10x | 1.41x | 1.10x | 29/30 |
| fen_write_native_ep | 1.00x | 2.02x | 6.82x | 1.23x | 19.17x | 1.64x | 30/30 |
| snapshot | 1.12x | 1.46x | 1.12x | 2.91x | 1.12x | 1.00x | 29/30 |
| san_parse_all | 0.99x | 1.67x | N/A | 3.88x | N/A | N/A | 12/12 |
| san_render_native | 1.02x | 9.02x | N/A | 2.09x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.62x | 1.06x | 3.05x | 1.01x | 1.14x | 25/30 |
| make_unmake_full | 1.05x | N/A | N/A | 1.01x | N/A | N/A | 2/6 |
| startpos | 1.01x | 1.36x | 57.72x | 108.96x | 121.65x | 96.40x | 5/5 |
| chess960_materialize | 1.00x | 1.60x | 1.58x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 1.82x | 1.29x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.98x | 42.02x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.03x | 1.04x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.95x | 22.77x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 4.47x | 2.27x | 1.23x | 2.28x | 3.08x | 10/10 |
| corpus_materialize | 1.02x | 1.54x | 1.42x | 1.17x | 1.90x | 1.31x | 10/10 |
| corpus_uci_sorted | 0.99x | 1.21x | 1.92x | 1.38x | 1.89x | 0.96x | 8/10 |
| replay_trusted_native | 1.00x | 0.59x | 1.16x | 1.16x | 1.55x | 1.39x | 7/10 |
| replay_checked_native | 1.02x | 1.73x | 0.97x | 7.49x | 7.24x | 8.28x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
