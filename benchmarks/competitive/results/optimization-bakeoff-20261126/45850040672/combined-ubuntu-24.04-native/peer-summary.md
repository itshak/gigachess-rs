| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.99x | 4.21x | 1.82x | 1.54x | 2.33x | 1.88x | 30/30 |
| materialize | 1.00x | 2.21x | 1.68x | 1.74x | 2.62x | 1.88x | 30/30 |
| perft_bulk_d4 | 0.99x | 3.72x | 1.54x | 1.16x | 2.26x | 1.84x | 29/30 |
| perft_full_d3 | 1.00x | 1.33x | 1.59x | 1.50x | 1.94x | 1.65x | 30/30 |
| fen_parse | 1.00x | 1.29x | 1.36x | 1.15x | 1.46x | 1.13x | 30/30 |
| fen_write_native_ep | 0.96x | 4.71x | 9.58x | 1.83x | 26.70x | 2.07x | 30/30 |
| snapshot | 1.00x | 1.67x | 1.33x | 3.99x | 1.33x | 1.33x | 30/30 |
| san_parse_all | 1.08x | 1.91x | N/A | 4.29x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 9.37x | N/A | 2.15x | N/A | N/A | 12/12 |
| apply_children_full | 1.03x | 2.11x | 1.33x | 4.03x | 1.26x | 1.74x | 30/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.19x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.25x | 99.59x | 183.57x | 220.09x | 181.13x | 5/5 |
| chess960_materialize | 1.02x | 1.76x | 1.75x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.57x | 1.51x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 39.69x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.03x | 23.74x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.98x | 5.28x | 2.00x | 1.08x | 2.38x | 2.21x | 9/10 |
| corpus_materialize | 1.00x | 1.38x | 1.28x | 1.07x | 1.69x | 1.16x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.28x | 3.00x | 2.22x | 2.91x | 1.55x | 10/10 |
| replay_trusted_native | 1.05x | 0.91x | 1.68x | 1.67x | 2.27x | 2.69x | 9/10 |
| replay_checked_native | 1.11x | 2.21x | 1.16x | 9.44x | 9.02x | 10.67x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
