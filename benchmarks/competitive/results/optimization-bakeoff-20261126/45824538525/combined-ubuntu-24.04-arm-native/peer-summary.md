| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.93x | 1.49x | 1.61x | 1.94x | 1.93x | 30/30 |
| materialize | 1.01x | 1.86x | 1.47x | 1.27x | 2.04x | 1.40x | 30/30 |
| perft_bulk_d4 | 0.99x | 3.40x | 1.39x | 1.25x | 1.77x | 1.81x | 30/30 |
| perft_full_d3 | 1.00x | 1.18x | 1.28x | 1.06x | 1.56x | 1.48x | 29/30 |
| fen_parse | 1.00x | 1.25x | 1.36x | 1.29x | 1.75x | 1.43x | 30/30 |
| fen_write_native_ep | 0.99x | 4.48x | 9.19x | 1.58x | 30.23x | 2.39x | 30/30 |
| snapshot | 0.99x | 1.05x | 1.07x | 1.45x | 1.08x | 0.96x | 24/30 |
| san_parse_all | 1.01x | 1.87x | N/A | 4.28x | N/A | N/A | 12/12 |
| san_render_native | 1.02x | 11.98x | N/A | 2.57x | N/A | N/A | 12/12 |
| apply_children_full | 0.96x | 1.33x | 1.58x | 4.29x | 1.27x | 1.30x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.53x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.33x | 77.40x | 130.02x | 174.81x | 142.19x | 5/5 |
| chess960_materialize | 1.01x | 1.60x | 1.48x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.66x | 1.35x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 25.79x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.02x | 1.09x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.99x | 18.29x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 3.95x | 1.51x | 1.21x | 1.78x | 1.98x | 10/10 |
| corpus_materialize | 1.01x | 1.54x | 1.47x | 1.17x | 1.82x | 1.32x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.10x | 2.91x | 2.08x | 2.84x | 1.46x | 10/10 |
| replay_trusted_native | 1.03x | 0.63x | 1.65x | 2.00x | 2.24x | 2.28x | 8/10 |
| replay_checked_native | 1.01x | 2.22x | 1.08x | 7.28x | 6.38x | 8.28x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
