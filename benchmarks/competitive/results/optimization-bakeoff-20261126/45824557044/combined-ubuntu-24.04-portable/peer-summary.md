| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.04x | 2.91x | 1.48x | 1.34x | 1.87x | 1.34x | 30/30 |
| materialize | 0.99x | 1.98x | 1.75x | 1.26x | 2.32x | 1.50x | 30/30 |
| perft_bulk_d4 | 1.01x | 2.74x | 1.48x | 1.16x | 1.85x | 1.61x | 30/30 |
| perft_full_d3 | 1.00x | 1.41x | 1.66x | 1.35x | 1.97x | 1.81x | 30/30 |
| fen_parse | 1.05x | 1.54x | 1.48x | 1.22x | 2.28x | 1.34x | 30/30 |
| fen_write_native_ep | 1.00x | 2.80x | 9.73x | 1.71x | 27.70x | 1.97x | 30/30 |
| snapshot | 1.00x | 1.69x | 1.21x | 3.05x | 1.02x | 1.07x | 30/30 |
| san_parse_all | 1.02x | 2.12x | N/A | 4.28x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 10.76x | N/A | 2.25x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 2.18x | 1.50x | 4.31x | 1.36x | 1.36x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.34x | N/A | N/A | 6/6 |
| startpos | 0.97x | 1.27x | 63.91x | 109.55x | 182.07x | 115.05x | 5/5 |
| chess960_materialize | 1.00x | 1.65x | 1.49x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.17x | 1.45x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 23.24x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.03x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.05x | 18.30x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 3.60x | 1.77x | 1.22x | 1.94x | 2.38x | 10/10 |
| corpus_materialize | 1.00x | 1.53x | 1.52x | 1.13x | 1.98x | 1.30x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.13x | 3.21x | 2.44x | 3.04x | 1.76x | 10/10 |
| replay_trusted_native | 1.01x | 0.85x | 1.74x | 1.95x | 2.62x | 2.37x | 9/10 |
| replay_checked_native | 1.01x | 2.14x | 1.05x | 7.40x | 7.38x | 8.28x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
