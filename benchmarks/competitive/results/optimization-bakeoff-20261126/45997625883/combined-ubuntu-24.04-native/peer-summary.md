| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.22x | 1.84x | 1.55x | 2.33x | 1.88x | 30/30 |
| materialize | 1.00x | 2.23x | 1.69x | 1.74x | 2.62x | 1.87x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.73x | 1.54x | 1.18x | 2.30x | 1.85x | 29/30 |
| perft_full_d3 | 1.00x | 1.34x | 1.56x | 1.49x | 1.95x | 1.63x | 30/30 |
| fen_parse | 0.98x | 1.25x | 1.37x | 1.17x | 1.57x | 1.13x | 30/30 |
| fen_write_native_ep | 1.00x | 4.68x | 10.48x | 2.07x | 30.46x | 2.24x | 30/30 |
| snapshot | 1.00x | 1.67x | 1.33x | 4.00x | 1.33x | 1.33x | 30/30 |
| san_parse_all | 1.00x | 1.92x | N/A | 4.43x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 9.20x | N/A | 2.14x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.11x | 1.33x | 4.04x | 1.26x | 1.73x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.20x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.26x | 99.65x | 183.48x | 231.09x | 178.94x | 5/5 |
| chess960_materialize | 1.00x | 1.75x | 1.72x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.48x | 1.51x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 39.96x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.00x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.99x | 19.75x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 5.25x | 1.98x | 1.08x | 2.36x | 2.21x | 9/10 |
| corpus_materialize | 1.00x | 1.38x | 1.28x | 1.05x | 1.67x | 1.15x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.25x | 3.06x | 2.25x | 2.98x | 1.56x | 10/10 |
| replay_trusted_native | 1.04x | 0.89x | 1.58x | 1.64x | 2.24x | 2.63x | 9/10 |
| replay_checked_native | 1.01x | 2.14x | 1.14x | 9.15x | 8.90x | 10.49x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
