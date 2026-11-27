| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.03x | 5.32x | 1.93x | 1.94x | 2.81x | 2.30x | 30/30 |
| materialize | 0.90x | 2.05x | 1.52x | 1.05x | 2.26x | 1.26x | 28/30 |
| perft_bulk_d4 | 1.02x | 4.02x | 1.66x | 1.20x | 2.07x | 1.86x | 28/30 |
| perft_full_d3 | 0.99x | 1.43x | 1.66x | 1.38x | 1.89x | 1.97x | 30/30 |
| fen_parse | 0.99x | 1.20x | 1.46x | 1.22x | 1.66x | 1.37x | 30/30 |
| fen_write_native_ep | 1.16x | 4.56x | 9.02x | 1.93x | 27.57x | 2.44x | 30/30 |
| snapshot | 0.93x | 1.67x | 1.16x | 2.20x | 1.31x | 0.97x | 27/30 |
| san_parse_all | 1.02x | 1.66x | N/A | 3.51x | N/A | N/A | 12/12 |
| san_render_native | 1.05x | 11.84x | N/A | 2.65x | N/A | N/A | 12/12 |
| apply_children_full | 1.08x | 1.12x | 1.16x | 5.35x | 1.09x | 1.34x | 23/30 |
| make_unmake_full | 1.09x | N/A | N/A | 1.30x | N/A | N/A | 6/6 |
| startpos | 0.99x | 1.23x | 81.19x | 156.04x | 182.53x | 149.99x | 5/5 |
| chess960_materialize | 1.03x | 1.98x | 1.54x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.02x | 3.14x | 1.51x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.90x | 32.66x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.86x | 0.85x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 0.92x | 18.91x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 6.13x | 1.79x | 1.20x | 2.19x | 2.51x | 10/10 |
| corpus_materialize | 0.89x | 1.27x | 1.34x | 1.24x | 1.73x | 1.33x | 10/10 |
| corpus_uci_sorted | 0.90x | 1.94x | 2.58x | 1.76x | 2.19x | 1.18x | 10/10 |
| replay_trusted_native | 1.09x | 0.64x | 1.65x | 1.85x | 1.76x | 1.92x | 8/10 |
| replay_checked_native | 1.17x | 2.63x | 1.42x | 9.62x | 8.40x | 10.51x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
