| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.94x | 5.29x | 1.50x | 1.74x | 2.33x | 1.87x | 30/30 |
| materialize | 1.02x | 2.03x | 1.52x | 1.08x | 2.27x | 1.47x | 27/30 |
| perft_bulk_d4 | 0.99x | 3.67x | 1.38x | 1.15x | 1.72x | 1.69x | 28/30 |
| perft_full_d3 | 1.04x | 1.62x | 1.71x | 1.40x | 2.04x | 2.12x | 30/30 |
| fen_parse | 0.94x | 1.21x | 1.26x | 1.33x | 1.48x | 1.45x | 30/30 |
| fen_write_native_ep | 0.90x | 4.03x | 8.71x | 1.68x | 25.72x | 2.20x | 30/30 |
| snapshot | 0.94x | 1.98x | 1.07x | 2.65x | 1.13x | 1.05x | 26/30 |
| san_parse_all | 0.97x | 1.84x | N/A | 3.80x | N/A | N/A | 12/12 |
| san_render_native | 1.02x | 10.84x | N/A | 2.38x | N/A | N/A | 12/12 |
| apply_children_full | 0.97x | 1.09x | 1.19x | 5.59x | 1.16x | 1.25x | 25/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.27x | N/A | N/A | 6/6 |
| startpos | 0.99x | 1.24x | 68.85x | 137.02x | 218.37x | 147.82x | 5/5 |
| chess960_materialize | 1.03x | 1.80x | 1.70x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.66x | 1.14x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.02x | 35.26x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 0.89x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 1.20x | 21.65x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 5.12x | 1.96x | 0.89x | 1.86x | 2.02x | 8/10 |
| corpus_materialize | 1.14x | 1.34x | 1.28x | 1.09x | 1.81x | 1.15x | 10/10 |
| corpus_uci_sorted | 1.49x | 1.73x | 2.61x | 1.97x | 2.42x | 1.40x | 10/10 |
| replay_trusted_native | 1.05x | 0.65x | 1.50x | 1.47x | 1.62x | 1.61x | 8/10 |
| replay_checked_native | 1.12x | 2.33x | 1.04x | 7.92x | 6.89x | 8.68x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
