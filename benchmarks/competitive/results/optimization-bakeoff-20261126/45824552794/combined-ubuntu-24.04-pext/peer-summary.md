| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.12x | 1.72x | 1.50x | 2.36x | 1.76x | 30/30 |
| materialize | 1.00x | 2.16x | 1.67x | 1.67x | 2.62x | 1.83x | 30/30 |
| perft_bulk_d4 | 0.98x | 3.34x | 1.31x | 1.03x | 2.17x | 1.61x | 28/30 |
| perft_full_d3 | 1.00x | 1.46x | 1.68x | 1.63x | 2.11x | 1.73x | 30/30 |
| fen_parse | 0.94x | 1.25x | 1.23x | 1.03x | 1.35x | 1.06x | 28/30 |
| fen_write_native_ep | 0.98x | 5.01x | 10.18x | 2.12x | 29.49x | 2.62x | 30/30 |
| snapshot | 1.00x | 1.67x | 1.33x | 4.00x | 1.33x | 1.33x | 30/30 |
| san_parse_all | 1.00x | 2.09x | N/A | 4.47x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 9.33x | N/A | 2.15x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.04x | 1.29x | 3.90x | 1.21x | 1.68x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.16x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.26x | 99.40x | 174.89x | 210.49x | 176.71x | 5/5 |
| chess960_materialize | 1.04x | 1.62x | 1.60x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.36x | 1.36x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 40.28x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.95x | 0.97x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 1.01x | 18.48x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.72x | 1.75x | 0.97x | 2.11x | 1.96x | 8/10 |
| corpus_materialize | 1.00x | 1.37x | 1.29x | 1.05x | 1.68x | 1.14x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.25x | 3.01x | 2.20x | 2.95x | 1.55x | 10/10 |
| replay_trusted_native | 1.00x | 0.89x | 1.59x | 1.55x | 2.22x | 2.55x | 9/10 |
| replay_checked_native | 1.02x | 2.10x | 1.09x | 8.75x | 8.50x | 9.87x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
