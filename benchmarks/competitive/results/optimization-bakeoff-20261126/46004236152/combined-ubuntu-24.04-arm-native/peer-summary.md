| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.41x | 1.69x | 1.82x | 2.19x | 2.19x | 30/30 |
| materialize | 1.01x | 1.96x | 1.55x | 1.37x | 2.17x | 1.50x | 30/30 |
| perft_bulk_d4 | 1.05x | 4.02x | 1.64x | 1.48x | 2.11x | 2.16x | 30/30 |
| perft_full_d3 | 1.14x | 1.67x | 1.81x | 1.52x | 2.22x | 2.10x | 30/30 |
| fen_parse | 0.98x | 1.21x | 1.31x | 1.25x | 1.73x | 1.39x | 30/30 |
| fen_write_native_ep | 1.00x | 4.55x | 9.51x | 1.58x | 30.05x | 2.40x | 30/30 |
| snapshot | 1.00x | 1.16x | 1.01x | 1.49x | 0.99x | 0.94x | 19/30 |
| san_parse_all | 1.00x | 1.84x | N/A | 4.24x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 12.34x | N/A | 2.70x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.26x | 1.48x | 4.04x | 1.21x | 1.21x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.58x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.27x | 76.46x | 132.08x | 170.34x | 135.39x | 5/5 |
| chess960_materialize | 1.01x | 1.72x | 1.58x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.02x | 2.99x | 1.52x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 25.19x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.09x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.00x | 19.04x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.10x | 4.86x | 1.86x | 1.49x | 2.19x | 2.41x | 10/10 |
| corpus_materialize | 1.01x | 1.61x | 1.53x | 1.24x | 1.93x | 1.38x | 10/10 |
| corpus_uci_sorted | 0.98x | 2.07x | 2.87x | 2.04x | 2.82x | 1.43x | 10/10 |
| replay_trusted_native | 1.01x | 0.64x | 1.68x | 2.03x | 2.28x | 2.30x | 8/10 |
| replay_checked_native | 1.02x | 2.36x | 1.15x | 7.79x | 6.87x | 8.92x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
