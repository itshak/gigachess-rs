| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.99x | 4.46x | 1.81x | 1.59x | 2.15x | 1.94x | 30/30 |
| materialize | 0.99x | 2.05x | 1.37x | 1.58x | 2.41x | 1.67x | 30/30 |
| perft_bulk_d4 | 0.99x | 4.00x | 1.55x | 1.18x | 2.16x | 1.85x | 29/30 |
| perft_full_d3 | 1.01x | 2.13x | 1.54x | 1.43x | 1.96x | 1.63x | 30/30 |
| fen_parse | 1.00x | 1.27x | 1.54x | 1.32x | 1.76x | 1.35x | 30/30 |
| fen_write_native_ep | 1.00x | 3.97x | 7.67x | 1.68x | 30.96x | 2.16x | 30/30 |
| snapshot | 0.93x | 2.01x | 1.02x | 4.73x | 1.02x | 0.99x | 21/30 |
| san_parse_all | 1.00x | 1.77x | N/A | 5.33x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 9.63x | N/A | 4.23x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.08x | 1.37x | 10.66x | 1.41x | 1.64x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.21x | N/A | N/A | 6/6 |
| startpos | 0.99x | 1.02x | 81.28x | 179.80x | 226.51x | 179.44x | 5/5 |
| chess960_materialize | 0.96x | 1.56x | 1.30x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.92x | 2.41x | 1.44x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.98x | 39.86x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.90x | 0.93x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.00x | 26.46x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.94x | 1.93x | 1.00x | 2.11x | 2.08x | 9/10 |
| corpus_materialize | 0.99x | 1.45x | 1.26x | 1.11x | 1.73x | 1.20x | 10/10 |
| corpus_uci_sorted | 0.92x | 1.68x | 2.18x | 1.74x | 1.98x | 1.44x | 10/10 |
| replay_trusted_native | 0.99x | 0.84x | 1.68x | 1.66x | 2.23x | 2.67x | 9/10 |
| replay_checked_native | 0.98x | 2.26x | 1.18x | 9.26x | 9.44x | 10.77x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
