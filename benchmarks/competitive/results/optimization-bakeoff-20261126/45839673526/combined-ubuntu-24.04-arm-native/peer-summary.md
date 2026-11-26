| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.98x | 4.27x | 1.63x | 1.76x | 2.12x | 2.11x | 30/30 |
| materialize | 0.99x | 1.93x | 1.51x | 1.33x | 2.13x | 1.47x | 30/30 |
| perft_bulk_d4 | 0.96x | 3.69x | 1.51x | 1.37x | 1.93x | 1.97x | 30/30 |
| perft_full_d3 | 1.03x | 0.94x | 1.03x | 0.86x | 1.25x | 1.19x | 19/30 |
| fen_parse | 1.02x | 1.24x | 1.33x | 1.26x | 1.77x | 1.42x | 30/30 |
| fen_write_native_ep | 0.98x | 4.49x | 9.22x | 1.55x | 29.79x | 2.36x | 30/30 |
| snapshot | 1.08x | 1.20x | 0.98x | 1.58x | 0.97x | 1.00x | 20/30 |
| san_parse_all | 0.92x | 1.69x | N/A | 3.88x | N/A | N/A | 12/12 |
| san_render_native | 0.90x | 10.49x | N/A | 2.28x | N/A | N/A | 12/12 |
| apply_children_full | 0.96x | 1.19x | 1.40x | 3.91x | 1.14x | 1.15x | 30/30 |
| make_unmake_full | 0.91x | N/A | N/A | 1.41x | N/A | N/A | 6/6 |
| startpos | 0.87x | 1.09x | 66.90x | 108.60x | 149.76x | 122.48x | 5/5 |
| chess960_materialize | 0.99x | 1.68x | 1.58x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.94x | 2.80x | 1.42x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 25.96x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 1.08x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.01x | 19.63x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.96x | 4.24x | 1.64x | 1.29x | 1.96x | 2.16x | 10/10 |
| corpus_materialize | 0.98x | 1.56x | 1.49x | 1.19x | 1.88x | 1.38x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.10x | 2.92x | 2.08x | 2.82x | 1.47x | 10/10 |
| replay_trusted_native | 0.96x | 0.60x | 1.57x | 1.91x | 2.09x | 2.18x | 8/10 |
| replay_checked_native | 0.96x | 2.04x | 1.01x | 6.80x | 6.09x | 7.91x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
