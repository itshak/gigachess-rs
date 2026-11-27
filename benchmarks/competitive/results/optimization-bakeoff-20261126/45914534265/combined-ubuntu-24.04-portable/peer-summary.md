| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.02x | 3.13x | 1.63x | 1.40x | 1.98x | 1.40x | 30/30 |
| materialize | 1.13x | 2.40x | 1.85x | 1.35x | 2.69x | 1.76x | 30/30 |
| perft_bulk_d4 | 0.99x | 3.09x | 1.59x | 1.21x | 1.96x | 1.80x | 30/30 |
| perft_full_d3 | 0.99x | 1.44x | 1.75x | 1.35x | 1.97x | 1.79x | 30/30 |
| fen_parse | 1.07x | 1.67x | 1.59x | 1.29x | 1.97x | 1.33x | 30/30 |
| fen_write_native_ep | 1.02x | 2.79x | 8.91x | 1.47x | 25.85x | 1.99x | 30/30 |
| snapshot | 0.99x | 1.62x | 1.16x | 2.63x | 1.15x | 0.99x | 27/30 |
| san_parse_all | 1.01x | 1.87x | N/A | 3.69x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 10.89x | N/A | 2.40x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 2.25x | 1.50x | 5.14x | 1.30x | 1.35x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.32x | N/A | N/A | 6/6 |
| startpos | 1.02x | 1.29x | 61.71x | 83.68x | 117.62x | 83.08x | 5/5 |
| chess960_materialize | 1.02x | 1.84x | 1.69x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.30x | 1.49x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.98x | 32.07x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.02x | 1.01x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.18x | 24.72x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 4.36x | 1.79x | 1.10x | 1.99x | 2.86x | 9/10 |
| corpus_materialize | 1.02x | 1.83x | 1.71x | 1.27x | 2.36x | 1.51x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.02x | 2.84x | 2.20x | 2.82x | 1.56x | 10/10 |
| replay_trusted_native | 1.09x | 0.78x | 1.74x | 1.76x | 2.26x | 2.07x | 9/10 |
| replay_checked_native | 0.96x | 2.17x | 1.26x | 8.65x | 8.44x | 9.87x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
