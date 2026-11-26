| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.23x | 1.72x | 1.49x | 2.04x | 1.79x | 30/30 |
| materialize | 1.01x | 2.01x | 1.35x | 1.50x | 2.37x | 1.60x | 30/30 |
| perft_bulk_d4 | 1.00x | 4.02x | 1.60x | 1.22x | 2.22x | 1.92x | 30/30 |
| perft_full_d3 | 0.99x | 1.65x | 1.79x | 1.67x | 2.34x | 1.93x | 30/30 |
| fen_parse | 1.07x | 1.24x | 1.39x | 1.26x | 1.73x | 1.37x | 30/30 |
| fen_write_native_ep | 1.01x | 3.34x | 5.97x | 1.25x | 22.88x | 1.61x | 30/30 |
| snapshot | 2.28x | 2.08x | 1.13x | 4.87x | 1.12x | 1.12x | 30/30 |
| san_parse_all | 1.03x | 1.77x | N/A | 5.24x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 9.67x | N/A | 4.16x | N/A | N/A | 12/12 |
| apply_children_full | 1.04x | 1.67x | 1.08x | 8.40x | 1.07x | 1.32x | 30/30 |
| make_unmake_full | 0.98x | N/A | N/A | 1.09x | N/A | N/A | 5/6 |
| startpos | 1.40x | 1.12x | 86.89x | 175.90x | 239.46x | 190.47x | 5/5 |
| chess960_materialize | 1.00x | 1.54x | 1.30x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.02x | 2.35x | 1.38x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.02x | 42.42x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.02x | 1.03x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.24x | 29.93x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.12x | 6.47x | 2.48x | 1.33x | 2.88x | 2.89x | 10/10 |
| corpus_materialize | 1.02x | 1.42x | 1.21x | 1.09x | 1.76x | 1.21x | 10/10 |
| corpus_uci_sorted | 1.04x | 1.81x | 2.21x | 1.88x | 2.22x | 1.57x | 10/10 |
| replay_trusted_native | 1.00x | 0.70x | 1.22x | 1.32x | 1.78x | 2.00x | 7/10 |
| replay_checked_native | 1.04x | 2.07x | 1.10x | 8.52x | 8.12x | 9.58x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
