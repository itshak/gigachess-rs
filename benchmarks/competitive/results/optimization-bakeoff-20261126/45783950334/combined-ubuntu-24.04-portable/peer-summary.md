| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 2.90x | 1.48x | 1.33x | 1.82x | 1.30x | 29/30 |
| materialize | 1.01x | 2.23x | 1.73x | 1.22x | 2.28x | 1.58x | 30/30 |
| perft_bulk_d4 | 1.01x | 2.86x | 1.52x | 1.20x | 1.90x | 1.64x | 30/30 |
| perft_full_d3 | 1.00x | 1.22x | 1.46x | 1.14x | 1.71x | 1.62x | 28/30 |
| fen_parse | 0.91x | 1.37x | 1.37x | 1.08x | 1.77x | 1.20x | 30/30 |
| fen_write_native_ep | 1.00x | 2.65x | 9.31x | 1.70x | 26.26x | 2.02x | 30/30 |
| snapshot | 1.00x | 1.72x | 1.21x | 2.82x | 1.15x | 0.93x | 24/30 |
| san_parse_all | 1.00x | 2.18x | N/A | 3.87x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.13x | N/A | 2.32x | N/A | N/A | 12/12 |
| apply_children_full | 0.96x | 1.80x | 1.13x | 4.25x | 1.05x | 1.07x | 25/30 |
| make_unmake_full | 0.92x | N/A | N/A | 1.19x | N/A | N/A | 6/6 |
| startpos | 0.99x | 1.30x | 73.85x | 104.76x | 159.03x | 114.54x | 5/5 |
| chess960_materialize | 0.95x | 1.68x | 1.35x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.18x | 1.41x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 21.61x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 0.98x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.95x | 19.24x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 3.17x | 1.77x | 1.23x | 1.92x | 1.84x | 10/10 |
| corpus_materialize | 1.00x | 1.98x | 1.79x | 1.29x | 2.89x | 1.66x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.04x | 2.95x | 2.21x | 2.80x | 1.58x | 10/10 |
| replay_trusted_native | 0.83x | 0.57x | 1.14x | 1.25x | 1.83x | 1.65x | 7/10 |
| replay_checked_native | 0.94x | 2.16x | 0.93x | 5.06x | 5.61x | 6.39x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
