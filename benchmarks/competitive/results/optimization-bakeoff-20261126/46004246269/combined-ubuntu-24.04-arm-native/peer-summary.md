| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.45x | 1.69x | 1.81x | 2.20x | 2.19x | 30/30 |
| materialize | 1.02x | 1.97x | 1.56x | 1.37x | 2.18x | 1.50x | 30/30 |
| perft_bulk_d4 | 1.04x | 4.03x | 1.64x | 1.49x | 2.09x | 2.15x | 30/30 |
| perft_full_d3 | 1.13x | 1.67x | 1.82x | 1.51x | 2.23x | 2.10x | 30/30 |
| fen_parse | 0.99x | 1.25x | 1.31x | 1.25x | 1.74x | 1.39x | 30/30 |
| fen_write_native_ep | 1.00x | 4.53x | 9.30x | 1.59x | 30.21x | 2.45x | 30/30 |
| snapshot | 1.00x | 1.10x | 1.05x | 1.47x | 1.05x | 0.95x | 24/30 |
| san_parse_all | 1.00x | 1.84x | N/A | 4.25x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 12.46x | N/A | 2.67x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.25x | 1.48x | 4.05x | 1.20x | 1.20x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.55x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.30x | 76.31x | 127.64x | 174.27x | 140.00x | 5/5 |
| chess960_materialize | 1.01x | 1.71x | 1.58x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.02x | 3.02x | 1.53x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 25.68x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 1.08x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.00x | 19.17x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.07x | 4.76x | 1.85x | 1.48x | 2.19x | 2.36x | 10/10 |
| corpus_materialize | 0.99x | 1.60x | 1.53x | 1.23x | 1.92x | 1.37x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.11x | 2.90x | 2.08x | 2.86x | 1.46x | 10/10 |
| replay_trusted_native | 1.01x | 0.64x | 1.66x | 2.02x | 2.26x | 2.30x | 8/10 |
| replay_checked_native | 0.98x | 2.27x | 1.11x | 7.45x | 6.68x | 8.60x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
