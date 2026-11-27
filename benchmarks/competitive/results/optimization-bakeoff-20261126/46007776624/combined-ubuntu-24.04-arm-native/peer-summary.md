| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.44x | 1.69x | 1.81x | 2.21x | 2.19x | 30/30 |
| materialize | 1.01x | 1.98x | 1.55x | 1.35x | 2.20x | 1.50x | 30/30 |
| perft_bulk_d4 | 1.05x | 4.06x | 1.64x | 1.52x | 2.10x | 2.15x | 30/30 |
| perft_full_d3 | 1.13x | 1.68x | 1.82x | 1.51x | 2.23x | 2.10x | 30/30 |
| fen_parse | 0.98x | 1.20x | 1.29x | 1.22x | 1.70x | 1.37x | 30/30 |
| fen_write_native_ep | 0.99x | 4.46x | 9.33x | 1.57x | 30.04x | 2.38x | 30/30 |
| snapshot | 1.01x | 1.14x | 1.01x | 1.49x | 0.99x | 0.94x | 18/30 |
| san_parse_all | 1.00x | 1.84x | N/A | 4.28x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 12.44x | N/A | 2.70x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.24x | 1.47x | 4.04x | 1.21x | 1.20x | 30/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.57x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.27x | 76.76x | 130.60x | 176.87x | 145.19x | 5/5 |
| chess960_materialize | 1.00x | 1.71x | 1.58x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.02x | 3.02x | 1.52x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 25.01x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.87x | 0.94x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 1.00x | 18.84x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.10x | 4.88x | 1.85x | 1.51x | 2.18x | 2.40x | 10/10 |
| corpus_materialize | 1.01x | 1.63x | 1.54x | 1.23x | 1.92x | 1.40x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.11x | 2.93x | 2.08x | 2.87x | 1.46x | 10/10 |
| replay_trusted_native | 1.01x | 0.63x | 1.70x | 2.02x | 2.28x | 2.32x | 8/10 |
| replay_checked_native | 1.01x | 2.34x | 1.14x | 7.64x | 6.91x | 8.87x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
