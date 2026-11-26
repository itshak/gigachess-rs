| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.42x | 1.69x | 1.80x | 2.19x | 2.17x | 30/30 |
| materialize | 1.00x | 1.95x | 1.54x | 1.33x | 2.15x | 1.48x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.85x | 1.58x | 1.43x | 2.01x | 2.06x | 30/30 |
| perft_full_d3 | 1.00x | 1.48x | 1.60x | 1.33x | 1.96x | 1.85x | 30/30 |
| fen_parse | 1.00x | 1.23x | 1.32x | 1.29x | 1.74x | 1.38x | 30/30 |
| fen_write_native_ep | 1.00x | 4.53x | 9.33x | 1.62x | 29.40x | 2.43x | 30/30 |
| snapshot | 1.02x | 1.12x | 1.01x | 1.49x | 1.00x | 0.94x | 20/30 |
| san_parse_all | 1.00x | 1.86x | N/A | 4.26x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 12.36x | N/A | 2.67x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.25x | 1.47x | 4.00x | 1.20x | 1.21x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.56x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.28x | 76.40x | 130.01x | 168.75x | 134.05x | 5/5 |
| chess960_materialize | 1.00x | 1.71x | 1.58x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 3.00x | 1.51x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 26.23x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.09x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.00x | 19.61x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 4.45x | 1.71x | 1.36x | 2.00x | 2.20x | 10/10 |
| corpus_materialize | 1.00x | 1.60x | 1.54x | 1.22x | 1.91x | 1.37x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.10x | 2.94x | 2.08x | 2.86x | 1.46x | 10/10 |
| replay_trusted_native | 0.99x | 0.63x | 1.62x | 1.96x | 2.22x | 2.25x | 8/10 |
| replay_checked_native | 1.00x | 2.32x | 1.13x | 7.64x | 6.82x | 8.78x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
