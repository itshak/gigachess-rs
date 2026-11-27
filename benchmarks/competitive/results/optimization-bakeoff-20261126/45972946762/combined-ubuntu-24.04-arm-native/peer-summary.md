| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.43x | 1.67x | 1.80x | 2.17x | 2.16x | 30/30 |
| materialize | 1.01x | 1.98x | 1.53x | 1.34x | 2.15x | 1.49x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.89x | 1.58x | 1.44x | 2.02x | 2.06x | 30/30 |
| perft_full_d3 | 1.00x | 1.49x | 1.61x | 1.33x | 1.96x | 1.86x | 30/30 |
| fen_parse | 0.99x | 1.23x | 1.32x | 1.25x | 1.74x | 1.40x | 30/30 |
| fen_write_native_ep | 0.99x | 4.50x | 9.23x | 1.56x | 29.81x | 2.39x | 30/30 |
| snapshot | 0.98x | 1.05x | 1.08x | 1.45x | 1.09x | 0.96x | 24/30 |
| san_parse_all | 0.99x | 1.84x | N/A | 4.28x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 12.49x | N/A | 2.67x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 1.25x | 1.48x | 4.00x | 1.19x | 1.19x | 30/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.58x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.34x | 76.65x | 129.51x | 175.65x | 142.98x | 5/5 |
| chess960_materialize | 1.01x | 1.72x | 1.58x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.99x | 1.50x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 25.33x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.09x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.00x | 19.25x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.45x | 1.81x | 1.35x | 2.05x | 2.22x | 10/10 |
| corpus_materialize | 0.99x | 1.60x | 1.57x | 1.22x | 1.93x | 1.39x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.11x | 2.94x | 2.11x | 2.89x | 1.46x | 10/10 |
| replay_trusted_native | 0.99x | 0.62x | 1.65x | 2.00x | 2.23x | 2.28x | 8/10 |
| replay_checked_native | 1.00x | 2.29x | 1.13x | 7.54x | 6.85x | 8.77x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
