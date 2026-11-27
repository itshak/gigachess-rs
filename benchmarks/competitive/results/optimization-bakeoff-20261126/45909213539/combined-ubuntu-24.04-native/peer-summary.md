| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.25x | 1.84x | 1.55x | 2.43x | 1.86x | 30/30 |
| materialize | 1.03x | 2.21x | 1.68x | 1.72x | 2.65x | 1.98x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.75x | 1.53x | 1.17x | 2.33x | 1.84x | 29/30 |
| perft_full_d3 | 1.01x | 1.36x | 1.55x | 1.51x | 2.03x | 1.69x | 30/30 |
| fen_parse | 1.00x | 1.50x | 1.39x | 1.18x | 1.59x | 1.26x | 30/30 |
| fen_write_native_ep | 1.00x | 4.48x | 10.29x | 1.92x | 30.40x | 2.57x | 30/30 |
| snapshot | 1.00x | 2.00x | 1.00x | 4.66x | 1.00x | 1.33x | 23/30 |
| san_parse_all | 1.00x | 1.90x | N/A | 4.30x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 9.12x | N/A | 2.08x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.16x | 1.35x | 4.03x | 1.44x | 1.65x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.20x | N/A | N/A | 6/6 |
| startpos | 0.89x | 1.00x | 79.01x | 143.12x | 182.10x | 154.37x | 4/5 |
| chess960_materialize | 1.00x | 1.74x | 1.71x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.48x | 1.50x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.98x | 39.01x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.01x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.00x | 19.71x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 5.24x | 1.97x | 1.08x | 2.47x | 2.20x | 9/10 |
| corpus_materialize | 1.00x | 1.38x | 1.27x | 1.04x | 1.70x | 1.16x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.25x | 3.05x | 2.26x | 2.95x | 1.57x | 10/10 |
| replay_trusted_native | 1.00x | 0.86x | 1.52x | 1.56x | 2.16x | 2.52x | 9/10 |
| replay_checked_native | 1.00x | 2.13x | 1.12x | 9.05x | 9.09x | 10.41x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
