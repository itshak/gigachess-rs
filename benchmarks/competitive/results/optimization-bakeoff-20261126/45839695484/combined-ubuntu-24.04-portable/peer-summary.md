| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.93x | 2.93x | 1.51x | 1.30x | 1.84x | 1.29x | 27/30 |
| materialize | 1.02x | 2.29x | 1.85x | 1.40x | 2.56x | 1.70x | 30/30 |
| perft_bulk_d4 | 1.02x | 3.16x | 1.64x | 1.22x | 2.02x | 1.78x | 30/30 |
| perft_full_d3 | 1.00x | 1.22x | 1.48x | 1.12x | 1.67x | 1.49x | 30/30 |
| fen_parse | 1.11x | 1.68x | 1.67x | 1.27x | 2.04x | 1.36x | 30/30 |
| fen_write_native_ep | 1.00x | 2.73x | 8.68x | 1.47x | 25.42x | 1.91x | 30/30 |
| snapshot | 1.00x | 1.62x | 1.16x | 2.64x | 1.16x | 1.00x | 28/30 |
| san_parse_all | 1.03x | 1.78x | N/A | 4.01x | N/A | N/A | 12/12 |
| san_render_native | 0.96x | 10.16x | N/A | 2.21x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 2.10x | 1.39x | 4.53x | 1.22x | 1.25x | 30/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.34x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.29x | 58.59x | 80.66x | 119.78x | 82.14x | 5/5 |
| chess960_materialize | 1.02x | 1.85x | 1.68x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.30x | 1.47x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.02x | 32.03x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 1.00x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.21x | 24.91x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.98x | 4.23x | 1.73x | 1.07x | 1.88x | 2.74x | 9/10 |
| corpus_materialize | 1.02x | 1.83x | 1.70x | 1.23x | 2.37x | 1.47x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.03x | 2.86x | 2.23x | 2.92x | 1.62x | 10/10 |
| replay_trusted_native | 0.99x | 0.67x | 1.58x | 1.57x | 2.10x | 1.88x | 8/10 |
| replay_checked_native | 1.01x | 2.27x | 1.22x | 8.26x | 8.26x | 9.56x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
