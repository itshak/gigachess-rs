| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.98x | 5.48x | 1.70x | 1.92x | 2.67x | 2.02x | 30/30 |
| materialize | 0.96x | 2.03x | 1.58x | 1.13x | 2.28x | 1.39x | 29/30 |
| perft_bulk_d4 | 1.04x | 4.01x | 1.49x | 1.24x | 2.07x | 1.88x | 29/30 |
| perft_full_d3 | 0.99x | 1.61x | 1.74x | 1.39x | 1.93x | 2.03x | 30/30 |
| fen_parse | 1.04x | 1.28x | 1.44x | 1.36x | 1.62x | 1.68x | 30/30 |
| fen_write_native_ep | 1.09x | 4.37x | 9.35x | 1.73x | 30.30x | 2.28x | 30/30 |
| snapshot | 0.90x | 1.63x | 0.98x | 2.02x | 0.95x | 0.99x | 21/30 |
| san_parse_all | 0.99x | 1.71x | N/A | 3.74x | N/A | N/A | 12/12 |
| san_render_native | 0.98x | 12.47x | N/A | 3.14x | N/A | N/A | 12/12 |
| apply_children_full | 0.90x | 1.09x | 1.21x | 5.73x | 1.17x | 1.27x | 27/30 |
| make_unmake_full | 0.86x | N/A | N/A | 1.21x | N/A | N/A | 6/6 |
| startpos | 0.61x | 0.73x | 59.57x | 100.30x | 108.82x | 124.21x | 4/5 |
| chess960_materialize | 1.10x | 1.66x | 1.58x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.95x | 2.94x | 1.58x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.97x | 37.69x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.88x | 1.00x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.09x | 19.36x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.29x | 6.34x | 2.02x | 1.56x | 2.81x | 2.88x | 10/10 |
| corpus_materialize | 1.03x | 1.36x | 1.34x | 1.04x | 1.69x | 1.42x | 10/10 |
| corpus_uci_sorted | 0.95x | 1.89x | 2.43x | 1.70x | 2.33x | 1.40x | 10/10 |
| replay_trusted_native | 0.88x | 0.70x | 1.72x | 1.67x | 2.01x | 1.87x | 8/10 |
| replay_checked_native | 0.87x | 2.41x | 1.18x | 7.87x | 7.75x | 9.35x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
