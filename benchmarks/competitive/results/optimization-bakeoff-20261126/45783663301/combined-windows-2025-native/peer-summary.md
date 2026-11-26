| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.87x | 3.92x | 1.32x | 1.23x | 1.85x | 1.63x | 28/30 |
| materialize | 0.98x | 2.01x | 1.34x | 1.13x | 2.24x | 1.43x | 29/30 |
| perft_bulk_d4 | 0.96x | 3.81x | 1.45x | 1.21x | 2.27x | 2.06x | 30/30 |
| perft_full_d3 | 0.98x | 1.10x | 1.22x | 1.01x | 1.58x | 1.71x | 24/30 |
| fen_parse | 0.96x | 1.31x | 1.46x | 1.47x | 2.10x | 1.54x | 26/30 |
| fen_write_native_ep | 1.01x | 3.02x | 5.79x | 1.26x | 25.72x | 1.72x | 30/30 |
| snapshot | 1.24x | 2.10x | 1.25x | 3.86x | 1.25x | 1.24x | 30/30 |
| san_parse_all | 0.99x | 1.96x | N/A | 4.62x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 10.45x | N/A | 3.68x | N/A | N/A | 12/12 |
| apply_children_full | 0.98x | 1.45x | 1.10x | 6.34x | 1.12x | 1.51x | 29/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.25x | N/A | N/A | 6/6 |
| startpos | 1.02x | 1.25x | 89.16x | 174.43x | 245.65x | 186.09x | 5/5 |
| chess960_materialize | 1.06x | 1.42x | 1.17x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.90x | 2.30x | 1.18x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.06x | 38.01x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.07x | 0.98x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.97x | 25.43x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.61x | 3.60x | 1.07x | 0.70x | 1.38x | 1.36x | 7/10 |
| corpus_materialize | 1.09x | 1.79x | 1.42x | 1.19x | 2.40x | 1.48x | 10/10 |
| corpus_uci_sorted | 1.04x | 1.63x | 1.99x | 1.73x | 1.94x | 1.38x | 10/10 |
| replay_trusted_native | 1.01x | 0.66x | 1.20x | 1.44x | 2.30x | 2.68x | 8/10 |
| replay_checked_native | 1.05x | 2.02x | 0.92x | 7.13x | 7.20x | 8.94x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
