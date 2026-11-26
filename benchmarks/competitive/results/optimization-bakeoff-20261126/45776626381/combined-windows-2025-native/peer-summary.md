| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.04x | 4.58x | 1.56x | 1.53x | 2.35x | 1.90x | 30/30 |
| materialize | 1.01x | 2.13x | 1.34x | 1.13x | 2.21x | 1.60x | 30/30 |
| perft_bulk_d4 | 0.99x | 3.74x | 1.53x | 1.36x | 2.49x | 2.10x | 30/30 |
| perft_full_d3 | 1.61x | 1.20x | 1.26x | 1.04x | 1.70x | 1.52x | 27/30 |
| fen_parse | 0.79x | 1.23x | 1.30x | 1.32x | 1.85x | 1.30x | 30/30 |
| fen_write_native_ep | 0.99x | 3.10x | 5.73x | 1.22x | 24.11x | 1.54x | 30/30 |
| snapshot | 0.32x | 1.23x | 1.10x | 0.94x | 1.09x | 0.99x | 19/30 |
| san_parse_all | 1.08x | 1.92x | N/A | 4.65x | N/A | N/A | 12/12 |
| san_render_native | 1.02x | 10.28x | N/A | 3.44x | N/A | N/A | 12/12 |
| apply_children_full | 1.75x | 1.53x | 1.05x | 6.35x | 1.21x | 1.37x | 27/30 |
| make_unmake_full | 1.37x | N/A | N/A | 1.25x | N/A | N/A | 6/6 |
| startpos | 0.98x | 2.74x | 89.17x | 157.35x | 235.37x | 158.83x | 5/5 |
| chess960_materialize | 1.01x | 1.55x | 1.21x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.97x | 2.45x | 1.36x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 20.06x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.96x | 0.92x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 0.94x | 16.83x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 4.46x | 1.78x | 1.34x | 2.35x | 2.17x | 10/10 |
| corpus_materialize | 0.98x | 1.76x | 1.34x | 1.19x | 2.78x | 1.47x | 10/10 |
| corpus_uci_sorted | 1.40x | 1.78x | 2.16x | 1.84x | 2.13x | 1.40x | 10/10 |
| replay_trusted_native | 1.48x | 0.76x | 1.26x | 1.57x | 2.60x | 3.36x | 8/10 |
| replay_checked_native | 1.04x | 2.11x | 0.93x | 5.24x | 6.63x | 7.37x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
