| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.04x | 4.73x | 1.96x | 1.70x | 2.67x | 2.01x | 30/30 |
| materialize | 0.90x | 2.09x | 1.52x | 1.74x | 2.46x | 1.87x | 30/30 |
| perft_bulk_d4 | 1.05x | 4.04x | 1.60x | 1.26x | 2.50x | 1.95x | 29/30 |
| perft_full_d3 | 1.17x | 1.58x | 1.90x | 1.77x | 2.36x | 1.92x | 30/30 |
| fen_parse | 0.94x | 1.23x | 1.29x | 1.10x | 1.40x | 1.21x | 30/30 |
| fen_write_native_ep | 0.99x | 4.95x | 10.47x | 1.89x | 29.03x | 2.15x | 30/30 |
| snapshot | 1.00x | 2.00x | 1.00x | 7.60x | 1.00x | 1.34x | 25/30 |
| san_parse_all | 1.12x | 2.11x | N/A | 4.30x | N/A | N/A | 12/12 |
| san_render_native | 0.96x | 8.94x | N/A | 2.05x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 2.15x | 1.35x | 4.03x | 1.43x | 1.64x | 30/30 |
| make_unmake_full | 0.96x | N/A | N/A | 1.14x | N/A | N/A | 6/6 |
| startpos | 0.89x | 1.00x | 82.24x | 146.15x | 168.80x | 155.47x | 4/5 |
| chess960_materialize | 0.93x | 1.64x | 1.55x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.02x | 2.56x | 1.51x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 39.68x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.69x | 1.64x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.21x | 23.23x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.22x | 6.27x | 2.33x | 1.31x | 2.85x | 2.62x | 10/10 |
| corpus_materialize | 1.00x | 1.43x | 1.29x | 1.09x | 1.73x | 1.17x | 10/10 |
| corpus_uci_sorted | 1.02x | 2.36x | 3.05x | 2.26x | 3.01x | 1.58x | 10/10 |
| replay_trusted_native | 0.97x | 0.83x | 1.55x | 1.53x | 2.08x | 2.42x | 9/10 |
| replay_checked_native | 0.94x | 2.02x | 1.12x | 8.59x | 8.19x | 9.69x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
