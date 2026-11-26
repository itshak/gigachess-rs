| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.18x | 1.73x | 1.48x | 2.04x | 1.76x | 30/30 |
| materialize | 1.01x | 2.04x | 1.39x | 1.61x | 2.54x | 1.61x | 30/30 |
| perft_bulk_d4 | 1.01x | 3.99x | 1.64x | 1.28x | 2.38x | 1.90x | 30/30 |
| perft_full_d3 | 1.10x | 1.53x | 1.74x | 1.58x | 2.17x | 1.86x | 30/30 |
| fen_parse | 1.02x | 1.41x | 1.37x | 1.39x | 1.76x | 1.40x | 30/30 |
| fen_write_native_ep | 0.98x | 3.93x | 7.54x | 1.59x | 29.89x | 2.16x | 30/30 |
| snapshot | 1.13x | 1.73x | 1.08x | 3.04x | 1.06x | 0.99x | 25/30 |
| san_parse_all | 1.00x | 1.80x | N/A | 5.10x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 9.58x | N/A | 3.82x | N/A | N/A | 12/12 |
| apply_children_full | 0.77x | 1.32x | 0.84x | 5.68x | 0.86x | 1.00x | 14/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.07x | N/A | N/A | 4/6 |
| startpos | 0.96x | 1.06x | 66.06x | 121.52x | 182.19x | 125.95x | 5/5 |
| chess960_materialize | 0.96x | 1.59x | 1.40x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.86x | 2.30x | 1.35x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.96x | 43.49x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.92x | 1.00x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.93x | 23.62x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.96x | 6.52x | 2.53x | 1.45x | 3.00x | 2.80x | 10/10 |
| corpus_materialize | 1.00x | 1.48x | 1.28x | 1.11x | 1.86x | 1.21x | 10/10 |
| corpus_uci_sorted | 1.02x | 1.86x | 2.20x | 1.88x | 2.21x | 1.58x | 10/10 |
| replay_trusted_native | 0.95x | 0.70x | 1.21x | 1.29x | 1.79x | 2.22x | 8/10 |
| replay_checked_native | 1.00x | 2.05x | 1.11x | 8.79x | 8.35x | 10.17x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
