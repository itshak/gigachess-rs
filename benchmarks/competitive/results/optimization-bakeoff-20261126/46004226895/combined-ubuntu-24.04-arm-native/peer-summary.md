| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.42x | 1.69x | 1.82x | 2.19x | 2.20x | 30/30 |
| materialize | 1.01x | 1.97x | 1.55x | 1.37x | 2.18x | 1.50x | 30/30 |
| perft_bulk_d4 | 1.04x | 4.03x | 1.64x | 1.48x | 2.10x | 2.14x | 30/30 |
| perft_full_d3 | 1.14x | 1.67x | 1.81x | 1.52x | 2.22x | 2.10x | 30/30 |
| fen_parse | 0.99x | 1.21x | 1.31x | 1.26x | 1.73x | 1.38x | 30/30 |
| fen_write_native_ep | 1.00x | 4.46x | 9.40x | 1.58x | 30.17x | 2.40x | 30/30 |
| snapshot | 1.01x | 1.16x | 1.00x | 1.49x | 1.01x | 0.94x | 20/30 |
| san_parse_all | 1.00x | 1.84x | N/A | 4.23x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 12.46x | N/A | 2.69x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.25x | 1.48x | 4.07x | 1.21x | 1.21x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.54x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.27x | 76.44x | 125.31x | 175.27x | 141.37x | 5/5 |
| chess960_materialize | 1.01x | 1.72x | 1.59x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.02x | 3.01x | 1.52x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 25.47x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 1.08x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.00x | 19.26x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.09x | 4.85x | 1.86x | 1.52x | 2.18x | 2.41x | 10/10 |
| corpus_materialize | 1.01x | 1.62x | 1.54x | 1.25x | 1.92x | 1.39x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.11x | 2.93x | 2.08x | 2.87x | 1.46x | 10/10 |
| replay_trusted_native | 1.00x | 0.63x | 1.67x | 2.00x | 2.25x | 2.28x | 8/10 |
| replay_checked_native | 1.01x | 2.35x | 1.14x | 7.66x | 6.80x | 8.82x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
