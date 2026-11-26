| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.26x | 1.86x | 1.55x | 2.33x | 1.90x | 30/30 |
| materialize | 1.00x | 2.23x | 1.68x | 1.73x | 2.60x | 1.86x | 30/30 |
| perft_bulk_d4 | 0.98x | 3.68x | 1.51x | 1.16x | 2.32x | 1.83x | 29/30 |
| perft_full_d3 | 1.00x | 1.34x | 1.62x | 1.51x | 1.93x | 1.64x | 30/30 |
| fen_parse | 1.01x | 1.40x | 1.37x | 1.18x | 1.52x | 1.27x | 30/30 |
| fen_write_native_ep | 1.01x | 4.99x | 10.08x | 1.90x | 28.70x | 2.56x | 30/30 |
| snapshot | 1.01x | 1.67x | 1.34x | 4.02x | 1.34x | 1.34x | 30/30 |
| san_parse_all | 1.01x | 1.77x | N/A | 3.90x | N/A | N/A | 12/12 |
| san_render_native | 1.02x | 9.15x | N/A | 2.14x | N/A | N/A | 12/12 |
| apply_children_full | 1.02x | 2.11x | 1.33x | 4.01x | 1.26x | 1.73x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.17x | N/A | N/A | 6/6 |
| startpos | 1.12x | 1.26x | 103.06x | 182.22x | 220.45x | 192.40x | 5/5 |
| chess960_materialize | 0.95x | 1.67x | 1.63x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.56x | 1.51x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 40.21x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.03x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.82x | 19.74x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 5.31x | 2.00x | 1.09x | 2.44x | 2.25x | 9/10 |
| corpus_materialize | 1.00x | 1.39x | 1.28x | 1.05x | 1.67x | 1.15x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.30x | 3.00x | 2.23x | 2.95x | 1.57x | 10/10 |
| replay_trusted_native | 1.04x | 0.92x | 1.67x | 1.62x | 2.28x | 2.67x | 9/10 |
| replay_checked_native | 1.09x | 2.21x | 1.15x | 9.25x | 8.94x | 10.49x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
